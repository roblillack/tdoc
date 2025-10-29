pub mod document;
pub mod formatter;
pub mod html;
pub mod inline;
pub mod markdown;
pub mod paragraph;
pub mod parser;
pub mod test_helpers;
pub mod writer;

pub use document::Document;
pub use inline::{InlineStyle, Span};
pub use paragraph::{Paragraph, ParagraphType};
pub use parser::parse;
pub use writer::write;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;