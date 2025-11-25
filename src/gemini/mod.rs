//! Convert between Gemini text (.gmi) and FTML [`Document`](crate::Document) trees.
//!
//! Gemini is a lightweight markup language for the Gemini protocol.
//! This module provides bidirectional conversion between Gemini text
//! and FTML documents.

use crate::{Document, InlineStyle, Paragraph, ParagraphType, Span};
use std::io::{BufRead, BufReader, Read, Write};

/// Parses Gemini text into a [`Document`].
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// use tdoc::{gemini, ParagraphType};
///
/// let doc = gemini::parse(Cursor::new("# Heading")).unwrap();
/// assert_eq!(doc.paragraphs[0].paragraph_type(), ParagraphType::Header1);
/// ```
pub fn parse<R: Read>(reader: R) -> crate::Result<Document> {
    let buf_reader = BufReader::new(reader);
    let mut builder = GeminiBuilder::new();

    for line in buf_reader.lines() {
        let line = line?;
        builder.process_line(&line);
    }

    Ok(builder.finish())
}

struct GeminiBuilder {
    paragraphs: Vec<Paragraph>,
    in_preformatted: bool,
    preformatted_alt: String,
    preformatted_lines: Vec<String>,
    list_items: Vec<Vec<Paragraph>>,
    quote_lines: Vec<String>,
}

impl GeminiBuilder {
    fn new() -> Self {
        Self {
            paragraphs: Vec::new(),
            in_preformatted: false,
            preformatted_alt: String::new(),
            preformatted_lines: Vec::new(),
            list_items: Vec::new(),
            quote_lines: Vec::new(),
        }
    }

    fn process_line(&mut self, line: &str) {
        // Handle preformatted toggle
        if let Some(stripped) = line.strip_prefix("```") {
            if self.in_preformatted {
                // End preformatted block
                self.flush_preformatted();
                self.in_preformatted = false;
            } else {
                // Start preformatted block
                self.flush_list();
                self.flush_quote();
                self.in_preformatted = true;
                self.preformatted_alt = stripped.trim().to_string();
                self.preformatted_lines.clear();
            }
            return;
        }

        // Inside preformatted block
        if self.in_preformatted {
            self.preformatted_lines.push(line.to_string());
            return;
        }

        // Link lines
        if let Some(rest) = line.strip_prefix("=>") {
            self.flush_list();
            self.flush_quote();
            let rest = rest.trim_start();
            if let Some((url, description)) = parse_link_line(rest) {
                let span = if description.is_empty() {
                    Span::new_styled(InlineStyle::Link).with_link_target(url.to_string())
                } else {
                    Span::new_styled(InlineStyle::Link)
                        .with_link_target(url.to_string())
                        .with_children(vec![Span::new_text(description)])
                };
                let paragraph = Paragraph::new_text().with_content(vec![span]);
                self.paragraphs.push(paragraph);
            }
            return;
        }

        // Heading lines
        if let Some(rest) = line.strip_prefix("###") {
            self.flush_list();
            self.flush_quote();
            let content = rest.trim();
            if !content.is_empty() {
                let paragraph = Paragraph::new(ParagraphType::Header3)
                    .with_content(vec![Span::new_text(content)]);
                self.paragraphs.push(paragraph);
            }
            return;
        }
        if let Some(rest) = line.strip_prefix("##") {
            self.flush_list();
            self.flush_quote();
            let content = rest.trim();
            if !content.is_empty() {
                let paragraph = Paragraph::new(ParagraphType::Header2)
                    .with_content(vec![Span::new_text(content)]);
                self.paragraphs.push(paragraph);
            }
            return;
        }
        if let Some(rest) = line.strip_prefix('#') {
            self.flush_list();
            self.flush_quote();
            let content = rest.trim();
            if !content.is_empty() {
                let paragraph = Paragraph::new(ParagraphType::Header1)
                    .with_content(vec![Span::new_text(content)]);
                self.paragraphs.push(paragraph);
            }
            return;
        }

        // List item lines
        if let Some(rest) = line.strip_prefix('*') {
            self.flush_quote();
            let content = rest.trim();
            if !content.is_empty() {
                let paragraph = Paragraph::new_text().with_content(vec![Span::new_text(content)]);
                self.list_items.push(vec![paragraph]);
            }
            return;
        }

        // Quote lines
        if let Some(rest) = line.strip_prefix('>') {
            self.flush_list();
            let content = rest.trim_start();
            self.quote_lines.push(content.to_string());
            return;
        }

        // Empty line - flush accumulated content
        if line.trim().is_empty() {
            self.flush_list();
            self.flush_quote();
            return;
        }

        // Regular text line
        self.flush_list();
        self.flush_quote();
        let paragraph = Paragraph::new_text().with_content(vec![Span::new_text(line)]);
        self.paragraphs.push(paragraph);
    }

