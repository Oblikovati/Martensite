//! Field decoding is shared by source policies without duplicating the format.
use super::input_source::BinaryReadSource;
use super::strings::units_from_bytes;
use super::{FileHeader, header_input};
use crate::ncollection::Sequence;
use crate::tcollection::{AsciiString, ExtendedString};

pub(super) fn integer<I: BinaryReadSource>(input: &mut I, value: &mut i32) -> Result<(), I::Error> {
    header_input::read_integer(value, |bytes| {
        input.read(bytes)?;
        if input.count() != 4 {
            return Err(input.incomplete_integer());
        }
        Ok(())
    })
}

/// String reads fail in bounded steps so a corrupt length cannot allocate the whole size.
fn block<I: BinaryReadSource>(
    input: &mut I,
    size: usize,
    member: &str,
) -> Result<Vec<u8>, I::Error> {
    const STEP: usize = 1 << 16;
    if !input.good() {
        return Err(input.incomplete_block(member, size));
    }
    let mut bytes = Vec::new();
    while bytes.len() < size {
        let start = bytes.len();
        let step = (size - start).min(STEP);
        bytes.resize(start + step, 0);
        block_step(input, &mut bytes[start..], member)?;
    }
    Ok(bytes)
}

fn block_step<I: BinaryReadSource>(
    input: &mut I,
    bytes: &mut [u8],
    member: &str,
) -> Result<(), I::Error> {
    input.read(bytes)?;
    if input.count() != bytes.len() {
        return Err(input.incomplete_block(member, bytes.len()));
    }
    Ok(())
}

pub(super) fn ascii<I: BinaryReadSource>(
    input: &mut I,
    text: &mut AsciiString,
) -> Result<(), I::Error> {
    let mut size = 0;
    integer(input, &mut size)?;
    if size <= 0 {
        *text = AsciiString::default();
        return Ok(());
    }
    let bytes = block(input, size as usize, "FSD_BinaryFile::ReadString")?;
    *text = AsciiString::from_bytes(&bytes);
    Ok(())
}

pub(super) fn extended<I: BinaryReadSource>(
    input: &mut I,
    text: &mut ExtendedString,
) -> Result<(), I::Error> {
    let mut size = 0;
    integer(input, &mut size)?;
    if size <= 0 {
        *text = ExtendedString::default();
        return Ok(());
    }
    let bytes = block(
        input,
        size as usize * 2,
        "FSD_BinaryFile::ReadExtendedString",
    )?;
    *text = ExtendedString::from_utf16(&units_from_bytes(&bytes));
    Ok(())
}

pub(super) fn header<I: BinaryReadSource>(
    input: &mut I,
    fields: &mut FileHeader,
) -> Result<(), I::Error> {
    header_input::read_fields(fields, |field| integer(input, field))
}

pub(super) fn comments<I: BinaryReadSource>(
    input: &mut I,
    lines: &mut Sequence<ExtendedString>,
) -> Result<(), I::Error> {
    let mut length = 0;
    integer(input, &mut length)?;
    let mut line = ExtendedString::default();
    let mut index = 1;
    while index <= length && input.good() {
        extended(input, &mut line)?;
        lines.append(line.clone());
        index += 1;
    }
    Ok(())
}

pub(super) fn section_size<I: BinaryReadSource>(input: &mut I) -> Result<i32, I::Error> {
    let mut size = 0;
    integer(input, &mut size)?;
    Ok(size)
}

pub(super) fn type_entry<I: BinaryReadSource>(
    input: &mut I,
    number: &mut i32,
    name: &mut AsciiString,
) -> Result<(), I::Error> {
    integer(input, number)?;
    ascii(input, name)
}

pub(super) fn root_entry<I: BinaryReadSource>(
    input: &mut I,
    name: &mut AsciiString,
    reference: &mut i32,
    kind: &mut AsciiString,
) -> Result<(), I::Error> {
    integer(input, reference)?;
    ascii(input, name)?;
    ascii(input, kind)
}

pub(super) fn reference_entry<I: BinaryReadSource>(
    input: &mut I,
    reference: &mut i32,
    kind: &mut i32,
) -> Result<(), I::Error> {
    integer(input, reference)?;
    integer(input, kind)
}
