use martensite_tkernel::standard::Failure;
use std::io::{self, BufRead, Read, Seek, SeekFrom};

pub(crate) struct FailingBufferedInput {
    pub bytes: Vec<u8>,
    pub cursor: usize,
    pub failure_at: Option<usize>,
    pub fail_position: bool,
}
impl FailingBufferedInput {
    pub(crate) fn new(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
            cursor: 0,
            failure_at: None,
            fail_position: false,
        }
    }
    fn refusal() -> io::Error {
        io::Error::other(Failure::with_type(
            "InputQuotaExceeded",
            "quota 0, expected positive capacity",
            "owned stack",
        ))
    }
}
impl BufRead for FailingBufferedInput {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.failure_at == Some(self.cursor) {
            return Err(Self::refusal());
        }
        Ok(&self.bytes[self.cursor..(self.cursor + 1).min(self.bytes.len())])
    }
    fn consume(&mut self, amount: usize) {
        assert!(amount <= 1);
        self.cursor += amount;
    }
}
impl Read for FailingBufferedInput {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let available = self.fill_buf()?;
        let count = output.len().min(available.len());
        output[..count].copy_from_slice(&available[..count]);
        self.consume(count);
        Ok(count)
    }
}
impl Seek for FailingBufferedInput {
    fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
        assert_eq!(position, SeekFrom::Current(0));
        if self.fail_position {
            return Err(Self::refusal());
        }
        Ok(self.cursor as u64)
    }
}
