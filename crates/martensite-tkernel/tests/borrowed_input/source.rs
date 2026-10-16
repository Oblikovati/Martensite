//! Borrow caller buffers, with position available only through an explicit capability.
use std::io::{self, BufRead, Seek};

pub(crate) trait InputBuffer {
    fn buffer(&mut self) -> &mut dyn BufRead;
    fn position(&mut self) -> io::Result<Option<u64>> {
        Ok(None)
    }
}
pub(crate) struct BufferedInput<'a>(pub &'a mut dyn BufRead);
impl InputBuffer for BufferedInput<'_> {
    fn buffer(&mut self) -> &mut dyn BufRead {
        self.0
    }
}

pub(crate) trait PositionedBuffer: BufRead + Seek {}
impl<T: BufRead + Seek> PositionedBuffer for T {}
pub(crate) struct PositionedInput<'a>(pub &'a mut dyn PositionedBuffer);
impl InputBuffer for PositionedInput<'_> {
    fn buffer(&mut self) -> &mut dyn BufRead {
        self.0
    }
    fn position(&mut self) -> io::Result<Option<u64>> {
        self.0.stream_position().map(Some)
    }
}
