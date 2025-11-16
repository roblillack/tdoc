//! Serialize [`Document`](crate::Document) trees back into FTML/HTML.

use crate::{ChecklistItem, Document, InlineStyle, Paragraph, ParagraphType, Span};
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Write};

/// Emits FTML/HTML markup from a [`Document`] tree.
///
/// `Writer` focuses on producing readable markup that preserves semantic tags
/// such as lists, block quotes, and inline styles. It defaults to two-space
/// indentation and an 80 character wrap width.
///
/// # Examples
///
/// ```
/// use tdoc::{Document, Paragraph, Span, writer::Writer};
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
    /// Creates a new writer with default indentation, wrapping, and styling.
    pub fn new() -> Self {
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
            if first {
                first = false;
            } else {
                writeln!(writer)?;
            }
            self.write_paragraph(writer, paragraph, 0)?;
        }
        Ok(())
    }

    fn write_paragraph<W: Write>(
        &self,
        writer: &mut W,
        paragraph: &Paragraph,
        level: usize,
    ) -> io::Result<()> {
        let paragraph_type = paragraph.paragraph_type();
        let tag = paragraph_type.html_tag();

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
