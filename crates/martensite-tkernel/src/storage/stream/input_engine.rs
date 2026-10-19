//! Shared extraction mechanics; callers retain their state and publication policy.
use super::IoStream;
use std::io;

pub(crate) trait InputBytes {
    fn peek(&mut self) -> io::Result<Option<u8>>;
    fn consume(&mut self, count: usize);
    fn copy_prefix(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let Some(byte) = self.peek()? else {
            return Ok(0);
        };
        let Some(slot) = output.first_mut() else {
            return Ok(0);
        };
        *slot = byte;
        self.consume(1);
        Ok(1)
    }
}

pub(crate) trait LineOutput {
    fn has_room(&self, count: usize) -> bool;
    fn append(&mut self, byte: u8);
}

#[derive(Clone, Copy)]
pub(crate) enum LineEnd {
    Delimiter,
    Eof,
    Capacity,
}

pub(crate) fn scan_byte(
    source: &mut impl InputBytes,
    output: &mut u8,
    count: &mut usize,
) -> io::Result<()> {
    if let Some(byte) = source.peek()? {
        source.consume(1);
        *output = byte;
        *count = 1;
    }
    Ok(())
}

pub(crate) fn scan_line(
    source: &mut impl InputBytes,
    output: &mut impl LineOutput,
    delimiter: u8,
    count: &mut usize,
) -> io::Result<LineEnd> {
    loop {
        let Some(byte) = source.peek()? else {
            return Ok(LineEnd::Eof);
        };
        if byte == delimiter {
            source.consume(1);
            *count += 1;
            return Ok(LineEnd::Delimiter);
        }
        if !output.has_room(*count) {
            return Ok(LineEnd::Capacity);
        }
        output.append(byte);
        source.consume(1);
        *count += 1;
    }
}

pub(crate) fn scan_block(
    source: &mut impl InputBytes,
    output: &mut [u8],
    count: &mut usize,
) -> io::Result<()> {
    while *count < output.len() {
        let copied = source.copy_prefix(&mut output[*count..])?;
        if copied == 0 {
            break;
        }
        *count += copied;
    }
    Ok(())
}

// The owned adapter keeps its established policy: refill refusal is end-of-input.
// The shared scan therefore never receives a device error through this adapter (#142).
impl InputBytes for IoStream {
    fn peek(&mut self) -> io::Result<Option<u8>> {
        Ok(self.peek_byte())
    }
    fn consume(&mut self, count: usize) {
        self.cursor += count;
    }
}

pub(crate) struct VectorLine<'a> {
    pub(crate) bytes: &'a mut Vec<u8>,
    pub(crate) limit: i64,
}
impl LineOutput for VectorLine<'_> {
    fn has_room(&self, count: usize) -> bool {
        (count as i64) + 1 < self.limit
    }
    fn append(&mut self, byte: u8) {
        self.bytes.push(byte);
    }
}
