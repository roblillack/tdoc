//! Convert between Markdown text and FTML [`Document`](crate::Document) trees.

use crate::metadata;
use crate::{ChecklistItem, Document, InlineStyle, Paragraph, ParagraphType, Span};
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::borrow::Cow;
use std::io::{Read, Write};

/// Parses Markdown into a [`Document`], including YAML metadata (frontmatter) if present.
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// use tdoc::{markdown, ParagraphType};
///
/// let doc = markdown::parse(Cursor::new("# Heading")).unwrap();
/// assert_eq!(doc.paragraphs[0].paragraph_type(), ParagraphType::Header1);
/// ```
///
/// With metadata (frontmatter):
///
/// ```
/// use std::io::Cursor;
/// use tdoc::markdown;
///
/// let input = "---\ntitle: Hello\n---\n\n# Heading";
/// let doc = markdown::parse(Cursor::new(input)).unwrap();
/// assert!(doc.metadata.is_some());
/// let meta = doc.metadata.as_ref().unwrap();
/// assert_eq!(meta.get("title").unwrap().as_str(), Some("Hello"));
/// ```
pub fn parse<R: Read>(mut reader: R) -> crate::Result<Document> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    // Extract metadata (frontmatter) if present
    let (metadata, content) = metadata::extract(&input)?;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_WIKILINKS);

    let parser = Parser::new_ext(content, options);
    let mut builder = MarkdownBuilder::new();

    for event in parser {
        builder.handle_event(event);
    }

    let mut doc = builder.finish();
    doc.metadata = metadata;
    Ok(doc)
}

/// Parses Markdown into a [`Document`] without processing metadata.
///
/// Use this if you want to parse metadata separately or don't expect
/// metadata (frontmatter) in the input.
pub fn parse_without_metadata<R: Read>(mut reader: R) -> crate::Result<Document> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_WIKILINKS);

    let parser = Parser::new_ext(&input, options);
    let mut builder = MarkdownBuilder::new();

    for event in parser {
        builder.handle_event(event);
    }

    Ok(builder.finish())
}

struct MarkdownBuilder {
    stack: Vec<BlockContext>,
    in_html_comment: bool,
}

