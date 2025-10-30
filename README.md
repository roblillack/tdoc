# tdoc

[![Build Status](https://github.com/roblillack/tdoc/workflows/build-lint-test/badge.svg)](https://github.com/roblillack/tdoc/actions)
[![Crates.io](https://img.shields.io/crates/v/tdoc.svg)](https://crates.io/crates/tdoc)
[![Downloads](https://img.shields.io/crates/d/tdoc.svg)](https://crates.io/crates/tdoc)
[![Docs.rs](https://docs.rs/tdoc/badge.svg)](https://docs.rs/tdoc)

A Rust library and CLI tools for working with FTML (Formatted Text Markup Language) documents.

This project is a partial rewrite of the Go library available at https://github.com/roblillack/ftml, bringing FTML support to the Rust ecosystem with improved performance and memory safety.

![tdoc terminal rendering example](screenshot.png)

## What is FTML?

**FTML (Formatted Text Markup Language)** is a lightweight document format designed for simplicity and ease of processing. As a **strict subset of HTML5**, it remains fully compatible with standard web technologies while being far easier to parse and work with programmatically. FTML provides the essential features needed for rich text documents—such as paragraph structures, headings, lists, and inline styles—without the complexity of full HTML or Markdown. It’s ideal for straightforward text content like emails, memos, notes, and help documentation.

**Key features:**

- **Simple structure**: Only the most essential formatting options
- **HTML-compatible**: Valid FTML is valid HTML5
- **Diffable**: Designed to work well with version control
- **Unambiguous**: Usually only one way to express something

For the full FTML specification, see the [original repository](https://github.com/roblillack/ftml).

## Features

tdoc provides a comprehensive toolkit for working with FTML documents in Rust:

- **Load and Save**: Parse FTML documents from files or streams, and write them back with proper formatting
- **Terminal Rendering**: Render documents to terminal screens with full support for ASCII/ANSI formatting, including **bold**, _italic_, <u>underline</u>, <del>strikethrough</del>, <mark>highlight</mark>, `code` and all supported paragraph types
- **Format Conversion**: Convert between FTML and other formats:
  - **Markdown**: Export FTML documents to Markdown for compatibility with documentation systems
  - **HTML**: Import HTML documents into FTML (basic support), with plans for full HTML export
- **Document Manipulation**: Build and modify FTML documents programmatically with a clean, type-safe API
- **Command-line Tools**: Ready-to-use CLI utilities for viewing, converting, and formatting FTML documents

## Document Structure

FTML documents consist of a hierarchy of elements:

### Block-level Elements

- **Text paragraphs** (`<p>`)
- **Headers** (`<h1>`, `<h2>`, `<h3>`)
- **Lists** - ordered (`<ol>`) or unordered (`<ul>`)
- **Blockquotes** (`<blockquote>`)

### Inline Styles

Text spans can have optional styles:

- **Bold** (`<b>`)
- **Italic** (`<i>`)
- **Underline** (`<u>`)
- **Strike** (`<s>`)
- **Highlight** (`<mark>`)
- **Code** (`<code>`)
- **Link** (`<a>`)

### Example Document

```ftml
<h1>This <i>very</i> simple example shows ...</h1>
<p>How FTML really is this:</p>
<ul>
  <li><p>A <mark>strict</mark> subset of HTML,</p></li>
  <li><p>That is <b>easy</b> to wrap your head around.</p></li>
</ul>
```

## Library Usage

### Reading Documents

```rust
use tdoc::{parse, Document};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse from a file
    let file = File::open("document.ftml")?;
    let document = parse(file)?;

    // Access document structure
    for paragraph in &document.paragraphs {
        println!("Paragraph type: {}", paragraph.paragraph_type);
    }

    Ok(())
}
```

### Writing Documents

```rust
use tdoc::{write, Document, Paragraph, Span};
use std::io::stdout;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a document programmatically
    let mut doc = Document::new();

    let paragraph = Paragraph::new_text()
        .with_content(vec![
            Span::new_text("Hello, "),
            Span::new_styled(tdoc::InlineStyle::Bold)
                .with_children(vec![Span::new_text("world!")]),
        ]);

    doc.add_paragraph(paragraph);

    // Write to stdout
    write(&mut stdout(), &doc)?;

    Ok(())
}
```

### Exporting to Markdown

```rust
use tdoc::{parse, markdown};
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("document.ftml")?;
    let document = parse(file)?;

    // Export to Markdown
    markdown::write(&mut std::io::stdout(), &document)?;

    Ok(())
}
```

### Importing from HTML

```rust
use tdoc::html;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("document.html")?;
    let document = html::parse(file)?;

    // Now you have an FTML document
    println!("Parsed {} paragraphs", document.paragraphs.len());

    Ok(())
}
```

## CLI Tools

### viewftml

View FTML and HTML files with formatted terminal output:

```bash
# View a local FTML file
viewftml document.ftml

# View from a URL
viewftml https://example.com/document.html

# Disable ANSI formatting
viewftml --no-ansi document.ftml

# Save formatted FTML to stdout
viewftml --save document.html > output.ftml
```

### ftml2md

Convert FTML documents to Markdown:

```bash
# Convert file
ftml2md input.ftml output.md

# Use stdin/stdout
cat input.ftml | ftml2md - - > output.md
```

### fmtftml

Format FTML documents:

```bash
# Format a file
fmtftml input.ftml

# Format in-place
fmtftml -w document.ftml
```

## Implementation Status

This Rust implementation is a work in progress. Here's how it compares to the [Go version](https://github.com/roblillack/ftml):

| Feature                | Rust (tdoc)            | Go (ftml)     | Notes                                  |
| ---------------------- | ---------------------- | ------------- | -------------------------------------- |
| **Core Library**       |                        |               |                                        |
| FTML Parsing           | ✅ Full                | ✅ Full       | Both implementations complete          |
| FTML Writing           | ✅ Full                | ✅ Full       | Both implementations complete          |
| **Terminal Rendering** |                        |               |                                        |
| ASCII Support          | ✅ Full                | ✅ Full       | Both implementations complete          |
| ANSI Support           | ✅ Full                | ✅ Full       | Both implementations complete          |
| **Import/Export**      |                        |               |                                        |
| Markdown Import        | ✅ Full                | ❌ Planned    | No implementation yet                  |
| Markdown Export        | ⚠️ Basic               | ✅ Full       | Both support all FTML elements         |
| HTML Import            | ❌ Rudimentary support | ✅ Full       | Only Go version has proper HTML parser |
| HTML Export            | ❌ Planned             | ✅ Full       | Not yet implemented in Rust            |
| **CLI Tools**          |                        |               |                                        |
| Document Viewer        | ✅ `viewftml`          | ✅ `viewftml` | Both with terminal formatting          |
| **Advanced Features**  |                        |               |                                        |
| URL Fetching           | ✅ Yes                 | ❌ No         | `viewftml` can fetch from URLs         |
| Paged Output           | ✅ Yes                 | ✅ Yes        | Both support pager integration         |

**Note on HTML Import**: The current HTML import in the Rust version is functional but uses a simplified regex-based approach to convert HTML to FTML before parsing. It handles common patterns but may not correctly parse complex or malformed HTML. The Go version has a more robust HTML parser. Improvements to the Rust HTML import are planned for future releases.

## Building from Source

```bash
# Build the library and all tools
cargo build --release

# Run tests
cargo test

# Build specific binary
cargo build --release --bin viewftml
```

## License

MIT

## Contributing

This is a work in progress. Contributions are welcome! Please see the [original FTML repository](https://github.com/roblillack/ftml) for the specification details.
