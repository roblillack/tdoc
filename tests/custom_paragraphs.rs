//! End-to-end tests for application-defined (custom) paragraph types.

use std::io::Cursor;
use tdoc::custom::{builtins::Image, CustomRegistry, CustomRenderOptions, CustomType};
use tdoc::formatter::Formatter;
use tdoc::{ftml, gemini, html, markdown};
use tdoc::{CustomParagraph, Document, Paragraph, ParagraphType, Span};

fn image_registry() -> CustomRegistry {
    CustomRegistry::new().register(Image)
}

// --- FTML stays strict: it does not model custom paragraphs ------------------

#[test]
fn ftml_does_not_capture_unknown_tags() {
    // FTML is a strict, closed subset of HTML5; unknown elements are not
    // captured as custom paragraphs (they are simply skipped).
    let doc = ftml::parse(Cursor::new("<email-signature></email-signature>")).unwrap();
    assert!(doc.is_empty());
}

#[test]
fn ftml_writer_salvages_custom_content_as_paragraph() {
    // A content-bearing custom paragraph degrades to a plain `<p>`…
    let note = CustomParagraph::new("note").with_content(vec![Span::new_text("hello")]);
    let doc = Document::new().with_paragraphs(vec![Paragraph::Custom(note)]);
    let out = ftml::Writer::new().write_to_string(&doc).unwrap();
    assert_eq!(out, "<p>hello</p>\n");
}

#[test]
fn ftml_writer_omits_content_less_custom() {
    // …and a content-less custom (e.g. an image) has no FTML representation.
    let doc = image_document();
    let out = ftml::Writer::new().write_to_string(&doc).unwrap();
    assert_eq!(out, "");
}

// --- Markdown: standalone image preservation --------------------------------

#[test]
fn markdown_preserves_standalone_image_as_custom() {
    let registry = image_registry();
    let doc = markdown::parse_with(Cursor::new("![Logo](logo.png)\n"), &registry).unwrap();

    assert_eq!(doc.paragraphs.len(), 1);
    let custom = doc.paragraphs[0].custom().expect("image custom");
    assert_eq!(custom.kind, "image");
    assert_eq!(custom.attribute("src"), Some("logo.png"));
    assert_eq!(custom.attribute("alt"), Some("Logo"));

    let mut out = Vec::new();
    markdown::write_with(&mut out, &doc, &registry).unwrap();
    assert_eq!(
        String::from_utf8(out).unwrap().trim_end(),
        "![Logo](logo.png)"
    );
}

#[test]
fn markdown_inline_image_stays_a_link_span() {
    // An image mixed with surrounding text is not the sole content of its
    // paragraph, so it keeps the legacy inline-link behavior.
    let registry = image_registry();
    let doc = markdown::parse_with(Cursor::new("see ![x](b.png) here\n"), &registry).unwrap();

    assert_eq!(doc.paragraphs[0].paragraph_type(), ParagraphType::Text);
    assert!(doc.paragraphs[0].custom().is_none());
}

#[test]
fn markdown_without_registry_keeps_legacy_image_behavior() {
    let doc = markdown::parse(Cursor::new("![Logo](logo.png)\n")).unwrap();
    assert_eq!(doc.paragraphs[0].paragraph_type(), ParagraphType::Text);
}

// --- Cross-format serialization via a handler -------------------------------

fn image_document() -> Document {
    let image = CustomParagraph::new("image")
        .with_attribute("src", "logo.png")
        .with_attribute("alt", "Logo");
    Document::new().with_paragraphs(vec![Paragraph::Custom(image)])
}

#[test]
fn image_custom_serializes_across_permissive_formats() {
    let registry = image_registry();
    let doc = image_document();

    // HTML is a genuine superset and represents the image as `<img>`.
    let mut html_out = Vec::new();
    html::write_with(&mut html_out, &doc, &registry).unwrap();
    assert_eq!(
        String::from_utf8(html_out).unwrap(),
        "<img src=\"logo.png\" alt=\"Logo\" />\n"
    );

    // Gemini represents it as a link line.
    let mut gemini_out = Vec::new();
    gemini::write_with(&mut gemini_out, &doc, &registry).unwrap();
    assert_eq!(String::from_utf8(gemini_out).unwrap(), "=> logo.png Logo\n");

    // FTML, being strict, has no image representation: the paragraph is omitted
    // (and the registry has no effect on FTML output).
    let ftml_out = ftml::Writer::new()
        .with_custom_registry(registry)
        .write_to_string(&doc)
        .unwrap();
    assert_eq!(ftml_out, "");
}

// --- Terminal formatter ------------------------------------------------------

#[test]
fn formatter_uses_registered_renderer() {
    let mut out = Vec::new();
    Formatter::new_ascii(&mut out)
        .with_custom_registry(image_registry())
        .write_document(&image_document())
        .unwrap();
    assert_eq!(String::from_utf8(out).unwrap(), "[image: Logo]\n");
}

#[test]
fn formatter_falls_back_to_placeholder_without_handler() {
    // No registry: the custom paragraph has no inline content, so a `[kind]`
    // placeholder is emitted rather than dropping it.
    let mut out = Vec::new();
    Formatter::new_ascii(&mut out)
        .write_document(&image_document())
        .unwrap();
    assert_eq!(String::from_utf8(out).unwrap(), "[image]\n");
}

#[test]
fn formatter_falls_back_to_inline_content() {
    let custom = CustomParagraph::new("note").with_content(vec![Span::new_text("hello")]);
    let doc = Document::new().with_paragraphs(vec![Paragraph::Custom(custom)]);

    let mut out = Vec::new();
    Formatter::new_ascii(&mut out).write_document(&doc).unwrap();
    assert_eq!(String::from_utf8(out).unwrap(), "hello\n");
}

// --- Custom render options are threaded through -----------------------------

#[test]
fn render_options_report_width_and_ansi() {
    struct Probe;
    impl CustomType for Probe {
        fn kind(&self) -> &str {
            "probe"
        }
        fn render(&self, _p: &CustomParagraph, o: &CustomRenderOptions) -> Option<Vec<String>> {
            Some(vec![format!("w={} ansi={}", o.width, o.ansi)])
        }
    }

    let doc = Document::new().with_paragraphs(vec![Paragraph::new_custom("probe")]);

    let mut ascii = Vec::new();
    Formatter::new_ascii(&mut ascii)
        .with_custom_registry(CustomRegistry::new().register(Probe))
        .write_document(&doc)
        .unwrap();
    assert_eq!(String::from_utf8(ascii).unwrap(), "w=72 ansi=false\n");

    let mut ansi = Vec::new();
    Formatter::new_ansi(&mut ansi)
        .with_custom_registry(CustomRegistry::new().register(Probe))
        .write_document(&doc)
        .unwrap();
    assert!(String::from_utf8(ansi).unwrap().contains("ansi=true"));
}
