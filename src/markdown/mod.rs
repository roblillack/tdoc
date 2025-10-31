//! Convert between Markdown text and FTML [`Document`](crate::Document) trees.

use crate::{Document, InlineStyle, Paragraph, ParagraphType, Span};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag};
use std::io::{Read, Write};

/// Parses Markdown into a [`Document`].
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// use tdoc::{markdown, ParagraphType};
///
/// let doc = markdown::parse(Cursor::new("# Heading")).unwrap();
/// assert_eq!(doc.paragraphs[0].paragraph_type, ParagraphType::Header1);
/// ```
pub fn parse<R: Read>(mut reader: R) -> crate::Result<Document> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&input, options);
    let mut builder = MarkdownBuilder::new();

    for event in parser {
        builder.handle_event(event);
    }

    Ok(builder.finish())
}

struct MarkdownBuilder {
    stack: Vec<BlockContext>,
}

impl MarkdownBuilder {
    fn new() -> Self {
        Self {
            stack: vec![BlockContext::Document {
                paragraphs: Vec::new(),
            }],
        }
    }

    fn finish(mut self) -> Document {
        self.close_open_paragraphs();
        if self.stack.len() != 1 {
            // Best effort: collapse any remaining containers
            while self.stack.len() > 1 {
                match self.stack.pop() {
                    Some(BlockContext::Paragraph(ctx)) => {
                        let paragraph = ctx.finish();
                        self.add_paragraph_to_parent(paragraph);
                    }
                    Some(BlockContext::List {
                        ordered,
                        entries,
                        is_checklist,
                    }) => {
                        let mut paragraph = if is_checklist {
                            Paragraph::new_checklist()
                        } else if ordered {
                            Paragraph::new_ordered_list()
                        } else {
                            Paragraph::new_unordered_list()
                        };
                        paragraph.entries = entries;
                        self.add_paragraph_to_parent(paragraph);
                    }
                    Some(BlockContext::ListItem {
                        paragraphs,
                        checklist_state,
                    }) => {
                        if let Some(BlockContext::List { entries, .. }) = self.stack.last_mut() {
                            if let Some(checked) = checklist_state {
                                let item = Self::build_checklist_item(paragraphs, checked);
                                entries.push(vec![item]);
                            } else {
                                entries.push(paragraphs);
                            }
                        }
                    }
                    Some(BlockContext::Quote { children }) => {
                        let paragraph = Paragraph::new_quote().with_children(children);
                        self.add_paragraph_to_parent(paragraph);
                    }
                    Some(BlockContext::Document { paragraphs }) => {
                        return Document { paragraphs };
                    }
                    None => break,
                }
            }
        }

        match self.stack.pop() {
            Some(BlockContext::Document { paragraphs }) => Document { paragraphs },
            _ => Document::new(),
        }
    }

    fn handle_event(&mut self, event: Event<'_>) {
        match event {
            Event::Start(tag) => self.handle_start_tag(tag),
            Event::End(tag) => self.handle_end_tag(tag),
            Event::Text(text) => self.push_text(text.as_ref()),
            Event::Html(html) => self.handle_html(html.as_ref()),
            Event::Code(text) => self.push_code(text.as_ref()),
            Event::FootnoteReference(reference) => {
                let marker = format!("[^{}]", reference);
                self.push_text(&marker);
            }
            Event::SoftBreak => self.push_soft_break(),
            Event::HardBreak => self.push_hard_break(),
            Event::Rule => self.push_thematic_break(),
            Event::TaskListMarker(checked) => self.push_task_marker(checked),
        }
    }

