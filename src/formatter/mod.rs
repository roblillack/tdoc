//! Render documents to formatted plain text suitable for terminals or logs.

use crate::{Document, InlineStyle, Paragraph, ParagraphType, Span};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::io::Write;

const DEFAULT_WRAP_WIDTH: usize = 72;
const DEFAULT_QUOTE_PREFIX: &str = "| ";
const DEFAULT_UNORDERED_LIST_ITEM_PREFIX: &str = " • ";

static ANSI_ESCAPE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\x1b\[[0-9;]*m").expect("valid ANSI escape regex"));
static OSC8_SEQUENCE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\x1b]8;([^;]*);([^\x1b]*)\x1b\\").expect("valid OSC8 regex"));
static OSC8_ESCAPE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\x1b]8;[^\x1b]*\x1b\\").expect("valid OSC8 escape regex"));

#[derive(Clone)]
/// Opening and closing escape sequences for a particular inline style.
pub struct StyleTags {
    pub begin: String,
    pub end: String,
}

impl StyleTags {
    pub fn new(begin: impl Into<String>, end: impl Into<String>) -> Self {
        Self {
            begin: begin.into(),
            end: end.into(),
        }
    }
}

/// Controls how inline link references are rendered when links need textual markers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LinkIndexFormat {
    /// Render inline link markers as superscript-style Arabic numerals.
    #[default]
    SuperscriptArabic,
    /// Render inline link markers as bracketed numbers such as `[1]`.
    Bracketed,
}

#[derive(Clone)]
/// High-level configuration that influences how the [`Formatter`] renders output.
pub struct FormattingStyle {
    pub reset_styles: String,
    pub text_styles: HashMap<InlineStyle, StyleTags>,
    pub quote_prefix: String,
    pub unordered_list_item_prefix: String,
    pub wrap_width: usize,
    pub left_padding: usize,
    /// When set, wrap link text in OSC 8 control sequences so supporting terminals emit clickable hyperlinks.
    pub enable_osc8_hyperlinks: bool,
    /// Selects the text marker style used for numbering links when hyperlinks require an inline index.
    pub link_index_format: LinkIndexFormat,
    /// When true, numbered link references are emitted after each section.
    pub link_footnotes: bool,
}

impl Default for FormattingStyle {
    fn default() -> Self {
        Self {
            reset_styles: String::new(),
            text_styles: HashMap::new(),
            quote_prefix: DEFAULT_QUOTE_PREFIX.to_string(),
            unordered_list_item_prefix: DEFAULT_UNORDERED_LIST_ITEM_PREFIX.to_string(),
            wrap_width: DEFAULT_WRAP_WIDTH,
            left_padding: 0,
            enable_osc8_hyperlinks: false,
            link_index_format: LinkIndexFormat::default(),
            link_footnotes: true,
        }
    }
}

impl FormattingStyle {
    /// Creates a plain ASCII style without color or terminal escape sequences.
    pub fn ascii() -> Self {
        Self::default()
    }

    /// Creates a style that emits ANSI escape codes for bold, italic, and other emphasis.
    pub fn ansi() -> Self {
        let mut text_styles = HashMap::new();
        text_styles.insert(InlineStyle::Bold, StyleTags::new("\x1b[1m", "\x1b[22m"));
        text_styles.insert(InlineStyle::Italic, StyleTags::new("\x1b[3m", "\x1b[23m"));
        text_styles.insert(
            InlineStyle::Highlight,
            StyleTags::new("\x1b[7m", "\x1b[27m"),
        );
        text_styles.insert(
            InlineStyle::Underline,
            StyleTags::new("\x1b[4m", "\x1b[24m"),
        );
        text_styles.insert(InlineStyle::Strike, StyleTags::new("\x1b[9m", "\x1b[29m"));

        Self {
            reset_styles: "\x1b[0m".to_string(),
            text_styles,
            quote_prefix: DEFAULT_QUOTE_PREFIX.to_string(),
            unordered_list_item_prefix: DEFAULT_UNORDERED_LIST_ITEM_PREFIX.to_string(),
            wrap_width: DEFAULT_WRAP_WIDTH,
            left_padding: 0,
            enable_osc8_hyperlinks: true,
            link_index_format: LinkIndexFormat::default(),
            link_footnotes: true,
        }
    }
}

/// Pretty-prints [`Document`] trees using the supplied [`FormattingStyle`].
///
/// # Examples
///
/// ```
/// use tdoc::{Document, Paragraph, Span};
/// use tdoc::formatter::Formatter;
///
/// let doc = Document::new().with_paragraphs(vec![
///     Paragraph::new_text().with_content(vec![Span::new_text("Hello")])
/// ]);
///
/// let mut output = Vec::new();
/// Formatter::new_ascii(&mut output).write_document(&doc).unwrap();
/// assert_eq!(String::from_utf8(output).unwrap(), "Hello\n");
/// ```
pub struct Formatter<W: Write> {
    pub style: FormattingStyle,
    writer: W,
    pending_links: Vec<LinkReference>,
    link_indices: HashMap<String, usize>,
    next_link_index: usize,
    next_hyperlink_id: usize,
}

#[derive(Clone, Debug)]
struct LinkReference {
    index: usize,
    target: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Osc8Link {
    id: Option<String>,
    target: String,
}

impl Osc8Link {
    fn new(id: Option<String>, target: impl Into<String>) -> Self {
        Self {
            id,
            target: target.into(),
        }
    }
}

impl<W: Write> Formatter<W> {
    /// Creates a formatter over the given writer with the provided style.
    pub fn new(writer: W, style: FormattingStyle) -> Self {
        Self {
            writer,
            style,
            pending_links: Vec::new(),
            link_indices: HashMap::new(),
            next_link_index: 1,
            next_hyperlink_id: 1,
        }
    }

    /// Creates a formatter that produces plain ASCII output.
    pub fn new_ascii(writer: W) -> Self {
        Self::new(writer, FormattingStyle::ascii())
    }

    /// Creates a formatter that emits ANSI escape sequences for styling.
    pub fn new_ansi(writer: W) -> Self {
        Self::new(writer, FormattingStyle::ansi())
    }

    /// Writes the entire document into the wrapped writer.
    pub fn write_document(&mut self, document: &Document) -> std::io::Result<()> {
        self.next_hyperlink_id = 1;
        let indent = " ".repeat(self.style.left_padding);
        self.write_paragraphs(&document.paragraphs, &indent, &indent, &indent)?;
        let _ = self.flush_pending_links(&indent)?;

        // Write reset styles if we have any
        if !self.style.reset_styles.is_empty() {
            write!(self.writer, "{}", self.style.reset_styles)?;
        }

        Ok(())
    }

    fn write_paragraphs(
        &mut self,
        paragraphs: &[Paragraph],
        prefix: &str,
        continuation_prefix: &str,
        blank_line_prefix: &str,
    ) -> std::io::Result<()> {
        self.write_paragraphs_with_prefixes(
            paragraphs,
            &[],
            prefix,
            continuation_prefix,
            blank_line_prefix,
        )
    }

    fn write_paragraphs_with_prefixes(
        &mut self,
        paragraphs: &[Paragraph],
        first_line_prefixes: &[&str],
        default_first_line_prefix: &str,
        continuation_prefix: &str,
        blank_line_prefix: &str,
    ) -> std::io::Result<()> {
        let mut previous_type: Option<ParagraphType> = None;

        for (idx, paragraph) in paragraphs.iter().enumerate() {
            let flushed_links = if matches!(
                paragraph.paragraph_type,
                ParagraphType::Header1 | ParagraphType::Header2 | ParagraphType::Header3
            ) {
                self.flush_pending_links(blank_line_prefix)?
            } else {
                false
            };
            let previous_after = previous_type
                .map(|ty| self.blank_lines_after(ty))
                .unwrap_or(0);
            let mut blank_lines = self.blank_lines_before(previous_type, paragraph.paragraph_type);
            if flushed_links && blank_lines > 0 {
                blank_lines -= 1;
            }
            self.write_blank_lines_with_prefix(blank_line_prefix, previous_after.max(blank_lines))?;
            let paragraph_prefix = if idx < first_line_prefixes.len() {
                first_line_prefixes[idx]
            } else {
                default_first_line_prefix
            };

            self.write_paragraph(
                paragraph,
                paragraph_prefix,
                continuation_prefix,
                blank_line_prefix,
            )?;
            previous_type = Some(paragraph.paragraph_type);
        }

        if let Some(last_type) = previous_type {
            self.write_blank_lines_with_prefix(
                blank_line_prefix,
                self.blank_lines_after(last_type),
            )?;
        }

        Ok(())
    }

