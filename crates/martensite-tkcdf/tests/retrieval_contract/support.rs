//! Named metadata and application callbacks, with an owned ordered call trace.
use martensite_tkcdf::{
    SharedResourceManager,
    cdf::{CdfApplication, CdfApplicationBase, CdfMetaDataDriver},
    cdf_application_cdm_members,
    cdm::{CdmApplication, CdmDocument, CdmMetaData, MetaDataLookUpTable},
    pcdm::{PcdmReader, PcdmReaderStatus},
};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient, exceptions},
    tcollection::ExtendedString,
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
#[path = "reader.rs"]
mod reader;

pub type CallTrace = Arc<Mutex<Vec<String>>>;

pub struct FakeRetrievalMetaDataDriver {
    transient: StandardTransient,
    pub metadata: Handle<CdmMetaData>,
    trace: CallTrace,
    pub found: bool,
    pub readable: bool,
    pub fail_metadata: bool,
}
impl_transient!(FakeRetrievalMetaDataDriver, "FakeRetrievalMetaDataDriver", StandardTransient,
    transient, interfaces: [dyn CdfMetaDataDriver]);
impl CdfMetaDataDriver for FakeRetrievalMetaDataDriver {
    fn find(
        &self,
        folder: &ExtendedString,
        name: &ExtendedString,
        version: &ExtendedString,
    ) -> bool {
        self.trace.lock().unwrap().push(format!(
            "find:{}/{}/{}",
            folder.to_utf8(),
            name.to_utf8(),
            version.to_utf8()
        ));
        self.found
    }
    fn has_read_permission(
        &self,
        _folder: &ExtendedString,
        _name: &ExtendedString,
        _version: &ExtendedString,
    ) -> bool {
        self.trace.lock().unwrap().push("permission".into());
        self.readable
    }
    fn meta_data(
        &self,
        _folder: &ExtendedString,
        _name: &ExtendedString,
        _version: &ExtendedString,
    ) -> Result<Handle<CdmMetaData>, Failure> {
        self.trace.lock().unwrap().push("metadata".into());
        if self.fail_metadata {
            return Err(exceptions::domain_error(
                "owned metadata failure; expected metadata",
            ));
        }
        Ok(self.metadata.clone())
    }
    fn build_file_name(
        &self,
        _document: &Handle<dyn CdmDocument>,
    ) -> Result<ExtendedString, Failure> {
        Ok(self.metadata.file_name())
    }
    fn create_meta_data(
        &self,
        _document: &Handle<dyn CdmDocument>,
        _file: &ExtendedString,
    ) -> Result<Handle<CdmMetaData>, Failure> {
        Ok(self.metadata.clone())
    }
    fn find_folder(&self, _folder: &ExtendedString) -> bool {
        true
    }
    fn default_folder(&self) -> ExtendedString {
        ExtendedString::default()
    }
}

pub struct FakeHeaderApplication {
    transient: StandardTransient,
    pub base: CdfApplicationBase,
    pub trace: CallTrace,
    pub reader_failure: bool,
    pub live_reader: AtomicBool,
    pub read_calls: Arc<AtomicUsize>,
}
impl_transient!(FakeHeaderApplication, "FakeHeaderApplication", dyn CdfApplication,
    transient, interfaces: [dyn CdmApplication, dyn CdfApplication]);
impl CdmApplication for FakeHeaderApplication {
    cdf_application_cdm_members!();
    fn resources(&self) -> Result<SharedResourceManager, Failure> {
        self.trace.lock().unwrap().push("resources".into());
        Err(exceptions::domain_error(
            "owned Resources failure; expected XML header precedence",
        ))
    }
}
impl CdfApplication for FakeHeaderApplication {
    fn cdf_application_base(&self) -> &CdfApplicationBase {
        &self.base
    }
    fn reader_from_format(
        &self,
        format: &ExtendedString,
    ) -> Result<Handle<dyn PcdmReader>, Failure> {
        self.trace
            .lock()
            .unwrap()
            .push(format!("reader:{}", format.to_utf8()));
        if self.reader_failure {
            self.base
                .set_retrievable_status(PcdmReaderStatus::ReaderException);
            return Err(exceptions::domain_error(
                "owned reader failure; expected availability suppression",
            ));
        }
        if self.live_reader.load(Ordering::SeqCst) {
            return Ok(
                Handle::new(reader::AvailabilityReader::new(self.read_calls.clone())).cast(),
            );
        }
        Ok(Handle::default())
    }
}

/// Owned metadata; `metadata(&text("header.model"))`.
pub fn metadata(file: &ExtendedString) -> Handle<CdmMetaData> {
    CdmMetaData::look_up(
        &mut MetaDataLookUpTable::default(),
        &text("folder"),
        &text("name"),
        file,
        file,
        false,
    )
}

/// Bound callback fake; `application(metadata(&text("file")), true, true, false, false)`.
pub fn application(
    metadata: Handle<CdmMetaData>,
    found: bool,
    readable: bool,
    fail_metadata: bool,
    reader_failure: bool,
) -> Handle<FakeHeaderApplication> {
    let app = bare_application(reader_failure);
    let driver = Handle::new(legacy_driver(
        &app,
        metadata,
        found,
        readable,
        fail_metadata,
    ));
    app.base.set_meta_data_driver(&driver.cast());
    app.application_base().bind_self(&app.cast());
    app
}

pub(super) fn legacy_driver(
    app: &FakeHeaderApplication,
    metadata: Handle<CdmMetaData>,
    found: bool,
    readable: bool,
    fail_metadata: bool,
) -> FakeRetrievalMetaDataDriver {
    FakeRetrievalMetaDataDriver {
        transient: StandardTransient::default(),
        metadata,
        trace: app.trace.clone(),
        found,
        readable,
        fail_metadata,
    }
}

fn bare_application(reader_failure: bool) -> Handle<FakeHeaderApplication> {
    Handle::new(FakeHeaderApplication {
        transient: StandardTransient::default(),
        base: CdfApplicationBase::default(),
        trace: Arc::new(Mutex::new(Vec::new())),
        reader_failure,
        live_reader: AtomicBool::new(false),
        read_calls: Arc::new(AtomicUsize::new(0)),
    })
}

/// ASCII fixture text; `text("Header")`.
pub fn text(value: &str) -> ExtendedString {
    ExtendedString::new(value)
}
