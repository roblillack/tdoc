//! Parse real-world HTML into the internal FTML representation.

pub mod gockl;

use crate::{Document, InlineStyle, Paragraph, ParagraphType, Span};
use gockl::{Token, Tokenizer, TokenizerError};
use html_escape::decode_html_entities;
use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;
use thiserror::Error;

const LINE_BREAK_ELEMENT_NAME: &str = "br";

type ParagraphNode = Rc<RefCell<ParagraphBuilder>>;

struct SpanOutcome {
    span: Span,
    had_visible_text: bool,
}

/// Parses a snippet of HTML into a [`Document`](crate::Document).
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// use tdoc::html;
///
/// let html = Cursor::new("<p>Hello</p>");
/// let document = html::parse(html).unwrap();
/// assert_eq!(document.paragraphs.len(), 1);
/// ```
pub fn parse<R: Read>(mut reader: R) -> crate::Result<Document> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    Parser::new(&input)
        .parse()
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)
}

struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    document: Vec<ParagraphNode>,
    breadcrumbs: Vec<ParagraphNode>,
    list_item_level: usize,
    skip_stack: Vec<String>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            tokenizer: Tokenizer::new(input),
            document: Vec::new(),
            breadcrumbs: Vec::new(),
            list_item_level: 0,
            skip_stack: Vec::new(),
        }
    }

    fn parse(mut self) -> Result<Document, HtmlError> {
        while let Ok(token) = self.tokenizer.next_token() {
            self.process_token(token)?;
        }

        let paragraphs = self
            .document
            .iter()
            .map(ParagraphBuilder::to_paragraph)
            .collect();

        Ok(Document { paragraphs })
    }

    fn process_token(&mut self, token: Token) -> Result<(), HtmlError> {
        if self.process_skipped_tags(&token) {
            return Ok(());
        }

        match token {
            Token::StartElement(start) => {
                let tag = lowercase_name(start.name());

                if tag == "li" {
                    let parent = match self.parent() {
                        Some(parent) => parent,
                        None => self.down(ParagraphType::UnorderedList)?,
                    };

                    let parent_type = {
                        let borrowed = parent.borrow();
                        borrowed.paragraph_type
                    };
                    if parent_type != ParagraphType::UnorderedList
                        && parent_type != ParagraphType::OrderedList
                        && parent_type != ParagraphType::Checklist
                    {
                        return Err(HtmlError::UnexpectedListItem);
                    }

                    parent.borrow_mut().start_new_list_item();
                    self.list_item_level += 1;
                    return Ok(());
                }

                if let Some(para_type) = paragraph_type_for(&tag) {
                    return self.read_paragraph(para_type, Some(tag), None);
                }
            }
            Token::EndElement(end) => {
                let tag = lowercase_name(end.name());

                if tag == "li" {
                    if self.list_item_level > 0 {
                        self.list_item_level -= 1;
                    }
                    return Ok(());
                }

                if let Some(para_type) = paragraph_type_for(&tag) {
                    if para_type.is_leaf() {
                        return Ok(());
                    }
                    return self.up(para_type);
                }
            }
            Token::Text(raw) => {
                if raw.trim().is_empty() {
                    return Ok(());
                }

                return self.read_paragraph(ParagraphType::Text, None, Some(raw));
            }
            _ => {}
        }

        Ok(())
    }

    fn read_paragraph(
        &mut self,
        para_type: ParagraphType,
        end_tag: Option<String>,
        start_text: Option<String>,
    ) -> Result<(), HtmlError> {
        let node = self.down(para_type)?;

        let (content, extra_token, closed) = if para_type == ParagraphType::CodeBlock {
            let (text, token, closed) =
                self.read_preformatted_content(end_tag.as_deref(), start_text)?;
            let spans = if text.is_empty() {
                Vec::new()
            } else {
                vec![Span::new_text(text)]
            };
            (spans, token, closed)
        } else {
            self.read_content(end_tag.as_deref(), start_text)?
        };

        if para_type == ParagraphType::Quote && has_meaningful_content(&content) {
            let text_para = self.down(ParagraphType::Text)?;
            text_para.borrow_mut().content = content;
            let _ = self.up(ParagraphType::Text);
        } else if (para_type == ParagraphType::Text && !content.is_empty())
            || matches!(
                para_type,
                ParagraphType::Header1
                    | ParagraphType::Header2
                    | ParagraphType::Header3
                    | ParagraphType::CodeBlock
            )
        {
            node.borrow_mut().content = content;
        }

        let should_remove_empty = if para_type.is_leaf() {
            let borrowed = node.borrow();
            let empty = borrowed.content.is_empty()
                && borrowed.children.is_empty()
                && borrowed.entries.is_empty();
            empty
                && end_tag
                    .as_deref()
                    .is_some_and(is_transparent_container_element)
        } else {
            false
        };

        if let Some(token) = extra_token {
            if should_remove_empty {
                self.remove_leaf(&node);
            }
            self.process_token(token)?;
            return Ok(());
        }

        if closed && !para_type.is_leaf() {
            self.up(para_type)?;
        }

        Ok(())
    }

    fn read_preformatted_content(
        &mut self,
        end_tag: Option<&str>,
        start_text: Option<String>,
    ) -> Result<(String, Option<Token>, bool), HtmlError> {
        let mut buffer = String::new();
        if let Some(text) = start_text {
            if !text.is_empty() {
                buffer.push_str(&decode_html(text));
            }
        }

        loop {
            let (text, token) = self.read_text()?;

            if !text.is_empty() {
                buffer.push_str(&decode_html(text));
            }

            let Some(token) = token else {
                return Ok((buffer, None, false));
            };

            if is_line_break(&token) {
                buffer.push('\n');
                continue;
            }

            if let Some(end_tag) = end_tag {
                if let Token::EndElement(ref end) = token {
                    if lowercase_name(end.name()) == end_tag {
                        return Ok((buffer, None, true));
                    }
                }
            }

            match token {
                Token::StartElement(start) => {
                    let name = lowercase_name(start.name());
                    if should_skip_tag(&name) {
                        continue;
                    }

                    if is_block_level(&name) {
                        return Ok((buffer, Some(Token::StartElement(start)), false));
                    }

                    let (nested_text, extra_token, closed) =
                        self.read_preformatted_content(Some(&name), None)?;
                    buffer.push_str(&nested_text);
                    if let Some(extra) = extra_token {
                        return Ok((buffer, Some(extra), closed));
                    }
                    if !closed {
                        continue;
                    }
                }
                Token::EmptyElement(empty) => {
                    let name = lowercase_name(empty.name());
                    if should_skip_tag(&name) {
                        continue;
                    }
                    if is_line_break_element(&name) {
                        buffer.push('\n');
                    }
                }
                Token::EndElement(end) => {
                    let name = lowercase_name(end.name());
                    if Some(name.as_str()) == end_tag {
                        return Ok((buffer, None, true));
                    }
                    if is_block_level(&name) {
                        return Ok((buffer, Some(Token::EndElement(end)), false));
                    }
                    return Ok((buffer, Some(Token::EndElement(end)), false));
                }
                _ => {}
            }
        }
    }

    fn read_content(
        &mut self,
        end_tag: Option<&str>,
        start_text: Option<String>,
    ) -> Result<(Vec<Span>, Option<Token>, bool), HtmlError> {
        let mut spans = BufferedSpanList::new();
        if let Some(text) = start_text {
            if !text.is_empty() {
                spans.add_text(text);
            }
        }

        loop {
            let (text, token) = self.read_text()?;

            if !text.is_empty() {
                spans.add_text(text);
            }

            let Some(token) = token else {
                return Ok((spans.close(), None, false));
            };

            if let Some(end_tag) = end_tag {
                if let Token::EndElement(ref end) = token {
                    if lowercase_name(end.name()) == end_tag {
                        return Ok((spans.close(), None, true));
                    }
                }
            }

            if is_line_break(&token) {
                spans.add_line_break();
                continue;
            }

            if let Some(element) = token.as_element() {
                let name = lowercase_name(element.name());
                if is_block_level(&name) {
                    return Ok((spans.close(), Some(token), false));
                }
            }

            match token {
                Token::StartElement(start) => {
                    let name = lowercase_name(start.name());
                    if should_skip_tag(&name) {
                        continue;
                    }

                    let style = inline_style_for(&name).unwrap_or(InlineStyle::None);
                    let link_target = if style == InlineStyle::Link {
                        start.attribute("href")
                    } else {
                        None
                    };
                    let outcome = self.read_span(style, &name, link_target)?;
                    if should_skip_link_span(&outcome.span, outcome.had_visible_text) {
                        continue;
                    }
                    spans.add(outcome.span);
                }
                Token::EmptyElement(empty) => {
                    let name = lowercase_name(empty.name());
                    if should_skip_tag(&name) {
                        continue;
                    }

                    if name == "input" {
                        let is_checkbox = empty
                            .attribute("type")
                            .map(|value| value.eq_ignore_ascii_case("checkbox"))
                            .unwrap_or(false);
                        if is_checkbox {
                            let checked = empty.attribute("checked").is_some();
                            self.mark_current_list_item_checkbox(checked);
                        }
                        continue;
                    }

                    if is_line_break_element(&name) {
                        spans.add_line_break();
                    }
                }
                _ => {}
            }
        }
    }

    fn read_span(
        &mut self,
        style: InlineStyle,
        end_tag: &str,
        link_target: Option<String>,
    ) -> Result<SpanOutcome, HtmlError> {
        let mut children = Vec::new();
        let mut first = false;
        let link_target = link_target.map(decode_html);
        let mut had_visible_text = false;

        loop {
            let (text, token) = self.read_text()?;

            if !text.is_empty() {
                let collapsed = collapse_whitespace(&text, first, false);
                let decoded = decode_html(collapsed);
                if !decoded.is_empty() {
                    if !decoded.trim().is_empty() {
                        had_visible_text = true;
                    }
                    children.push(Span::new_text(decoded));
                }
            }

            let Some(token) = token else {
                let span = build_span(style, children, link_target.clone());
                return Ok(SpanOutcome {
                    span,
                    had_visible_text,
                });
            };

            if is_line_break(&token) {
                if let Some(last) = children.last_mut() {
                    if last.children.is_empty() {
                        last.text.push('\n');
                    } else {
                        children.push(Span::new_text("\n"));
                    }
                } else {
                    children.push(Span::new_text("\n"));
                }
                first = true;
                continue;
            }

            match token {
                Token::StartElement(start) => {
                    let name = lowercase_name(start.name());
                    if is_block_level(&name) {
                        let span = build_span(style, children, link_target.clone());
                        return Ok(SpanOutcome {
                            span,
                            had_visible_text,
                        });
                    }

                    let nested_style = inline_style_for(&name).unwrap_or(InlineStyle::None);
                    let nested_link = if nested_style == InlineStyle::Link {
                        start.attribute("href")
                    } else {
                        None
                    };
                    let outcome = self.read_span(nested_style, &name, nested_link)?;
                    if should_skip_link_span(&outcome.span, outcome.had_visible_text) {
                        continue;
                    }
                    if outcome.had_visible_text {
                        had_visible_text = true;
                    }
                    children.push(outcome.span);
                }
                Token::EndElement(end) => {
                    let name = lowercase_name(end.name());
                    if name == end_tag || is_block_level(&name) {
                        let span = build_span(style, children, link_target.clone());
                        return Ok(SpanOutcome {
                            span,
                            had_visible_text,
                        });
                    }
                }
                _ => {}
            }
        }
    }

    fn read_text(&mut self) -> Result<(String, Option<Token>), HtmlError> {
        let mut buffer = String::new();

        loop {
            let token = match self.tokenizer.next_token() {
                Ok(token) => token,
                Err(TokenizerError::Eof) => return Ok((buffer, None)),
            };

            if self.process_skipped_tags(&token) {
                continue;
            }

            if let Token::Text(raw) = &token {
                buffer.push_str(raw);
                continue;
            }

            return Ok((buffer, Some(token)));
        }
    }

    fn down(&mut self, para_type: ParagraphType) -> Result<ParagraphNode, HtmlError> {
        let node = Rc::new(RefCell::new(ParagraphBuilder::new(para_type)));

        if let Some(parent) = self.parent() {
            let parent_type = {
                let borrowed = parent.borrow();
                if borrowed.paragraph_type.is_leaf() {
                    return Err(HtmlError::InvalidParagraphNesting {
                        parent: borrowed.paragraph_type,
                        child: para_type,
                    });
                }
                borrowed.paragraph_type
            };

            match parent_type {
                ParagraphType::OrderedList
                | ParagraphType::UnorderedList
                | ParagraphType::Checklist => {
                    let mut parent_mut = parent.borrow_mut();
                    parent_mut.ensure_current_list_item();
                    parent_mut
                        .entries
                        .last_mut()
                        .expect("list entry present")
                        .push(Rc::clone(&node));
                }
                _ => parent.borrow_mut().children.push(Rc::clone(&node)),
            }
        } else {
            self.document.push(Rc::clone(&node));
        }

        if !para_type.is_leaf() {
            self.breadcrumbs.push(Rc::clone(&node));
        }

        Ok(node)
    }

    fn up(&mut self, expected: ParagraphType) -> Result<(), HtmlError> {
        let Some(current) = self.breadcrumbs.last() else {
            return Ok(());
        };

        let current_type = current.borrow().paragraph_type;
        if !current_type.matches_closing_tag(expected) {
            return Err(HtmlError::ParagraphCloseMismatch {
                expected,
                found: current_type,
            });
        }

        self.breadcrumbs.pop();
        Ok(())
    }

    fn parent(&self) -> Option<ParagraphNode> {
        self.breadcrumbs.last().cloned()
    }

    fn remove_leaf(&mut self, node: &ParagraphNode) {
        if let Some(parent) = self.parent() {
            let parent_type = parent.borrow().paragraph_type;

            match parent_type {
                ParagraphType::OrderedList
                | ParagraphType::UnorderedList
                | ParagraphType::Checklist => {
                    let mut parent_mut = parent.borrow_mut();
                    if let Some(entry) = parent_mut.entries.last_mut() {
                        if let Some(last) = entry.last() {
                            if Rc::ptr_eq(last, node) {
                                entry.pop();
                            }
                        }
                    }
                }
                _ => {
                    let mut parent_mut = parent.borrow_mut();
                    if let Some(last) = parent_mut.children.last() {
                        if Rc::ptr_eq(last, node) {
                            parent_mut.children.pop();
                        }
                    }
                }
            }
        } else if let Some(last) = self.document.last() {
            if Rc::ptr_eq(last, node) {
                self.document.pop();
            }
        }
    }

    fn mark_current_list_item_checkbox(&mut self, checked: bool) {
        if let Some(list_node) = self
            .breadcrumbs
            .iter()
            .rev()
            .find(|node| {
                matches!(
                    node.borrow().paragraph_type,
                    ParagraphType::UnorderedList
                        | ParagraphType::OrderedList
                        | ParagraphType::Checklist
                )
            })
            .cloned()
        {
            list_node
                .borrow_mut()
                .mark_current_list_item_checkbox(checked);
        }
    }

    fn process_skipped_tags(&mut self, token: &Token) -> bool {
        if !self.skip_stack.is_empty() {
            match token {
                Token::StartElement(start) => {
                    self.skip_stack.push(lowercase_name(start.name()));
                }
                Token::EndElement(end) => {
                    let name = lowercase_name(end.name());
                    if let Some(pos) = self.skip_stack.iter().rposition(|item| item == &name) {
                        self.skip_stack.truncate(pos);
                    }
                }
                _ => {}
            }

            return true;
        }

        match token {
            Token::StartElement(start) => {
                let name = lowercase_name(start.name());
                if should_skip_tag(&name) {
                    self.skip_stack.push(name);
                    return true;
                }
            }
            Token::EmptyElement(empty) => {
                let name = lowercase_name(empty.name());
                if should_skip_tag(&name) {
                    return true;
                }
            }
            _ => {}
        }

        false
    }
}

