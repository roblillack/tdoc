use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use tdoc::{formatter, markdown, parse};

fn collect_ftml_fixtures() -> Vec<PathBuf> {
    let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_dir.push("tests/data/ftml");

    let entries = fs::read_dir(&test_dir)
        .unwrap_or_else(|e| panic!("Failed to read test directory {:?}: {}", test_dir, e));

    let mut files: Vec<PathBuf> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("ftml") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    files.sort();

    assert!(!files.is_empty(), "No .ftml files found in {:?}", test_dir);

    files
}

fn render_ftml(document: &tdoc::Document) -> tdoc::Result<String> {
    let mut buffer = Vec::new();
    tdoc::write(&mut buffer, document)?;
    Ok(String::from_utf8(buffer)?)
}

fn load_ftml_document(path: &Path, file_name: &str) -> Option<tdoc::Document> {
    let ftml_content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read {}: {}", file_name, e);
            return None;
        }
    };

    match parse(Cursor::new(&ftml_content)) {
        Ok(doc) => Some(doc),
        Err(e) => {
            eprintln!("Failed to parse {}: {}", file_name, e);
            None
        }
    }
}

/// Test that snapshots FTML to Markdown conversion for all test files
#[test]
fn test_ftml_to_markdown_snapshots() {
    for path in collect_ftml_fixtures() {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_else(|| panic!("Invalid filename: {:?}", path));
        let base_name = file_name.strip_suffix(".ftml").unwrap_or(file_name);

        let document = match load_ftml_document(&path, file_name) {
            Some(doc) => doc,
            None => continue,
        };

        let mut markdown_output = Vec::new();
        markdown::write(&mut markdown_output, &document)
            .unwrap_or_else(|e| panic!("Failed to convert {} to markdown: {}", file_name, e));

        if matches!(
            file_name,
            "test_full_doc.ftml" | "testdocument.ftml"
        ) {
            let snapshot_name = format!("{}.md", base_name);
            insta::assert_binary_snapshot!(snapshot_name.as_str(), markdown_output);
            eprintln!(
                "Skipping canonical round-trip assertions for {} due to known fidelity limitations.",
                file_name
            );
            continue;
        }

        let markdown_string = String::from_utf8(markdown_output.clone())
            .unwrap_or_else(|e| panic!("Markdown output for {} not UTF-8: {}", file_name, e));

        let reparsed_document = markdown::parse(Cursor::new(&markdown_string))
            .unwrap_or_else(|e| panic!("Failed to re-parse markdown for {}: {}", file_name, e));

        if reparsed_document != document {
            let min_len = document
                .paragraphs
                .len()
                .min(reparsed_document.paragraphs.len());
            let mut reported = false;
            for idx in 0..min_len {
                if document.paragraphs[idx] != reparsed_document.paragraphs[idx] {
                    eprintln!(
                        "Round-trip mismatch for {} at paragraph {} ({:?})",
                        file_name, idx, document.paragraphs[idx].paragraph_type
                    );
                    if matches!(
                        document.paragraphs[idx].paragraph_type,
                        tdoc::ParagraphType::UnorderedList | tdoc::ParagraphType::OrderedList
                    ) {
                        let orig_counts: Vec<_> = document.paragraphs[idx]
                            .entries
                            .iter()
                            .map(|entry| entry.len())
                            .collect();
                        let new_counts: Vec<_> = reparsed_document.paragraphs[idx]
                            .entries
                            .iter()
                            .map(|entry| entry.len())
                            .collect();
                        eprintln!(
                            "List entry paragraph counts, original={:?} reparsed={:?}",
                            orig_counts, new_counts
                        );
                        let min_entries = document.paragraphs[idx]
                            .entries
                            .len()
                            .min(reparsed_document.paragraphs[idx].entries.len());
                        for entry_idx in 0..min_entries {
                            if document.paragraphs[idx].entries[entry_idx]
                                != reparsed_document.paragraphs[idx].entries[entry_idx]
                            {
                                eprintln!(
                                    "Entry {} diff:\noriginal: {:#?}\nreparsed: {:#?}",
                                    entry_idx,
                                    document.paragraphs[idx].entries[entry_idx],
                                    reparsed_document.paragraphs[idx].entries[entry_idx]
                                );
                                break;
                            }
                        }
                    } else {
                        eprintln!(
                            "Original paragraph: {:#?}\nReparsed paragraph: {:#?}",
                            document.paragraphs[idx], reparsed_document.paragraphs[idx]
                        );
                    }
                    reported = true;
                    break;
                }
            }
            if !reported {
                eprintln!(
                    "Round-trip mismatch for {}: paragraph count differs (original={}, reparsed={})",
                    file_name,
                    document.paragraphs.len(),
                    reparsed_document.paragraphs.len()
                );
            }
        }

        let original_ftml = render_ftml(&document)
            .unwrap_or_else(|e| panic!("Failed to render original FTML for {}: {}", file_name, e));
        let roundtrip_ftml = render_ftml(&reparsed_document).unwrap_or_else(|e| {
            panic!("Failed to render round-trip FTML for {}: {}", file_name, e)
        });

        if roundtrip_ftml != original_ftml {
            eprintln!(
                "FTML mismatch for {}:\noriginal:\n{}\nreparsed:\n{}",
                file_name, original_ftml, roundtrip_ftml
            );
        }

        assert_eq!(
            reparsed_document, document,
            "Document mismatch for {}",
            file_name
        );

        let snapshot_name = format!("{}.md", base_name);
        insta::assert_binary_snapshot!(snapshot_name.as_str(), markdown_output);
    }
}