    fn flush_pending_links(&mut self, prefix: &str) -> std::io::Result<bool> {
        if !self.style.link_footnotes {
            if !self.pending_links.is_empty() {
                self.pending_links.clear();
            }
            if !self.link_indices.is_empty() {
                self.link_indices.clear();
            }
            self.next_link_index = 1;
            return Ok(false);
        }

        if self.pending_links.is_empty() {
            self.link_indices.clear();
            self.next_link_index = 1;
            return Ok(false);
        }

        self.write_blank_lines_with_prefix(prefix, 1)?;

        let links = std::mem::take(&mut self.pending_links);
        self.link_indices.clear();

        let max_label_width = links
            .last()
            .map(|link| {
                let formatted = self.format_link_index(link.index);
                formatted.chars().count()
            })
            .unwrap_or(1);

        for link in &links {
            let label = self.link_label(link.index, max_label_width);
            let first_prefix = format!("{}{}", prefix, label);
            let continuation_prefix = format!("{}{}", prefix, " ".repeat(label.chars().count()));
            let footnote_text = if self.style.enable_osc8_hyperlinks {
                let hyperlink = self.next_osc8_link(&link.target);
                self.osc8_wrap(&hyperlink, &link.target)
            } else {
                link.target.clone()
            };
            let parts = vec![footnote_text];
            self.write_wrapped_text(&parts, &first_prefix, &continuation_prefix)?;
            writeln!(self.writer)?;
        }

        self.next_link_index = 1;
        Ok(true)
    }

