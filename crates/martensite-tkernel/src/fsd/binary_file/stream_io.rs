//! Binary file reads and writes through an [`IoStream`] instead of the driver's `FILE*`.
//! The writers return the byte size and, with `only_count`, write nothing.
use super::strings::bytes_from_units;
use super::{BinaryFile, FileHeader, input_fields, input_info, input_source::OwnedReadSource};
use crate::ncollection::Sequence;
use crate::standard::{Failure, Handle};
use crate::storage::{HeaderData, IoStream, StorageError, StreamException};
use crate::tcollection::{AsciiString, ExtendedString};

fn stream_write(stream: &mut IoStream, bytes: &[u8], member: &str) -> Result<(), Failure> {
    stream.write(bytes);
    if stream.fail() {
        return Err(StreamException::Write.raise(&format!(
            "{member}: the stream refused {} bytes",
            bytes.len()
        )));
    }
    Ok(())
}

impl BinaryFile {
    /// Writes one host-order integer and returns its size, 4; `BinaryFile::put_integer_stream(&mut out, 1, false)?`.
    pub fn put_integer_stream(
        stream: &mut IoStream,
        value: i32,
        only_count: bool,
    ) -> Result<i32, Failure> {
        if !only_count {
            stream_write(stream, &value.to_ne_bytes(), "FSD_BinaryFile::PutInteger")?;
        }
        Ok(4)
    }

    /// Reads one host-order integer: a short read keeps the bytes it got and is a
    /// `Storage_StreamTypeMismatchError`; `BinaryFile::get_integer_stream(&mut input, &mut v)?`.
    pub fn get_integer_stream(stream: &mut IoStream, value: &mut i32) -> Result<(), Failure> {
        input_fields::integer(&mut OwnedReadSource(stream), value)
    }

    /// Reads a reference number, encoded like an integer.
    /// Example: `BinaryFile::get_reference_stream(&mut input, &mut reference)?`.
    pub fn get_reference_stream(stream: &mut IoStream, value: &mut i32) -> Result<(), Failure> {
        input_fields::integer(&mut OwnedReadSource(stream), value)
    }

    /// Writes the length and the bytes, `4 + length` bytes.
    /// Example: `BinaryFile::write_string_stream(&mut out, &text, false)?`.
    pub fn write_string_stream(
        stream: &mut IoStream,
        text: &AsciiString,
        only_count: bool,
    ) -> Result<i32, Failure> {
        let size = Self::put_integer_stream(stream, text.length(), only_count)?;
        if !text.is_empty() && !only_count {
            stream_write(stream, text.as_bytes(), "FSD_BinaryFile::WriteString")?;
        }
        Ok(size + text.length())
    }

    /// Reads a length-prefixed string; the text stops at its
    /// first NUL; `BinaryFile::read_string_stream(&mut input, &mut text)?`.
    pub fn read_string_stream(
        stream: &mut IoStream,
        text: &mut AsciiString,
    ) -> Result<(), Failure> {
        input_fields::ascii(&mut OwnedReadSource(stream), text)
    }

    /// Writes the unit count and the host-order UTF-16 units.
    /// Example: `BinaryFile::write_extended_string_stream(&mut out, &text, false)?`.
    pub fn write_extended_string_stream(
        stream: &mut IoStream,
        text: &ExtendedString,
        only_count: bool,
    ) -> Result<i32, Failure> {
        let size = Self::put_integer_stream(stream, text.length(), only_count)?;
        if !text.is_empty() && !only_count {
            let bytes = bytes_from_units(text.as_utf16());
            stream_write(stream, &bytes, "FSD_BinaryFile::WriteExtendedString")?;
        }
        Ok(size + 2 * text.length())
    }

    /// Reads a length-prefixed UTF-16 string; the text stops
    /// at its first zero unit; `BinaryFile::read_extended_string_stream(&mut input, &mut text)?`.
    pub fn read_extended_string_stream(
        stream: &mut IoStream,
        text: &mut ExtendedString,
    ) -> Result<(), Failure> {
        input_fields::extended(&mut OwnedReadSource(stream), text)
    }

    /// Writes the 52-byte file header.
    /// Example: `BinaryFile::write_header_stream(&mut out, &header, false)?`.
    pub fn write_header_stream(
        stream: &mut IoStream,
        header: &FileHeader,
        only_count: bool,
    ) -> Result<i32, Failure> {
        let mut size = 0;
        for field in header.fields() {
            size += Self::put_integer_stream(stream, field, only_count)?;
        }
        Ok(size)
    }

    /// Reads the file header; a short read stops at that field.
    /// Example: `BinaryFile::read_header_stream(&mut input, &mut header)?`.
    pub fn read_header_stream(
        stream: &mut IoStream,
        header: &mut FileHeader,
    ) -> Result<(), Failure> {
        input_fields::header(&mut OwnedReadSource(stream), header)
    }
}

