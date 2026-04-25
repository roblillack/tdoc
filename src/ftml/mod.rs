//! Parse and emit FTML (Formatted Text Markup Language).
//!
//! FTML is the canonical, HTML-like serialization format for [`Document`](crate::Document)
//! trees. This module is the FTML counterpart to [`crate::html`],
//! [`crate::markdown`], and [`crate::gemini`].
//!
//! # Examples
//!
//! Parse FTML from a reader:
//!
//! ```
//! use std::io::Cursor;
//! use tdoc::ftml;
//!
//! let document = ftml::parse(Cursor::new("<p>Hello!</p>")).unwrap();
//! assert_eq!(document.paragraphs.len(), 1);
//! ```
//!
//! Serialize a document back to FTML:
//!
//! ```
//! use tdoc::{ftml, Document, Paragraph, Span};
//!
//! let paragraph = Paragraph::new_text().with_content(vec![Span::new_text("Hi")]);
//! let document = Document::new().with_paragraphs(vec![paragraph]);
//!
//! let mut output = Vec::new();
//! ftml::write(&mut output, &document).unwrap();
//! assert_eq!(String::from_utf8(output).unwrap(), "<p>Hi</p>\n");
//! ```

pub mod parser;
pub mod writer;

pub use parser::{parse, ParseError, Parser};
pub use writer::{write, Writer};
