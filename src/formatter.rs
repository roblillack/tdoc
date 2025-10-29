use crate::{Document, InlineStyle, Paragraph, ParagraphType, Span};
use std::collections::HashMap;
use std::io::Write;

const DEFAULT_WRAP_WIDTH: usize = 72;
const DEFAULT_QUOTE_PREFIX: &str = "| ";
const DEFAULT_UNORDERED_LIST_ITEM_PREFIX: &str = " • ";

#[derive(Clone)]
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

#[derive(Clone)]
pub struct FormattingStyle {
    pub reset_styles: String,
    pub text_styles: HashMap<InlineStyle, StyleTags>,
    pub quote_prefix: String,
    pub unordered_list_item_prefix: String,
    pub wrap_width: usize,
    pub left_padding: usize,
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
        }
    }
}

impl FormattingStyle {
    pub fn ascii() -> Self {
        Self::default()
    }

    pub fn ansi() -> Self {
        let mut text_styles = HashMap::new();
        text_styles.insert(
            InlineStyle::Bold,
            StyleTags::new("\x1b[1m", "\x1b[22m"),
        );
        text_styles.insert(
            InlineStyle::Italic,
            StyleTags::new("\x1b[3m", "\x1b[23m"),
        );
        text_styles.insert(
            InlineStyle::Highlight,
            StyleTags::new("\x1b[7m", "\x1b[27m"),
        );
        text_styles.insert(
            InlineStyle::Underline,
            StyleTags::new("\x1b[4m", "\x1b[24m"),
        );
        text_styles.insert(
            InlineStyle::Strike,
            StyleTags::new("\x1b[9m", "\x1b[29m"),
        );

        Self {
            reset_styles: "\x1b[0m".to_string(),
            text_styles,
            quote_prefix: DEFAULT_QUOTE_PREFIX.to_string(),
            unordered_list_item_prefix: DEFAULT_UNORDERED_LIST_ITEM_PREFIX.to_string(),
            wrap_width: DEFAULT_WRAP_WIDTH,
            left_padding: 0,
        }
    }
}

pub struct Formatter<W: Write> {
    pub style: FormattingStyle,
    writer: W,
    current_line_width: usize,
}

impl<W: Write> Formatter<W> {
    pub fn new(writer: W, style: FormattingStyle) -> Self {
        Self {
            writer,
            style,
            current_line_width: 0,
        }
    }

    pub fn new_ascii(writer: W) -> Self {
        Self::new(writer, FormattingStyle::ascii())
    }

    pub fn new_ansi(writer: W) -> Self {
        Self::new(writer, FormattingStyle::ansi())
    }

    pub fn write_document(&mut self, document: &Document) -> std::io::Result<()> {
        let indent = " ".repeat(self.style.left_padding);
        self.write_paragraphs(&document.paragraphs, &indent, &indent)?;

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
    ) -> std::io::Result<()> {
        for (i, paragraph) in paragraphs.iter().enumerate() {
            if i > 0 {
                writeln!(self.writer)?;
            }
            self.write_paragraph(paragraph, prefix, continuation_prefix)?;
        }
        Ok(())
    }

    fn write_paragraph(
        &mut self,
        paragraph: &Paragraph,
        prefix: &str,
        continuation_prefix: &str,
    ) -> std::io::Result<()> {
        match paragraph.paragraph_type {
            ParagraphType::Text
            | ParagraphType::Header1
            | ParagraphType::Header2
            | ParagraphType::Header3 => {
                self.write_text_paragraph(&paragraph.content, prefix, continuation_prefix)?;
            }
            ParagraphType::Quote => {
                let quote_prefix = format!("{}{}", prefix, self.style.quote_prefix);
                let quote_continuation = format!("{}{}", continuation_prefix, self.style.quote_prefix);
                
                for child in &paragraph.children {
                    self.write_paragraph(child, &quote_prefix, &quote_continuation)?;
                }
            }
            ParagraphType::UnorderedList => {
                for entry in &paragraph.entries {
                    let bullet_prefix = format!("{}{}", prefix, self.style.unordered_list_item_prefix);
                    let bullet_continuation = format!("{}  ", continuation_prefix);
                    
                    self.write_paragraphs(entry, &bullet_prefix, &bullet_continuation)?;
                }
            }
            ParagraphType::OrderedList => {
                for (i, entry) in paragraph.entries.iter().enumerate() {
                    let bullet_prefix = format!("{}{:2}. ", prefix, i + 1);
                    let bullet_continuation = format!("{}    ", continuation_prefix);
                    
                    self.write_paragraphs(entry, &bullet_prefix, &bullet_continuation)?;
                }
            }
        }
        Ok(())
    }

