//! Serialize [`Document`](crate::Document) trees back into FTML.
//!
//! For HTML output that preserves table structure, see [`crate::html::write`].

use crate::{
    ChecklistItem, Document, InlineStyle, Paragraph, ParagraphType, Span, TableCell, TableRow,
};
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Write};

/// Emits FTML markup from a [`Document`] tree.
///
/// `Writer` focuses on producing readable markup that preserves semantic tags
/// such as lists, block quotes, and inline styles. It defaults to two-space
/// indentation and an 80 character wrap width.
///
/// FTML has no table syntax, so tables are flattened into individual `<p>`
/// paragraphs. Use [`Writer::new_html`] (or [`crate::html::write`]) to retain
/// the original `<table>` markup.
///
/// # Examples
///
/// ```
/// use tdoc::{Document, Paragraph, Span, ftml::Writer};
///
/// let paragraph = Paragraph::new_text().with_content(vec![Span::new_text("Hello!")]);
/// let document = Document::new().with_paragraphs(vec![paragraph]);
///
/// let writer = Writer::new();
/// let html = writer.write_to_string(&document).unwrap();
/// assert_eq!(html, "<p>Hello!</p>\n");
/// ```
pub struct Writer {
    indentation: String,
    max_width: usize,
    style_tags: HashMap<InlineStyle, String>,
    /// When `true`, tables are emitted as `<table>/<tr>/<td>` markup. When
    /// `false` (FTML default), tables are flattened into individual `<p>`
    /// paragraphs because FTML has no table syntax.
    emit_tables: bool,
    multiple_spaces_regex: Regex,
    trailing_spaces_regex: Regex,
    leading_spaces_regex: Regex,
    spaces_at_start_regex: Regex,
    spaces_at_end_regex: Regex,
    any_space_regex: Regex,
}

impl Default for Writer {
    fn default() -> Self {
        Self::new()
    }
}

impl Writer {
    /// Creates a new FTML writer. Tables are flattened into paragraphs
    /// because FTML has no table syntax.
    pub fn new() -> Self {
        Self::with_tables(false)
    }

    /// Creates a writer that emits real `<table>` markup. Use this for HTML
    /// output where the structure should be preserved.
    pub fn new_html() -> Self {
        Self::with_tables(true)
    }

    fn with_tables(emit_tables: bool) -> Self {
        let mut style_tags = HashMap::new();
        style_tags.insert(InlineStyle::Bold, "b".to_string());
        style_tags.insert(InlineStyle::Italic, "i".to_string());
        style_tags.insert(InlineStyle::Underline, "u".to_string());
        style_tags.insert(InlineStyle::Strike, "s".to_string());
        style_tags.insert(InlineStyle::Highlight, "mark".to_string());
        style_tags.insert(InlineStyle::Code, "code".to_string());

        Self {
            indentation: "  ".to_string(),
            max_width: 80,
            style_tags,
            emit_tables,
            multiple_spaces_regex: Regex::new(r"  +").unwrap(),
            trailing_spaces_regex: Regex::new(r"\s +").unwrap(),
            leading_spaces_regex: Regex::new(r" +\s").unwrap(),
            spaces_at_start_regex: Regex::new(r"^ +").unwrap(),
            spaces_at_end_regex: Regex::new(r" +$").unwrap(),
            any_space_regex: Regex::new(r"\s").unwrap(),
        }
    }

