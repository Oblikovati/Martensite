//! `RwStlReader::read_ascii`, with the C string helpers of its line parser.
use super::RwStlReader;
use super::merge_tool::ReaderMergeTool;
use martensite_tkernel::{
    message::{Message, ProgressRange, ProgressScope},
    osd::IStream,
    standard::{Failure, ReadLineBuffer, strtod_bytes},
    tcollection::AsciiString,
};
use martensite_tkmath::gp::Xyz;
use std::io::{self, Read};

const PROGRESS_STEP_BYTES: i64 = 1024 * 1024;

/// `std::istream::read` for the line buffer: a short read keeps the stream's eof and fail bits,
/// which `RwStlReader::Read` then tests.
struct StreamLines<'a> {
    stream: &'a mut dyn IStream,
    buffer: &'a mut ReadLineBuffer,
}
struct StreamBytes<'a>(&'a mut dyn IStream);
impl Read for StreamBytes<'_> {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        let count = self.0.read(out);
        if self.0.bad() {
            return Err(io::Error::other("bad STL input stream"));
        }
        Ok(count)
    }
}
impl StreamLines<'_> {
    /// The next line up to its first NUL, as the C string reads it.
    fn next_line(&mut self) -> Option<&[u8]> {
        let mut bytes = StreamBytes(&mut *self.stream);
        let line = self.buffer.read_line(&mut bytes)?;
        let end = line.iter().position(|byte| *byte == 0);
        Some(&line[..end.unwrap_or(line.len())])
    }
}

/// What the lines of one facet held.
enum FacetLines {
    Vertices([Xyz; 3]),
    /// `endsolid`, or the end of data where a vertex line was due: reading ends without failure.
    Stop,
    Failed,
}

pub(super) fn read_ascii_facets<R: RwStlReader + ?Sized>(
    reader: &R,
    stream: &mut dyn IStream,
    buffer: &mut ReadLineBuffer,
    until_pos: i64,
    progress: &ProgressRange<'_>,
) -> Result<bool, Failure> {
    let start = stream.tellg();
    let mut lines = StreamLines { stream, buffer };
    if !skip_solid_header(&mut lines) {
        send_premature_end();
        return Ok(false);
    }
    let mut tool = ReaderMergeTool::new(reader, -1);
    let nb_steps = 1 + ((until_pos - start) / PROGRESS_STEP_BYTES) as i32;
    let scope = ProgressScope::with_literal(
        progress,
        "Reading text STL file",
        f64::from(nb_steps),
        false,
    );
    read_facets(&mut tool, &mut lines, start + PROGRESS_STEP_BYTES, &scope)
}

/// Skips `solid ...` and any empty lines before it; false at the end of data.
fn skip_solid_header(lines: &mut StreamLines<'_>) -> bool {
    loop {
        match lines.next_line() {
            None => return false,
            Some([]) => continue,
            Some(_) => return true,
        }
    }
}

fn read_facets<R: RwStlReader + ?Sized>(
    tool: &mut ReaderMergeTool<'_, R>,
    lines: &mut StreamLines<'_>,
    mut progress_pos: i64,
    scope: &ProgressScope<'_>,
) -> Result<bool, Failure> {
    let mut nb_line = 1;
    while scope.more() {
        if lines.stream.tellg() > progress_pos {
            drop(scope.next(1.0));
            progress_pos += PROGRESS_STEP_BYTES;
        }
        match read_facet_lines(lines, nb_line) {
            FacetLines::Failed => return Ok(false),
            FacetLines::Stop => break,
            FacetLines::Vertices(vertices) => {
                tool.add_triangle(&vertices)?;
                nb_line += 7;
            }
        }
        // "endloop" and "endfacet" are not checked.
        let _ = lines.next_line();
        let _ = lines.next_line();
    }
    Ok(scope.more())
}

fn read_facet_lines(lines: &mut StreamLines<'_>, nb_line: i32) -> FacetLines {
    let Some(facet) = lines.next_line() else {
        send_premature_end();
        return FacetLines::Failed;
    };
    if starts_with_word(facet, b"endsolid") {
        return FacetLines::Stop;
    }
    if !starts_with_word(facet, b"facet") {
        send_facet_format_failure(nb_line);
        return FacetLines::Failed;
    }
    if !lines
        .next_line()
        .is_some_and(|outer| starts_with_word(outer, b"outer"))
    {
        send_facet_format_failure(nb_line);
        return FacetLines::Failed;
    }
    read_vertex_lines(lines, nb_line)
}

fn read_vertex_lines(lines: &mut StreamLines<'_>, nb_line: i32) -> FacetLines {
    let mut vertices = [Xyz::new(0., 0., 0.); 3];
    for vertex in &mut vertices {
        // A well-formed file never ends on a vertex line, so this is an early end, not an error.
        let Some(line) = lines.next_line() else {
            return FacetLines::Stop;
        };
        let Some(read) = parse_vertex(line) else {
            Message::send_fail_text(&AsciiString::new(&format!(
                "Error: cannot read vertex coordinates at line {nb_line}"
            )));
            return FacetLines::Failed;
        };
        *vertex = read;
    }
    FacetLines::Vertices(vertices)
}

fn send_premature_end() {
    Message::send_fail_text(&AsciiString::new("Error: premature end of file"));
}

fn send_facet_format_failure(nb_line: i32) {
    Message::send_fail_text(&AsciiString::new(&format!(
        "Error: unexpected format of facet at line {}",
        nb_line + 1
    )));
}

/// C-locale `isspace`; `u8::is_ascii_whitespace` leaves out the vertical tab.
pub(super) const fn is_c_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | 11 | 12 | b'\r')
}

/// `str_starts_with`: the word after any leading space, without case.
fn starts_with_word(line: &[u8], word: &[u8]) -> bool {
    let start = line.iter().position(|byte| !is_c_space(*byte));
    let text = &line[start.unwrap_or(line.len())..];
    text.get(..word.len())
        .is_some_and(|head| head.eq_ignore_ascii_case(word))
}

/// `read_vertex`: skips the keyword and reads three numbers. Tests only the third
/// conversion, so a line whose first numbers fail also fails at the same text position.
fn parse_vertex(line: &[u8]) -> Option<Xyz> {
    let keyword = line
        .iter()
        .position(|byte| !is_c_space(*byte) && !byte.is_ascii_alphabetic());
    let mut text = &line[keyword.unwrap_or(line.len())..];
    let mut read_number = || {
        let (value, consumed) = strtod_bytes(text);
        text = &text[consumed..];
        (value, consumed)
    };
    let (x, _) = read_number();
    let (y, _) = read_number();
    let (z, consumed) = read_number();
    (consumed != 0).then(|| Xyz::new(x, y, z))
}
