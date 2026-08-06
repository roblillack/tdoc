//! Drives tdoc against pulldown-cmark's own definition-list conformance suite.
//!
//! `tests/data/spec/definition_lists.txt` is a verbatim copy of the spec file
//! that generates pulldown-cmark's `definition_lists` tests. Since
//! pulldown-cmark is tdoc's Markdown engine, that file describes exactly the
//! dialect tdoc inherits, which makes it the right corpus to hold the Markdown
//! importer and writer against. See `tests/data/CREDITS.md` for provenance.
//!
//! Each case asserts the property that matters for a document toolkit: parsing
//! a document and writing it back out must not change what the document *is*.
//! The rendered Markdown itself is snapshotted so that changes to the writer's
//! output are reviewed rather than discovered.

use std::io::Cursor;
use std::path::PathBuf;

use tdoc::{markdown, Document, Paragraph};

const SPEC: &str = include_str!("data/spec/definition_lists.txt");

/// Spec cases that do not survive a Markdown round-trip today, each for a
/// reason that has nothing to do with definition lists themselves. They are
/// listed here rather than silently skipped, and
/// [`every_spec_case_parses_and_round_trips`] asserts that each one still
/// fails — so whoever fixes the underlying limitation is told to remove it.
const KNOWN_ROUND_TRIP_GAPS: &[(usize, &str)] = &[
    (
        19,
        "indented code blocks re-split their inline spans on reparse; the code text is \
         identical, only its span segmentation differs",
    ),
    (
        28,
        "raw HTML blocks are not supported by the importer, so `<div>` terms come back as \
         escaped text",
    ),
    (
        31,
        "a link reference definition plus a bare `:` yields an empty definition list, and an \
         empty list is dropped when written",
    ),
];

struct SpecCase {
    number: usize,
    input: String,
    expected_html: String,
}

/// Splits the spec file into its examples.
///
/// The format is a long backtick fence opened with `example_deflists`, holding
/// the Markdown input and the reference HTML separated by a lone `.` line.
fn parse_spec(spec: &str) -> Vec<SpecCase> {
    let mut cases = Vec::new();
    let mut lines = spec.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed = line.trim_end();
        let is_open = trimmed.starts_with("````")
            && trimmed.trim_start_matches('`').trim() == "example_deflists";
        if !is_open {
            continue;
        }
        let fence = &trimmed[..trimmed.len() - trimmed.trim_start_matches('`').len()];

        let mut input = String::new();
        let mut expected_html = String::new();
        let mut past_separator = false;

        for body in lines.by_ref() {
            if body.trim_end() == fence {
                break;
            }
            if body == "." && !past_separator {
                past_separator = true;
                continue;
            }
            let target = if past_separator {
                &mut expected_html
            } else {
                &mut input
            };
            target.push_str(body);
            target.push('\n');
        }

        cases.push(SpecCase {
            number: cases.len() + 1,
            input,
            expected_html,
        });
    }

    cases
}

fn render_markdown(document: &Document) -> String {
    let mut output = Vec::new();
    markdown::write(&mut output, document).expect("failed to write Markdown");
    String::from_utf8(output).expect("Markdown output is not UTF-8")
}

fn count_definition_lists(paragraphs: &[Paragraph]) -> usize {
    paragraphs
        .iter()
        .map(|paragraph| match paragraph {
            Paragraph::DefinitionList { items } => {
                1 + items
                    .iter()
                    .map(|item| count_definition_lists(&item.definition))
                    .sum::<usize>()
            }
            Paragraph::Quote { children } => count_definition_lists(children),
            Paragraph::UnorderedList { entries } | Paragraph::OrderedList { entries } => {
                entries.iter().map(|e| count_definition_lists(e)).sum()
            }
            _ => 0,
        })
        .sum()
}

#[test]
fn spec_file_is_present_and_parses() {
    let cases = parse_spec(SPEC);
    assert_eq!(
        cases.len(),
        31,
        "unexpected number of spec cases — was tests/data/spec/definition_lists.txt updated?"
    );
    assert!(
        cases.iter().all(|case| !case.input.is_empty()),
        "every spec case must carry a Markdown input"
    );
    assert!(
        cases.iter().all(|case| !case.expected_html.is_empty()),
        "every spec case must carry reference HTML"
    );
}

/// Parses `input`, writes it back, and reports what (if anything) the
/// round-trip changed.
fn round_trip_failure(case: &SpecCase) -> Option<String> {
    let parsed = match markdown::parse(Cursor::new(case.input.as_bytes())) {
        Ok(document) => document,
        Err(err) => return Some(format!("case {}: failed to parse: {err}", case.number)),
    };

    let rendered = render_markdown(&parsed);

    let reparsed = match markdown::parse(Cursor::new(rendered.as_bytes())) {
        Ok(document) => document,
        Err(err) => return Some(format!("case {}: failed to reparse: {err}", case.number)),
    };

    if reparsed != parsed {
        return Some(format!(
            "case {}: round-trip changed the document\n  input:    {:?}\n  rendered: {:?}",
            case.number, case.input, rendered
        ));
    }

    // Writing a second time must be a fixed point: the canonical form the
    // writer emits has to survive re-rendering byte for byte.
    let rerendered = render_markdown(&reparsed);
    if rerendered != rendered {
        return Some(format!(
            "case {}: writer is not idempotent\n  first:  {:?}\n  second: {:?}",
            case.number, rendered, rerendered
        ));
    }

    None
}