    /// Renders the document into a `String` buffer.
    pub fn write_to_string(&self, document: &Document) -> io::Result<String> {
        let mut buffer = Vec::new();
        self.write(&mut buffer, document)?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

    /// Writes the document to any [`Write`] implementor.
    pub fn write<W: Write>(&self, writer: &mut W, document: &Document) -> io::Result<()> {
        let mut first = true;
        for paragraph in &document.paragraphs {
            if self.should_skip(paragraph) {
                continue;
            }
            if first {
                first = false;
            } else {
                writeln!(writer)?;
            }
            self.write_paragraph(writer, paragraph, 0)?;
        }
        Ok(())
    }

    /// Whether a paragraph is dropped entirely by this writer. Strict FTML has
    /// no thematic-break element, so [`Paragraph::HorizontalRule`] nodes (which
    /// can only arrive via conversion from Markdown or HTML) are skipped; the
    /// HTML writer keeps them as `<hr />`. Skipping them before the separator
    /// logic runs avoids emitting a stray blank line in their place.
    fn should_skip(&self, paragraph: &Paragraph) -> bool {
        !self.emit_tables && paragraph.paragraph_type() == ParagraphType::HorizontalRule
    }

    fn write_paragraph<W: Write>(
        &self,
        writer: &mut W,
        paragraph: &Paragraph,
        level: usize,
    ) -> io::Result<()> {
        let paragraph_type = paragraph.paragraph_type();
        let tag = paragraph_type.html_tag();

        if paragraph_type == ParagraphType::Table {
            return self.write_table_paragraph(writer, paragraph.rows(), level);
        }

        if paragraph_type == ParagraphType::HorizontalRule {
            // Only the HTML writer represents a thematic break; strict FTML has
            // no such element, so the rule is dropped there. (Top-level and
            // block-quote iteration already filter these out via `should_skip`;
            // this also covers any remaining position, such as a list item.)
            if self.emit_tables {
                self.write_indent(writer, level)?;
                return writeln!(writer, "<hr />");
            }
            return Ok(());
        }

        if paragraph_type.is_leaf() {
            if paragraph_type == ParagraphType::CodeBlock {
                self.write_code_block_paragraph(writer, paragraph.content(), level)
            } else {
                self.write_leaf_paragraph(writer, paragraph.content(), tag, level)
            }
        } else {
            self.write_indent(writer, level)?;
            writeln!(writer, "<{}>", tag)?;

            if paragraph_type == ParagraphType::Checklist {
                for item in paragraph.checklist_items() {
                    self.write_checklist_item(writer, item, level + 1)?;
                }
            } else if paragraph_type == ParagraphType::UnorderedList
                || paragraph_type == ParagraphType::OrderedList
            {
                let mut first = true;
                for entry in paragraph.entries() {
                    if first {
                        first = false;
                    } else {
                        writeln!(writer)?;
                    }
                    self.write_indent(writer, level + 1)?;
                    writeln!(writer, "<li>")?;

                    for child in entry {
                        self.write_paragraph(writer, child, level + 2)?;
                    }

                    self.write_indent(writer, level + 1)?;
                    writeln!(writer, "</li>")?;
                }
            } else {
                let mut first = true;
                for child in paragraph.children() {
                    if self.should_skip(child) {
                        continue;
                    }
                    if first {
                        first = false;
                    } else {
                        writeln!(writer)?;
                    }
                    self.write_paragraph(writer, child, level + 1)?;
                }
            }

            self.write_indent(writer, level)?;
            writeln!(writer, "</{}>", tag)
        }
    }

    fn write_code_block_paragraph<W: Write>(
        &self,
        writer: &mut W,
        content: &[Span],
        level: usize,
    ) -> io::Result<()> {
        let mut code_text = self.collect_code_text(content);
        if !code_text.is_empty() {
            code_text = code_text.replace("\r\n", "\n").replace('\r', "\n");
        }

        let needs_newline_after_tag = code_text.is_empty() || !code_text.starts_with('\n');

        self.write_indent(writer, level)?;
        write!(writer, "<pre>")?;
        if needs_newline_after_tag {
            writeln!(writer)?;
        }

        if !code_text.is_empty() {
            let encoded = self.encode_pre_text(&code_text);
            writer.write_all(encoded.as_bytes())?;
        }

        if !code_text.ends_with('\n') {
            writeln!(writer)?;
        }

        self.write_indent(writer, level)?;
        writeln!(writer, "</pre>")
    }

    fn write_leaf_paragraph<W: Write>(
        &self,
        writer: &mut W,
        content: &[Span],
        tag: &str,
        level: usize,
    ) -> io::Result<()> {
        // Try single-line output first
        let single_line = self.render_single_line(content, tag, level);

        if single_line.chars().count() <= self.max_width && !single_line.trim_end().contains('\n') {
            write!(writer, "{}", single_line)?;
            return Ok(());
        }

        // Multi-line output
        self.write_indent(writer, level)?;
        writeln!(writer, "<{}>", tag)?;

        self.write_indent(writer, level + 1)?;
        self.write_spans(writer, content, level + 1, true, true)?;
        writeln!(writer)?;

        self.write_indent(writer, level)?;
        writeln!(writer, "</{}>", tag)
    }

    fn write_table_paragraph<W: Write>(
        &self,
        writer: &mut W,
        rows: &[TableRow],
        level: usize,
    ) -> io::Result<()> {
        if self.emit_tables {
            self.write_html_table(writer, rows, level)
        } else {
            self.write_flattened_table(writer, rows, level)
        }
    }

    fn write_flattened_table<W: Write>(
        &self,
        writer: &mut W,
        rows: &[TableRow],
        level: usize,
    ) -> io::Result<()> {
        // FTML has no table syntax. Flatten each non-empty cell into its own
        // `<p>` paragraph so the content survives the round-trip even though
        // the table structure is lost.
        let mut first = true;
        for row in rows {
            for cell in &row.cells {
                if cell.content.iter().all(|span| span.is_content_empty()) {
                    continue;
                }
                if !first {
                    writeln!(writer)?;
                }
                first = false;
                self.write_leaf_paragraph(writer, &cell.content, "p", level)?;
            }
        }
        Ok(())
    }

    fn write_html_table<W: Write>(
        &self,
        writer: &mut W,
        rows: &[TableRow],
        level: usize,
    ) -> io::Result<()> {
        self.write_indent(writer, level)?;
        writeln!(writer, "<table>")?;

        for row in rows {
            self.write_indent(writer, level + 1)?;
            writeln!(writer, "<tr>")?;
            for cell in &row.cells {
                self.write_table_cell(writer, cell, level + 2)?;
            }
            self.write_indent(writer, level + 1)?;
            writeln!(writer, "</tr>")?;
        }

        self.write_indent(writer, level)?;
        writeln!(writer, "</table>")
    }

    fn write_table_cell<W: Write>(
        &self,
        writer: &mut W,
        cell: &TableCell,
        level: usize,
    ) -> io::Result<()> {
        let tag = if cell.is_header { "th" } else { "td" };

        if cell.content.is_empty() {
            self.write_indent(writer, level)?;
            writeln!(writer, "<{}></{}>", tag, tag)?;
            return Ok(());
        }

        let single_line = self.render_single_line(&cell.content, tag, level);

        if single_line.chars().count() <= self.max_width && !single_line.trim_end().contains('\n') {
            write!(writer, "{}", single_line)?;
            return Ok(());
        }

        self.write_indent(writer, level)?;
        writeln!(writer, "<{}>", tag)?;

        self.write_indent(writer, level + 1)?;
        self.write_spans(writer, &cell.content, level + 1, true, true)?;
        writeln!(writer)?;

        self.write_indent(writer, level)?;
        writeln!(writer, "</{}>", tag)
    }

    fn write_checklist_item<W: Write>(
        &self,
        writer: &mut W,
        item: &ChecklistItem,
        level: usize,
    ) -> io::Result<()> {
        if item.children.is_empty() {
            let single_line = self.render_checklist_item_single_line(item, level);

            if !single_line.is_empty()
                && single_line.chars().count() <= self.max_width
                && !single_line.trim_end().contains('\n')
            {
                write!(writer, "{}", single_line)?;
                return Ok(());
            }
        }

        self.write_indent(writer, level)?;
        writeln!(writer, "<li>")?;

        self.write_indent(writer, level + 1)?;
        write!(writer, "<input type=\"checkbox\"")?;
        if item.checked {
            write!(writer, " checked")?;
        }
        write!(writer, " />")?;

        if !item.content.is_empty() {
            write!(writer, " ")?;
            self.write_spans(writer, &item.content, level + 1, true, true)?;
        }
        writeln!(writer)?;

        if !item.children.is_empty() {
            self.write_indent(writer, level + 1)?;
            writeln!(writer, "<ul>")?;
            for child in &item.children {
                self.write_checklist_item(writer, child, level + 2)?;
            }
            self.write_indent(writer, level + 1)?;
            writeln!(writer, "</ul>")?;
        }

        self.write_indent(writer, level)?;
        writeln!(writer, "</li>")
    }

    fn render_checklist_item_single_line(&self, item: &ChecklistItem, level: usize) -> String {
        if !item.children.is_empty() {
            return String::new();
        }

        let mut result = String::new();

        for _ in 0..level {
            result.push_str(&self.indentation);
        }

        result.push_str("<li><input type=\"checkbox\"");
        if item.checked {
            result.push_str(" checked");
        }
        result.push_str(" />");

        if !item.content.is_empty() {
            result.push(' ');
            for (idx, span) in item.content.iter().enumerate() {
                result.push_str(&self.render_span_simple(
                    span,
                    idx == 0,
                    idx == item.content.len() - 1,
                ));
            }
        }

        result.push_str("</li>\n");
        result
    }

    fn render_single_line(&self, content: &[Span], tag: &str, level: usize) -> String {
        let mut result = String::new();

        // Add indentation
        for _ in 0..level {
            result.push_str(&self.indentation);
        }

        result.push_str(&format!("<{}>", tag));

        for (idx, span) in content.iter().enumerate() {
            result.push_str(&self.render_span_simple(span, idx == 0, idx == content.len() - 1));
        }

        result.push_str(&format!("</{}>\n", tag));
        result
    }

    fn render_span_simple(&self, span: &Span, first: bool, last: bool) -> String {
        if span.style == InlineStyle::Link {
            return self.render_link_simple(span, first, last);
        }

        let mut result = String::new();

        if !span.children.is_empty() {
            if let Some(tag) = self.style_tags.get(&span.style) {
                result.push_str(&format!("<{}>", tag));
            }

            for child in &span.children {
                result.push_str(&self.render_span_simple(child, false, false));
            }

            if let Some(tag) = self.style_tags.get(&span.style) {
                result.push_str(&format!("</{}>", tag));
            }
        } else {
            let encoded_text = self.encode_entities(&span.text, first, last);
            let text_with_breaks = encoded_text.replace('\n', "<br />\n");
            result.push_str(&text_with_breaks);
        }

        result
    }

    fn render_link_simple(&self, span: &Span, first: bool, last: bool) -> String {
        let mut result = String::new();
        result.push_str("<a");
        if let Some(target) = &span.link_target {
            result.push_str(" href=\"");
            result.push_str(&self.encode_attribute(target));
            result.push('"');
        }
        result.push('>');

        if span.has_content() {
            if !span.text.is_empty() {
                let encoded_text = self.encode_entities(&span.text, first, last);
                let text_with_breaks = encoded_text.replace('\n', "<br />\n");
                result.push_str(&text_with_breaks);
            }
            for child in &span.children {
                result.push_str(&self.render_span_simple(child, false, false));
            }
        } else if let Some(target) = &span.link_target {
            let encoded_text = self.encode_entities(target, first, last);
            let text_with_breaks = encoded_text.replace('\n', "<br />\n");
            result.push_str(&text_with_breaks);
        }

        result.push_str("</a>");
        result
    }

    fn write_spans<W: Write>(
        &self,
        writer: &mut W,
        spans: &[Span],
        level: usize,
        first: bool,
        last: bool,
    ) -> io::Result<()> {
        for (idx, span) in spans.iter().enumerate() {
            let is_first = first && idx == 0;
            let is_last = last && idx == spans.len() - 1;
            self.write_span(writer, span, level, is_first, is_last)?;
        }
        Ok(())
    }

    fn collect_code_text(&self, spans: &[Span]) -> String {
        let mut buffer = String::new();
        for span in spans {
            Self::append_span_text(span, &mut buffer);
        }
        buffer
    }

    fn append_span_text(span: &Span, buffer: &mut String) {
        if !span.text.is_empty() {
            buffer.push_str(&span.text);
        }
        for child in &span.children {
            Self::append_span_text(child, buffer);
        }
    }

    fn encode_pre_text(&self, text: &str) -> String {
        let mut encoded = String::new();
        for ch in text.chars() {
            match ch {
                '&' => encoded.push_str("&amp;"),
                '<' => encoded.push_str("&lt;"),
                '>' => encoded.push_str("&gt;"),
                _ => encoded.push(ch),
            }
        }
        encoded
    }

    fn write_span<W: Write>(
        &self,
        writer: &mut W,
        span: &Span,
        level: usize,
        first: bool,
        last: bool,
    ) -> io::Result<()> {
        if span.style == InlineStyle::Link {
            return self.write_link_span(writer, span, level, first, last);
        }

        if let Some(tag) = self.style_tags.get(&span.style) {
            write!(writer, "<{}>", tag)?;
        }

        if span.children.is_empty() {
            let encoded_text = self.encode_entities(&span.text, first, last);
            let text_with_breaks = encoded_text.replace('\n', "<br />\n");
            self.emit_text(writer, &text_with_breaks, level)?;
        } else {
            for child in &span.children {
                self.write_span(writer, child, level, false, false)?;
            }
        }

        if let Some(tag) = self.style_tags.get(&span.style) {
            write!(writer, "</{}>", tag)?;
        }

        Ok(())
    }

    fn write_link_span<W: Write>(
        &self,
        writer: &mut W,
        span: &Span,
        level: usize,
        first: bool,
        last: bool,
    ) -> io::Result<()> {
        write!(writer, "<a")?;
        if let Some(target) = &span.link_target {
            write!(writer, " href=\"{}\"", self.encode_attribute(target))?;
        }
        write!(writer, ">")?;

        if span.has_content() {
            if !span.text.is_empty() {
                let encoded_text = self.encode_entities(&span.text, first, last);
                let text_with_breaks = encoded_text.replace('\n', "<br />\n");
                self.emit_text(writer, &text_with_breaks, level)?;
            }
            for child in &span.children {
                self.write_span(writer, child, level, false, false)?;
            }
        } else if let Some(target) = &span.link_target {
            let encoded_text = self.encode_entities(target, first, last);
            let text_with_breaks = encoded_text.replace('\n', "<br />\n");
            self.emit_text(writer, &text_with_breaks, level)?;
        }

        write!(writer, "</a>")?;
        Ok(())
    }

    fn emit_text<W: Write>(&self, writer: &mut W, text: &str, level: usize) -> io::Result<()> {
        let lines: Vec<&str> = text.split('\n').collect();

        for (line_idx, line) in lines.iter().enumerate() {
            if line_idx > 0 {
                writeln!(writer)?;
                self.write_indent(writer, level)?;
            }

            let words: Vec<&str> = self.any_space_regex.split(line).collect();
            let mut current_width = level * self.indentation.chars().count();

            for (word_idx, word) in words.iter().enumerate() {
                let word_width = word.chars().count();

                if word_idx > 0 {
                    if current_width + word_width + 1 >= self.max_width {
                        writeln!(writer)?;
                        self.write_indent(writer, level)?;
                        current_width = level * self.indentation.chars().count();
                    } else {
                        write!(writer, " ")?;
                        current_width += 1;
                    }
                }

                write!(writer, "{}", word)?;
                current_width += word_width;
            }
        }

        Ok(())
    }

    fn write_indent<W: Write>(&self, writer: &mut W, level: usize) -> io::Result<()> {
        for _ in 0..level {
            write!(writer, "{}", self.indentation)?;
        }
        Ok(())
    }

    fn encode_entities(&self, text: &str, first: bool, last: bool) -> String {
        let mut result = text.replace('\u{2005}', "&emsp14;");
        result = result.replace('\u{00A0}', "&nbsp;");

        // Handle spaces at start/end and multiple spaces
        if first {
            result = self
                .spaces_at_start_regex
                .replace_all(&result, |caps: &regex::Captures| {
                    self.replace_spaces(&caps[0])
                })
                .to_string();
        }

        if last {
            result = self
                .spaces_at_end_regex
                .replace_all(&result, |caps: &regex::Captures| {
                    self.replace_spaces(&caps[0])
                })
                .to_string();
        }

        result = self
            .multiple_spaces_regex
            .replace_all(&result, |caps: &regex::Captures| {
                self.replace_spaces(&caps[0])
            })
            .to_string();

        result = self
            .trailing_spaces_regex
            .replace_all(&result, |caps: &regex::Captures| {
                self.replace_trailing_spaces(&caps[0])
            })
            .to_string();

        result = self
            .leading_spaces_regex
            .replace_all(&result, |caps: &regex::Captures| {
                self.replace_leading_spaces(&caps[0])
            })
            .to_string();

        // Encode HTML entities
        result = result.replace('<', "&lt;");

        result
    }

    fn encode_attribute(&self, value: &str) -> String {
        let mut encoded = String::new();
        for ch in value.chars() {
            match ch {
                '&' => encoded.push_str("&amp;"),
                '"' => encoded.push_str("&quot;"),
                '<' => encoded.push_str("&lt;"),
                '>' => encoded.push_str("&gt;"),
                _ => encoded.push(ch),
            }
        }
        encoded
    }

    fn replace_spaces(&self, s: &str) -> String {
        "&emsp14;".repeat(s.len())
    }

    fn replace_trailing_spaces(&self, s: &str) -> String {
        if s.len() <= 1 {
            return s.to_string();
        }
        let first_char = &s[0..1];
        let spaces = &s[1..];
        format!("{}{}", first_char, "&emsp14;".repeat(spaces.len()))
    }

    fn replace_leading_spaces(&self, s: &str) -> String {
        if s.len() <= 1 {
            return s.to_string();
        }
        let spaces = &s[..s.len() - 1];
        let last_char = &s[s.len() - 1..];
        format!("{}{}", "&emsp14;".repeat(spaces.len()), last_char)
    }
}

/// Convenience helper that writes using a fresh [`Writer`] with default settings.
pub fn write<W: Write>(writer: &mut W, document: &Document) -> io::Result<()> {
    let w = Writer::new();
    w.write(writer, document)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ftml, Document, InlineStyle, Paragraph, Span};

