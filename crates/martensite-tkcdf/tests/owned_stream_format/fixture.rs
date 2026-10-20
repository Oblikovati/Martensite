//! Original storage stream and caller-owned storage identity, without format I/O mocks.
use martensite_tkcdf::pcdm::PcdmReadWriterBase;
use martensite_tkernel::{
    standard::{Failure, Handle},
    storage::{Data, IoStream},
    tcollection::{AsciiString, ExtendedString},
};

pub struct FormatProbe {
    pub stream: IoStream,
    pub storage: Handle<Data>,
    pub before: Handle<Data>,
}

impl FormatProbe {
    pub fn new(bytes: Vec<u8>, existing: bool, origin: i64) -> Self {
        let mut stream = IoStream::from_bytes(bytes);
        stream.seekg(origin);
        let storage = previous_storage(existing);
        Self {
            stream,
            before: storage.clone(),
            storage,
        }
    }

    pub fn query(&mut self) -> Result<ExtendedString, Failure> {
        PcdmReadWriterBase::file_format_storage_stream(&mut self.stream, &mut self.storage)
    }

    pub fn assert_stream(&mut self, state: u8, count: i64, position: i64) {
        assert_eq!(self.stream.rdstate(), state);
        assert_eq!(self.stream.gcount(), count);
        // As in the native diagnostic, observe position only after recording failed state.
        self.stream.clear(0);
        assert_eq!(self.stream.tellg(), position);
        self.stream.clear(state);
    }

    pub fn assert_storage(&self, allocated: bool, identical: bool, lines: i32, objects: i32) {
        assert_eq!(!self.storage.is_null(), allocated);
        assert_eq!(self.storage == self.before, identical);
        if let Some(storage) = self.storage.get() {
            assert_eq!(storage.user_info().length(), lines);
            assert_eq!(storage.number_of_objects(), objects);
        }
    }
}

fn previous_storage(existing: bool) -> Handle<Data> {
    if !existing {
        return Handle::default();
    }
    let storage = Handle::new(Data::default());
    storage.add_to_user_info(&AsciiString::new("FILE_FORMAT: previous"));
    storage
        .header_data()
        .get()
        .unwrap()
        .set_number_of_objects(99);
    storage
}
