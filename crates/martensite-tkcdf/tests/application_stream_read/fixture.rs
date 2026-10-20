//! A named stream reader and an application that records `NewDocument` and reader lookup.
use martensite_tkcdf::{
    SharedResourceManager,
    cdf::{CdfApplication, CdfApplicationBase},
    cdf_application_cdm_members,
    cdm::{CdmApplication, CdmDocument},
    pcdm::{PcdmReader, PcdmReaderBase, PcdmReaderFilter, PcdmReaderStatus},
};
use martensite_tkernel::{
    impl_transient,
    message::ProgressRange,
    standard::{Failure, Handle, StandardTransient, exceptions},
    storage::Data,
    tcollection::ExtendedString,
};
use std::{
    io,
    sync::{Arc, Mutex},
};

/// What one reader call observed.
#[derive(Default)]
pub struct ReaderCall {
    pub body: Vec<u8>,
    pub format_lines: i32,
    pub document: Handle<dyn CdmDocument>,
}

/// Reads the rest of the stream, sets `status` and fails when `failure` is set.
pub struct StreamReader {
    transient: StandardTransient,
    base: PcdmReaderBase,
    pub status: PcdmReaderStatus,
    pub failure: Option<Failure>,
    pub calls: Mutex<Vec<ReaderCall>>,
}
impl_transient!(StreamReader, "StreamReader", dyn PcdmReader,
    transient, interfaces: [dyn PcdmReader]);

impl StreamReader {
    pub fn new(status: PcdmReaderStatus, failure: Option<Failure>) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: PcdmReaderBase::default(),
            status,
            failure,
            calls: Mutex::default(),
        }
    }
}

impl PcdmReader for StreamReader {
    fn reader_base(&self) -> &PcdmReaderBase {
        &self.base
    }
    fn read_file(
        &self,
        _file: &ExtendedString,
        _document: &Handle<dyn CdmDocument>,
        _application: &Handle<dyn CdmApplication>,
        _filter: &Handle<PcdmReaderFilter>,
        _range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        panic!("file callback invoked; expected a stream read");
    }
    fn read_stream(
        &self,
        stream: &mut dyn io::Read,
        storage: &Handle<Data>,
        document: &Handle<dyn CdmDocument>,
        _application: &Handle<dyn CdmApplication>,
        _filter: &Handle<PcdmReaderFilter>,
        _range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let mut body = Vec::new();
        if let Err(error) = stream.read_to_end(&mut body) {
            return Err(Failure::new(&format!("owned reader: stream error {error}")));
        }
        let format_lines = storage.get().map_or(0, |data| data.user_info().length());
        let document = document.clone();
        let call = ReaderCall {
            body,
            format_lines,
            document,
        };
        self.calls.lock().unwrap().push(call);
        self.base.set_status(self.status);
        self.failure.clone().map_or(Ok(()), Err)
    }
}

/// An application with one reader for the format `last` and a traced `NewDocument`.
pub struct StreamApplication {
    transient: StandardTransient,
    pub base: CdfApplicationBase,
    pub reader: Handle<StreamReader>,
    pub new_document: Handle<dyn CdmDocument>,
    pub trace: Arc<Mutex<Vec<String>>>,
}
impl_transient!(StreamApplication, "StreamApplication", dyn CdfApplication,
    transient, interfaces: [dyn CdmApplication, dyn CdfApplication]);

impl StreamApplication {
    pub fn bound(reader: StreamReader, new_document: Handle<dyn CdmDocument>) -> Handle<Self> {
        let application = Handle::new(Self {
            transient: StandardTransient::default(),
            base: CdfApplicationBase::default(),
            reader: Handle::new(reader),
            new_document,
            trace: Arc::default(),
        });
        application
            .application_base()
            .bind_self(&application.cast());
        application
    }

    pub fn trace(&self) -> Vec<String> {
        self.trace.lock().unwrap().clone()
    }
}

impl CdmApplication for StreamApplication {
    cdf_application_cdm_members!();
    fn resources(&self) -> Result<SharedResourceManager, Failure> {
        panic!("Resources invoked; expected the registered reader lookup");
    }
}

impl CdfApplication for StreamApplication {
    fn cdf_application_base(&self) -> &CdfApplicationBase {
        &self.base
    }
    fn new_document(
        &self,
        format: &ExtendedString,
        document: &mut Handle<dyn CdmDocument>,
    ) -> Result<(), Failure> {
        self.trace
            .lock()
            .unwrap()
            .push(format!("new:{}", format.to_utf8()));
        *document = self.new_document.clone();
        Ok(())
    }
    fn reader_from_format(
        &self,
        format: &ExtendedString,
    ) -> Result<Handle<dyn PcdmReader>, Failure> {
        self.trace
            .lock()
            .unwrap()
            .push(format!("reader:{}", format.to_utf8()));
        if format.to_utf8() == "missing" {
            return Err(exceptions::no_such_object(
                "owned lookup: no reader for missing",
            ));
        }
        Ok(self.reader.clone().cast())
    }
}
