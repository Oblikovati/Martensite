use martensite_tkcdf::pcdm::PcdmInputStream;
use martensite_tkernel::{
    fsd::{File, NativeDriverFileSystem},
    standard::Failure,
    storage::BaseDriver,
    tcollection::AsciiString,
};
use std::{
    io::{self, Read},
    rc::Rc,
};

pub struct DriverProbeDevice<'a> {
    pub bytes: &'a [u8],
    pub consumed: usize,
    pub calls: usize,
    pub failure_at: Option<usize>,
    pub kernel: bool,
}
impl<'a> DriverProbeDevice<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            consumed: 0,
            calls: 0,
            failure_at: None,
            kernel: false,
        }
    }
    pub fn input(&mut self, mask: u8) -> PcdmInputStream<'_> {
        PcdmInputStream::with_exception_mask(self, mask)
    }
}
impl Read for DriverProbeDevice<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        assert_eq!(buffer.len(), 1);
        self.calls += 1;
        if self.failure_at == Some(self.consumed) {
            return Err(if self.kernel {
                io::Error::other(Failure::with_message(
                    "Standard_DomainError",
                    "probe refusal",
                ))
            } else {
                io::Error::new(io::ErrorKind::PermissionDenied, "probe refusal")
            });
        }
        let Some(byte) = self.bytes.get(self.consumed) else {
            return Ok(0);
        };
        buffer[0] = *byte;
        self.consumed += 1;
        Ok(1)
    }
}

pub fn marked_driver() -> Option<Box<dyn BaseDriver>> {
    let mut driver = File::new(Rc::new(NativeDriverFileSystem));
    driver.set_name(&AsciiString::new("sentinel"));
    Some(Box::new(driver))
}

pub fn retained(driver: &Option<Box<dyn BaseDriver>>) -> bool {
    driver
        .as_ref()
        .is_some_and(|driver| driver.name().as_bytes() == b"sentinel")
}