    fn handle_start_tag(&mut self, tag: Tag<'_>) {
        match tag {
            Tag::Paragraph => {
                self.start_paragraph(ParagraphType::Text);
            }
            Tag::Heading(level, _, _) => {
                let paragraph_type = match level {
                    HeadingLevel::H1 => ParagraphType::Header1,
                    HeadingLevel::H2 => ParagraphType::Header2,
                    HeadingLevel::H3 => ParagraphType::Header3,
                    _ => ParagraphType::Text,
                };
                self.start_paragraph(paragraph_type);
            }
            Tag::BlockQuote => {
                self.close_open_paragraphs();
                self.stack.push(BlockContext::Quote {
                    children: Vec::new(),
                });
            }
            Tag::List(start) => {
                self.close_open_paragraphs();
                let ordered = start.is_some();
                self.stack.push(BlockContext::List {
                    ordered,
                    entries: Vec::new(),
                    is_checklist: false,
                });
            }
            Tag::Item => {
                self.stack.push(BlockContext::ListItem {
                    paragraphs: Vec::new(),
                    checklist_state: None,
                });
            }
            Tag::Emphasis => {
                self.ensure_paragraph()
                    .start_inline(Span::new_styled(InlineStyle::Italic));
            }
            Tag::Strong => {
                self.ensure_paragraph()
                    .start_inline(Span::new_styled(InlineStyle::Bold));
            }
            Tag::Strikethrough => {
                self.ensure_paragraph()
                    .start_inline(Span::new_styled(InlineStyle::Strike));
            }
            Tag::Link(_link_type, dest, _) => {
                let span = Span::new_styled(InlineStyle::Link).with_link_target(dest.into_string());
                self.ensure_paragraph().start_inline(span);
            }
            Tag::Image(_link_type, dest, _) => {
                let span = Span::new_styled(InlineStyle::Link).with_link_target(dest.into_string());
                self.ensure_paragraph().start_inline(span);
            }
            Tag::CodeBlock(_) => {
                self.start_paragraph(ParagraphType::CodeBlock);
            }
            Tag::FootnoteDefinition(name) => {
                let paragraph = self.start_paragraph(ParagraphType::Text);
                paragraph.push_text(&format!("[^{}]: ", name));
            }
            Tag::Table(_) | Tag::TableHead | Tag::TableRow | Tag::TableCell => {
                // Tables are flattened into text paragraphs.
            }
        }
    }

    fn handle_end_tag(&mut self, tag: Tag<'_>) {
        match tag {
            Tag::Paragraph | Tag::Heading(_, _, _) => {
                self.finish_paragraph();
            }
            Tag::BlockQuote => {
                self.close_open_paragraphs();
                if let Some(BlockContext::Quote { children }) = self.stack.pop() {
                    let paragraph = Paragraph::new_quote().with_children(children);
                    self.add_paragraph_to_parent(paragraph);
                }
            }
            Tag::List(_) => {
                self.close_open_paragraphs();
                if let Some(BlockContext::List {
                    ordered,
                    entries,
                    is_checklist,
                }) = self.stack.pop()
                {
                    let mut paragraph = if is_checklist {
                        Paragraph::new_checklist()
                    } else if ordered {
                        Paragraph::new_ordered_list()
                    } else {
                        Paragraph::new_unordered_list()
                    };
                    paragraph.entries = entries;
                    self.add_paragraph_to_parent(paragraph);
                }
            }
            Tag::Item => {
                self.close_open_paragraphs();
                if let Some(BlockContext::ListItem {
                    paragraphs,
                    checklist_state,
                }) = self.stack.pop()
                {
                    if let Some(BlockContext::List {
                        entries,
                        is_checklist,
                        ..
                    }) = self.stack.last_mut()
                    {
                        if let Some(checked) = checklist_state {
                            let item = Self::build_checklist_item(paragraphs, checked);
                            entries.push(vec![item]);
                            *is_checklist = true;
                        } else {
                            entries.push(paragraphs);
                        }
                    }
                }
            }
            Tag::Emphasis => {
                self.current_paragraph_inline_end(InlineStyle::Italic);
            }
            Tag::Strong => {
                self.current_paragraph_inline_end(InlineStyle::Bold);
            }
            Tag::Strikethrough => {
                self.current_paragraph_inline_end(InlineStyle::Strike);
            }
            Tag::Link(_, _, _) | Tag::Image(_, _, _) => {
                self.current_paragraph_inline_end(InlineStyle::Link);
            }
            Tag::CodeBlock(_) => {
                self.finish_paragraph();
            }
            Tag::FootnoteDefinition(_) => {
                self.finish_paragraph();
            }
            Tag::Table(_) | Tag::TableHead | Tag::TableRow | Tag::TableCell => {
                self.close_open_paragraphs();
            }
        }
    }

    fn handle_html(&mut self, html: &str) {
        let trimmed = html.trim();
        if trimmed.is_empty() {
            return;
        }

        let lowercase = trimmed.to_ascii_lowercase();

        if is_open_tag(&lowercase, "mark") {
            self.ensure_paragraph()
                .start_inline(Span::new_styled(InlineStyle::Highlight));
            return;
        }

        if is_close_tag(&lowercase, "mark") {
            self.current_paragraph_inline_end(InlineStyle::Highlight);
            return;
        }

        if is_open_tag(&lowercase, "u") {
            self.ensure_paragraph()
                .start_inline(Span::new_styled(InlineStyle::Underline));
            return;
        }

        if is_close_tag(&lowercase, "u") {
            self.current_paragraph_inline_end(InlineStyle::Underline);
            return;
        }

        if is_open_tag(&lowercase, "del") {
            self.ensure_paragraph()
                .start_inline(Span::new_styled(InlineStyle::Strike));
            return;
        }

        if is_close_tag(&lowercase, "del") {
            self.current_paragraph_inline_end(InlineStyle::Strike);
            return;
        }

        self.push_text(html);
    }

