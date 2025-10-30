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
        loop {
            match self.tokenizer.next_token() {
                Ok(token) => self.process_token(token)?,
                Err(TokenizerError::Eof) => break,
            }
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

        let (content, extra_token, closed) =
            self.read_content(end_tag.as_deref(), start_text)?;

        if para_type == ParagraphType::Quote && has_meaningful_content(&content) {
            let text_para = self.down(ParagraphType::Text)?;
            text_para.borrow_mut().content = content;
            let _ = self.up(ParagraphType::Text);
        } else if (para_type == ParagraphType::Text && !content.is_empty())
            || matches!(
                para_type,
                ParagraphType::Header1 | ParagraphType::Header2 | ParagraphType::Header3
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
                    .map_or(false, is_transparent_container_element)
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
                    let span = self.read_span(style, &name)?;
                    spans.add(span);
                }
                Token::EmptyElement(empty) => {
                    let name = lowercase_name(empty.name());
                    if should_skip_tag(&name) {
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

    fn read_span(&mut self, style: InlineStyle, end_tag: &str) -> Result<Span, HtmlError> {
        let mut children = Vec::new();
        let mut first = false;

        loop {
            let (text, token) = self.read_text()?;

            if !text.is_empty() {
                let collapsed = collapse_whitespace(&text, first, false);
                let decoded = decode_html(collapsed);
                if !decoded.is_empty() {
                    children.push(Span::new_text(decoded));
                }
            }

            let Some(token) = token else {
                return Ok(build_span(style, children));
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
                        return Ok(build_span(style, children));
                    }

                    let nested_style = inline_style_for(&name).unwrap_or(InlineStyle::None);
                    let span = self.read_span(nested_style, &name)?;
                    children.push(span);
                }
                Token::EndElement(end) => {
                    let name = lowercase_name(end.name());
                    if name == end_tag || is_block_level(&name) {
                        return Ok(build_span(style, children));
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
                ParagraphType::OrderedList | ParagraphType::UnorderedList => {
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
        if current_type != expected {
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
                ParagraphType::OrderedList | ParagraphType::UnorderedList => {
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
}

impl ParagraphBuilder {
    fn new(paragraph_type: ParagraphType) -> Self {
        Self {
            paragraph_type,
            children: Vec::new(),
            content: Vec::new(),
            entries: Vec::new(),
        }
    }

    fn start_new_list_item(&mut self) {
        self.entries.push(Vec::new());
    }

    fn ensure_current_list_item(&mut self) {
        if self.entries.is_empty() {
            self.entries.push(Vec::new());
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
        paragraph.entries = borrowed
            .entries
            .iter()
            .map(|entry| {
                entry
                    .iter()
                    .map(ParagraphBuilder::to_paragraph)
                    .collect::<Vec<_>>()
            })
            .collect();
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
        .get(0)
        .map(|span| !span.children.is_empty() || !span.text.trim().is_empty())
        .unwrap_or(false)
}

fn build_span(style: InlineStyle, children: Vec<Span>) -> Span {
    let mut span = Span::new_styled(style);
    span.children = children;
    span
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
        _ => None,
    }
}

fn is_block_level(tag: &str) -> bool {
    matches!(
        tag,
        "p"
            | "div"
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
