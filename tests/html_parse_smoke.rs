use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use tdoc::html;

fn list_html_fixtures() -> Vec<PathBuf> {
    let mut fixtures = Vec::new();
    if let Ok(entries) = std::fs::read_dir("tests/data/html") {
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
fn parse_all_html_fixtures() {
    let fixtures = list_html_fixtures();
    assert!(
        !fixtures.is_empty(),
        "expected HTML fixtures under tests/data/html"
    );

    for html_path in fixtures {
        let display_name = html_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("<unknown>");

        let mut file = File::open(&html_path)
            .unwrap_or_else(|err| panic!("unable to open {}: {}", display_name, err));
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .unwrap_or_else(|err| panic!("unable to read {}: {}", display_name, err));

        match html::parse(buffer.as_bytes()) {
            Ok(_) => {}
            Err(err) => panic!("parsing {} failed: {}", display_name, err),
        }
    }
}
