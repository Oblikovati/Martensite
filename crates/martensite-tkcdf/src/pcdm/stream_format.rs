//! `PCDM_ReadWriter::FileFormat(stream, data)` over owned storage streams and borrowed input.
use super::file_driver_probe::{StreamDriverKind, container_driver, probe_driver};
use super::{PcdmInputStream, PcdmStreamFailure, input_reader::PcdmInputReader};
use super::{binary_stream::read_binary_information, read_writer::FILE_FORMAT, xml_header};
use martensite_tkernel::{
    standard::{Failure, Handle},
    storage::{Data, DriverBase, IoStream},
    tcollection::ExtendedString,
};
use std::io;

/// A probed stream format and the staged bytes that precede the live input.
pub(crate) struct ProbedFormat {
    pub(crate) format: ExtendedString,
    pub(crate) ahead: Vec<u8>,
}

/// Probe borrowed input: driver type, then the container header or the XML document tag.
pub(crate) fn probe_input_format(
    input: &mut PcdmInputStream<'_>,
    storage: &mut Handle<Data>,
) -> Result<ProbedFormat, PcdmStreamFailure> {
    let kind = probe_driver(input, &mut None).map_err(PcdmStreamFailure::in_file_format)?;
    let mut ahead = Vec::new();
    let format = match kind {
        StreamDriverKind::Text | StreamDriverKind::Compatible => format_from_header(storage),
        StreamDriverKind::Binary => {
            ahead = read_binary_information(input, storage)
                .map_err(PcdmStreamFailure::in_file_format)?;
            format_from_header(storage)
        }
        StreamDriverKind::Xml => xml_input_format(input)?,
        StreamDriverKind::Unknown => ExtendedString::default(),
    };
    Ok(ProbedFormat { format, ahead })
}

fn xml_input_format(input: &mut PcdmInputStream<'_>) -> Result<ExtendedString, PcdmStreamFailure> {
    if input.state() != 0 {
        return Ok(ExtendedString::default());
    }
    let mut reader = PcdmInputReader::new(input, Vec::new());
    let format = xml_header::stream_format(&mut reader);
    reader.finish().map_err(PcdmStreamFailure::in_file_format)?;
    Ok(format)
}

pub(super) fn file_format_storage(
    stream: &mut IoStream,
    storage: &mut Handle<Data>,
) -> Result<ExtendedString, Failure> {
    let magic = DriverBase::read_magic_number(stream);
    let Some((_, mut driver)) = container_driver(magic.as_bytes()) else {
        return Ok(noncontainer_format(stream, magic.as_bytes()));
    };
    driver
        .read_complete_info(stream, storage)
        .map_err(native_stream_failure)?;
    Ok(format_from_header(storage))
}

/// An XML stream skips its declaration and reads the document tag; others are unknown.
fn noncontainer_format(stream: &mut IoStream, magic: &[u8]) -> ExtendedString {
    if !magic.windows(5).any(|part| part == b"<?xml") {
        return ExtendedString::default();
    }
    let mut character = b' ';
    while stream.good() && !stream.eof() && character != b'>' {
        stream.get(&mut character);
    }
    if !stream.good() {
        return ExtendedString::default();
    }
    xml_header::stream_format(&mut StorageStreamBytes(stream))
}

/// One `get` per read, so the parser consumes the owned stream exactly as natively.
struct StorageStreamBytes<'s>(&'s mut IoStream);

impl io::Read for StorageStreamBytes<'_> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let mut byte = 0;
        if output.is_empty() || !self.0.good() || self.0.get(&mut byte).gcount() != 1 {
            return Ok(0);
        }
        output[0] = byte;
        Ok(1)
    }
}

pub(super) fn native_stream_failure(mut failure: Failure) -> Failure {
    // These released-driver diagnostics enrich the source's empty constructor payload.
    // Preserve arbitrary typed causes; only these exact intrinsic paths are normalized.
    if intrinsic_empty_payload(&failure) {
        failure.set_message("");
    }
    failure
}

fn intrinsic_empty_payload(failure: &Failure) -> bool {
    matches!(
        (failure.exception_type(), failure.what()),
        (
            "Storage_StreamTypeMismatchError",
            b"FSD_BinaryFile::GetInteger: expected more bytes in the file"
        ) | (
            "Storage_StreamReadError",
            b"FSD_BinaryFile::ReadString: the stream ended inside a string"
        ) | (
            "Storage_StreamReadError",
            b"FSD_BinaryFile::ReadExtendedString: the stream ended inside a string"
        )
    )
}

// FSD_File::ReadCompleteInfo is empty, and FSD_CmpFile inherits it. Inspect the
// existing header directly; adapting/copying the borrowed stream would change its effects.
pub(super) fn format_from_header(storage: &Handle<Data>) -> ExtendedString {
    let Some(storage) = storage.get() else {
        return ExtendedString::default();
    };
    let mut format = ExtendedString::default();
    for line in storage.user_info().iter() {
        if line.search(FILE_FORMAT.as_bytes()) != -1 {
            format = ExtendedString::from_ascii_string(&line.token(b" ", 2), true);
        }
    }
    format
}

#[cfg(test)]
#[path = "stream_format_tests.rs"]
mod tests;