    fn current_paragraph_inline_end(&mut self, style: InlineStyle) {
        if let Some(BlockContext::Paragraph(context)) = self.stack.last_mut() {
            context.end_inline(style);
        }
    }

    fn push_text(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }
        let paragraph = self.ensure_paragraph();
        paragraph.push_text(text);
    }

    fn push_code(&mut self, text: &str) {
        let paragraph = self.ensure_paragraph();
        paragraph.push_code(text);
    }

    fn push_soft_break(&mut self) {
        let paragraph = self.ensure_paragraph();
        paragraph.push_text("\n");
    }

    fn push_hard_break(&mut self) {
        let paragraph = self.ensure_paragraph();
        paragraph.push_text("\n");
    }

    fn push_task_marker(&mut self, checked: bool) {
        if let Some(BlockContext::ListItem {
            checklist_state, ..
        }) = self
            .stack
            .iter_mut()
            .rev()
            .find(|ctx| matches!(ctx, BlockContext::ListItem { .. }))
        {
            *checklist_state = Some(checked);
        }

        if let Some(BlockContext::List { is_checklist, .. }) = self
            .stack
            .iter_mut()
            .rev()
            .find(|ctx| matches!(ctx, BlockContext::List { .. }))
        {
            *is_checklist = true;
        }
    }

    fn push_thematic_break(&mut self) {
        self.close_open_paragraphs();
        let mut paragraph = Paragraph::new_text();
        paragraph.content.push(Span::new_text("---"));
        self.add_paragraph_to_parent(paragraph);
    }

    fn start_paragraph(&mut self, paragraph_type: ParagraphType) -> &mut ParagraphContext {
        self.stack
            .push(BlockContext::Paragraph(ParagraphContext::new(
                paragraph_type,
            )));
        match self.stack.last_mut() {
            Some(BlockContext::Paragraph(context)) => context,
            _ => unreachable!(),
        }
    }

    fn ensure_paragraph(&mut self) -> &mut ParagraphContext {
        let needs_new = !matches!(self.stack.last(), Some(BlockContext::Paragraph(_)));
        if needs_new {
            self.start_paragraph(ParagraphType::Text);
        }

        match self.stack.last_mut() {
            Some(BlockContext::Paragraph(context)) => context,
            _ => unreachable!("Paragraph context should exist after initialization"),
        }
    }

    fn finish_paragraph(&mut self) {
        if let Some(BlockContext::Paragraph(context)) = self.stack.pop() {
            let paragraph = context.finish();
            self.add_paragraph_to_parent(paragraph);
        }
    }

    fn close_open_paragraphs(&mut self) {
        while matches!(self.stack.last(), Some(BlockContext::Paragraph(_))) {
            self.finish_paragraph();
        }
    }

    fn add_paragraph_to_parent(&mut self, paragraph: Paragraph) {
        if let Some(parent) = self.stack.last_mut() {
            match parent {
                BlockContext::Document { paragraphs } => paragraphs.push(paragraph),
                BlockContext::Quote { children } => children.push(paragraph),
                BlockContext::ListItem {
                    paragraphs: items, ..
                } => items.push(paragraph),
                BlockContext::List { entries, .. } => {
                    entries.push(vec![paragraph]);
                }
                BlockContext::Paragraph(context) => {
                    context.push_nested_paragraph(paragraph);
                }
            }
        }
    }

    fn build_checklist_item(paragraphs: Vec<Paragraph>, checked: bool) -> Paragraph {
        let mut item = Paragraph::new_checklist_item(checked);
        let mut content = Vec::new();

        for (idx, paragraph) in paragraphs.into_iter().enumerate() {
            if paragraph.content.is_empty() {
                continue;
            }

            if idx > 0 && !content.is_empty() {
                content.push(Span::new_text("\n"));
            }

            content.extend(paragraph.content.into_iter());
        }

        item.content = content;
        item
    }
}

enum BlockContext {
    Document {
        paragraphs: Vec<Paragraph>,
    },
    Quote {
        children: Vec<Paragraph>,
    },
    List {
        ordered: bool,
        entries: Vec<Vec<Paragraph>>,
        is_checklist: bool,
    },
    ListItem {
        paragraphs: Vec<Paragraph>,
        checklist_state: Option<bool>,
    },
    Paragraph(ParagraphContext),
}

fn is_open_tag(tag: &str, name: &str) -> bool {
    let prefix = format!("<{}", name);
    tag.starts_with(&prefix) && !tag.starts_with("</") && tag.contains('>')
}