    #[test]
    fn test_simple_paragraph() {
        let paragraph = Paragraph::new_text().with_content(vec![Span::new_text("This is a test.")]);
        let doc = Document::new().with_paragraphs(vec![paragraph]);

        let writer = Writer::new();
        let result = writer.write_to_string(&doc).unwrap();

        assert_eq!(result, "<p>This is a test.</p>\n");
    }

    #[test]
    fn test_bold_text() {
        let bold_span =
            Span::new_styled(InlineStyle::Bold).with_children(vec![Span::new_text("bold")]);
        let paragraph = Paragraph::new_text().with_content(vec![
            Span::new_text("This is "),
            bold_span,
            Span::new_text(" text."),
        ]);
        let doc = Document::new().with_paragraphs(vec![paragraph]);

        let writer = Writer::new();
        let result = writer.write_to_string(&doc).unwrap();

        assert_eq!(result, "<p>This is <b>bold</b> text.</p>\n");
    }

    #[test]
    fn test_link_text() {
        let link_span = Span::new_styled(InlineStyle::Link)
            .with_link_target("https://example.com")
            .with_children(vec![Span::new_text("Example")]);
        let paragraph = Paragraph::new_text().with_content(vec![Span::new_text("See "), link_span]);
        let doc = Document::new().with_paragraphs(vec![paragraph]);

        let writer = Writer::new();
        let result = writer.write_to_string(&doc).unwrap();

        assert_eq!(
            result,
            "<p>See <a href=\"https://example.com\">Example</a></p>\n"
        );
    }

