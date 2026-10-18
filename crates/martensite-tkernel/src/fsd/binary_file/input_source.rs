//! Source policies keep owned diagnostics separate from byte decoding.
use super::type_mismatch;
use crate::standard::Failure;
use crate::storage::{IoStream, StreamException};

pub(super) trait BinaryReadSource {
    type Error: From<Failure>;
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), Self::Error>;
    fn count(&self) -> usize;
    fn good(&self) -> bool;
    fn eof(&self) -> bool;
    fn position(&mut self) -> Result<i32, Self::Error>;
    fn incomplete_integer(&self) -> Self::Error;
    fn incomplete_block(&self, member: &str, expected: usize) -> Self::Error;
}

pub(super) struct OwnedReadSource<'a>(pub(super) &'a mut IoStream);
impl BinaryReadSource for OwnedReadSource<'_> {
    type Error = Failure;
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), Failure> {
        self.0.read(bytes);
        Ok(())
    }
    fn count(&self) -> usize {
        self.0.gcount() as usize
    }
    fn good(&self) -> bool {
        self.0.good()
    }
    fn eof(&self) -> bool {
        self.0.eof()
    }
    fn position(&mut self) -> Result<i32, Failure> {
        Ok(self.0.tellg() as i32)
    }
    fn incomplete_integer(&self) -> Failure {
        type_mismatch("FSD_BinaryFile::GetInteger")
    }
    fn incomplete_block(&self, member: &str, _expected: usize) -> Failure {
        StreamException::Read.raise(&format!("{member}: the stream ended inside a string"))
    }
}
