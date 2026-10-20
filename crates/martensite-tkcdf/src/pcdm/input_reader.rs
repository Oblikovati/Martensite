//! An `io::Read` view of borrowed input: staged bytes first, then one native get per call.
use super::{PcdmInputStream, PcdmStreamFailure};
use std::io;

/// Reads what a staged container read fetched ahead, then the live borrowed input.
/// A stream exception is kept and the read reports an I/O error; `finish` returns it.
/// Example: `let mut reader = PcdmInputReader::new(&mut input, Vec::new());`.
pub(crate) struct PcdmInputReader<'s, 'a> {
    input: &'s mut PcdmInputStream<'a>,
    ahead: Vec<u8>,
    cursor: usize,
    thrown: Option<PcdmStreamFailure>,
}

impl<'s, 'a> PcdmInputReader<'s, 'a> {
    /// Serve `ahead` before the live input; `ahead` is empty for an unstaged stream.
    pub(crate) fn new(input: &'s mut PcdmInputStream<'a>, ahead: Vec<u8>) -> Self {
        Self {
            input,
            ahead,
            cursor: 0,
            thrown: None,
        }
    }

    /// The exception a read raised, if any; `reader.finish()?`.
    pub(crate) fn finish(self) -> Result<(), PcdmStreamFailure> {
        self.thrown.map_or(Ok(()), Err)
    }

    /// The input's state bits after the reads so far.
    pub(crate) fn state(&self) -> u8 {
        self.input.state()
    }

    /// The bytes extracted so far, including the staged ones served.
    pub(crate) fn extracted(&self) -> usize {
        self.input.extracted()
    }

    fn serve_ahead(&mut self, output: &mut [u8]) -> usize {
        let staged = &self.ahead[self.cursor..];
        let count = staged.len().min(output.len());
        output[..count].copy_from_slice(&staged[..count]);
        self.cursor += count;
        self.input.count_staged(count);
        count
    }
}

impl io::Read for PcdmInputReader<'_, '_> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        if output.is_empty() || self.thrown.is_some() {
            return Ok(0);
        }
        if self.cursor < self.ahead.len() {
            return Ok(self.serve_ahead(output));
        }
        match self.input.next_byte() {
            Ok(Some(byte)) => {
                output[0] = byte;
                Ok(1)
            }
            Ok(None) => Ok(0),
            Err(failure) => {
                let message = failure.to_string();
                self.thrown = Some(failure);
                Err(io::Error::other(message))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn staged_bytes_precede_single_byte_live_reads() {
        let mut source: &[u8] = b"cd";
        let mut input = PcdmInputStream::new(&mut source);
        let mut reader = PcdmInputReader::new(&mut input, b"ab".to_vec());
        let mut output = [0; 4];
        assert_eq!(reader.read(&mut output).unwrap(), 2);
        assert_eq!(&output[..2], b"ab");
        assert_eq!(reader.read(&mut output).unwrap(), 1);
        assert_eq!(output[0], b'c');
        assert_eq!(reader.extracted(), 3);
    }

    #[test]
    fn end_of_live_input_sets_eof_and_fail_without_exception() {
        let mut source: &[u8] = b"";
        let mut input = PcdmInputStream::new(&mut source);
        let mut reader = PcdmInputReader::new(&mut input, Vec::new());
        assert_eq!(reader.read(&mut [0; 1]).unwrap(), 0);
        assert_eq!(reader.state(), 6);
        assert!(reader.finish().is_ok());
    }
}
