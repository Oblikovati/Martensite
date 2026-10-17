//! Named byte-device observations for output-position contracts.
use martensite_tkernel::storage::IoStream;
use std::{
    cell::RefCell,
    io::{self, Cursor, Read, Seek, SeekFrom, Write},
    rc::Rc,
};

pub struct SeekObservations {
    pub cursor: Cursor<Vec<u8>>,
    pub requests: Vec<SeekFrom>,
}
pub struct OutputSeekFixture {
    pub stream: IoStream,
    pub observations: Rc<RefCell<SeekObservations>>,
}
impl OutputSeekFixture {
    pub fn new(bytes: &[u8], writable: bool, refused: Option<SeekFrom>) -> Self {
        let observations = Rc::new(RefCell::new(SeekObservations {
            cursor: Cursor::new(bytes.to_vec()),
            requests: Vec::new(),
        }));
        let device = RecordingSeekDevice {
            observations: observations.clone(),
            refused,
        };
        Self {
            stream: IoStream::with_device(Box::new(device), true, writable),
            observations,
        }
    }
}
struct RecordingSeekDevice {
    observations: Rc<RefCell<SeekObservations>>,
    refused: Option<SeekFrom>,
}
impl Read for RecordingSeekDevice {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        self.observations.borrow_mut().cursor.read(bytes)
    }
}
impl Write for RecordingSeekDevice {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.observations.borrow_mut().cursor.write(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl Seek for RecordingSeekDevice {
    fn seek(&mut self, target: SeekFrom) -> io::Result<u64> {
        let mut observed = self.observations.borrow_mut();
        observed.requests.push(target);
        if self.refused == Some(target) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("seek target={target:?} refused; expected an allowed device position"),
            ));
        }
        observed.cursor.seek(target)
    }
}
