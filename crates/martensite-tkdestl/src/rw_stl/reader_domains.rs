//! `RwStlReader::Read` and `RwStlReader::is_ascii`.
use super::reader_ascii::is_c_space;
use super::{RwStlReader, read_binary_mode};
use martensite_tkernel::{
    message::{Message, ProgressRange, ProgressScope},
    osd::{IStream, SeekDir, default_file_system},
    standard::{Failure, ReadLineBuffer},
    tcollection::AsciiString,
};
use std::sync::PoisonError;

/// 80 bytes of header, 4 of facet count and 50 of one facet.
const MIN_BINARY_FILE_BYTES: usize = 134;
pub(super) const LINE_BUFFER_BYTES: usize = 1024;

pub(super) fn read_file_domains<R: RwStlReader + ?Sized>(
    reader: &R,
    file: &AsciiString,
    progress: &ProgressRange<'_>,
) -> Result<bool, Failure> {
    let Some(opened) = default_file_system().open_istream(file, read_binary_mode(), 0, None)?
    else {
        send_file_not_found(file);
        return Ok(false);
    };
    let mut guard = opened.lock().unwrap_or_else(PoisonError::into_inner);
    let stream: &mut dyn IStream = &mut *guard;
    let end = stream_end(stream);
    // Probing a file too short to be binary could fail the stream at its end.
    let is_ascii = (end as usize) < MIN_BINARY_FILE_BYTES || reader.is_ascii(stream, true);
    read_stream_domains(reader, stream, is_ascii, end, progress)
}

fn send_file_not_found(file: &AsciiString) {
    let name = String::from_utf8_lossy(file.as_bytes());
    Message::send_fail_text(&AsciiString::new(&format!(
        "Error: file '{name}' is not found"
    )));
}

/// The position after a seek to the end, then back to the start; feeds text-mode progress.
pub(super) fn stream_end(stream: &mut dyn IStream) -> i64 {
    stream.seekg(0, SeekDir::End);
    let end = stream.tellg();
    stream.seekg(0, SeekDir::Beg);
    end
}

/// Reads concatenated domains on a logarithmic progress scale, so the first takes most of it.
fn read_stream_domains<R: RwStlReader + ?Sized>(
    reader: &R,
    stream: &mut dyn IStream,
    is_ascii: bool,
    end: i64,
    progress: &ProgressRange<'_>,
) -> Result<bool, Failure> {
    let mut lines = ReadLineBuffer::new(LINE_BUFFER_BYTES);
    let scope = ProgressScope::unnamed(progress, 1.0, true);
    let mut reached_end = false;
    while !reached_end && stream.good() {
        let range = scope.next(2.0);
        let done = if is_ascii {
            reader.read_ascii(stream, &mut lines, end, &range)?
        } else {
            reader.read_binary(stream, &range)?
        };
        if !done {
            break;
        }
        reached_end = skip_stream_space(stream);
        reader.add_solid()?;
    }
    Ok(!stream.fail())
}

/// `*stream >> std::ws`, true when it reached the end of data. `ws` sets only eofbit there,
/// which this stream interface cannot express, so the bits are cleared and the caller's loop
/// ends on the result: `Read` opened this stream, so nothing else observes the difference.
fn skip_stream_space(stream: &mut dyn IStream) -> bool {
    if !stream.good() {
        // The sentry of `ws`: a stream that is not good gets failbit.
        stream.read(&mut []);
        return false;
    }
    let mut byte = [0; 1];
    loop {
        if stream.read(&mut byte) == 0 {
            stream.clear();
            return true;
        }
        if !is_c_space(byte[0]) {
            stream.seekg(-1, SeekDir::Cur);
            return false;
        }
    }
}

pub(super) fn head_is_ascii(stream: &mut dyn IStream, is_seek_available: bool) -> bool {
    let mut head = [0; MIN_BINARY_FILE_BYTES];
    let nb_read = stream.read(&mut head);
    if stream.fail() {
        Message::send_fail_text(&AsciiString::new("Error: Cannot read file"));
        return true;
    }
    if is_seek_available {
        stream.seekg(0, SeekDir::Beg);
    } else {
        stream.seekg(-(nb_read as i64), SeekDir::Cur);
    }
    // A binary file can start with "solid " too, so only a byte beyond '~' proves it.
    head[..nb_read].iter().all(|byte| *byte <= b'~')
}
