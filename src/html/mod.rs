use crate::{Document, InlineStyle, Paragraph, ParagraphType, Span};
use std::collections::HashMap;
use std::io::Read;

pub fn parse<R: Read>(mut reader: R) -> crate::Result<Document> {
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let parser = HtmlParser::new();
    Ok(parser.parse_string(&input))
}

struct HtmlParser {
    inline_style_map: HashMap<String, InlineStyle>,
    block_element_map: HashMap<String, ParagraphType>,
}

impl HtmlParser {
    fn new() -> Self {
        let mut inline_style_map = HashMap::new();
        inline_style_map.insert("b".to_string(), InlineStyle::Bold);
        inline_style_map.insert("strong".to_string(), InlineStyle::Bold);
        inline_style_map.insert("i".to_string(), InlineStyle::Italic);
        inline_style_map.insert("em".to_string(), InlineStyle::Italic);
        inline_style_map.insert("u".to_string(), InlineStyle::Underline);
        inline_style_map.insert("s".to_string(), InlineStyle::Strike);
        inline_style_map.insert("del".to_string(), InlineStyle::Strike);
        inline_style_map.insert("strike".to_string(), InlineStyle::Strike);
        inline_style_map.insert("mark".to_string(), InlineStyle::Highlight);
        inline_style_map.insert("code".to_string(), InlineStyle::Code);
        inline_style_map.insert("tt".to_string(), InlineStyle::Code);

        let mut block_element_map = HashMap::new();
        block_element_map.insert("p".to_string(), ParagraphType::Text);
        block_element_map.insert("div".to_string(), ParagraphType::Text);
        block_element_map.insert("h1".to_string(), ParagraphType::Header1);
        block_element_map.insert("h2".to_string(), ParagraphType::Header2);
        block_element_map.insert("h3".to_string(), ParagraphType::Header3);
        block_element_map.insert("blockquote".to_string(), ParagraphType::Quote);
        block_element_map.insert("ul".to_string(), ParagraphType::UnorderedList);
        block_element_map.insert("ol".to_string(), ParagraphType::OrderedList);

        Self {
            inline_style_map,
            block_element_map,
        }
    }

    fn parse_string(&self, input: &str) -> Document {
        // Simple HTML to FTML conversion
        // This is a basic implementation that demonstrates the conversion logic

        let mut document = Document::new();
        let cleaned_input = self.clean_html(input);

        // Convert common HTML patterns to FTML
        let converted = self.convert_html_to_ftml(&cleaned_input);

        // Parse the converted FTML using our existing parser
        if let Ok(ftml_doc) = crate::parse(std::io::Cursor::new(converted)) {
            return ftml_doc;
        }

        // Fallback: create a simple text paragraph
        let paragraph = Paragraph::new_text()
            .with_content(vec![Span::new_text(self.extract_text_content(input))]);
        document.add_paragraph(paragraph);

        document
    }

    fn clean_html(&self, input: &str) -> String {
        // Remove HTML comments
        let comment_regex = regex::Regex::new(r"<!--.*?-->").unwrap();
        let mut cleaned = comment_regex.replace_all(input, "").to_string();

        // Remove script and style tags
        let script_regex = regex::Regex::new(r"(?is)<script[^>]*>.*?</script>").unwrap();
        cleaned = script_regex.replace_all(&cleaned, "").to_string();

        let style_regex = regex::Regex::new(r"(?is)<style[^>]*>.*?</style>").unwrap();
        cleaned = style_regex.replace_all(&cleaned, "").to_string();

        cleaned
    }

