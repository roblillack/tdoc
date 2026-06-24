//! Parse and emit HTML.
//!
//! Unlike [`crate::ftml`], which flattens tables to paragraphs, [`write`]
//! emits real `<table>` markup so HTML output retains the tabular structure.
//! The two writers share the inline rendering logic; only table handling
//! differs.

pub mod gockl;

use crate::ftml::Writer;
use crate::{
    ChecklistItem, Document, InlineStyle, Paragraph, ParagraphType, Span, TableCell, TableRow,
};
use gockl::{StartElementToken, Token, Tokenizer, TokenizerError};
use html_escape::decode_html_entities;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::{Read, Write};
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
    pending_token: Option<Token>,
    /// Tokens to consume before the live tokenizer. Used to replay a buffered
    /// `<table>` body when it turns out to be layout scaffolding.
    injected: VecDeque<Token>,
    /// While `true`, [`Parser::pull_token`] stops at the end of `injected`
    /// instead of falling through to the live tokenizer, keeping a replay
    /// bounded to its buffered tokens.
    replaying: bool,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            tokenizer: Tokenizer::new(input),
            document: Vec::new(),
            breadcrumbs: Vec::new(),
            list_item_level: 0,
            skip_stack: Vec::new(),
            pending_token: None,
            injected: VecDeque::new(),
            replaying: false,
        }
    }

    /// Returns the next token from, in order: the single put-back slot, the
    /// replay buffer, and finally the live tokenizer. During a bounded replay
    /// (`replaying`) the live tokenizer is never touched.
    fn pull_token(&mut self) -> Result<Token, TokenizerError> {
        if let Some(token) = self.pending_token.take() {
            return Ok(token);
        }
        if let Some(token) = self.injected.pop_front() {
            return Ok(token);
        }
        if self.replaying {
            return Err(TokenizerError::Eof);
        }
        self.tokenizer.next_token()
    }

    fn parse(mut self) -> Result<Document, HtmlError> {
        while let Ok(token) = self.tokenizer.next_token() {
            self.process_token(token)?;
        }

        let mut paragraphs = self
            .document
            .iter()
            .map(ParagraphBuilder::to_paragraph)
            .collect::<Vec<_>>();

        paragraphs.retain(|paragraph| !is_empty_list(paragraph));

        Ok(Document {
            metadata: None,
            paragraphs,
        })
    }

    fn process_token(&mut self, token: Token) -> Result<(), HtmlError> {
        if self.process_skipped_tags(&token) {
            return Ok(());
        }

        match token {
            Token::StartElement(start) => {
                let tag = lowercase_name(start.name());

                if tag == "table" {
                    return self.read_table(&start);
                }

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

                if inline_style_for(&tag).is_some() {
                    self.pending_token = Some(Token::StartElement(start));
                    return self.read_paragraph(ParagraphType::Text, None, None);
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

                // Stray table-structure closing tags are benign once the
                // dedicated reader has consumed its `<table>`.
                if matches!(
                    tag.as_str(),
                    "table" | "thead" | "tbody" | "tfoot" | "tr" | "td" | "th"
                ) {
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

        let (mut content, extra_token, closed) = if para_type == ParagraphType::CodeBlock {
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

        if para_type != ParagraphType::CodeBlock {
            trim_trailing_line_breaks(&mut content);
        }

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
            borrowed.content.is_empty()
                && borrowed.children.is_empty()
                && borrowed.entries.is_empty()
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

        if should_remove_empty {
            self.remove_leaf(&node);
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
                    if should_skip_link_span(&outcome.span, outcome.had_visible_text)
                        || should_skip_empty_styled_span(&outcome.span)
                    {
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
        let mut first = true;
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
                    if let Some(last) = children.last() {
                        first = last.ends_with_line_break();
                    }
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
                    if should_skip_link_span(&outcome.span, outcome.had_visible_text)
                        || should_skip_empty_styled_span(&outcome.span)
                    {
                        continue;
                    }
                    if outcome.had_visible_text {
                        had_visible_text = true;
                    }
                    children.push(outcome.span);
                    if let Some(last) = children.last() {
                        first = last.ends_with_line_break();
                    }
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

    fn read_table(&mut self, start: &StartElementToken) -> Result<(), HtmlError> {
        // Buffer the whole table once. We need the tokens twice: to build rows
        // for the keep-or-flatten decision, and — if the table is layout
        // scaffolding — to replay the cell contents as ordinary block flow.
        let body = self.collect_table_tokens()?;
        let rows = self.rows_from_tokens(body.clone())?;

        if is_genuine_table(start, &rows) {
            let node = self.down(ParagraphType::Table)?;
            node.borrow_mut().table_rows = rows;
            self.up(ParagraphType::Table)?;
        } else {
            // The `<table>` is layout scaffolding (presentational role, a single
            // row/column, or no real content) rather than tabular data. HTML
            // mail in particular nests dozens of such tables purely for spacing.
            // Drop the structure and re-parse the contents as normal block flow:
            // `<tr>`/`<td>`/`<th>` carry no semantics of their own and are
            // ignored, so lists, headings, and paragraphs survive intact —
            // exactly how documents read before table support existed.
            self.replay_tokens(body)?;
        }

        Ok(())
    }

    /// Consumes the current `<table>` (its opening tag already read) and returns
    /// every token up to, but excluding, the matching `</table>`. Nested tables
    /// are retained so they can be re-evaluated on their own merits on replay.
    fn collect_table_tokens(&mut self) -> Result<Vec<Token>, HtmlError> {
        let mut buffer = Vec::new();
        let mut depth = 1usize;

        while let Ok(token) = self.pull_token() {
            match &token {
                Token::EndElement(end) if lowercase_name(end.name()) == "table" => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                Token::StartElement(start) if lowercase_name(start.name()) == "table" => {
                    depth += 1;
                }
                _ => {}
            }

            buffer.push(token);
        }

        Ok(buffer)
    }

    /// Builds the table rows from a buffered body without disturbing the live
    /// token stream. Used only to decide whether the table is genuine.
    fn rows_from_tokens(&mut self, tokens: Vec<Token>) -> Result<Vec<TableRow>, HtmlError> {
        self.run_over_tokens(tokens, |parser| parser.read_table_body())
    }

    /// Re-runs buffered tokens through the normal block-flow parser, so a layout
    /// table's contents become regular paragraphs, lists, and headings.
    fn replay_tokens(&mut self, tokens: Vec<Token>) -> Result<(), HtmlError> {
        self.run_over_tokens(tokens, |parser| {
            while let Ok(token) = parser.pull_token() {
                parser.process_token(token)?;
            }
            Ok(())
        })
    }

    /// Runs `f` with `tokens` queued as the sole token source (the live
    /// tokenizer is fenced off via `replaying`), restoring the previous token
    /// state afterwards. This makes the helper reentrant for nested tables.
    fn run_over_tokens<T>(
        &mut self,
        tokens: Vec<Token>,
        f: impl FnOnce(&mut Self) -> Result<T, HtmlError>,
    ) -> Result<T, HtmlError> {
        let saved_injected = std::mem::replace(&mut self.injected, VecDeque::from(tokens));
        let saved_pending = self.pending_token.take();
        let saved_replaying = self.replaying;
        self.replaying = true;

        let result = f(self);

        self.injected = saved_injected;
        self.pending_token = saved_pending;
        self.replaying = saved_replaying;

        result
    }

    fn read_table_body(&mut self) -> Result<Vec<TableRow>, HtmlError> {
        let mut rows = Vec::new();

        loop {
            let Some(token) = self.next_table_token()? else {
                return Ok(rows);
            };

            match token {
                Token::StartElement(start) => {
                    let name = lowercase_name(start.name());
                    match name.as_str() {
                        "thead" | "tbody" | "tfoot" | "colgroup" | "caption" | "col" => {}
                        "tr" => {
                            let row = self.read_table_row()?;
                            if !row.cells.is_empty() {
                                rows.push(row);
                            }
                        }
                        "th" | "td" => {
                            // Implicit row for orphan cells.
                            let mut row = TableRow::new();
                            let is_header = name == "th";
                            let cell = self.read_table_cell(is_header, &name)?;
                            row.cells.push(cell);
                            let mut trailing = self.read_table_row()?;
                            row.cells.append(&mut trailing.cells);
                            if !row.cells.is_empty() {
                                rows.push(row);
                            }
                        }
                        _ => {}
                    }
                }
                Token::EndElement(end) => {
                    let name = lowercase_name(end.name());
                    if name == "table" {
                        return Ok(rows);
                    }
                }
                _ => {}
            }
        }
    }

    fn read_table_row(&mut self) -> Result<TableRow, HtmlError> {
        let mut row = TableRow::new();

        loop {
            let Some(token) = self.next_table_token()? else {
                return Ok(row);
            };

            match token {
                Token::StartElement(start) => {
                    let name = lowercase_name(start.name());
                    match name.as_str() {
                        "th" => {
                            let cell = self.read_table_cell(true, &name)?;
                            row.cells.push(cell);
                        }
                        "td" => {
                            let cell = self.read_table_cell(false, &name)?;
                            row.cells.push(cell);
                        }
                        "tr" => {
                            // Missing `</tr>`; yield control to the table reader.
                            self.pending_token = Some(Token::StartElement(start));
                            return Ok(row);
                        }
                        _ => {}
                    }
                }
                Token::EndElement(end) => {
                    let name = lowercase_name(end.name());
                    if name == "tr" {
                        return Ok(row);
                    }
                    if matches!(name.as_str(), "table" | "thead" | "tbody" | "tfoot") {
                        self.pending_token = Some(Token::EndElement(end));
                        return Ok(row);
                    }
                }
                _ => {}
            }
        }
    }

    fn read_table_cell(&mut self, is_header: bool, end_tag: &str) -> Result<TableCell, HtmlError> {
        // Real-world HTML frequently wraps cell content in `<div>`, `<p>`, and
        // similar block-level containers. Flatten those wrappers so the cell
        // ends up with the combined inline content rather than an empty shell.
        let mut content: Vec<Span> = Vec::new();

        loop {
            let (mut spans, extra, closed) = self.read_content(Some(end_tag), None)?;

            if !spans.is_empty() {
                if !content.is_empty() {
                    content.push(Span::new_text("\n"));
                }
                content.append(&mut spans);
            }

            if closed {
                break;
            }

            let Some(token) = extra else {
                break;
            };

            let name = match &token {
                Token::StartElement(e) => lowercase_name(e.name()),
                Token::EndElement(e) => lowercase_name(e.name()),
                _ => {
                    self.pending_token = Some(token);
                    break;
                }
            };

            if matches!(
                name.as_str(),
                "td" | "th" | "tr" | "table" | "thead" | "tbody" | "tfoot"
            ) {
                self.pending_token = Some(token);
                break;
            }

            // Other block-level wrappers (div, p, li, h1..h3, blockquote, ul,
            // ol, hr) are dropped; their inline text keeps flowing into the
            // current cell.
        }

        trim_trailing_line_breaks(&mut content);
        trim_trailing_inline_whitespace(&mut content);

        Ok(TableCell { is_header, content })
    }

    fn next_table_token(&mut self) -> Result<Option<Token>, HtmlError> {
        loop {
            let token = match self.pull_token() {
                Ok(token) => token,
                Err(TokenizerError::Eof) => return Ok(None),
            };

            if self.process_skipped_tags(&token) {
                continue;
            }

            if let Token::Text(ref raw) = token {
                if raw.trim().is_empty() {
                    continue;
                }
            }

            return Ok(Some(token));
        }
    }

    fn read_text(&mut self) -> Result<(String, Option<Token>), HtmlError> {
        let mut buffer = String::new();

        loop {
            let token = match self.pull_token() {
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
    table_rows: Vec<TableRow>,
}

impl ParagraphBuilder {
    fn new(paragraph_type: ParagraphType) -> Self {
        Self {
            paragraph_type,
            children: Vec::new(),
            content: Vec::new(),
            entries: Vec::new(),
            checklist_states: Vec::new(),
            table_rows: Vec::new(),
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
        let children: Vec<Paragraph> = borrowed
            .children
            .iter()
            .map(ParagraphBuilder::to_paragraph)
            .filter(|child| !is_empty_list(child))
            .collect();

        let mut entries = Vec::new();
        let mut checklist_states = Vec::new();

        for (idx, entry_nodes) in borrowed.entries.iter().enumerate() {
            let entry: Vec<Paragraph> = entry_nodes
                .iter()
                .map(ParagraphBuilder::to_paragraph)
                .filter(|child| !is_empty_list(child))
                .collect();

            if !list_entry_has_meaningful_content(&entry) {
                continue;
            }

            entries.push(entry);
            let state = borrowed.checklist_states.get(idx).copied().unwrap_or(None);
            checklist_states.push(state);
        }

        let is_checklist = borrowed.paragraph_type == ParagraphType::Checklist
            || (!checklist_states.is_empty()
                && checklist_states.iter().all(|state| state.is_some()));

        if is_checklist {
            let mut checklist_items = Vec::new();
            for (entry, state) in entries.into_iter().zip(checklist_states) {
                let checked = state.unwrap_or(false);
                if let Some(item) = ParagraphBuilder::entry_to_checklist_item(entry, checked) {
                    checklist_items.push(item);
                }
            }
            Paragraph::new_checklist().with_checklist_items(checklist_items)
        } else {
            match borrowed.paragraph_type {
                ParagraphType::Text => Paragraph::new_text().with_content(borrowed.content.clone()),
                ParagraphType::Header1 => {
                    Paragraph::new_header1().with_content(borrowed.content.clone())
                }
                ParagraphType::Header2 => {
                    Paragraph::new_header2().with_content(borrowed.content.clone())
                }
                ParagraphType::Header3 => {
                    Paragraph::new_header3().with_content(borrowed.content.clone())
                }
                ParagraphType::CodeBlock => {
                    Paragraph::new_code_block().with_content(borrowed.content.clone())
                }
                ParagraphType::Quote => Paragraph::new_quote().with_children(children),
                ParagraphType::OrderedList => Paragraph::new_ordered_list().with_entries(entries),
                ParagraphType::UnorderedList => {
                    Paragraph::new_unordered_list().with_entries(entries)
                }
                ParagraphType::Checklist => {
                    Paragraph::new_checklist().with_checklist_items(Vec::new())
                }
                ParagraphType::Table => {
                    Paragraph::new_table().with_rows(borrowed.table_rows.clone())
                }
            }
        }
    }

    fn entry_to_checklist_item(entry: Vec<Paragraph>, checked: bool) -> Option<ChecklistItem> {
        let mut item = ChecklistItem::new(checked);
        let mut content = Vec::new();

        for paragraph in entry {
            match paragraph {
                Paragraph::Checklist { mut items } => {
                    item.children.append(&mut items);
                }
                Paragraph::Text { content: mut spans }
                | Paragraph::Header1 { content: mut spans }
                | Paragraph::Header2 { content: mut spans }
                | Paragraph::Header3 { content: mut spans }
                | Paragraph::CodeBlock { content: mut spans } => {
                    if spans.is_empty() {
                        continue;
                    }

                    if !content.is_empty() {
                        content.push(Span::new_text("\n"));
                    }

                    content.append(&mut spans);
                }
                _ => {}
            }
        }

        trim_trailing_line_breaks(&mut content);
        trim_trailing_inline_whitespace(&mut content);

        if content.is_empty() && item.children.is_empty() {
            return None;
        }

        item.content = content;
        Some(item)
    }
}

fn list_entry_has_meaningful_content(entry: &[Paragraph]) -> bool {
    entry.iter().any(paragraph_has_meaningful_content)
}

fn is_empty_list(paragraph: &Paragraph) -> bool {
    match paragraph {
        Paragraph::OrderedList { entries } | Paragraph::UnorderedList { entries } => {
            entries.iter().all(|entry| entry.is_empty())
        }
        Paragraph::Checklist { items } => items.is_empty(),
        _ => false,
    }
}

fn paragraph_has_meaningful_content(paragraph: &Paragraph) -> bool {
    match paragraph {
        Paragraph::Text { content }
        | Paragraph::Header1 { content }
        | Paragraph::Header2 { content }
        | Paragraph::Header3 { content }
        | Paragraph::CodeBlock { content } => content.iter().any(|span| !span.is_content_empty()),
        Paragraph::Quote { children } => children.iter().any(paragraph_has_meaningful_content),
        Paragraph::OrderedList { entries } | Paragraph::UnorderedList { entries } => entries
            .iter()
            .any(|nested| list_entry_has_meaningful_content(nested)),
        Paragraph::Checklist { items } => !items.is_empty(),
        Paragraph::Table { rows } => rows
            .iter()
            .any(|row| row.cells.iter().any(|cell| !cell.content.is_empty())),
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

/// Decides whether a parsed `<table>` carries genuine tabular data or is merely
/// used for layout. HTML email is built almost entirely from nested layout
/// tables; treating every one as a real table buries the actual content in
/// spurious one-cell grids and spacer rows.
///
/// A table is kept only when it is *not* flagged as presentational and forms a
/// real grid: at least two rows, at least two columns, and at least one cell
/// with content.
fn is_genuine_table(start: &StartElementToken, rows: &[TableRow]) -> bool {
    // `role="presentation"` / `role="none"` is the standard, explicit signal
    // that a table exists purely for layout.
    if let Some(role) = start.attribute("role") {
        let role = role.trim();
        if role.eq_ignore_ascii_case("presentation") || role.eq_ignore_ascii_case("none") {
            return false;
        }
    }

    if rows.len() < 2 {
        return false;
    }

    let columns = rows.iter().map(|row| row.cells.len()).max().unwrap_or(0);
    if columns < 2 {
        return false;
    }

    rows.iter().any(|row| {
        row.cells
            .iter()
            .any(|cell| cell.content.iter().any(|span| !span.is_content_empty()))
    })
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

/// Returns `true` for styled spans that carry no text or children. These add
/// no information and confuse round-trips through formats with no inline
/// representation for empty markers (e.g. Markdown emits `__` for an empty
/// italic).
fn should_skip_empty_styled_span(span: &Span) -> bool {
    span.style != InlineStyle::None && span.style != InlineStyle::Link && !span.has_content()
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
            | "thead"
            | "tbody"
            | "tfoot"
            | "td"
            | "th"
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

fn trim_trailing_line_breaks(spans: &mut Vec<Span>) {
    trim_trailing_line_breaks_impl(spans);
}

fn trim_trailing_line_breaks_impl(spans: &mut Vec<Span>) -> bool {
    let mut trimmed_any = false;

    while let Some(last) = spans.last_mut() {
        let mut trimmed = false;

        if !last.children.is_empty() {
            trimmed |= trim_trailing_line_breaks_impl(&mut last.children);
        }

        while last.text.ends_with('\n') {
            last.text.pop();
            trimmed = true;
        }

        if last.is_content_empty() {
            if last.link_target.is_none() {
                spans.pop();
                trimmed_any = true;
                continue;
            } else {
                break;
            }
        }

        if trimmed {
            trimmed_any = true;
            continue;
        }

        break;
    }

    trimmed_any
}

fn trim_trailing_inline_whitespace(spans: &mut Vec<Span>) {
    while let Some(last) = spans.last_mut() {
        if last.style != InlineStyle::None
            || !last.children.is_empty()
            || last.link_target.is_some()
        {
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

/// Writes a [`Document`] as HTML markup. Tables are preserved using
/// `<table>/<tr>/<td>` markup, unlike [`crate::ftml::write`] which flattens
/// tables to paragraphs because FTML has no table syntax.
///
/// The output is just the document body: it does not include `<!DOCTYPE>` or
/// `<html>` wrapper elements. Use [`write_document`] when a full HTML page is
/// required.
///
/// # Examples
///
/// ```
/// use tdoc::{html, Paragraph, Span, TableCell, TableRow, Document};
///
/// let table = Paragraph::new_table().with_rows(vec![
///     TableRow::new().with_cells(vec![
///         TableCell::new_header().with_content(vec![Span::new_text("Col")]),
///     ]),
/// ]);
/// let doc = Document::new().with_paragraphs(vec![table]);
///
/// let mut output = Vec::new();
/// html::write(&mut output, &doc).unwrap();
/// let html = String::from_utf8(output).unwrap();
/// assert!(html.contains("<table>"));
/// assert!(html.contains("<th>Col</th>"));
/// ```
pub fn write<W: Write>(writer: &mut W, document: &Document) -> std::io::Result<()> {
    Writer::new_html().write(writer, document)
}

/// A self-contained stylesheet embedded in [`write_document`] output. It is
/// modelled on the clean, professional look of Visual Studio Code's Markdown
/// preview: a system font stack, a centered reading column, GitHub-flavoured
/// headings, code blocks, tables, and blockquotes, plus an automatic dark mode
/// that follows the reader's `prefers-color-scheme`.
const STYLESHEET: &str = r##"
:root { color-scheme: light dark; }

body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe WPC", "Segoe UI",
    system-ui, "Ubuntu", "Droid Sans", sans-serif;
  font-size: 14px;
  line-height: 1.6;
  color: #1f2328;
  background-color: #ffffff;
  max-width: 760px;
  margin: 0 auto;
  padding: 24px 26px 64px;
  word-wrap: break-word;
}

a { color: #0969da; text-decoration: none; }
a:hover { text-decoration: underline; }

h1, h2, h3, h4, h5, h6 {
  margin-top: 24px;
  margin-bottom: 16px;
  font-weight: 600;
  line-height: 1.25;
}
h1 { font-size: 2em; padding-bottom: 0.3em; border-bottom: 1px solid #d8dee4; }
h2 { font-size: 1.5em; padding-bottom: 0.3em; border-bottom: 1px solid #d8dee4; }
h3 { font-size: 1.25em; }

body > :first-child { margin-top: 0; }

p { margin-top: 0; margin-bottom: 16px; }

ul, ol { margin-top: 0; margin-bottom: 16px; padding-left: 2em; }
li + li { margin-top: 0.25em; }
li > ul, li > ol { margin-top: 0.25em; margin-bottom: 0; }
li:has(> input[type="checkbox"]) { list-style: none; }
li > input[type="checkbox"] { margin: 0 0.4em 0 -1.4em; vertical-align: middle; }

blockquote {
  margin: 0 0 16px 0;
  padding: 0 1em;
  color: #656d76;
  border-left: 0.25em solid #d0d7de;
}
blockquote > :first-child { margin-top: 0; }
blockquote > :last-child { margin-bottom: 0; }

code, tt {
  font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas,
    "Liberation Mono", monospace;
  font-size: 0.9em;
  padding: 0.2em 0.4em;
  background-color: rgba(175, 184, 193, 0.2);
  border-radius: 6px;
}

pre {
  margin-top: 0;
  margin-bottom: 16px;
  padding: 16px;
  overflow: auto;
  font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas,
    "Liberation Mono", monospace;
  font-size: 0.9em;
  line-height: 1.45;
  background-color: #f6f8fa;
  border-radius: 6px;
}
pre code, pre tt {
  padding: 0;
  font-size: inherit;
  background-color: transparent;
  border-radius: 0;
}

table {
  margin-top: 0;
  margin-bottom: 16px;
  border-collapse: collapse;
  display: block;
  width: max-content;
  max-width: 100%;
  overflow: auto;
}
th, td { padding: 6px 13px; border: 1px solid #d0d7de; }
th { font-weight: 600; }
tr:nth-child(2n) { background-color: #f6f8fa; }

mark { background-color: #fff8c5; color: inherit; }

hr { height: 0.25em; margin: 24px 0; background-color: #d0d7de; border: 0; }

img { max-width: 100%; }

@media (prefers-color-scheme: dark) {
  body { color: #e6edf3; background-color: #0d1117; }
  a { color: #4493f8; }
  h1, h2 { border-bottom-color: #30363d; }
  blockquote { color: #9198a1; border-left-color: #30363d; }
  code, tt { background-color: rgba(110, 118, 129, 0.4); }
  pre { background-color: #161b22; }
  th, td { border-color: #30363d; }
  tr:nth-child(2n) { background-color: #161b22; }
  mark { background-color: #bb8009; color: #1f2328; }
  hr { background-color: #30363d; }
}
"##;

/// Writes a [`Document`] wrapped in a complete, styled HTML page (`<!DOCTYPE>`,
/// `<html>`, `<head>`, `<body>`). The `<head>` embeds [`STYLESHEET`], a
/// self-contained stylesheet that gives the document the clean, professional
/// look of Visual Studio Code's Markdown preview.
pub fn write_document<W: Write>(writer: &mut W, document: &Document) -> std::io::Result<()> {
    writer.write_all(
        b"<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n\
          <meta charset=\"utf-8\" />\n\
          <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n\
          <style>",
    )?;
    writer.write_all(STYLESHEET.as_bytes())?;
    writer.write_all(b"</style>\n</head>\n<body>\n")?;
    write(writer, document)?;
    writer.write_all(b"\n</body>\n</html>\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn skips_link_without_description() {
        let input = "<p><a href=\"https://example.com\"></a></p>";
        let document = parse(Cursor::new(input)).unwrap();

        assert!(document.paragraphs.is_empty());
    }

    #[test]
    fn keeps_link_when_description_matches_target() {
        let input = "<p><a href=\"https://example.com\">https://example.com</a></p>";
        let document = parse(Cursor::new(input)).unwrap();

        assert_eq!(document.paragraphs.len(), 1);
        let paragraph = &document.paragraphs[0];
        assert_eq!(paragraph.paragraph_type(), ParagraphType::Text);
        assert_eq!(paragraph.content().len(), 1);

        let span = &paragraph.content()[0];
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
        assert_eq!(paragraph.content().len(), 1);

        let span = &paragraph.content()[0];
        assert_eq!(span.style, InlineStyle::None);
        assert!(span.link_target.is_none());
        assert_eq!(span.text, "Example");
    }

    #[test]
    fn ignores_hash_link_targets() {
        let input = "<p><a href=\"#\">Anchor label</a></p>";
        let document = parse(Cursor::new(input)).unwrap();

        let paragraph = &document.paragraphs[0];
        assert_eq!(paragraph.content().len(), 1);

        let span = &paragraph.content()[0];
        assert_eq!(span.style, InlineStyle::None);
        assert!(span.link_target.is_none());
        assert_eq!(span.text, "Anchor label");
    }

    #[test]
    fn handles_link_href_with_leading_newline() {
        let input = "<p><a href=\n\"https://example.com/resource\">Example</a></p>";
        let document = parse(Cursor::new(input)).unwrap();

        assert_eq!(document.paragraphs.len(), 1);
        let paragraph = &document.paragraphs[0];
        assert_eq!(paragraph.content().len(), 1);

        let span = &paragraph.content()[0];
        assert_eq!(span.style, InlineStyle::Link);
        assert_eq!(
            span.link_target.as_deref(),
            Some("https://example.com/resource")
        );
    }

    #[test]
    fn trims_trailing_line_breaks_from_text_paragraphs() {
        let input = "<p>Hello<br></p>";
        let document = parse(Cursor::new(input)).unwrap();

        assert_eq!(document.paragraphs.len(), 1);
        let paragraph = &document.paragraphs[0];
        assert_eq!(paragraph.paragraph_type(), ParagraphType::Text);
        assert_eq!(paragraph.content().len(), 1);
        assert_eq!(paragraph.content()[0].text, "Hello");
    }

    #[test]
    fn drops_empty_paragraphs_created_by_line_breaks() {
        let input = "<p><br></p><p>World</p>";
        let document = parse(Cursor::new(input)).unwrap();

        assert_eq!(document.paragraphs.len(), 1);
        let paragraph = &document.paragraphs[0];
        assert_eq!(paragraph.paragraph_type(), ParagraphType::Text);
        assert_eq!(paragraph.content().len(), 1);
        assert_eq!(paragraph.content()[0].text, "World");
    }

    #[test]
    fn parses_links_inside_navigation_container() {
        let input = r#"
<div class="nav_container">
  <div class="nav_title">
    <a href="/">roblog.</a>
  </div>
  <div class="site_nav" id="site_nav">
    <ul>
      <li>
        <a href="/" class="active">
          home
        </a>
      </li>
      <li>
        <a href="/articles" class>
          articles
        </a>
      </li>
    </ul>
  </div>
</div>
"#;

        let document = parse(Cursor::new(input)).unwrap();

        let mut tokenizer = Tokenizer::new(input);
        let mut seen_links = Vec::new();
        while let Ok(token) = tokenizer.next_token() {
            if let Token::StartElement(start) = token {
                if start.name().eq_ignore_ascii_case("a") {
                    seen_links.push(start.attribute("href"));
                }
            }
        }

        assert_eq!(
            seen_links,
            vec![
                Some("/".to_string()),
                Some("/".to_string()),
                Some("/articles".to_string())
            ]
        );

        let list = document
            .paragraphs
            .iter()
            .find(|paragraph| paragraph.paragraph_type() == ParagraphType::UnorderedList)
            .expect("expected a list paragraph");

        assert_eq!(list.entries().len(), 2);

        for (entry, expected_href) in list.entries().iter().zip(["/", "/articles"]) {
            assert_eq!(entry.len(), 1);
            let text_paragraph = &entry[0];
            assert_eq!(text_paragraph.paragraph_type(), ParagraphType::Text);
            assert!(
                text_paragraph.content().iter().any(|span| {
                    span.style == InlineStyle::Link
                        && span.link_target.as_deref() == Some(expected_href)
                }),
                "expected list item to contain a link span for href '{expected_href}', got {:?}",
                text_paragraph.content()
            );
        }
    }

    #[test]
    fn parses_simple_table_with_header_row() {
        let input = "<table><thead><tr><th>Name</th><th>Age</th></tr></thead>\
                     <tbody><tr><td>Alice</td><td>30</td></tr>\
                     <tr><td>Bob</td><td>25</td></tr></tbody></table>";
        let document = parse(Cursor::new(input)).unwrap();

        assert_eq!(document.paragraphs.len(), 1);
        let paragraph = &document.paragraphs[0];
        assert_eq!(paragraph.paragraph_type(), ParagraphType::Table);

        let rows = paragraph.rows();
        assert_eq!(rows.len(), 3);

        assert!(rows[0].cells.iter().all(|cell| cell.is_header));
        assert_eq!(rows[0].cells[0].content[0].text, "Name");
        assert_eq!(rows[0].cells[1].content[0].text, "Age");

        assert!(rows[1].cells.iter().all(|cell| !cell.is_header));
        assert_eq!(rows[1].cells[0].content[0].text, "Alice");
        assert_eq!(rows[1].cells[1].content[0].text, "30");

        assert_eq!(rows[2].cells[0].content[0].text, "Bob");
    }

    #[test]
    fn parses_table_with_inline_styles_in_cells() {
        let input = "<table><tr><th>Col</th><th>Other</th></tr>\
                     <tr><td>hello <b>world</b></td><td>plain</td></tr></table>";
        let document = parse(Cursor::new(input)).unwrap();

        let table = &document.paragraphs[0];
        let data_cell = &table.rows()[1].cells[0];
        assert_eq!(data_cell.content.len(), 2);
        assert_eq!(data_cell.content[0].text, "hello ");
        assert_eq!(data_cell.content[1].style, InlineStyle::Bold);
        assert_eq!(data_cell.content[1].children[0].text, "world");
    }

    #[test]
    fn parses_table_without_explicit_tbody() {
        let input = "<table><tr><td>A</td><td>B</td></tr><tr><td>C</td><td>D</td></tr></table>";
        let document = parse(Cursor::new(input)).unwrap();

        let table = &document.paragraphs[0];
        assert_eq!(table.rows().len(), 2);
        assert_eq!(table.rows()[0].cells.len(), 2);
        assert!(!table.rows()[0].cells[0].is_header);
    }

    #[test]
    fn presentational_table_is_flattened_to_paragraphs() {
        let input = "<table role=\"presentation\">\
                     <tr><td>First cell</td><td>Second cell</td></tr>\
                     <tr><td>Third cell</td><td>Fourth cell</td></tr></table>";
        let document = parse(Cursor::new(input)).unwrap();

        assert!(document
            .paragraphs
            .iter()
            .all(|paragraph| paragraph.paragraph_type() != ParagraphType::Table));
        assert_eq!(document.paragraphs.len(), 4);
        assert_eq!(document.paragraphs[0].content()[0].text, "First cell");
        assert_eq!(document.paragraphs[3].content()[0].text, "Fourth cell");
    }

    #[test]
    fn single_row_table_is_flattened_to_paragraphs() {
        // Classic admonition layout: an empty icon cell plus a content cell.
        let input = "<table><tr><td></td><td>Back up your data first.</td></tr></table>";
        let document = parse(Cursor::new(input)).unwrap();

        assert_eq!(document.paragraphs.len(), 1);
        assert_eq!(document.paragraphs[0].paragraph_type(), ParagraphType::Text);
        assert_eq!(
            document.paragraphs[0].content()[0].text,
            "Back up your data first."
        );
    }

    #[test]
    fn single_column_table_is_flattened_to_paragraphs() {
        let input = "<table><tr><td>One</td></tr><tr><td>Two</td></tr></table>";
        let document = parse(Cursor::new(input)).unwrap();

        assert_eq!(document.paragraphs.len(), 2);
        assert!(document
            .paragraphs
            .iter()
            .all(|paragraph| paragraph.paragraph_type() == ParagraphType::Text));
        assert_eq!(document.paragraphs[0].content()[0].text, "One");
        assert_eq!(document.paragraphs[1].content()[0].text, "Two");
    }

    #[test]
    fn layout_table_preserves_block_structure() {
        // A presentational wrapper around real block content: the table is
        // dropped, but the heading, paragraph, and list survive as distinct
        // blocks rather than being mashed into one inline blob.
        let input = "<table role=\"presentation\"><tr><td>\
                     <h2>Heading</h2>\
                     <p>Intro paragraph</p>\
                     <ul><li>First</li><li>Second</li></ul>\
                     </td></tr></table>";
        let document = parse(Cursor::new(input)).unwrap();

        assert_eq!(document.paragraphs.len(), 3);
        assert_eq!(
            document.paragraphs[0].paragraph_type(),
            ParagraphType::Header2
        );
        assert_eq!(document.paragraphs[0].content()[0].text, "Heading");
        assert_eq!(document.paragraphs[1].paragraph_type(), ParagraphType::Text);
        assert_eq!(document.paragraphs[1].content()[0].text, "Intro paragraph");

        let list = &document.paragraphs[2];
        assert_eq!(list.paragraph_type(), ParagraphType::UnorderedList);
        assert_eq!(list.entries().len(), 2);
        assert_eq!(list.entries()[0][0].content()[0].text, "First");
        assert_eq!(list.entries()[1][0].content()[0].text, "Second");
    }

    #[test]
    fn empty_layout_table_is_dropped() {
        let input = "<table role=\"presentation\"><tr><td></td><td></td></tr>\
                     <tr><td></td><td></td></tr></table>";
        let document = parse(Cursor::new(input)).unwrap();

        assert!(document.paragraphs.is_empty());
    }

    #[test]
    fn write_document_embeds_stylesheet_and_body() {
        let doc = parse(Cursor::new("<h1>Title</h1><p>Body</p>")).unwrap();

        let mut output = Vec::new();
        write_document(&mut output, &doc).unwrap();
        let html = String::from_utf8(output).unwrap();

        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("<style>"));
        assert!(html.contains("prefers-color-scheme: dark"));
        assert!(html.contains("</style>"));
        // The styled <head> must precede the document content in the <body>.
        let style_end = html.find("</style>").unwrap();
        let body_start = html.find("<h1>Title</h1>").unwrap();
        assert!(style_end < body_start);
        assert!(html.trim_end().ends_with("</html>"));
    }
}