    #[test]
    fn test_link_attribute_escaping() {
        let link_span = Span::new_styled(InlineStyle::Link)
            .with_link_target("https://example.com/?foo=1&bar=2")
            .with_children(vec![Span::new_text("Example")]);
        let paragraph = Paragraph::new_text().with_content(vec![link_span]);
        let doc = Document::new().with_paragraphs(vec![paragraph]);

        let writer = Writer::new();
        let result = writer.write_to_string(&doc).unwrap();

        assert!(
            result.contains("href=\"https://example.com/?foo=1&amp;bar=2\""),
            "unexpected writer output: {result}"
        );
    }

    #[test]
    fn test_horizontal_rule_dropped_by_ftml_writer() {
        // Strict FTML has no thematic-break element, so a horizontal rule that
        // arrived via conversion is dropped without leaving a stray blank line.
        let doc = Document::new().with_paragraphs(vec![
            Paragraph::new_text().with_content(vec![Span::new_text("A")]),
            Paragraph::new_horizontal_rule(),
            Paragraph::new_text().with_content(vec![Span::new_text("B")]),
        ]);

        let ftml = Writer::new().write_to_string(&doc).unwrap();
        assert_eq!(ftml, "<p>A</p>\n\n<p>B</p>\n");

        // The HTML writer, by contrast, keeps it as a `<hr />` void element.
        let html = Writer::new_html().write_to_string(&doc).unwrap();
        assert_eq!(html, "<p>A</p>\n\n<hr />\n\n<p>B</p>\n");
    }

