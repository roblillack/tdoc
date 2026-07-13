use tdoc::test_helpers::{
    b__, code__, code_block__, doc as doc_, h1_, i__, li_, link_, link__, link_text__, mark__, ol_,
    p_, p__, quote_, s__, span, u__, ul_,
};
use tdoc::{doc, ftml, Paragraph, Span, TableCell, TableRow};

#[test]
fn builds_document_trees() {
    let doc = ftml! {
        h1 { "Hello World!" }
        ul {
            li {
                p { "This is a text paragraph inside a list item" }
                quote { p { "And this is a quoted paragraph in the same item" } }
            }
        }
        p { "Inline styles work " b { "just as well" } "." }
    };

    let expected = doc_(vec![
        h1_("Hello World!"),
        ul_(vec![li_(vec![
            p__("This is a text paragraph inside a list item"),
            quote_(vec![p__("And this is a quoted paragraph in the same item")]),
        ])]),
        p_(vec![
            span("Inline styles work "),
            b__("just as well"),
            span("."),
        ]),
    ]);

    assert_eq!(doc, expected);
}

#[test]
fn supports_inline_styles_and_lists() {
    let doc = ftml! {
        p {
            "Plain "
            b { "bold" }
            " and "
            i { "italic" }
            " plus "
            u { "underline" }
            " and "
            del { "deleted" }
            " with "
            mark { "highlight" }
            " and "
            code { "inline code" }
        }
        ol {
            li { p { "First" } }
            li {
                p { "Second" }
                ul {
                    li { p { "Nested" } }
                }
            }
        }
    };

    let expected = doc_(vec![
        p_(vec![
            span("Plain "),
            b__("bold"),
            span(" and "),
            i__("italic"),
            span(" plus "),
            u__("underline"),
            span(" and "),
            s__("deleted"),
            span(" with "),
            mark__("highlight"),
            span(" and "),
            code__("inline code"),
        ]),
        ol_(vec![
            li_(vec![p__("First")]),
            li_(vec![p__("Second"), ul_(vec![li_(vec![p__("Nested")])])]),
        ]),
    ]);

    assert_eq!(doc, expected);
}

#[test]
fn supports_links() {
    let doc = ftml! {
        p { "Visit ", link { "https://example.org" } }
        p { link { "https://example.org/docs" "Docs" } }
        p { link { "https://example.org/mixed" "Mixed ", b { "Bold" } } }
    };

    let expected = doc_(vec![
        p_(vec![span("Visit "), link__("https://example.org")]),
        p_(vec![link_text__("https://example.org/docs", "Docs")]),
        p_(vec![link_(
            "https://example.org/mixed",
            vec![span("Mixed "), b__("Bold")],
        )]),
    ]);

    assert_eq!(doc, expected);
}

#[test]
fn supports_code_blocks() {
    let doc = ftml! {
        code { "fn main() {}\nprintln!(\"hi\");" }
    };

    let expected = doc_(vec![code_block__("fn main() {}\nprintln!(\"hi\");")]);

    assert_eq!(doc, expected);
}

#[test]
fn doc_supports_horizontal_rules() {
    // Horizontal rules are a `doc!` extension (not strict FTML), like tables.
    let document = doc! {
        p { "Above" }
        hr {}
        p { "Below" }
    };

    let expected = doc_(vec![
        p__("Above"),
        Paragraph::new_horizontal_rule(),
        p__("Below"),
    ]);

    assert_eq!(document, expected);
}

#[test]
fn doc_accepts_the_same_syntax_as_ftml() {
    // `doc!` is a superset of `ftml!`, so any strict-FTML document builds
    // identically with either macro.
    let via_ftml = ftml! {
        h1 { "Hello World!" }
        p { "Inline styles work " b { "just as well" } "." }
        ul { li { p { "Item" } } }
    };
    let via_doc = doc! {
        h1 { "Hello World!" }
        p { "Inline styles work " b { "just as well" } "." }
        ul { li { p { "Item" } } }
    };

    assert_eq!(via_ftml, via_doc);
}

#[test]
fn doc_supports_tables() {
    let document = doc! {
        h1 { "Report" }
        table {
            row { th { "Name" } th { "Score" } }
            row { td { "Alice" } td { "42" } }
        }
    };

    let header = TableRow::new().with_cells(vec![
        TableCell::new_header().with_content(vec![Span::new_text("Name")]),
        TableCell::new_header().with_content(vec![Span::new_text("Score")]),
    ]);
    let body = TableRow::new().with_cells(vec![
        TableCell::new_data().with_content(vec![Span::new_text("Alice")]),
        TableCell::new_data().with_content(vec![Span::new_text("42")]),
    ]);
    let expected = doc_(vec![
        h1_("Report"),
        Paragraph::new_table().with_rows(vec![header, body]),
    ]);

    assert_eq!(document, expected);
}

#[test]
fn doc_supports_tables_nested_in_other_blocks() {
    // The `mode` token must thread through nested block contexts so extensions
    // are available inside lists, quotes, etc. — not just at the top level.
    let document = doc! {
        ul {
            li {
                table {
                    row { td { "cell" } }
                }
            }
        }
    };

    let table = Paragraph::new_table().with_rows(vec![TableRow::new().with_cells(vec![
        TableCell::new_data().with_content(vec![Span::new_text("cell")]),
    ])]);
    let expected = doc_(vec![ul_(vec![li_(vec![table])])]);

    assert_eq!(document, expected);
}