    fn flush_preformatted(&mut self) {
        if self.preformatted_lines.is_empty() {
            return;
        }

        let mut content = self.preformatted_lines.join("\n");
        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }

        let paragraph = Paragraph::new_code_block().with_content(vec![Span::new_text(content)]);
        self.paragraphs.push(paragraph);
        self.preformatted_lines.clear();
        self.preformatted_alt.clear();
    }

    fn flush_list(&mut self) {
        if self.list_items.is_empty() {
            return;
        }

        let paragraph =
            Paragraph::new_unordered_list().with_entries(std::mem::take(&mut self.list_items));
        self.paragraphs.push(paragraph);
    }

    fn flush_quote(&mut self) {
        if self.quote_lines.is_empty() {
            return;
        }

        let text = self.quote_lines.join("\n");
        let child = Paragraph::new_text().with_content(vec![Span::new_text(text)]);
        let paragraph = Paragraph::new_quote().with_children(vec![child]);
        self.paragraphs.push(paragraph);
        self.quote_lines.clear();
    }

    fn finish(mut self) -> Document {
        if self.in_preformatted {
            self.flush_preformatted();
        }
        self.flush_list();
        self.flush_quote();

        Document {
            paragraphs: self.paragraphs,
        }
    }
}

fn parse_link_line(rest: &str) -> Option<(&str, &str)> {
    // Format: URL [DESCRIPTION]
    // Find first whitespace to separate URL from description
    if let Some(idx) = rest.find(char::is_whitespace) {
        let url = &rest[..idx];
        let description = rest[idx..].trim();
        Some((url, description))
    } else {
        // No description, just URL
        Some((rest, ""))
    }
}

/// Serializes a [`Document`] structure back to Gemini text.
///
/// # Examples
///
/// ```
/// use tdoc::{Document, Paragraph, Span};
/// use tdoc::gemini;
///
/// let paragraph = Paragraph::new_text().with_content(vec![Span::new_text("Hello")]);
/// let document = Document::new().with_paragraphs(vec![paragraph]);
///
/// let mut output = Vec::new();
/// gemini::write(&mut output, &document).unwrap();
/// assert_eq!(String::from_utf8(output).unwrap(), "Hello\n");
/// ```
pub fn write<W: Write>(writer: &mut W, document: &Document) -> std::io::Result<()> {
    let mut first = true;
    for paragraph in &document.paragraphs {
        if !first {
            writeln!(writer)?;
        }
        write_paragraph(writer, paragraph)?;
        first = false;
    }
    Ok(())
}

fn write_paragraph<W: Write>(writer: &mut W, paragraph: &Paragraph) -> std::io::Result<()> {
    match paragraph {
        Paragraph::Text { content } => {
            write_text_paragraph(writer, content)?;
        }
        Paragraph::Header1 { content } => {
            write!(writer, "# ")?;
            write_spans_plain(writer, content)?;
            writeln!(writer)?;
        }
        Paragraph::Header2 { content } => {
            write!(writer, "## ")?;
            write_spans_plain(writer, content)?;
            writeln!(writer)?;
        }
        Paragraph::Header3 { content } => {
            write!(writer, "### ")?;
            write_spans_plain(writer, content)?;
            writeln!(writer)?;
        }
        Paragraph::CodeBlock { content } => {
            writeln!(writer, "```")?;
            write_spans_plain(writer, content)?;
            writeln!(writer, "```")?;
        }
        Paragraph::Quote { children } => {
            for child in children {
                write_quoted_paragraph(writer, child)?;
            }
        }
        Paragraph::UnorderedList { entries } | Paragraph::OrderedList { entries } => {
            for entry in entries {
                write!(writer, "* ")?;
                for (i, p) in entry.iter().enumerate() {
                    if i > 0 {
                        write!(writer, " ")?;
                    }
                    write_paragraph_inline(writer, p)?;
                }
                writeln!(writer)?;
            }
        }
        Paragraph::Checklist { items } => {
            // Gemini doesn't have native checklist support, render as unordered list
            for item in items {
                let marker = if item.checked { "[x]" } else { "[ ]" };
                write!(writer, "* {} ", marker)?;
                write_spans_plain(writer, &item.content)?;
                writeln!(writer)?;

                // Render nested children as indented items
                for child in &item.children {
                    write!(writer, "* {} ", if child.checked { "[x]" } else { "[ ]" })?;
                    write_spans_plain(writer, &child.content)?;
                    writeln!(writer)?;
                }
            }
        }
    }
    Ok(())
}

