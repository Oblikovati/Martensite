//! Borrowed, unbuffered input with the state observed by the header probe.
use super::PcdmStreamFailure;
use martensite_tkernel::{storage::state, tcollection::AsciiString};
use std::{io, sync::Arc};

/// Borrow a byte reader while retaining stream state and the original device error.
/// This adapter is thread-confined because its borrowed `dyn Read` need not be `Send`.
/// Example: `let input = PcdmInputStream::new(&mut reader); assert_eq!(input.state(), 0);`.
pub struct PcdmInputStream<'a> {
    reader: &'a mut dyn io::Read,
    state: u8,
    exception_mask: u8,
    cause: Option<Arc<io::Error>>,
    extracted: usize,
}

impl<'a> PcdmInputStream<'a> {
    /// Use the default mask, which records device failures without throwing.
    /// Example: `let input = PcdmInputStream::new(&mut reader);`.
    pub fn new(reader: &'a mut dyn io::Read) -> Self {
        Self::with_exception_mask(reader, 0)
    }

    /// Enable exceptions for the supplied storage stream state bits.
    /// Example: `let input = PcdmInputStream::with_exception_mask(&mut reader, state::BAD);`.
    pub fn with_exception_mask(reader: &'a mut dyn io::Read, exception_mask: u8) -> Self {
        Self {
            reader,
            state: 0,
            exception_mask,
            cause: None,
            extracted: 0,
        }
    }

    /// Return the storage stream state bits, including EOF, FAIL and BAD.
    /// Example: `assert_eq!(input.state(), 0);`.
    pub fn state(&self) -> u8 {
        self.state
    }

    /// Retain access to the actual read failure even when the default mask swallowed it.
    /// Example: `assert!(input.last_error().is_none());`.
    pub fn last_error(&self) -> Option<&io::Error> {
        self.cause.as_deref()
    }

    pub(super) fn read_magic_number(&mut self) -> Result<AsciiString, PcdmStreamFailure> {
        let (mut count, mut character) = (0, 0_u8);
        let mut magic = AsciiString::default();
        while self.state == 0 && count < 7 {
            if let Some(byte) = self.extract_byte(count)? {
                character = byte;
                count += 1;
            }
            // ReadMagicNumber appends the previous character after a default-mask EOF.
            magic.assign_cat(character);
        }
        Ok(magic)
    }

    pub(super) fn consume_xml_declaration(&mut self) -> Result<(), PcdmStreamFailure> {
        let mut consumed = 7;
        while self.state == 0 {
            let Some(byte) = self.extract_byte(consumed)? else {
                break;
            };
            consumed += 1;
            if byte == b'>' {
                break;
            }
        }
        Ok(())
    }

    fn extract_byte(&mut self, consumed: usize) -> Result<Option<u8>, PcdmStreamFailure> {
        let mut output = [0];
        match self.reader.read(&mut output) {
            Ok(1) => {
                self.extracted += 1;
                Ok(Some(output[0]))
            }
            Ok(_) => self.end_of_input(consumed).map(|_| None),
            Err(cause) => self.device_failed(cause, consumed).map(|_| None),
        }
    }

    /// Total bytes extracted from the borrowed reader at the logical position.
    pub(crate) fn extracted(&self) -> usize {
        self.extracted
    }

    /// Count staged bytes a reader adapter served after a staged read settled.
    pub(super) fn count_staged(&mut self, count: usize) {
        self.extracted += count;
    }

    /// Extract one byte for a reader adapter; a stream that is not good extracts nothing.
    pub(super) fn next_byte(&mut self) -> Result<Option<u8>, PcdmStreamFailure> {
        if self.state != 0 {
            return Ok(None);
        }
        self.extract_byte(self.extracted)
    }

    /// Fetch one byte past the logical position without changing the stream state.
    /// Staged reads use it; `settle_staged` later applies the state at the logical end.
    pub(super) fn fetch_ahead(&mut self) -> io::Result<Option<u8>> {
        let mut output = [0];
        loop {
            match self.reader.read(&mut output) {
                Ok(0) => return Ok(None),
                Ok(_) => return Ok(Some(output[0])),
                Err(cause) if cause.kind() == io::ErrorKind::Interrupted => {}
                Err(cause) => return Err(cause),
            }
        }
    }

    /// Apply a staged read: `consumed` logical bytes and the state bits it ended with.
    /// A device error met at the logical end becomes `badbit`, as for a direct get.
    pub(super) fn settle_staged(
        &mut self,
        consumed: usize,
        bits: u8,
        device_error: Option<io::Error>,
    ) -> Result<(), PcdmStreamFailure> {
        self.extracted = consumed;
        if let Some(cause) = device_error {
            return self.device_failed(cause, consumed).map(drop);
        }
        if bits & state::EOF != 0 {
            return self.end_of_input(consumed).map(drop);
        }
        if bits & state::FAIL != 0 {
            return self.finish_failed_get(consumed).map(drop);
        }
        Ok(())
    }

    fn end_of_input(&mut self, consumed: usize) -> Result<bool, PcdmStreamFailure> {
        self.state |= state::EOF | state::FAIL;
        if self.state & self.exception_mask != 0 {
            let cause = io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "basic_ios::clear: iostream error",
            );
            return Err(PcdmStreamFailure::new(
                Arc::new(cause),
                consumed,
                self.state,
            ));
        }
        Ok(false)
    }

    fn device_failed(
        &mut self,
        cause: io::Error,
        consumed: usize,
    ) -> Result<bool, PcdmStreamFailure> {
        let cause = Arc::new(cause);
        self.cause = Some(cause.clone());
        self.state |= state::BAD;
        if self.exception_mask & state::BAD != 0 {
            return Err(PcdmStreamFailure::new(cause, consumed, self.state));
        }
        self.finish_failed_get(consumed)
    }

    fn finish_failed_get(&mut self, consumed: usize) -> Result<bool, PcdmStreamFailure> {
        self.state |= state::FAIL;
        if self.exception_mask & state::FAIL != 0 {
            let cause = io::Error::other("basic_ios::clear: iostream error");
            return Err(PcdmStreamFailure::new(
                Arc::new(cause),
                consumed,
                self.state,
            ));
        }
        Ok(false)
    }
}
