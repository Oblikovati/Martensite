//! A delegating test file system retains actual opened owners for observation.
use super::fakes::MemoryFileSystem;
use martensite_tkernel::osd::{
    FileSystem, IosOpenMode, SeekDir, SharedIStream, SharedOStream, SharedStreamBuf,
};
use martensite_tkernel::{standard::Failure, tcollection::AsciiString};
use std::sync::{Arc, Mutex};

pub(super) struct ObservedFiles {
    pub(super) files: MemoryFileSystem,
    input: Mutex<Option<SharedIStream>>,
    input_buffer: Mutex<Option<SharedStreamBuf>>,
    output: Mutex<Option<SharedOStream>>,
    fail_input: bool,
    fail_output: bool,
}
#[derive(Debug, PartialEq, Eq)]
pub(super) struct InputObservation {
    pub(super) state: u8,
    pub(super) count: usize,
    pub(super) position: i64,
    pub(super) same_buffer: bool,
}
impl ObservedFiles {
    /// Retain opened owners around an existing named fake; `ObservedFiles::new(files, false, false)`.
    pub(super) fn new(files: MemoryFileSystem, fail_input: bool, fail_output: bool) -> Self {
        let observed: Self = Self {
            files,
            input: Mutex::new(None),
            input_buffer: Mutex::new(None),
            output: Mutex::new(None),
            fail_input,
            fail_output,
        };
        observed
    }
    /// Observe the buffer without changing failed-stream state; `files.input_observation()`.
    pub(super) fn input_observation(&self) -> InputObservation {
        let retained = self.input.lock().unwrap();
        let input = retained.as_ref().unwrap().lock().unwrap();
        let buffer = input.rdbuf();
        let saved = self.input_buffer.lock().unwrap();
        InputObservation {
            state: input.rdstate().bits(),
            count: input.gcount(),
            position: buffer_position(&buffer, IosOpenMode::IN),
            same_buffer: Arc::ptr_eq(&buffer, saved.as_ref().unwrap()),
        }
    }
    /// Inspect output flags and its actual buffer cursor; `files.output_observation()`.
    pub(super) fn output_observation(&self) -> (u8, i64) {
        let retained = self.output.lock().unwrap();
        let output = retained.as_ref().unwrap().lock().unwrap();
        (
            output.rdstate().bits(),
            buffer_position(&output.rdbuf(), IosOpenMode::OUT),
        )
    }
}
impl FileSystem for ObservedFiles {
    fn is_supported_path(&self, name: &AsciiString) -> bool {
        let supported: bool = self.files.is_supported_path(name);
        supported
    }
    fn is_open_istream(&self, stream: &SharedIStream) -> bool {
        let opened: bool = self.files.is_open_istream(stream);
        opened
    }
    fn is_open_ostream(&self, stream: &SharedOStream) -> bool {
        let opened: bool = self.files.is_open_ostream(stream);
        opened
    }
    fn open_istream(
        &self,
        name: &AsciiString,
        mode: IosOpenMode,
        offset: i64,
        old: Option<&SharedIStream>,
    ) -> Result<Option<SharedIStream>, Failure> {
        let opened = self.files.open_istream(name, mode, offset, old)?;
        if let Some(input) = &opened {
            seed_input(input, self.fail_input);
            *self.input_buffer.lock().unwrap() = Some(input.lock().unwrap().rdbuf());
            *self.input.lock().unwrap() = Some(Arc::clone(input));
        }
        Ok(opened)
    }
    fn open_ostream(
        &self,
        name: &AsciiString,
        mode: IosOpenMode,
    ) -> Result<Option<SharedOStream>, Failure> {
        let opened = self.files.open_ostream(name, mode)?;
        if let Some(output) = &opened {
            seed_output(output, self.fail_output);
            *self.output.lock().unwrap() = Some(Arc::clone(output));
        }
        Ok(opened)
    }
    fn open_stream_buffer(
        &self,
        name: &AsciiString,
        mode: IosOpenMode,
        offset: i64,
        size: Option<&mut i64>,
    ) -> Result<Option<SharedStreamBuf>, Failure> {
        let opened: Result<Option<SharedStreamBuf>, Failure> =
            self.files.open_stream_buffer(name, mode, offset, size);
        opened
    }
}
fn seed_input(input: &SharedIStream, failed: bool) {
    let mut input = input.lock().unwrap();
    input.read(&mut [0; 3]);
    input.clear();
    input.seekg(0, SeekDir::Beg);
    if failed {
        input.seekg(-1, SeekDir::Beg);
    }
}
fn seed_output(output: &SharedOStream, failed: bool) {
    if !failed {
        return;
    }
    output.lock().unwrap().seekp(-1, SeekDir::Beg);
}
fn buffer_position(buffer: &SharedStreamBuf, mode: IosOpenMode) -> i64 {
    let mut buffer = buffer.lock().unwrap();
    buffer.pubseekoff(0, SeekDir::Cur, mode)
}
