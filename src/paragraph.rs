//! Paragraph primitives that make up the [`Document`](crate::Document) tree.

use crate::Span;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The structural role a [`Paragraph`] plays within a document.
pub enum ParagraphType {
    /// A plain text paragraph.
    Text,
    /// A level-1 heading (`<h1>`).
    Header1,
    /// A level-2 heading (`<h2>`).
    Header2,
    /// A level-3 heading (`<h3>`).
    Header3,
    /// A preformatted code block (`<pre>`).
    CodeBlock,
    /// An ordered list (`<ol>`) paragraph.
    OrderedList,
    /// An unordered (bulleted) list (`<ul>`) paragraph.
    UnorderedList,
    /// A checklist (`<ul>` with checkbox items).
    Checklist,
    /// A block quote (`<blockquote>`).
    Quote,
    /// A single checklist item (`<li>` with a checkbox input).
    ChecklistItem,
}

impl fmt::Display for ParagraphType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ParagraphType::Text => "Text",
            ParagraphType::Header1 => "Header Lvl 1",
            ParagraphType::Header2 => "Header Lvl 2",
            ParagraphType::Header3 => "Header Lvl 3",
            ParagraphType::CodeBlock => "Code Block",
            ParagraphType::OrderedList => "Ordered List",
            ParagraphType::UnorderedList => "Unordered List",
            ParagraphType::Checklist => "Checklist",
            ParagraphType::Quote => "Quote",
            ParagraphType::ChecklistItem => "Checklist Item",
        };
        write!(f, "{}", s)
    }
}

impl ParagraphType {
    /// Returns `true` if paragraphs of this type cannot contain child paragraphs.
    pub fn is_leaf(&self) -> bool {
        matches!(
            self,
            ParagraphType::Text
                | ParagraphType::Header1
                | ParagraphType::Header2
                | ParagraphType::Header3
                | ParagraphType::ChecklistItem
                | ParagraphType::CodeBlock
        )
    }

