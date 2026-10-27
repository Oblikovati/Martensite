//! I/O unwinds retain their device and never publish incomplete scratch input (#318).
use martensite_tkernel::storage::{IoStream, state};
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};
use std::panic::{AssertUnwindSafe, catch_unwind, panic_any};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FaultSite {
    Read,
    Write,
    Seek,
}

#[derive(Debug)]
struct DevicePanicked(FaultSite);

struct FaultingDevice {
    bytes: Cursor<Vec<u8>>,
    site: FaultSite,
}

impl Read for FaultingDevice {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        if matches!(self.site, FaultSite::Read) {
            let count = bytes.len().min(2);
            self.bytes.read_exact(&mut bytes[..count])?;
            panic_any(DevicePanicked(self.site));
        }
        self.bytes.read(bytes)
    }
}
impl Write for FaultingDevice {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if matches!(self.site, FaultSite::Write) {
            panic_any(DevicePanicked(self.site));
        }
        self.bytes.write(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        let result: std::io::Result<()> = self.bytes.flush();
        result
    }
}
impl Seek for FaultingDevice {
    fn seek(&mut self, target: SeekFrom) -> io::Result<u64> {
        if matches!(self.site, FaultSite::Seek) {
            panic_any(DevicePanicked(self.site));
        }
        self.bytes.seek(target)
    }
}

fn faulting_stream(site: FaultSite) -> IoStream {
    let bytes = Cursor::new(b"abcdef".to_vec());
    let device = FaultingDevice { bytes, site };
    IoStream::with_device(Box::new(device), true, true)
}

fn assert_device_fault(payload: &(dyn std::any::Any + Send), site: FaultSite) {
    let actual = payload.downcast_ref::<DevicePanicked>().unwrap_or_else(|| {
        panic!(
            "panic payload {:?}, expected DevicePanicked",
            payload.type_id()
        )
    });
    assert_eq!(actual.0, site);
}

#[test]
fn read_panic_retains_device_and_refuses_invalid_pending_storage() {
    let mut stream = faulting_stream(FaultSite::Read);
    let payload = catch_unwind(AssertUnwindSafe(|| {
        stream.read(&mut [0; 1]);
    }))
    .unwrap_err();
    assert_device_fault(&*payload, FaultSite::Read);
    assert!(stream.is_open());
    assert!(
        stream.bad(),
        "panicked read must not publish resized pending bytes as valid input"
    );
    assert_eq!(stream.gcount(), 0);
    let mut next = [9];
    stream.read(&mut next);
    assert_eq!(next, [9]);
    assert_eq!(stream.rdstate(), state::BAD | state::FAIL);
}

#[test]
fn write_panic_retains_the_owned_device_slot() {
    let mut stream = faulting_stream(FaultSite::Write);
    let payload = catch_unwind(AssertUnwindSafe(|| {
        stream.write(b"x");
    }))
    .unwrap_err();
    assert_device_fault(&*payload, FaultSite::Write);
    assert!(
        stream.is_open(),
        "panicked write must retain the owned device"
    );
}

#[test]
fn seek_panic_retains_the_owned_device_slot() {
    let mut stream = faulting_stream(FaultSite::Seek);
    let payload = catch_unwind(AssertUnwindSafe(|| {
        stream.seekg(0);
    }))
    .unwrap_err();
    assert_device_fault(&*payload, FaultSite::Seek);
    assert!(
        stream.is_open(),
        "panicked seek must retain the owned device"
    );
}

fn assert_positioning_fault(operation: fn(&mut IoStream)) {
    let mut stream = faulting_stream(FaultSite::Seek);
    let payload = catch_unwind(AssertUnwindSafe(|| operation(&mut stream))).unwrap_err();
    assert_device_fault(&*payload, FaultSite::Seek);
    assert!(
        stream.is_open(),
        "positioning panic: expected retained device slot"
    );
}

#[test]
fn output_seek_panic_retains_the_owned_device_slot() {
    let operation: fn(&mut IoStream) = |stream| {
        stream.seekp(0);
    };
    assert_positioning_fault(operation);
}

#[test]
fn input_position_panic_retains_the_owned_device_slot() {
    let operation: fn(&mut IoStream) = |stream| {
        stream.tellg();
    };
    assert_positioning_fault(operation);
}

#[test]
fn output_position_panic_retains_the_owned_device_slot() {
    let operation: fn(&mut IoStream) = |stream| {
        stream.tellp();
    };
    assert_positioning_fault(operation);
}