fn is_close_tag(tag: &str, name: &str) -> bool {
    let prefix = format!("</{}", name);
    tag.starts_with(&prefix) && tag.contains('>')
}

struct ParagraphContext {
    paragraph_type: ParagraphType,
    spans: Vec<Span>,
    inline_stack: Vec<Span>,
}

impl ParagraphContext {
    fn new(paragraph_type: ParagraphType) -> Self {
        Self {
            paragraph_type,
            spans: Vec::new(),
            inline_stack: Vec::new(),
        }
    }

    fn push_text(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }
        let span = Span::new_text(text);
        self.push_span(span);
    }

    fn push_code(&mut self, text: &str) {
        let mut span = Span::new_styled(InlineStyle::Code);
        if !text.is_empty() {
            span.children.push(Span::new_text(text));
        }
        self.push_span(span);
    }

    fn push_nested_paragraph(&mut self, paragraph: Paragraph) {
        for span in paragraph.content {
            self.push_span(span);
        }
    }

    fn start_inline(&mut self, span: Span) {
        self.inline_stack.push(span);
    }

    fn end_inline(&mut self, style: InlineStyle) {
        if let Some(mut span) = self.inline_stack.pop() {
            if span.style == InlineStyle::Link {
                span.strip_redundant_link_description();
            }
            if span.style != style
                && !(span.style == InlineStyle::Link && style == InlineStyle::Link)
            {
                // Style mismatch; keep span as-is
            }
            self.push_span(span);
        }
    }

    fn push_span(&mut self, span: Span) {
        if let Some(parent) = self.inline_stack.last_mut() {
            Self::append_span(&mut parent.children, span);
        } else {
            Self::append_span(&mut self.spans, span);
        }
    }

    fn append_span(target: &mut Vec<Span>, span: Span) {
        if let Some(last) = target.last_mut() {
            if Self::can_merge(last, &span) {
                last.text.push_str(&span.text);
                return;
            }
        }
        target.push(span);
    }

    fn can_merge(a: &Span, b: &Span) -> bool {
        a.style == InlineStyle::None
            && b.style == InlineStyle::None
            && a.link_target.is_none()
            && b.link_target.is_none()
            && a.children.is_empty()
            && b.children.is_empty()
    }

    fn finish(mut self) -> Paragraph {
        while let Some(span) = self.inline_stack.pop() {
            self.push_span(span);
        }

        let mut paragraph = Paragraph::new(self.paragraph_type);
        paragraph.content = self.spans;
        paragraph
    }
}

const LINE_WIDTH: usize = 80;

/// Serializes a [`Document`] structure back to Markdown.
///
/// # Examples
///
/// ```
/// use tdoc::{Document, Paragraph, Span};
/// use tdoc::markdown;
///
/// let paragraph = Paragraph::new_text().with_content(vec![Span::new_text("Hello")]);
/// let document = Document::new().with_paragraphs(vec![paragraph]);
///
/// let mut output = Vec::new();
/// markdown::write(&mut output, &document).unwrap();
/// assert_eq!(String::from_utf8(output).unwrap(), "Hello\n");
/// ```
pub fn write<W: Write>(writer: &mut W, document: &Document) -> std::io::Result<()> {
    write_paragraphs(writer, &document.paragraphs, "", "")
}

fn write_paragraphs<W: Write>(
    writer: &mut W,
    paragraphs: &[Paragraph],
    prefix: &str,
    continuation_prefix: &str,
) -> std::io::Result<()> {
    for (i, paragraph) in paragraphs.iter().enumerate() {
        if i > 0 {
            if !continuation_prefix.is_empty() {
                write!(writer, "{}", continuation_prefix)?;
            }
            writeln!(writer)?;
        }
        let current_prefix = if i == 0 { prefix } else { continuation_prefix };
        write_paragraph(writer, paragraph, current_prefix, continuation_prefix)?;
    }
    Ok(())
}

