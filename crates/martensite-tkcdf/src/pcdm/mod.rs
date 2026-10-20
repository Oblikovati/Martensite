//! The PCDM package: the persistent-document interfaces of the framework (readers, writers,
//! storage and retrieval drivers, reader filters). Format probing of files and streams
//! uses the FSD container drivers and LDOM; document-format readers are plugged in.
mod binary_stream;
mod file_driver_probe;
pub use file_driver_probe::Pcdm;
mod file_header;
#[cfg(test)]
mod file_header_faults;
#[cfg(test)]
mod file_header_test_support;
mod header_counters;
mod header_references;
mod input_reader;
pub(crate) use input_reader::PcdmInputReader;
mod input_stream;
mod native_header;
mod read_writer;
mod reader;
mod reader_filter;
mod reference;
mod reference_iterator;
mod reference_paths;
mod statuses;
mod stream_failure;
mod stream_format;
pub(crate) use stream_format::{ProbedFormat, probe_input_format};
mod xml_header;
pub use input_stream::PcdmInputStream;
pub use stream_failure::PcdmStreamFailure;
mod writer;
pub use read_writer::{PcdmReadWriter, PcdmReadWriterBase};
pub use reader::{PcdmReader, PcdmReaderBase, PcdmRetrievalDriver, PcdmRetrievalDriverBase};
pub use reader_filter::{
    PcdmDefaultReaderFilterHooks, PcdmReaderFilter, PcdmReaderFilterAppendMode,
    PcdmReaderFilterHooks,
};
pub use reference::PcdmReference;
pub use reference_iterator::{
    PcdmDefaultReferenceIteratorHooks, PcdmReferenceIterator, PcdmReferenceIteratorHooks,
};
pub use statuses::{PcdmReaderStatus, PcdmStoreStatus, PcdmTypeOfFileDriver, pcdm_driver_error};
pub use writer::{PcdmStorageDriver, PcdmStorageDriverBase, PcdmWriter};