    fn convert_html_to_ftml(&self, input: &str) -> String {
        let mut result = input.to_string();

        // Convert HTML entities
        result = result.replace("&nbsp;", "&emsp14;");
        result = result.replace("&amp;", "&");
        result = result.replace("&quot;", "\"");
        result = result.replace("&#39;", "'");

        // Convert common HTML inline elements to FTML
        result = result.replace("<strong>", "<b>");
        result = result.replace("</strong>", "</b>");
        result = result.replace("<em>", "<i>");
        result = result.replace("</em>", "</i>");
        result = result.replace("<del>", "<s>");
        result = result.replace("</del>", "</s>");
        result = result.replace("<strike>", "<s>");
        result = result.replace("</strike>", "</s>");
        result = result.replace("<tt>", "<code>");
        result = result.replace("</tt>", "</code>");

        // Convert line breaks
        let br_regex = regex::Regex::new(r"<br\s*/?>\s*").unwrap();
        result = br_regex.replace_all(&result, "<br />").to_string();

        // Convert div to p (basic block-level conversion)
        result = result.replace("<div", "<p");
        result = result.replace("</div>", "</p>");

        // Remove unsupported attributes from tags
        result = self.remove_attributes(&result);

        // Wrap bare text in paragraphs
        result = self.wrap_bare_text(&result);

        result
    }

    fn remove_attributes(&self, input: &str) -> String {
        let tag_regex = regex::Regex::new(r"<(/?)(\w+)(?:\s+[^>]*)?(/?)>").unwrap();
        tag_regex
            .replace_all(input, |caps: &regex::Captures| {
                let closing_slash = &caps[1];
                let tag_name = &caps[2];
                let self_closing = &caps[3];

                if self_closing.is_empty() {
                    format!("<{}{}>", closing_slash, tag_name)
                } else {
                    format!("<{}{} />", closing_slash, tag_name)
                }
            })
            .to_string()
    }

    fn wrap_bare_text(&self, input: &str) -> String {
        // This is a simplified version - in practice, you'd want more sophisticated
        // text wrapping logic that preserves existing paragraph structures
        let lines: Vec<&str> = input.lines().collect();
        let mut result = String::new();
        let mut in_tag = false;
        let mut current_text = String::new();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if trimmed.starts_with('<') {
                // If we have accumulated text, wrap it in a paragraph
                if !current_text.trim().is_empty() {
                    result.push_str(&format!("<p>{}</p>\n", current_text.trim()));
                    current_text.clear();
                }
                result.push_str(line);
                result.push('\n');
                in_tag = true;
            } else if !in_tag {
                current_text.push_str(line);
                current_text.push(' ');
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }

        // Handle any remaining text
        if !current_text.trim().is_empty() {
            result.push_str(&format!("<p>{}</p>\n", current_text.trim()));
        }

        result
    }

    fn extract_text_content(&self, input: &str) -> String {
        let tag_regex = regex::Regex::new(r"<[^>]*>").unwrap();
        let text = tag_regex.replace_all(input, " ");

        // Decode HTML entities
        let mut result = text.to_string();
        result = result.replace("&nbsp;", " ");
        result = result.replace("&amp;", "&");
        result = result.replace("&lt;", "<");
        result = result.replace("&gt;", ">");
        result = result.replace("&quot;", "\"");
        result = result.replace("&#39;", "'");

        // Normalize whitespace
        let whitespace_regex = regex::Regex::new(r"\s+").unwrap();
        whitespace_regex
            .replace_all(&result, " ")
            .trim()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_simple_html_conversion() {
        let input = "<p>Hello <strong>world</strong>!</p>";
        let doc = parse(Cursor::new(input)).unwrap();

        assert!(!doc.paragraphs.is_empty());
        assert_eq!(doc.paragraphs[0].paragraph_type, ParagraphType::Text);
    }

    #[test]
    fn test_text_extraction() {
        let parser = HtmlParser::new();
        let input = "<p>Hello <b>world</b>!</p>";
        let text = parser.extract_text_content(input);
        assert_eq!(text, "Hello world !");
    }

    #[test]
    fn test_html_entity_conversion() {
        let parser = HtmlParser::new();
        let input = "Hello&nbsp;world&amp;test";
        let converted = parser.convert_html_to_ftml(input);
        assert!(converted.contains("&emsp14;"));
        assert!(converted.contains("&"));
    }
}
