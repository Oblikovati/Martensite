//! Named in-memory file system for filename header probes, isolated by protocol URL.
use martensite_tkernel::{
    osd::{
        FileSystem, FileSystemHandle, IosOpenMode, SeekDir, SharedIStream, SharedOStream,
        SharedStreamBuf, StreamBuf, add_default_protocol, remove_default_protocol,
        share_stream_buf,
    },
    standard::Failure,
    tcollection::{AsciiString, ExtendedString},
};
use std::sync::Arc;

struct HeaderBytes {
    bytes: Vec<u8>,
    cursor: usize,
}
impl StreamBuf for HeaderBytes {
    fn sputn(&mut self, _bytes: &[u8]) -> usize {
        0
    }
    fn pubsync(&mut self) -> i32 {
        0
    }
    fn sgetn(&mut self, output: &mut [u8]) -> usize {
        let count = output.len().min(self.bytes.len() - self.cursor);
        output[..count].copy_from_slice(&self.bytes[self.cursor..self.cursor + count]);
        self.cursor += count;
        count
    }
    fn pubseekoff(&mut self, offset: i64, direction: SeekDir, _mode: IosOpenMode) -> i64 {
        let base = match direction {
            SeekDir::Beg => 0,
            SeekDir::Cur => self.cursor as i64,
            SeekDir::End => self.bytes.len() as i64,
        };
        let target = base + offset;
        if !(0..=self.bytes.len() as i64).contains(&target) {
            return -1;
        }
        self.cursor = target as usize;
        target
    }
}

struct FakeHeaderFileSystem {
    url: String,
    xml: Option<Vec<u8>>,
}
impl FileSystem for FakeHeaderFileSystem {
    fn is_supported_path(&self, url: &AsciiString) -> bool {
        url.as_bytes() == self.url.as_bytes()
    }
    fn is_open_istream(&self, _stream: &SharedIStream) -> bool {
        true
    }
    fn is_open_ostream(&self, _stream: &SharedOStream) -> bool {
        false
    }
    fn open_stream_buffer(
        &self,
        _url: &AsciiString,
        _mode: IosOpenMode,
        _offset: i64,
        _size: Option<&mut i64>,
    ) -> Result<Option<SharedStreamBuf>, Failure> {
        Ok(self.xml.as_ref().map(|bytes| {
            share_stream_buf(HeaderBytes {
                bytes: bytes.clone(),
                cursor: 0,
            })
        }))
    }
}

pub struct HeaderFixture {
    protocol: FileSystemHandle,
    pub path: ExtendedString,
}
impl HeaderFixture {
    /// An isolated input URL; `HeaderFixture::new("root", Some(b"<document/>"))`.
    pub fn new(case: &str, xml: Option<&[u8]>) -> Self {
        let url = format!("tkcdf-owned-header://{case}.model");
        let protocol: FileSystemHandle = Arc::new(FakeHeaderFileSystem {
            url: url.clone(),
            xml: xml.map(<[u8]>::to_vec),
        });
        add_default_protocol(protocol.clone(), true);
        Self {
            protocol,
            path: ExtendedString::new(url.as_str()),
        }
    }
}
impl Drop for HeaderFixture {
    fn drop(&mut self) {
        remove_default_protocol(&self.protocol);
    }
}
