//! The reading of a file record by record: fixed widths, the end of lines they may or may not have,
//! the enciphered form, and the section each record belongs to.
use super::structure::{IgesFileStructure, c_text};
use std::io::Read;

/// The size of a record buffer, and of what is read from a stream at once.
pub(super) const RECORD_SIZE: usize = 100;
const INPUT_SIZE: usize = 4096;

/// What the stream of records gave.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum IgesFileRead {
    /// The end of the data.
    End,
    /// The stream failed.
    Error,
    /// The record has no trailer that says its section.
    BadTrailer,
    /// A record of the section numbered from 1 for start to 5 for terminate.
    Section(i32),
}

/// A record buffer: a C text of fixed room.
pub(super) type IgesFileRecord = [u8; RECORD_SIZE];

/// The buffered bytes of a stream.
pub(super) struct IgesFileInput<'a> {
    stream: &'a mut dyn Read,
    buffer: Vec<u8>,
    offset: usize,
    size: usize,
    pending: Option<u8>,
}

impl<'a> IgesFileInput<'a> {
    pub(super) fn new(stream: &'a mut dyn Read) -> Self {
        Self {
            stream,
            buffer: vec![0; INPUT_SIZE],
            offset: 0,
            size: 0,
            pending: None,
        }
    }
    /// The next byte, `None` at the end; an error of the stream is an `Err`.
    fn read_byte(&mut self) -> Result<Option<u8>, ()> {
        if let Some(byte) = self.pending.take() {
            return Ok(Some(byte));
        }
        if self.offset == self.size {
            let read = loop {
                match self.stream.read(&mut self.buffer) {
                    Ok(read) => break read,
                    Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(_) => return Err(()),
                }
            };
            if read == 0 {
                return Ok(None);
            }
            self.offset = 0;
            self.size = read;
        }
        let byte = self.buffer[self.offset];
        self.offset += 1;
        Ok(Some(byte))
    }
    fn discard_line_remainder(&mut self) -> Result<(), ()> {
        if self.pending.is_none() {
            return Ok(());
        }
        while let Some(byte) = self.read_byte()? {
            if byte == b'\r' || byte == b'\n' {
                return Ok(());
            }
        }
        Ok(())
    }
    /// Reads one record of at most 80 characters. End of lines are skipped before it; after it a
    /// byte that is not one starts the next record, for files without end of lines. `None` at the
    /// end of the data.
    fn read_record(&mut self, record: &mut IgesFileRecord) -> Result<Option<usize>, ()> {
        record.fill(0);
        let first = loop {
            match self.read_byte()? {
                None => return Ok(None),
                Some(b'\r' | b'\n') => {}
                Some(byte) => break byte,
            }
        };
        record[0] = first;
        let mut length = 1;
        while length < 80 {
            match self.read_byte()? {
                None | Some(b'\n') => return Ok(Some(length)),
                Some(byte) => {
                    record[length] = byte;
                    length += 1;
                }
            }
        }
        match self.read_byte()? {
            None | Some(b'\n') => {}
            Some(b'\r') => {
                if let Some(byte) = self.read_byte()?
                    && byte != b'\n'
                {
                    self.pending = Some(byte);
                }
            }
            Some(byte) => self.pending = Some(byte),
        }
        Ok(Some(length))
    }
}

/// Restores the decimal point lost from a record of 79 columns that starts with a real written
/// with a `D` exponent.
fn recover_shifted_real(record: &mut IgesFileRecord, length: usize) {
    if length != 79 || !record[0].is_ascii_digit() {
        return;
    }
    let exponent = record[..length]
        .iter()
        .position(|byte| !byte.is_ascii_digit())
        .unwrap_or(length);
    if exponent >= length || !matches!(record[exponent], b'D' | b'd') {
        return;
    }
    record.copy_within(0..=length, 1);
    record[0] = b'.';
}

/// Deciphers a record whose first byte has its high bit up; true when it did.
fn decode_fnes(record: &mut IgesFileRecord, is_fnes: bool) -> bool {
    if !is_fnes || record[0] & 128 == 0 {
        return false;
    }
    for (index, byte) in record.iter_mut().take(80).enumerate() {
        *byte ^= 150 + (index & 3) as u8;
    }
    true
}

/// The number a C text starts with, after blanks, and whether there was one.
fn parse_number(text: &[u8]) -> Option<i32> {
    let start = text
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(text.len());
    let rest = &text[start..];
    let (negative, digits) = match rest.first() {
        Some(b'-') => (true, &rest[1..]),
        Some(b'+') => (false, &rest[1..]),
        _ => (false, rest),
    };
    let count = digits
        .iter()
        .take_while(|byte| byte.is_ascii_digit())
        .count();
    if count == 0 {
        return None;
    }
    let mut value: i64 = 0;
    for digit in &digits[..count] {
        value = value.wrapping_mul(10).wrapping_add(i64::from(digit - b'0'));
    }
    Some(if negative { -value } else { value } as i32)
}

