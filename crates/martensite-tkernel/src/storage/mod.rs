//! The legacy persistence framework of drivers, schemas and
//! data sets. Drivers read and write through an [`IoStream`] over an injected device.
mod base_driver;
mod bucket;
mod callback;
pub mod checked_input;
mod data;
mod header_data;
mod internal_data;
mod kinds;
mod numbers;
mod root;
mod root_data;
mod schema;
mod status;
mod stream;
mod type_data;

pub use base_driver::{
    BaseDriver, DataSections, DriverBase, DriverPrimitives, DriverValue, HeaderSections,
    TableSections,
};
use bucket::{BucketIterator, BucketOfPersistent};
pub(crate) use callback::CallBackTable;
pub use callback::{CallBack, DefaultCallBack, TypedCallBack};
pub use data::Data;
pub use header_data::HeaderData;
pub(crate) use internal_data::InternalData;
pub(crate) use kinds::STORAGE_HIERARCHY;
pub use kinds::{
    OpenMode, Position, SolveMode, StorageError, StreamException, stream_ext_char_parity_error,
    stream_format_error, stream_mode_error, stream_read_error, stream_type_mismatch_error,
    stream_unknown_type_error, stream_write_error,
};
pub use kinds::{exception_bytes, exception_stacks};
pub(crate) use numbers::PersistentNumbers;
pub(crate) use root::Root;
pub(crate) use root_data::RootData;
pub use schema::{
    CreationDateSource, MigrationFile, MigrationSource, NativeCreationDate, NativeMigrationSource,
    Schema, SchemaContext, TypeMigration,
};
use status::SectionStatus;
pub use stream::borrowed_failure::{
    BorrowedInputUnwind, StreamBridgeObservation, StreamBridgeStage, StreamPanicPayload,
    StreamRejoinCause, StreamRejoinFailure,
};
pub use stream::borrowed_input::with_borrowed_input_stream;
pub use stream::{IoStream, StreamDevice, format_general, state};
pub use type_data::TypeData;

use crate::tcollection::AsciiString;

/// The version of Storage's read/write routines, `"1.3"`.
pub(super) fn version() -> AsciiString {
    AsciiString::new("1.3")
}

#[cfg(test)]
#[path = "../../tests/storage_api_behavior/data.rs"]
mod data_tests;

#[cfg(test)]
#[path = "../../tests/storage_api_behavior/root_sections.rs"]
mod root_section_tests;
#[cfg(test)]
#[path = "../../tests/storage_api_behavior/scripted_driver.rs"]
mod scripted_driver;
