//! Parsing a directory path with the light XML parser must not fail hard.
//!
//! On Unix a directory opens as an empty stream: `parse_file` reports success (it answers false), and
//! the document has no root element. The summary prints `Document parsed` and then
//! `Root element is null`; the test asserts that outcome on a temporary directory.
use martensite_tkcdf::ldom::LdomParser;

/// The parse summary up to its first answer: the lines it prints for a document without
/// root element.
fn parse_summary(file: &[u8]) -> Vec<&'static str> {
    let mut parser = LdomParser::new();
    if parser.parse_file(file) {
        return vec!["Document not parsed"];
    }
    let root = parser.get_document().get_document_element();
    if root.is_null() {
        return vec!["Document parsed", "Root element is null"];
    }
    vec!["Document parsed", "Root element"]
}

#[cfg(unix)]
#[test]
fn directory_input_parses_to_a_document_without_root() {
    let directory = std::env::temp_dir();
    let lines = parse_summary(directory.to_string_lossy().as_bytes());
    assert_eq!(lines, ["Document parsed", "Root element is null"]);
}

/// Windows cannot open a directory as a file, so the parse fails cleanly instead.
#[cfg(windows)]
#[test]
fn directory_input_is_refused_without_a_hard_failure() {
    let directory = std::env::temp_dir();
    let lines = parse_summary(directory.to_string_lossy().as_bytes());
    assert_eq!(lines, ["Document not parsed"]);
}
