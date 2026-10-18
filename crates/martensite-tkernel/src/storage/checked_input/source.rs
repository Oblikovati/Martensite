//! Borrow caller buffers, with position available only through an explicit capability.
use std::io::{self, BufRead, Seek};

/// Buffered source capability; implement `buffer()` for a custom borrowed source.
pub trait InputBuffer {
    /// Access non-consuming lookahead; `source.buffer().fill_buf()?`.
    fn buffer(&mut self) -> &mut dyn BufRead;
    /// Query an actual position or report no capability; `source.position()?`.
    fn position(&mut self) -> io::Result<Option<u64>> {
        Ok(None)
    }
}
/// Borrow a buffered reader without position requirements; `BufferedInput(&mut bytes)`.
/// This erased adapter is thread confined.
pub struct BufferedInput<'a>(pub &'a mut dyn BufRead);
impl InputBuffer for BufferedInput<'_> {
    fn buffer(&mut self) -> &mut dyn BufRead {
        self.0
    }
}

/// A buffer with an actual position; `Cursor<Vec<u8>>` implements this capability.
pub trait PositionedBuffer: BufRead + Seek {}
impl<T: BufRead + Seek> PositionedBuffer for T {}
/// Borrow a buffer with actual position; `PositionedInput(&mut cursor)`.
/// This erased adapter is thread confined.
pub struct PositionedInput<'a>(pub &'a mut dyn PositionedBuffer);
impl InputBuffer for PositionedInput<'_> {
    fn buffer(&mut self) -> &mut dyn BufRead {
        self.0
    }
    fn position(&mut self) -> io::Result<Option<u64>> {
        self.0.stream_position().map(Some)
    }
}
