use std::io::Cursor;

use tdoc::html;
use tdoc::test_helpers::*;
use tdoc::Document;
use tdoc::writer::Writer;
use tdoc::Span;

fn parse(input: &str) -> Document {
    html::parse(Cursor::new(input)).expect("failed to parse html fragment")
}

fn render(document: &Document) -> String {
    Writer::new()
        .write_to_string(document)
        .expect("failed to render document")
        .trim()
        .to_string()
}

#[test]
fn parsing_simple_paragraphs() {
    let cases: &[(&str, Document)] = &[
        ("<p>This is a test.</p>", doc(vec![p__("This is a test.")])),
        ("<p>one</p><p>two</p>", doc(vec![p__("one"), p__("two")])),
        (
            "<blockquote><p>one</p></blockquote><p>two</p>",
            doc(vec![quote_(vec![p__("one")]), p__("two")]),
        ),
        (
            "<p><b>Bold</b> text.</p>",
            doc(vec![p_(vec![b__("Bold"), span(" text.")])]),
        ),
        (
            "<p><b>Bold<br />text.</b></p>",
            doc(vec![p_(vec![b_(vec![span("Bold\n"), span("text.")])])]),
        ),
        (
            "<p>Test <b>bold</b></p>",
            doc(vec![p_(vec![span("Test "), b__("bold")])]),
        ),
        ("<h1> Hello World! </h1>", doc(vec![h1_("Hello World!")])),
        (
            "<p>A<br/> B</p>",
            doc(vec![p_(vec![span("A"), span("\n"), span("B")])]),
        ),
        (
            "<ul><li><p>a</p></li><li><p>b</p></li></ul>",
            doc(vec![ul_(vec![li_(vec![p__("a")]), li_(vec![p__("b")])])]),
        ),
        (
            "<ul><li><p>a</p></li><li><p>b</p><p>c</p></li></ul>",
            doc(vec![ul_(vec![
                li_(vec![p__("a")]),
                li_(vec![p__("b"), p__("c")]),
            ])]),
        ),
    ];

    for (input, expected) in cases {
        let actual = parse(input);
        if actual != *expected {
            panic!(
                "mismatched parse for input `{}`\nexpected:\n{}\nactual:\n{}",
                input,
                render(expected),
                render(&actual)
            );
        }
    }
}

#[test]
fn parsing_paragraphs_with_extra_tags() {
    let cases: &[(&str, Document)] = &[
        ("<title>bla</title><p>This is a test.</p>", doc(vec![p__("This is a test.")])),
        (
            "<!--[if !((mso)|(IE))]><!-- --><div class=\"hse-column-container\" style=\"min-width:280px; max-width:600px; Margin-left:auto; Margin-right:auto; border-collapse:collapse; border-spacing:0; background-color:#003740; padding-bottom:25px\" bgcolor=\"#003740\"><!--<![endif]-->",
            doc(vec![]),
        ),
        (
            "<p>one</p><div><p>two</p></div>",
            doc(vec![p__("one"), p__("two")]),
        ),
        (
            "<blockquote><p>one</p></blockquote><p>two</p>",
            doc(vec![quote_(vec![p__("one")]), p__("two")]),
        ),
        (
            "<p><b>Bold</b> text.</p>",
            doc(vec![p_(vec![b__("Bold"), span(" text.")])]),
        ),
        (
            "<p><b>Bold<br />text.</b></p>",
            doc(vec![p_(vec![b_(vec![span("Bold\n"), span("text.")])])]),
        ),
        (
            "<p>Test <b>bold</b></p>",
            doc(vec![p_(vec![span("Test "), b__("bold")])]),
        ),
        ("<h1> Hello World! </h1>", doc(vec![h1_("Hello World!")])),
        (
            "<p>A<br/> B</p>",
            doc(vec![p_(vec![span("A"), span("\n"), span("B")])]),
        ),
        (
            "<ul><li><p>a</p></li><li><p>b</p></li></ul>",
            doc(vec![ul_(vec![li_(vec![p__("a")]), li_(vec![p__("b")])])]),
        ),
        (
            "<ul><li><p>a</p></li><li><p>b</p><p>c</p></li></ul>",
            doc(vec![ul_(vec![
                li_(vec![p__("a")]),
                li_(vec![p__("b"), p__("c")]),
            ])]),
        ),
    ];

    for (input, expected) in cases {
        let actual = parse(input);
        if actual != *expected {
            panic!(
                "mismatched parse for input `{}`\nexpected:\n{}\nactual:\n{}",
                input,
                render(expected),
                render(&actual)
            );
        }
    }
}

#[test]
fn parsing_and_writing_styles() {
    let simple_cases: &[(&str, Vec<Span>)] = &[
        ("This is a test.", vec![span("This is a test.")]),
        ("&emsp14;This is a test.", vec![span("\u{2005}This is a test.")]),
        ("This is a test.&emsp14;", vec![span("This is a test.\u{2005}")]),
        ("A&emsp14;&emsp14;&emsp14;B", vec![span("A\u{2005}\u{2005}\u{2005}B")]),
    ];

    let indented_cases: &[(&str, Vec<Span>)] = &[
        (
            "This is a <b>test</b>.",
            vec![span("This is a "), b__("test"), span(".")],
        ),
        (
            "This is a <b> test </b>.",
            vec![span("This is a "), b__(" test "), span(".")],
        ),
        (
            "This is a <b><i>second</i> test</b>.",
            vec![
                span("This is a "),
                b_(vec![i__("second"), span(" test")]),
                span("."),
            ],
        ),
    ];

    for (input, spans_vec) in simple_cases {
        let markup = format!("<p>{}</p>\n", input);
        let expected = doc(vec![p_(spans_vec.clone())]);
        let actual = parse(&markup);
        assert_eq!(
            expected, actual,
            "simple style parse mismatch for `{}`",
            input
        );
    }

    for (input, spans_vec) in indented_cases {
        let markup = format!("<p>{}</p>\n", input);
        let expected = doc(vec![p_(spans_vec.clone())]);
        let actual = parse(&markup);
        assert_eq!(
            expected, actual,
            "indented style parse mismatch for `{}`",
            input
        );
    }
}

