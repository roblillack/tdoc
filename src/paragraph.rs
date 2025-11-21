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
    /// An unordered (bulleted) list (`<ul>`).
    UnorderedList,
    /// A checklist (`<ul>` with checkbox items).
    Checklist,
    /// A block quote (`<blockquote>`).
    Quote,
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
/// [`ParagraphType`]. Modeling paragraphs as an enum ensures only valid
/// combinations of data are representable (e.g. lists always carry entries).
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
pub enum Paragraph {
    /// A plain text paragraph with inline spans.
    Text { content: Vec<Span> },
    /// A level-1 heading paragraph.
    Header1 { content: Vec<Span> },
    /// A level-2 heading paragraph.
    Header2 { content: Vec<Span> },
    /// A level-3 heading paragraph.
    Header3 { content: Vec<Span> },
    /// A preformatted code block paragraph.
    CodeBlock { content: Vec<Span> },
    /// An ordered list paragraph that owns list entries.
    OrderedList { entries: Vec<Vec<Paragraph>> },
    /// An unordered/bulleted list paragraph.
    UnorderedList { entries: Vec<Vec<Paragraph>> },
    /// A checklist paragraph with checklist items.
    Checklist { items: Vec<ChecklistItem> },
    /// A block quote paragraph that contains nested paragraphs.
    Quote { children: Vec<Paragraph> },
}

impl Paragraph {
    /// Creates a paragraph with the provided [`ParagraphType`].
    pub fn new(paragraph_type: ParagraphType) -> Self {
        match paragraph_type {
            ParagraphType::Text => Self::new_text(),
            ParagraphType::Header1 => Self::new_header1(),
            ParagraphType::Header2 => Self::new_header2(),
            ParagraphType::Header3 => Self::new_header3(),
            ParagraphType::CodeBlock => Self::new_code_block(),
            ParagraphType::OrderedList => Self::new_ordered_list(),
            ParagraphType::UnorderedList => Self::new_unordered_list(),
            ParagraphType::Checklist => Self::new_checklist(),
            ParagraphType::Quote => Self::new_quote(),
        }
    }

    /// Convenience constructor for [`ParagraphType::Text`].
    pub fn new_text() -> Self {
        Self::Text {
            content: Vec::new(),
        }
    }

    /// Convenience constructor for [`ParagraphType::Header1`].
    pub fn new_header1() -> Self {
        Self::Header1 {
            content: Vec::new(),
        }
    }

    /// Convenience constructor for [`ParagraphType::Header2`].
    pub fn new_header2() -> Self {
        Self::Header2 {
            content: Vec::new(),
        }
    }

    /// Convenience constructor for [`ParagraphType::Header3`].
    pub fn new_header3() -> Self {
        Self::Header3 {
            content: Vec::new(),
        }
    }

    /// Convenience constructor for [`ParagraphType::CodeBlock`].
    pub fn new_code_block() -> Self {
        Self::CodeBlock {
            content: Vec::new(),
        }
    }

    /// Convenience constructor for [`ParagraphType::OrderedList`].
    pub fn new_ordered_list() -> Self {
        Self::OrderedList {
            entries: Vec::new(),
        }
    }

    /// Convenience constructor for [`ParagraphType::UnorderedList`].
    pub fn new_unordered_list() -> Self {
        Self::UnorderedList {
            entries: Vec::new(),
        }
    }

    /// Convenience constructor for [`ParagraphType::Checklist`].
    pub fn new_checklist() -> Self {
        Self::Checklist { items: Vec::new() }
    }

    /// Convenience constructor for [`ParagraphType::Quote`].
    pub fn new_quote() -> Self {
        Self::Quote {
            children: Vec::new(),
        }
    }

    /// Returns the [`ParagraphType`] of the current paragraph.
    pub fn paragraph_type(&self) -> ParagraphType {
        match self {
            Paragraph::Text { .. } => ParagraphType::Text,
            Paragraph::Header1 { .. } => ParagraphType::Header1,
            Paragraph::Header2 { .. } => ParagraphType::Header2,
            Paragraph::Header3 { .. } => ParagraphType::Header3,
            Paragraph::CodeBlock { .. } => ParagraphType::CodeBlock,
            Paragraph::OrderedList { .. } => ParagraphType::OrderedList,
            Paragraph::UnorderedList { .. } => ParagraphType::UnorderedList,
            Paragraph::Checklist { .. } => ParagraphType::Checklist,
            Paragraph::Quote { .. } => ParagraphType::Quote,
        }
    }

    /// Returns `true` if this paragraph cannot contain nested paragraphs.
    pub fn is_leaf(&self) -> bool {
        self.paragraph_type().is_leaf()
    }

    /// Returns the inline content for leaf paragraphs, or an empty slice otherwise.
    pub fn content(&self) -> &[Span] {
        match self {
            Paragraph::Text { content }
            | Paragraph::Header1 { content }
            | Paragraph::Header2 { content }
            | Paragraph::Header3 { content }
            | Paragraph::CodeBlock { content } => content,
            _ => &[],
        }
    }

