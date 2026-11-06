//! The lines of a text that a mesh format writes as comments.
use martensite_tkernel::tcollection::AsciiString;

/// Splits text at each line break into right-trimmed lines, each kept however often it repeats.
/// An empty line is dropped; `split_text_lines(&AsciiString::new("a\nb"))` has two lines.
pub fn split_text_lines(text: &AsciiString) -> Vec<AsciiString> {
    let mut lines = Vec::new();
    let bytes = text.as_bytes();
    let (mut from, mut index) = (0, 0);
    while index < bytes.len() {
        if matches!(bytes[index], b'\r' | b'\n') {
            add_line(&mut lines, &bytes[from..index]);
            if bytes[index] == b'\r' && bytes.get(index + 1) == Some(&b'\n') {
                index += 1;
            }
            from = index + 1;
        }
        index += 1;
    }
    add_line(&mut lines, &bytes[from..]);
    lines
}

fn add_line(lines: &mut Vec<AsciiString>, line: &[u8]) {
    if !line.is_empty() {
        let mut line = AsciiString::from_raw(line);
        line.right_adjust();
        lines.push(line);
    }
}
