use std::fs::{self, File};
use std::path::PathBuf;

use tdoc::markdown;

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
