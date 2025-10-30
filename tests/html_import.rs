use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, vec};

use insta::Settings;
use tdoc::formatter::Formatter;
use tdoc::{html, write};

fn collect_html_fixtures() -> Vec<PathBuf> {
    let data_dir = PathBuf::from("tests/data/html");
    let mut fixtures = Vec::new();

    if let Ok(entries) = fs::read_dir(&data_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path
                .extension()
                .and_then(|ext| ext.to_str())
                .map_or(false, |ext| ext.eq_ignore_ascii_case("html"))
            {
                fixtures.push(path);
            }
        }
    }

    fixtures.sort();
    fixtures
}

#[test]
fn html_import_snapshots() {
    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("snapshots/html/import");
    settings.set_prepend_module_to_snapshot(false);
    settings.bind(|| {
        let fixtures = collect_html_fixtures();
        assert!(
            !fixtures.is_empty(),
            "expected HTML fixtures under tests/data/html"
        );

        for html_path in fixtures {
            let document = {
                let file = File::open(&html_path).expect("unable to open HTML fixture");
                html::parse(file).expect("failed to parse HTML fixture")
            };

            let mut rendered: Vec<u8> = vec![];
            Formatter::new_ascii(&mut rendered)
                .write_document(&document)
                .expect("failed to format document");
            // write(&mut rendered, &document).expect("failed to write FTML snapshot");

            let snapshot_name = format!(
                "{}.txt",
                html_path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or_default()
            );

            insta::assert_binary_snapshot!(snapshot_name.as_str(), rendered);
        }
    });
}
