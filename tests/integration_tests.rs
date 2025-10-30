use std::io::Cursor;
use tdoc::test_helpers::*;
use tdoc::{parse, write, Document, InlineStyle, Paragraph, ParagraphType, Span};

#[test]
fn test_parsing_simple_paragraphs() {
    let tests = vec![
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

    for (input, expected) in tests {
        let result = parse(Cursor::new(input)).unwrap();
        assert_eq!(result, expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_parsing_and_writing_styles() {
    let simple_tests = vec![
        ("This is a test.", vec![span("This is a test.")]),
        ("&emsp14;This is a test.", vec![span(" This is a test.")]),
        ("This is a test.&emsp14;", vec![span("This is a test. ")]),
        ("A&emsp14;&emsp14;&emsp14;B", vec![span("A   B")]),
    ];

    let indented_tests = vec![
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

    fn check_parse_and_write(input: &str, expected_doc: &Document, expected_output: &str) {
        let parsed_doc = parse(Cursor::new(&format!("<p>{}</p>\n", input))).unwrap();
        assert_eq!(parsed_doc, *expected_doc, "Parsing failed for: {}", input);

        let mut buf = Vec::new();
        write(&mut buf, expected_doc).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(output, expected_output, "Writing failed for: {}", input);
    }

    for (input, spans) in simple_tests {
        let expected_output = format!("<p>{}</p>\n", input);
        let expected_doc = doc(vec![Paragraph::new_text().with_content(spans)]);
        check_parse_and_write(input, &expected_doc, &expected_output);
    }

    for (input, spans) in indented_tests {
        let expected_output = format!("<p>{}</p>\n", input);
        let expected_doc = doc(vec![Paragraph::new_text().with_content(spans)]);
        check_parse_and_write(input, &expected_doc, &expected_output);
    }
}

#[test]
fn test_parsing_inline_styles() {
    let tests = vec![
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

    for (input, expected) in tests {
        let full_input = format!("<p>{}</p>", input);
        let doc = parse(Cursor::new(full_input)).unwrap();
        assert_eq!(doc.paragraphs[0].content, expected, "Failed for: {}", input);
    }
}

#[test]
fn test_write_spaces() {
    let tests = vec![
        (
            "<p>&emsp14;This is a test.</p>\n",
            vec![span(" This is a test.")],
        ),
        (
            "<p>This is a test.&emsp14;</p>\n",
            vec![span("This is a test. ")],
        ),
        ("<p>A&emsp14;&emsp14;&emsp14;B</p>\n", vec![span("A   B")]),
    ];

    for (expected_output, spans) in tests {
        let doc = doc(vec![p_(spans)]);
        let mut buf = Vec::new();
        write(&mut buf, &doc).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(output, expected_output);
    }
}

#[test]
fn test_simple_paragraph_roundtrip() {
    let input = "<p>This is a test.</p>";
    let doc = parse(Cursor::new(input)).unwrap();
    let mut buf = Vec::new();
    write(&mut buf, &doc).unwrap();
    let output = String::from_utf8(buf).unwrap();
    assert_eq!(output, format!("{}\n", input));
}

#[test]
fn test_paragraph_with_styles_roundtrip() {
    let input = "<p>This is <i>a little more complex</i> test.</p>";
    let doc = parse(Cursor::new(input)).unwrap();
    let mut buf = Vec::new();
    write(&mut buf, &doc).unwrap();
    let output = String::from_utf8(buf).unwrap();
    assert_eq!(output, format!("{}\n", input));
}

#[test]
fn test_header_roundtrip() {
    let input = "<h1>Header</h1>";
    let doc = parse(Cursor::new(input)).unwrap();
    let mut buf = Vec::new();
    write(&mut buf, &doc).unwrap();
    let output = String::from_utf8(buf).unwrap();
    assert_eq!(output, format!("{}\n", input));
}

#[test]
fn test_nested_list() {
    let input = r#"<ul>
<li>
<ul>
<li>
<p>a</p>
</li>
</ul>
</li>
<li>
<p>b</p>
<p>c</p>
</li>
</ul>"#;

    let expected = doc(vec![ul_(vec![
        li_(vec![ul_(vec![li_(vec![p__("a")])])]),
        li_(vec![p__("b"), p__("c")]),
    ])]);

    let result = parse(Cursor::new(input)).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_parsing_errors() {
    let tests = vec![
        ("This is a test.", "Unexpected text content"),
        ("<p>one<p>two</p></p>", "Closing unopened paragraph"),
        ("<blockquote>one</blockquote>", "Unexpected text content"),
    ];

    for (input, expected_error) in tests {
        let result = parse(Cursor::new(input));
        assert!(result.is_err(), "Expected error for input: {}", input);
        let error = result.unwrap_err();
        assert!(
            error
                .to_string()
                .to_lowercase()
                .contains(&expected_error.to_lowercase()),
            "Error '{}' should contain '{}' for input: {}",
            error,
            expected_error,
            input
        );
    }
}

#[test]
fn test_trim_whitespace() {
    // Test the whitespace trimming functionality
    let input = "<p>  \n  test, test.  \n  </p>";
    let doc = parse(Cursor::new(input)).unwrap();

    // Should trim leading/trailing whitespace
    assert_eq!(doc.paragraphs[0].content[0].text, "test, test.");
}

#[test]
fn test_parsing_hard_newlines() {
    let input = r#"<p>
    This is a paragraph that contains a very long line of <b>highlighted text
    to force the formatter to break<br />
    the<br />
    line<br />
	in the middle.</b> But afterwards, of course, things should continue
    normally.
  </p>"#;

    let parsed_doc = parse(Cursor::new(input)).unwrap();
    let expected = doc(vec![p_(vec![
        span("This is a paragraph that contains a very long line of "),
        b_(vec![
            span("highlighted text to force the formatter to break\n"),
            span("the\n"),
            span("line\n"),
            span("in the middle."),
        ]),
        span(" But afterwards, of course, things should continue normally."),
    ])]);

    assert_eq!(parsed_doc, expected);
}

#[test]
fn test_all_inline_styles() {
    let tests = vec![
        ("<p><b>bold</b></p>", InlineStyle::Bold),
        ("<p><i>italic</i></p>", InlineStyle::Italic),
        ("<p><u>underline</u></p>", InlineStyle::Underline),
        ("<p><s>strike</s></p>", InlineStyle::Strike),
        ("<p><mark>highlight</mark></p>", InlineStyle::Highlight),
        ("<p><code>code</code></p>", InlineStyle::Code),
    ];

    for (input, expected_style) in tests {
        let doc = parse(Cursor::new(input)).unwrap();
        assert_eq!(doc.paragraphs[0].content[0].style, expected_style);
    }
}

#[test]
fn test_all_paragraph_types() {
    let tests = vec![
        ("<p>text</p>", ParagraphType::Text),
        ("<h1>header1</h1>", ParagraphType::Header1),
        ("<h2>header2</h2>", ParagraphType::Header2),
        ("<h3>header3</h3>", ParagraphType::Header3),
        (
            "<blockquote><p>quote</p></blockquote>",
            ParagraphType::Quote,
        ),
        (
            "<ul><li><p>item</p></li></ul>",
            ParagraphType::UnorderedList,
        ),
        ("<ol><li><p>item</p></li></ol>", ParagraphType::OrderedList),
    ];

    for (input, expected_type) in tests {
        let doc = parse(Cursor::new(input)).unwrap();
        assert_eq!(doc.paragraphs[0].paragraph_type, expected_type);
    }
}

#[test]
fn test_list_item_parsing() {
    // Test case specifically for the testdocument.ftml parsing error
    // This should reproduce the "Non-inline token: l" error before the fix
    let input = r#"<ul>
  <li>
    <p>Test item</p>
  </li>
</ul>"#;

    let parsed_doc = parse(Cursor::new(input)).unwrap();
    let expected = doc(vec![ul_(vec![li_(vec![p__("Test item")])])]);

    assert_eq!(parsed_doc, expected);
}

#[test]
fn test_complex_list_structure() {
    // Test case for complex list structures like in testdocument.ftml
    let input = r#"<ul>
  <li>
    <p>First item</p>
  </li>
  <li>
    <p>Second item with multiple paragraphs</p>
    <p>This is the second paragraph of the same list item</p>
  </li>
  <li>
    <blockquote>
      <p>A list item containing a quote</p>
    </blockquote>
  </li>
</ul>"#;

    let parsed_doc = parse(Cursor::new(input)).unwrap();
    let expected = doc(vec![ul_(vec![
        li_(vec![p__("First item")]),
        li_(vec![
            p__("Second item with multiple paragraphs"),
            p__("This is the second paragraph of the same list item"),
        ]),
        li_(vec![quote_(vec![p__("A list item containing a quote")])]),
    ])]);

    assert_eq!(parsed_doc, expected);
}

#[test]
fn test_testdocument_specific_structure() {
    // Test a specific structure from testdocument.ftml that might be causing the issue
    let input = r#"<ul>
  <li>
    <p><b>Bold</b> text.</p>
  </li>
</ul>"#;

    // This should reproduce the actual error if it's related to inline elements within list items
    let _parsed_doc = parse(Cursor::new(input)).unwrap();
}

#[test]
fn test_testdocument_exact_section() {
    // Test the exact structure from testdocument.ftml that should be failing
    let input = r#"<ul>
  <li>
    <p><b>Bold</b> text.</p>
  </li>

  <li>
    <p><i>Italic</i> text.</p>
  </li>

  <li>
    <p><mark>Highlighted</mark> text.</p>
  </li>

  <li>
    <p><u>Underlined</u> text.</p>
  </li>

  <li>
    <p><s>Striked</s> text.</p>
  </li>

  <li>
    <p>Text formatted as <code>code</code>.</p>
  </li>
</ul>"#;

    let _parsed_doc = parse(Cursor::new(input)).unwrap();
}

#[test]
fn test_nested_lists_structure() {
    // This is the actual problematic structure from testdocument.ftml
    // Nested list inside list item
    let input = r#"<ul>
    <li>
      <ol>
        <li>
          <p>One</p>
        </li>
      </ol>
    </li>
</ul>"#;

    let _parsed_doc = parse(Cursor::new(input));
    // This should fail with "Non-inline token: l" error
}