    /// Returns the canonical HTML tag used when serializing this paragraph type.
    pub fn html_tag(&self) -> &'static str {
        match self {
            ParagraphType::Text => "p",
            ParagraphType::Header1 => "h1",
            ParagraphType::Header2 => "h2",
            ParagraphType::Header3 => "h3",
            ParagraphType::CodeBlock => "pre",
            ParagraphType::OrderedList => "ol",
            ParagraphType::UnorderedList => "ul",
            ParagraphType::Checklist => "ul",
            ParagraphType::Quote => "blockquote",
            ParagraphType::ChecklistItem => "li",
        }
    }

    /// Attempts to map an HTML tag back to a [`ParagraphType`].
    pub fn from_html_tag(tag: &str) -> Option<Self> {
        match tag {
            "p" => Some(ParagraphType::Text),
            "h1" => Some(ParagraphType::Header1),
            "h2" => Some(ParagraphType::Header2),
            "h3" => Some(ParagraphType::Header3),
            "pre" => Some(ParagraphType::CodeBlock),
            "ol" => Some(ParagraphType::OrderedList),
            "ul" => Some(ParagraphType::UnorderedList),
            "blockquote" => Some(ParagraphType::Quote),
            "li" => Some(ParagraphType::ChecklistItem),
            _ => None,
        }
    }

    /// Returns `true` if the current paragraph type can be closed by the
    /// provided closing type (derived from the tag name).
    pub fn matches_closing_tag(self, closing: ParagraphType) -> bool {
        if self == closing {
            return true;
        }

        matches!(
            (self, closing),
            (ParagraphType::Checklist, ParagraphType::UnorderedList)
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
/// A node in the document tree representing text, lists, headings, or quotes.
///
/// Paragraphs can contain nested paragraphs (for quotes or nested lists), inline
/// [`Span`](crate::Span) content, or list entries depending on their
/// [`ParagraphType`].
///
/// # Examples
///
/// ```
/// use tdoc::{Paragraph, ParagraphType, Span};
///
/// // Simple paragraph with inline content.
/// let text = Paragraph::new_text().with_content(vec![Span::new_text("Hello FTML!")]);
/// assert!(text.is_leaf());
///
/// // List paragraphs manage their items via entries.
/// let mut list = Paragraph::new_unordered_list();
/// list.add_list_item(vec![Paragraph::new_text().with_content(vec![Span::new_text("One")])]);
/// list.add_list_item(vec![Paragraph::new_text().with_content(vec![Span::new_text("Two")])]);
/// assert!(!list.is_leaf());
/// ```
pub struct Paragraph {
    pub paragraph_type: ParagraphType,
    pub children: Vec<Paragraph>,
    pub content: Vec<Span>,
    pub entries: Vec<Vec<Paragraph>>, // For lists
    pub checklist_item_checked: Option<bool>,
}

impl Paragraph {
    /// Creates a paragraph with the provided [`ParagraphType`].
    pub fn new(paragraph_type: ParagraphType) -> Self {
        Self {
            paragraph_type,
            children: Vec::new(),
            content: Vec::new(),
            entries: Vec::new(),
            checklist_item_checked: None,
        }
    }

    /// Convenience constructor for [`ParagraphType::Text`].
    pub fn new_text() -> Self {
        Self::new(ParagraphType::Text)
    }

    /// Convenience constructor for [`ParagraphType::Header1`].
    pub fn new_header1() -> Self {
        Self::new(ParagraphType::Header1)
    }

    /// Convenience constructor for [`ParagraphType::Header2`].
    pub fn new_header2() -> Self {
        Self::new(ParagraphType::Header2)
    }

    /// Convenience constructor for [`ParagraphType::Header3`].
    pub fn new_header3() -> Self {
        Self::new(ParagraphType::Header3)
    }

    /// Convenience constructor for [`ParagraphType::CodeBlock`].
    pub fn new_code_block() -> Self {
        Self::new(ParagraphType::CodeBlock)
    }

    /// Convenience constructor for [`ParagraphType::OrderedList`].
    pub fn new_ordered_list() -> Self {
        Self::new(ParagraphType::OrderedList)
    }

    /// Convenience constructor for [`ParagraphType::UnorderedList`].
    pub fn new_unordered_list() -> Self {
        Self::new(ParagraphType::UnorderedList)
    }

    /// Convenience constructor for [`ParagraphType::Checklist`].
    pub fn new_checklist() -> Self {
        Self::new(ParagraphType::Checklist)
    }

    /// Convenience constructor for [`ParagraphType::ChecklistItem`].
    pub fn new_checklist_item(checked: bool) -> Self {
        let mut paragraph = Self::new(ParagraphType::ChecklistItem);
        paragraph.checklist_item_checked = Some(checked);
        paragraph
    }

    /// Convenience constructor for [`ParagraphType::Quote`].
    pub fn new_quote() -> Self {
        Self::new(ParagraphType::Quote)
    }

    /// Replaces the inline content of the paragraph.
    pub fn with_content(mut self, content: Vec<Span>) -> Self {
        self.content = content;
        self
    }

    /// Replaces the paragraph's child paragraphs.
    pub fn with_children(mut self, children: Vec<Paragraph>) -> Self {
        self.children = children;
        self
    }

    /// Replaces the paragraph's list entries.
    pub fn with_entries(mut self, entries: Vec<Vec<Paragraph>>) -> Self {
        self.entries = entries;
        self
    }

    /// Sets the checklist completion state for checklist items.
    pub fn with_checklist_state(mut self, checked: Option<bool>) -> Self {
        self.checklist_item_checked = checked;
        self
    }

    /// Returns `true` if this paragraph cannot contain nested paragraphs.
    pub fn is_leaf(&self) -> bool {
        self.paragraph_type.is_leaf()
    }

    /// Appends a child paragraph (used for quotes or nested structures).
    pub fn add_child(&mut self, child: Paragraph) {
        self.children.push(child);
    }

    /// Appends a single list item built from nested paragraphs.
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
        assert_eq!(ParagraphType::CodeBlock.html_tag(), "pre");
        assert_eq!(
            ParagraphType::from_html_tag("pre"),
            Some(ParagraphType::CodeBlock)
        );
        assert_eq!(ParagraphType::from_html_tag("div"), None);
    }

    #[test]
    fn test_is_leaf() {
        assert!(ParagraphType::Text.is_leaf());
        assert!(ParagraphType::Header1.is_leaf());
        assert!(ParagraphType::CodeBlock.is_leaf());
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
