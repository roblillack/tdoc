//! Application-defined ("custom") paragraph types.
//!
//! `tdoc`'s [`Paragraph`](crate::Paragraph) enum models a fixed set of
//! block types. Some content — images, text diagrams, email signatures — does
//! not fit those types. Rather than dropping or misrepresenting it, applications
//! register a [`CustomType`] in a [`CustomRegistry`] and pass that registry to
//! the parsers, writers, and [`Formatter`](crate::formatter::Formatter).
//!
//! The document model itself only ever stores plain data (see
//! [`CustomParagraph`](crate::CustomParagraph)); all behavior lives here, in the
//! registry, so the model stays cloneable and diffable.
//!
//! ```
//! use tdoc::custom::{builtins::Image, CustomRegistry};
//!
//! let registry = CustomRegistry::new().register(Image);
//! assert!(registry.get("image").is_some());
//! ```

use crate::CustomParagraph;
use std::collections::HashMap;
use std::sync::Arc;

/// Options passed to [`CustomType::render`] describing the terminal target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomRenderOptions {
    /// The number of columns available for content (after any indentation).
    pub width: usize,
    /// Whether ANSI escape codes may be emitted.
    pub ansi: bool,
}

/// Behavior for a single application-defined paragraph kind.
///
/// Implementations are registered in a [`CustomRegistry`] and consulted by the
/// parsers, writers, and [`Formatter`](crate::formatter::Formatter). Every method
/// except [`kind`](CustomType::kind) has a default, so a type only overrides what
/// it needs.
pub trait CustomType: Send + Sync {
    /// The kind identifier this type handles, matching [`CustomParagraph::kind`].
    fn kind(&self) -> &str;

    /// HTML tag names captured as this kind during parsing.
    ///
    /// Defaults to the [`kind`](CustomType::kind) itself, so a `<signature>`
    /// element maps to a `"signature"` kind out of the box. (FTML is a strict,
    /// closed format and never captures custom tags, so this affects HTML only.)
    fn html_tags(&self) -> Vec<&str> {
        vec![self.kind()]
    }

    /// Whether the Markdown parser should preserve standalone images as this kind.
    fn captures_markdown_images(&self) -> bool {
        false
    }

    /// Renders the paragraph for terminal output as a list of visible lines.
    ///
    /// Returning `None` lets the [`Formatter`](crate::formatter::Formatter) apply
    /// its generic fallback (the paragraph's inline content, or a placeholder).
    fn render(
        &self,
        _paragraph: &CustomParagraph,
        _options: &CustomRenderOptions,
    ) -> Option<Vec<String>> {
        None
    }

    /// Serializes the paragraph to HTML. `None` uses the generic fallback.
    ///
    /// There is deliberately no `to_ftml`: FTML is a strict subset of HTML5 and
    /// does not model custom paragraphs, so the FTML writer never consults a
    /// handler (it salvages inline content as a `<p>` instead).
    fn to_html(&self, _paragraph: &CustomParagraph) -> Option<String> {
        None
    }

    /// Serializes the paragraph to Markdown. `None` uses the generic fallback.
    fn to_markdown(&self, _paragraph: &CustomParagraph) -> Option<String> {
        None
    }

    /// Serializes the paragraph to Gemini text. `None` uses the generic fallback.
    fn to_gemini(&self, _paragraph: &CustomParagraph) -> Option<String> {
        None
    }
}

/// A collection of [`CustomType`] handlers keyed by kind.
///
/// An empty registry (the [`Default`]) reproduces `tdoc`'s behavior without
/// custom types, so passing one everywhere is always safe. The registry is cheap
/// to [`Clone`] (handlers are shared behind [`Arc`]).
#[derive(Clone, Default)]
pub struct CustomRegistry {
    types: HashMap<String, Arc<dyn CustomType>>,
    tag_to_kind: HashMap<String, String>,
    image_kind: Option<String>,
}

impl CustomRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a [`CustomType`], returning the updated registry (builder style).
    ///
    /// The type is indexed by its [`kind`](CustomType::kind) and by every tag in
    /// [`html_tags`](CustomType::html_tags); if it
    /// [captures images](CustomType::captures_markdown_images) it becomes the
    /// registry's image handler.
    pub fn register(mut self, custom_type: impl CustomType + 'static) -> Self {
        let handler: Arc<dyn CustomType> = Arc::new(custom_type);
        let kind = handler.kind().to_string();
        for tag in handler.html_tags() {
            self.tag_to_kind.insert(tag.to_string(), kind.clone());
        }
        if handler.captures_markdown_images() {
            self.image_kind = Some(kind.clone());
        }
        self.types.insert(kind, handler);
        self
    }

    /// Returns the handler for a kind, if registered.
    pub fn get(&self, kind: &str) -> Option<&dyn CustomType> {
        self.types.get(kind).map(|handler| handler.as_ref())
    }

    /// Returns the kind a given HTML tag should be captured as, if any.
    pub fn kind_for_tag(&self, tag: &str) -> Option<&str> {
        self.tag_to_kind.get(tag).map(String::as_str)
    }

    /// Returns `true` if the given HTML tag is registered for capture.
    pub fn captures_tag(&self, tag: &str) -> bool {
        self.tag_to_kind.contains_key(tag)
    }

    /// Returns the kind that preserves Markdown images, if one is registered.
    pub fn image_kind(&self) -> Option<&str> {
        self.image_kind.as_deref()
    }

    /// Returns `true` if no handlers are registered.
    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }
}

