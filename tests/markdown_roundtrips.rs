use std::io::Cursor;

use tdoc::{markdown, parse};

mod ftml_roundtrips;
use ftml_roundtrips::{
    collect_ftml_fixtures, load_ftml_document, render_ftml, should_skip_roundtrip,
};

pub fn canonicalize_document(document: &tdoc::Document) -> tdoc::Document {
    let rendered = render_ftml(document);
    let mut canonical = parse(Cursor::new(&rendered))
        .unwrap_or_else(|err| panic!("failed to parse canonical FTML representation: {}", err));
    normalize_soft_breaks(&mut canonical);
    canonical
}

fn normalize_soft_breaks(document: &mut tdoc::Document) {
    for paragraph in &mut document.paragraphs {
        normalize_paragraph(paragraph);
    }
}

fn normalize_paragraph(paragraph: &mut tdoc::Paragraph) {
    normalize_spans(&mut paragraph.content);
    for child in &mut paragraph.children {
        normalize_paragraph(child);
    }
    for entry in &mut paragraph.entries {
        for child in entry {
            normalize_paragraph(child);
        }
    }
}

fn normalize_spans(spans: &mut Vec<tdoc::Span>) {
    for span in spans.iter_mut() {
        if !span.text.is_empty() {
            span.text = span.text.replace('\n', " ");
        }
        if !span.children.is_empty() {
            normalize_spans(&mut span.children);
        }
    }

    let mut normalized: Vec<tdoc::Span> = Vec::with_capacity(spans.len());
    for span in spans.drain(..) {
        if let Some(last) = normalized.last_mut() {
            if can_merge_spans(last, &span) {
                last.text.push_str(&span.text);
                continue;
            }
        }
        normalized.push(span);
    }
    *spans = normalized;
}

fn can_merge_spans(a: &tdoc::Span, b: &tdoc::Span) -> bool {
    a.style == b.style
        && a.link_target == b.link_target
        && a.children.is_empty()
        && b.children.is_empty()
}

#[test]
fn markdown_roundtrips_ftml_documents() {
    let fixtures = collect_ftml_fixtures();
    assert!(
        !fixtures.is_empty(),
        "expected FTML fixtures under tests/data/ftml or tests/snapshots/html/import"
    );

    for ftml_path in fixtures {
        let Some(expected) = load_ftml_document(&ftml_path) else {
            continue;
        };

        let mut markdown_output = Vec::new();
        markdown::write(&mut markdown_output, &expected).unwrap_or_else(|err| {
            panic!(
                "failed to convert {} to Markdown: {}",
                ftml_path.display(),
                err
            )
        });

        let markdown_string = String::from_utf8(markdown_output).unwrap_or_else(|err| {
            panic!(
                "Markdown output for {} is not UTF-8: {}",
                ftml_path.display(),
                err
            )
        });

        let roundtripped = markdown::parse(Cursor::new(&markdown_string)).unwrap_or_else(|err| {
            panic!(
                "failed to re-import Markdown for {}: {}",
                ftml_path.display(),
                err
            )
        });

        if should_skip_roundtrip(&ftml_path) {
            eprintln!(
                "Skipping strict round-trip assertion for {} due to known Markdown fidelity limitations.",
                ftml_path.display()
            );
            continue;
        }

        let canonical_expected = canonicalize_document(&expected);
        let canonical_roundtrip = canonicalize_document(&roundtripped);

        if canonical_roundtrip != canonical_expected {
            let expected_ftml = render_ftml(&canonical_expected);
            let roundtrip_ftml = render_ftml(&canonical_roundtrip);
            panic!(
                "round-trip mismatch for {}\nexpected:\n{}\nroundtrip:\n{}",
                ftml_path.display(),
                expected_ftml,
                roundtrip_ftml
            );
        }
    }
}
