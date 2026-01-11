//! FTML parser that transforms HTML-like markup into [`Document`](crate::Document) trees.

use crate::{ChecklistItem, Document, InlineStyle, Paragraph, ParagraphType, Span};
use regex::Regex;
use std::collections::HashMap;
use std::io::Read;
use thiserror::Error;

#[derive(Error, Debug)]
/// Errors that can occur while parsing FTML source.
pub enum ParseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unexpected text content: {0}")]
    UnexpectedTextContent(String),
    #[error("Paragraphs not allowed inside leaf paragraph nodes when trying to add {new_type} below {parent_type}")]
    InvalidNesting {
        new_type: ParagraphType,
        parent_type: ParagraphType,
    },
    #[error("Closing unopened paragraph of type {0}")]
    ClosingUnopenedParagraph(ParagraphType),
    #[error("Cannot close {actual} with {expected}")]
    MismatchedClosingTag {
        actual: ParagraphType,
        expected: ParagraphType,
    },
    #[error("Unexpected list item, parent: {0:?}")]
    UnexpectedListItem(Option<ParagraphType>),
    #[error("Checklist items must include a checkbox input")]
    ChecklistItemMissingCheckbox,
    #[error("Cannot mix checklist items with regular list items")]
    MixedChecklistTypes,
    #[error("Checklist items may only contain inline text and nested checklists; found {found}")]
    InvalidChecklistContent { found: ParagraphType },
    #[error("Unexpected closing tag for list item")]
    UnexpectedClosingListItem,
    #[error("Paragraph content for list without list item")]
    ListContentWithoutItem,
    #[error("Non-inline token: {0}")]
    NonInlineToken(String),
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),
    #[error("Unexpected EOF")]
    UnexpectedEof,
    #[error("Unexpected closing tag: {0}")]
    UnexpectedClosingTag(String),
    #[error("No closing tag for {0}")]
    NoClosingTag(InlineStyle),
}

// Simple tokenizer for FTML parsing
#[derive(Debug, Clone)]
struct Tag {
    name: String,
    attributes: HashMap<String, String>,
}

#[derive(Debug, Clone)]
enum Token {
    Text(String),
    StartTag(Tag),
    EndTag(String),
    SelfClosingTag(Tag),
}

struct Tokenizer {
    input: String,
    pos: usize,
    putback_token: Option<(Token, usize, usize)>,
}

impl Tokenizer {
    fn new(input: String) -> Self {
        Self {
            input,
            pos: 0,
            putback_token: None,
        }
    }

    fn next(&mut self) -> Option<Token> {
        // Return putback token if available and restore position
        if let Some((token, _start_pos, end_pos)) = self.putback_token.take() {
            self.pos = end_pos;
            return Some(token);
        }

        self.skip_whitespace_between_tags();

        if self.pos >= self.input.len() {
            return None;
        }

        // Parse token
        if self.input.get(self.pos..self.pos + 1) == Some("<") {
            self.parse_tag()
        } else {
            self.parse_text()
        }
    }

    fn next_with_pos(&mut self) -> Option<(Token, usize)> {
        // Return putback token if available and restore position
        if let Some((token, start_pos, end_pos)) = self.putback_token.take() {
            self.pos = end_pos;
            return Some((token, start_pos));
        }

        self.skip_whitespace_between_tags();

        if self.pos >= self.input.len() {
            return None;
        }

        // Save position before parsing token
        let token_start_pos = self.pos;
        let token = if self.input.get(self.pos..self.pos + 1) == Some("<") {
            self.parse_tag()
        } else {
            self.parse_text()
        };

        token.map(|t| (t, token_start_pos))
    }

    fn putback(&mut self, token: Token, start_pos: usize) {
        self.putback_token = Some((token, start_pos, self.pos));
    }

    fn skip_whitespace_between_tags(&mut self) {
        // Skip ASCII/Unicode whitespace that appears between '>' and the next '<'
        // without touching whitespace that is part of text content.
        let start_pos = self.pos;
        while self.pos < self.input.len() {
            let remaining = &self.input[self.pos..];
            let mut iter = remaining.chars();
            if let Some(ch) = iter.next() {
                if ch.is_whitespace() {
                    // Only skip if, after trimming, the next non-space begins a tag
                    if remaining.trim_start().starts_with('<') {
                        self.pos += ch.len_utf8();
                        continue;
                    } else {
                        break; // whitespace belongs to content
                    }
                } else {
                    break; // non-whitespace
                }
            } else {
                break;
            }
        }
        // If we didn't actually pass over any whitespace, keep position as-is
        if self.pos == start_pos {}
    }

    fn parse_tag(&mut self) -> Option<Token> {
        self.pos += 1; // skip '<'

        let mut end_pos = self.pos;
        let mut in_quotes = false;
        let mut quote_char = '"';

        while end_pos < self.input.len() {
            let remaining = &self.input[end_pos..];
            if let Some(ch) = remaining.chars().next() {
                if !in_quotes && (ch == '"' || ch == '\'') {
                    in_quotes = true;
                    quote_char = ch;
                } else if in_quotes && ch == quote_char {
                    in_quotes = false;
                } else if !in_quotes && ch == '>' {
                    break;
                }
                end_pos += ch.len_utf8();
            } else {
                break;
            }
        }

        if end_pos >= self.input.len() {
            return None;
        }

        let tag_content = &self.input[self.pos..end_pos];
        self.pos = end_pos + 1; // skip '>'

        let trimmed = tag_content.trim();
        if let Some(stripped) = trimmed.strip_prefix('/') {
            let name = stripped.split_whitespace().next().unwrap_or("");
            return Some(Token::EndTag(name.to_ascii_lowercase()));
        }

        let mut effective = trimmed.to_string();
        let mut self_closing = false;
        if effective.ends_with('/') {
            self_closing = true;
            effective.truncate(effective.trim_end_matches('/').len());
            effective = effective.trim_end().to_string();
        }

        let (name, attributes) = Self::parse_tag_parts(&effective);
        if self_closing || name == "br" {
            return Some(Token::SelfClosingTag(Tag { name, attributes }));
        }

        Some(Token::StartTag(Tag { name, attributes }))
    }

    fn parse_tag_parts(content: &str) -> (String, HashMap<String, String>) {
        let mut chars = content.chars().peekable();

        while matches!(chars.peek(), Some(ch) if ch.is_whitespace()) {
            chars.next();
        }

        let mut name = String::new();
        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() {
                break;
            }
            name.push(ch.to_ascii_lowercase());
            chars.next();
        }

        let mut attributes = HashMap::new();

