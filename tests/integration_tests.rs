use std::io::Cursor;
use tdoc::{
    formatter::Formatter, ftml, markdown, parse, write, Document, InlineStyle, ParagraphType,
};

#[test]
fn test_parsing_simple_paragraphs() {
    let tests: Vec<(&str, Document)> = vec![
        ("<p>This is a test.</p>", ftml! { p { "This is a test." } }),
        ("<p>one</p><p>two</p>", ftml! { p { "one" } p { "two" } }),
        (
            "<blockquote><p>one</p></blockquote><p>two</p>",
            ftml! { quote { p { "one" } } p { "two" } },
        ),
        (
            "<p><b>Bold</b> text.</p>",
            ftml! { p { b { "Bold" } " text." } },
        ),
        (
            "<p><b>Bold<br />text.</b></p>",
            ftml! { p { b { "Bold\n" "text." } } },
        ),
        (
            "<p>Test <b>bold</b></p>",
            ftml! { p { "Test " b { "bold" } } },
        ),
        ("<h1> Hello World! </h1>", ftml! { h1 { "Hello World!" } }),
        ("<p>A<br/> B</p>", ftml! { p { "A" "\n" "B" } }),
        (
            "<ul><li><p>a</p></li><li><p>b</p></li></ul>",
            ftml! { ul { li { p { "a" } } li { p { "b" } } } },
        ),
        (
            "<ul><li><p>a</p></li><li><p>b</p><p>c</p></li></ul>",
            ftml! { ul { li { p { "a" } } li { p { "b" } p { "c" } } } },
        ),
    ];

    for (input, expected) in tests {
        let result = parse(Cursor::new(input)).unwrap();
        assert_eq!(result, expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_parsing_and_writing_styles() {
    let simple_tests: Vec<(&str, Document)> = vec![
        ("This is a test.", ftml! { p { "This is a test." } }),
        (
            "&emsp14;This is a test.",
            ftml! { p { " This is a test." } },
        ),
        (
            "This is a test.&emsp14;",
            ftml! { p { "This is a test. " } },
        ),
        ("A&emsp14;&emsp14;&emsp14;B", ftml! { p { "A   B" } }),
    ];

    let indented_tests: Vec<(&str, Document)> = vec![
        (
            "This is a <b>test</b>.",
            ftml! { p { "This is a " b { "test" } "." } },
        ),
        (
            "This is a <b> test </b>.",
            ftml! { p { "This is a " b { " test " } "." } },
        ),
        (
            "This is a <b><i>second</i> test</b>.",
            ftml! { p { "This is a " b { i { "second" } " test" } "." } },
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

    for (input, expected_doc) in simple_tests {
        let expected_output = format!("<p>{}</p>\n", input);
        check_parse_and_write(input, &expected_doc, &expected_output);
    }

    for (input, expected_doc) in indented_tests {
        let expected_output = format!("<p>{}</p>\n", input);
        check_parse_and_write(input, &expected_doc, &expected_output);
    }
}

#[test]
fn test_parsing_inline_styles() {
    let tests: Vec<(&str, Document)> = vec![
        (
            "This is a <b>test</b>.",
            ftml! { p { "This is a " b { "test" } "." } },
        ),
        (
            "This is a <b> test </b>.",
            ftml! { p { "This is a " b { " test " } "." } },
        ),
        (
            "This is a <b><i>second</i> test</b>.",
            ftml! { p { "This is a " b { i { "second" } " test" } "." } },
        ),
        (
            "Visit <a href=\"https://example.com\">Example</a>",
            ftml! { p { "Visit " link { "https://example.com" "Example" } } },
        ),
    ];

    for (input, expected) in tests {
        let full_input = format!("<p>{}</p>", input);
        let doc = parse(Cursor::new(full_input)).unwrap();
        assert_eq!(doc, expected, "Failed for: {}", input);
    }
}

#[test]
fn test_write_spaces() {
    let tests: Vec<(&str, Document)> = vec![
        (
            "<p>&emsp14;This is a test.</p>\n",
            ftml! { p { " This is a test." } },
        ),
        (
            "<p>This is a test.&emsp14;</p>\n",
            ftml! { p { "This is a test. " } },
        ),
        (
            "<p>A&emsp14;&emsp14;&emsp14;B</p>\n",
            ftml! { p { "A   B" } },
        ),
    ];

    for (expected_output, expected_doc) in tests {
        let mut buf = Vec::new();
        write(&mut buf, &expected_doc).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert_eq!(output, expected_output);
    }
}

#[test]
fn test_code_block_html_roundtrip() {
    let input = "<pre>fn main() {}\nprintln!(\"hi\");</pre>";
    let doc = parse(Cursor::new(input)).unwrap();
    let expected = ftml! { code { "fn main() {}\nprintln!(\"hi\");" } };
    assert_eq!(doc, expected);

    let mut buf = Vec::new();
    write(&mut buf, &doc).unwrap();
    let output = String::from_utf8(buf).unwrap();
    assert_eq!(output, "<pre>\nfn main() {}\nprintln!(\"hi\");\n</pre>\n");
}

#[test]
fn test_code_block_html_roundtrip_with_leading_newline() {
    let input = "<pre>\nfn main() {}\nprintln!(\"hi\");\n</pre>\n";
    let doc = parse(Cursor::new(input)).unwrap();

    let mut buf = Vec::new();
    write(&mut buf, &doc).unwrap();
    let output = String::from_utf8(buf).unwrap();

    assert_eq!(output, input);
}

#[test]
fn test_formatter_code_block_ascii() {
    let doc = ftml! { code { "fn main() {}\nprintln!(\"hi\");" } };
    let mut output = Vec::new();
    let mut formatter = Formatter::new_ascii(&mut output);
    let fence = "-".repeat(formatter.style.wrap_width);
    formatter.write_document(&doc).unwrap();
    let rendered = String::from_utf8(output).unwrap();
    let body = "fn main() {}\nprintln!(\"hi\");";
    let expected = format!("{fence}\n{body}\n{fence}\n", fence = fence, body = body);
    assert_eq!(rendered, expected);
}

#[test]
fn test_formatter_code_block_wrapping() {
    let doc = ftml! { code { "abcdefghijk" } };
    let mut output = Vec::new();
    let mut formatter = Formatter::new_ascii(&mut output);
    formatter.style.wrap_width = 8;
    let fence = "-".repeat(formatter.style.wrap_width);
    formatter.write_document(&doc).unwrap();
    let rendered = String::from_utf8(output).unwrap();
    let expected = format!("{fence}\nabcdefgh\nijk\n{fence}\n", fence = fence);
    assert_eq!(rendered, expected);
}

#[test]
fn test_markdown_code_block_roundtrip() {
    let input = "```\nfn main() {}\nprintln!(\"hi\");\n```\n";
    let doc = markdown::parse(Cursor::new(input)).unwrap();
    let expected = ftml! { code { "fn main() {}\nprintln!(\"hi\");\n" } };
    assert_eq!(doc, expected);

    let mut output = Vec::new();
    markdown::write(&mut output, &doc).unwrap();
    let markdown_out = String::from_utf8(output).unwrap();
    assert_eq!(
        markdown_out,
        "```\nfn main() {}\nprintln!(\"hi\");\n```\n\n"
    );
}

#[test]
fn test_markdown_code_block_trims_leading_newline() {
    let input = "<pre>\nfn main() {}\nprintln!(\"hi\");\n</pre>\n";
    let doc = parse(Cursor::new(input)).unwrap();

    let mut output = Vec::new();
    markdown::write(&mut output, &doc).unwrap();
    let markdown_out = String::from_utf8(output).unwrap();

    assert_eq!(markdown_out, "```\nfn main() {}\nprintln!(\"hi\");\n```\n\n");
}

#[test]
fn test_markdown_code_block_preserves_blank_first_line() {
    let input = "<pre>\n\nfn main() {}\nprintln!(\"hi\");\n</pre>\n";
    let doc = parse(Cursor::new(input)).unwrap();

    let mut output = Vec::new();
    markdown::write(&mut output, &doc).unwrap();
    let markdown_out = String::from_utf8(output).unwrap();

    assert_eq!(
        markdown_out,
        "```\n\nfn main() {}\nprintln!(\"hi\");\n```\n\n"
    );
}

#[test]
fn test_parse_code_block_trims_leading_newline() {
    let input = "<pre>\nfn main() {}\n</pre>";
    let doc = parse(Cursor::new(input)).unwrap();
    let expected = ftml! { code { "fn main() {}\n" } };
    assert_eq!(doc, expected);
}

#[test]
fn test_parse_code_block_preserves_blank_first_line() {
    let input = "<pre>\n\nfn main() {}\n</pre>";
    let doc = parse(Cursor::new(input)).unwrap();
    let expected = ftml! { code { "\nfn main() {}\n" } };
    assert_eq!(doc, expected);
}

#[test]
fn test_markdown_soft_break_collapses() {
    let input = "Hello\nworld\nagain\n";
    let parsed = markdown::parse(Cursor::new(input)).unwrap();
    let expected = ftml! { p { "Hello world again" } };
    assert_eq!(parsed, expected);
}

#[test]
fn test_markdown_hard_break_preserved() {
    let input = "Hello  \nworld\n";
    let parsed = markdown::parse(Cursor::new(input)).unwrap();
    let expected = ftml! { p { "Hello" "\n" "world" } };
    assert_eq!(parsed, expected);
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
fn test_html_checklist_roundtrip() {
    let input = r#"<ul>
  <li><input type="checkbox" checked /> This one is done</li>
  <li><input type="checkbox" /> This one is not done</li>
  <li>
    <input type="checkbox" /> This one is not done and also contains a very
    large amount of text that will wrap onto multiple lines in the terminal
    output.
  </li>
</ul>"#;

    let expected_doc = ftml! {
        checklist {
            done { "This one is done" }
            todo { "This one is not done" }
            todo { "This one is not done and also contains a very large amount of text that will wrap onto multiple lines in the terminal output." }
        }
    };

    let parsed = parse(Cursor::new(input)).unwrap();
    assert_eq!(parsed, expected_doc);

    let mut buf = Vec::new();
    write(&mut buf, &expected_doc).unwrap();
    let output = String::from_utf8(buf).unwrap();
    let expected_output = r#"<ul>
  <li><input type="checkbox" checked /> This one is done</li>
  <li><input type="checkbox" /> This one is not done</li>
  <li>
    <input type="checkbox" /> This one is not done and also contains a very large amount of text that
    will wrap onto multiple lines in the terminal output.
  </li>
</ul>
"#;
    assert_eq!(output, expected_output);
}

#[test]
fn test_markdown_checklist_roundtrip() {
    let input = "- [x] First item\n- [ ] Second item\n";
    let expected_doc = ftml! {
        checklist {
            done { "First item" }
            todo { "Second item" }
        }
    };

    let parsed = markdown::parse(Cursor::new(input)).unwrap();
    assert_eq!(parsed, expected_doc);

    let mut buf = Vec::new();
    markdown::write(&mut buf, &expected_doc).unwrap();
    let output = String::from_utf8(buf).unwrap();
    assert_eq!(output, input);
}

#[test]
fn test_html_checklist_with_bold_text() {
    let input = r#"<ul>
  <li><input type="checkbox" checked /> This one has <b>bold</b> text</li>
  <li><input type="checkbox" /> This one has <i>italic</i> text</li>
</ul>
"#;

    let expected_doc = ftml! {
        checklist {
            done { "This one has " b { "bold" } " text" }
            todo { "This one has " i { "italic" } " text" }
        }
    };

    let parsed = parse(Cursor::new(input)).unwrap();
    assert_eq!(parsed, expected_doc);

    let mut buf = Vec::new();
    write(&mut buf, &expected_doc).unwrap();
    let output = String::from_utf8(buf).unwrap();
    assert_eq!(output, input);
}

#[test]
fn test_markdown_wikilink_roundtrip() {
    let input = "[[WikiLink]]";
    let parsed = markdown::parse(Cursor::new(input)).unwrap();

    let expected_doc = ftml! {
        p { link { "WikiLink" "WikiLink" } }
    };

    assert_eq!(parsed, expected_doc);

    let mut buf = Vec::new();
    markdown::write(&mut buf, &expected_doc).unwrap();
    let output = String::from_utf8(buf).unwrap();
    assert_eq!(output, "[WikiLink](WikiLink)\n");
}

#[test]
fn test_markdown_wikilink_with_label_roundtrip() {
    let input = "[[WikiLink|Custom label]]";
    let parsed = markdown::parse(Cursor::new(input)).unwrap();

    let expected_doc = ftml! {
        p { link { "WikiLink" "Custom label" } }
    };

    assert_eq!(parsed, expected_doc);

    let mut buf = Vec::new();
    markdown::write(&mut buf, &expected_doc).unwrap();
    let output = String::from_utf8(buf).unwrap();
    assert_eq!(output, "[Custom label](WikiLink)\n");
}

#[test]
fn test_markdown_nested_unordered_lists() {
    let input = "- top level\n  - second level\n";
    let parsed = markdown::parse(Cursor::new(input)).unwrap();
    let expected = ftml! {
        ul {
            li {
                p { "top level" }
                ul {
                    li { p { "second level" } }
                }
            }
        }
    };

    assert_eq!(parsed, expected);
}

#[test]
fn test_markdown_nested_mixed_lists() {
    let input = "1. ordered item\n   - unordered child\n     - [x] nested task\n";
    let parsed = markdown::parse(Cursor::new(input)).unwrap();
    let expected = ftml! {
        ol {
            li {
                p { "ordered item" }
                ul {
                    li {
                        p { "unordered child" }
                        checklist {
                            done { "nested task" }
                        }
                    }
                }
            }
        }
    };

    assert_eq!(parsed, expected);
}

#[test]
fn test_markdown_nested_lists_in_blockquote() {
    let input = "> - quoted bullet\n>   1. quoted number\n>      - [ ] quoted task\n";
    let parsed = markdown::parse(Cursor::new(input)).unwrap();
    let expected = ftml! {
        quote {
            ul {
                li {
                    p { "quoted bullet" }
                    ol {
                        li {
                            p { "quoted number" }
                            checklist {
                                todo { "quoted task" }
                            }
                        }
                    }
                }
            }
        }
    };

    assert_eq!(parsed, expected);
}

#[test]
fn test_markdown_ignores_html_comments() {
    let input = "Before\n\n<!-- comment line 1\nline 2\n-->\n\nAfter\n";
    let parsed = markdown::parse(Cursor::new(input)).unwrap();
    let expected = ftml! {
        p { "Before" }
        p { "After" }
    };

    assert_eq!(parsed, expected);
}

#[test]
fn test_formatter_checklist_output() {
    let doc = ftml! {
        checklist {
            done { "This one is done" }
            todo { "This one is not done" }
            todo { "This one is not done and also contains a very large amount of text that will wrap onto multiple lines in the terminal output." }
        }
    };

    let mut buf = Vec::new();
    Formatter::new_ascii(&mut buf).write_document(&doc).unwrap();
    let output = String::from_utf8(buf).unwrap();
    let expected = "[✓] This one is done\n[ ] This one is not done\n[ ] This one is not done and also contains a very large amount of text\n    that will wrap onto multiple lines in the terminal output.\n";
    assert_eq!(output, expected);
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

    let expected = ftml! {
        ul {
            li { ul { li { p { "a" } } } }
            li { p { "b" } p { "c" } }
        }
    };

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
    let expected = ftml! {
        p {
            "This is a paragraph that contains a very long line of "
            b {
                "highlighted text to force the formatter to break\n"
                "the\n"
                "line\n"
                "in the middle."
            }
            " But afterwards, of course, things should continue normally."
        }
    };

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
    let expected = ftml! {
        ul {
            li { p { "Test item" } }
        }
    };

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
    let expected = ftml! {
        ul {
            li { p { "First item" } }
            li {
                p { "Second item with multiple paragraphs" }
                p { "This is the second paragraph of the same list item" }
            }
            li {
                quote { p { "A list item containing a quote" } }
            }
        }
    };

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