#[derive(Debug, Error)]
enum HtmlError {
    #[error(
        "paragraphs not allowed inside leaf paragraph nodes when trying to add {child:?} below {parent:?}"
    )]
    InvalidParagraphNesting {
        parent: ParagraphType,
        child: ParagraphType,
    },
    #[error("unexpected list item outside list context")]
    UnexpectedListItem,
    #[error("cannot close {found:?} with {expected:?}")]
    ParagraphCloseMismatch {
        expected: ParagraphType,
        found: ParagraphType,
    },
}

#[derive(Debug)]
struct ParagraphBuilder {
    paragraph_type: ParagraphType,
    children: Vec<ParagraphNode>,
    content: Vec<Span>,
    entries: Vec<Vec<ParagraphNode>>,
    checklist_states: Vec<Option<bool>>,
}

impl ParagraphBuilder {
    fn new(paragraph_type: ParagraphType) -> Self {
        Self {
            paragraph_type,
            children: Vec::new(),
            content: Vec::new(),
            entries: Vec::new(),
            checklist_states: Vec::new(),
        }
    }

    fn start_new_list_item(&mut self) {
        self.entries.push(Vec::new());
        self.checklist_states.push(None);
    }

    fn ensure_current_list_item(&mut self) {
        if self.entries.is_empty() {
            self.entries.push(Vec::new());
            self.checklist_states.push(None);
        }
    }