        loop {
            while matches!(chars.peek(), Some(ch) if ch.is_whitespace()) {
                chars.next();
            }

            let mut attr_name = String::new();
            while let Some(&ch) = chars.peek() {
                if ch.is_whitespace() || ch == '=' {
                    break;
                }
                attr_name.push(ch.to_ascii_lowercase());
                chars.next();
            }

            if attr_name.is_empty() {
                break;
            }

            while matches!(chars.peek(), Some(ch) if ch.is_whitespace()) {
                chars.next();
            }

            let mut value = String::new();
            if matches!(chars.peek(), Some('=')) {
                chars.next(); // consume '='

                while matches!(chars.peek(), Some(ch) if ch.is_whitespace()) {
                    chars.next();
                }

                if let Some(&quote) = chars.peek() {
                    if quote == '"' || quote == '\'' {
                        chars.next(); // consume opening quote
                        while let Some(&ch) = chars.peek() {
                            chars.next();
                            if ch == quote {
                                break;
                            }
                            value.push(ch);
                        }
                    } else {
                        while let Some(&ch) = chars.peek() {
                            if ch.is_whitespace() {
                                break;
                            }
                            value.push(ch);
                            chars.next();
                        }
                    }
                }
            }

            attributes.insert(attr_name, value);
        }

        (name, attributes)
    }

    fn parse_text(&mut self) -> Option<Token> {
        let start = self.pos;

        while self.pos < self.input.len() {
            let remaining = &self.input[self.pos..];
            if let Some(ch) = remaining.chars().next() {
                if ch == '<' {
                    break;
                }
                self.pos += ch.len_utf8();
            } else {
                break;
            }
        }

        let text = &self.input[start..self.pos];
        if text.is_empty() {
            None
        } else {
            Some(Token::Text(text.to_string()))
        }
    }
}

/// Stateful FTML parser that understands a restricted HTML-like tag set.
///
/// The parser can be reused across multiple inputs. Use [`Parser::parse_string`]
/// for in-memory data or [`parse`] for any [`Read`] implementation.
///
/// # Examples
///
/// ```
/// use tdoc::parser::Parser;
///
/// let parser = Parser::new();
/// let document = parser.parse_string("<p>Hello, world!</p>").unwrap();
/// assert_eq!(document.paragraphs.len(), 1);
/// ```
pub struct Parser {
    wrapper_elements: HashMap<String, ParagraphType>,
    inline_elements: HashMap<String, InlineStyle>,
    space_regex: Regex,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

enum ListItemContent {
    Checklist(ChecklistItem),
    Paragraphs(Vec<Paragraph>),
}

impl Parser {
    /// Creates a parser configured with the default FTML tag mappings.
    pub fn new() -> Self {
        let mut wrapper_elements = HashMap::new();
        wrapper_elements.insert("p".to_string(), ParagraphType::Text);
        wrapper_elements.insert("h1".to_string(), ParagraphType::Header1);
        wrapper_elements.insert("h2".to_string(), ParagraphType::Header2);
        wrapper_elements.insert("h3".to_string(), ParagraphType::Header3);
        wrapper_elements.insert("pre".to_string(), ParagraphType::CodeBlock);
        wrapper_elements.insert("blockquote".to_string(), ParagraphType::Quote);
        wrapper_elements.insert("ul".to_string(), ParagraphType::UnorderedList);
        wrapper_elements.insert("ol".to_string(), ParagraphType::OrderedList);

        let mut inline_elements = HashMap::new();
        inline_elements.insert("b".to_string(), InlineStyle::Bold);
        inline_elements.insert("i".to_string(), InlineStyle::Italic);
        inline_elements.insert("u".to_string(), InlineStyle::Underline);
        inline_elements.insert("s".to_string(), InlineStyle::Strike);
        inline_elements.insert("mark".to_string(), InlineStyle::Highlight);
        inline_elements.insert("code".to_string(), InlineStyle::Code);
        inline_elements.insert("a".to_string(), InlineStyle::Link);

        Self {
            wrapper_elements,
            inline_elements,
            space_regex: Regex::new(r"\s+").unwrap(),
        }
    }

    /// Parses a string slice into a [`Document`].
    pub fn parse_string(&self, input: &str) -> Result<Document, ParseError> {
        let mut tokenizer = Tokenizer::new(input.to_string());
        let mut document = Document::new();
        let mut breadcrumbs: Vec<Paragraph> = Vec::new();
        let mut list_item_level = 0;

        while let Some(token) = tokenizer.next() {
            self.process_token(
                token,
                &mut document,
                &mut breadcrumbs,
                &mut list_item_level,
                &mut tokenizer,
            )?;
        }

        normalize_entity_whitespace(&mut document);
        Ok(document)
    }

