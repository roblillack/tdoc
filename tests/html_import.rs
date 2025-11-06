use std::fs::File;
use std::path::{Path, PathBuf};
use std::{fs, vec};

use tdoc::{formatter::Formatter, html, markdown, write};

fn collect_html_fixtures() -> Vec<PathBuf> {
    let data_dir = PathBuf::from("tests/data/html");
    let mut fixtures = Vec::new();

    if let Ok(entries) = fs::read_dir(&data_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("html"))
            {
                fixtures.push(path);
            }
        }
    }

    fixtures.sort();
    fixtures
}

fn load_document(path: &Path) -> tdoc::Document {
    let file = File::open(path).expect("unable to open HTML fixture");
    html::parse(file).expect("failed to parse HTML fixture")
}

fn snapshot_name(path: &Path, extension: &str) -> String {
    format!(
        "{}.{}",
        path.file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or_default(),
        extension
    )
}

#[test]
fn html_convert_text_snapshots() {
    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("snapshots/html/convert-text");
    settings.set_prepend_module_to_snapshot(false);
    settings.bind(|| {
        let fixtures = collect_html_fixtures();
        assert!(
            !fixtures.is_empty(),
            "expected HTML fixtures under tests/data/html"
        );

        for html_path in fixtures {
            let document = load_document(&html_path);
            let mut rendered: Vec<u8> = vec![];
            Formatter::new_ascii(&mut rendered)
                .write_document(&document)
                .expect("failed to format document");

            let snapshot_name = snapshot_name(&html_path, "txt");

            insta::assert_binary_snapshot!(snapshot_name.as_str(), rendered);
        }
    });
}

#[test]
fn html_convert_markdown_snapshots() {
    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("snapshots/html/convert-markdown");
    settings.set_prepend_module_to_snapshot(false);
    settings.bind(|| {
        let fixtures = collect_html_fixtures();
        assert!(
            !fixtures.is_empty(),
            "expected HTML fixtures under tests/data/html"
        );

        for html_path in fixtures {
            let document = load_document(&html_path);
            let mut rendered: Vec<u8> = vec![];
            markdown::write(&mut rendered, &document)
                .expect("failed to convert document to Markdown");

            let snapshot_name = snapshot_name(&html_path, "md");

            insta::assert_binary_snapshot!(snapshot_name.as_str(), rendered);
        }
    });
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
            let document = load_document(&html_path);
            let mut rendered: Vec<u8> = vec![];
            write(&mut rendered, &document).expect("failed to write FTML snapshot");

            let snapshot_name = snapshot_name(&html_path, "ftml");

            insta::assert_binary_snapshot!(snapshot_name.as_str(), rendered);
        }
    });
}