    fn write_paragraph(
        &mut self,
        paragraph: &Paragraph,
        prefix: &str,
        continuation_prefix: &str,
        blank_line_prefix: &str,
    ) -> std::io::Result<()> {
        match paragraph.paragraph_type {
            ParagraphType::Header1 => {
                self.write_header1_paragraph(&paragraph.content, prefix)?;
            }
            ParagraphType::Header2 => {
                self.write_header2_paragraph(&paragraph.content, prefix)?;
            }
            ParagraphType::Header3 => {
                self.write_header3_paragraph(&paragraph.content, prefix)?;
            }
            ParagraphType::Text => {
                self.write_text_paragraph(&paragraph.content, prefix, continuation_prefix)?;
            }
            ParagraphType::ChecklistItem => {
                let marker = if paragraph.checklist_item_checked.unwrap_or(false) {
                    "[✓] "
                } else {
                    "[ ] "
                };
                let first_prefix = format!("{}{}", prefix, marker);
                let continuation = format!(
                    "{}{}",
                    continuation_prefix,
                    " ".repeat(marker.chars().count())
                );
                self.write_checklist_text(paragraph, &first_prefix, &continuation)?;
                writeln!(self.writer)?;
            }
            ParagraphType::CodeBlock => {
                self.write_code_block_paragraph(&paragraph.content, prefix, continuation_prefix)?;
            }
            ParagraphType::Quote => {
                let quote_continuation =
                    format!("{}{}", continuation_prefix, self.style.quote_prefix);

                let shared_prefix_len = prefix
                    .chars()
                    .zip(continuation_prefix.chars())
                    .take_while(|(a, b)| a == b)
                    .count();
                let list_context = prefix != continuation_prefix
                    && shared_prefix_len > 0
                    && !paragraph.children.is_empty()
                    && paragraph.children.len() > 1
                    && matches!(
                        paragraph.children.first().map(|p| p.paragraph_type),
                        Some(ParagraphType::Text)
                    );

                if list_context {
                    let quote_prefix =
                        format!("{}{}", continuation_prefix, self.style.quote_prefix);

                    // Maintain owned storage for custom prefixes so borrowed slices stay valid.
                    let owned_prefixes = [quote_prefix, quote_continuation.clone()];

                    let default_first_prefix = owned_prefixes[0].as_str();
                    let continuation = owned_prefixes[1].as_str();
                    let first_line_prefixes = [prefix];

                    self.write_paragraphs_with_prefixes(
                        &paragraph.children,
                        &first_line_prefixes,
                        default_first_prefix,
                        continuation,
                        continuation_prefix,
                    )?;
                } else {
                    let quote_prefix = format!("{}{}", prefix, self.style.quote_prefix);

                    self.write_paragraphs(
                        &paragraph.children,
                        &quote_prefix,
                        &quote_continuation,
                        &quote_prefix,
                    )?;
                }
            }
            ParagraphType::UnorderedList => {
                for (idx, entry) in paragraph.entries.iter().enumerate() {
                    if idx > 0 {
                        self.write_blank_lines_with_prefix(blank_line_prefix, 1)?;
                    }

                    let base_prefix = continuation_prefix;
                    let bullet_prefix =
                        format!("{}{}", base_prefix, self.style.unordered_list_item_prefix);
                    let bullet_continuation = {
                        let desired_width = bullet_prefix.chars().count();
                        let current_width = base_prefix.chars().count();
                        let mut continuation = base_prefix.to_string();
                        if desired_width > current_width {
                            continuation.push_str(&" ".repeat(desired_width - current_width));
                        }
                        continuation
                    };

                    self.write_paragraphs_with_prefixes(
                        entry,
                        &[bullet_prefix.as_str()],
                        &bullet_continuation,
                        &bullet_continuation,
                        &bullet_continuation,
                    )?;
                }
            }
            ParagraphType::OrderedList => {
                for (i, entry) in paragraph.entries.iter().enumerate() {
                    if i > 0 {
                        self.write_blank_lines_with_prefix(blank_line_prefix, 1)?;
                    }

                    let base_prefix = continuation_prefix;
                    let numbering_prefix = if i == 0 && prefix != continuation_prefix {
                        prefix
                    } else {
                        base_prefix
                    };

                    let bullet_prefix = format!("{}{:2}. ", numbering_prefix, i + 1);
                    let bullet_continuation = {
                        let desired_width = bullet_prefix.chars().count();
                        let current_width = base_prefix.chars().count();
                        let mut continuation = base_prefix.to_string();
                        if desired_width > current_width {
                            continuation.push_str(&" ".repeat(desired_width - current_width));
                        }
                        continuation
                    };

                    self.write_paragraphs_with_prefixes(
                        entry,
                        &[bullet_prefix.as_str()],
                        &bullet_continuation,
                        &bullet_continuation,
                        &bullet_continuation,
                    )?;
                }
            }
            ParagraphType::Checklist => {
                for entry in &paragraph.entries {
                    let marker_item = entry
                        .iter()
                        .find(|p| p.paragraph_type == ParagraphType::ChecklistItem)
                        .or_else(|| entry.first());

                    if let Some(item) = marker_item {
                        let marker = if item.checklist_item_checked.unwrap_or(false) {
                            "[✓] "
                        } else {
                            "[ ] "
                        };
                        let base_prefix = continuation_prefix;
                        let first_prefix = format!("{}{}", base_prefix, marker);
                        let continuation = format!(
                            "{}{}",
                            continuation_prefix,
                            " ".repeat(marker.chars().count())
                        );
                        self.write_checklist_text(item, &first_prefix, &continuation)?;
                        writeln!(self.writer)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn write_code_block_paragraph(
        &mut self,
        spans: &[Span],
        prefix: &str,
        continuation_prefix: &str,
    ) -> std::io::Result<()> {
        self.write_code_block_fence(prefix)?;

        let mut code_text = Self::collect_code_text(spans);
        if !code_text.is_empty() {
            code_text = code_text.replace("\r\n", "\n").replace('\r', "\n");
            let mut lines = code_text.split('\n').peekable();
            while let Some(line) = lines.next() {
                let is_last = lines.peek().is_none();
                if is_last && line.is_empty() {
                    break;
                }
                self.write_hard_wrapped_code_line(line, continuation_prefix)?;
            }
        }

        self.write_code_block_fence(continuation_prefix)?;
        Ok(())
    }

    fn write_hard_wrapped_code_line(
        &mut self,
        line: &str,
        continuation_prefix: &str,
    ) -> std::io::Result<()> {
        let available_width = self
            .style
            .wrap_width
            .saturating_sub(continuation_prefix.chars().count())
            .max(1);

        if line.is_empty() {
            writeln!(self.writer, "{}", continuation_prefix)?;
            return Ok(());
        }

        let mut remaining = line;
        while !remaining.is_empty() {
            let mut end_idx = 0;
            for (count, (idx, ch)) in remaining.char_indices().enumerate() {
                if count >= available_width {
                    break;
                }
                end_idx = idx + ch.len_utf8();
            }

            if end_idx == 0 {
                end_idx = remaining.len();
            }

            let (chunk, rest) = remaining.split_at(end_idx);
            writeln!(self.writer, "{}{}", continuation_prefix, chunk)?;
            remaining = rest;
        }

        Ok(())
    }

    fn write_code_block_fence(&mut self, prefix: &str) -> std::io::Result<()> {
        const MIN_FENCE_WIDTH: usize = 4;
        let available_width = self.style.wrap_width.saturating_sub(prefix.chars().count());
        let dash_count = available_width.max(MIN_FENCE_WIDTH);
        writeln!(self.writer, "{}{}", prefix, "-".repeat(dash_count))
    }

    fn collect_code_text(spans: &[Span]) -> String {
        let mut buffer = String::new();
        for span in spans {
            Self::append_plain_text(span, &mut buffer);
        }
        buffer
    }

    fn append_plain_text(span: &Span, buffer: &mut String) {
        if !span.text.is_empty() {
            buffer.push_str(&span.text);
        }
        for child in &span.children {
            Self::append_plain_text(child, buffer);
        }
    }

    fn write_blank_lines_with_prefix(&mut self, prefix: &str, count: usize) -> std::io::Result<()> {
        for _ in 0..count {
            if prefix.is_empty() {
                writeln!(self.writer)?;
            } else {
                writeln!(self.writer, "{}", prefix)?;
            }
        }
        Ok(())
    }

    fn blank_lines_before(
        &self,
        previous_type: Option<ParagraphType>,
        current_type: ParagraphType,
    ) -> usize {
        match current_type {
            ParagraphType::Header1 => 3,
            ParagraphType::Header2 => 3,
            ParagraphType::Header3 => 2,
            _ => match previous_type {
                Some(_) => 1,
                None => 0,
            },
        }
    }

    fn blank_lines_after(&self, paragraph_type: ParagraphType) -> usize {
        match paragraph_type {
            ParagraphType::Header1 => 3,
            ParagraphType::Header2 => 2,
            ParagraphType::Header3 => 1,
            _ => 0,
        }
    }

    fn render_heading_text(&mut self, spans: &[Span]) -> std::io::Result<(String, usize)> {
        let mut parts = Vec::new();
        for span in spans {
            self.collect_formatted_text(span, &mut parts)?;
        }

        let mut combined = String::new();
        for part in parts {
            if part == "\n" {
                combined.push(' ');
            } else {
                combined.push_str(&part);
            }
        }

        let trimmed = combined.trim().to_string();
        if trimmed.is_empty() {
            return Ok((String::new(), 0));
        }

        let bold_text = self.apply_bold(&trimmed);
        let visible_width = self.visible_width(&bold_text);
        Ok((bold_text, visible_width))
    }

    fn write_header1_paragraph(&mut self, spans: &[Span], prefix: &str) -> std::io::Result<()> {
        let (bold_text, visible_width) = self.render_heading_text(spans)?;

        let prefix_width = prefix.chars().count();
        let available_width = self.style.wrap_width.saturating_sub(prefix_width);
        if visible_width <= available_width {
            let padding = if available_width > visible_width {
                (available_width - visible_width) / 2
            } else {
                0
            };

            write!(self.writer, "{}", prefix)?;
            for _ in 0..padding {
                write!(self.writer, " ")?;
            }
            write!(self.writer, "{}", bold_text)?;
            writeln!(self.writer)?;
        } else {
            let parts = vec![bold_text];
            self.write_wrapped_text(&parts, prefix, prefix)?;
            writeln!(self.writer)?;
        }

        Ok(())
    }

    fn write_header2_paragraph(&mut self, spans: &[Span], prefix: &str) -> std::io::Result<()> {
        let (bold_text, _) = self.render_heading_text(spans)?;
        let prefix_width = prefix.chars().count();
        let parts = vec![bold_text];
        let line_widths = self.measure_wrapped_lines(&parts, prefix_width, prefix_width);

        self.write_wrapped_text(&parts, prefix, prefix)?;
        writeln!(self.writer)?;

        let underline_width = line_widths.into_iter().max().unwrap_or(0);
        write!(self.writer, "{}", prefix)?;
        for _ in 0..underline_width {
            write!(self.writer, "=")?;
        }
        writeln!(self.writer)?;

        Ok(())
    }

    fn write_header3_paragraph(&mut self, spans: &[Span], prefix: &str) -> std::io::Result<()> {
        let (bold_text, _) = self.render_heading_text(spans)?;
        let prefix_width = prefix.chars().count();
        let parts = vec![bold_text];
        let line_widths = self.measure_wrapped_lines(&parts, prefix_width, prefix_width);

        self.write_wrapped_text(&parts, prefix, prefix)?;
        writeln!(self.writer)?;

        let underline_width = line_widths.into_iter().max().unwrap_or(0);
        write!(self.writer, "{}", prefix)?;
        for _ in 0..underline_width {
            write!(self.writer, "-")?;
        }
        writeln!(self.writer)?;

        Ok(())
    }

    fn write_text_paragraph(
        &mut self,
        spans: &[Span],
        prefix: &str,
        continuation_prefix: &str,
    ) -> std::io::Result<()> {
        if spans.is_empty() {
            writeln!(self.writer)?;
            return Ok(());
        }

        // Build the formatted text first
        let mut text_parts = Vec::new();
        for span in spans {
            self.collect_formatted_text(span, &mut text_parts)?;
        }

        // Now write with proper wrapping
        self.write_wrapped_text(&text_parts, prefix, continuation_prefix)?;
        writeln!(self.writer)?;

        Ok(())
    }

    fn write_checklist_text(
        &mut self,
        item: &Paragraph,
        first_prefix: &str,
        continuation_prefix: &str,
    ) -> std::io::Result<()> {
        let mut text_parts = Vec::new();
        for span in &item.content {
            self.collect_formatted_text(span, &mut text_parts)?;
        }

        if text_parts.is_empty() {
            write!(self.writer, "{}", first_prefix)?;
        } else {
            self.write_wrapped_text(&text_parts, first_prefix, continuation_prefix)?;
        }

        Ok(())
    }

    fn apply_bold(&self, text: &str) -> String {
        if text.is_empty() {
            return String::new();
        }

        if let Some(style_tags) = self.style.text_styles.get(&InlineStyle::Bold) {
            format!("{}{}{}", style_tags.begin, text, style_tags.end)
        } else {
            text.to_string()
        }
    }

    fn collect_formatted_text(
        &mut self,
        span: &Span,
        parts: &mut Vec<String>,
    ) -> std::io::Result<()> {
        if span.style == InlineStyle::Link {
            return self.collect_link_text(span, parts);
        }

        if span.children.is_empty() {
            self.push_text_fragment(parts, &span.text);
        } else {
            if let Some(style_tags) = self.style.text_styles.get(&span.style) {
                parts.push(style_tags.begin.clone());
            }

            if !span.text.is_empty() {
                self.push_text_fragment(parts, &span.text);
            }

            for child in &span.children {
                self.collect_formatted_text(child, parts)?;
            }

            if let Some(style_tags) = self.style.text_styles.get(&span.style) {
                parts.push(style_tags.end.clone());
            }
        }

        Ok(())
    }

    fn collect_link_text(&mut self, span: &Span, parts: &mut Vec<String>) -> std::io::Result<()> {
        let Some(target) = span.link_target.as_ref() else {
            if !span.text.is_empty() {
                self.push_text_fragment(parts, &span.text);
            }
            for child in &span.children {
                self.collect_formatted_text(child, parts)?;
            }
            return Ok(());
        };

        let hyperlink = if self.style.enable_osc8_hyperlinks {
            Some(self.next_osc8_link(target))
        } else {
            None
        };

        if !span.has_content() {
            let display = if let Some(link) = &hyperlink {
                self.osc8_wrap(link, target)
            } else {
                target.clone()
            };
            self.push_text_fragment(parts, &display);
            return Ok(());
        }

        if Self::is_mailto_with_matching_description(span, target) {
            if let Some(link) = &hyperlink {
                parts.push(self.osc8_start(link));
            }

            if !span.text.is_empty() {
                self.push_text_fragment(parts, &span.text);
            }

            for child in &span.children {
                self.collect_formatted_text(child, parts)?;
            }

            if hyperlink.is_some() {
                parts.push(self.osc8_end());
            }

            return Ok(());
        }

        let footnote_index = if self.style.link_footnotes {
            Some(self.register_numbered_link(target))
        } else {
            None
        };

        if let Some(link) = &hyperlink {
            parts.push(self.osc8_start(link));
        }

        if !span.text.is_empty() {
            self.push_text_fragment(parts, &span.text);
        }

        for child in &span.children {
            self.collect_formatted_text(child, parts)?;
        }

        if hyperlink.is_some() {
            parts.push(self.osc8_end());
        }

        if let Some(index) = footnote_index {
            parts.push(self.inline_link_index(index));
        }
        Ok(())
    }

    fn is_mailto_with_matching_description(span: &Span, target: &str) -> bool {
        let Some(address) = target.strip_prefix("mailto:") else {
            return false;
        };

        let mut description = String::new();
        Self::collect_visible_text(span, &mut description);

        if description.is_empty() {
            return false;
        }

        description.trim() == address.trim()
    }

    fn collect_visible_text(span: &Span, buffer: &mut String) {
        if !span.text.is_empty() {
            buffer.push_str(&span.text);
        }

        for child in &span.children {
            Self::collect_visible_text(child, buffer);
        }
    }

    fn push_text_fragment(&self, parts: &mut Vec<String>, text: &str) {
        if text.is_empty() {
            return;
        }

        if text.contains('\n') {
            for (i, line) in text.split('\n').enumerate() {
                if i > 0 {
                    parts.push("\n".to_string());
                }
                if !line.is_empty() {
                    parts.push(line.to_string());
                }
            }
        } else {
            parts.push(text.to_string());
        }
    }

    fn next_osc8_link(&mut self, target: &str) -> Osc8Link {
        let id = self.next_hyperlink_id.to_string();
        self.next_hyperlink_id += 1;
        Osc8Link::new(Some(id), target.to_string())
    }

    fn register_numbered_link(&mut self, target: &str) -> usize {
        if let Some(&index) = self.link_indices.get(target) {
            return index;
        }

        let index = self.next_link_index;
        self.next_link_index += 1;
        self.pending_links.push(LinkReference {
            index,
            target: target.to_string(),
        });
        self.link_indices.insert(target.to_string(), index);
        index
    }

    fn osc8_start(&self, link: &Osc8Link) -> String {
        let params = link
            .id
            .as_ref()
            .map(|id| format!("id={}", id))
            .unwrap_or_default();
        format!("\x1b]8;{};{}\x1b\\", params, link.target)
    }

    fn osc8_end(&self) -> String {
        "\x1b]8;;\x1b\\".to_string()
    }

    fn osc8_wrap(&self, link: &Osc8Link, text: &str) -> String {
        if self.style.enable_osc8_hyperlinks {
            format!("{}{}{}", self.osc8_start(link), text, self.osc8_end())
        } else {
            text.to_string()
        }
    }

    fn link_label(&self, index: usize, max_width: usize) -> String {
        let mut label = self.format_link_index(index);
        while label.chars().count() < max_width {
            label.insert(0, ' ');
        }
        label.push(' ');
        label
    }

    fn inline_link_index(&self, index: usize) -> String {
        match self.style.link_index_format {
            LinkIndexFormat::SuperscriptArabic => self.superscript_number(index),
            LinkIndexFormat::Bracketed => format!("[{}]", index),
        }
    }

    fn format_link_index(&self, index: usize) -> String {
        match self.style.link_index_format {
            LinkIndexFormat::SuperscriptArabic => self.superscript_number(index),
            LinkIndexFormat::Bracketed => format!("[{}]", index),
        }
    }

    fn superscript_number(&self, index: usize) -> String {
        const SUPERSCRIPTS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
        index
            .to_string()
            .chars()
            .map(|ch| {
                ch.to_digit(10)
                    .and_then(|digit| SUPERSCRIPTS.get(digit as usize).copied())
                    .unwrap_or(ch)
            })
            .collect()
    }

    fn write_wrapped_text(
        &mut self,
        parts: &[String],
        prefix: &str,
        continuation_prefix: &str,
    ) -> std::io::Result<()> {
        let mut full_text = String::new();
        let mut has_forced_breaks = false;

        for part in parts {
            if part == "\n" {
                has_forced_breaks = true;
                full_text.push('\n');
            } else {
                full_text.push_str(part);
            }
        }

        let mut active_styles: Vec<InlineStyle> = Vec::new();
        let mut active_osc_links: Vec<Osc8Link> = Vec::new();

        if has_forced_breaks {
            let lines: Vec<&str> = full_text.split('\n').collect();
            for (i, line) in lines.iter().enumerate() {
                if i == 0 {
                    write!(self.writer, "{}", prefix)?;
                    self.write_wrapped_line(
                        line,
                        prefix.chars().count(),
                        continuation_prefix,
                        &mut active_styles,
                        &mut active_osc_links,
                    )?;
                } else {
                    self.write_line_break(continuation_prefix, &active_styles, &active_osc_links)?;
                    self.write_wrapped_line(
                        line,
                        continuation_prefix.chars().count(),
                        continuation_prefix,
                        &mut active_styles,
                        &mut active_osc_links,
                    )?;
                }
            }
        } else {
            write!(self.writer, "{}", prefix)?;
            self.write_wrapped_line(
                &full_text,
                prefix.chars().count(),
                continuation_prefix,
                &mut active_styles,
                &mut active_osc_links,
            )?;
        }

        Ok(())
    }

    fn measure_wrapped_lines(
        &self,
        parts: &[String],
        initial_prefix_width: usize,
        continuation_prefix_width: usize,
    ) -> Vec<usize> {
        let mut full_text = String::new();
        let mut has_forced_breaks = false;

        for part in parts {
            if part == "\n" {
                has_forced_breaks = true;
                full_text.push('\n');
            } else {
                full_text.push_str(part);
            }
        }

        if full_text.is_empty() {
            return Vec::new();
        }

        let mut widths = Vec::new();

        if has_forced_breaks {
            for (idx, segment) in full_text.split('\n').enumerate() {
                let prefix_width = if idx == 0 {
                    initial_prefix_width
                } else {
                    continuation_prefix_width
                };
                self.measure_wrapped_segment(
                    segment,
                    prefix_width,
                    continuation_prefix_width,
                    &mut widths,
                );
            }
        } else {
            self.measure_wrapped_segment(
                &full_text,
                initial_prefix_width,
                continuation_prefix_width,
                &mut widths,
            );
        }

        widths
    }

    fn measure_wrapped_segment(
        &self,
        text: &str,
        initial_prefix_width: usize,
        continuation_prefix_width: usize,
        widths: &mut Vec<usize>,
    ) {
        if text.is_empty() {
            return;
        }

        let trimmed_text = text.trim_start_matches(' ');
        let leading_spaces = text.len() - trimmed_text.len();

        let mut current_width = 0usize;
        let mut line_width = initial_prefix_width;
        let mut pending_whitespace = 0usize;
        let mut saw_visible_token = false;

        for _ in 0..leading_spaces {
            current_width += 1;
            line_width += 1;
            saw_visible_token = true;
        }

        if trimmed_text.is_empty() {
            if current_width > 0 {
                widths.push(current_width);
            }
            return;
        }

        let tokens = self.tokenize_for_wrap(trimmed_text);

        if tokens.is_empty() {
            if current_width > 0 {
                widths.push(current_width);
            }
            return;
        }

        for (is_whitespace, token) in tokens {
            if is_whitespace {
                pending_whitespace += token.chars().count();
                continue;
            }

            let word_width = self.visible_width(&token);
            let whitespace_width = if current_width == 0 {
                0
            } else {
                pending_whitespace
            };

            if line_width + whitespace_width + word_width > self.style.wrap_width
                && current_width > 0
            {
                widths.push(current_width);
                line_width = continuation_prefix_width;
                current_width = 0;
                pending_whitespace = 0;
            }

            if pending_whitespace > 0 && current_width > 0 {
                line_width += pending_whitespace;
                current_width += pending_whitespace;
            }
            pending_whitespace = 0;

            current_width += word_width;
            line_width += word_width;
            if word_width > 0 {
                saw_visible_token = true;
            }
        }

        if current_width > 0 || saw_visible_token {
            widths.push(current_width);
        }
    }

    fn tokenize_for_wrap(&self, text: &str) -> Vec<(bool, String)> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut current_kind: Option<bool> = None;

        for ch in text.chars() {
            let is_whitespace = ch.is_whitespace();
            match current_kind {
                Some(kind) if kind == is_whitespace => current.push(ch),
                Some(kind) => {
                    tokens.push((kind, std::mem::take(&mut current)));
                    current.push(ch);
                    current_kind = Some(is_whitespace);
                }
                None => {
                    current.push(ch);
                    current_kind = Some(is_whitespace);
                }
            }
        }

        if let Some(kind) = current_kind {
            tokens.push((kind, current));
        }

        tokens
    }

    fn write_wrapped_line(
        &mut self,
        text: &str,
        initial_width: usize,
        continuation_prefix: &str,
        active_styles: &mut Vec<InlineStyle>,
        active_osc_links: &mut Vec<Osc8Link>,
    ) -> std::io::Result<()> {
        if text.is_empty() {
            return Ok(());
        }

        let trimmed_text = text.trim_start_matches(' ');
        let leading_spaces = text.len() - trimmed_text.len();
        let mut current_line = String::new();
        let mut line_width = initial_width;

        if trimmed_text.is_empty() {
            return Ok(());
        }

        for _ in 0..leading_spaces {
            current_line.push(' ');
            line_width += 1;
        }

        let tokens = self.tokenize_for_wrap(trimmed_text);

        if tokens.is_empty() {
            if !current_line.is_empty() {
                write!(self.writer, "{}", current_line)?;
            }
            return Ok(());
        }

        let mut pending_whitespace = String::new();

        for (is_whitespace, token) in tokens {
            if is_whitespace {
                pending_whitespace.push_str(&token);
                continue;
            }

            let word_width = self.visible_width(&token);
            let whitespace_width = if current_line.is_empty() {
                0
            } else {
                pending_whitespace.chars().count()
            };

            if line_width + whitespace_width + word_width > self.style.wrap_width
                && !current_line.is_empty()
            {
                let trimmed_line = current_line.trim_end();
                write!(self.writer, "{}", trimmed_line)?;
                self.write_line_break(continuation_prefix, active_styles, active_osc_links)?;
                line_width = continuation_prefix.chars().count();
                current_line.clear();
                pending_whitespace.clear();
            }

            if !pending_whitespace.is_empty() && !current_line.is_empty() {
                line_width += whitespace_width;
                current_line.push_str(&pending_whitespace);
            }
            pending_whitespace.clear();

            current_line.push_str(&token);
            line_width += word_width;
            self.update_active_styles_from_text(&token, active_styles);
            self.update_active_osc_links_from_text(&token, active_osc_links);
        }

        if !current_line.is_empty() {
            let trimmed_line = current_line.trim_end();
            write!(self.writer, "{}", trimmed_line)?;
        }

        Ok(())
    }

    fn write_line_break(
        &mut self,
        continuation_prefix: &str,
        active_styles: &[InlineStyle],
        active_osc_links: &[Osc8Link],
    ) -> std::io::Result<()> {
        self.write_style_resets(active_styles)?;
        self.write_osc8_resets(active_osc_links)?;
        writeln!(self.writer)?;
        write!(self.writer, "{}", continuation_prefix)?;
        self.reapply_osc8_links(active_osc_links)?;
        self.reapply_active_styles(active_styles)?;
        Ok(())
    }

    fn write_style_resets(&mut self, active_styles: &[InlineStyle]) -> std::io::Result<()> {
        for style in active_styles.iter().rev() {
            if let Some(tags) = self.style.text_styles.get(style) {
                write!(self.writer, "{}", tags.end)?;
            }
        }
        Ok(())
    }

    fn write_osc8_resets(&mut self, active_osc_links: &[Osc8Link]) -> std::io::Result<()> {
        if self.style.enable_osc8_hyperlinks {
            for _ in active_osc_links.iter().rev() {
                write!(self.writer, "{}", self.osc8_end())?;
            }
        }
        Ok(())
    }

    fn reapply_active_styles(&mut self, active_styles: &[InlineStyle]) -> std::io::Result<()> {
        for style in active_styles {
            if let Some(tags) = self.style.text_styles.get(style) {
                write!(self.writer, "{}", tags.begin)?;
            }
        }
        Ok(())
    }

    fn reapply_osc8_links(&mut self, active_osc_links: &[Osc8Link]) -> std::io::Result<()> {
        if self.style.enable_osc8_hyperlinks {
            for link in active_osc_links {
                write!(self.writer, "{}", self.osc8_start(link))?;
            }
        }
        Ok(())
    }

    fn update_active_styles_from_text(&self, text: &str, active_styles: &mut Vec<InlineStyle>) {
        for capture in ANSI_ESCAPE_REGEX.find_iter(text) {
            let sequence = capture.as_str();
            if let Some(style) = self.find_style_start(sequence) {
                active_styles.push(style);
            } else if let Some(style) = self.find_style_end(sequence) {
                if let Some(idx) = active_styles.iter().rposition(|s| *s == style) {
                    active_styles.remove(idx);
                }
            }
        }
    }

    fn update_active_osc_links_from_text(&self, text: &str, active_osc_links: &mut Vec<Osc8Link>) {
        if !self.style.enable_osc8_hyperlinks {
            return;
        }

        for capture in OSC8_SEQUENCE_REGEX.captures_iter(text) {
            let params = capture.get(1).map(|m| m.as_str()).unwrap_or("");
            let target = capture.get(2).map(|m| m.as_str()).unwrap_or("");
            if target.is_empty() {
                let _ = active_osc_links.pop();
            } else {
                let id = params
                    .split(':')
                    .find_map(|param| param.strip_prefix("id="))
                    .map(|value| value.to_string());
                active_osc_links.push(Osc8Link::new(id, target.to_string()));
            }
        }
    }

    fn find_style_start(&self, sequence: &str) -> Option<InlineStyle> {
        self.style.text_styles.iter().find_map(|(style, tags)| {
            if tags.begin == sequence {
                Some(*style)
            } else {
                None
            }
        })
    }

    fn find_style_end(&self, sequence: &str) -> Option<InlineStyle> {
        self.style.text_styles.iter().find_map(|(style, tags)| {
            if tags.end == sequence {
                Some(*style)
            } else {
                None
            }
        })
    }

    fn visible_width(&self, text: &str) -> usize {
        // Remove ANSI escape sequences for width calculation
        let without_ansi = ANSI_ESCAPE_REGEX.replace_all(text, "");
        let visible_text = OSC8_ESCAPE_REGEX.replace_all(&without_ansi, "");
        visible_text.chars().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;
    use crate::test_helpers::*;
    use std::io::Cursor;
    use std::time::{Duration, Instant};

    #[test]
    fn test_ascii_formatting() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        let doc = doc(vec![p_(vec![span("Hello "), b__("world"), span("!")])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        println!("ASCII format result: '{}'", result);

        // ASCII formatter should not add any styling
        assert!(result.contains("Hello world!"));
        assert!(!result.contains("\x1b["));
    }

    #[test]
    fn test_ansi_formatting() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ansi(&mut output);

        let doc = doc(vec![p_(vec![span("Hello "), b__("world"), span("!")])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        // ANSI formatter should add bold styling
        assert!(result.contains("\x1b[1m")); // Bold begin
        assert!(result.contains("\x1b[22m")); // Bold end
        assert!(result.contains("\x1b[0m")); // Reset at end
    }

    #[test]
    fn test_ascii_and_ansi_preserve_consecutive_spaces() {
        let doc = doc(vec![p_(vec![span("A   B")])]);

        let mut ascii_output = Vec::new();
        Formatter::new_ascii(&mut ascii_output)
            .write_document(&doc)
            .unwrap();
        let ascii_result = String::from_utf8(ascii_output).unwrap();
        assert_eq!(ascii_result, "A   B\n");

        let mut ansi_output = Vec::new();
        Formatter::new_ansi(&mut ansi_output)
            .write_document(&doc)
            .unwrap();
        let ansi_result = String::from_utf8(ansi_output).unwrap();
        assert_eq!(ansi_result, "A   B\n\x1b[0m");
    }

    #[test]
    fn test_ansi_wrapped_style_does_not_color_prefix() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ansi(&mut output);
        formatter.style.wrap_width = 20;

        let doc = doc(vec![ul_(vec![li_(vec![p_(vec![mark__(
            "Highlighted content that wraps to another line.",
        )])])])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(
            result.contains("\x1b[27m\n   \x1b[7m"),
            "Expected highlight styling to reset before the newline and resume after the indent"
        );
    }

    #[test]
    fn test_ansi_forced_newline_reapplies_after_prefix() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ansi(&mut output);

        let doc = doc(vec![quote_(vec![p_(vec![mark__(
            "Styled first line\nstyled second line continues.",
        )])])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(
            result.contains("\x1b[27m\n| \x1b[7m"),
            "Expected quote prefix to remain unstyled around forced line breaks"
        );
    }

    #[test]
    fn test_ascii_links_with_footnotes() {
        let doc = doc(vec![
            p_(vec![
                span("Visit "),
                link_text__("https://example.com/docs", "Docs"),
                span(" and "),
                link__("https://example.com/plain"),
                span("."),
            ]),
            h2_("Next section"),
        ]);

        let mut output = Vec::new();
        Formatter::new_ascii(&mut output)
            .write_document(&doc)
            .unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("Docs¹"));
        assert!(result.contains("¹ https://example.com/docs"));
        assert!(result.contains("https://example.com/plain"));
        assert!(!result.contains("https://example.com/plain¹"));

        let footnote_pos = result.find("¹ https://example.com/docs").unwrap();
        let heading_pos = result.find("Next section").unwrap();
        assert!(footnote_pos < heading_pos);
        let footnote_entry = "¹ https://example.com/docs";
        let footer_start = result.find(footnote_entry).unwrap();
        let after_entry = footer_start + footnote_entry.len();
        assert!(
            result[after_entry..].starts_with('\n'),
            "expected newline after footnote entry"
        );
    }

    #[test]
    fn test_ansi_links_with_footnotes() {
        let doc = doc(vec![
            p_(vec![
                span("Visit "),
                link_text__("https://example.com/docs", "Docs"),
                span(" and "),
                link__("https://example.com/plain"),
                span("."),
            ]),
            h2_("Next section"),
        ]);

        let mut output = Vec::new();
        Formatter::new_ansi(&mut output)
            .write_document(&doc)
            .unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("\x1b]8;id=1;https://example.com/docs\x1b\\Docs"));
        let docs_pos = result.find("Docs").unwrap();
        let index_marker = "\x1b]8;;\x1b\\¹";
        let index_pos = result
            .find(index_marker)
            .expect("superscript index marker missing");
        assert!(docs_pos < index_pos);
        assert!(result.contains(
            "\x1b]8;id=2;https://example.com/plain\x1b\\https://example.com/plain\x1b]8;;\x1b\\"
        ));
        assert!(result.contains(
            "¹ \x1b]8;id=3;https://example.com/docs\x1b\\https://example.com/docs\x1b]8;;\x1b\\"
        ));
        assert!(result.ends_with("\x1b[0m"));
    }

    #[test]
    fn test_ansi_links_without_footnotes_when_disabled() {
        let doc = doc(vec![
            p_(vec![
                span("Visit "),
                link_text__("https://example.com/docs", "Docs"),
                span(" and "),
                link__("https://example.com/plain"),
                span("."),
            ]),
            h2_("Next section"),
        ]);

        let mut output = Vec::new();
        let mut style = FormattingStyle::ansi();
        style.link_footnotes = false;
        Formatter::new(&mut output, style)
            .write_document(&doc)
            .unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("\x1b]8;id=1;https://example.com/docs\x1b\\Docs"));
        assert!(
            result.contains("\x1b]8;id=2;https://example.com/plain\x1b\\https://example.com/plain")
        );
        assert!(result.contains("Next section"));
        assert!(!result.contains("\x1b]8;;\x1b\\¹"));
        assert!(!result.contains("\n¹ "));
        assert!(result.ends_with("\x1b[0m"));
    }

    #[test]
    fn test_duplicate_links_share_indices() {
        let doc = doc(vec![p_(vec![
            span("See "),
            link_text__("https://example.com/docs", "Docs"),
            span(" and later revisit "),
            link_text__("https://example.com/docs", "Docs again"),
            span(" for details."),
        ])]);

        let mut ascii_output = Vec::new();
        Formatter::new_ascii(&mut ascii_output)
            .write_document(&doc)
            .unwrap();
        let ascii_result = String::from_utf8(ascii_output).unwrap();
        assert!(ascii_result.contains("Docs¹"));
        assert!(ascii_result.contains("Docs again¹"));
        assert_eq!(ascii_result.matches("\n¹ ").count(), 1);
        assert!(!ascii_result.contains('²'));

        let mut ansi_output = Vec::new();
        Formatter::new_ansi(&mut ansi_output)
            .write_document(&doc)
            .unwrap();
        let ansi_result = String::from_utf8(ansi_output).unwrap();
        assert!(ansi_result.contains("Docs\x1b]8;;\x1b\\¹"));
        assert!(ansi_result.contains("Docs again\x1b]8;;\x1b\\¹"));
        assert_eq!(ansi_result.matches("\n¹ ").count(), 1);
        assert!(!ansi_result.contains('²'));
    }

    #[test]
    fn test_ascii_links_with_bracketed_indices() {
        let doc = doc(vec![
            p_(vec![
                span("Visit "),
                link_text__("https://example.com/docs", "Docs"),
                span(" and "),
                link__("https://example.com/plain"),
                span("."),
            ]),
            h2_("Next section"),
        ]);

        let mut output = Vec::new();
        let mut style = FormattingStyle::ascii();
        style.link_index_format = LinkIndexFormat::Bracketed;
        Formatter::new(&mut output, style)
            .write_document(&doc)
            .unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("Docs[1]"));
        assert!(result.contains("[1] https://example.com/docs"));
        assert!(!result.contains("https://example.com/plain["));
    }

    #[test]
    fn test_ascii_mailto_links_skip_footnotes() {
        let doc = doc(vec![p_(vec![
            span("Contact "),
            link_text__("mailto:support@example.com", "support@example.com"),
            span(" or visit "),
            link_text__("https://example.com/docs", "Docs"),
            span("."),
        ])]);

        let mut output = Vec::new();
        Formatter::new_ascii(&mut output)
            .write_document(&doc)
            .unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("support@example.com"));
        assert!(!result.contains("support@example.com¹"));
        assert!(!result.contains("mailto:"));
        assert!(result.contains("Docs¹"));
        assert!(result.contains("¹ https://example.com/docs"));
    }

    #[test]
    fn test_ansi_mailto_links_skip_indices() {
        let doc = doc(vec![p_(vec![
            span("Contact "),
            link_text__("mailto:support@example.com", "support@example.com"),
            span(" or visit "),
            link_text__("https://example.com/docs", "Docs"),
            span("."),
        ])]);

        let mut output = Vec::new();
        Formatter::new_ansi(&mut output)
            .write_document(&doc)
            .unwrap();
        let result = String::from_utf8(output).unwrap();

        let mailto_sequence =
            "\x1b]8;id=1;mailto:support@example.com\x1b\\support@example.com\x1b]8;;\x1b\\";
        assert!(
            result.contains(mailto_sequence),
            "expected OSC 8 wrapped mailto link"
        );
        assert!(!result.contains("support@example.com\x1b]8;;\x1b\\¹"));
        assert!(result.contains("\x1b]8;id=2;https://example.com/docs\x1b\\Docs"));
        assert!(result.contains(
            "¹ \x1b]8;id=3;https://example.com/docs\x1b\\https://example.com/docs\x1b]8;;\x1b\\"
        ));
        assert!(result.ends_with("\x1b[0m"));
    }

    #[test]
    fn test_ansi_links_with_bracketed_indices() {
        let doc = doc(vec![
            p_(vec![
                span("Visit "),
                link_text__("https://example.com/docs", "Docs"),
                span(" and "),
                link__("https://example.com/plain"),
                span("."),
            ]),
            h2_("Next section"),
        ]);

        let mut output = Vec::new();
        let mut style = FormattingStyle::ansi();
        style.link_index_format = LinkIndexFormat::Bracketed;
        Formatter::new(&mut output, style)
            .write_document(&doc)
            .unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("\x1b]8;id=1;https://example.com/docs\x1b\\Docs"));
        assert!(result.contains("\x1b]8;;\x1b\\[1]"));
        assert!(result.contains(
            "[1] \x1b]8;id=3;https://example.com/docs\x1b\\https://example.com/docs\x1b]8;;\x1b\\"
        ));
    }

    #[test]
    fn test_ansi_wrapped_links_emit_osc8_sequences() {
        let doc = doc(vec![p_(vec![
            span("See "),
            link_text__(
                "https://example.com",
                "this link text will wrap across multiple lines for testing",
            ),
            span(" please."),
        ])]);

        let mut style = FormattingStyle::ansi();
        style.wrap_width = 30;

        let mut output = Vec::new();
        Formatter::new(&mut output, style)
            .write_document(&doc)
            .unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(
            result.contains("\x1b]8;id=1;https://example.com\x1b\\this"),
            "expected OSC 8 hyperlink start before link text"
        );
        assert!(
            result.contains("\x1b]8;;\x1b\\\n\x1b]8;id=1;https://example.com\x1b\\"),
            "expected OSC 8 hyperlink to close before wrap newline and reopen afterwards:\n{}",
            result
        );
    }

    #[test]
    fn test_superscript_link_list_alignment() {
        let mut spans = Vec::new();
        for i in 1..=10 {
            if i > 1 {
                spans.push(span(", "));
            }
            let target = format!("https://example.com/{}", i);
            let text = format!("Doc{}", i);
            spans.push(link_text__(&target, &text));
        }

        let doc = doc(vec![p_(spans)]);

        let mut output = Vec::new();
        Formatter::new_ascii(&mut output)
            .write_document(&doc)
            .unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("\n ¹ https://example.com/1"));
        assert!(!result.contains("\n¹ https://example.com/1"));
        assert!(result.contains("\n¹⁰ https://example.com/10"));
    }

    #[test]
    fn test_bracketed_link_list_alignment() {
        let mut spans = Vec::new();
        for i in 1..=10 {
            if i > 1 {
                spans.push(span(", "));
            }
            let target = format!("https://example.com/{}", i);
            let text = format!("Doc{}", i);
            spans.push(link_text__(&target, &text));
        }
        let doc = doc(vec![p_(spans)]);

        let mut output = Vec::new();
        let mut style = FormattingStyle::ascii();
        style.link_index_format = LinkIndexFormat::Bracketed;
        Formatter::new(&mut output, style)
            .write_document(&doc)
            .unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("\n [1] https://example.com/1"));
        assert!(!result.contains("\n[1] https://example.com/1"));
        assert!(result.contains("\n[10] https://example.com/10"));
    }

    #[test]
    fn test_quote_formatting() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        let doc = doc(vec![quote_(vec![p__("Quoted text")])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("| Quoted text"));
    }

    #[test]
    fn test_list_formatting() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        let doc = doc(vec![ul_(vec![
            li_(vec![p__("Item 1")]),
            li_(vec![p__("Item 2")]),
        ])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains(" • Item 1"));
        assert!(result.contains(" • Item 2"));
        assert!(result.contains(" • Item 1\n\n • Item 2"));
    }

    #[test]
    fn test_ordered_list_formatting() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        let doc = doc(vec![ol_(vec![
            li_(vec![p__("First")]),
            li_(vec![p__("Second")]),
        ])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains(" 1. First"));
        assert!(result.contains(" 2. Second"));
        assert!(result.contains(" 1. First\n\n 2. Second"));
    }

    #[test]
    fn test_list_item_multiple_paragraphs() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        let doc = doc(vec![ul_(vec![li_(vec![
            p__("Primary text."),
            p__("Follow-up paragraph."),
        ])])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains(" • Primary text."));
        assert!(result.contains("   \n   Follow-up paragraph."));
        assert!(!result.contains(" • Follow-up paragraph."));
    }

    #[test]
    fn test_list_item_forced_newline_spacing() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        let doc = doc(vec![ul_(vec![li_(vec![p_(vec![span(
            "First line\nSecond line",
        )])])])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains(" • First line\n   Second line"));
    }

    #[test]
    fn test_nested_list_item_additional_paragraphs_spacing() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        // Ensure a list item that only contains another list does not reintroduce bullets for nested paragraphs
        let doc = doc(vec![ul_(vec![li_(vec![ul_(vec![li_(vec![
            p__("Inner item primary text."),
            p__("Inner item follow-up paragraph."),
        ])])])])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("    • Inner item primary text."));
        assert!(result.contains("      Inner item follow-up paragraph."));
        assert!(!result.contains(" •  • "));
        assert!(!result.contains("• Inner item follow-up paragraph."));
    }

    #[test]
    fn test_list_item_only_nested_ordered_list_shows_bullet() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        let doc = doc(vec![ul_(vec![li_(vec![ol_(vec![
            li_(vec![p__("First")]),
            li_(vec![p__("Second")]),
        ])])])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        let lines: Vec<&str> = result.lines().collect();
        let bullet_index = lines
            .iter()
            .position(|line| *line == " •  1. First")
            .expect("Bullet line with first ordered entry missing");

        let first_entry = lines
            .get(bullet_index + 1)
            .expect("Expected spacer after first ordered list entry");
        assert!(first_entry.trim().is_empty());

        let second_entry = lines
            .iter()
            .skip(bullet_index + 2)
            .find(|line| line.trim_start().starts_with("2. Second"))
            .expect("Second ordered list entry missing");
        assert!(second_entry.starts_with("    2. Second"));
    }

    #[test]
    fn test_quote_multiple_paragraphs_spacing() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        let doc = doc(vec![quote_(vec![
            p__("Paragraph one."),
            p__("Paragraph two."),
        ])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("| Paragraph one."));
        assert!(result.contains("| \n| Paragraph two."));
    }

    #[test]
    fn test_quote_list_spacing() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        let doc = doc(vec![quote_(vec![ul_(vec![
            li_(vec![p__("Item 1")]),
            li_(vec![p__("Item 2")]),
        ])])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("|  • Item 1"));
        assert!(result.contains("| \n|  • Item 2"));
    }

    #[test]
    fn test_quote_list_with_nested_quote_spacing() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        let doc = doc(vec![quote_(vec![
            p__("Please see, how the following list is part of a quote and contains nested paragraphs."),
            ul_(vec![
                li_(vec![p__("This is a paragraph inside of a quoted paragraph")]),
                li_(vec![
                    p__("This bullet points contains another quote:"),
                    quote_(vec![p_(vec![span(
                        "You can never have enough nesting of paragraphs.\n   —Robert Lillack",
                    )])]),
                ]),
            ]),
        ])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        let bullet_count = result.matches(" • ").count();
        assert_eq!(bullet_count, 2);
        assert!(result.contains("|  • This bullet points contains another quote:"));
        assert!(result.contains("|    | You can never have enough nesting of paragraphs."));
        assert!(result.contains("|    |    —Robert Lillack"));
        assert!(!result.contains("|  • |"));
    }

    #[test]
    fn test_quote_list_with_nested_quote_blank_line_indent() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        let doc = doc(vec![quote_(vec![ul_(vec![li_(vec![
            p__("Para 1"),
            quote_(vec![p__("Para 2")]),
        ])])])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        println!("{}", result);
        let lines: Vec<&str> = result.lines().collect();
        let para1_index = lines
            .iter()
            .position(|line| *line == "|  • Para 1")
            .expect("Para 1 line missing");

        assert_eq!(lines.get(para1_index + 1), Some(&"|    "));
        assert_eq!(lines.get(para1_index + 2), Some(&"|    | Para 2"));
    }

    #[test]
    fn test_quote_list_with_nested_quote_blank_line_indent_ansi() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ansi(&mut output);

        let doc = doc(vec![quote_(vec![ul_(vec![li_(vec![
            p__("Para 1"),
            quote_(vec![p__("Para 2")]),
        ])])])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        let lines: Vec<&str> = result.lines().collect();
        let para1_index = lines
            .iter()
            .position(|line| *line == "|  • Para 1")
            .expect("Para 1 line missing");

        assert_eq!(lines.get(para1_index + 1), Some(&"|    "));
        assert_eq!(lines.get(para1_index + 2), Some(&"|    | Para 2"));
        assert_eq!(lines.last(), Some(&"\u{1b}[0m"));
    }

    #[test]
    fn test_quote_list_with_nested_quote_intro_inside_quote() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        let doc = doc(vec![quote_(vec![ul_(vec![li_(vec![quote_(vec![
            p__("Para 1"),
            p__("Para 2"),
        ])])])])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        let lines: Vec<&str> = result.lines().collect();
        let para1_index = lines
            .iter()
            .position(|line| *line == "|  • Para 1")
            .expect("Para 1 line missing");

        assert_eq!(lines.get(para1_index + 1), Some(&"|    "));
        assert_eq!(lines.get(para1_index + 2), Some(&"|    | Para 2"));
        assert!(!result.contains("|  • | Para 1"));
    }

    #[test]
    fn test_wrap_width() {
        let mut output = Vec::new();
        let mut style = FormattingStyle::ascii();
        style.wrap_width = 20; // Very short for testing
        let mut formatter = Formatter::new(&mut output, style);

        let doc = doc(vec![p__(
            "This is a very long line that should definitely be wrapped",
        )]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        // Should contain line breaks due to wrapping
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() > 1);
    }

    #[test]
    fn test_wrap_width_with_left_padding() {
        let mut output = Vec::new();
        let mut style = FormattingStyle::ascii();
        style.wrap_width = 10;
        style.left_padding = 4;
        let mut formatter = Formatter::new(&mut output, style);

        let doc = doc(vec![p__("123456 7890 1234")]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        let lines: Vec<&str> = result.lines().filter(|line| !line.is_empty()).collect();

        assert!(lines.len() >= 2);
        assert!(lines.iter().all(|line| line.starts_with("    ")));
        assert!(lines[0].chars().count() <= 10);
        assert!(lines[1].contains("7890"));
        assert!(lines.iter().any(|line| line.contains("1234")));
    }

    #[test]
    fn test_header2_wraps_and_underlines_to_longest_line() {
        let doc = doc(vec![h2_(
            "A level two header that definitely needs to wrap across multiple segments",
        )]);

        let mut output = Vec::new();
        let mut style = FormattingStyle::ascii();
        style.wrap_width = 30;

        Formatter::new(&mut output, style)
            .write_document(&doc)
            .unwrap();
        let result = String::from_utf8(output).unwrap();

        let non_empty: Vec<&str> = result
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();
        assert!(
            non_empty.len() >= 2,
            "expected at least header text and underline lines, got {:?}",
            non_empty
        );

        let underline_index = non_empty
            .iter()
            .position(|line| line.chars().all(|ch| ch == '=' || ch.is_whitespace()))
            .expect("expected underline made of '=' characters");
        let header_lines = &non_empty[..underline_index];
        assert!(
            !header_lines.is_empty(),
            "expected header text lines before underline"
        );

        let max_line_width = header_lines
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);
        assert!(
            max_line_width <= 30,
            "wrapped header line exceeds configured width ({max_line_width})"
        );

        let underline_width = non_empty[underline_index]
            .chars()
            .filter(|ch| *ch == '=')
            .count();
        assert_eq!(underline_width, max_line_width);
    }

    #[test]
    fn test_header1_wraps_when_text_exceeds_width() {
        let doc = doc(vec![h1_(
            "This level-one heading is long enough that it should wrap in the formatter output",
        )]);

        let mut output = Vec::new();
        let mut style = FormattingStyle::ascii();
        style.wrap_width = 24;

        Formatter::new(&mut output, style)
            .write_document(&doc)
            .unwrap();
        let result = String::from_utf8(output).unwrap();

        let lines: Vec<&str> = result
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();
        assert!(
            lines.len() >= 2,
            "expected heading text to wrap across multiple lines, got {:?}",
            lines
        );

        for line in &lines {
            assert!(
                line.chars().count() <= 24,
                "header line '{line}' exceeds configured wrap width"
            );
        }
    }

    #[test]
    fn test_header_formatting_ascii() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        let doc = doc(vec![h1_("Heading"), p__("Following text")]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 8);
        assert!(lines[0].is_empty());
        assert!(lines[1].is_empty());
        assert!(lines[2].is_empty());

        let header_line = lines[3];
        assert_eq!(header_line.trim(), "Heading");

        let leading_spaces = header_line.len() - header_line.trim_start().len();
        let expected_padding = (super::DEFAULT_WRAP_WIDTH - header_line.trim().len()) / 2;
        assert_eq!(leading_spaces, expected_padding);

        assert!(lines[4].is_empty());
        assert!(lines[5].is_empty());
        assert!(lines[6].is_empty());
        assert_eq!(lines[7], "Following text");
    }

    #[test]
    fn test_header_formatting_ansi() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ansi(&mut output);

        let doc = doc(vec![h1_("Heading"), p__("Following text")]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("\x1b[1m"));
        assert!(result.contains("\x1b[22m"));

        let lines: Vec<&str> = result.split('\n').collect();
        assert!(lines.len() >= 9);
        assert!(lines[0].is_empty());
        assert!(lines[1].is_empty());
        assert!(lines[2].is_empty());

        let header_line = lines[3];
        let ansi_regex = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
        let header_plain = ansi_regex.replace_all(header_line, "").to_string();
        assert_eq!(header_plain.trim(), "Heading");

        let leading_spaces = header_plain.len() - header_plain.trim_start().len();
        let expected_padding = (super::DEFAULT_WRAP_WIDTH - header_plain.trim().len()) / 2;
        assert_eq!(leading_spaces, expected_padding);

        assert!(lines[4].is_empty());
        assert!(lines[5].is_empty());
        assert!(lines[6].is_empty());
        assert!(lines[7].starts_with("Following text"));
        assert_eq!(lines[8], "\x1b[0m");
    }

    #[test]
    fn test_header2_formatting_ascii() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        let doc = doc(vec![h2_("Heading 2"), p__("Following text")]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 8);
        assert!(lines[0].is_empty());
        assert!(lines[1].is_empty());
        assert!(lines[2].is_empty());

        assert_eq!(lines[3], "Heading 2");
        assert!(lines[4].chars().all(|c| c == '='));
        assert_eq!(lines[4].chars().count(), lines[3].trim().chars().count());

        assert!(lines[5].is_empty());
        assert!(lines[6].is_empty());
        assert_eq!(lines[7], "Following text");
    }

    #[test]
    fn test_header2_formatting_ansi() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ansi(&mut output);

        let doc = doc(vec![h2_("Heading 2"), p__("Following text")]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("\x1b[1m"));

        let lines: Vec<&str> = result.split('\n').collect();
        let ansi_regex = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();

        assert!(lines.len() >= 9);
        assert!(lines[0].is_empty());
        assert!(lines[1].is_empty());
        assert!(lines[2].is_empty());

        let header_plain = ansi_regex.replace_all(lines[3], "").to_string();
        assert_eq!(header_plain.trim(), "Heading 2");

        assert!(lines[4].chars().all(|c| c == '='));
        assert_eq!(
            lines[4].chars().count(),
            header_plain.trim().chars().count()
        );

        assert!(lines[5].is_empty());
        assert!(lines[6].is_empty());
        assert!(lines[7].starts_with("Following text"));
        assert_eq!(lines[8], "\x1b[0m");
    }

    #[test]
    fn test_header3_formatting_ascii() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        let doc = doc(vec![h3_("Heading 3"), p__("Following text")]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 6);
        assert!(lines[0].is_empty());
        assert!(lines[1].is_empty());

        assert_eq!(lines[2], "Heading 3");
        assert!(lines[3].chars().all(|c| c == '-'));
        assert_eq!(lines[3].chars().count(), lines[2].trim().chars().count());

        assert!(lines[4].is_empty());
        assert_eq!(lines[5], "Following text");
    }

    #[test]
    fn test_header3_formatting_ansi() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ansi(&mut output);

        let doc = doc(vec![h3_("Heading 3"), p__("Following text")]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        assert!(result.contains("\x1b[1m"));

        let lines: Vec<&str> = result.split('\n').collect();
        let ansi_regex = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();

        assert!(lines.len() >= 7);
        assert!(lines[0].is_empty());
        assert!(lines[1].is_empty());

        let header_plain = ansi_regex.replace_all(lines[2], "").to_string();
        assert_eq!(header_plain.trim(), "Heading 3");

        assert!(lines[3].chars().all(|c| c == '-'));
        assert_eq!(
            lines[3].chars().count(),
            header_plain.trim().chars().count()
        );

        assert!(lines[4].is_empty());
        assert!(lines[5].starts_with("Following text"));
        assert_eq!(lines[6], "\x1b[0m");
    }

    #[test]
    fn test_heading_spacing_collapse() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        let doc = doc(vec![h2_("Heading 2"), h3_("Heading 3")]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();

        let lines: Vec<&str> = result.lines().collect();
        let h2_idx = lines.iter().position(|line| line == &"Heading 2").unwrap();
        let h2_underline_idx = h2_idx + 1;
        let h3_idx = lines.iter().position(|line| line == &"Heading 3").unwrap();

        assert!(lines[h2_underline_idx].chars().all(|c| c == '='));
        let blank_count = h3_idx.saturating_sub(h2_underline_idx + 1);
        assert_eq!(blank_count, 2);
        assert!(lines[h3_idx + 1].chars().all(|c| c == '-'));
    }

    #[test]
    fn renders_large_document_quickly() {
        let data = include_str!("../../tests/snapshots/markdown/import/progit1-de.snap.ftml");
        let doc = parse(Cursor::new(data.as_bytes())).expect("failed to parse FTML fixture");

        let mut output = Vec::new();
        let start = Instant::now();
        Formatter::new_ascii(&mut output)
            .write_document(&doc)
            .expect("render should succeed");
        let elapsed = start.elapsed();

        assert!(
            elapsed < Duration::from_secs(5),
            "Rendering took {elapsed:?}"
        );
        assert!(!output.is_empty());
    }
}
