use crate::{Document, InlineStyle, Paragraph, ParagraphType, Span};
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Write};

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

    pub fn write_to_string(&self, document: &Document) -> io::Result<String> {
        let mut buffer = Vec::new();
        self.write(&mut buffer, document)?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

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
        let tag = paragraph.paragraph_type.html_tag();

        if paragraph.paragraph_type.is_leaf() {
            self.write_leaf_paragraph(writer, &paragraph.content, tag, level)
        } else {
            self.write_indent(writer, level)?;
            writeln!(writer, "<{}>", tag)?;

            if paragraph.paragraph_type == ParagraphType::UnorderedList
                || paragraph.paragraph_type == ParagraphType::OrderedList
            {
                let mut first = true;
                for entry in &paragraph.entries {
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
                for child in &paragraph.children {
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

    fn write_span<W: Write>(
        &self,
        writer: &mut W,
        span: &Span,
        level: usize,
        first: bool,
        last: bool,
    ) -> io::Result<()> {
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
        let mut result = text.to_string();

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

pub fn write<W: Write>(writer: &mut W, document: &Document) -> io::Result<()> {
    let w = Writer::new();
    w.write(writer, document)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Document, Paragraph, Span};

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
}
