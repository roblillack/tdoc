use tdoc::ftml;
use tdoc::test_helpers::{
    b__, code__, doc as doc_, h1_, i__, li_, link_, link__, link_text__, mark__, ol_, p_, p__,
    quote_, s__, span, u__, ul_,
};

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