fn write_paragraph<W: Write>(
    writer: &mut W,
    paragraph: &Paragraph,
    prefix: &str,
    continuation_prefix: &str,
) -> std::io::Result<()> {
    match paragraph.paragraph_type {
        ParagraphType::Text => {
            let content = render_spans_to_string(&paragraph.content)?;
            write_wrapped_lines(writer, prefix, continuation_prefix, &content)?;
        }
        ParagraphType::CodeBlock => {
            write_code_block(writer, prefix, continuation_prefix, &paragraph.content)?;
        }
        ParagraphType::Header1 => {
            let content = render_spans_to_string(&paragraph.content)?;
            let first_prefix = format!("{}# ", prefix);
            write_wrapped_lines(writer, &first_prefix, continuation_prefix, &content)?;
        }
        ParagraphType::Header2 => {
            let content = render_spans_to_string(&paragraph.content)?;
            let first_prefix = format!("{}## ", prefix);
            write_wrapped_lines(writer, &first_prefix, continuation_prefix, &content)?;
        }
        ParagraphType::Header3 => {
            let content = render_spans_to_string(&paragraph.content)?;
            let first_prefix = format!("{}### ", prefix);
            write_wrapped_lines(writer, &first_prefix, continuation_prefix, &content)?;
        }
        ParagraphType::Quote => {
            let quote_prefix = format!("{}> ", prefix);
            let quote_continuation = format!("{}> ", continuation_prefix);

            for (idx, child) in paragraph.children.iter().enumerate() {
                if idx > 0 {
                    write!(writer, "{}", quote_continuation)?;
                    writeln!(writer)?;
                }
                write_paragraph(writer, child, &quote_prefix, &quote_continuation)?;
            }
        }
        ParagraphType::UnorderedList => {
            for entry in &paragraph.entries {
                let bullet_prefix = format!("{}- ", prefix);
                let bullet_continuation = format!("{}  ", continuation_prefix);

                write_paragraphs(writer, entry, &bullet_prefix, &bullet_continuation)?;
            }
        }
        ParagraphType::OrderedList => {
            for (i, entry) in paragraph.entries.iter().enumerate() {
                let bullet_prefix = format!("{}{}. ", prefix, i + 1);
                let bullet_continuation = format!("{}   ", continuation_prefix);

                write_paragraphs(writer, entry, &bullet_prefix, &bullet_continuation)?;
            }
        }
        ParagraphType::Checklist => {
            for entry in &paragraph.entries {
                let item = entry
                    .iter()
                    .find(|p| p.paragraph_type == ParagraphType::ChecklistItem)
                    .or_else(|| entry.first());

                if let Some(item) = item {
                    let marker = if item.checklist_item_checked.unwrap_or(false) {
                        'x'
                    } else {
                        ' '
                    };
                    let first_prefix = format!("{}- [{}] ", prefix, marker);
                    let continuation = format!("{}{}", continuation_prefix, " ".repeat(6));
                    let content = render_spans_to_string(&item.content)?;
                    write_wrapped_lines(writer, &first_prefix, &continuation, &content)?;
                }
            }
        }
        ParagraphType::ChecklistItem => {
            let marker = if paragraph.checklist_item_checked.unwrap_or(false) {
                'x'
            } else {
                ' '
            };
            let first_prefix = format!("{}- [{}] ", prefix, marker);
            let continuation = format!("{}{}", continuation_prefix, " ".repeat(6));
            let content = render_spans_to_string(&paragraph.content)?;
            write_wrapped_lines(writer, &first_prefix, &continuation, &content)?;
        }
    }
    Ok(())
}

fn write_code_block<W: Write>(
    writer: &mut W,
    prefix: &str,
    continuation_prefix: &str,
    spans: &[Span],
) -> std::io::Result<()> {
    writeln!(writer, "{}```", prefix)?;

    let mut content = String::new();
    for span in spans {
        collect_plain_text(span, &mut content);
    }
    let normalized = content.replace("\r\n", "\n").replace('\r', "\n");

    if !normalized.is_empty() {
        for line in normalized.split('\n') {
            writeln!(writer, "{}{}", continuation_prefix, line)?;
        }
    }

    writeln!(writer, "{}```", continuation_prefix)?;
    writeln!(writer)?;
    Ok(())
}

fn write_spans<W: Write>(
    writer: &mut W,
    spans: &[Span],
    state: &mut LineState<'_>,
) -> std::io::Result<()> {
    for span in spans {
        write_span(writer, span, state)?;
    }
    Ok(())
}

fn write_span<W: Write>(
    writer: &mut W,
    span: &Span,
    state: &mut LineState<'_>,
) -> std::io::Result<()> {
    match span.style {
        InlineStyle::Link => {
            if let Some(target) = &span.link_target {
                if span.has_content() {
                    state.write_chunk(writer, "[")?;
                    write_span_content(writer, span, state)?;
                    let closing = format!("]({})", escape_link_destination(target));
                    state.write_chunk(writer, &closing)?;
                } else {
                    let autop = format!("<{}>", escape_link_destination(target));
                    state.write_chunk(writer, &autop)?;
                }
                Ok(())
            } else {
                write_span_content(writer, span, state)
            }
        }
        InlineStyle::Code => write_code_span(writer, span, state),
        style => {
            let (begin_tag, end_tag) = inline_tags(style);
            if !begin_tag.is_empty() {
                state.write_chunk(writer, begin_tag)?;
            }
            write_span_content(writer, span, state)?;
            if !end_tag.is_empty() {
                state.write_chunk(writer, end_tag)?;
            }
            Ok(())
        }
    }
}

