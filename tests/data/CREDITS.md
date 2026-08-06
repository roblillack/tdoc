# Test fixture provenance

Some fixtures under `tests/data/` are third-party documents, kept verbatim or
lightly adapted so that tdoc is exercised against real-world markup rather than
hand-written samples only. This file records where each one came from and under
what licence, so the provenance stays with the repository.

Fixtures are **not** shipped in the published crate: `Cargo.toml` lists `tests/`
under `exclude`, so nothing here is redistributed via crates.io.

When adding a third-party fixture, add a row below with its source URL, the
licence it is under, and the date it was retrieved. If a document was trimmed or
restructured to make a useful fixture, say so in the notes.

## Definition lists

| Fixture | Source | Licence | Retrieved |
| --- | --- | --- | --- |
| `spec/definition_lists.txt` | [pulldown-cmark `pulldown-cmark/specs/definition_lists.txt`](https://github.com/pulldown-cmark/pulldown-cmark/blob/master/pulldown-cmark/specs/definition_lists.txt) | MIT | 2026-08-06 |
| `html/mdn-dl-element.html` | [MDN Web Docs, `<dl>: The Description List element`](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dl) | Code samples: [CC0 1.0](https://developer.mozilla.org/en-US/docs/MDN/Writing_guidelines/Attrib_copyright_license); prose: [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/) | 2026-08-06 |
| `markdown/definition-lists-markdown-extra.md` | [Michel Fortin, *PHP Markdown Extra*, "Definition Lists"](https://michelf.ca/projects/php-markdown/extra/#def-list) | Documentation for [PHP Markdown](https://github.com/michelf/php-markdown), which is [BSD-3-Clause](https://github.com/michelf/php-markdown/blob/lib/License.md); the page itself carries no explicit licence notice | 2026-08-06 |

### Notes

- **`spec/definition_lists.txt`** is verbatim. It is pulldown-cmark's own
  conformance suite for definition lists, which matters because pulldown-cmark
  *is* tdoc's Markdown engine — the file therefore defines exactly the dialect
  tdoc inherits, including the deliberate omission of the `~` marker. Its 31
  cases are driven by `tests/definition_lists.rs`. The suite is in turn derived
  from [commonmark-hs](https://github.com/jgm/commonmark-hs) (BSD-3-Clause), as
  the file header records.
- **`html/mdn-dl-element.html`** collects the HTML code samples from the MDN
  page into a single standalone document, with short linking prose adapted from
  the same page. The samples were chosen because between them they cover every
  structural shape the parser has to handle: one term with one description,
  several terms sharing a description, one term with several descriptions, a
  key-value metadata list, and name-value groups wrapped in `<div>` elements.
- **`markdown/definition-lists-markdown-extra.md`** reproduces the Markdown
  examples from the "Definition Lists" section of the PHP Markdown Extra
  documentation, which is the syntax's normative description, arranged into one
  document with headings. Note that some examples exercise behaviour
  pulldown-cmark does not implement (notably several terms sharing one
  definition); the snapshots record what tdoc actually does, not what Markdown
  Extra would do.
