//! Macros that make it ergonomic to build documents in tests or examples.
//!
//! Two entry points share the same DSL:
//!
//! - [`ftml!`](macro@crate::ftml) accepts only elements expressible in strict FTML.
//! - [`doc!`](macro@crate::doc) is a superset that also accepts tdoc's extensions
//!   (currently `table`), and is where future, non-FTML features are added.
//!
//! Both expand to the same [`Document`](crate::Document) tree; the only
//! difference is which elements they accept. The shared machinery threads a
//! `mode` token (`ftml` or `doc`) through the recursive helpers so the strict
//! macro can reject extended elements with a helpful message.

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_collect_blocks {
    ($mode:ident;) => {
        ::std::vec::Vec::<$crate::Paragraph>::new()
    };
    ($mode:ident; $($tt:tt)*) => {{
        let mut __blocks: ::std::vec::Vec<$crate::Paragraph> = ::std::vec::Vec::new();
        __tdoc_collect_blocks_inner!($mode, __blocks, $($tt)*);
        __blocks
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_collect_blocks_inner {
    ($mode:ident, $vec:ident,) => {};
    ($mode:ident, $vec:ident) => {};
    ($mode:ident, $vec:ident, , $($rest:tt)*) => {
        __tdoc_collect_blocks_inner!($mode, $vec, $($rest)*);
    };
    ($mode:ident, $vec:ident, $tag:ident { $($inner:tt)* } $($rest:tt)*) => {{
        $vec.push(__tdoc_build_block!($mode, $tag { $($inner)* }));
        __tdoc_collect_blocks_inner!($mode, $vec, $($rest)*);
    }};
    ($mode:ident, $vec:ident, $unexpected:tt $($rest:tt)*) => {{
        compile_error!(concat!(
            "Unexpected token in document block context: ",
            stringify!($unexpected)
        ));
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_build_block {
    // --- Elements valid in both strict FTML and the extended `doc!` dialect ---
    ($mode:ident, p { $($inner:tt)* }) => {{
        $crate::Paragraph::new_text().with_content(__tdoc_inline_nodes!($($inner)*))
    }};
    ($mode:ident, h1 { $($inner:tt)* }) => {{
        $crate::Paragraph::new_header1().with_content(__tdoc_inline_nodes!($($inner)*))
    }};
    ($mode:ident, h2 { $($inner:tt)* }) => {{
        $crate::Paragraph::new_header2().with_content(__tdoc_inline_nodes!($($inner)*))
    }};
    ($mode:ident, h3 { $($inner:tt)* }) => {{
        $crate::Paragraph::new_header3().with_content(__tdoc_inline_nodes!($($inner)*))
    }};
    ($mode:ident, code { $($inner:tt)* }) => {{
        let __text = __tdoc_collect_code_text!($($inner)*);
        $crate::Paragraph::new_code_block()
            .with_content(::std::vec::Vec::from([$crate::Span::new_text(__text)]))
    }};
    ($mode:ident, quote { $($inner:tt)* }) => {{
        $crate::Paragraph::new_quote().with_children(__tdoc_collect_blocks!($mode; $($inner)*))
    }};
    ($mode:ident, ul { $($inner:tt)* }) => {{
        $crate::Paragraph::new_unordered_list().with_entries(__tdoc_list_entries!($mode, $($inner)*))
    }};
    ($mode:ident, ol { $($inner:tt)* }) => {{
        $crate::Paragraph::new_ordered_list().with_entries(__tdoc_list_entries!($mode, $($inner)*))
    }};
    ($mode:ident, checklist { $($inner:tt)* }) => {{
        $crate::Paragraph::new_checklist()
            .with_checklist_items(__tdoc_checklist_entries!($($inner)*))
    }};
    // --- Extensions: accepted by `doc!`, rejected by strict `ftml!` ---
    (doc, table { $($inner:tt)* }) => {{
        $crate::Paragraph::new_table().with_rows(__tdoc_table_rows!($($inner)*))
    }};
    (ftml, table { $($inner:tt)* }) => {{
        compile_error!(
            "`table` is not part of strict FTML; use the `doc!` macro for tables and other extensions"
        );
    }};
    (doc, hr { }) => {{
        $crate::Paragraph::new_horizontal_rule()
    }};
    (ftml, hr { }) => {{
        compile_error!(
            "`hr` is not part of strict FTML; use the `doc!` macro for horizontal rules and other extensions"
        );
    }};
    (doc, dl { $($inner:tt)* }) => {{
        $crate::Paragraph::new_definition_list()
            .with_definition_items(__tdoc_definition_items!($($inner)*))
    }};
    (ftml, dl { $($inner:tt)* }) => {{
        compile_error!(
            "`dl` is not part of strict FTML; use the `doc!` macro for definition lists and other extensions"
        );
    }};
    // --- Anything else is genuinely unknown ---
    ($mode:ident, $other:ident { $($inner:tt)* }) => {{
        compile_error!(concat!("Unknown document element: ", stringify!($other)));
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_collect_code_text {
    () => {
        ::std::string::String::new()
    };
    ($($tt:tt)*) => {{
        let mut __text = ::std::string::String::new();
        __tdoc_collect_code_text_inner!(__text, $($tt)*);
        __text
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_collect_code_text_inner {
    ($buf:ident,) => {};
    ($buf:ident) => {};
    ($buf:ident, , $($rest:tt)*) => {
        __tdoc_collect_code_text_inner!($buf, $($rest)*);
    };
    ($buf:ident, $lit:literal $($rest:tt)*) => {{
        $buf.push_str($lit);
        __tdoc_collect_code_text_inner!($buf, $($rest)*);
    }};
    ($buf:ident, ($($expr:tt)*) $($rest:tt)*) => {{
        $buf.push_str(&::std::string::ToString::to_string(&($($expr)*)));
        __tdoc_collect_code_text_inner!($buf, $($rest)*);
    }};
    ($buf:ident, $other:tt $($rest:tt)*) => {{
        compile_error!(concat!(
            "Unsupported token inside code block: ",
            stringify!($other)
        ));
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_inline_nodes {
    () => {
        ::std::vec::Vec::<$crate::Span>::new()
    };
    ($($tt:tt)*) => {{
        let mut __spans: ::std::vec::Vec<$crate::Span> = ::std::vec::Vec::new();
        __tdoc_inline_nodes_inner!(__spans, $($tt)*);
        __spans
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_inline_nodes_inner {
    ($vec:ident,) => {};
    ($vec:ident) => {};
    ($vec:ident, , $($rest:tt)*) => {
        __tdoc_inline_nodes_inner!($vec, $($rest)*);
    };
    ($vec:ident, $tag:ident { $($inner:tt)* } $($rest:tt)*) => {{
        $vec.push(__tdoc_build_inline!($tag { $($inner)* }));
        __tdoc_inline_nodes_inner!($vec, $($rest)*);
    }};
    ($vec:ident, $lit:literal $($rest:tt)*) => {{
        $vec.push($crate::Span::new_text($lit));
        __tdoc_inline_nodes_inner!($vec, $($rest)*);
    }};
    ($vec:ident, ($($expr:tt)*) $($rest:tt)*) => {{
        $vec.push($crate::Span::new_text(($($expr)*)));
        __tdoc_inline_nodes_inner!($vec, $($rest)*);
    }};
    ($vec:ident, $ident:ident $($rest:tt)*) => {{
        $vec.push($crate::Span::new_text($ident));
        __tdoc_inline_nodes_inner!($vec, $($rest)*);
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_parse_link_children {
    () => {
        ::std::vec::Vec::<$crate::Span>::new()
    };
    ($($rest:tt)*) => {
        __tdoc_inline_nodes!($($rest)*)
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_link_target {
    ($target:literal) => {
        $target
    };
    (($($expr:tt)+)) => {
        ($($expr)+)
    };
    ($target:ident) => {
        $target
    };
    ($($path:ident)::+) => {
        $($path)::+
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_build_inline {
    (link { }) => {{
        compile_error!("`link { ... }` requires at least a link target");
    }};
    (link { $target:tt $($rest:tt)* }) => {{
        let mut __span = $crate::Span::new_styled($crate::InlineStyle::Link)
            .with_link_target(__tdoc_link_target!($target));
        let __children = __tdoc_parse_link_children!($($rest)*);
        if !__children.is_empty() {
            __span = __span.with_children(__children);
        }
        __span
    }};
    (b { $($inner:tt)* }) => {{
        $crate::Span::new_styled($crate::InlineStyle::Bold)
            .with_children(__tdoc_inline_nodes!($($inner)*))
    }};
    (i { $($inner:tt)* }) => {{
        $crate::Span::new_styled($crate::InlineStyle::Italic)
            .with_children(__tdoc_inline_nodes!($($inner)*))
    }};
    (u { $($inner:tt)* }) => {{
        $crate::Span::new_styled($crate::InlineStyle::Underline)
            .with_children(__tdoc_inline_nodes!($($inner)*))
    }};
    (del { $($inner:tt)* }) => {{
        $crate::Span::new_styled($crate::InlineStyle::Strike)
            .with_children(__tdoc_inline_nodes!($($inner)*))
    }};
    (mark { $($inner:tt)* }) => {{
        $crate::Span::new_styled($crate::InlineStyle::Highlight)
            .with_children(__tdoc_inline_nodes!($($inner)*))
    }};
    (code { $($inner:tt)* }) => {{
        $crate::Span::new_styled($crate::InlineStyle::Code)
            .with_children(__tdoc_inline_nodes!($($inner)*))
    }};
    ($other:ident { $($inner:tt)* }) => {{
        compile_error!(concat!("Unknown inline element: ", stringify!($other)));
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_list_entries {
    ($mode:ident,) => {
        ::std::vec::Vec::<::std::vec::Vec<$crate::Paragraph>>::new()
    };
    ($mode:ident, $($tt:tt)*) => {{
        let mut __entries: ::std::vec::Vec<::std::vec::Vec<$crate::Paragraph>> =
            ::std::vec::Vec::new();
        __tdoc_list_entries_inner!($mode, __entries, $($tt)*);
        __entries
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_list_entries_inner {
    ($mode:ident, $vec:ident,) => {};
    ($mode:ident, $vec:ident) => {};
    ($mode:ident, $vec:ident, , $($rest:tt)*) => {
        __tdoc_list_entries_inner!($mode, $vec, $($rest)*);
    };
    ($mode:ident, $vec:ident, li { $($inner:tt)* } $($rest:tt)*) => {{
        $vec.push(__tdoc_collect_blocks!($mode; $($inner)*));
        __tdoc_list_entries_inner!($mode, $vec, $($rest)*);
    }};
    ($mode:ident, $vec:ident, $other:ident { $($inner:tt)* } $($rest:tt)*) => {{
        compile_error!(concat!("Expected `li` inside list, found `", stringify!($other), "`"));
    }};
    ($mode:ident, $vec:ident, $unexpected:tt $($rest:tt)*) => {{
        compile_error!(concat!(
            "Unexpected token inside list: ",
            stringify!($unexpected)
        ));
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_checklist_entries {
    () => {
        ::std::vec::Vec::<$crate::ChecklistItem>::new()
    };
    ($($tt:tt)*) => {{
        let mut __entries: ::std::vec::Vec<$crate::ChecklistItem> = ::std::vec::Vec::new();
        __tdoc_checklist_entries_inner!(__entries, $($tt)*);
        __entries
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_checklist_entries_inner {
    ($vec:ident,) => {};
    ($vec:ident) => {};
    ($vec:ident, , $($rest:tt)*) => {
        __tdoc_checklist_entries_inner!($vec, $($rest)*);
    };
    ($vec:ident, todo { $($inner:tt)* } $($rest:tt)*) => {{
        let __item = $crate::ChecklistItem::new(false)
            .with_content(__tdoc_inline_nodes!($($inner)*));
        $vec.push(__item);
        __tdoc_checklist_entries_inner!($vec, $($rest)*);
    }};
    ($vec:ident, done { $($inner:tt)* } $($rest:tt)*) => {{
        let __item = $crate::ChecklistItem::new(true)
            .with_content(__tdoc_inline_nodes!($($inner)*));
        $vec.push(__item);
        __tdoc_checklist_entries_inner!($vec, $($rest)*);
    }};
    ($vec:ident, $other:ident { $($inner:tt)* } $($rest:tt)*) => {{
        compile_error!(concat!(
            "Expected `todo` or `done` inside checklist, found `",
            stringify!($other),
            "`"
        ));
    }};
    ($vec:ident, $unexpected:tt $($rest:tt)*) => {{
        compile_error!(concat!(
            "Unexpected token inside checklist: ",
            stringify!($unexpected)
        ));
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_table_rows {
    () => {
        ::std::vec::Vec::<$crate::TableRow>::new()
    };
    ($($tt:tt)*) => {{
        let mut __rows: ::std::vec::Vec<$crate::TableRow> = ::std::vec::Vec::new();
        __tdoc_table_rows_inner!(__rows, $($tt)*);
        __rows
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_table_rows_inner {
    ($vec:ident,) => {};
    ($vec:ident) => {};
    ($vec:ident, , $($rest:tt)*) => {
        __tdoc_table_rows_inner!($vec, $($rest)*);
    };
    ($vec:ident, row { $($inner:tt)* } $($rest:tt)*) => {{
        $vec.push($crate::TableRow::new().with_cells(__tdoc_table_cells!($($inner)*)));
        __tdoc_table_rows_inner!($vec, $($rest)*);
    }};
    ($vec:ident, $other:ident { $($inner:tt)* } $($rest:tt)*) => {{
        compile_error!(concat!("Expected `row` inside table, found `", stringify!($other), "`"));
    }};
    ($vec:ident, $unexpected:tt $($rest:tt)*) => {{
        compile_error!(concat!(
            "Unexpected token inside table: ",
            stringify!($unexpected)
        ));
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_table_cells {
    () => {
        ::std::vec::Vec::<$crate::TableCell>::new()
    };
    ($($tt:tt)*) => {{
        let mut __cells: ::std::vec::Vec<$crate::TableCell> = ::std::vec::Vec::new();
        __tdoc_table_cells_inner!(__cells, $($tt)*);
        __cells
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_table_cells_inner {
    ($vec:ident,) => {};
    ($vec:ident) => {};
    ($vec:ident, , $($rest:tt)*) => {
        __tdoc_table_cells_inner!($vec, $($rest)*);
    };
    ($vec:ident, th { $($inner:tt)* } $($rest:tt)*) => {{
        $vec.push($crate::TableCell::new_header().with_content(__tdoc_inline_nodes!($($inner)*)));
        __tdoc_table_cells_inner!($vec, $($rest)*);
    }};
    ($vec:ident, td { $($inner:tt)* } $($rest:tt)*) => {{
        $vec.push($crate::TableCell::new_data().with_content(__tdoc_inline_nodes!($($inner)*)));
        __tdoc_table_cells_inner!($vec, $($rest)*);
    }};
    ($vec:ident, $other:ident { $($inner:tt)* } $($rest:tt)*) => {{
        compile_error!(concat!(
            "Expected `th` or `td` inside table row, found `",
            stringify!($other),
            "`"
        ));
    }};
    ($vec:ident, $unexpected:tt $($rest:tt)*) => {{
        compile_error!(concat!(
            "Unexpected token inside table row: ",
            stringify!($unexpected)
        ));
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_definition_items {
    () => {
        ::std::vec::Vec::<$crate::DefinitionItem>::new()
    };
    ($($tt:tt)*) => {{
        let mut __items: ::std::vec::Vec<$crate::DefinitionItem> = ::std::vec::Vec::new();
        __tdoc_definition_items_inner!(__items, $($tt)*);
        __items
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_definition_items_inner {
    ($vec:ident,) => {};
    ($vec:ident) => {};
    ($vec:ident, , $($rest:tt)*) => {
        __tdoc_definition_items_inner!($vec, $($rest)*);
    };
    ($vec:ident, item { $($inner:tt)* } $($rest:tt)*) => {{
        let mut __item = $crate::DefinitionItem::new();
        __tdoc_definition_item_inner!(__item, $($inner)*);
        $vec.push(__item);
        __tdoc_definition_items_inner!($vec, $($rest)*);
    }};
    ($vec:ident, $other:ident { $($inner:tt)* } $($rest:tt)*) => {{
        compile_error!(concat!(
            "Expected `item` inside definition list, found `",
            stringify!($other),
            "`"
        ));
    }};
    ($vec:ident, $unexpected:tt $($rest:tt)*) => {{
        compile_error!(concat!(
            "Unexpected token inside definition list: ",
            stringify!($unexpected)
        ));
    }};
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! __tdoc_definition_item_inner {
    ($item:ident,) => {};
    ($item:ident) => {};
    ($item:ident, , $($rest:tt)*) => {
        __tdoc_definition_item_inner!($item, $($rest)*);
    };
    ($item:ident, term { $($inner:tt)* } $($rest:tt)*) => {{
        $item.terms.push(__tdoc_inline_nodes!($($inner)*));
        __tdoc_definition_item_inner!($item, $($rest)*);
    }};
    ($item:ident, def { $($inner:tt)* } $($rest:tt)*) => {{
        $item.definition = __tdoc_collect_blocks!(doc; $($inner)*);
        __tdoc_definition_item_inner!($item, $($rest)*);
    }};
    ($item:ident, $other:ident { $($inner:tt)* } $($rest:tt)*) => {{
        compile_error!(concat!(
            "Expected `term` or `def` inside definition item, found `",
            stringify!($other),
            "`"
        ));
    }};
    ($item:ident, $unexpected:tt $($rest:tt)*) => {{
        compile_error!(concat!(
            "Unexpected token inside definition item: ",
            stringify!($unexpected)
        ));
    }};
}

#[macro_export(local_inner_macros)]
/// Builds a [`Document`](crate::Document) using an inline DSL limited to
/// **strict FTML**.
///
/// The macro accepts block-level tags such as `p`, `h1`, `quote`, `ul`, `ol`,
/// `checklist`, and fenced `code` blocks. Inline runs can contain string
/// literals or inline tags like `b`, `i`, `mark`, `code`, and `link`.
///
/// For tdoc's non-FTML extensions (such as `table`), use [`doc!`](macro@crate::doc),
/// which understands the same syntax plus the extra elements.
///
/// # Examples
///
/// ```
/// use tdoc::{ftml, ParagraphType};
///
/// let document = ftml! {
///     h1 { "Heading" }
///     p  { "Hello, ", b { "world" }, "!" }
/// };
///
/// assert_eq!(document.paragraphs[0].paragraph_type(), ParagraphType::Header1);
/// assert_eq!(document.paragraphs[1].paragraph_type(), ParagraphType::Text);
/// ```
///
/// Extended elements such as `table` are rejected at compile time; reach for
/// [`doc!`](macro@crate::doc) instead:
///
/// ```compile_fail
/// use tdoc::ftml;
///
/// let document = ftml! {
///     table { row { td { "nope" } } }
/// };
/// ```
macro_rules! ftml {
    ($($tt:tt)*) => {{
        let __paragraphs = __tdoc_collect_blocks!(ftml; $($tt)*);
        $crate::Document::new().with_paragraphs(__paragraphs)
    }};
}

#[macro_export(local_inner_macros)]
/// Builds a [`Document`](crate::Document) using tdoc's full inline DSL.
///
/// `doc!` is a superset of [`ftml!`](macro@crate::ftml): it accepts every element the
/// strict macro does, plus tdoc's extensions that go beyond strict FTML. Today
/// that means tables; it is also the place where future extensions are added.
///
/// Tables follow the same HTML-flavored syntax as the rest of the DSL: a
/// `table` contains `row`s, and each `row` contains header cells (`th`) and data
/// cells (`td`), each holding inline content.
///
/// # Examples
///
/// ```
/// use tdoc::{doc, ParagraphType};
///
/// let document = doc! {
///     h1 { "Report" }
///     table {
///         row { th { "Name" } th { "Score" } }
///         row { td { "Alice" } td { "42" } }
///     }
/// };
///
/// assert_eq!(document.paragraphs[0].paragraph_type(), ParagraphType::Header1);
/// assert_eq!(document.paragraphs[1].paragraph_type(), ParagraphType::Table);
/// ```
macro_rules! doc {
    ($($tt:tt)*) => {{
        let __paragraphs = __tdoc_collect_blocks!(doc; $($tt)*);
        $crate::Document::new().with_paragraphs(__paragraphs)
    }};
}
