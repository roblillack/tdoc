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
        self.write_paragraphs(&document.paragraphs, &indent, &indent, &indent)?;

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
            let previous_after = previous_type
                .map(|ty| self.blank_lines_after(ty))
                .unwrap_or(0);
            let blank_lines = self.blank_lines_before(previous_type, paragraph.paragraph_type);
            self.write_blank_lines_with_prefix(
                blank_line_prefix,
                previous_after.max(blank_lines),
            )?;
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
                    let mut owned_prefixes = Vec::with_capacity(2);
                    owned_prefixes.push(quote_prefix);
                    owned_prefixes.push(quote_continuation.clone());

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
                            continuation
                                .push_str(&" ".repeat(desired_width - current_width));
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
                    let bullet_prefix = format!("{}{:2}. ", base_prefix, i + 1);
                    let bullet_continuation = {
                        let desired_width = bullet_prefix.chars().count();
                        let current_width = base_prefix.chars().count();
                        let mut continuation = base_prefix.to_string();
                        if desired_width > current_width {
                            continuation
                                .push_str(&" ".repeat(desired_width - current_width));
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
        }
        Ok(())
    }

    fn write_blank_lines_with_prefix(
        &mut self,
        prefix: &str,
        count: usize,
    ) -> std::io::Result<()> {
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

    fn render_heading_text(&self, spans: &[Span]) -> std::io::Result<(String, usize)> {
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

        Ok(())
    }

    fn write_header2_paragraph(&mut self, spans: &[Span], prefix: &str) -> std::io::Result<()> {
        let (bold_text, visible_width) = self.render_heading_text(spans)?;

        write!(self.writer, "{}{}", prefix, bold_text)?;
        writeln!(self.writer)?;

        write!(self.writer, "{}", prefix)?;
        for _ in 0..visible_width {
            write!(self.writer, "=")?;
        }
        writeln!(self.writer)?;

        Ok(())
    }

    fn write_header3_paragraph(&mut self, spans: &[Span], prefix: &str) -> std::io::Result<()> {
        let (bold_text, visible_width) = self.render_heading_text(spans)?;

        write!(self.writer, "{}{}", prefix, bold_text)?;
        writeln!(self.writer)?;

        write!(self.writer, "{}", prefix)?;
        for _ in 0..visible_width {
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
                    self.write_wrapped_line(
                        line,
                        continuation_prefix.chars().count(),
                        continuation_prefix,
                    )?;
                } else {
                    write!(self.writer, "{}", prefix)?;
                    self.write_wrapped_line(
                        line,
                        prefix.chars().count(),
                        continuation_prefix,
                    )?;
                }
            }
        } else {
            write!(self.writer, "{}", prefix)?;
            self.write_wrapped_line(
                &full_text,
                prefix.chars().count(),
                continuation_prefix,
            )?;
        }

        Ok(())
    }

    fn write_wrapped_line(
        &mut self,
        text: &str,
        initial_width: usize,
        continuation_prefix: &str,
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

        let words: Vec<&str> = trimmed_text.split_whitespace().collect();
        if words.is_empty() {
            if !current_line.is_empty() {
                write!(self.writer, "{}", current_line)?;
            }
            return Ok(());
        }

        for (i, word) in words.iter().enumerate() {
            let word_width = self.visible_width(word);
            let space_needed = if i > 0 { 1 } else { 0 };

            // Check if we need to wrap
            if line_width + space_needed + word_width > self.style.wrap_width
                && !current_line.is_empty()
            {
                // Write current line and start a new one
                write!(self.writer, "{}", current_line.trim_end())?;
                writeln!(self.writer)?;
                write!(self.writer, "{}", continuation_prefix)?;
                line_width = continuation_prefix.chars().count();
                current_line.clear();
            }

            // Add space if needed (only if there is already non-indentation content on this line)
            if !current_line.trim().is_empty() {
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
    fn test_quote_multiple_paragraphs_spacing() {
        let mut output = Vec::new();
        let mut formatter = Formatter::new_ascii(&mut output);

        let doc = doc(vec![quote_(vec![p__("Paragraph one."), p__("Paragraph two.")])]);

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
}
