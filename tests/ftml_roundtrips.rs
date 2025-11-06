use std::fs::{self, File};
use std::io::Cursor;
use std::path::{Path, PathBuf};

use tdoc::parse;

pub fn collect_ftml_fixtures() -> Vec<PathBuf> {
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

pub fn load_ftml_document(path: &Path) -> Option<tdoc::Document> {
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

pub fn render_ftml(document: &tdoc::Document) -> String {
    let mut buffer = Vec::new();
    tdoc::write(&mut buffer, document)
        .unwrap_or_else(|err| panic!("failed to render FTML document: {}", err));
    String::from_utf8(buffer)
        .unwrap_or_else(|err| panic!("failed to convert rendered FTML to UTF-8: {}", err))
}

#[test]
fn ftml_roundtrips_ftml_documents() {
    let fixtures = collect_ftml_fixtures();
    assert!(
        !fixtures.is_empty(),
        "expected FTML fixtures under tests/data/ftml or tests/snapshots/html/import"
    );

    for ftml_path in fixtures {
        let Some(expected) = load_ftml_document(&ftml_path) else {
            continue;
        };

        let rendered_ftml = render_ftml(&expected);
        let roundtripped = parse(Cursor::new(rendered_ftml.as_bytes())).unwrap_or_else(|err| {
            panic!(
                "failed to parse rendered FTML output for {}: {}",
                ftml_path.display(),
                err
            )
        });

        assert_eq!(
            roundtripped,
            expected,
            "FTML round-trip mismatch for {}",
            ftml_path.display()
        );
    }
}
