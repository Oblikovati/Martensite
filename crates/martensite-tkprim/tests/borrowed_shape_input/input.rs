//! Existing shared-memory files supply bytes; one named policy refuses seeks only.
use super::fakes::MemoryFileSystem;
use martensite_tkernel::osd::{
    FileSystem, IStreamBuffer, IosOpenMode, SeekDir, SharedStreamBuf, StreamAccess, StreamBuf,
    share_stream_buf,
};
struct NonSeekingBuffer {
    inner: SharedStreamBuf,
}
impl StreamBuf for NonSeekingBuffer {
    fn sgetn(&mut self, bytes: &mut [u8]) -> usize {
        let count: usize = self.inner.lock().unwrap().sgetn(bytes);
        count
    }
    fn sputn(&mut self, bytes: &[u8]) -> usize {
        let count: usize = self.inner.lock().unwrap().sputn(bytes);
        count
    }
    fn pubseekoff(&mut self, _offset: i64, _direction: SeekDir, _side: IosOpenMode) -> i64 {
        let refused: i64 = -1;
        refused
    }
    fn pubsync(&mut self) -> i32 {
        let flushed: i32 = self.inner.lock().unwrap().pubsync();
        flushed
    }
}
/// Retain the original shared byte owner after the factory returns; `source(bytes, true)`.
pub(super) fn source(bytes: &[u8], seekable: bool) -> IStreamBuffer {
    let files = MemoryFileSystem::default();
    files.insert("owned.input", bytes.to_vec());
    let buffer = files
        .open_stream_buffer(&"owned.input".into(), IosOpenMode::IN, 0, None)
        .unwrap()
        .unwrap();
    let selected = if seekable {
        buffer
    } else {
        share_stream_buf(NonSeekingBuffer { inner: buffer })
    };
    IStreamBuffer::new("owned.input", selected)
}
/// Query the actual buffer without failed-stream sentry effects; `position(&source)`.
pub(super) fn position(source: &IStreamBuffer) -> i64 {
    let buffer = source.rdbuf();
    buffer
        .lock()
        .unwrap()
        .pubseekoff(0, SeekDir::Cur, IosOpenMode::IN)
}