impl BinaryFile {
    /// Writes the info section and returns its byte size.
    /// Example: `BinaryFile::write_info_stream(&mut out, 0, &v, &d, &n, &sv, &an, &av, &dt, &info, false)?`.
    #[allow(clippy::too_many_arguments)]
    pub fn write_info_stream(
        stream: &mut IoStream,
        object_count: i32,
        storage_version: &AsciiString,
        date: &AsciiString,
        schema_name: &AsciiString,
        schema_version: &AsciiString,
        application_name: &ExtendedString,
        application_version: &AsciiString,
        data_type: &ExtendedString,
        user_info: &Sequence<AsciiString>,
        only_count: bool,
    ) -> Result<i32, Failure> {
        let mut size = Self::put_integer_stream(stream, object_count, only_count)?;
        for text in [storage_version, date, schema_name, schema_version] {
            size += Self::write_string_stream(stream, text, only_count)?;
        }
        size += Self::write_extended_string_stream(stream, application_name, only_count)?;
        size += Self::write_string_stream(stream, application_version, only_count)?;
        size += Self::write_extended_string_stream(stream, data_type, only_count)?;
        size += Self::put_integer_stream(stream, user_info.length(), only_count)?;
        for line in user_info.iter() {
            size += Self::write_string_stream(stream, line, only_count)?;
        }
        Ok(size)
    }

    /// Writes the comment section and returns its byte size.
    /// Example: `BinaryFile::write_comment_stream(&mut out, &comments, false)?`.
    pub fn write_comment_stream(
        stream: &mut IoStream,
        comments: &Sequence<ExtendedString>,
        only_count: bool,
    ) -> Result<i32, Failure> {
        let mut size = Self::put_integer_stream(stream, comments.length(), only_count)?;
        for comment in comments.iter() {
            size += Self::write_extended_string_stream(stream, comment, only_count)?;
        }
        Ok(size)
    }

    /// Reads the comment section, appending comments while the stream is good.
    /// Example: `BinaryFile::read_comment_stream(&mut input, &mut comments)?`.
    pub fn read_comment_stream(
        stream: &mut IoStream,
        comments: &mut Sequence<ExtendedString>,
    ) -> Result<(), Failure> {
        input_fields::comments(&mut OwnedReadSource(stream), comments)
    }

    /// Reads the info section into locals, then stores them in `header`.
    /// Example: `BinaryFile::read_header_data(&mut input, &header)?`.
    pub fn read_header_data(
        stream: &mut IoStream,
        header: &Handle<HeaderData>,
    ) -> Result<(), Failure> {
        input_info::header_data(&mut OwnedReadSource(stream), header)
    }
}

impl BinaryFile {
    /// Reads the type section size; `BinaryFile::type_section_size_stream(&mut input)?`.
    pub fn type_section_size_stream(stream: &mut IoStream) -> Result<i32, Failure> {
        input_fields::section_size(&mut OwnedReadSource(stream))
    }

    /// Reads one type entry: its number and its name.
    /// Example: `BinaryFile::read_type_informations_stream(&mut input, &mut n, &mut name)?`.
    pub fn read_type_informations_stream(
        stream: &mut IoStream,
        number: &mut i32,
        name: &mut AsciiString,
    ) -> Result<(), Failure> {
        input_fields::type_entry(&mut OwnedReadSource(stream), number, name)
    }

    /// Reads the root section size; `BinaryFile::root_section_size_stream(&mut input)?`.
    pub fn root_section_size_stream(stream: &mut IoStream) -> Result<i32, Failure> {
        input_fields::section_size(&mut OwnedReadSource(stream))
    }

    /// Reads one root entry: reference, name, type name.
    /// Example: `BinaryFile::read_root_stream(&mut input, &mut name, &mut r, &mut ty)?`.
    pub fn read_root_stream(
        stream: &mut IoStream,
        name: &mut AsciiString,
        reference: &mut i32,
        type_name: &mut AsciiString,
    ) -> Result<(), Failure> {
        input_fields::root_entry(&mut OwnedReadSource(stream), name, reference, type_name)
    }

    /// Reads the reference section size; `BinaryFile::ref_section_size_stream(&mut input)?`.
    pub fn ref_section_size_stream(stream: &mut IoStream) -> Result<i32, Failure> {
        input_fields::section_size(&mut OwnedReadSource(stream))
    }

    /// Reads one reference entry: its reference and type numbers.
    /// Example: `BinaryFile::read_reference_type_stream(&mut input, &mut r, &mut t)?`.
    pub fn read_reference_type_stream(
        stream: &mut IoStream,
        reference: &mut i32,
        type_number: &mut i32,
    ) -> Result<(), Failure> {
        input_fields::reference_entry(&mut OwnedReadSource(stream), reference, type_number)
    }

    /// Records `tellp()` as the info section end.
    /// Example: `driver.end_write_info_section_stream(&mut out)`.
    pub fn end_write_info_section_stream(&mut self, stream: &mut IoStream) -> StorageError {
        self.header.end_info = stream.tellp() as i32;
        StorageError::Ok
    }

    /// Records `tellp()` as the comment section start.
    /// Example: `driver.begin_write_comment_section_stream(&mut out)`.
    pub fn begin_write_comment_section_stream(&mut self, stream: &mut IoStream) -> StorageError {
        self.header.begin_comment = stream.tellp() as i32;
        StorageError::Ok
    }

    /// Records `tellp()` as the comment section end.
    /// Example: `driver.end_write_comment_section_stream(&mut out)`.
    pub fn end_write_comment_section_stream(&mut self, stream: &mut IoStream) -> StorageError {
        self.header.end_comment = stream.tellp() as i32;
        StorageError::Ok
    }
}
