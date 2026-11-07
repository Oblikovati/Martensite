//! A `std::stringbuf`-like fake behind the `istringstream` and `ostringstream` of the tests.
use martensite_tkernel::osd::{
    IStreamBuffer, IosOpenMode, OStreamBuffer, SeekDir, StreamBuf, share_stream_buf,
};
use std::sync::{Arc, Mutex, PoisonError};

/// The bytes of an output stream, as `ostringstream::str()` shows them.
#[derive(Clone, Default)]
pub struct WrittenBytes(Arc<Mutex<Vec<u8>>>);
impl WrittenBytes {
    /// A copy of everything written so far.
    pub fn contents(&self) -> Vec<u8> {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

struct MemoryStreamBuf {
    bytes: WrittenBytes,
    position: usize,
}
impl MemoryStreamBuf {
    fn stored(&self) -> std::sync::MutexGuard<'_, Vec<u8>> {
        self.bytes.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
impl StreamBuf for MemoryStreamBuf {
    fn sgetn(&mut self, out: &mut [u8]) -> usize {
        let position = self.position;
        let stored = self.stored();
        let count = out.len().min(stored.len() - position);
        out[..count].copy_from_slice(&stored[position..position + count]);
        drop(stored);
        self.position += count;
        count
    }
    fn sputn(&mut self, bytes: &[u8]) -> usize {
        let position = self.position;
        let mut stored = self.stored();
        stored.truncate(position);
        stored.extend_from_slice(bytes);
        let end = stored.len();
        drop(stored);
        self.position = end;
        bytes.len()
    }
    fn pubseekoff(&mut self, offset: i64, direction: SeekDir, _which: IosOpenMode) -> i64 {
        let end = self.stored().len() as i64;
        let base = match direction {
            SeekDir::Beg => 0,
            SeekDir::Cur => self.position as i64,
            SeekDir::End => end,
        };
        let target = base + offset;
        if target < 0 || target > end {
            return -1;
        }
        self.position = target as usize;
        target
    }
    fn pubsync(&mut self) -> i32 {
        0
    }
}

/// `std::istringstream(bytes)`.
pub fn memory_istream(bytes: &[u8]) -> IStreamBuffer {
    let stored = WrittenBytes(Arc::new(Mutex::new(bytes.to_vec())));
    let buffer = MemoryStreamBuf {
        bytes: stored,
        position: 0,
    };
    IStreamBuffer::new("memory", share_stream_buf(buffer))
}

/// `std::ostringstream()`, with the handle `str()` reads through.
pub fn memory_ostream() -> (OStreamBuffer, WrittenBytes) {
    let written = WrittenBytes::default();
    let buffer = MemoryStreamBuf {
        bytes: written.clone(),
        position: 0,
    };
    (
        OStreamBuffer::new("memory", share_stream_buf(buffer)),
        written,
    )
}
