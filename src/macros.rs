//! Macros that make it ergonomic to build FTML documents in tests or examples.

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __ftml_collect_blocks {
    () => {
        ::std::vec::Vec::<$crate::Paragraph>::new()
    };
    ($($tt:tt)*) => {{
        let mut __blocks: ::std::vec::Vec<$crate::Paragraph> = ::std::vec::Vec::new();
        __ftml_collect_blocks_inner!(__blocks, $($tt)*);
        __blocks
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __ftml_collect_blocks_inner {
    ($vec:ident,) => {};
    ($vec:ident) => {};
    ($vec:ident, , $($rest:tt)*) => {
        __ftml_collect_blocks_inner!($vec, $($rest)*);
    };
    ($vec:ident, $tag:ident { $($inner:tt)* } $($rest:tt)*) => {{
        $vec.push(__ftml_build_block!($tag { $($inner)* }));
        __ftml_collect_blocks_inner!($vec, $($rest)*);
    }};
    ($vec:ident, $unexpected:tt $($rest:tt)*) => {{
        compile_error!(concat!(
            "Unexpected token in FTML block context: ",
            stringify!($unexpected)
        ));
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __ftml_build_block {
    (p { $($inner:tt)* }) => {{
        $crate::Paragraph::new_text().with_content(__ftml_inline_nodes!($($inner)*))
    }};
    (h1 { $($inner:tt)* }) => {{
        $crate::Paragraph::new_header1().with_content(__ftml_inline_nodes!($($inner)*))
    }};
    (h2 { $($inner:tt)* }) => {{
        $crate::Paragraph::new_header2().with_content(__ftml_inline_nodes!($($inner)*))
    }};
    (h3 { $($inner:tt)* }) => {{
        $crate::Paragraph::new_header3().with_content(__ftml_inline_nodes!($($inner)*))
    }};
    (quote { $($inner:tt)* }) => {{
        $crate::Paragraph::new_quote().with_children(__ftml_collect_blocks!($($inner)*))
    }};
    (ul { $($inner:tt)* }) => {{
        $crate::Paragraph::new_unordered_list().with_entries(__ftml_list_entries!($($inner)*))
    }};
    (ol { $($inner:tt)* }) => {{
        $crate::Paragraph::new_ordered_list().with_entries(__ftml_list_entries!($($inner)*))
    }};
    ($other:ident { $($inner:tt)* }) => {{
        compile_error!(concat!("Unknown FTML element: ", stringify!($other)));
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __ftml_inline_nodes {
    () => {
        ::std::vec::Vec::<$crate::Span>::new()
    };
    ($($tt:tt)*) => {{
        let mut __spans: ::std::vec::Vec<$crate::Span> = ::std::vec::Vec::new();
        __ftml_inline_nodes_inner!(__spans, $($tt)*);
        __spans
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __ftml_inline_nodes_inner {
    ($vec:ident,) => {};
    ($vec:ident) => {};
    ($vec:ident, , $($rest:tt)*) => {
        __ftml_inline_nodes_inner!($vec, $($rest)*);
    };
    ($vec:ident, $tag:ident { $($inner:tt)* } $($rest:tt)*) => {{
        $vec.push(__ftml_build_inline!($tag { $($inner)* }));
        __ftml_inline_nodes_inner!($vec, $($rest)*);
    }};
    ($vec:ident, $lit:literal $($rest:tt)*) => {{
        $vec.push($crate::Span::new_text($lit));
        __ftml_inline_nodes_inner!($vec, $($rest)*);
    }};
    ($vec:ident, ($($expr:tt)*) $($rest:tt)*) => {{
        $vec.push($crate::Span::new_text(($($expr)*)));
        __ftml_inline_nodes_inner!($vec, $($rest)*);
    }};
    ($vec:ident, $ident:ident $($rest:tt)*) => {{
        $vec.push($crate::Span::new_text($ident));
        __ftml_inline_nodes_inner!($vec, $($rest)*);
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __ftml_build_inline {
    (b { $($inner:tt)* }) => {{
        $crate::Span::new_styled($crate::InlineStyle::Bold)
            .with_children(__ftml_inline_nodes!($($inner)*))
    }};
    (i { $($inner:tt)* }) => {{
        $crate::Span::new_styled($crate::InlineStyle::Italic)
            .with_children(__ftml_inline_nodes!($($inner)*))
    }};
    (u { $($inner:tt)* }) => {{
        $crate::Span::new_styled($crate::InlineStyle::Underline)
            .with_children(__ftml_inline_nodes!($($inner)*))
    }};
    (del { $($inner:tt)* }) => {{
        $crate::Span::new_styled($crate::InlineStyle::Strike)
            .with_children(__ftml_inline_nodes!($($inner)*))
    }};
    (mark { $($inner:tt)* }) => {{
        $crate::Span::new_styled($crate::InlineStyle::Highlight)
            .with_children(__ftml_inline_nodes!($($inner)*))
    }};
    (code { $($inner:tt)* }) => {{
        $crate::Span::new_styled($crate::InlineStyle::Code)
            .with_children(__ftml_inline_nodes!($($inner)*))
    }};
    ($other:ident { $($inner:tt)* }) => {{
        compile_error!(concat!("Unknown FTML inline element: ", stringify!($other)));
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __ftml_list_entries {
    () => {
        ::std::vec::Vec::<::std::vec::Vec<$crate::Paragraph>>::new()
    };
    ($($tt:tt)*) => {{
        let mut __entries: ::std::vec::Vec<::std::vec::Vec<$crate::Paragraph>> =
            ::std::vec::Vec::new();
        __ftml_list_entries_inner!(__entries, $($tt)*);
        __entries
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __ftml_list_entries_inner {
    ($vec:ident,) => {};
    ($vec:ident) => {};
    ($vec:ident, , $($rest:tt)*) => {
        __ftml_list_entries_inner!($vec, $($rest)*);
    };
    ($vec:ident, li { $($inner:tt)* } $($rest:tt)*) => {{
        $vec.push(__ftml_collect_blocks!($($inner)*));
        __ftml_list_entries_inner!($vec, $($rest)*);
    }};
    ($vec:ident, $other:ident { $($inner:tt)* } $($rest:tt)*) => {{
        compile_error!(concat!("Expected `li` inside list, found `", stringify!($other), "`"));
    }};
    ($vec:ident, $unexpected:tt $($rest:tt)*) => {{
        compile_error!(concat!(
            "Unexpected token inside list: ",
            stringify!($unexpected)
        ));
    }};
}

#[macro_export(local_inner_macros)]
/// Builds a [`Document`](crate::Document) using an inline FTML DSL.
///
/// The macro accepts block-level tags such as `p`, `h1`, `quote`, `ul`, and
/// `ol`. Inline runs can contain string literals or inline tags like `b`, `i`,
/// `mark`, or `code`.
///
/// # Examples
///
/// ```
/// use tdoc::{ftml, ParagraphType};
///
/// let doc = ftml! {
///     h1 { "Heading" }
///     p  { "Hello, ", b { "world" }, "!" }
/// };
///
/// assert_eq!(doc.paragraphs[0].paragraph_type, ParagraphType::Header1);
/// assert_eq!(doc.paragraphs[1].paragraph_type, ParagraphType::Text);
/// ```
macro_rules! ftml {
    ($($tt:tt)*) => {{
        let __paragraphs = __ftml_collect_blocks!($($tt)*);
        $crate::Document::new().with_paragraphs(__paragraphs)
    }};
}