fn write_text_paragraph<W: Write>(writer: &mut W, content: &[Span]) -> std::io::Result<()> {
    // Check if this is a single link span (Gemini link line)
    if content.len() == 1 {
        let span = &content[0];
        if span.style == InlineStyle::Link {
            if let Some(url) = &span.link_target {
                write!(writer, "=> {}", url)?;
                if span.has_content() {
                    write!(writer, " ")?;
                    write_span_content(writer, span)?;
                }
                writeln!(writer)?;
                return Ok(());
            }
        }
    }

    // Regular text paragraph
    write_spans_plain(writer, content)?;
    writeln!(writer)?;
    Ok(())
}

fn write_quoted_paragraph<W: Write>(writer: &mut W, paragraph: &Paragraph) -> std::io::Result<()> {
    match paragraph {
        Paragraph::Text { content } => {
            // Split content by newlines and prefix each with >
            let text = collect_plain_text_from_spans(content);
            for line in text.lines() {
                writeln!(writer, "> {}", line)?;
            }
        }
        Paragraph::Quote { children } => {
            for child in children {
                write_quoted_paragraph(writer, child)?;
            }
        }
        _ => {
            write!(writer, "> ")?;
            write_paragraph_inline(writer, paragraph)?;
            writeln!(writer)?;
        }
    }
    Ok(())
}

fn write_paragraph_inline<W: Write>(writer: &mut W, paragraph: &Paragraph) -> std::io::Result<()> {
    match paragraph {
        Paragraph::Text { content }
        | Paragraph::Header1 { content }
        | Paragraph::Header2 { content }
        | Paragraph::Header3 { content } => {
            write_spans_plain(writer, content)?;
        }
        Paragraph::CodeBlock { content } => {
            write_spans_plain(writer, content)?;
        }
        _ => {}
    }
    Ok(())
}

fn write_spans_plain<W: Write>(writer: &mut W, spans: &[Span]) -> std::io::Result<()> {
    for span in spans {
        write_span_plain(writer, span)?;
    }
    Ok(())
}

fn write_span_plain<W: Write>(writer: &mut W, span: &Span) -> std::io::Result<()> {
    // Gemini doesn't support inline formatting, so we flatten everything
    if !span.text.is_empty() {
        writer.write_all(span.text.as_bytes())?;
    }
    for child in &span.children {
        write_span_plain(writer, child)?;
    }
    Ok(())
}

fn write_span_content<W: Write>(writer: &mut W, span: &Span) -> std::io::Result<()> {
    if !span.text.is_empty() {
        writer.write_all(span.text.as_bytes())?;
    }
    for child in &span.children {
        write_span_plain(writer, child)?;
    }
    Ok(())
}

fn collect_plain_text_from_spans(spans: &[Span]) -> String {
    let mut result = String::new();
    for span in spans {
        collect_plain_text(span, &mut result);
    }
    result
}

