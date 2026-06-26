//! tdoc is a toolkit for building, parsing, formatting, and exporting documents
//! across FTML, HTML, Markdown, and Gemini.
//!
//! The crate is centered around three core concepts:
//! - [`Document`], [`Paragraph`], and [`Span`], which form an in-memory tree
//!   representation of document content.
//! - Format modules (see [`ftml`], [`html`], [`markdown`], and [`gemini`]) that
//!   provide both parsers and writers for each external format.
//! - A [`formatter`] for rendering the tree to richly styled terminal output.
//!
//! Checklists (Markdown `- [ ]` entries or HTML `<input type="checkbox">`
//! lists) map to [`ParagraphType::Checklist`] nodes that store [`ChecklistItem`]
//! children. Nested checklist items are preserved end-to-end so complex task
//! hierarchies round-trip across every parser and writer.
//!
//! Most applications start by building a [`Document`] manually or converting
//! some source text via one of the format modules, manipulate or inspect the
//! tree, and finally render it with [`ftml::Writer`], [`html::Writer`], or
//! [`formatter::Formatter`].

mod macros;

pub mod custom;
pub mod document;
pub mod formatter;
pub mod ftml;
pub mod gemini;
pub mod html;
pub mod inline;
pub mod markdown;
pub mod metadata;
pub mod pager;
pub mod paragraph;
pub mod test_helpers;

pub use custom::{CustomRegistry, CustomRenderOptions, CustomType};
pub use document::Document;
pub use inline::{InlineStyle, Span};
pub use pager::*;
pub use paragraph::{
    ChecklistItem, CustomParagraph, Paragraph, ParagraphType, TableCell, TableRow,
};

/// Convenience result type used across parsing and writing APIs.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