#[test]
fn parsing_inline_styles() {
    let cases: &[(&str, Vec<Span>)] = &[
        (
            "This is a <b>test</b>.",
            vec![span("This is a "), b__("test"), span(".")],
        ),
        (
            "This is a <strong>test</strong>.",
            vec![span("This is a "), b__("test"), span(".")],
        ),
        (
            "This is a <b> test </b>.",
            vec![span("This is a "), b__(" test "), span(".")],
        ),
        (
            "This is a <em>test</em>.",
            vec![span("This is a "), i__("test"), span(".")],
        ),
        (
            "This is a <i>test</i>.",
            vec![span("This is a "), i__("test"), span(".")],
        ),
        (
            "This is a <mark>test</mark>.",
            vec![span("This is a "), mark__("test"), span(".")],
        ),
        (
            "This is a <u>test</u>.",
            vec![span("This is a "), u__("test"), span(".")],
        ),
        (
            "This is a <s>test</s>.",
            vec![span("This is a "), s__("test"), span(".")],
        ),
        (
            "This is a <del>test</del>.",
            vec![span("This is a "), s__("test"), span(".")],
        ),
        (
            "This is a <strike>test</strike>.",
            vec![span("This is a "), s__("test"), span(".")],
        ),
        (
            "This is a <code>test</code>.",
            vec![span("This is a "), code__("test"), span(".")],
        ),
        (
            "This is a <tt>test</tt>.",
            vec![span("This is a "), code__("test"), span(".")],
        ),
    ];

    for (input, spans_vec) in cases {
        let markup = format!("<p>{}</p>\n", input);
        let expected = doc(vec![p_(spans_vec.clone())]);
        let actual = parse(&markup);
        assert_eq!(
            expected, actual,
            "inline style parse mismatch for `{}`",
            input
        );
    }
}

#[test]
fn parsing_inline_styles_in_list_items() {
    let input = "<li>Hello <strong>World";
    let expected = "<ul>\n  <li>\n    <p>Hello <b>World</b></p>\n  </li>\n</ul>";

    let document = parse(input);
    let rendered = render(&document);

    assert_eq!(
        expected,
        rendered,
        "inline styles inside list items parsed incorrectly"
    );
}

#[test]
fn unclosed_block_elements() {
    let input = "<p>Hello<h1>World</h1>";
    let expected = "<p>Hello</p>\n\n<h1>World</h1>";

    let document = parse(input);
    let rendered = render(&document);

    assert_eq!(expected, rendered);
}

#[test]
fn unclosed_list_items() {
    let input = "<p>Oh,<li>Hello<li>World";
    let expected = "<p>Oh,</p>\n\n<ul>\n  <li>\n    <p>Hello</p>\n  </li>\n\n  <li>\n    <p>World</p>\n  </li>\n</ul>";

    let document = parse(input);
    let rendered = render(&document);

    assert_eq!(expected, rendered);
}

#[test]
fn list_items_without_paragraphs() {
    let input = "<li>Hello</li><li>World</li>";
    let expected =
        "<ul>\n  <li>\n    <p>Hello</p>\n  </li>\n\n  <li>\n    <p>World</p>\n  </li>\n</ul>";

    let document = parse(input);
    let rendered = render(&document);

    assert_eq!(expected, rendered);
}

#[test]
fn blockquote_without_paragraph() {
    let input = "<blockquote>Hello World";
    let expected = "<blockquote>\n  <p>Hello World</p>\n</blockquote>";

    let document = parse(input);
    let rendered = render(&document);

    assert_eq!(expected, rendered);
}

#[test]
fn blockquote_with_paragraph_and_space() {
    let input = "<blockquote>\n<p>\nHello World";
    let expected = "<blockquote>\n  <p>Hello World</p>\n</blockquote>";

    let document = parse(input);
    let rendered = render(&document);

    assert_eq!(expected, rendered);
}

#[test]
fn blockquote_with_inline_and_padding() {
    let input = "<blockquote>   <b>   Hello World";
    let expected = "<blockquote>\n  <p><b> Hello World</b></p>\n</blockquote>";

    let document = parse(input);
    let rendered = render(&document);

    assert_eq!(expected, rendered);
}

#[test]
fn parsing_links_become_text() {
    let input = "<ul><li><a href=\"xxx\">Hello</a> World";
    let expected = "<ul>\n  <li>\n    <p>Hello World</p>\n  </li>\n</ul>";

    let document = parse(input);
    let rendered = render(&document);

    assert_eq!(expected, rendered);
}

#[test]
fn parsing_error_resilience() {
    let inputs = [
        "This is a test.",
        "<p>one<p>two</p></p>",
        "<blockquote>one</blockquote>",
        "<p>one</blockquote>",
        "<h1><p>one</p></h1>",
        "<ul><p>boo</p></ul>",
        "<li>boo</li>",
        "<ul></li>",
    ];

    for input in inputs {
        parse(input);
    }
}