    fn process_token(
        &self,
        token: Token,
        document: &mut Document,
        breadcrumbs: &mut Vec<Paragraph>,
        list_item_level: &mut i32,
        tokenizer: &mut Tokenizer,
    ) -> Result<(), ParseError> {
        match token {
            Token::StartTag(tag) => {
                let tag_name = tag.name.clone();
                if tag_name == "li" {
                    if let Some(parent) = breadcrumbs.last_mut() {
                        let parent_type = parent.paragraph_type();
                        if parent_type == ParagraphType::UnorderedList
                            || parent_type == ParagraphType::OrderedList
                            || parent_type == ParagraphType::Checklist
                        {
                            let parent_is_checklist = parent_type == ParagraphType::Checklist;
                            let (list_content, remaining_token) =
                                self.read_list_content(tokenizer, parent_is_checklist)?;

                            match list_content {
                                ListItemContent::Checklist(item) => match parent.paragraph_type() {
                                    ParagraphType::Checklist => {
                                        parent.add_checklist_item(item);
                                    }
                                    ParagraphType::UnorderedList => {
                                        if !parent.entries().is_empty() {
                                            return Err(ParseError::MixedChecklistTypes);
                                        }
                                        *parent = Paragraph::new_checklist();
                                        parent.add_checklist_item(item);
                                    }
                                    ParagraphType::OrderedList => {
                                        return Err(ParseError::UnexpectedListItem(Some(
                                            ParagraphType::OrderedList,
                                        )));
                                    }
                                    _ => unreachable!(),
                                },
                                ListItemContent::Paragraphs(entry) => {
                                    if parent.paragraph_type() == ParagraphType::Checklist {
                                        return Err(ParseError::ChecklistItemMissingCheckbox);
                                    }
                                    parent.add_list_item(entry);
                                }
                            }

                            // If there's a remaining token (parent structure ending), handle it
                            if let Some(token) = remaining_token {
                                return self.process_token(
                                    token,
                                    document,
                                    breadcrumbs,
                                    list_item_level,
                                    tokenizer,
                                );
                            }
                            *list_item_level += 1;
                        } else {
                            return Err(ParseError::UnexpectedListItem(Some(parent_type)));
                        }
                    } else {
                        return Err(ParseError::UnexpectedListItem(None));
                    }
                } else if let Some(&paragraph_type) = self.wrapper_elements.get(&tag_name) {
                    self.process_start_paragraph(
                        paragraph_type,
                        document,
                        breadcrumbs,
                        list_item_level,
                        tokenizer,
                    )?;
                }
            }
            Token::EndTag(tag_name) => {
                if tag_name == "li" {
                    if *list_item_level < 1 {
                        return Err(ParseError::UnexpectedClosingListItem);
                    }
                    *list_item_level -= 1;
                } else if let Some(&paragraph_type) = self.wrapper_elements.get(&tag_name) {
                    self.process_end_paragraph(paragraph_type, breadcrumbs, document)?;
                }
            }
            Token::SelfClosingTag(_tag_name) => {
                // Handle self-closing tags like <br />
            }
            Token::Text(text) => {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    if breadcrumbs.is_empty() {
                        return Err(ParseError::UnexpectedTextContent(trimmed.to_string()));
                    }

                    // Check if we have text content in a non-leaf paragraph
                    if let Some(parent) = breadcrumbs.last() {
                        let parent_type = parent.paragraph_type();
                        if !parent_type.is_leaf()
                            && parent_type != ParagraphType::UnorderedList
                            && parent_type != ParagraphType::OrderedList
                        {
                            return Err(ParseError::UnexpectedTextContent(trimmed.to_string()));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn process_start_paragraph(
        &self,
        paragraph_type: ParagraphType,
        document: &mut Document,
        breadcrumbs: &mut Vec<Paragraph>,
        _list_item_level: &mut i32,
        tokenizer: &mut Tokenizer,
    ) -> Result<(), ParseError> {
        let mut paragraph = Paragraph::new(paragraph_type);

        if paragraph_type.is_leaf() {
            // Read content for leaf paragraphs
            let content = if paragraph_type == ParagraphType::CodeBlock {
                self.read_code_block_content(tokenizer, paragraph_type.html_tag())?
            } else {
                self.read_content(tokenizer, paragraph_type.html_tag())?
            };
            paragraph = paragraph.with_content(content);
            self.add_paragraph_to_current_context(&paragraph, document, breadcrumbs)?;
        } else {
            // For non-leaf paragraphs, add to breadcrumbs
            breadcrumbs.push(paragraph);
        }

        Ok(())
    }

    fn process_end_paragraph(
        &self,
        paragraph_type: ParagraphType,
        breadcrumbs: &mut Vec<Paragraph>,
        document: &mut Document,
    ) -> Result<(), ParseError> {
        if let Some(current) = breadcrumbs.last() {
            let current_type = current.paragraph_type();
            if !current_type.matches_closing_tag(paragraph_type) {
                return Err(ParseError::MismatchedClosingTag {
                    actual: current_type,
                    expected: paragraph_type,
                });
            }
        } else {
            return Err(ParseError::ClosingUnopenedParagraph(paragraph_type));
        }

        let paragraph = breadcrumbs.pop().unwrap();

        // Add the completed paragraph to its parent or document
        if let Some(parent) = breadcrumbs.last_mut() {
            let parent_type = parent.paragraph_type();
            if parent_type == ParagraphType::UnorderedList
                || parent_type == ParagraphType::OrderedList
            {
                if let Some(last_entry) = parent.entries_mut().last_mut() {
                    last_entry.push(paragraph);
                } else {
                    return Err(ParseError::ListContentWithoutItem);
                }
            } else if parent_type == ParagraphType::Checklist {
                return Err(ParseError::InvalidChecklistContent {
                    found: paragraph.paragraph_type(),
                });
            } else {
                parent.children_mut().push(paragraph);
            }
        } else {
            // Add to document if no parent
            document.add_paragraph(paragraph);
        }

        Ok(())
    }

    fn add_paragraph_to_current_context(
        &self,
        paragraph: &Paragraph,
        document: &mut Document,
        breadcrumbs: &mut [Paragraph],
    ) -> Result<(), ParseError> {
        if let Some(parent) = breadcrumbs.last_mut() {
            let parent_type = parent.paragraph_type();
            // If the parent is a list, add to the current list entry
            if parent_type == ParagraphType::UnorderedList
                || parent_type == ParagraphType::OrderedList
                || parent_type == ParagraphType::Checklist
            {
                if let Some(last_entry) = parent.entries_mut().last_mut() {
                    last_entry.push(paragraph.clone());
                } else {
                    return Err(ParseError::ListContentWithoutItem);
                }
            } else {
                // Otherwise add to the parent paragraph's children
                parent.children_mut().push(paragraph.clone());
            }
        } else {
            document.add_paragraph(paragraph.clone());
        }
        Ok(())
    }

    fn read_list_content(
        &self,
        tokenizer: &mut Tokenizer,
        parent_is_checklist: bool,
    ) -> Result<(ListItemContent, Option<Token>), ParseError> {
        let mut paragraphs = Vec::new();
        let mut breadcrumbs: Vec<(Paragraph, usize)> = Vec::new();
        let mut inline_spans: Vec<Span> = Vec::new();
        let mut checklist_state: Option<bool> = None;

        while let Some(token) = tokenizer.next() {
            match token {
                Token::EndTag(tag_name) if tag_name == "li" => {
                    Self::flush_inline_spans(&mut inline_spans, &mut paragraphs);
                    break;
                }
                Token::EndTag(ref tag_name) => {
                    if let Some(&paragraph_type) = self.wrapper_elements.get(tag_name) {
                        if let Some((paragraph, _)) = breadcrumbs.last() {
                            if paragraph
                                .paragraph_type()
                                .matches_closing_tag(paragraph_type)
                            {
                                let (mut paragraph, start_len) = breadcrumbs.pop().unwrap();
                                if !paragraph_type.is_leaf() {
                                    let children = paragraphs.split_off(start_len);
                                    if !children.is_empty() {
                                        match paragraph.paragraph_type() {
                                            ParagraphType::Quote => {
                                                paragraph = paragraph.with_children(children);
                                            }
                                            _ => {
                                                // Non-quote containers (like stray lists)
                                                // should not have unconsumed children.
                                                // Treat them as siblings to avoid panics.
                                                paragraphs.extend(children);
                                            }
                                        }
                                    }
                                }
                                paragraphs.push(paragraph);
                                continue;
                            }
                        }

                        Self::flush_inline_spans(&mut inline_spans, &mut paragraphs);
                        Self::finalize_list_breadcrumbs(&mut breadcrumbs, &mut paragraphs);
                        let content = Self::finalize_list_item(
                            paragraphs,
                            checklist_state,
                            parent_is_checklist,
                        )?;
                        return Ok((content, Some(Token::EndTag(tag_name.clone()))));
                    } else {
                        return Err(ParseError::UnexpectedClosingTag(tag_name.clone()));
                    }
                }
                Token::StartTag(tag) => {
                    let tag_name = tag.name.clone();
                    if tag_name == "li" {
                        Self::flush_inline_spans(&mut inline_spans, &mut paragraphs);
                        if let Some((parent, _)) = breadcrumbs.last_mut() {
                            let parent_type = parent.paragraph_type();
                            if parent_type == ParagraphType::UnorderedList
                                || parent_type == ParagraphType::OrderedList
                                || parent_type == ParagraphType::Checklist
                            {
                                let (list_content, remaining_token) = self.read_list_content(
                                    tokenizer,
                                    parent_type == ParagraphType::Checklist,
                                )?;

                                match list_content {
                                    ListItemContent::Checklist(item) => {
                                        match parent.paragraph_type() {
                                            ParagraphType::Checklist => {
                                                parent.add_checklist_item(item);
                                            }
                                            ParagraphType::UnorderedList => {
                                                if !parent.entries().is_empty() {
                                                    return Err(ParseError::MixedChecklistTypes);
                                                }
                                                *parent = Paragraph::new_checklist();
                                                parent.add_checklist_item(item);
                                            }
                                            ParagraphType::OrderedList => {
                                                return Err(ParseError::UnexpectedListItem(Some(
                                                    ParagraphType::OrderedList,
                                                )));
                                            }
                                            _ => unreachable!(),
                                        }
                                    }
                                    ListItemContent::Paragraphs(entry) => {
                                        if parent.paragraph_type() == ParagraphType::Checklist {
                                            return Err(ParseError::ChecklistItemMissingCheckbox);
                                        }
                                        parent.add_list_item(entry);
                                    }
                                }

                                if let Some(token) = remaining_token {
                                    Self::finalize_list_breadcrumbs(
                                        &mut breadcrumbs,
                                        &mut paragraphs,
                                    );
                                    let content = Self::finalize_list_item(
                                        paragraphs,
                                        checklist_state,
                                        parent_is_checklist,
                                    )?;
                                    return Ok((content, Some(token)));
                                }
                            } else {
                                return Err(ParseError::UnexpectedListItem(Some(parent_type)));
                            }
                        } else {
                            return Err(ParseError::UnexpectedListItem(None));
                        }
                    } else if let Some(&paragraph_type) = self.wrapper_elements.get(&tag_name) {
                        Self::flush_inline_spans(&mut inline_spans, &mut paragraphs);
                        let mut paragraph = Paragraph::new(paragraph_type);

                        if paragraph_type.is_leaf() {
                            let content =
                                self.read_content(tokenizer, paragraph_type.html_tag())?;
                            paragraph = paragraph.with_content(content);
                            paragraphs.push(paragraph);
                        } else {
                            breadcrumbs.push((paragraph, paragraphs.len()));
                        }
                    } else if let Some(&style) = self.inline_elements.get(&tag_name) {
                        // Handle inline style tags (b, i, u, etc.) when in checklist context
                        if checklist_state.is_some() || parent_is_checklist {
                            let span = self.read_span(tokenizer, style, tag)?;
                            inline_spans.push(span);
                        } else {
                            return Err(ParseError::NonInlineToken(tag_name));
                        }
                    } else {
                        return Err(ParseError::NonInlineToken(tag_name));
                    }
                }
                Token::SelfClosingTag(tag) => {
                    let tag_name = tag.name.to_ascii_lowercase();
                    if tag_name == "input" {
                        let is_checkbox = tag
                            .attributes
                            .get("type")
                            .map(|value| value.eq_ignore_ascii_case("checkbox"))
                            .unwrap_or(false);
                        if is_checkbox {
                            let checked = tag.attributes.contains_key("checked");
                            checklist_state = Some(checked);
                        }
                    }
                }
                Token::Text(text) => {
                    let trimmed = text.trim();
                    if trimmed.is_empty() {
                        continue;
                    }

                    if checklist_state.is_some() || parent_is_checklist {
                        let collapsed =
                            self.collapse_whitespace(&text, inline_spans.is_empty(), false);
                        let decoded = self.decode_entities(&collapsed);
                        if !decoded.is_empty() {
                            inline_spans.push(Span::new_text(decoded));
                        }
                        continue;
                    }

                    return Err(ParseError::UnexpectedTextContent(trimmed.to_string()));
                }
            }
        }

        Self::flush_inline_spans(&mut inline_spans, &mut paragraphs);
        Self::finalize_list_breadcrumbs(&mut breadcrumbs, &mut paragraphs);

        let content = Self::finalize_list_item(paragraphs, checklist_state, parent_is_checklist)?;
        Ok((content, None))
    }

    fn flush_inline_spans(spans: &mut Vec<Span>, paragraphs: &mut Vec<Paragraph>) {
        if spans.is_empty() {
            return;
        }

        let mut paragraph = Paragraph::new_text();
        paragraph.content_mut().append(spans);
        paragraphs.push(paragraph);
    }

    fn convert_to_checklist_item(
        paragraphs: Vec<Paragraph>,
        checked: bool,
    ) -> Result<ChecklistItem, ParseError> {
        let mut item = ChecklistItem::new(checked);
        let mut content = Vec::new();
        let mut children = Vec::new();

        for paragraph in paragraphs {
            match paragraph {
                Paragraph::Text { content: mut spans } => {
                    if spans.is_empty() {
                        continue;
                    }
                    if !content.is_empty() {
                        content.push(Span::new_text("\n"));
                    }
                    content.append(&mut spans);
                }
                Paragraph::Checklist { mut items } => {
                    children.append(&mut items);
                }
                other => {
                    if other.content().is_empty()
                        && other.children().is_empty()
                        && other.entries().is_empty()
                        && other.checklist_items().is_empty()
                    {
                        continue;
                    }
                    return Err(ParseError::InvalidChecklistContent {
                        found: other.paragraph_type(),
                    });
                }
            }
        }

        trim_trailing_inline_whitespace(&mut content);
        item.content = content;
        item.children = children;
        Ok(item)
    }

    fn finalize_list_item(
        paragraphs: Vec<Paragraph>,
        checklist_state: Option<bool>,
        parent_is_checklist: bool,
    ) -> Result<ListItemContent, ParseError> {
        if let Some(checked) = checklist_state {
            let item = Self::convert_to_checklist_item(paragraphs, checked)?;
            Ok(ListItemContent::Checklist(item))
        } else if parent_is_checklist {
            Err(ParseError::ChecklistItemMissingCheckbox)
        } else {
            Ok(ListItemContent::Paragraphs(paragraphs))
        }
    }

    fn finalize_list_breadcrumbs(
        breadcrumbs: &mut Vec<(Paragraph, usize)>,
        paragraphs: &mut Vec<Paragraph>,
    ) {
        while let Some((mut paragraph, start_len)) = breadcrumbs.pop() {
            let new_children = paragraphs.split_off(start_len);
            if !new_children.is_empty() {
                paragraph = match paragraph {
                    Paragraph::Quote { mut children } => {
                        children.extend(new_children);
                        Paragraph::Quote { children }
                    }
                    other => {
                        debug_assert!(
                            new_children.is_empty(),
                            "unexpected children for non-quote paragraph"
                        );
                        other
                    }
                };
            }
            paragraphs.push(paragraph);
        }
    }

    fn read_code_block_content(
        &self,
        tokenizer: &mut Tokenizer,
        end_tag: &str,
    ) -> Result<Vec<Span>, ParseError> {
        let mut text = self.read_code_block_inner(tokenizer, end_tag)?;
        Self::normalize_code_block_text(&mut text);
        if text.is_empty() {
            Ok(Vec::new())
        } else {
            Ok(vec![Span::new_text(text)])
        }
    }

    fn read_code_block_inner(
        &self,
        tokenizer: &mut Tokenizer,
        end_tag: &str,
    ) -> Result<String, ParseError> {
        let mut buffer = String::new();

        while let Some((token, token_pos)) = tokenizer.next_with_pos() {
            match token {
                Token::EndTag(tag_name) if tag_name == end_tag => {
                    return Ok(buffer);
                }
                Token::Text(text) => {
                    buffer.push_str(&self.decode_entities(&text));
                }
                Token::SelfClosingTag(tag) => {
                    if tag.name == "br" {
                        buffer.push('\n');
                    } else {
                        buffer.push_str(&format!("<{} />", tag.name));
                    }
                }
                Token::StartTag(tag) => {
                    let tag_name = tag.name.clone();
                    if self.wrapper_elements.contains_key(&tag_name) {
                        tokenizer.putback(Token::StartTag(tag), token_pos);
                        return Ok(buffer);
                    }
                    let inner = self.read_code_block_inner(tokenizer, &tag_name)?;
                    buffer.push_str(&inner);
                }
                Token::EndTag(tag_name) => {
                    tokenizer.putback(Token::EndTag(tag_name), token_pos);
                    return Ok(buffer);
                }
            }
        }

        Ok(buffer)
    }

    fn normalize_code_block_text(text: &mut String) {
        // Strip leading newline
        if text.starts_with("\r\n") {
            text.drain(..2);
        } else if text.starts_with('\n') || text.starts_with('\r') {
            text.drain(..1);
        }
        // Strip trailing newline
        if text.ends_with("\r\n") {
            text.truncate(text.len() - 2);
        } else if text.ends_with('\n') || text.ends_with('\r') {
            text.pop();
        }
    }

    fn read_content(
        &self,
        tokenizer: &mut Tokenizer,
        end_tag: &str,
    ) -> Result<Vec<Span>, ParseError> {
        let mut spans = Vec::new();
        let mut buffer = String::new();
        let mut has_leading_entity = false;
        let mut has_trailing_entity = false;

        while let Some((token, token_pos)) = tokenizer.next_with_pos() {
            match token {
                Token::EndTag(tag_name) if tag_name == end_tag => {
                    if !buffer.is_empty() {
                        spans.push(Span::new_text(self.decode_entities(
                            &self.collapse_whitespace(&buffer, spans.is_empty(), true),
                        )));
                        buffer.clear();
                    }
                    break;
                }
                Token::SelfClosingTag(tag) => {
                    if tag.name == "br" {
                        if !buffer.is_empty() {
                            let decoded = self.decode_entities(&buffer);
                            spans.push(Span::new_text(self.collapse_whitespace(
                                &decoded,
                                spans.is_empty(),
                                false,
                            )));
                            buffer.clear();
                        }
                        spans.push(Span::new_text("\n"));
                    } else {
                        return Err(ParseError::UnexpectedToken(format!(
                            "self-closing tag {}",
                            tag.name
                        )));
                    }
                }
                Token::StartTag(tag) => {
                    let tag_name = tag.name.clone();
                    if let Some(&style) = self.inline_elements.get(&tag_name) {
                        if !buffer.is_empty() {
                            spans.push(Span::new_text(self.decode_entities(
                                &self.collapse_whitespace(&buffer, spans.is_empty(), false),
                            )));
                            buffer.clear();
                        }
                        let span = self.read_span(tokenizer, style, tag)?;
                        spans.push(span);
                    } else if self.wrapper_elements.contains_key(&tag_name) {
                        // This is a structural element that should be handled by parent context
                        // Put the token back and return the content we've read so far
                        if !buffer.is_empty() {
                            spans.push(Span::new_text(self.decode_entities(
                                &self.collapse_whitespace(&buffer, spans.is_empty(), false),
                            )));
                        }
                        tokenizer.putback(Token::StartTag(tag), token_pos);
                        return Ok(spans);
                    } else {
                        return Err(ParseError::NonInlineToken(tag_name));
                    }
                }
                Token::Text(text) => {
                    // Track if we have entities at the start/end of the content
                    if spans.is_empty() && buffer.is_empty() && text.starts_with('&') {
                        has_leading_entity = true;
                    }
                    if text.ends_with(';') && text.contains('&') {
                        has_trailing_entity = true;
                    }

                    // If we just added a line break, trim leading whitespace from the next text
                    if !spans.is_empty() && spans.last().map(|s| s.text.as_str()) == Some("\n") {
                        buffer.push_str(text.trim_start());
                    } else {
                        buffer.push_str(&text);
                    }
                }
                Token::EndTag(tag_name) => {
                    if tag_name == end_tag {
                        // This is our expected end tag
                        break;
                    } else if self.wrapper_elements.contains_key(&tag_name) {
                        // This is a parent structure ending - put it back for parent to handle
                        if !buffer.is_empty() {
                            spans.push(Span::new_text(self.decode_entities(&buffer)));
                        }
                        tokenizer.putback(Token::EndTag(tag_name), token_pos);
                        return Ok(spans);
                    } else {
                        return Err(ParseError::UnexpectedToken(format!("end tag {}", tag_name)));
                    }
                }
            }
        }

        if !buffer.is_empty() {
            let decoded = self.decode_entities(&buffer);
            let buffer_has_leading_entity = buffer.starts_with('&');
            let buffer_has_trailing_entity = buffer.ends_with(';') && buffer.contains('&');

            if buffer_has_leading_entity {
                has_leading_entity = true;
            }
            if buffer_has_trailing_entity {
                has_trailing_entity = true;
            }

            spans.push(Span::new_text(self.collapse_whitespace(
                &decoded,
                spans.is_empty() && !buffer_has_leading_entity,
                !buffer_has_trailing_entity,
            )));
        }

        Ok(self.trim_whitespace_with_entities(spans, has_leading_entity, has_trailing_entity))
    }

    fn read_span(
        &self,
        tokenizer: &mut Tokenizer,
        style: InlineStyle,
        start_tag: Tag,
    ) -> Result<Span, ParseError> {
        let end_tag = start_tag.name.clone();
        let mut span = Span::new_styled(style);
        if style == InlineStyle::Link {
            if let Some(target) = start_tag.attributes.get("href") {
                let decoded = self.decode_entities(target);
                span = span.with_link_target(decoded);
            }
        }

        let mut children = Vec::new();
        let mut buffer = String::new();

        while let Some(token) = tokenizer.next() {
            match token {
                Token::EndTag(tag_name) if tag_name == end_tag => {
                    if !buffer.is_empty() {
                        let text = self.decode_entities(&buffer);
                        let normalized = self.normalize_span_whitespace(&text);
                        children.push(Span::new_text(normalized));
                        buffer.clear();
                    }
                    span.children = children;
                    span.strip_redundant_link_description();
                    return Ok(span);
                }
                Token::SelfClosingTag(tag) => {
                    if tag.name == "br" {
                        if !buffer.is_empty() {
                            let text = self.decode_entities(&buffer);
                            let normalized = self.normalize_span_whitespace(&text);
                            children.push(Span::new_text(format!("{}\n", normalized)));
                            buffer.clear();
                        } else if let Some(last) = children.last_mut() {
                            if last.style == InlineStyle::None && last.children.is_empty() {
                                last.text.push('\n');
                            } else {
                                children.push(Span::new_text("\n"));
                            }
                        } else {
                            children.push(Span::new_text("\n"));
                        }
                    } else {
                        return Err(ParseError::UnexpectedToken(format!(
                            "self-closing tag {}",
                            tag.name
                        )));
                    }
                }
                Token::StartTag(tag) => {
                    let tag_name = tag.name.clone();
                    if let Some(&child_style) = self.inline_elements.get(&tag_name) {
                        if !buffer.is_empty() {
                            let text = self.decode_entities(&buffer);
                            let normalized = self.normalize_span_whitespace(&text);
                            children.push(Span::new_text(normalized));
                            buffer.clear();
                        }
                        let child_span = self.read_span(tokenizer, child_style, tag)?;
                        children.push(child_span);
                    } else {
                        return Err(ParseError::NonInlineToken(tag_name));
                    }
                }
                Token::Text(text) => {
                    if !children.is_empty()
                        && children
                            .last()
                            .map(|s| s.text.ends_with('\n'))
                            .unwrap_or(false)
                    {
                        buffer.push_str(text.trim_start());
                    } else {
                        buffer.push_str(&text);
                    }
                }
                Token::EndTag(tag_name) => {
                    return Err(ParseError::UnexpectedToken(format!("end tag {}", tag_name)));
                }
            }
        }

        Err(ParseError::NoClosingTag(style))
    }

    fn collapse_whitespace(&self, s: &str, first: bool, last: bool) -> String {
        const FIGURE_SPACE_PLACEHOLDER: char = '\u{E000}';
        const NBSP_PLACEHOLDER: char = '\u{E001}';
        const FIGURE_SPACE_PLACEHOLDER_STR: &str = "\u{E000}";
        const NBSP_PLACEHOLDER_STR: &str = "\u{E001}";

        let mut result = s.replace('\u{2005}', FIGURE_SPACE_PLACEHOLDER_STR);
        result = result.replace('\u{00A0}', NBSP_PLACEHOLDER_STR);

        if first {
            result = result.trim_start().to_string();
        }
        if last {
            result = result.trim_end().to_string();
        }

        let result = self.space_regex.replace_all(&result, " ").to_string();

        result
            .replace(FIGURE_SPACE_PLACEHOLDER, "\u{2005}")
            .replace(NBSP_PLACEHOLDER, "\u{00A0}")
    }

    fn normalize_span_whitespace(&self, s: &str) -> String {
        // Only normalize if the text contains newlines (indicating HTML formatting whitespace)
        if s.contains('\n') {
            const FIGURE_SPACE_PLACEHOLDER: char = '\u{E000}';
            const NBSP_PLACEHOLDER: char = '\u{E001}';
            const FIGURE_SPACE_PLACEHOLDER_STR: &str = "\u{E000}";
            const NBSP_PLACEHOLDER_STR: &str = "\u{E001}";

            let mut masked = s.replace('\u{2005}', FIGURE_SPACE_PLACEHOLDER_STR);
            masked = masked.replace('\u{00A0}', NBSP_PLACEHOLDER_STR);

            // Collapse all whitespace (including newlines) to single spaces and trim
            let collapsed = self.space_regex.replace_all(&masked, " ");
            collapsed
                .trim()
                .replace(FIGURE_SPACE_PLACEHOLDER, "\u{2005}")
                .replace(NBSP_PLACEHOLDER, "\u{00A0}")
        } else {
            // If no newlines, preserve the text as-is (including intentional spaces)
            s.to_string()
        }
    }

    fn decode_entities(&self, s: &str) -> String {
        s.replace("&emsp14;", "\u{2005}")
            .replace("&nbsp;", "\u{00A0}")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&amp;", "&")
            .replace("&quot;", "\"")
            .replace("&apos;", "'")
    }

    fn trim_whitespace_with_entities(
        &self,
        mut spans: Vec<Span>,
        preserve_leading: bool,
        preserve_trailing: bool,
    ) -> Vec<Span> {
        // Trim leading whitespace (unless we need to preserve it due to entities)
        if !preserve_leading {
            loop {
                let action = match spans.first() {
                    Some(first)
                        if first.style == InlineStyle::None && first.children.is_empty() =>
                    {
                        let start_idx = trim_start_index_preserving_entities(&first.text);
                        if start_idx == 0 {
                            None
                        } else if start_idx >= first.text.len() {
                            Some(TrimLeadingAction::RemoveSpan)
                        } else {
                            Some(TrimLeadingAction::UpdateText(
                                first.text[start_idx..].to_string(),
                            ))
                        }
                    }
                    _ => None,
                };

                match action {
                    Some(TrimLeadingAction::RemoveSpan) => {
                        spans.remove(0);
                        continue;
                    }
                    Some(TrimLeadingAction::UpdateText(new_text)) => {
                        if let Some(first) = spans.first_mut() {
                            first.text = new_text;
                        }
                        break;
                    }
                    None => break,
                }
            }
        }

        // Trim trailing whitespace (unless we need to preserve it due to entities)
        if !preserve_trailing {
            loop {
                let action = match spans.last() {
                    Some(last) if last.style == InlineStyle::None && last.children.is_empty() => {
                        let end_idx = trim_end_index_preserving_entities(&last.text);
                        if end_idx == last.text.len() {
                            None
                        } else if end_idx == 0 {
                            Some(TrimTrailingAction::RemoveSpan)
                        } else {
                            Some(TrimTrailingAction::Truncate(end_idx))
                        }
                    }
                    _ => None,
                };

                match action {
                    Some(TrimTrailingAction::RemoveSpan) => {
                        spans.pop();
                        continue;
                    }
                    Some(TrimTrailingAction::Truncate(end_idx)) => {
                        if let Some(last) = spans.last_mut() {
                            last.text.truncate(end_idx);
                        }
                        break;
                    }
                    None => break,
                }
            }
        }

        spans
    }
}

enum TrimLeadingAction {
    RemoveSpan,
    UpdateText(String),
}

enum TrimTrailingAction {
    RemoveSpan,
    Truncate(usize),
}

const FIGURE_SPACE: char = '\u{2005}';
const NON_BREAKING_SPACE: char = '\u{00A0}';

fn is_preserved_entity_space(ch: char) -> bool {
    matches!(ch, FIGURE_SPACE | NON_BREAKING_SPACE)
}

fn trim_start_index_preserving_entities(text: &str) -> usize {
    let mut candidate = 0;
    let mut saw_trimmed_whitespace = false;

    for (idx, ch) in text.char_indices() {
        if is_preserved_entity_space(ch) {
            return idx;
        }
        if ch.is_whitespace() {
            candidate = idx + ch.len_utf8();
            saw_trimmed_whitespace = true;
            continue;
        }
        return idx;
    }

    if saw_trimmed_whitespace {
        candidate
    } else {
        0
    }
}

fn trim_end_index_preserving_entities(text: &str) -> usize {
    let mut end = text.len();
    let mut saw_trimmed_whitespace = false;

    for (idx, ch) in text.char_indices().rev() {
        if is_preserved_entity_space(ch) {
            return end;
        }
        if ch.is_whitespace() {
            end = idx;
            saw_trimmed_whitespace = true;
            continue;
        }
        return end;
    }

    if saw_trimmed_whitespace {
        end
    } else {
        text.len()
    }
}

fn trim_trailing_inline_whitespace(spans: &mut Vec<Span>) {
    while let Some(last) = spans.last_mut() {
        if last.style != InlineStyle::None
            || !last.children.is_empty()
            || last.link_target.is_some()
        {
            break;
        }

        let end_idx = trim_end_index_preserving_entities(&last.text);
        if end_idx == last.text.len() {
            break;
        }

        if end_idx == 0 {
            spans.pop();
        } else {
            last.text.truncate(end_idx);
            break;
        }
    }
}

fn normalize_entity_whitespace(document: &mut Document) {
    for paragraph in &mut document.paragraphs {
        normalize_paragraph_spaces(paragraph);
    }
}

fn normalize_paragraph_spaces(paragraph: &mut Paragraph) {
    if paragraph.is_leaf() {
        normalize_spans_spaces(paragraph.content_mut());
    }

    match paragraph {
        Paragraph::Quote { children } => {
            for child in children {
                normalize_paragraph_spaces(child);
            }
        }
        Paragraph::OrderedList { entries } | Paragraph::UnorderedList { entries } => {
            for entry in entries {
                for item in entry {
                    normalize_paragraph_spaces(item);
                }
            }
        }
        Paragraph::Checklist { .. }
        | Paragraph::Text { .. }
        | Paragraph::Header1 { .. }
        | Paragraph::Header2 { .. }
        | Paragraph::Header3 { .. }
        | Paragraph::CodeBlock { .. } => {}
    }
}

fn normalize_spans_spaces(spans: &mut [Span]) {
    for span in spans {
        if !span.text.is_empty() {
            span.text = span
                .text
                .replace('\u{E000}', " ")
                .replace('\u{E001}', "\u{00A0}")
                .replace('\u{2005}', " ");
        }

        if !span.children.is_empty() {
            normalize_spans_spaces(&mut span.children);
        }
    }
}

/// Parses FTML content from any [`Read`] implementor.
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// use tdoc::parser;
///
/// let mut input = Cursor::new("<p>Hello!</p>");
/// let document = parser::parse(&mut input).unwrap();
/// assert_eq!(document.paragraphs.len(), 1);
/// ```
pub fn parse<R: Read>(mut reader: R) -> Result<Document, ParseError> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let parser = Parser::new();
    parser.parse_string(&input)
}

#[cfg(test)]
mod tests {
    use crate::ftml;

    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_simple_paragraph() {
        let input = "<p>This is a test.</p>";
        let doc = parse(Cursor::new(input)).unwrap();

        assert_eq!(doc.paragraphs.len(), 1);
        assert_eq!(doc.paragraphs[0].paragraph_type(), ParagraphType::Text);
        assert_eq!(doc.paragraphs[0].content().len(), 1);
        assert_eq!(doc.paragraphs[0].content()[0].text, "This is a test.");
    }

    #[test]
    fn test_header_paragraph() {
        let input = "<h1>Header</h1>";
        let doc = parse(Cursor::new(input)).unwrap();

        assert_eq!(doc.paragraphs.len(), 1);
        assert_eq!(doc.paragraphs[0].paragraph_type(), ParagraphType::Header1);
        assert_eq!(doc.paragraphs[0].content()[0].text, "Header");
    }

    #[test]
    fn test_bold_text() {
        let input = "<p>This is <b>bold</b> text.</p>";
        let doc = parse(Cursor::new(input)).unwrap();

        assert_eq!(doc.paragraphs[0].content().len(), 3);
        assert_eq!(doc.paragraphs[0].content()[0].text, "This is ");
        assert_eq!(doc.paragraphs[0].content()[1].style, InlineStyle::Bold);
        assert_eq!(doc.paragraphs[0].content()[1].children[0].text, "bold");
        assert_eq!(doc.paragraphs[0].content()[2].text, " text.");
    }

    #[test]
    fn test_line_break() {
        let input = "<p>A<br/>B</p>";
        let doc = parse(Cursor::new(input)).unwrap();

        assert_eq!(doc.paragraphs[0].content().len(), 3);
        assert_eq!(doc.paragraphs[0].content()[0].text, "A");
        assert_eq!(doc.paragraphs[0].content()[1].text, "\n");
        assert_eq!(doc.paragraphs[0].content()[2].text, "B");
    }

    #[test]
    fn test_whitespace_handling() {
        fn doc(s: &str) -> Document {
            parse(Cursor::new(s)).unwrap()
        }
        assert_eq!(
            doc("<p> Hier kommt ein Test! </p>"),
            ftml! { p { "Hier kommt ein Test!" } }
        );

        assert_eq!(doc("<p> A   B </p>"), ftml! { p { "A B" } });

        assert_eq!(doc("<p> A&emsp14;&emsp14;B </p>"), ftml! { p { "A  B" } });
    }

    #[test]
    fn test_space_at_start_end() {
        fn doc(s: &str) -> Document {
            parse(Cursor::new(s)).unwrap()
        }
        assert_eq!(
            doc("<p>&emsp14;Start with space</p>"),
            ftml! { p { " Start with space" } }
        );

        assert_eq!(
            doc("<p>End with space&emsp14;</p>"),
            ftml! { p { "End with space " } }
        );

        assert_eq!(
            doc("<p>&emsp14;Surrounded by space&emsp14;</p>"),
            ftml! { p { " Surrounded by space " } }
        );
    }

    #[test]
    fn test_extra_space_at_start_end() {
        fn doc(s: &str) -> Document {
            parse(Cursor::new(s)).unwrap()
        }
        assert_eq!(
            doc("<p> &emsp14;Start with space </p>"),
            ftml! { p { " Start with space" } }
        );

        assert_eq!(
            doc("<p> End with space&emsp14; </p>"),
            ftml! { p { "End with space " } }
        );

        assert_eq!(
            doc("<p> &emsp14;Surrounded by space&emsp14; </p>"),
            ftml! { p { " Surrounded by space " } }
        );
    }

    #[test]
    fn test_footer_links() {
        assert_eq!(
            parse(Cursor::new("<p>\n  <a href=\"https://www.cnn.com/terms\">Terms of Use </a> |  <a href=\"https://www.cnn.com/privacy\">Privacy Policy </a> |  <a href=\"https://www.cnn.com/ad-choices\">Ad Choices </a> |  Cookie Settings&emsp14;\n</p>")).unwrap(),
            ftml! {
                p {
                    link { "https://www.cnn.com/terms" "Terms of Use " }
                    " | ",
                    link { "https://www.cnn.com/privacy" "Privacy Policy " }
                    " | ",
                    link { "https://www.cnn.com/ad-choices" "Ad Choices " }
                    " | Cookie Settings "
                }
            }
        );
    }

    #[test]
    fn test_link() {
        let input = "<p>See <a href=\"https://example.com\">Example</a></p>";
        let doc = parse(Cursor::new(input)).unwrap();

        assert_eq!(doc.paragraphs.len(), 1);
        let paragraph = &doc.paragraphs[0];
        assert_eq!(paragraph.content().len(), 2);
        assert_eq!(paragraph.content()[0].text, "See ");

        let link_span = &paragraph.content()[1];
        assert_eq!(link_span.style, InlineStyle::Link);
        assert_eq!(
            link_span.link_target.as_deref(),
            Some("https://example.com")
        );
        assert_eq!(link_span.children.len(), 1);
        assert_eq!(link_span.children[0].text, "Example");
    }

    #[test]
    fn test_link_without_description() {
        let input = "<p><a href=\"https://example.com\">https://example.com</a></p>";
        let doc = parse(Cursor::new(input)).unwrap();

        assert_eq!(doc.paragraphs.len(), 1);
        let paragraph = &doc.paragraphs[0];
        assert_eq!(paragraph.content().len(), 1);

        let link_span = &paragraph.content()[0];
        assert_eq!(link_span.style, InlineStyle::Link);
        assert_eq!(
            link_span.link_target.as_deref(),
            Some("https://example.com")
        );
        assert!(link_span.children.is_empty());
        assert!(link_span.text.is_empty());
    }

    #[test]
    fn test_link_href_decodes_entities() {
        let input = "<p><a href=\"https://example.com/path?foo=1&amp;bar=2\">Example</a></p>";
        let doc = parse(Cursor::new(input)).unwrap();

        let paragraph = &doc.paragraphs[0];
        let link_span = &paragraph.content()[0];
        assert_eq!(
            link_span.link_target.as_deref(),
            Some("https://example.com/path?foo=1&bar=2")
        );
    }

    #[test]
    fn test_space_before_link_is_preserved() {
        let input = "<p>Zugriff auf <a href=\"https://example.com\">Dienste</a></p>";
        let doc = parse(Cursor::new(input)).unwrap();

        let paragraph = &doc.paragraphs[0];
        assert_eq!(paragraph.content().len(), 2);
        assert_eq!(paragraph.content()[0].text, "Zugriff auf ");
        assert_eq!(
            paragraph.content()[1].link_target.as_deref(),
            Some("https://example.com")
        );
    }

    #[test]
    fn test_whitespace_edge_in_span() {
        fn doc(s: &str) -> Document {
            parse(Cursor::new(s)).unwrap()
        }
        assert_eq!(
            doc("<p><a href=\"yadayada\">Hier kommt ein Test! </a></p>\n"),
            ftml! { p { link { "yadayada" "Hier kommt ein Test! " } } },
        );
    }

    #[test]
    fn test_parse_code_block_no_trailing_newline() {
        let input = "<pre>\nhello\nworld\n</pre>";
        let parsed = parse(Cursor::new(input)).unwrap();
        assert_eq!(parsed.paragraphs.len(), 1);
        if let crate::Paragraph::CodeBlock { content } = &parsed.paragraphs[0] {
            assert_eq!(content.len(), 1);
            // Trailing newline should be stripped
            assert_eq!(content[0].text, "hello\nworld");
        } else {
            panic!("Expected code block");
        }
    }
}
