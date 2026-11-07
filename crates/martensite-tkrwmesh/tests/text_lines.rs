//! The lines of a comment.
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkrwmesh::rw_mesh::split_text_lines;

fn lines_of(text: &str) -> Vec<String> {
    split_text_lines(&AsciiString::new(text))
        .iter()
        .map(|line| String::from_utf8_lossy(line.as_bytes()).into_owned())
        .collect()
}

#[test]
fn every_kind_of_line_break_ends_a_line() {
    assert_eq!(lines_of("a\nb\r\nc\rd"), ["a", "b", "c", "d"]);
}

#[test]
fn a_one_letter_last_line_and_a_repeated_line_are_kept() {
    assert_eq!(lines_of("same\nsame\nx"), ["same", "same", "x"]);
}

#[test]
fn empty_lines_go_and_trailing_blanks_are_trimmed() {
    assert_eq!(lines_of("\n\nfirst  \n\n"), ["first"]);
    assert!(lines_of("").is_empty());
}