fn write_span_content<W: Write>(
    writer: &mut W,
    span: &Span,
    state: &mut LineState<'_>,
) -> std::io::Result<()> {
    if !span.text.is_empty() {
        write_plain_text(writer, &span.text, state)?;
    }

    for child in &span.children {
        write_span(writer, child, state)?;
    }

    Ok(())
}

fn render_spans_to_string(spans: &[Span]) -> std::io::Result<String> {
    let mut buffer = Vec::new();
    let mut state = LineState::new("");
    write_spans(&mut buffer, spans, &mut state)?;
    Ok(String::from_utf8(buffer).expect("Rendered markdown should be valid UTF-8"))
}

fn write_wrapped_lines<W: Write>(
    writer: &mut W,
    first_prefix: &str,
    continuation_prefix: &str,
    content: &str,
) -> std::io::Result<()> {
    let mut wrote_line = false;

    for (idx, raw_line) in content.split('\n').enumerate() {
        let prefix_for_line = if idx == 0 {
            first_prefix
        } else {
            continuation_prefix
        };

        for line in wrap_single_line(raw_line, prefix_for_line, continuation_prefix) {
            if wrote_line {
                writeln!(writer)?;
            }
            writer.write_all(line.as_bytes())?;
            wrote_line = true;
        }
    }

    writeln!(writer)?;
    Ok(())
}

fn wrap_single_line(line: &str, first_prefix: &str, continuation_prefix: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    current_line.push_str(first_prefix);

    let mut base_len = first_prefix.chars().count();
    let mut current_len = base_len;
    let mut pending_whitespace = String::new();

    let mut chars = line.char_indices().peekable();
    while let Some((start, ch)) = chars.next() {
        let is_space = ch.is_whitespace();
        let mut end = start + ch.len_utf8();
        while let Some(&(next_idx, next_ch)) = chars.peek() {
            if next_ch.is_whitespace() == is_space {
                chars.next();
                end = next_idx + next_ch.len_utf8();
            } else {
                break;
            }
        }

        let token = &line[start..end];

        if is_space {
            pending_whitespace.push_str(token);
            continue;
        }

        let pending_len = pending_whitespace.chars().count();
        let token_len = token.chars().count();

        if current_len + pending_len + token_len > LINE_WIDTH && current_len > base_len {
            lines.push(current_line);
            current_line = String::new();
            current_line.push_str(continuation_prefix);
            base_len = continuation_prefix.chars().count();
            current_len = base_len;
            pending_whitespace.clear();
        } else {
            current_line.push_str(&pending_whitespace);
            current_len += pending_len;
            pending_whitespace.clear();
        }

        current_line.push_str(token);
        current_len += token_len;
    }

    if !pending_whitespace.is_empty() {
        current_line.push_str(&pending_whitespace);
    }

    lines.push(current_line);
    lines
}

fn write_plain_text<W: Write>(
    writer: &mut W,
    text: &str,
    state: &mut LineState<'_>,
) -> std::io::Result<()> {
    if text.is_empty() {
        return Ok(());
    }

    let mut start = 0;
    for (idx, ch) in text.char_indices() {
        if ch == '\n' {
            let chunk = &text[start..idx];
            if !chunk.is_empty() {
                let escaped = escape_markdown_text(chunk);
                state.write_chunk(writer, escaped.as_str())?;
            }
            state.write_chunk(writer, "\\")?;
            state.handle_newline(writer)?;
            start = idx + ch.len_utf8();
        }
    }

    if start < text.len() {
        let chunk = &text[start..];
        if !chunk.is_empty() {
            let escaped = escape_markdown_text(chunk);
            state.write_chunk(writer, escaped.as_str())?;
        }
    }

    Ok(())
}

fn inline_tags(style: InlineStyle) -> (&'static str, &'static str) {
    match style {
        InlineStyle::None => ("", ""),
        InlineStyle::Bold => ("**", "**"),
        InlineStyle::Italic => ("_", "_"),
        InlineStyle::Highlight => ("<mark>", "</mark>"),
        InlineStyle::Underline => ("<u>", "</u>"),
        InlineStyle::Strike => ("~~", "~~"),
        _ => ("", ""),
    }
}

