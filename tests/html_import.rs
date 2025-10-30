use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use insta::Settings;
use tdoc::html;
use tdoc::formatter::Formatter;

struct BufferWriter {
    buffer: Vec<u8>,
}

impl BufferWriter {
    fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    fn into_string(self) -> String {
        String::from_utf8(self.buffer).expect("formatter produced invalid UTF-8")
    }
}

impl Write for BufferWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

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

fn normalize(text: &str) -> String {
    text.trim_end().to_string()
}

#[test]
fn html_import_snapshots() {
    let fixtures = collect_html_fixtures();
    assert!(
        !fixtures.is_empty(),
        "expected HTML fixtures under tests/data/html"
    );

    for html_path in fixtures {
        let expected_path = html_path.with_extension("txt");

        let expected =
            fs::read_to_string(&expected_path).expect("unable to read expected snapshot file");

        let document = {
            let file = File::open(&html_path).expect("unable to open HTML fixture");
            html::parse(file).expect("failed to parse HTML fixture")
        };

        let rendered = {
            let mut sink = BufferWriter::new();
            {
                let mut formatter = Formatter::new_ascii(&mut sink);
                formatter
                    .write_document(&document)
                    .expect("failed to format document");
            }
            sink.into_string()
        };

        let rendered_norm = normalize(&rendered);
        let expected_norm = normalize(&expected);

        assert_eq!(
            expected_norm, rendered_norm,
            "rendered FTML differs from expected output for {}",
            html_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default()
        );

        let mut settings = Settings::clone_current();
        settings.set_snapshot_path("snapshots/html");
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();

        let snapshot_name = format!(
            "html_import__{}",
            html_path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or_default()
        );

        insta::assert_snapshot!(snapshot_name, rendered_norm);
    }
}
