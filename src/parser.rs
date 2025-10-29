use crate::{Document, InlineStyle, Paragraph, ParagraphType, Span};
use regex::Regex;
use std::collections::HashMap;
use std::io::Read;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unexpected text content: {0}")]
    UnexpectedTextContent(String),
    #[error("Paragraphs not allowed inside leaf paragraph nodes when trying to add {new_type} below {parent_type}")]
    InvalidNesting { new_type: ParagraphType, parent_type: ParagraphType },
    #[error("Closing unopened paragraph of type {0}")]
    ClosingUnopenedParagraph(ParagraphType),
    #[error("Cannot close {actual} with {expected}")]
    MismatchedClosingTag { actual: ParagraphType, expected: ParagraphType },
    #[error("Unexpected list item, parent: {0:?}")]
    UnexpectedListItem(Option<ParagraphType>),
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
enum Token {
    Text(String),
    StartTag(String),
    EndTag(String),
    SelfClosingTag(String),
}

struct Tokenizer {
    input: String,
    pos: usize,
    putback_token: Option<(Token, usize)>,
}

impl Tokenizer {
    fn new(input: String) -> Self {
        Self { input, pos: 0, putback_token: None }
    }
    
    fn next(&mut self) -> Option<Token> {
        // Return putback token if available and restore position
        if let Some((token, saved_pos)) = self.putback_token.take() {
            self.pos = saved_pos;
            return Some(token);
        }
        
        self.skip_whitespace_between_tags();
        
        if self.pos >= self.input.len() {
            return None;
        }
        
        // Save position before parsing token
        let token_start_pos = self.pos;
        let token = if self.input.get(self.pos..self.pos+1) == Some("<") {
            self.parse_tag()
        } else {
            self.parse_text()
        };
        
        token
    }
    
    fn next_with_pos(&mut self) -> Option<(Token, usize)> {
        // Return putback token if available and restore position
        if let Some((token, saved_pos)) = self.putback_token.take() {
            self.pos = saved_pos;
            return Some((token, saved_pos));
        }
        
        self.skip_whitespace_between_tags();
        
        if self.pos >= self.input.len() {
            return None;
        }
        
        // Save position before parsing token
        let token_start_pos = self.pos;
        let token = if self.input.get(self.pos..self.pos+1) == Some("<") {
            self.parse_tag()
        } else {
            self.parse_text()
        };
        
        token.map(|t| (t, token_start_pos))
    }
    
    fn putback(&mut self, token: Token, start_pos: usize) {
        self.putback_token = Some((token, start_pos));
    }
    
    fn peek(&mut self) -> Option<Token> {
        if let Some((ref token, _pos)) = self.putback_token {
            return Some(token.clone());
        }
        
        // Create a temporary tokenizer to peek without affecting state
        let saved_pos = self.pos;
        self.skip_whitespace_between_tags();
        
        if self.pos >= self.input.len() {
            self.pos = saved_pos;
            return None;
        }
        
        let token = if self.input.get(self.pos..self.pos+1) == Some("<") {
            self.parse_tag()
        } else {
            self.parse_text()
        };
        
        // Reset position 
        self.pos = saved_pos;
        token
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
        if self.pos == start_pos {
            return;
        }
    }
    
