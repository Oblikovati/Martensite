//! A named shared-memory source controls only actual device faults.
use martensite_tkernel::osd::{IStreamBuffer, IosOpenMode, SeekDir, StreamBuf, share_stream_buf};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

#[derive(Default)]
pub(super) struct FaultControl {
    pub refuse_rewind: AtomicBool,
    pub panic_rewind: AtomicBool,
    pub panic_read: AtomicBool,
}
#[derive(Debug)]
pub(super) struct ReadPanicked;
#[derive(Debug)]
pub(super) struct RewindPanicked;
struct ObservedMemoryInput {
    bytes: Cursor<Vec<u8>>,
    faults: Arc<FaultControl>,
}
impl StreamBuf for ObservedMemoryInput {
    fn sgetn(&mut self, bytes: &mut [u8]) -> usize {
        if self.faults.panic_read.swap(false, Ordering::SeqCst) {
            let count = bytes.len().min(2);
            self.bytes.read_exact(&mut bytes[..count]).unwrap();
            std::panic::panic_any(ReadPanicked);
        }
        self.bytes.read(bytes).unwrap()
    }
    fn sputn(&mut self, bytes: &[u8]) -> usize {
        let written: usize = self.bytes.write(bytes).unwrap();
        written
    }
    fn pubseekoff(&mut self, offset: i64, direction: SeekDir, _: IosOpenMode) -> i64 {
        if offset < 0 && self.faults.panic_rewind.load(Ordering::SeqCst) {
            std::panic::panic_any(RewindPanicked);
        }
        if offset < 0 && self.faults.refuse_rewind.load(Ordering::SeqCst) {
            return -1;
        }
        self.seek_position(offset, direction)
    }
    fn pubsync(&mut self) -> i32 {
        let synchronized: i32 = 0;
        synchronized
    }
}
impl ObservedMemoryInput {
    fn seek_position(&mut self, offset: i64, direction: SeekDir) -> i64 {
        let Some(target) = seek_target(offset, direction) else {
            return -1;
        };
        self.bytes.seek(target).map_or(-1, |position| position as i64)
    }
}
fn seek_target(offset: i64, direction: SeekDir) -> Option<SeekFrom> {
    match direction {
        SeekDir::Beg => u64::try_from(offset).ok().map(SeekFrom::Start),
        SeekDir::Cur => Some(SeekFrom::Current(offset)),
        SeekDir::End => Some(SeekFrom::End(offset)),
    }
}
/// Create one owned buffer, with no file-system or whole-input adapter; `input(b"abc")`.
pub(super) fn input(bytes: &[u8]) -> (IStreamBuffer, Arc<FaultControl>) {
    let faults = Arc::new(FaultControl::default());
    let device = ObservedMemoryInput {
        bytes: Cursor::new(bytes.to_vec()),
        faults: Arc::clone(&faults),
    };
    let stream = IStreamBuffer::new("owned-memory", share_stream_buf(device));
    (stream, faults)
}
