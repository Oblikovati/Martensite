//! A live reader whose callbacks must never run during an availability query.
use martensite_tkcdf::{
    cdm::{CdmApplication, CdmDocument},
    pcdm::{PcdmReader, PcdmReaderBase, PcdmReaderFilter},
};
use martensite_tkernel::{
    impl_transient,
    message::ProgressRange,
    standard::{Failure, Handle, StandardTransient},
    storage::Data,
    tcollection::ExtendedString,
};
use std::{
    io,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

pub struct AvailabilityReader {
    transient: StandardTransient,
    base: PcdmReaderBase,
    calls: Arc<AtomicUsize>,
}
impl_transient!(AvailabilityReader, "AvailabilityReader", dyn PcdmReader,
    transient, interfaces: [dyn PcdmReader]);
impl AvailabilityReader {
    /// A live reader spy; `AvailabilityReader::new(Arc::new(AtomicUsize::new(0)))`.
    pub fn new(calls: Arc<AtomicUsize>) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: PcdmReaderBase::default(),
            calls,
        }
    }
}
impl PcdmReader for AvailabilityReader {
    fn reader_base(&self) -> &PcdmReaderBase {
        &self.base
    }
    fn read_file(
        &self,
        _file: &ExtendedString,
        _document: &Handle<dyn CdmDocument>,
        _application: &Handle<dyn CdmApplication>,
        _filter: &Handle<PcdmReaderFilter>,
        _progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Err(Failure::new(
            "availability reader invoked; expected no read during CanRetrieve",
        ))
    }
    fn read_stream(
        &self,
        _stream: &mut dyn io::Read,
        _storage: &Handle<Data>,
        _document: &Handle<dyn CdmDocument>,
        _application: &Handle<dyn CdmApplication>,
        _filter: &Handle<PcdmReaderFilter>,
        _progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Err(Failure::new(
            "availability reader invoked; expected no read during CanRetrieve",
        ))
    }
}