impl std::fmt::Debug for CustomRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut kinds: Vec<&String> = self.types.keys().collect();
        kinds.sort();
        f.debug_struct("CustomRegistry")
            .field("kinds", &kinds)
            .field("image_kind", &self.image_kind)
            .finish()
    }
}

/// Ready-made [`CustomType`] implementations shipped with the crate.
pub mod builtins {
    use super::{CustomParagraph, CustomRenderOptions, CustomType};

    /// A built-in image type (`kind = "image"`).
    ///
    /// Captures Markdown images and `<img>` elements, storing `src`, `alt`, and
    /// optional `title` as ordered attributes, and re-emits them in every format.
    /// Register it to give an application lossless image round-trips:
    ///
    /// ```
    /// use std::io::Cursor;
    /// use tdoc::custom::{builtins::Image, CustomRegistry};
    /// use tdoc::markdown;
    ///
    /// let registry = CustomRegistry::new().register(Image);
    /// let doc = markdown::parse_with(Cursor::new("![Logo](logo.png)\n"), &registry).unwrap();
    ///
    /// let mut out = Vec::new();
    /// markdown::write_with(&mut out, &doc, &registry).unwrap();
    /// assert_eq!(String::from_utf8(out).unwrap().trim(), "![Logo](logo.png)");
    /// ```
    #[derive(Debug, Clone, Copy, Default)]
    pub struct Image;

    impl Image {
        /// The kind identifier this type uses.
        pub const KIND: &'static str = "image";
    }

    impl CustomType for Image {
        fn kind(&self) -> &str {
            Image::KIND
        }

        fn html_tags(&self) -> Vec<&str> {
            vec!["img"]
        }

        fn captures_markdown_images(&self) -> bool {
            true
        }

        fn to_markdown(&self, paragraph: &CustomParagraph) -> Option<String> {
            let src = paragraph.attribute("src").unwrap_or_default();
            let alt = paragraph.attribute("alt").unwrap_or_default();
            Some(match paragraph.attribute("title") {
                Some(title) if !title.is_empty() => format!("![{alt}]({src} \"{title}\")"),
                _ => format!("![{alt}]({src})"),
            })
        }

        fn to_html(&self, paragraph: &CustomParagraph) -> Option<String> {
            Some(img_tag(paragraph))
        }

        fn to_gemini(&self, paragraph: &CustomParagraph) -> Option<String> {
            let src = paragraph.attribute("src").unwrap_or_default();
            let alt = paragraph.attribute("alt").unwrap_or_default();
            Some(if alt.is_empty() {
                format!("=> {src}")
            } else {
                format!("=> {src} {alt}")
            })
        }

        fn render(
            &self,
            paragraph: &CustomParagraph,
            options: &CustomRenderOptions,
        ) -> Option<Vec<String>> {
            let src = paragraph.attribute("src").unwrap_or_default();
            let alt = paragraph.attribute("alt").unwrap_or_default();
            let label = if alt.is_empty() { src } else { alt };
            let line = if options.ansi {
                format!("\u{1b}[2m[image: {label}]\u{1b}[0m")
            } else {
                format!("[image: {label}]")
            };
            Some(vec![line])
        }
    }

    /// Builds an `<img …/>` tag with HTML-escaped attribute values, matching the
    /// escaping the HTML writer uses elsewhere.
    fn img_tag(paragraph: &CustomParagraph) -> String {
        let mut out = String::from("<img");
        for (key, value) in &paragraph.attributes {
            out.push(' ');
            out.push_str(key);
            out.push_str("=\"");
            out.push_str(&encode_attribute(value));
            out.push('"');
        }
        out.push_str(" />");
        out
    }

    fn encode_attribute(value: &str) -> String {
        let mut encoded = String::new();
        for ch in value.chars() {
            match ch {
                '&' => encoded.push_str("&amp;"),
                '"' => encoded.push_str("&quot;"),
                '<' => encoded.push_str("&lt;"),
                '>' => encoded.push_str("&gt;"),
                _ => encoded.push(ch),
            }
        }
        encoded
    }
}

#[cfg(test)]
mod tests {
    use super::builtins::Image;
    use super::*;

    #[test]
    fn registry_indexes_by_kind_and_tag() {
        let registry = CustomRegistry::new().register(Image);
        assert!(!registry.is_empty());
        assert!(registry.get("image").is_some());
        assert_eq!(registry.kind_for_tag("img"), Some("image"));
        assert_eq!(registry.image_kind(), Some("image"));
        assert!(registry.get("missing").is_none());
    }

    #[test]
    fn image_serializes_per_format() {
        let image = CustomParagraph::new("image")
            .with_attribute("src", "logo.png")
            .with_attribute("alt", "Logo");
        let handler = Image;
        assert_eq!(handler.to_markdown(&image).unwrap(), "![Logo](logo.png)");
        assert_eq!(
            handler.to_html(&image).unwrap(),
            "<img src=\"logo.png\" alt=\"Logo\" />"
        );
        assert_eq!(handler.to_gemini(&image).unwrap(), "=> logo.png Logo");
    }
}