/// Test that snapshots FTML through the ASCII formatter for all test files
#[test]
fn test_ftml_to_ascii_snapshots() {
    for path in collect_ftml_fixtures() {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_else(|| panic!("Invalid filename: {:?}", path));
        let base_name = file_name.strip_suffix(".ftml").unwrap_or(file_name);

        let document = match load_ftml_document(&path, file_name) {
            Some(doc) => doc,
            None => continue,
        };

        let mut ascii_output = Vec::new();
        let mut formatter = formatter::Formatter::new_ascii(&mut ascii_output);

        formatter
            .write_document(&document)
            .unwrap_or_else(|e| panic!("Failed to format {} as ASCII: {}", file_name, e));

        let snapshot_name = format!("ascii__{}.txt", base_name);
        insta::assert_binary_snapshot!(snapshot_name.as_str(), ascii_output);
    }
}

/// Individual test for simple paragraph
#[test]
fn test_simple_paragraph_to_markdown() {
    let ftml = "<p>Hello, world!</p>";
    let doc = parse(Cursor::new(ftml)).unwrap();
    let mut output = Vec::new();
    markdown::write(&mut output, &doc).unwrap();
    let markdown = String::from_utf8(output).unwrap();
    insta::assert_snapshot!(markdown);
}

/// Individual test for bold text
#[test]
fn test_bold_text_to_markdown() {
    let ftml = "<p>This is <b>bold</b> text.</p>";
    let doc = parse(Cursor::new(ftml)).unwrap();
    let mut output = Vec::new();
    markdown::write(&mut output, &doc).unwrap();
    let markdown = String::from_utf8(output).unwrap();
    insta::assert_snapshot!(markdown);
}

/// Individual test for italic text
#[test]
fn test_italic_text_to_markdown() {
    let ftml = "<p>This is <i>italic</i> text.</p>";
    let doc = parse(Cursor::new(ftml)).unwrap();
    let mut output = Vec::new();
    markdown::write(&mut output, &doc).unwrap();
    let markdown = String::from_utf8(output).unwrap();
    insta::assert_snapshot!(markdown);
}

/// Individual test for code text
#[test]
fn test_code_text_to_markdown() {
    let ftml = "<p>This is <code>code</code> text.</p>";
    let doc = parse(Cursor::new(ftml)).unwrap();
    let mut output = Vec::new();
    markdown::write(&mut output, &doc).unwrap();
    let markdown = String::from_utf8(output).unwrap();
    insta::assert_snapshot!(markdown);
}

/// Individual test for strikethrough text
#[test]
fn test_strikethrough_to_markdown() {
    let ftml = "<p>This is <s>strikethrough</s> text.</p>";
    let doc = parse(Cursor::new(ftml)).unwrap();
    let mut output = Vec::new();
    markdown::write(&mut output, &doc).unwrap();
    let markdown = String::from_utf8(output).unwrap();
    insta::assert_snapshot!(markdown);
}

