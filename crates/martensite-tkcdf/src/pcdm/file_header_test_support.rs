//! Named FSD file system/device spies; their streams are owned and thread-confined.
use martensite_tkernel::{
    fsd::{DriverFileSystem, FileOpenMode, MemoryFileSystem},
    storage::StreamDevice,
    tcollection::AsciiString,
};
use std::{
    cell::{Cell, RefCell},
    io::{self, Read, Seek, SeekFrom, Write},
    rc::Rc,
};

#[derive(Default)]
pub(super) struct Trace {
    pub opens: RefCell<Vec<FileOpenMode>>,
    pub reads: RefCell<Vec<usize>>,
    pub closed: Cell<usize>,
}

pub(super) struct FakeHeaderFiles {
    pub memory: MemoryFileSystem,
    pub trace: Rc<Trace>,
    pub refuse_open: Option<usize>,
}
impl FakeHeaderFiles {
    pub fn new(bytes: &[u8], refuse_open: Option<usize>) -> Rc<Self> {
        let memory = MemoryFileSystem::default();
        memory.insert(&AsciiString::new("owned.fsd"), bytes.to_vec());
        Rc::new(Self {
            memory,
            trace: Rc::new(Trace::default()),
            refuse_open,
        })
    }
}
impl DriverFileSystem for FakeHeaderFiles {
    fn open(&self, name: &AsciiString, mode: FileOpenMode) -> io::Result<Box<dyn StreamDevice>> {
        self.trace.opens.borrow_mut().push(mode);
        if self.refuse_open == Some(self.trace.opens.borrow().len()) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "owned open refusal",
            ));
        }
        Ok(Box::new(TrackedDevice {
            inner: self.memory.open(name, mode)?,
            trace: self.trace.clone(),
        }))
    }
}
struct TrackedDevice {
    inner: Box<dyn StreamDevice>,
    trace: Rc<Trace>,
}
impl Read for TrackedDevice {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        self.trace.reads.borrow_mut().push(bytes.len());
        self.inner.read(bytes)
    }
}
impl Write for TrackedDevice {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.inner.write(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
impl Seek for TrackedDevice {
    fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
        self.inner.seek(position)
    }
}
impl Drop for TrackedDevice {
    fn drop(&mut self) {
        self.trace.closed.set(self.trace.closed.get() + 1);
    }
}

macro_rules! frozen_files {
    ($kind:literal) => {
        [
            include_bytes!(concat!(
                "../../tests/file_header/fixtures/",
                $kind,
                "-0.fsd"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "../../tests/file_header/fixtures/",
                $kind,
                "-1.fsd"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "../../tests/file_header/fixtures/",
                $kind,
                "-2.fsd"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "../../tests/file_header/fixtures/",
                $kind,
                "-3.fsd"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "../../tests/file_header/fixtures/",
                $kind,
                "-4.fsd"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "../../tests/file_header/fixtures/",
                $kind,
                "-5.fsd"
            ))
            .as_slice(),
            include_bytes!(concat!(
                "../../tests/file_header/fixtures/",
                $kind,
                "-6.fsd"
            ))
            .as_slice(),
        ]
    };
}
pub(super) const FROZEN_FILES: [(&str, [&[u8]; 7]); 3] = [
    ("cmp", frozen_files!("cmp")),
    ("text", frozen_files!("text")),
    ("binary", frozen_files!("binary")),
];
