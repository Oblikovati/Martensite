//! A stream owning a shared `std::streambuf`-like buffer and remembering the
//! URL it was opened from. The stream direction (input, output, both)
//! is a marker type; the directions are distinct types, so an
//! `IoStreamBuffer` is not an `IStreamBuffer` for the file systems' checks.
use super::{IStream, OStream, SharedStreamBuf, StreamAccess, StreamIo};
use std::marker::PhantomData;

/// Marker for an input stream buffer.
#[derive(Debug)]
pub struct InputDirection;
/// Marker for an output stream buffer.
#[derive(Debug)]
pub struct OutputDirection;
/// Marker for an input/output stream buffer.
#[derive(Debug)]
pub struct InputOutputDirection;

/// A stream buffer with its URL; `IStreamBuffer::new("file.brep", buffer).url()`.
pub struct StreamBuffer<D> {
    url: String,
    io: StreamIo,
    direction: PhantomData<fn() -> D>,
}
/// An input stream buffer.
pub type IStreamBuffer = StreamBuffer<InputDirection>;
/// An output stream buffer.
pub type OStreamBuffer = StreamBuffer<OutputDirection>;
/// An input/output stream buffer.
pub type IoStreamBuffer = StreamBuffer<InputOutputDirection>;

impl<D> StreamBuffer<D> {
    /// Wraps the shared buffer and keeps the URL; `IStreamBuffer::new("a.step", buffer)`.
    pub fn new(url: impl Into<String>, buffer: SharedStreamBuf) -> Self {
        Self {
            url: url.into(),
            io: StreamIo::new(buffer),
            direction: PhantomData,
        }
    }
    /// The URL given at construction; `stream.url()`.
    pub fn url(&self) -> &str {
        &self.url
    }
}

impl<D: 'static> StreamAccess for StreamBuffer<D> {
    fn stream_io(&self) -> &StreamIo {
        &self.io
    }
    fn stream_io_mut(&mut self) -> &mut StreamIo {
        &mut self.io
    }
}
impl IStream for IStreamBuffer {}
impl IStream for IoStreamBuffer {}
impl OStream for OStreamBuffer {}
impl OStream for IoStreamBuffer {}