    fn mark_current_list_item_checkbox(&mut self, checked: bool) {
        self.paragraph_type = ParagraphType::Checklist;
        self.ensure_current_list_item();
        if let Some(state) = self.checklist_states.last_mut() {
            *state = Some(checked);
        }
    }

    fn to_paragraph(node: &ParagraphNode) -> Paragraph {
        let borrowed = node.borrow();
        let mut paragraph = Paragraph::new(borrowed.paragraph_type);
        paragraph.content = borrowed.content.clone();
        paragraph.children = borrowed
            .children
            .iter()
            .map(ParagraphBuilder::to_paragraph)
            .collect();

        let entries: Vec<Vec<Paragraph>> = borrowed
            .entries
            .iter()
            .map(|entry| {
                entry
                    .iter()
                    .map(ParagraphBuilder::to_paragraph)
                    .collect::<Vec<_>>()
            })
            .collect();

        let is_checklist = borrowed.paragraph_type == ParagraphType::Checklist
            || (!borrowed.checklist_states.is_empty()
                && borrowed
                    .checklist_states
                    .iter()
                    .all(|state| state.is_some()));

        if is_checklist {
            paragraph = Paragraph::new_checklist();
            let mut converted_entries = Vec::new();
            for (idx, entry) in entries.into_iter().enumerate() {
                if let Some(Some(checked)) = borrowed.checklist_states.get(idx) {
                    let mut item = Paragraph::new_checklist_item(*checked);
                    let mut content = Vec::new();
                    for (idx, child) in entry.into_iter().enumerate() {
                        if child.content.is_empty() {
                            continue;
                        }

                        if idx > 0 && !content.is_empty() {
                            content.push(Span::new_text("\n"));
                        }

                        content.extend(child.content.into_iter());
                    }

                    trim_trailing_inline_whitespace(&mut content);

                    item.content = content;
                    converted_entries.push(vec![item]);
                } else {
                    converted_entries.push(entry);
                }
            }
            paragraph.entries = converted_entries;
        } else {
            paragraph.entries = entries;
        }

        paragraph
    }
}

