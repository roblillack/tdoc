//! tdoc is a toolkit for building, parsing, formatting, and exporting FTML
//! (Formatted Text Markup Language) documents.
//!
//! The crate is centered around three core concepts:
//! - [`Document`], [`Paragraph`], and [`Span`], which form an in-memory tree
//!   representation of FTML content.
//! - Parsers (see [`parser`], [`html`], [`markdown`], and [`gemini`]) that turn external text
//!   into that tree.
//! - Writers and formatters (see [`writer`] and [`formatter`]) that turn the tree
//!   back into HTML, Markdown, Gemini, or richly styled terminal output.
//!
//! Checklists (Markdown `- [ ]` entries or HTML `<input type="checkbox">`
//! lists) map to [`ParagraphType::Checklist`] nodes that store [`ChecklistItem`]
//! children. Nested checklist items are preserved end-to-end so complex task
//! hierarchies round-trip across every parser and writer.
//!
//! Most applications start by building a [`Document`] manually or converting
//! some source text via [`parse`], manipulate or inspect the tree, and finally
//! render it with [`writer::Writer`] or [`formatter::Formatter`].

mod macros;

pub mod document;
pub mod formatter;
pub mod gemini;
pub mod html;
pub mod inline;
pub mod markdown;
pub mod pager;
pub mod paragraph;
pub mod parser;
pub mod test_helpers;
pub mod writer;

pub use document::Document;
pub use inline::{InlineStyle, Span};
pub use pager::*;
pub use paragraph::{ChecklistItem, Paragraph, ParagraphType};
pub use parser::parse;
pub use writer::write;

/// Convenience result type used across parsing and writing APIs.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