impl MarkdownBuilder {
    fn new() -> Self {
        Self {
            stack: vec![BlockContext::Document {
                paragraphs: Vec::new(),
            }],
            in_html_comment: false,
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
                        checklist_items,
                        is_checklist,
                    }) => {
                        let paragraph = if is_checklist {
                            debug_assert!(entries.is_empty());
                            Paragraph::new_checklist().with_checklist_items(checklist_items)
                        } else if ordered {
                            Paragraph::new_ordered_list().with_entries(entries)
                        } else {
                            Paragraph::new_unordered_list().with_entries(entries)
                        };
                        self.add_paragraph_to_parent(paragraph);
                    }
                    Some(BlockContext::ListItem {
                        paragraphs,
                        checklist_state,
                    }) => {
                        if let Some(BlockContext::List {
                            entries,
                            checklist_items,
                            is_checklist,
                            ..
                        }) = self.stack.last_mut()
                        {
                            if let Some(checked) = checklist_state {
                                let item = Self::build_checklist_item(paragraphs, checked);
                                if !*is_checklist && !entries.is_empty() {
                                    let converted = entries
                                        .drain(..)
                                        .map(|entry| Self::build_checklist_item(entry, false))
                                        .collect::<Vec<_>>();
                                    checklist_items.extend(converted);
                                }
                                *is_checklist = true;
                                checklist_items.push(item);
                            } else if *is_checklist {
                                let item = Self::build_checklist_item(paragraphs, false);
                                checklist_items.push(item);
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
                        return Document {
                            metadata: None,
                            paragraphs,
                        };
                    }
                    None => break,
                }
            }
        }

        match self.stack.pop() {
            Some(BlockContext::Document { paragraphs }) => Document {
                metadata: None,
                paragraphs,
            },
            _ => Document::new(),
        }
    }

    fn handle_event(&mut self, event: Event<'_>) {
        match event {
            Event::Start(tag) => self.handle_start_tag(tag),
            Event::End(tag_end) => self.handle_end_tag(tag_end),
            Event::Text(text) => self.handle_text(text.as_ref()),
            Event::Html(html) => self.handle_html(html.as_ref()),
            Event::InlineHtml(html) => self.handle_html(html.as_ref()),
            Event::Code(text) => self.push_code(text.as_ref()),
            Event::FootnoteReference(reference) => {
                let marker = format!("[^{}]", reference);
                self.push_text(&marker);
            }
            Event::InlineMath(math) | Event::DisplayMath(math) => {
                self.push_text(math.as_ref());
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
            Tag::Heading { level, .. } => {
                let paragraph_type = match level {
                    HeadingLevel::H1 => ParagraphType::Header1,
                    HeadingLevel::H2 => ParagraphType::Header2,
                    HeadingLevel::H3 => ParagraphType::Header3,
                    _ => ParagraphType::Text,
                };
                self.start_paragraph(paragraph_type);
            }
            Tag::BlockQuote(_) => {
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
                    checklist_items: Vec::new(),
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
            Tag::Link { dest_url, .. } => {
                let span =
                    Span::new_styled(InlineStyle::Link).with_link_target(dest_url.into_string());
                self.ensure_paragraph().start_inline(span);
            }
            Tag::Image { dest_url, .. } => {
                let span =
                    Span::new_styled(InlineStyle::Link).with_link_target(dest_url.into_string());
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
            Tag::HtmlBlock
            | Tag::DefinitionList
            | Tag::DefinitionListTitle
            | Tag::DefinitionListDefinition
            | Tag::Superscript
            | Tag::Subscript
            | Tag::MetadataBlock(_) => {
                // Currently unsupported tags.
            }
        }
    }

    fn handle_end_tag(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Paragraph | TagEnd::Heading(_) => {
                self.finish_paragraph();
            }
            TagEnd::BlockQuote(_) => {
                self.close_open_paragraphs();
                if let Some(BlockContext::Quote { children }) = self.stack.pop() {
                    let paragraph = Paragraph::new_quote().with_children(children);
                    self.add_paragraph_to_parent(paragraph);
                }
            }
            TagEnd::List(_) => {
                self.close_open_paragraphs();
                if let Some(BlockContext::List {
                    ordered,
                    entries,
                    checklist_items,
                    is_checklist,
                }) = self.stack.pop()
                {
                    let paragraph = if is_checklist {
                        debug_assert!(entries.is_empty());
                        Paragraph::new_checklist().with_checklist_items(checklist_items)
                    } else if ordered {
                        Paragraph::new_ordered_list().with_entries(entries)
                    } else {
                        Paragraph::new_unordered_list().with_entries(entries)
                    };
                    self.add_paragraph_to_parent(paragraph);
                }
            }
            TagEnd::Item => {
                self.close_open_paragraphs();
                if let Some(BlockContext::ListItem {
                    paragraphs,
                    checklist_state,
                }) = self.stack.pop()
                {
                    if let Some(BlockContext::List {
                        entries,
                        checklist_items,
                        is_checklist,
                        ..
                    }) = self.stack.last_mut()
                    {
                        if let Some(checked) = checklist_state {
                            let item = Self::build_checklist_item(paragraphs, checked);
                            if !*is_checklist && !entries.is_empty() {
                                let converted = entries
                                    .drain(..)
                                    .map(|entry| Self::build_checklist_item(entry, false))
                                    .collect::<Vec<_>>();
                                checklist_items.extend(converted);
                            }
                            *is_checklist = true;
                            checklist_items.push(item);
                        } else if *is_checklist {
                            let item = Self::build_checklist_item(paragraphs, false);
                            checklist_items.push(item);
                        } else {
                            entries.push(paragraphs);
                        }
                    }
                }
            }
            TagEnd::Emphasis => {
                self.current_paragraph_inline_end(InlineStyle::Italic);
            }
            TagEnd::Strong => {
                self.current_paragraph_inline_end(InlineStyle::Bold);
            }
            TagEnd::Strikethrough => {
                self.current_paragraph_inline_end(InlineStyle::Strike);
            }
            TagEnd::Link | TagEnd::Image => {
                self.current_paragraph_inline_end(InlineStyle::Link);
            }
            TagEnd::CodeBlock => {
                self.finish_paragraph();
            }
            TagEnd::FootnoteDefinition => {
                self.finish_paragraph();
            }
            TagEnd::Table | TagEnd::TableHead | TagEnd::TableRow | TagEnd::TableCell => {
                self.close_open_paragraphs();
            }
            TagEnd::HtmlBlock
            | TagEnd::DefinitionList
            | TagEnd::DefinitionListTitle
            | TagEnd::DefinitionListDefinition
            | TagEnd::MetadataBlock(_)
            | TagEnd::Superscript
            | TagEnd::Subscript => {
                // Currently unsupported block types; ignore closures.
            }
        }
    }

    fn strip_html_comments<'a>(&mut self, html: &'a str) -> Option<Cow<'a, str>> {
        let mut remaining = html;
        let mut output: Option<String> = None;

        loop {
            if self.in_html_comment {
                if let Some(end_idx) = remaining.find("-->") {
                    remaining = &remaining[end_idx + 3..];
                    self.in_html_comment = false;
                } else {
                    return output.map(Cow::Owned);
                }
            }

            match remaining.find("<!--") {
                Some(start_idx) => {
                    let (before, rest) = remaining.split_at(start_idx);
                    if !before.is_empty() {
                        output.get_or_insert_with(String::new).push_str(before);
                    }

                    remaining = &rest[4..];
                    if let Some(end_idx) = remaining.find("-->") {
                        remaining = &remaining[end_idx + 3..];
                    } else {
                        self.in_html_comment = true;
                        return output.map(Cow::Owned);
                    }
                }
                None => {
                    if remaining.is_empty() {
                        return output.map(Cow::Owned);
                    }

                    if let Some(mut collected) = output {
                        collected.push_str(remaining);
                        return Some(Cow::Owned(collected));
                    } else {
                        return Some(Cow::Borrowed(remaining));
                    }
                }
            }
        }
    }

    fn handle_text(&mut self, text: &str) {
        let Some(text) = self.strip_html_comments(text) else {
            return;
        };

        if text.is_empty() {
            return;
        }

        self.push_text(text.as_ref());
    }

    fn handle_html(&mut self, html: &str) {
        let Some(html) = self.strip_html_comments(html) else {
            return;
        };

        let trimmed = html.trim();
        if trimmed.is_empty() {
            return;
        }

        if trimmed.starts_with("<!--") && trimmed.ends_with("-->") {
            // Drop HTML comments entirely.
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

        self.push_text(html.as_ref());
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
        paragraph.push_soft_break();
    }

    fn push_hard_break(&mut self) {
        let paragraph = self.ensure_paragraph();
        paragraph.push_hard_break();
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
        paragraph.content_mut().push(Span::new_text("---"));
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
                BlockContext::List {
                    entries,
                    checklist_items,
                    is_checklist,
                    ..
                } => {
                    if *is_checklist {
                        let item = Self::build_checklist_item(vec![paragraph], false);
                        checklist_items.push(item);
                    } else {
                        entries.push(vec![paragraph]);
                    }
                }
                BlockContext::Paragraph(context) => {
                    context.push_nested_paragraph(paragraph);
                }
            }
        }
    }

    fn build_checklist_item(paragraphs: Vec<Paragraph>, checked: bool) -> ChecklistItem {
        let mut item = ChecklistItem::new(checked);
        let mut content = Vec::new();

        for paragraph in paragraphs {
            match paragraph {
                Paragraph::Checklist { mut items } => item.children.append(&mut items),
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
        checklist_items: Vec<ChecklistItem>,
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

    fn span_target_mut(&mut self) -> &mut Vec<Span> {
        if let Some(parent) = self.inline_stack.last_mut() {
            &mut parent.children
        } else {
            &mut self.spans
        }
    }

    fn push_text(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }
        let normalized = text.replace('\u{2005}', " ");
        let span = Span::new_text(normalized);
        self.push_span(span);
    }

    fn push_soft_break(&mut self) {
        let target = self.span_target_mut();

        if let Some(last) = target.last_mut() {
            if last.style == InlineStyle::None
                && last.link_target.is_none()
                && last.children.is_empty()
            {
                if last.ends_with_whitespace() {
                    return;
                }

                last.text.push(' ');
                return;
            }
        }

        target.push(Span::new_text(" "));
    }

    fn push_hard_break(&mut self) {
        let inline_active = !self.inline_stack.is_empty();
        let target = self.span_target_mut();

        if inline_active {
            if let Some(last) = target.last_mut() {
                if last.style == InlineStyle::None
                    && last.link_target.is_none()
                    && last.children.is_empty()
                {
                    last.text.push('\n');
                    return;
                }
            }
        }

        target.push(Span::new_text("\n"));
    }

    fn push_code(&mut self, text: &str) {
        let mut span = Span::new_styled(InlineStyle::Code);
        if !text.is_empty() {
            span.children.push(Span::new_text(text));
        }
        self.push_span(span);
    }

    fn push_nested_paragraph(&mut self, paragraph: Paragraph) {
        match paragraph {
            Paragraph::Text { content }
            | Paragraph::Header1 { content }
            | Paragraph::Header2 { content }
            | Paragraph::Header3 { content }
            | Paragraph::CodeBlock { content } => {
                for span in content {
                    self.push_span(span);
                }
            }
            _ => {}
        }
    }

    fn start_inline(&mut self, span: Span) {
        self.inline_stack.push(span);
    }

    fn end_inline(&mut self, style: InlineStyle) {
        if let Some(mut span) = self.inline_stack.pop() {
            if span.style == InlineStyle::Link {
                span.strip_redundant_link_description();
                if let Some(target) = span.link_target.clone() {
                    if span.is_content_empty() && !target.contains(':') {
                        span.text = target;
                    }
                }
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
            && !a.text.contains('\n')
            && !b.text.contains('\n')
    }

    fn finish(mut self) -> Paragraph {
        while let Some(span) = self.inline_stack.pop() {
            self.push_span(span);
        }

        Paragraph::new(self.paragraph_type).with_content(self.spans)
    }
}

const LINE_WIDTH: usize = 80;

/// Serializes a [`Document`] structure back to Markdown, including metadata.
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
///
/// With metadata:
///
/// ```
/// use tdoc::{Document, Paragraph, Span};
/// use tdoc::markdown;
/// use tdoc::metadata::{Metadata, Value};
///
/// let mut meta = Metadata::new();
/// meta.insert("title".to_string(), Value::String("Test".to_string()));
///
/// let paragraph = Paragraph::new_text().with_content(vec![Span::new_text("Hello")]);
/// let document = Document::new()
///     .with_metadata(meta)
///     .with_paragraphs(vec![paragraph]);
///
/// let mut output = Vec::new();
/// markdown::write(&mut output, &document).unwrap();
/// let result = String::from_utf8(output).unwrap();
/// assert!(result.starts_with("---\n"));
/// assert!(result.contains("title: Test"));
/// ```
pub fn write<W: Write>(writer: &mut W, document: &Document) -> std::io::Result<()> {
    // Write metadata if present
    if let Some(ref meta) = document.metadata {
        let yaml = metadata::serialize(meta).map_err(std::io::Error::other)?;
        if !yaml.is_empty() {
            writer.write_all(yaml.as_bytes())?;
            // Add a blank line after metadata
            writer.write_all(b"\n")?;
        }
    }

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
        let mut current_prefix = if i == 0 { prefix } else { continuation_prefix };

        if i == 0
            && !prefix.is_empty()
            && prefix != continuation_prefix
            && needs_block_prefix_line(paragraph)
        {
            writer.write_all(prefix.as_bytes())?;
            writer.write_all(b"\n")?;
            current_prefix = continuation_prefix;
        }
        write_paragraph(writer, paragraph, current_prefix, continuation_prefix)?;
    }
    Ok(())
}

fn needs_block_prefix_line(paragraph: &Paragraph) -> bool {
    matches!(
        paragraph,
        Paragraph::OrderedList { .. }
            | Paragraph::UnorderedList { .. }
            | Paragraph::Checklist { .. }
    )
}

fn write_paragraph<W: Write>(
    writer: &mut W,
    paragraph: &Paragraph,
    prefix: &str,
    continuation_prefix: &str,
) -> std::io::Result<()> {
    match paragraph {
        Paragraph::Text { content } => {
            let content = render_spans_to_string(content)?;
            write_wrapped_lines(writer, prefix, continuation_prefix, &content)?;
        }
        Paragraph::CodeBlock { content } => {
            write_code_block(writer, prefix, continuation_prefix, content)?;
        }
        Paragraph::Header1 { content } => {
            let content = render_spans_to_string(content)?;
            let first_prefix = format!("{}# ", prefix);
            write_wrapped_lines(writer, &first_prefix, continuation_prefix, &content)?;
        }
        Paragraph::Header2 { content } => {
            let content = render_spans_to_string(content)?;
            let first_prefix = format!("{}## ", prefix);
            write_wrapped_lines(writer, &first_prefix, continuation_prefix, &content)?;
        }
        Paragraph::Header3 { content } => {
            let content = render_spans_to_string(content)?;
            let first_prefix = format!("{}### ", prefix);
            write_wrapped_lines(writer, &first_prefix, continuation_prefix, &content)?;
        }
        Paragraph::Quote { children } => {
            let quote_prefix = format!("{}> ", prefix);
            let quote_continuation = format!("{}> ", continuation_prefix);

            for (idx, child) in children.iter().enumerate() {
                if idx > 0 {
                    write!(writer, "{}", quote_continuation)?;
                    writeln!(writer)?;
                }
                write_paragraph(writer, child, &quote_prefix, &quote_continuation)?;
            }
        }
        Paragraph::UnorderedList { entries } => {
            for entry in entries {
                let bullet_prefix = format!("{}- ", prefix);
                let bullet_continuation = format!("{}  ", continuation_prefix);

                write_paragraphs(writer, entry, &bullet_prefix, &bullet_continuation)?;
            }
        }
        Paragraph::OrderedList { entries } => {
            for (i, entry) in entries.iter().enumerate() {
                let marker = format!("{}. ", i + 1);
                let bullet_prefix = format!("{}{}", prefix, marker);
                let bullet_continuation =
                    format!("{}{}", continuation_prefix, " ".repeat(marker.len()));

                write_paragraphs(writer, entry, &bullet_prefix, &bullet_continuation)?;
            }
        }
        Paragraph::Checklist { items } => {
            write_checklist_items(writer, items, prefix, continuation_prefix)?;
        }
    }
    Ok(())
}

fn write_checklist_items<W: Write>(
    writer: &mut W,
    items: &[ChecklistItem],
    prefix: &str,
    continuation_prefix: &str,
) -> std::io::Result<()> {
    for item in items {
        let marker = if item.checked { 'x' } else { ' ' };
        let content = render_spans_to_string(&item.content)?;
        let first_prefix = format!("{}- [{}] ", prefix, marker);
        let continuation = format!("{}{}", continuation_prefix, " ".repeat(6));
        write_wrapped_lines(writer, &first_prefix, &continuation, &content)?;

        if !item.children.is_empty() {
            let child_prefix = format!("{}  ", prefix);
            let child_continuation = format!("{}  ", continuation_prefix);
            write_checklist_items(writer, &item.children, &child_prefix, &child_continuation)?;
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
        let mut ends_with_newline = false;

        for chunk in normalized.split_inclusive('\n') {
            ends_with_newline = chunk.ends_with('\n');
            write!(writer, "{}{}", continuation_prefix, chunk)?;
        }

        if !ends_with_newline {
            writeln!(writer)?;
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
    for (idx, span) in spans.iter().enumerate() {
        let has_more = idx + 1 < spans.len();
        write_span(writer, span, state, has_more)?;
    }
    Ok(())
}

fn write_span<W: Write>(
    writer: &mut W,
    span: &Span,
    state: &mut LineState<'_>,
    has_more_siblings: bool,
) -> std::io::Result<()> {
    match span.style {
        InlineStyle::Link => {
            if let Some(target) = &span.link_target {
                if span.has_content() {
                    state.write_chunk(writer, "[")?;
                    write_span_content(writer, span, state, has_more_siblings)?;
                    let closing = format!("]({})", escape_link_destination(target));
                    state.write_chunk(writer, &closing)?;
                } else {
                    let autop = format!("<{}>", escape_link_destination(target));
                    state.write_chunk(writer, &autop)?;
                }
                Ok(())
            } else {
                write_span_content(writer, span, state, has_more_siblings)
            }
        }
        InlineStyle::Code => write_code_span(writer, span, state),
        style => {
            let (begin_tag, end_tag) = inline_tags(style);
            if !begin_tag.is_empty() {
                state.write_chunk(writer, begin_tag)?;
            }
            write_span_content(writer, span, state, has_more_siblings)?;
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
    has_more_siblings: bool,
) -> std::io::Result<()> {
    if !span.text.is_empty() {
        write_plain_text(
            writer,
            &span.text,
            has_more_siblings || !span.children.is_empty(),
            state,
        )?;
    }

    for (idx, child) in span.children.iter().enumerate() {
        let child_has_more = idx + 1 < span.children.len() || has_more_siblings;
        write_span(writer, child, state, child_has_more)?;
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
    has_more_content: bool,
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
                let escaped = escape_markdown_text(chunk, state.is_at_line_start(), false);
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
            let is_final_chunk = !has_more_content;
            let escaped = escape_markdown_text(chunk, state.is_at_line_start(), is_final_chunk);
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

fn escape_markdown_text(text: &str, line_start: bool, is_final_chunk: bool) -> String {
    if text.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = text.chars().collect();
    let mut escaped = String::with_capacity(text.len());
    let mut idx = 0;
    let mut at_line_start = line_start;

    while idx < chars.len() {
        let ch = chars[idx];
        if ch == ' ' {
            let mut run_end = idx;
            while run_end < chars.len() && chars[run_end] == ' ' {
                run_end += 1;
            }
            let run_len = run_end - idx;
            let is_leading = at_line_start;
            let is_trailing = run_end == chars.len();
            let encode_trailing = is_trailing && is_final_chunk;

            if run_len > 1 || is_leading || encode_trailing {
                for _ in 0..run_len {
                    escaped.push_str("&emsp14;");
                }
            } else {
                escaped.push(' ');
            }
            idx = run_end;
            at_line_start = false;
            continue;
        }

        match ch {
            '\\' | '`' | '*' | '_' | '{' | '}' | '[' | ']' | '(' | ')' | '#' | '+' | '-' | '|'
            | '~' => {
                escaped.push('\\');
                escaped.push(ch);
            }
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '&' => escaped.push_str("&amp;"),
            '\u{2005}' => escaped.push_str("&emsp14;"),
            '\u{00A0}' => escaped.push_str("&nbsp;"),
            _ => escaped.push(ch),
        }

        idx += 1;
        at_line_start = false;
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

    fn is_at_line_start(&self) -> bool {
        self.at_line_start
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
    use crate::{ftml, test_helpers::*};
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
        assert_eq!(paragraph.content().len(), 2);
        assert_eq!(paragraph.content()[0].text, "See ");

        let link_span = &paragraph.content()[1];
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

    #[test]
    fn test_nested_lists_roundtrip_inside_quote() {
        let inner_list = ol_(vec![
            li_(vec![p__("One")]),
            li_(vec![p__("Two")]),
            li_(vec![p__("Three")]),
        ]);

        let quoted_list = quote_(vec![ul_(vec![
            li_(vec![p__("Text inside quote list")]),
            li_(vec![inner_list]),
        ])]);

        let doc = doc(vec![quoted_list]);

        let mut markdown = Vec::new();
        write(&mut markdown, &doc).unwrap();

        let reparsed = parse(Cursor::new(markdown.as_slice())).unwrap();
        assert_eq!(reparsed, doc);
    }

    #[test]
    fn test_whitespace_edge_in_span() {
        fn r(s: &str) -> Document {
            parse(Cursor::new(s)).unwrap()
        }
        fn w(d: Document) -> String {
            let mut output = Vec::new();
            write(&mut output, &d).unwrap();
            String::from_utf8(output).unwrap()
        }
        assert_eq!(
            w(ftml! { p { link { "yadayada" "Hier kommt ein Test! " } } }),
            "[Hier kommt ein Test!&emsp14;](yadayada)\n",
        );

        assert_eq!(
            r("[Hier kommt ein Test!&emsp14;](yadayada)\n"),
            ftml! { p { link { "yadayada" "Hier kommt ein Test! " } } },
        );

        // with newline
        assert_eq!(
            r("[Hier kommt ein Test!\n](yadayada)\n"),
            ftml! { p { link { "yadayada" "Hier kommt ein Test! " } } },
        );
    }

    #[test]
    fn test_write_whitespace_edge_in_span_with_wrapping() {
        fn w(d: Document) -> String {
            let mut output = Vec::new();
            write(&mut output, &d).unwrap();
            String::from_utf8(output).unwrap()
        }
        let word = format!("{} ", "A".repeat(LINE_WIDTH - 11));
        assert_eq!(
            w(ftml! { p { link { "TARGET" word } " BBBB"} }),
            "[AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA ](TARGET)\nBBBB\n",
        );
    }

    #[test]
    fn test_parse_whitespace_edge_in_span_with_wrapping() {
        fn r(s: &str) -> Document {
            parse(Cursor::new(s)).unwrap()
        }

        let word = format!("{} ", "A".repeat(LINE_WIDTH - 11));
        assert_eq!(
            r("[AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA ](TARGET)\nBBBB\n"),
            ftml! { p { link { "TARGET" word } " BBBB"} },
        );
    }
}