struct BufferedSpanList {
    spans: Vec<Span>,
    first: bool,
    trim_end: bool,
    buffer: String,
}

impl BufferedSpanList {
    fn new() -> Self {
        Self {
            spans: Vec::new(),
            first: true,
            trim_end: false,
            buffer: String::new(),
        }
    }

    fn add_line_break(&mut self) {
        self.trim_end = true;
        self.flush();
        self.spans.push(Span::new_text("\n"));
        self.first = true;
    }

    fn add(&mut self, span: Span) {
        self.flush();
        self.first = span.ends_with_line_break();
        self.trim_end = false;
        self.spans.push(span);
    }

    fn add_text(&mut self, text: String) {
        self.trim_end = false;
        self.flush();
        self.buffer = text;
    }

    fn close(mut self) -> Vec<Span> {
        self.trim_end = true;
        self.flush();
        self.spans
    }

    fn flush(&mut self) {
        if self.buffer.is_empty() {
            return;
        }

        let collapsed = collapse_whitespace(&self.buffer, self.first, self.trim_end);
        let decoded = decode_html(collapsed);

        if !decoded.is_empty() {
            self.spans.push(Span::new_text(decoded));
        }

        self.buffer.clear();
        self.first = false;
    }
}

fn decode_html(input: String) -> String {
    decode_html_entities(&input).into_owned()
}

