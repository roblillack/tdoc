use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InlineStyle {
    None,
    Bold,
    Italic,
    Highlight,
    Underline,
    Strike,
    Link,
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
pub struct Span {
    pub style: InlineStyle,
    pub text: String,
    pub link_target: Option<String>,
    pub children: Vec<Span>,
}

impl Span {
    pub fn new_text(text: impl Into<String>) -> Self {
        Self {
            style: InlineStyle::None,
            text: text.into(),
            link_target: None,
            children: Vec::new(),
        }
    }

    pub fn new_styled(style: InlineStyle) -> Self {
        Self {
            style,
            text: String::new(),
            link_target: None,
            children: Vec::new(),
        }
    }

    pub fn with_children(mut self, children: Vec<Span>) -> Self {
        self.children = children;
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn with_link_target(mut self, target: impl Into<String>) -> Self {
        self.link_target = Some(target.into());
        self
    }

    pub fn ends_with_line_break(&self) -> bool {
        if !self.children.is_empty() {
            if let Some(last) = self.children.last() {
                return last.ends_with_line_break();
            }
        }
        !self.text.is_empty() && self.text.ends_with('\n')
    }

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