    #[test]
    fn test_header() {
        let paragraph = Paragraph::new_header1().with_content(vec![Span::new_text("Header")]);
        let doc = Document::new().with_paragraphs(vec![paragraph]);

        let writer = Writer::new();
        let result = writer.write_to_string(&doc).unwrap();

        assert_eq!(result, "<h1>Header</h1>\n");
    }

    #[test]
    fn test_entity_encoding() {
        let paragraph = Paragraph::new_text().with_content(vec![Span::new_text(" test ")]);
        let doc = Document::new().with_paragraphs(vec![paragraph]);

        let writer = Writer::new();
        let result = writer.write_to_string(&doc).unwrap();

        assert_eq!(result, "<p>&emsp14;test&emsp14;</p>\n");
    }

    #[test]
    fn test_whitespace_edge_in_span() {
        fn w(doc: Document) -> String {
            let writer = Writer::new();
            writer.write_to_string(&doc).unwrap()
        }
        assert_eq!(
            w(ftml! { p { link { "yadayada" "Hier kommt ein Test! " } } }),
            "<p><a href=\"yadayada\">Hier kommt ein Test! </a></p>\n",
        );
    }

    #[test]
    fn test_table_writer_flattens_to_paragraphs() {
        use crate::{TableCell, TableRow};

        let rows = vec![
            TableRow::new().with_cells(vec![
                TableCell::new_header().with_content(vec![Span::new_text("Name")]),
                TableCell::new_header().with_content(vec![Span::new_text("Age")]),
            ]),
            TableRow::new().with_cells(vec![
                TableCell::new_data().with_content(vec![Span::new_text("Alice")]),
                TableCell::new_data().with_content(vec![]),
            ]),
        ];
        let paragraph = Paragraph::new_table().with_rows(rows);
        let doc = Document::new().with_paragraphs(vec![paragraph]);

        let writer = Writer::new();
        let result = writer.write_to_string(&doc).unwrap();

        // FTML has no table syntax; cells are flattened to paragraphs and
        // empty cells are dropped.
        assert_eq!(result, "<p>Name</p>\n\n<p>Age</p>\n\n<p>Alice</p>\n");
    }

    #[test]
    fn test_whitespace_handling() {
        fn w(doc: Document) -> String {
            let writer = Writer::new();
            writer.write_to_string(&doc).unwrap()
        }
        assert_eq!(
            w(ftml! { p { "Hier kommt ein Test!" } }),
            "<p>Hier kommt ein Test!</p>\n",
        );

        assert_eq!(w(ftml! { p { "A B" } }), "<p>A B</p>\n");

        assert_eq!(w(ftml! { p { "A  B" } }), "<p>A&emsp14;&emsp14;B</p>\n");

        assert_eq!(
            w(ftml! {
                p {
                    link { "https://www.cnn.com/terms" "Terms of Use " }
                    " | ",
                    link { "https://www.cnn.com/privacy" "Privacy Policy " }
                    " | ",
                    link { "https://www.cnn.com/ad-choices" "Ad Choices " }
                    " | Cookie Settings "
                }
            }),
            "<p>\n  <a href=\"https://www.cnn.com/terms\">Terms of Use </a> | <a href=\"https://www.cnn.com/privacy\">Privacy Policy </a> | <a href=\"https://www.cnn.com/ad-choices\">Ad Choices </a> | Cookie Settings&emsp14;\n</p>\n"
        );
    }
}