fn collapse_whitespace(input: &str, first: bool, last: bool) -> String {
    let mut slice = input;
    if first {
        slice = slice.trim_start_matches(|c: char| c.is_whitespace());
    }
    if last {
        slice = slice.trim_end_matches(|c: char| c.is_whitespace());
    }

    let mut result = String::new();
    let mut prev_space = false;

    for ch in slice.chars() {
        if ch.is_whitespace() {
            if !prev_space {
                result.push(' ');
                prev_space = true;
            }
        } else {
            result.push(ch);
            prev_space = false;
        }
    }

    result
}

fn has_meaningful_content(spans: &[Span]) -> bool {
    if spans.len() > 1 {
        return true;
    }

    spans
        .first()
        .map(|span| !span.children.is_empty() || !span.text.trim().is_empty())
        .unwrap_or(false)
}

fn build_span(style: InlineStyle, children: Vec<Span>, link_target: Option<String>) -> Span {
    if style == InlineStyle::Link {
        if let Some(target) = link_target {
            let trimmed = target.trim();
            if trimmed.is_empty() || trimmed == "#" {
                return collapse_link_children(children);
            }

            let mut span = Span::new_styled(InlineStyle::Link);
            span.children = children;
            if trimmed == target {
                span.link_target = Some(target);
            } else {
                span.link_target = Some(trimmed.to_string());
            }
            span.strip_redundant_link_description();
            return span;
        } else {
            return collapse_link_children(children);
        }
    }

    let mut span = Span::new_styled(style);
    span.children = children;
    span.link_target = link_target;
    span
}

