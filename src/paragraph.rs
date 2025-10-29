use crate::Span;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParagraphType {
    Text,
    Header1,
    Header2,
    Header3,
    OrderedList,
    UnorderedList,
    Quote,
}

impl fmt::Display for ParagraphType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ParagraphType::Text => "Text",
            ParagraphType::Header1 => "Header Lvl 1",
            ParagraphType::Header2 => "Header Lvl 2",
            ParagraphType::Header3 => "Header Lvl 3",
            ParagraphType::OrderedList => "Ordered List",
            ParagraphType::UnorderedList => "Unordered List",
            ParagraphType::Quote => "Quote",
        };
        write!(f, "{}", s)
    }
}

impl ParagraphType {
    pub fn is_leaf(&self) -> bool {
        matches!(
            self,
            ParagraphType::Text
                | ParagraphType::Header1
                | ParagraphType::Header2
                | ParagraphType::Header3
        )
    }

    pub fn html_tag(&self) -> &'static str {
        match self {
            ParagraphType::Text => "p",
            ParagraphType::Header1 => "h1",
            ParagraphType::Header2 => "h2",
            ParagraphType::Header3 => "h3",
            ParagraphType::OrderedList => "ol",
            ParagraphType::UnorderedList => "ul",
            ParagraphType::Quote => "blockquote",
        }
    }

    pub fn from_html_tag(tag: &str) -> Option<Self> {
        match tag {
            "p" => Some(ParagraphType::Text),
            "h1" => Some(ParagraphType::Header1),
            "h2" => Some(ParagraphType::Header2),
            "h3" => Some(ParagraphType::Header3),
            "ol" => Some(ParagraphType::OrderedList),
            "ul" => Some(ParagraphType::UnorderedList),
            "blockquote" => Some(ParagraphType::Quote),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Paragraph {
    pub paragraph_type: ParagraphType,
    pub children: Vec<Paragraph>,
    pub content: Vec<Span>,
    pub entries: Vec<Vec<Paragraph>>, // For lists
}

impl Paragraph {
    pub fn new(paragraph_type: ParagraphType) -> Self {
        Self {
            paragraph_type,
            children: Vec::new(),
            content: Vec::new(),
            entries: Vec::new(),
        }
    }

    pub fn new_text() -> Self {
        Self::new(ParagraphType::Text)
    }

    pub fn new_header1() -> Self {
        Self::new(ParagraphType::Header1)
    }

    pub fn new_header2() -> Self {
        Self::new(ParagraphType::Header2)
    }

    pub fn new_header3() -> Self {
        Self::new(ParagraphType::Header3)
    }

    pub fn new_ordered_list() -> Self {
        Self::new(ParagraphType::OrderedList)
    }

    pub fn new_unordered_list() -> Self {
        Self::new(ParagraphType::UnorderedList)
    }

    pub fn new_quote() -> Self {
        Self::new(ParagraphType::Quote)
    }

    pub fn with_content(mut self, content: Vec<Span>) -> Self {
        self.content = content;
        self
    }

    pub fn with_children(mut self, children: Vec<Paragraph>) -> Self {
        self.children = children;
        self
    }

    pub fn with_entries(mut self, entries: Vec<Vec<Paragraph>>) -> Self {
        self.entries = entries;
        self
    }

    pub fn is_leaf(&self) -> bool {
        self.paragraph_type.is_leaf()
    }

    pub fn add_child(&mut self, child: Paragraph) {
        self.children.push(child);
    }

    pub fn add_list_item(&mut self, item: Vec<Paragraph>) {
        self.entries.push(item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Span;

    #[test]
    fn test_paragraph_type_display() {
        assert_eq!(format!("{}", ParagraphType::Text), "Text");
        assert_eq!(format!("{}", ParagraphType::Header1), "Header Lvl 1");
    }

    #[test]
    fn test_html_tag_conversion() {
        assert_eq!(ParagraphType::Text.html_tag(), "p");
        assert_eq!(ParagraphType::from_html_tag("p"), Some(ParagraphType::Text));
        assert_eq!(ParagraphType::from_html_tag("div"), None);
    }

    #[test]
    fn test_is_leaf() {
        assert!(ParagraphType::Text.is_leaf());
        assert!(ParagraphType::Header1.is_leaf());
        assert!(!ParagraphType::OrderedList.is_leaf());
        assert!(!ParagraphType::Quote.is_leaf());
    }

    #[test]
    fn test_paragraph_creation() {
        let p = Paragraph::new_text().with_content(vec![Span::new_text("Hello")]);

        assert_eq!(p.paragraph_type, ParagraphType::Text);
        assert_eq!(p.content.len(), 1);
        assert_eq!(p.content[0].text, "Hello");
    }
}
