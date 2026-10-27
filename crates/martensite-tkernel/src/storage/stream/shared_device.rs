//! Byte-device access to the same caller-owned shared buffer.
use crate::osd::{IosOpenMode, SeekDir, SharedStreamBuf, StreamBuf};
use std::io::{self, Read, Seek, SeekFrom, Write};

/// The `std::streambuf` of an opened OSD stream, as the byte device of a storage
/// stream: reads are `sgetn`, writes `sputn`, positions `pubseekoff` on the side
/// (`in` or `out`) the stream was opened for, and `flush` is `pubsync`.
pub(super) struct StreamBufDevice {
    buffer: SharedStreamBuf,
    side: IosOpenMode,
}

impl StreamBufDevice {
    /// Retain one shared buffer and seek side; `StreamBufDevice::new(buffer, side)`.
    pub(super) fn new(buffer: SharedStreamBuf, side: IosOpenMode) -> Self {
        let device: Self = Self { buffer, side };
        device
    }

    fn with_buffer<R>(&self, action: impl FnOnce(&mut dyn StreamBuf) -> R) -> R {
        let mut buffer = self
            .buffer
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        action(&mut *buffer)
    }
}

impl Read for StreamBufDevice {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        let received = self.with_buffer(|buffer| buffer.sgetn(out));
        Ok(received)
    }
}

impl Write for StreamBufDevice {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let written = self.with_buffer(|buffer| buffer.sputn(bytes));
        if written == 0 && !bytes.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "streambuf refused the bytes",
            ));
        }
        Ok(written)
    }
    fn flush(&mut self) -> io::Result<()> {
        match self.with_buffer(|buffer| buffer.pubsync()) {
            -1 => Err(io::Error::other("streambuf pubsync failed")),
            _ => Ok(()),
        }
    }
}

impl Seek for StreamBufDevice {
    fn seek(&mut self, target: SeekFrom) -> io::Result<u64> {
        let (offset, direction) = match target {
            SeekFrom::Start(offset) => (offset as i64, SeekDir::Beg),
            SeekFrom::Current(offset) => (offset, SeekDir::Cur),
            SeekFrom::End(offset) => (offset, SeekDir::End),
        };
        let side = self.side;
        let position = self.with_buffer(|buffer| buffer.pubseekoff(offset, direction, side));
        u64::try_from(position)
            .map_err(|_| io::Error::other(format!("streambuf refused the position {target:?}")))
    }
}

impl super::IoStream {
    /// Wrap a shared buffer without changing its owning stream's flags; `IoStream::with_shared_buffer(buffer, IosOpenMode::IN, true, false)`.
    /// Reading/writing permissions belong to this stream; side selects its seek position.
    pub fn with_shared_buffer(
        buffer: SharedStreamBuf,
        side: IosOpenMode,
        readable: bool,
        writable: bool,
    ) -> Self {
        let device = StreamBufDevice::new(buffer, side);
        Self::with_device(Box::new(device), readable, writable)
    }
}