fn collapse_link_children(mut children: Vec<Span>) -> Span {
    match children.len() {
        0 => Span::new_styled(InlineStyle::None),
        1 => children.pop().unwrap(),
        _ => Span::new_styled(InlineStyle::None).with_children(children),
    }
}

fn should_skip_link_span(span: &Span, had_visible_text: bool) -> bool {
    span.style == InlineStyle::Link && span.link_target.is_some() && !had_visible_text
}

fn lowercase_name(name: &str) -> String {
    name.chars().flat_map(|c| c.to_lowercase()).collect()
}

fn paragraph_type_for(tag: &str) -> Option<ParagraphType> {
    match tag {
        "p" => Some(ParagraphType::Text),
        "h1" => Some(ParagraphType::Header1),
        "h2" => Some(ParagraphType::Header2),
        "h3" => Some(ParagraphType::Header3),
        "pre" => Some(ParagraphType::CodeBlock),
        "blockquote" => Some(ParagraphType::Quote),
        "ul" => Some(ParagraphType::UnorderedList),
        "ol" => Some(ParagraphType::OrderedList),
        _ => None,
    }
}

fn inline_style_for(tag: &str) -> Option<InlineStyle> {
    match tag {
        "b" | "strong" => Some(InlineStyle::Bold),
        "i" | "em" => Some(InlineStyle::Italic),
        "u" => Some(InlineStyle::Underline),
        "s" | "del" | "strike" => Some(InlineStyle::Strike),
        "mark" => Some(InlineStyle::Highlight),
        "code" | "tt" => Some(InlineStyle::Code),
        "a" => Some(InlineStyle::Link),
        _ => None,
    }
}

