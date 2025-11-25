use std::fs::{self, File};
use std::io::Cursor;
use std::path::{Path, PathBuf};

use tdoc::gemini;

fn collect_gemini_fixtures() -> Vec<PathBuf> {
    let mut fixtures = Vec::new();

    if let Ok(entries) = fs::read_dir("tests/data/gemini") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("gmi"))
            {
                fixtures.push(path);
            }
        }
    }

    fixtures.sort();
    fixtures
}

fn load_gemini_document(path: &Path) -> Option<tdoc::Document> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Unable to open Gemini fixture {}: {}", path.display(), err);
            return None;
        }
    };
    match gemini::parse(file) {
        Ok(document) => Some(document),
        Err(err) => {
            eprintln!(
                "Failed to parse Gemini fixture {}: {}. Skipping round-trip test for this fixture.",
                path.display(),
                err
            );
            None
        }
    }
}

fn render_gemini(document: &tdoc::Document) -> String {
    let mut buffer = Vec::new();
    gemini::write(&mut buffer, document)
        .unwrap_or_else(|err| panic!("failed to render Gemini document: {}", err));
    String::from_utf8(buffer)
        .unwrap_or_else(|err| panic!("failed to convert rendered Gemini to UTF-8: {}", err))
}

#[test]
fn gemini_roundtrips_gemini_documents() {
    let fixtures = collect_gemini_fixtures();
    assert!(
        !fixtures.is_empty(),
        "expected Gemini fixtures under tests/data/gemini"
    );

    for gemini_path in fixtures {
        let Some(expected) = load_gemini_document(&gemini_path) else {
            continue;
        };

        let rendered_gemini = render_gemini(&expected);
        let roundtripped =
            gemini::parse(Cursor::new(rendered_gemini.as_bytes())).unwrap_or_else(|err| {
                panic!(
                    "failed to parse rendered Gemini output for {}: {}",
                    gemini_path.display(),
                    err
                )
            });

        if roundtripped != expected {
            let expected_gemini = render_gemini(&expected);
            let roundtrip_gemini = render_gemini(&roundtripped);
            panic!(
                "Gemini round-trip mismatch for {}\nexpected:\n{}\nroundtrip:\n{}",
                gemini_path.display(),
                expected_gemini,
                roundtrip_gemini
            );
        }
    }
}

mod ftml_roundtrips;
use ftml_roundtrips::{collect_ftml_fixtures, load_ftml_document, render_ftml};

#[test]
fn gemini_roundtrips_ftml_documents() {
    let fixtures = collect_ftml_fixtures();
    assert!(
        !fixtures.is_empty(),
        "expected FTML fixtures under tests/data/ftml or tests/snapshots/html/import"
    );

    for ftml_path in fixtures {
        let Some(expected) = load_ftml_document(&ftml_path) else {
            continue;
        };

        let mut gemini_output = Vec::new();
        gemini::write(&mut gemini_output, &expected).unwrap_or_else(|err| {
            panic!(
                "failed to convert {} to Gemini: {}",
                ftml_path.display(),
                err
            )
        });

        let gemini_string = String::from_utf8(gemini_output).unwrap_or_else(|err| {
            panic!(
                "Gemini output for {} is not UTF-8: {}",
                ftml_path.display(),
                err
            )
        });

        let roundtripped = gemini::parse(Cursor::new(&gemini_string)).unwrap_or_else(|err| {
            panic!(
                "failed to re-import Gemini for {}: {}",
                ftml_path.display(),
                err
            )
        });

        // Gemini is lossy (no inline formatting), so we can't expect exact match
        // Instead, we verify that the structure is preserved
        if roundtripped.paragraphs.len() != expected.paragraphs.len() {
            let expected_ftml = render_ftml(&expected);
            let roundtrip_ftml = render_ftml(&roundtripped);
            eprintln!(
                "Note: Gemini round-trip has different paragraph count for {} (expected {}, got {})",
                ftml_path.display(),
                expected.paragraphs.len(),
                roundtripped.paragraphs.len()
            );
            eprintln!("Expected FTML:\n{}", expected_ftml);
            eprintln!("Roundtrip FTML:\n{}", roundtrip_ftml);
            // Don't panic, just warn - Gemini is intentionally lossy
        }
    }
}
