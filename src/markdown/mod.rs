use crate::{Document, InlineStyle, Paragraph, ParagraphType, Span};
use std::io::Write;

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
            writeln!(writer)?;
        }
        write_paragraph(writer, paragraph, prefix, continuation_prefix)?;
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
            write!(writer, "{}", prefix)?;
            write_spans(writer, &paragraph.content)?;
            writeln!(writer)?;
        }
        ParagraphType::Header1 => {
            write!(writer, "{}# ", prefix)?;
            write_spans(writer, &paragraph.content)?;
            writeln!(writer)?;
        }
        ParagraphType::Header2 => {
            write!(writer, "{}## ", prefix)?;
            write_spans(writer, &paragraph.content)?;
            writeln!(writer)?;
        }
        ParagraphType::Header3 => {
            write!(writer, "{}### ", prefix)?;
            write_spans(writer, &paragraph.content)?;
            writeln!(writer)?;
        }
        ParagraphType::Quote => {
            let quote_prefix = format!("{}> ", prefix);
            let quote_continuation = format!("{}> ", continuation_prefix);

            for child in &paragraph.children {
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
    }
    Ok(())
}

fn write_spans<W: Write>(writer: &mut W, spans: &[Span]) -> std::io::Result<()> {
    for span in spans {
        write_span(writer, span)?;
    }
    Ok(())
}

fn write_span<W: Write>(writer: &mut W, span: &Span) -> std::io::Result<()> {
    let (begin_tag, end_tag) = match span.style {
        InlineStyle::None => ("", ""),
        InlineStyle::Bold => ("**", "**"),
        InlineStyle::Italic => ("_", "_"),
        InlineStyle::Highlight => ("<mark>", "</mark>"), // Markdown doesn't have native highlight
        InlineStyle::Underline => ("<u>", "</u>"),       // Markdown doesn't have native underline
        InlineStyle::Strike => ("~~", "~~"),
        InlineStyle::Link => {
            // Handle links if we had link targets
            if let Some(target) = &span.link_target {
                write!(writer, "[")?;
                write_span_content(writer, span)?;
                write!(writer, "]({})", target)?;
                return Ok(());
            } else {
                ("", "")
            }
        }
        InlineStyle::Code => ("`", "`"),
    };

    write!(writer, "{}", begin_tag)?;
    write_span_content(writer, span)?;
    write!(writer, "{}", end_tag)?;

    Ok(())
}

fn write_span_content<W: Write>(writer: &mut W, span: &Span) -> std::io::Result<()> {
    if span.children.is_empty() {
        // Handle line breaks in markdown
        if span.text.ends_with('\n') {
            let text_without_newline = span.text.trim_end_matches('\n');
            if !text_without_newline.is_empty() {
                write!(writer, "{}", text_without_newline)?;
            }
            writeln!(writer, "\\")?; // Markdown line break
        } else {
            write!(writer, "{}", span.text)?;
        }
    } else {
        for child in &span.children {
            write_span(writer, child)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

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
}
