use std::io::Cursor;

use tdoc::markdown;

mod ftml_roundtrips;
use ftml_roundtrips::{
    collect_ftml_fixtures, load_ftml_document, render_ftml, should_skip_roundtrip,
};

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

        if roundtripped != expected {
            let expected_ftml = render_ftml(&expected);
            let roundtrip_ftml = render_ftml(&roundtripped);
            panic!(
                "round-trip mismatch for {}\nexpected:\n{}\nroundtrip:\n{}",
                ftml_path.display(),
                expected_ftml,
                roundtrip_ftml
            );
        }
    }
}
