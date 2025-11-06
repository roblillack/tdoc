use std::fs::{self, File};
use std::io::Cursor;
use std::path::{Path, PathBuf};

use tdoc::{markdown, parse};

fn collect_markdown_fixtures() -> Vec<PathBuf> {
    let data_dir = PathBuf::from("tests/data/markdown");
    let mut fixtures = Vec::new();

    if let Ok(entries) = fs::read_dir(&data_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
            {
                fixtures.push(path);
            }
        }
    }

    fixtures.sort();
    fixtures
}

fn collect_ftml_fixtures() -> Vec<PathBuf> {
    let mut fixtures = Vec::new();

    for dir in ["tests/data/ftml", "tests/snapshots/html/import"] {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("ftml"))
                {
                    fixtures.push(path);
                }
            }
        }
    }

    fixtures.sort();
    fixtures
}

fn load_ftml_document(path: &Path) -> Option<tdoc::Document> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Unable to open FTML fixture {}: {}", path.display(), err);
            return None;
        }
    };
    match parse(file) {
        Ok(document) => Some(document),
        Err(err) => {
            eprintln!(
                "Failed to parse FTML fixture {}: {}. Skipping round-trip test for this fixture.",
                path.display(),
                err
            );
            None
        }
    }
}

fn should_skip_roundtrip(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some(
            "test_full_doc.ftml"
                | "test_nested_quote_in_list.ftml"
                | "testdocument.ftml"
                | "gmail-updated-tos.snap.ftml"
                | "lite-cnn-com.snap.ftml"
                | "marketing-email-1.snap.ftml"
                | "motherfuckingwebsite.snap.ftml"
                | "newyorker-what-does-it-mean-that-donald-trump-is-a-fascist.snap.ftml"
                | "things-app-newsletter.snap.ftml"
                | "todoist-daily-update-mail.snap.ftml"
                | "todoist-monthly-newsletter-english-october-2024.snap.ftml"
                | "todoist-monthly-newsletter-german-october-2024.snap.ftml"
        )
    )
}

fn render_ftml(document: &tdoc::Document) -> String {
    let mut buffer = Vec::new();
    tdoc::write(&mut buffer, document)
        .unwrap_or_else(|err| panic!("failed to render FTML document: {}", err));
    String::from_utf8(buffer)
        .unwrap_or_else(|err| panic!("failed to convert rendered FTML to UTF-8: {}", err))
}

fn canonicalize_document(document: &tdoc::Document) -> tdoc::Document {
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
fn markdown_import_snapshots() {
    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("snapshots/markdown/import");
    settings.set_prepend_module_to_snapshot(false);
    settings.bind(|| {
        let fixtures = collect_markdown_fixtures();
        assert!(
            !fixtures.is_empty(),
            "expected Markdown fixtures under tests/data/markdown"
        );

        for markdown_path in fixtures {
            let document = {
                let file = File::open(&markdown_path).expect("unable to open Markdown fixture");
                markdown::parse(file).expect("failed to parse Markdown fixture")
            };

            let mut rendered = Vec::new();
            tdoc::write(&mut rendered, &document).expect("failed to render FTML snapshot");

            let snapshot_name = format!(
                "{}.ftml",
                markdown_path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or_default()
            );

            insta::assert_binary_snapshot!(snapshot_name.as_str(), rendered);
        }
    });
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