    fn parse_tag(&mut self) -> Option<Token> {
        let start_pos = self.pos;
        self.pos += 1; // skip '<'
        
        let mut end_pos = self.pos;
        let mut in_quotes = false;
        let mut quote_char = '"';
        
        // Find the end of the tag (looking for '>')
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
        
        if tag_content.starts_with('/') {
            let tag_name = tag_content[1..].trim().split_whitespace().next().unwrap_or("").to_string();
            Some(Token::EndTag(tag_name))
        } else if tag_content.ends_with('/') || tag_content == "br" {
            let tag_name = tag_content.trim_end_matches('/').trim().split_whitespace().next().unwrap_or("").to_string();
            Some(Token::SelfClosingTag(tag_name))
        } else {
            let tag_name = tag_content.trim().split_whitespace().next().unwrap_or("").to_string();
            Some(Token::StartTag(tag_name))
        }
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

impl Parser {
    pub fn new() -> Self {
        let mut wrapper_elements = HashMap::new();
        wrapper_elements.insert("p".to_string(), ParagraphType::Text);
        wrapper_elements.insert("h1".to_string(), ParagraphType::Header1);
        wrapper_elements.insert("h2".to_string(), ParagraphType::Header2);
        wrapper_elements.insert("h3".to_string(), ParagraphType::Header3);
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

        Self {
            wrapper_elements,
            inline_elements,
            space_regex: Regex::new(r"\s+").unwrap(),
        }
    }

    pub fn parse_string(&self, input: &str) -> Result<Document, ParseError> {
        let mut tokenizer = Tokenizer::new(input.to_string());
        let mut document = Document::new();
        let mut breadcrumbs: Vec<Paragraph> = Vec::new();
        let mut list_item_level = 0;

        while let Some(token) = tokenizer.next() {
            self.process_token(token, &mut document, &mut breadcrumbs, &mut list_item_level, &mut tokenizer)?;
        }

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
            Token::StartTag(tag_name) => {
                if tag_name == "li" {
                    if let Some(parent) = breadcrumbs.last_mut() {
                        if parent.paragraph_type == ParagraphType::UnorderedList 
                            || parent.paragraph_type == ParagraphType::OrderedList {
                            let (list_content, remaining_token) = self.read_list_content(tokenizer)?;
                            parent.add_list_item(list_content);
                            
                            // If there's a remaining token (parent structure ending), handle it
                            if let Some(token) = remaining_token {
                                return self.process_token(token, document, breadcrumbs, list_item_level, tokenizer);
                            }
                            *list_item_level += 1;
                        } else {
                            return Err(ParseError::UnexpectedListItem(Some(parent.paragraph_type)));
                        }
                    } else {
                        return Err(ParseError::UnexpectedListItem(None));
                    }
                } else if let Some(&paragraph_type) = self.wrapper_elements.get(&tag_name) {
                    self.process_start_paragraph(paragraph_type, document, breadcrumbs, list_item_level, tokenizer)?;
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
                        if !parent.paragraph_type.is_leaf() && parent.paragraph_type != ParagraphType::UnorderedList && parent.paragraph_type != ParagraphType::OrderedList {
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
        list_item_level: &mut i32,
        tokenizer: &mut Tokenizer,
    ) -> Result<(), ParseError> {
        let mut paragraph = Paragraph::new(paragraph_type);
        
        if paragraph_type.is_leaf() {
            // Read content for leaf paragraphs
            let content = self.read_content(tokenizer, paragraph_type.html_tag())?;
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
            if current.paragraph_type != paragraph_type {
                return Err(ParseError::MismatchedClosingTag { 
                    actual: current.paragraph_type, 
                    expected: paragraph_type 
                });
            }
        } else {
            return Err(ParseError::ClosingUnopenedParagraph(paragraph_type));
        }

        let paragraph = breadcrumbs.pop().unwrap();
        
        // Add the completed paragraph to its parent or document
        if let Some(parent) = breadcrumbs.last_mut() {
            if parent.paragraph_type == ParagraphType::UnorderedList || parent.paragraph_type == ParagraphType::OrderedList {
                if let Some(last_entry) = parent.entries.last_mut() {
                    last_entry.push(paragraph);
                } else {
                    return Err(ParseError::ListContentWithoutItem);
                }
            } else {
                parent.children.push(paragraph);
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
        breadcrumbs: &mut Vec<Paragraph>,
    ) -> Result<(), ParseError> {
        if let Some(parent) = breadcrumbs.last_mut() {
            // If the parent is a list, add to the current list entry
            if parent.paragraph_type == ParagraphType::UnorderedList || parent.paragraph_type == ParagraphType::OrderedList {
                if let Some(last_entry) = parent.entries.last_mut() {
                    last_entry.push(paragraph.clone());
                } else {
                    return Err(ParseError::ListContentWithoutItem);
                }
            } else {
                // Otherwise add to the parent paragraph's children
                parent.children.push(paragraph.clone());
            }
        } else {
            document.add_paragraph(paragraph.clone());
        }
        Ok(())
    }

    fn read_list_content(&self, tokenizer: &mut Tokenizer) -> Result<(Vec<Paragraph>, Option<Token>), ParseError> {
        let mut paragraphs = Vec::new();
        let mut breadcrumbs: Vec<Paragraph> = Vec::new();

        while let Some(token) = tokenizer.next() {
            match token {
                Token::EndTag(tag_name) if tag_name == "li" => {
                    break;
                }
                Token::EndTag(ref tag_name) => {
                    // Check if this matches a breadcrumb (current structure ending)
                    if let Some(&paragraph_type) = self.wrapper_elements.get(tag_name) {
                        if let Some(paragraph) = breadcrumbs.last() {
                            if paragraph.paragraph_type == paragraph_type {
                                // This is our current structure ending - pop it and continue
                                let paragraph = breadcrumbs.pop().unwrap();
                                if !paragraph_type.is_leaf() {
                                    let mut paragraph_with_children = paragraph;
                                    paragraph_with_children.children.extend(paragraphs.drain(..));
                                    paragraphs.push(paragraph_with_children);
                                } else {
                                    paragraphs.push(paragraph);
                                }
                                continue;
                            }
                        }
                        // This is a parent structure ending - return it for parent to handle
                        paragraphs.extend(breadcrumbs);
                        return Ok((paragraphs, Some(token)));
                    } else {
                        return Err(ParseError::UnexpectedClosingTag(tag_name.clone()));
                    }
                }
                Token::StartTag(tag_name) => {
                    if tag_name == "li" {
                        if let Some(parent) = breadcrumbs.last_mut() {
                            if parent.paragraph_type == ParagraphType::UnorderedList 
                                || parent.paragraph_type == ParagraphType::OrderedList {
                                let (list_content, remaining_token) = self.read_list_content(tokenizer)?;
                                parent.add_list_item(list_content);
                                
                                // If there's a remaining token (parent structure ending), bubble it up
                                if let Some(token) = remaining_token {
                                    paragraphs.extend(breadcrumbs);
                                    return Ok((paragraphs, Some(token)));
                                }
                            } else {
                                return Err(ParseError::UnexpectedListItem(Some(parent.paragraph_type)));
                            }
                        } else {
                            return Err(ParseError::UnexpectedListItem(None));
                        }
                    } else if let Some(&paragraph_type) = self.wrapper_elements.get(&tag_name) {
                        let mut paragraph = Paragraph::new(paragraph_type);
                        
                        if paragraph_type.is_leaf() {
                            let content = self.read_content(tokenizer, paragraph_type.html_tag())?;
                            paragraph = paragraph.with_content(content);
                            paragraphs.push(paragraph);
                        } else {
                            breadcrumbs.push(paragraph);
                        }
                    } else {
                        return Err(ParseError::NonInlineToken(tag_name));
                    }
                }
                Token::Text(text) => {
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        return Err(ParseError::UnexpectedTextContent(trimmed.to_string()));
                    }
                }
                _ => {}
            }
        }

        paragraphs.extend(breadcrumbs);
        Ok((paragraphs, None))
    }

    fn read_content(&self, tokenizer: &mut Tokenizer, end_tag: &str) -> Result<Vec<Span>, ParseError> {
        let mut spans = Vec::new();
        let mut buffer = String::new();
        let mut has_leading_entity = false;
        let mut has_trailing_entity = false;

        while let Some((token, token_pos)) = tokenizer.next_with_pos() {
            match token {
                Token::EndTag(tag_name) if tag_name == end_tag => {
                    if !buffer.is_empty() {
                        spans.push(Span::new_text(self.decode_entities(&self.collapse_whitespace(&buffer, spans.is_empty(), true))));
                        buffer.clear();
                    }
                    break;
                }
                Token::SelfClosingTag(tag_name) if tag_name == "br" => {
                    if !buffer.is_empty() {
                        let decoded = self.decode_entities(&buffer);
                        spans.push(Span::new_text(self.collapse_whitespace(&decoded, spans.is_empty(), false)));
                        buffer.clear();
                    }
                    spans.push(Span::new_text("\n"));
                }
                Token::StartTag(tag_name) => {
                    if let Some(&style) = self.inline_elements.get(&tag_name) {
                        if !buffer.is_empty() {
                            spans.push(Span::new_text(self.decode_entities(&self.collapse_whitespace(&buffer, spans.is_empty(), false))));
                            buffer.clear();
                        }
                        let span = self.read_span(tokenizer, style, &tag_name)?;
                        spans.push(span);
                    } else if self.wrapper_elements.contains_key(&tag_name) {
                        // This is a structural element that should be handled by parent context
                        // Put the token back and return the content we've read so far
                        if !buffer.is_empty() {
                            spans.push(Span::new_text(self.decode_entities(&self.collapse_whitespace(&buffer, spans.is_empty(), false))));
                        }
                        tokenizer.putback(Token::StartTag(tag_name), token_pos);
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
                Token::SelfClosingTag(tag_name) => {
                    return Err(ParseError::UnexpectedToken(format!("self-closing tag {}", tag_name)));
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
            
            spans.push(Span::new_text(self.collapse_whitespace(&decoded, spans.is_empty() && !buffer_has_leading_entity, !buffer_has_trailing_entity)));
        }

        Ok(self.trim_whitespace_with_entities(spans, has_leading_entity, has_trailing_entity))
    }

    fn read_span(&self, tokenizer: &mut Tokenizer, style: InlineStyle, end_tag: &str) -> Result<Span, ParseError> {
        let mut children = Vec::new();
        let mut buffer = String::new();

        while let Some(token) = tokenizer.next() {
            match token {
                Token::EndTag(tag_name) if tag_name == end_tag => {
                    if !buffer.is_empty() {
                        let text = self.decode_entities(&buffer);
                        // Normalize whitespace in styled spans, but preserve line breaks
                        let normalized = self.normalize_span_whitespace(&text);
                        children.push(Span::new_text(normalized));
                    }
                    return Ok(Span::new_styled(style).with_children(children));
                }
                Token::SelfClosingTag(tag_name) if tag_name == "br" => {
                    if !buffer.is_empty() {
                        let text = self.decode_entities(&buffer);
                        let normalized = self.normalize_span_whitespace(&text);
                        children.push(Span::new_text(format!("{}\n", normalized)));
                        buffer.clear();
                    } else {
                        // If buffer is empty, just add newline to the last span if possible
                        if let Some(last) = children.last_mut() {
                            if last.style == InlineStyle::None && last.children.is_empty() {
                                last.text.push('\n');
                            } else {
                                children.push(Span::new_text("\n"));
                            }
                        } else {
                            children.push(Span::new_text("\n"));
                        }
                    }
                    
                    // Mark that we just processed a line break so the next text can be trimmed
                    // This will be handled by the Text token processing
                }
                Token::StartTag(tag_name) => {
                    if let Some(&child_style) = self.inline_elements.get(&tag_name) {
                        if !buffer.is_empty() {
                            let text = self.decode_entities(&buffer);
                            let normalized = self.normalize_span_whitespace(&text);
                            children.push(Span::new_text(normalized));
                            buffer.clear();
                        }
                        let child_span = self.read_span(tokenizer, child_style, &tag_name)?;
                        children.push(child_span);
                    } else {
                        return Err(ParseError::NonInlineToken(tag_name));
                    }
                }
                Token::Text(text) => {
                    // If we just added a line break, trim leading whitespace from the next text
                    if !children.is_empty() && children.last().map(|s| s.text.ends_with('\n')).unwrap_or(false) {
                        buffer.push_str(text.trim_start());
                    } else {
                        buffer.push_str(&text);
                    }
                }
                _ => {
                    return Err(ParseError::UnexpectedToken(format!("{:?}", token)));
                }
            }
        }

        Err(ParseError::NoClosingTag(style))
    }

    fn collapse_whitespace(&self, s: &str, first: bool, last: bool) -> String {
        let mut result = s.to_string();
        
        if first {
            result = result.trim_start().to_string();
        }
        if last {
            result = result.trim_end().to_string();
        }
        
        self.space_regex.replace_all(&result, " ").to_string()
    }

    fn normalize_span_whitespace(&self, s: &str) -> String {
        // Only normalize if the text contains newlines (indicating HTML formatting whitespace)
        if s.contains('\n') {
            // Collapse all whitespace (including newlines) to single spaces and trim
            let collapsed = self.space_regex.replace_all(s, " ");
            collapsed.trim().to_string()
        } else {
            // If no newlines, preserve the text as-is (including intentional spaces)
            s.to_string()
        }
    }

    fn decode_entities(&self, s: &str) -> String {
        s.replace("&emsp14;", " ")
         .replace("&lt;", "<")
         .replace("&gt;", ">")
         .replace("&amp;", "&")
         .replace("&quot;", "\"")
         .replace("&apos;", "'")
         .replace("&nbsp;", " ")
    }

    fn trim_whitespace(&self, mut spans: Vec<Span>) -> Vec<Span> {
        self.trim_whitespace_with_entities(spans, false, false)
    }

    fn trim_whitespace_with_entities(&self, mut spans: Vec<Span>, preserve_leading: bool, preserve_trailing: bool) -> Vec<Span> {
        // Trim leading whitespace (unless we need to preserve it due to entities)
        if !preserve_leading {
            while let Some(first) = spans.first_mut() {
                if first.style == InlineStyle::None && first.children.is_empty() {
                    let trimmed = first.text.trim_start();
                    if trimmed.is_empty() {
                        spans.remove(0);
                    } else if trimmed != first.text {
                        first.text = trimmed.to_string();
                        break;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        // Trim trailing whitespace (unless we need to preserve it due to entities)
        if !preserve_trailing {
            while let Some(last) = spans.last_mut() {
                if last.style == InlineStyle::None && last.children.is_empty() {
                    let trimmed = last.text.trim_end();
                    if trimmed.is_empty() {
                        spans.pop();
                    } else if trimmed != last.text {
                        last.text = trimmed.to_string();
                        break;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        spans
    }
}

pub fn parse<R: Read>(mut reader: R) -> Result<Document, ParseError> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;
    
    let parser = Parser::new();
    parser.parse_string(&input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_simple_paragraph() {
        let input = "<p>This is a test.</p>";
        let doc = parse(Cursor::new(input)).unwrap();
        
        assert_eq!(doc.paragraphs.len(), 1);
        assert_eq!(doc.paragraphs[0].paragraph_type, ParagraphType::Text);
        assert_eq!(doc.paragraphs[0].content.len(), 1);
        assert_eq!(doc.paragraphs[0].content[0].text, "This is a test.");
    }

    #[test]
    fn test_header_paragraph() {
        let input = "<h1>Header</h1>";
        let doc = parse(Cursor::new(input)).unwrap();
        
        assert_eq!(doc.paragraphs.len(), 1);
        assert_eq!(doc.paragraphs[0].paragraph_type, ParagraphType::Header1);
        assert_eq!(doc.paragraphs[0].content[0].text, "Header");
    }

    #[test]
    fn test_bold_text() {
        let input = "<p>This is <b>bold</b> text.</p>";
        let doc = parse(Cursor::new(input)).unwrap();
        
        assert_eq!(doc.paragraphs[0].content.len(), 3);
        assert_eq!(doc.paragraphs[0].content[0].text, "This is ");
        assert_eq!(doc.paragraphs[0].content[1].style, InlineStyle::Bold);
        assert_eq!(doc.paragraphs[0].content[1].children[0].text, "bold");
        assert_eq!(doc.paragraphs[0].content[2].text, " text.");
    }

    #[test]
    fn test_line_break() {
        let input = "<p>A<br/>B</p>";
        let doc = parse(Cursor::new(input)).unwrap();
        
        assert_eq!(doc.paragraphs[0].content.len(), 3);
        assert_eq!(doc.paragraphs[0].content[0].text, "A");
        assert_eq!(doc.paragraphs[0].content[1].text, "\n");
        assert_eq!(doc.paragraphs[0].content[2].text, "B");
    }
}
