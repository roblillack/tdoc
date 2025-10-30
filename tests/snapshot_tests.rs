use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use tdoc::{formatter, markdown, parse};

/// Test that snapshots FTML to Markdown conversion for all test files
#[test]
fn test_ftml_to_markdown_snapshots() {
    // Get the test data directory
    let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_dir.push("tests/data/ftml");

    // Read all .ftml files from the directory
    let entries = fs::read_dir(&test_dir)
        .unwrap_or_else(|e| panic!("Failed to read test directory {:?}: {}", test_dir, e));

    // Collect and sort file paths
    let mut ftml_files: Vec<PathBuf> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "ftml" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    ftml_files.sort();

    assert!(
        !ftml_files.is_empty(),
        "No .ftml files found in {:?}",
        test_dir
    );

    for path in ftml_files {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_else(|| panic!("Invalid filename: {:?}", path));

        // Read FTML file
        let ftml_content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to read {}: {}", file_name, e);
                continue;
            }
        };

        // Parse FTML
        let document = match parse(Cursor::new(&ftml_content)) {
            Ok(doc) => doc,
            Err(e) => {
                eprintln!("Failed to parse {}: {}", file_name, e);
                continue;
            }
        };

        // Convert to Markdown
        let mut markdown_output = Vec::new();
        if let Err(e) = markdown::write(&mut markdown_output, &document) {
            eprintln!("Failed to convert {} to markdown: {}", file_name, e);
            continue;
        }

        // Create snapshot name from filename (remove .ftml extension)
        let snapshot_name = format!(
            "{}.md",
            file_name.strip_suffix(".ftml").unwrap_or(file_name)
        );

        // Snapshot the markdown output
        insta::assert_binary_snapshot!(snapshot_name.as_str(), markdown_output);
    }
}

/// Test that snapshots FTML through the ASCII formatter for all test files
#[test]
fn test_ftml_to_ascii_snapshots() {
    // Get the test data directory
    let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_dir.push("tests/data/ftml");

    // Read all .ftml files from the directory
    let entries = fs::read_dir(&test_dir)
        .unwrap_or_else(|e| panic!("Failed to read test directory {:?}: {}", test_dir, e));

    // Collect and sort file paths
    let mut ftml_files: Vec<PathBuf> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "ftml" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    ftml_files.sort();

    assert!(
        !ftml_files.is_empty(),
        "No .ftml files found in {:?}",
        test_dir
    );

    for path in ftml_files {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_else(|| panic!("Invalid filename: {:?}", path));

        // Read FTML file
        let ftml_content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to read {}: {}", file_name, e);
                continue;
            }
        };

        // Parse FTML
        let document = match parse(Cursor::new(&ftml_content)) {
            Ok(doc) => doc,
            Err(e) => {
                eprintln!("Failed to parse {}: {}", file_name, e);
                continue;
            }
        };

        // Format with the ASCII formatter
        let mut ascii_output = Vec::new();
        let mut formatter = formatter::Formatter::new_ascii(&mut ascii_output);

        if let Err(e) = formatter.write_document(&document) {
            eprintln!("Failed to format {} as ASCII: {}", file_name, e);
            continue;
        }

        let base_name = file_name.strip_suffix(".ftml").unwrap_or(file_name);

        // Create snapshot name from filename with an ASCII-specific prefix
        let snapshot_name = format!("ascii__{}.txt", base_name);

        // Snapshot the ASCII formatted output
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
