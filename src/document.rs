//! Defines the [`Document`] root node for FTML content.

use crate::metadata::Metadata;
use crate::Paragraph;

#[derive(Debug, Clone, PartialEq)]
/// A collection of top-level [`Paragraph`] nodes that make up an FTML document.
///
/// The struct is intentionally lightweight: it simply stores the top-level
/// paragraphs in the order in which they should render.
///
/// # Examples
///
/// Building a document manually:
///
/// ```
/// use tdoc::{Document, Paragraph, Span};
///
/// let mut doc = Document::new();
/// let paragraph = Paragraph::new_text().with_content(vec![Span::new_text("Hello!")]);
/// doc.add_paragraph(paragraph);
///
/// assert_eq!(doc.paragraphs.len(), 1);
/// assert!(!doc.is_empty());
/// ```
pub struct Document {
    /// Optional document metadata (e.g., YAML frontmatter in Markdown).
    pub metadata: Option<Metadata>,
    /// The document's content as a list of paragraphs.
    pub paragraphs: Vec<Paragraph>,
}

impl Document {
    /// Creates an empty document with no paragraphs.
    pub fn new() -> Self {
        Self {
            metadata: None,
            paragraphs: Vec::new(),
        }
    }

    /// Replaces the document's paragraphs, returning the updated document.
    pub fn with_paragraphs(mut self, paragraphs: Vec<Paragraph>) -> Self {
        self.paragraphs = paragraphs;
        self
    }

    /// Sets the document's metadata, returning the updated document.
    pub fn with_metadata(mut self, metadata: Metadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Appends a new paragraph to the end of the document.
    pub fn add_paragraph(&mut self, paragraph: Paragraph) {
        self.paragraphs.push(paragraph);
    }

    /// Returns `true` when the document contains no paragraphs.
    pub fn is_empty(&self) -> bool {
        self.paragraphs.is_empty()
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Paragraph, Span};

    #[test]
    fn test_document_creation() {
        let doc = Document::new();
        assert!(doc.is_empty());

        let p = Paragraph::new_text().with_content(vec![Span::new_text("Hello")]);
        let doc = Document::new().with_paragraphs(vec![p]);

        assert!(!doc.is_empty());
        assert_eq!(doc.paragraphs.len(), 1);
    }

    #[test]
    fn test_add_paragraph() {
        let mut doc = Document::new();
        let p = Paragraph::new_text();
        doc.add_paragraph(p);

        assert!(!doc.is_empty());
        assert_eq!(doc.paragraphs.len(), 1);
    }
}