fn section_of(mark: u8) -> Option<i32> {
    match mark {
        b'S' => Some(1),
        b'G' => Some(2),
        b'D' => Some(3),
        b'P' => Some(4),
        b'T' => Some(5),
        _ => None,
    }
}

/// Reads a record and finds its section and its number in it; `record` ends before its trailer.
pub(super) fn read_section_record(
    input: &mut IgesFileInput<'_>,
    section_number: &mut i32,
    record: &mut IgesFileRecord,
    is_fnes: bool,
) -> IgesFileRead {
    match read_decoded(input, section_number, record, is_fnes) {
        Err(()) => IgesFileRead::Error,
        Ok(false) => IgesFileRead::End,
        Ok(true) => {
            if c_text(record.as_slice()).contains(&0x1A) {
                return IgesFileRead::End;
            }
            trailer_of(section_number, record)
        }
    }
}

/// Reads a record, deciphered, past a plain line before the first one; false at the end of data.
fn read_decoded(
    input: &mut IgesFileInput<'_>,
    section_number: &i32,
    record: &mut IgesFileRecord,
    is_fnes: bool,
) -> Result<bool, ()> {
    let Some(length) = input.read_record(record)? else {
        return Ok(false);
    };
    let encoded = decode_fnes(record, is_fnes);
    recover_shifted_real(record, length);
    let is_preamble = *section_number == 0
        && record[72] != b'S'
        && !encoded
        && (is_fnes || length < 80 || record[79] == b' ');
    if !is_preamble {
        return Ok(true);
    }
    input.discard_line_remainder()?;
    let Some(length) = input.read_record(record)? else {
        return Ok(false);
    };
    decode_fnes(record, is_fnes);
    recover_shifted_real(record, length);
    Ok(true)
}

/// The section of a record from its trailer, in its columns or, when the padding before it was
/// omitted, at the end of the text.
fn trailer_of(section_number: &mut i32, record: &mut IgesFileRecord) -> IgesFileRead {
    if let Some(number) = parse_number(c_text(&record[73..])) {
        *section_number = number;
        if let Some(section) = section_of(record[72]) {
            record[72] = 0;
            return IgesFileRead::Section(section);
        }
    }
    let mut length = c_text(record.as_slice()).len();
    while length > 0 && record[length - 1] == b' ' {
        length -= 1;
    }
    let mut number_start = length;
    while number_start > 0 && record[number_start - 1].is_ascii_digit() {
        number_start -= 1;
    }
    if number_start == length {
        return IgesFileRead::BadTrailer;
    }
    let Some(number) = parse_number(c_text(&record[number_start..])) else {
        return IgesFileRead::BadTrailer;
    };
    *section_number = number;
    let mut mark = number_start;
    while mark > 0 && record[mark - 1] == b' ' {
        mark -= 1;
    }
    if mark == 0 {
        return IgesFileRead::BadTrailer;
    }
    match section_of(record[mark - 1]) {
        Some(section) => {
            record[mark - 1] = 0;
            IgesFileRead::Section(section)
        }
        None => IgesFileRead::BadTrailer,
    }
}

/// An integer of a fixed-width field: its digits from the right, up to a blank.
fn decode_field(line: &[u8], start: usize, length: usize) -> i32 {
    const POWERS: [i32; 10] = [
        1,
        10,
        100,
        1000,
        10000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000_000,
    ];
    let mut value: i32 = 0;
    for index in 0..length {
        match line[start + length - 1 - index] {
            b' ' => return value,
            b'+' | b'0' => {}
            b'-' => value = -value,
            byte => {
                let digit = i32::from(byte as i8) - i32::from(b'0');
                value = value.wrapping_add(digit.wrapping_mul(POWERS[index]));
            }
        }
    }
    value
}

fn text_field(line: &[u8], start: usize) -> [u8; 9] {
    let mut text = [0; 9];
    text[..8].copy_from_slice(&line[start..start + 8]);
    text
}

/// Parses one line of a directory entry; `status` tells which: 0 the first, 1 the second.
pub(super) fn directory_line(
    data: &mut IgesFileStructure,
    status: &mut i32,
    number: i32,
    line: &[u8],
) {
    if *status == 0 {
        data.new_part(number);
        if let Some(part) = data.current_part_mut() {
            for rank in 0..8 {
                part.values[rank] = decode_field(line, rank * 8, 8);
            }
            for rank in 0..4 {
                part.values[8 + rank] = decode_field(line, 64 + rank * 2, 2);
            }
        }
        *status = 1;
    } else if *status == 1 {
        if let Some(part) = data.current_part_mut() {
            for rank in 0..5 {
                part.values[12 + rank] = decode_field(line, rank * 8, 8);
            }
            part.res1 = text_field(line, 40);
            part.res2 = text_field(line, 48);
            part.nom = text_field(line, 56);
            part.num = text_field(line, 64);
        }
        *status = 0;
    }
}