    /// Returns mutable inline content for leaf paragraphs.
    pub fn content_mut(&mut self) -> &mut Vec<Span> {
        match self {
            Paragraph::Text { content }
            | Paragraph::Header1 { content }
            | Paragraph::Header2 { content }
            | Paragraph::Header3 { content }
            | Paragraph::CodeBlock { content } => content,
            _ => panic!("only leaf paragraphs contain inline content"),
        }
    }

    /// Replaces the inline content of the paragraph.
    pub fn with_content(self, content: Vec<Span>) -> Self {
        match self {
            Paragraph::Text { .. } => Paragraph::Text { content },
            Paragraph::Header1 { .. } => Paragraph::Header1 { content },
            Paragraph::Header2 { .. } => Paragraph::Header2 { content },
            Paragraph::Header3 { .. } => Paragraph::Header3 { content },
            Paragraph::CodeBlock { .. } => Paragraph::CodeBlock { content },
            _ => panic!("only leaf paragraphs can hold inline content"),
        }
    }

    /// Returns the child paragraphs for quote nodes (or an empty slice).
    pub fn children(&self) -> &[Paragraph] {
        match self {
            Paragraph::Quote { children } => children,
            _ => &[],
        }
    }

    /// Returns mutable child paragraphs for quote nodes.
    pub fn children_mut(&mut self) -> &mut Vec<Paragraph> {
        match self {
            Paragraph::Quote { children } => children,
            _ => panic!("only block quotes hold child paragraphs"),
        }
    }

    /// Replaces the paragraph's child paragraphs.
    pub fn with_children(self, children: Vec<Paragraph>) -> Self {
        match self {
            Paragraph::Quote { .. } => Paragraph::Quote { children },
            _ => panic!("only block quotes can hold child paragraphs"),
        }
    }

    /// Appends a child paragraph (used for quotes or nested structures).
    pub fn add_child(&mut self, child: Paragraph) {
        self.children_mut().push(child);
    }

    /// Returns the list entries for list paragraphs (or an empty slice).
    pub fn entries(&self) -> &[Vec<Paragraph>] {
        match self {
            Paragraph::OrderedList { entries } | Paragraph::UnorderedList { entries } => entries,
            _ => &[],
        }
    }

    /// Returns mutable access to list entries for list paragraphs.
    pub fn entries_mut(&mut self) -> &mut Vec<Vec<Paragraph>> {
        match self {
            Paragraph::OrderedList { entries } | Paragraph::UnorderedList { entries } => entries,
            _ => panic!("only list paragraphs can hold entries"),
        }
    }

    /// Replaces the paragraph's list entries.
    pub fn with_entries(self, entries: Vec<Vec<Paragraph>>) -> Self {
        match self {
            Paragraph::OrderedList { .. } => Paragraph::OrderedList { entries },
            Paragraph::UnorderedList { .. } => Paragraph::UnorderedList { entries },
            _ => panic!("only list paragraphs can hold entries"),
        }
    }

    /// Appends a single list item built from nested paragraphs.
    pub fn add_list_item(&mut self, item: Vec<Paragraph>) {
        self.entries_mut().push(item);
    }

    /// Returns the checklist items for checklist paragraphs (or an empty slice).
    pub fn checklist_items(&self) -> &[ChecklistItem] {
        match self {
            Paragraph::Checklist { items } => items,
            _ => &[],
        }
    }

    /// Returns mutable access to checklist items for checklist paragraphs.
    pub fn checklist_items_mut(&mut self) -> &mut Vec<ChecklistItem> {
        match self {
            Paragraph::Checklist { items } => items,
            _ => panic!("only checklist paragraphs can hold checklist items"),
        }
    }

    /// Replaces the paragraph's checklist items.
    pub fn with_checklist_items(self, items: Vec<ChecklistItem>) -> Self {
        match self {
            Paragraph::Checklist { .. } => Paragraph::Checklist { items },
            _ => panic!("only checklist paragraphs can hold checklist items"),
        }
    }

    /// Appends a single checklist item. Only valid for checklist paragraphs.
    pub fn add_checklist_item(&mut self, item: ChecklistItem) {
        self.checklist_items_mut().push(item);
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a single item within a checklist.
///
/// Checklist items contain inline [`Span`](crate::Span) content along with
/// optional nested checklist items. Nested content is restricted to other
/// checklist items.
pub struct ChecklistItem {
    pub checked: bool,
    pub content: Vec<Span>,
    pub children: Vec<ChecklistItem>,
}

impl ChecklistItem {
    /// Creates a new checklist item with the provided completion state.
    pub fn new(checked: bool) -> Self {
        Self {
            checked,
            content: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Replaces the inline content of the checklist item.
    pub fn with_content(mut self, content: Vec<Span>) -> Self {
        self.content = content;
        self
    }

    /// Replaces the nested checklist children.
    pub fn with_children(mut self, children: Vec<ChecklistItem>) -> Self {
        self.children = children;
        self
    }

    /// Appends a nested checklist item.
    pub fn add_child(&mut self, child: ChecklistItem) {
        self.children.push(child);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(p.paragraph_type(), ParagraphType::Text);
        assert_eq!(p.content().len(), 1);
        assert_eq!(p.content()[0].text, "Hello");
    }
}