#[test]
fn every_spec_case_parses_and_round_trips() {
    let cases = parse_spec(SPEC);
    let mut failures = Vec::new();

    for case in &cases {
        let known_gap = KNOWN_ROUND_TRIP_GAPS
            .iter()
            .find(|(number, _)| *number == case.number);
        let failure = round_trip_failure(case);

        match (known_gap, failure) {
            // A tracked gap that is still a gap: nothing to report, but say so
            // out loud rather than skipping in silence.
            (Some((number, reason)), Some(_)) => {
                eprintln!("skipping known round-trip gap, case {number}: {reason}");
            }
            (Some((number, reason)), None) => failures.push(format!(
                "case {number} now round-trips — remove it from KNOWN_ROUND_TRIP_GAPS \
                 (was: {reason})"
            )),
            (None, Some(failure)) => failures.push(failure),
            (None, None) => {}
        }
    }

    assert!(
        failures.is_empty(),
        "{} of {} spec cases failed:\n\n{}",
        failures.len(),
        cases.len(),
        failures.join("\n\n")
    );
}

#[test]
fn spec_cases_that_define_a_list_are_imported_as_one() {
    // The reference HTML tells us which cases are supposed to yield a `<dl>`.
    // Those must import as a definition list rather than degrading to text,
    // otherwise the importer silently stops recognizing the syntax.
    let cases = parse_spec(SPEC);
    let mut missing = Vec::new();

    for case in &cases {
        let expects_dl = case.expected_html.contains("<dl>");
        let parsed =
            markdown::parse(Cursor::new(case.input.as_bytes())).expect("failed to parse spec case");
        let found = count_definition_lists(&parsed.paragraphs) > 0;

        if expects_dl && !found {
            missing.push(format!(
                "case {}: reference HTML has a <dl> but tdoc produced none\n  input: {:?}",
                case.number, case.input
            ));
        }
    }

    assert!(
        missing.is_empty(),
        "{} spec cases lost their definition list:\n\n{}",
        missing.len(),
        missing.join("\n\n")
    );
}

#[test]
fn spec_markdown_output_snapshot() {
    let cases = parse_spec(SPEC);
    let mut report = String::new();

    for case in &cases {
        let parsed =
            markdown::parse(Cursor::new(case.input.as_bytes())).expect("failed to parse spec case");
        report.push_str(&format!("=== case {} ===\n", case.number));
        report.push_str("--- input ---\n");
        report.push_str(&case.input);
        report.push_str("--- tdoc markdown ---\n");
        report.push_str(&render_markdown(&parsed));
        report.push('\n');
    }

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("snapshots/markdown");
    settings.set_prepend_module_to_snapshot(false);
    settings.bind(|| {
        insta::assert_snapshot!("definition_lists_spec", report);
    });
}

#[test]
fn markdown_extra_fixture_html_snapshot() {
    // `markdown_import.rs` snapshots every Markdown fixture as FTML, which has
    // no `<dl>` and so flattens definition lists into paragraphs. Snapshot this
    // one through HTML as well, where the structure survives and a regression
    // in the importer would actually show up.
    let source = std::fs::read_to_string("tests/data/markdown/definition-lists-markdown-extra.md")
        .expect("missing Markdown Extra fixture");
    let document =
        markdown::parse(Cursor::new(source.as_bytes())).expect("failed to parse the fixture");

    assert!(
        count_definition_lists(&document.paragraphs) >= 5,
        "expected the Markdown Extra fixture to import several definition lists"
    );

    let mut rendered = Vec::new();
    tdoc::html::write(&mut rendered, &document).expect("failed to render HTML");
    let html = String::from_utf8(rendered).expect("HTML output is not UTF-8");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path("snapshots/markdown");
    settings.set_prepend_module_to_snapshot(false);
    settings.bind(|| {
        insta::assert_snapshot!("definition-lists-markdown-extra.html", html);
    });
}

#[test]
fn mdn_fixture_survives_an_html_round_trip() {
    // Every structural shape MDN documents for `<dl>` — shared descriptions,
    // several descriptions per term, `<div>`-wrapped groups — has to come back
    // unchanged through the HTML writer.
    let file = std::fs::File::open("tests/data/html/mdn-dl-element.html")
        .expect("missing MDN <dl> fixture");
    let document = tdoc::html::parse(file).expect("failed to parse the fixture");

    assert!(
        count_definition_lists(&document.paragraphs) >= 6,
        "expected the MDN fixture to import every one of its definition lists"
    );

    let mut rendered = Vec::new();
    tdoc::html::write(&mut rendered, &document).expect("failed to render HTML");

    let reparsed = tdoc::html::parse(Cursor::new(&rendered)).expect("failed to reparse HTML");
    assert_eq!(
        reparsed, document,
        "the MDN definition lists changed on an HTML round-trip"
    );
}

#[test]
fn spec_fixture_provenance_is_recorded() {
    // A third-party fixture without its licence and source recorded is a
    // liability; keep the credits file honest.
    let credits = std::fs::read_to_string(PathBuf::from("tests/data/CREDITS.md"))
        .expect("tests/data/CREDITS.md must exist");
    for fixture in [
        "spec/definition_lists.txt",
        "html/mdn-dl-element.html",
        "markdown/definition-lists-markdown-extra.md",
    ] {
        assert!(
            credits.contains(fixture),
            "tests/data/CREDITS.md does not record the origin of {fixture}"
        );
    }
}