fn escape_markdown_text(text: &str) -> String {
    fn needs_escape(ch: char) -> bool {
        matches!(
            ch,
            '\\' | '`'
                | '*'
                | '_'
                | '{'
                | '}'
                | '['
                | ']'
                | '('
                | ')'
                | '#'
                | '+'
                | '-'
                | '|'
                | '~'
        ) || ch == '<'
            || ch == '>'
            || ch == '&'
    }

    if !text.chars().any(needs_escape) {
        return text.to_string();
    }

    let mut escaped = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '\\' | '`' | '*' | '_' | '{' | '}' | '[' | ']' | '(' | ')' | '#' | '+' | '-' | '|'
            | '~' => {
                escaped.push('\\');
                escaped.push(ch);
            }
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '&' => escaped.push_str("&amp;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn escape_link_destination(dest: &str) -> String {
    const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

    let mut escaped = String::with_capacity(dest.len());

    for byte in dest.bytes() {
        match byte {
            b'a'..=b'z'
            | b'A'..=b'Z'
            | b'0'..=b'9'
            | b'-'
            | b'.'
            | b'_'
            | b'~'
            | b':'
            | b'/'
            | b'?'
            | b'#'
            | b'@'
            | b'!'
            | b'$'
            | b'&'
            | b'\''
            | b'*'
            | b'+'
            | b','
            | b';'
            | b'='
            | b'%'
            | b'['
            | b']' => escaped.push(byte as char),
            _ => {
                escaped.push('%');
                escaped.push(HEX_DIGITS[(byte >> 4) as usize] as char);
                escaped.push(HEX_DIGITS[(byte & 0x0F) as usize] as char);
            }
        }
    }

    escaped
}

fn write_code_span<W: Write>(
    writer: &mut W,
    span: &Span,
    state: &mut LineState<'_>,
) -> std::io::Result<()> {
    let mut content = String::new();
    collect_plain_text(span, &mut content);

    if content.contains('\r') {
        content = content.replace('\r', " ");
    }
    if content.contains('\n') {
        content = content.replace('\n', " ");
    }

    let delimiter_len = longest_backtick_sequence(&content) + 1;
    let delimiter = "`".repeat(delimiter_len.max(1));
    let needs_padding = content.starts_with(' ') || content.ends_with(' ');

    state.ensure_prefix(writer)?;
    writer.write_all(delimiter.as_bytes())?;
    if needs_padding {
        writer.write_all(b" ")?;
    }
    writer.write_all(content.as_bytes())?;
    if needs_padding {
        writer.write_all(b" ")?;
    }
    writer.write_all(delimiter.as_bytes())?;
    state.mark_written();
    Ok(())
}

fn collect_plain_text(span: &Span, buffer: &mut String) {
    if !span.text.is_empty() {
        buffer.push_str(&span.text);
    }
    for child in &span.children {
        collect_plain_text(child, buffer);
    }
}

fn longest_backtick_sequence(text: &str) -> usize {
    let mut max = 0;
    let mut current = 0;
    for ch in text.chars() {
        if ch == '`' {
            current += 1;
            if current > max {
                max = current;
            }
        } else {
            current = 0;
        }
    }
    max
}

struct LineState<'a> {
    continuation_prefix: &'a str,
    at_line_start: bool,
}

impl<'a> LineState<'a> {
    fn new(continuation_prefix: &'a str) -> Self {
        Self {
            continuation_prefix,
            at_line_start: false,
        }
    }

    fn write_chunk<W: Write>(&mut self, writer: &mut W, chunk: &str) -> std::io::Result<()> {
        if chunk.is_empty() {
            return Ok(());
        }
        self.ensure_prefix(writer)?;
        writer.write_all(chunk.as_bytes())?;
        self.at_line_start = false;
        Ok(())
    }

    fn ensure_prefix<W: Write>(&mut self, writer: &mut W) -> std::io::Result<()> {
        if self.at_line_start {
            if !self.continuation_prefix.is_empty() {
                writer.write_all(self.continuation_prefix.as_bytes())?;
            }
            self.at_line_start = false;
        }
        Ok(())
    }

    fn handle_newline<W: Write>(&mut self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(b"\n")?;
        self.at_line_start = true;
        Ok(())
    }

