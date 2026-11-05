//! Shared readable geometry output at the command precision.
pub fn printed_text(stream: martensite_tkernel::storage::IoStream) -> String {
    use std::io::{Read, Seek, SeekFrom};
    let mut device = stream.into_device().unwrap();
    device.seek(SeekFrom::Start(0)).unwrap();
    let mut text = String::new();
    device.read_to_string(&mut text).unwrap();
    text
}

pub fn readable_text(write: impl FnOnce(&mut martensite_tkernel::storage::IoStream)) -> String {
    let mut stream = martensite_tkernel::storage::IoStream::from_bytes(Vec::new());
    stream.set_precision(15);
    write(&mut stream);
    printed_text(stream)
}
