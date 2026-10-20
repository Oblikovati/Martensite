//! Named reader and application faults for the filename retrieval catch.
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
    standard::{Failure, Handle, StandardTransient},
    storage::Data,
    tcollection::ExtendedString,
};
use std::{io, sync::Mutex};

pub struct FaultReader {
    pub transient: StandardTransient,
    pub base: PcdmReaderBase,
    pub failure: Failure,
    pub status: PcdmReaderStatus,
    pub expected_document: Handle<dyn CdmDocument>,
    pub expected_filter: Handle<PcdmReaderFilter>,
    pub calls: Mutex<Vec<&'static str>>,
}
impl_transient!(FaultReader, "FaultReader", dyn PcdmReader,
    transient, interfaces: [dyn PcdmReader]);
impl PcdmReader for FaultReader {
    fn reader_base(&self) -> &PcdmReaderBase {
        &self.base
    }
    fn read_file(
        &self,
        _file: &ExtendedString,
        document: &Handle<dyn CdmDocument>,
        application: &Handle<dyn CdmApplication>,
        filter: &Handle<PcdmReaderFilter>,
        _range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        assert!(*document == self.expected_document);
        assert!(*filter == self.expected_filter);
        let app = application.cast::<FaultApplication>();
        assert!(!app.is_null());
        assert!(std::ptr::eq(app.reader.get().unwrap(), self));
        self.calls.lock().unwrap().push("read-file");
        self.base.set_status(self.status);
        Err(self.failure.clone())
    }
    fn read_stream(
        &self,
        _stream: &mut dyn io::Read,
        _storage: &Handle<Data>,
        _document: &Handle<dyn CdmDocument>,
        _application: &Handle<dyn CdmApplication>,
        _filter: &Handle<PcdmReaderFilter>,
        _range: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        panic!("stream callback invoked; expected filename retrieval");
    }
}

pub struct FaultApplication {
    pub transient: StandardTransient,
    pub base: CdfApplicationBase,
    pub reader: Handle<FaultReader>,
}
impl_transient!(FaultApplication, "FaultApplication", dyn CdfApplication,
    transient, interfaces: [dyn CdmApplication, dyn CdfApplication]);
impl CdmApplication for FaultApplication {
    cdf_application_cdm_members!();
    fn resources(&self) -> Result<SharedResourceManager, Failure> {
        panic!("Resources invoked; expected XML header precedence");
    }
}
impl CdfApplication for FaultApplication {
    fn cdf_application_base(&self) -> &CdfApplicationBase {
        &self.base
    }
    fn reader_from_format(
        &self,
        format: &ExtendedString,
    ) -> Result<Handle<dyn PcdmReader>, Failure> {
        assert_eq!(format, &ExtendedString::new("Owned"));
        self.reader.calls.lock().unwrap().push("reader:Owned");
        Ok(self.reader.cast())
    }
    fn new_document(
        &self,
        _format: &ExtendedString,
        _document: &mut Handle<dyn CdmDocument>,
    ) -> Result<(), Failure> {
        panic!("NewDocument invoked; expected existing append document");
    }
}
