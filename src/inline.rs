//! Inline styling primitives used by paragraphs.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Available inline styles that can be applied to [`Span`] nodes.
pub enum InlineStyle {
    /// Unstyled text.
    None,
    /// Bold emphasis.
    Bold,
    /// Italic emphasis.
    Italic,
    /// Highlighted text (e.g. `<mark>`).
    Highlight,
    /// Underlined text.
    Underline,
    /// Strikethrough text.
    Strike,
    /// Hyperlink (`<a>`).
    Link,
    /// Inline code.
    Code,
}

impl fmt::Display for InlineStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            InlineStyle::None => "text",
            InlineStyle::Bold => "bold",
            InlineStyle::Italic => "italic",
            InlineStyle::Underline => "underline",
            InlineStyle::Strike => "striked",
            InlineStyle::Highlight => "highlight",
            InlineStyle::Link => "link",
            InlineStyle::Code => "code",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Inline-level node that holds styled or plain text content.
///
/// Spans can either contain literal text, nested spans (for composite styling),
/// or a combination of both. When `style` is [`InlineStyle::Link`], the optional
/// `link_target` is populated with the URL.
///
/// # Examples
///
/// ```
/// use tdoc::{InlineStyle, Span};
///
/// let plain = Span::new_text("plain text");
/// let link = Span::new_styled(InlineStyle::Link)
///     .with_children(vec![Span::new_text("The Book")])
///     .with_link_target("https://example.test");
///
/// assert_eq!(plain.text, "plain text");
/// assert_eq!(link.link_target.as_deref(), Some("https://example.test"));
/// ```
pub struct Span {
    pub style: InlineStyle,
    pub text: String,
    pub link_target: Option<String>,
    pub children: Vec<Span>,
}

impl Span {
    /// Creates an unstyled span that owns the provided text.
    pub fn new_text(text: impl Into<String>) -> Self {
        Self {
            style: InlineStyle::None,
            text: text.into(),
            link_target: None,
            children: Vec::new(),
        }
    }

    /// Creates a span with the given style and no text or children.
    pub fn new_styled(style: InlineStyle) -> Self {
        Self {
            style,
            text: String::new(),
            link_target: None,
            children: Vec::new(),
        }
    }

    /// Replaces the child spans, returning the updated span.
    pub fn with_children(mut self, children: Vec<Span>) -> Self {
        self.children = children;
        self
    }

    /// Replaces the span's text content, returning the updated span.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    /// Returns `true` when the span has either direct text or child spans.
    pub fn has_content(&self) -> bool {
        !self.text.is_empty() || !self.children.is_empty()
    }

    /// Returns `true` when the span has neither text nor child spans.
    pub fn is_content_empty(&self) -> bool {
        self.text.is_empty() && self.children.is_empty()
    }

    /// Sets the link target for [`InlineStyle::Link`] spans.
    pub fn with_link_target(mut self, target: impl Into<String>) -> Self {
        self.link_target = Some(target.into());
        self
    }

    /// Removes redundant link descriptions when they match the target URL.
    pub fn strip_redundant_link_description(&mut self) {
        if self.style != InlineStyle::Link {
            return;
        }

        let Some(target) = self.link_target.as_ref() else {
            return;
        };

        let mut description = String::new();
        self.collect_visible_text(&mut description);

        if description.trim() == target.trim() {
            self.text.clear();
            self.children.clear();
        }
    }

    fn collect_visible_text(&self, buffer: &mut String) {
        if !self.text.is_empty() {
            buffer.push_str(&self.text);
        }
        for child in &self.children {
            child.collect_visible_text(buffer);
        }
    }

    /// Returns `true` when the span's text or last descendant ends with whitespace.
    pub fn ends_with_whitespace(&self) -> bool {
        fn last_char(span: &Span) -> Option<char> {
            for child in span.children.iter().rev() {
                if let Some(ch) = last_char(child) {
                    return Some(ch);
                }
            }
            span.text.chars().next_back()
        }

        last_char(self)
            .map(|ch| ch.is_whitespace())
            .unwrap_or(false)
    }

    /// Returns `true` if the span's text or last descendant ends with `\n`.
    pub fn ends_with_line_break(&self) -> bool {
        if !self.children.is_empty() {
            if let Some(last) = self.children.last() {
                return last.ends_with_line_break();
            }
        }
        !self.text.is_empty() && self.text.ends_with('\n')
    }

    /// Computes the visible width of the span by counting Unicode scalar values.
    pub fn width(&self) -> usize {
        let text_width = self.text.chars().count();
        let children_width: usize = self.children.iter().map(|c| c.width()).sum();
        text_width + children_width
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.children.is_empty() {
            write!(f, "[{}:", self.style)?;
            for child in &self.children {
                write!(f, "{}", child)?;
            }
            write!(f, "]")
        } else {
            write!(f, "'{}'", self.text)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_display() {
        let span = Span::new_text("test");
        assert_eq!(format!("{}", span), "'test'");

        let bold_span =
            Span::new_styled(InlineStyle::Bold).with_children(vec![Span::new_text("bold text")]);
        assert_eq!(format!("{}", bold_span), "[bold:'bold text']");
    }

    #[test]
    fn test_ends_with_line_break() {
        let span = Span::new_text("test\n");
        assert!(span.ends_with_line_break());

        let span = Span::new_text("test");
        assert!(!span.ends_with_line_break());
    }

    #[test]
    fn test_width() {
        let span = Span::new_text("test");
        assert_eq!(span.width(), 4);

        let bold_span =
            Span::new_styled(InlineStyle::Bold).with_children(vec![Span::new_text("hi")]);
        assert_eq!(bold_span.width(), 2);
    }
}