fn collect_plain_text(span: &Span, buffer: &mut String) {
    if !span.text.is_empty() {
        buffer.push_str(&span.text);
    }
    for child in &span.children {
        collect_plain_text(child, buffer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_simple_paragraph() {
        let input = "Hello world!";
        let parsed = parse(Cursor::new(input)).unwrap();
        let expected = doc(vec![p__("Hello world!")]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_header_1() {
        let input = "# Heading level 1";
        let parsed = parse(Cursor::new(input)).unwrap();
        let expected = doc(vec![h1_("Heading level 1")]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_header_2() {
        let input = "## Heading level 2";
        let parsed = parse(Cursor::new(input)).unwrap();
        let expected = doc(vec![h2_("Heading level 2")]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_header_3() {
        let input = "### Heading level 3";
        let parsed = parse(Cursor::new(input)).unwrap();
        let expected = doc(vec![h3_("Heading level 3")]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_link() {
        let input = "=> https://example.com Example Site";
        let parsed = parse(Cursor::new(input)).unwrap();
        assert_eq!(parsed.paragraphs.len(), 1);

        let paragraph = &parsed.paragraphs[0];
        assert_eq!(paragraph.content().len(), 1);

        let span = &paragraph.content()[0];
        assert_eq!(span.style, InlineStyle::Link);
        assert_eq!(span.link_target.as_deref(), Some("https://example.com"));
        assert_eq!(span.children.len(), 1);
        assert_eq!(span.children[0].text, "Example Site");
    }

    #[test]
    fn test_parse_link_without_description() {
        let input = "=> https://example.com";
        let parsed = parse(Cursor::new(input)).unwrap();
        assert_eq!(parsed.paragraphs.len(), 1);

        let paragraph = &parsed.paragraphs[0];
        assert_eq!(paragraph.content().len(), 1);

        let span = &paragraph.content()[0];
        assert_eq!(span.style, InlineStyle::Link);
        assert_eq!(span.link_target.as_deref(), Some("https://example.com"));
        assert!(span.children.is_empty());
    }

    #[test]
    fn test_parse_list() {
        let input = "* First\n* Second\n* Third";
        let parsed = parse(Cursor::new(input)).unwrap();
        let expected = doc(vec![ul_(vec![
            li_(vec![p__("First")]),
            li_(vec![p__("Second")]),
            li_(vec![p__("Third")]),
        ])]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_quote() {
        let input = "> This is a quote\n> spanning multiple lines";
        let parsed = parse(Cursor::new(input)).unwrap();
        assert_eq!(parsed.paragraphs.len(), 1);
        match &parsed.paragraphs[0] {
            Paragraph::Quote { children } => {
                assert_eq!(children.len(), 1);
                match &children[0] {
                    Paragraph::Text { content } => {
                        let text = collect_plain_text_from_spans(content);
                        assert_eq!(text, "This is a quote\nspanning multiple lines");
                    }
                    _ => panic!("Expected text paragraph in quote"),
                }
            }
            _ => panic!("Expected quote paragraph"),
        }
    }

    #[test]
    fn test_parse_preformatted() {
        let input = "```\nfn main() {\n    println!(\"Hello\");\n}\n```";
        let parsed = parse(Cursor::new(input)).unwrap();
        assert_eq!(parsed.paragraphs.len(), 1);
        match &parsed.paragraphs[0] {
            Paragraph::CodeBlock { content } => {
                let text = collect_plain_text_from_spans(content);
                assert_eq!(text, "fn main() {\n    println!(\"Hello\");\n}\n");
            }
            _ => panic!("Expected code block paragraph"),
        }
    }

    #[test]
    fn test_write_simple_paragraph() {
        let mut output = Vec::new();
        let doc = doc(vec![p__("Hello world!")]);
        write(&mut output, &doc).unwrap();
        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "Hello world!\n");
    }

    #[test]
    fn test_write_header() {
        let mut output = Vec::new();
        let doc = doc(vec![h1_("Main Header")]);
        write(&mut output, &doc).unwrap();
        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "# Main Header\n");
    }

    #[test]
    fn test_write_link() {
        let mut output = Vec::new();
        let doc = doc(vec![p_(vec![link_text__(
            "https://example.com",
            "Example",
        )])]);
        write(&mut output, &doc).unwrap();
        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "=> https://example.com Example\n");
    }

    #[test]
    fn test_write_list() {
        let mut output = Vec::new();
        let doc = doc(vec![ul_(vec![
            li_(vec![p__("First")]),
            li_(vec![p__("Second")]),
        ])]);
        write(&mut output, &doc).unwrap();
        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "* First\n* Second\n");
    }

    #[test]
    fn test_write_quote() {
        let mut output = Vec::new();
        let doc = doc(vec![quote_(vec![p__("This is quoted.")])]);
        write(&mut output, &doc).unwrap();
        let result = String::from_utf8(output).unwrap();
        assert_eq!(result, "> This is quoted.\n");
    }

    #[test]
    fn test_write_code_block() {
        let mut output = Vec::new();
        let doc = doc(vec![code_block__(
            "fn main() {\n    println!(\"Hello\");\n}\n",
        )]);
        write(&mut output, &doc).unwrap();
        let result = String::from_utf8(output).unwrap();
        assert_eq!(
            result,
            "```\nfn main() {\n    println!(\"Hello\");\n}\n```\n"
        );
    }
}