fn is_block_level(tag: &str) -> bool {
    matches!(
        tag,
        "p" | "div"
            | "h1"
            | "h2"
            | "h3"
            | "blockquote"
            | "ul"
            | "ol"
            | "li"
            | "hr"
            | "tr"
            | "table"
    )
}

fn should_skip_tag(tag: &str) -> bool {
    matches!(tag, "title" | "style" | "script" | "nav")
}

fn is_line_break(token: &Token) -> bool {
    match token {
        Token::StartElement(start) => is_line_break_element(&lowercase_name(start.name())),
        Token::EmptyElement(empty) => is_line_break_element(&lowercase_name(empty.name())),
        _ => false,
    }
}

fn is_line_break_element(tag: &str) -> bool {
    tag == LINE_BREAK_ELEMENT_NAME
}

fn is_transparent_container_element(tag: &str) -> bool {
    matches!(tag, "div")
}

fn trim_trailing_inline_whitespace(spans: &mut Vec<Span>) {
    while let Some(last) = spans.last_mut() {
        if last.style != InlineStyle::None || !last.children.is_empty() || last.link_target.is_some() {
            break;
        }

        let trimmed = last.text.trim_end();
        if trimmed.len() == last.text.len() {
            break;
        }

        if trimmed.is_empty() {
            spans.pop();
        } else {
            last.text = trimmed.to_string();
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn skips_link_without_description() {
        let input = "<p><a href=\"https://example.com\"></a></p>";
        let document = parse(Cursor::new(input)).unwrap();

        assert_eq!(document.paragraphs.len(), 1);
        let paragraph = &document.paragraphs[0];
        assert_eq!(paragraph.paragraph_type, ParagraphType::Text);
        assert!(paragraph.content.is_empty());
    }

    #[test]
    fn keeps_link_when_description_matches_target() {
        let input = "<p><a href=\"https://example.com\">https://example.com</a></p>";
        let document = parse(Cursor::new(input)).unwrap();

        assert_eq!(document.paragraphs.len(), 1);
        let paragraph = &document.paragraphs[0];
        assert_eq!(paragraph.paragraph_type, ParagraphType::Text);
        assert_eq!(paragraph.content.len(), 1);

        let span = &paragraph.content[0];
        assert_eq!(span.style, InlineStyle::Link);
        assert_eq!(span.link_target.as_deref(), Some("https://example.com"));
        assert!(span.children.is_empty());
        assert!(span.text.is_empty());
    }

    #[test]
    fn ignores_empty_link_targets() {
        let input = "<p><a href=\"\">Example</a></p>";
        let document = parse(Cursor::new(input)).unwrap();

        let paragraph = &document.paragraphs[0];
        assert_eq!(paragraph.content.len(), 1);

        let span = &paragraph.content[0];
        assert_eq!(span.style, InlineStyle::None);
        assert!(span.link_target.is_none());
        assert_eq!(span.text, "Example");
    }

    #[test]
    fn ignores_hash_link_targets() {
        let input = "<p><a href=\"#\">Anchor label</a></p>";
        let document = parse(Cursor::new(input)).unwrap();

        let paragraph = &document.paragraphs[0];
        assert_eq!(paragraph.content.len(), 1);

        let span = &paragraph.content[0];
        assert_eq!(span.style, InlineStyle::None);
        assert!(span.link_target.is_none());
        assert_eq!(span.text, "Anchor label");
    }
}