    fn write_text_paragraph(
        &mut self,
        spans: &[Span],
        prefix: &str,
        continuation_prefix: &str,
    ) -> std::io::Result<()> {
        if spans.is_empty() {
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

    fn collect_formatted_text(&self, span: &Span, parts: &mut Vec<String>) -> std::io::Result<()> {
        if span.children.is_empty() {
            // Handle line breaks specially
            if span.text.contains('\n') {
                for (i, line) in span.text.split('\n').enumerate() {
                    if i > 0 {
                        parts.push("\n".to_string());
                    }
                    if !line.is_empty() {
                        parts.push(line.to_string());
                    }
                }
            } else {
                parts.push(span.text.clone());
            }
        } else {
            // Apply styling to children
            if let Some(style_tags) = self.style.text_styles.get(&span.style) {
                parts.push(style_tags.begin.clone());
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

    fn write_wrapped_text(
        &mut self,
        parts: &[String],
        prefix: &str,
        continuation_prefix: &str,
    ) -> std::io::Result<()> {
        // First, concatenate all non-newline parts to get the full text
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

        // If we have forced line breaks, handle them specially
        if has_forced_breaks {
            let lines: Vec<&str> = full_text.split('\n').collect();
            for (i, line) in lines.iter().enumerate() {
                if i > 0 {
                    writeln!(self.writer)?;
                    write!(self.writer, "{}", continuation_prefix)?;
                }
                if i == 0 {
                    write!(self.writer, "{}", prefix)?;
                }
                self.write_wrapped_line(line, continuation_prefix)?;
            }
        } else {
            write!(self.writer, "{}", prefix)?;
            self.write_wrapped_line(&full_text, continuation_prefix)?;
        }

        Ok(())
    }

    fn write_wrapped_line(&mut self, text: &str, continuation_prefix: &str) -> std::io::Result<()> {
        if text.is_empty() {
            return Ok(());
        }

        let words: Vec<&str> = text.split_whitespace().collect();
        let mut current_line = String::new();
        let mut line_width = 0;

        for (i, word) in words.iter().enumerate() {
            let word_width = self.visible_width(word);
            let space_needed = if i > 0 { 1 } else { 0 };
            
            // Check if we need to wrap
            if line_width + space_needed + word_width > self.style.wrap_width && !current_line.is_empty() {
                // Write current line and start a new one
                write!(self.writer, "{}", current_line.trim_end())?;
                writeln!(self.writer)?;
                write!(self.writer, "{}", continuation_prefix)?;
                current_line.clear();
                line_width = continuation_prefix.chars().count();
            }
            
            // Add space if needed
            if !current_line.is_empty() {
                current_line.push(' ');
                line_width += 1;
            }
            
            current_line.push_str(word);
            line_width += word_width;
        }

        // Write any remaining content
        if !current_line.is_empty() {
            write!(self.writer, "{}", current_line)?;
        }

        Ok(())
    }

    fn visible_width(&self, text: &str) -> usize {
        // Remove ANSI escape sequences for width calculation
        let ansi_regex = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
        let visible_text = ansi_regex.replace_all(text, "");
        visible_text.chars().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn test_ascii_formatting() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);
        
        let doc = doc(vec![p_(vec![
            span("Hello "),
            b__("world"),
            span("!"),
        ])]);

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
        
        let doc = doc(vec![p_(vec![
            span("Hello "),
            b__("world"),
            span("!"),
        ])]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();
        
        // ANSI formatter should add bold styling
        assert!(result.contains("\x1b[1m")); // Bold begin
        assert!(result.contains("\x1b[22m")); // Bold end
        assert!(result.contains("\x1b[0m")); // Reset at end
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
    }

    #[test]
    fn test_wrap_width() {
        let mut output = Vec::new();
        let mut style = FormattingStyle::ascii();
        style.wrap_width = 20; // Very short for testing
        let mut formatter = Formatter::new(&mut output, style);
        
        let doc = doc(vec![p__("This is a very long line that should definitely be wrapped")]);

        formatter.write_document(&doc).unwrap();
        let result = String::from_utf8(output).unwrap();
        
        // Should contain line breaks due to wrapping
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() > 1);
    }
}