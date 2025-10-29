use crate::Paragraph;

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub paragraphs: Vec<Paragraph>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            paragraphs: Vec::new(),
        }
    }

    pub fn with_paragraphs(mut self, paragraphs: Vec<Paragraph>) -> Self {
        self.paragraphs = paragraphs;
        self
    }

    pub fn add_paragraph(&mut self, paragraph: Paragraph) {
        self.paragraphs.push(paragraph);
    }

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
        
        let p = Paragraph::new_text()
            .with_content(vec![Span::new_text("Hello")]);
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