    fn mark_written(&mut self) {
        self.at_line_start = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_simple_paragraph() {
        let input = "Hello **world**!";
        let parsed = parse(Cursor::new(input)).unwrap();
        let expected = doc(vec![p_(vec![span("Hello "), b__("world"), span("!")])]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_header() {
        let input = "# Heading level 1";
        let parsed = parse(Cursor::new(input)).unwrap();
        let expected = doc(vec![h1_("Heading level 1")]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_unordered_list() {
        let input = "- First\n- Second";
        let parsed = parse(Cursor::new(input)).unwrap();
        let expected = doc(vec![ul_(vec![
            li_(vec![p__("First")]),
            li_(vec![p__("Second")]),
        ])]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_mark_highlight() {
        let input = "A <mark>highlighted</mark> word";
        let parsed = parse(Cursor::new(input)).unwrap();
        let expected = doc(vec![p_(vec![
            span("A "),
            mark__("highlighted"),
            span(" word"),
        ])]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_underline() {
        let input = "A <u>styled</u> word";
        let parsed = parse(Cursor::new(input)).unwrap();
        let expected = doc(vec![p_(vec![span("A "), u__("styled"), span(" word")])]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_del_strike() {
        let input = "A <del>struck</del> word";
        let parsed = parse(Cursor::new(input)).unwrap();
        let expected = doc(vec![p_(vec![span("A "), s__("struck"), span(" word")])]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_simple_paragraph() {
        let mut output = Vec::new();
        let doc = doc(vec![p__("Hello world!")]);

        write(&mut output, &doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert_eq!(result, "Hello world!\n");
    }

    #[test]
    fn test_header() {
        let mut output = Vec::new();
        let doc = doc(vec![h1_("Main Header")]);

        write(&mut output, &doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert_eq!(result, "# Main Header\n");
    }

    #[test]
    fn test_bold_text() {
        let mut output = Vec::new();
        let doc = doc(vec![p_(vec![
            span("This is "),
            b__("bold"),
            span(" text."),
        ])]);

        write(&mut output, &doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert_eq!(result, "This is **bold** text.\n");
    }

    #[test]
    fn test_italic_text() {
        let mut output = Vec::new();
        let doc = doc(vec![p_(vec![
            span("This is "),
            i__("italic"),
            span(" text."),
        ])]);

        write(&mut output, &doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert_eq!(result, "This is _italic_ text.\n");
    }

    #[test]
    fn test_quote() {
        let mut output = Vec::new();
        let doc = doc(vec![quote_(vec![p__("This is quoted.")])]);

        write(&mut output, &doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert_eq!(result, "> This is quoted.\n");
    }

    #[test]
    fn test_unordered_list() {
        let mut output = Vec::new();
        let doc = doc(vec![ul_(vec![
            li_(vec![p__("First item")]),
            li_(vec![p__("Second item")]),
        ])]);

        write(&mut output, &doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert_eq!(result, "- First item\n- Second item\n");
    }

    #[test]
    fn test_ordered_list() {
        let mut output = Vec::new();
        let doc = doc(vec![ol_(vec![
            li_(vec![p__("First item")]),
            li_(vec![p__("Second item")]),
        ])]);

        write(&mut output, &doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert_eq!(result, "1. First item\n2. Second item\n");
    }

    #[test]
    fn test_code() {
        let mut output = Vec::new();
        let doc = doc(vec![p_(vec![
            span("Use "),
            code__("println!()"),
            span(" for output."),
        ])]);

        write(&mut output, &doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert_eq!(result, "Use `println!()` for output.\n");
    }

    #[test]
    fn test_line_break() {
        let mut output = Vec::new();
        let doc = doc(vec![p_(vec![span("Line one\n"), span("Line two")])]);

        write(&mut output, &doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert_eq!(result, "Line one\\\nLine two\n");
    }

    #[test]
    fn test_parse_markdown_link() {
        let input = "See [docs](https://example.com)";
        let parsed = parse(Cursor::new(input)).unwrap();

        assert_eq!(parsed.paragraphs.len(), 1);
        let paragraph = &parsed.paragraphs[0];
        assert_eq!(paragraph.content.len(), 2);
        assert_eq!(paragraph.content[0].text, "See ");

        let link_span = &paragraph.content[1];
        assert_eq!(link_span.style, InlineStyle::Link);
        assert_eq!(
            link_span.link_target.as_deref(),
            Some("https://example.com")
        );
        assert_eq!(link_span.children.len(), 1);
        assert_eq!(link_span.children[0].text, "docs");
    }

    #[test]
    fn test_parse_markdown_link_without_description() {
        let input = "[https://example.com](https://example.com)";
        let parsed = parse(Cursor::new(input)).unwrap();

        assert_eq!(parsed.paragraphs.len(), 1);
        let paragraph = &parsed.paragraphs[0];
        assert_eq!(paragraph.content.len(), 1);

        let link_span = &paragraph.content[0];
        assert_eq!(link_span.style, InlineStyle::Link);
        assert_eq!(
            link_span.link_target.as_deref(),
            Some("https://example.com")
        );
        assert!(link_span.children.is_empty());
        assert!(link_span.text.is_empty());
    }

    #[test]
    fn test_write_markdown_links() {
        let doc = doc(vec![p_(vec![
            span("See "),
            link_text__("https://example.com/docs", "docs"),
            span(" and "),
            link__("https://example.com/quick"),
        ])]);

        let mut output = Vec::new();
        write(&mut output, &doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert_eq!(
            result,
            "See [docs](https://example.com/docs) and <https://example.com/quick>\n"
        );
    }
}
