//! Convenience constructors for assembling documents in tests.

use crate::{Document, InlineStyle, Paragraph, Span};

pub fn p__(s: &str) -> Paragraph {
    Paragraph::new_text().with_content(vec![span(s)])
}

pub fn p_(content: Vec<Span>) -> Paragraph {
    Paragraph::new_text().with_content(content)
}

pub fn li_(entries: Vec<Paragraph>) -> Vec<Paragraph> {
    entries
}

pub fn ul_(entries: Vec<Vec<Paragraph>>) -> Paragraph {
    Paragraph::new_unordered_list().with_entries(entries)
}

pub fn ol_(entries: Vec<Vec<Paragraph>>) -> Paragraph {
    Paragraph::new_ordered_list().with_entries(entries)
}

pub fn h1_(s: &str) -> Paragraph {
    Paragraph::new_header1().with_content(vec![span(s)])
}

pub fn h2_(s: &str) -> Paragraph {
    Paragraph::new_header2().with_content(vec![span(s)])
}

pub fn h3_(s: &str) -> Paragraph {
    Paragraph::new_header3().with_content(vec![span(s)])
}

pub fn code_block__(s: &str) -> Paragraph {
    Paragraph::new_code_block().with_content(vec![span(s)])
}

pub fn quote_(children: Vec<Paragraph>) -> Paragraph {
    Paragraph::new_quote().with_children(children)
}

pub fn doc(children: Vec<Paragraph>) -> Document {
    Document::new().with_paragraphs(children)
}

pub fn span(txt: &str) -> Span {
    Span::new_text(txt)
}

pub fn spans(txt: &str) -> Vec<Span> {
    vec![Span::new_text(txt)]
}

pub fn b_(args: Vec<Span>) -> Span {
    Span::new_styled(InlineStyle::Bold).with_children(args)
}

pub fn i_(args: Vec<Span>) -> Span {
    Span::new_styled(InlineStyle::Italic).with_children(args)
}

pub fn b__(txt: &str) -> Span {
    Span::new_styled(InlineStyle::Bold).with_children(spans(txt))
}

pub fn i__(txt: &str) -> Span {
    Span::new_styled(InlineStyle::Italic).with_children(spans(txt))
}

pub fn mark__(txt: &str) -> Span {
    Span::new_styled(InlineStyle::Highlight).with_children(spans(txt))
}

pub fn u__(txt: &str) -> Span {
    Span::new_styled(InlineStyle::Underline).with_children(spans(txt))
}

pub fn s__(txt: &str) -> Span {
    Span::new_styled(InlineStyle::Strike).with_children(spans(txt))
}

pub fn code__(txt: &str) -> Span {
    Span::new_styled(InlineStyle::Code).with_children(spans(txt))
}

pub fn link__(target: &str) -> Span {
    Span::new_styled(InlineStyle::Link).with_link_target(target)
}

pub fn link_text__(target: &str, text: &str) -> Span {
    Span::new_styled(InlineStyle::Link)
        .with_link_target(target)
        .with_children(spans(text))
}

pub fn link_(target: &str, children: Vec<Span>) -> Span {
    Span::new_styled(InlineStyle::Link)
        .with_link_target(target)
        .with_children(children)
}