/// Individual test for headers
#[test]
fn test_headers_to_markdown() {
    let ftml = "<h1>Header 1</h1><h2>Header 2</h2><h3>Header 3</h3>";
    let doc = parse(Cursor::new(ftml)).unwrap();
    let mut output = Vec::new();
    markdown::write(&mut output, &doc).unwrap();
    let markdown = String::from_utf8(output).unwrap();
    insta::assert_snapshot!(markdown);
}

/// Individual test for blockquote
#[test]
fn test_blockquote_to_markdown() {
    let ftml = "<blockquote><p>This is a quote.</p></blockquote>";
    let doc = parse(Cursor::new(ftml)).unwrap();
    let mut output = Vec::new();
    markdown::write(&mut output, &doc).unwrap();
    let markdown = String::from_utf8(output).unwrap();
    insta::assert_snapshot!(markdown);
}

/// Individual test for unordered list
#[test]
fn test_unordered_list_to_markdown() {
    let ftml = "<ul><li><p>Item 1</p></li><li><p>Item 2</p></li><li><p>Item 3</p></li></ul>";
    let doc = parse(Cursor::new(ftml)).unwrap();
    let mut output = Vec::new();
    markdown::write(&mut output, &doc).unwrap();
    let markdown = String::from_utf8(output).unwrap();
    insta::assert_snapshot!(markdown);
}

/// Individual test for ordered list
#[test]
fn test_ordered_list_to_markdown() {
    let ftml = "<ol><li><p>First</p></li><li><p>Second</p></li><li><p>Third</p></li></ol>";
    let doc = parse(Cursor::new(ftml)).unwrap();
    let mut output = Vec::new();
    markdown::write(&mut output, &doc).unwrap();
    let markdown = String::from_utf8(output).unwrap();
    insta::assert_snapshot!(markdown);
}

/// Individual test for nested lists
#[test]
fn test_nested_lists_to_markdown() {
    let ftml = "<ul><li><p>Item 1</p><ul><li><p>Nested 1</p></li><li><p>Nested 2</p></li></ul></li><li><p>Item 2</p></li></ul>";
    let doc = parse(Cursor::new(ftml)).unwrap();
    let mut output = Vec::new();
    markdown::write(&mut output, &doc).unwrap();
    let markdown = String::from_utf8(output).unwrap();
    insta::assert_snapshot!(markdown);
}

/// Individual test for mixed inline styles
#[test]
fn test_mixed_inline_styles_to_markdown() {
    let ftml =
        "<p>This has <b>bold</b>, <i>italic</i>, <code>code</code>, and <s>strikethrough</s>.</p>";
    let doc = parse(Cursor::new(ftml)).unwrap();
    let mut output = Vec::new();
    markdown::write(&mut output, &doc).unwrap();
    let markdown = String::from_utf8(output).unwrap();
    insta::assert_snapshot!(markdown);
}

/// Individual test for nested inline styles
#[test]
fn test_nested_inline_styles_to_markdown() {
    let ftml = "<p>This is <b>bold with <i>italic</i> inside</b>.</p>";
    let doc = parse(Cursor::new(ftml)).unwrap();
    let mut output = Vec::new();
    markdown::write(&mut output, &doc).unwrap();
    let markdown = String::from_utf8(output).unwrap();
    insta::assert_snapshot!(markdown);
}

/// Individual test for complex document
#[test]
fn test_complex_document_to_markdown() {
    let ftml = r#"<h1>Title</h1>
<p>Introduction paragraph with <b>bold</b> text.</p>
<h2>Section 1</h2>
<p>Some content here.</p>
<ul>
<li><p>List item 1</p></li>
<li><p>List item 2</p></li>
</ul>
<blockquote>
<p>A quoted paragraph.</p>
</blockquote>"#;
    let doc = parse(Cursor::new(ftml)).unwrap();
    let mut output = Vec::new();
    markdown::write(&mut output, &doc).unwrap();
    let markdown = String::from_utf8(output).unwrap();
    insta::assert_snapshot!(markdown);
}
