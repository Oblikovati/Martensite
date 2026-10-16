//! The storage enumerations and the stream exception family.
use crate::standard::Failure;

/// The status of a storage or retrieval step; `StorageError::Ok`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StorageError {
    /// Success.
    #[default]
    Ok,
    /// The container could not be opened.
    OpenError,
    /// The open mode does not allow the operation.
    ModeError,
    /// The container could not be closed.
    CloseError,
    /// The container is already open.
    AlreadyOpen,
    /// The container is not open.
    NotOpen,
    /// A section was not found.
    SectionNotFound,
    /// A write failed.
    WriteError,
    /// The data does not have the expected format.
    FormatError,
    /// A type is not known to the schema.
    UnknownType,
    /// A value has an unexpected type.
    TypeMismatch,
    /// An internal inconsistency.
    InternalError,
    /// An extended character has the wrong parity.
    ExtCharParityError,
    /// The file belongs to another driver.
    WrongFileDriver,
}

impl StorageError {
    /// The enumerator value, used by dumps; `StorageError::FormatError.ordinal() == 8`.
    pub fn ordinal(self) -> i32 {
        self as i32
    }
}

/// How a driver opened its container; `OpenMode::Read`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OpenMode {
    /// Not open.
    #[default]
    None,
    /// Open for reading.
    Read,
    /// Open for writing.
    Write,
    /// Open for reading and writing.
    ReadWrite,
}

impl OpenMode {
    /// The enumerator value; `OpenMode::Write.ordinal() == 2`.
    pub fn ordinal(self) -> i32 {
        self as i32
    }
}

/// Which schema step resolves an unknown type; `SolveMode::ReadSolve`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolveMode {
    /// Resolve while adding objects.
    AddSolve,
    /// Resolve while writing.
    WriteSolve,
    /// Resolve while reading.
    ReadSolve,
}

/// A `long` stream offset, -1 on error; `let position: Position = -1;`.
pub type Position = i64;

crate::standard::exceptions::define_standard_exceptions! {
    STORAGE_HIERARCHY;
    stream_ext_char_parity_error => "Storage_StreamExtCharParityError" : "Storage_StreamReadError";
    stream_format_error => "Storage_StreamFormatError" : "Standard_Failure";
    stream_mode_error => "Storage_StreamModeError" : "Standard_Failure";
    stream_read_error => "Storage_StreamReadError" : "Standard_Failure";
    stream_type_mismatch_error => "Storage_StreamTypeMismatchError" : "Storage_StreamReadError";
    stream_unknown_type_error => "Storage_StreamUnknownTypeError" : "Storage_StreamReadError";
    stream_write_error => "Storage_StreamWriteError" : "Standard_Failure";
}

/// The stream exception classes, as a typed name for raising and catching.
///
/// ```
/// use martensite_tkernel::storage::StreamException;
/// let failure = StreamException::TypeMismatch.raise("GetInteger: expected 4 bytes");
/// assert!(StreamException::TypeMismatch.is(&failure));
/// assert!(StreamException::Read.is(&failure));
/// assert_eq!(failure.exception_type(), "Storage_StreamTypeMismatchError");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamException {
    /// Raised as `Storage_StreamExtCharParityError`.
    ExtCharParity,
    /// Raised as `Storage_StreamFormatError`.
    Format,
    /// Raised as `Storage_StreamModeError`.
    Mode,
    /// Raised as `Storage_StreamReadError`.
    Read,
    /// Raised as `Storage_StreamTypeMismatchError`.
    TypeMismatch,
    /// Raised as `Storage_StreamUnknownTypeError`.
    UnknownType,
    /// Raised as `Storage_StreamWriteError`.
    Write,
}

impl StreamException {
    /// The class name; `StreamException::Read.type_name() == "Storage_StreamReadError"`.
    pub fn type_name(self) -> &'static str {
        match self {
            Self::ExtCharParity => "Storage_StreamExtCharParityError",
            Self::Format => "Storage_StreamFormatError",
            Self::Mode => "Storage_StreamModeError",
            Self::Read => "Storage_StreamReadError",
            Self::TypeMismatch => "Storage_StreamTypeMismatchError",
            Self::UnknownType => "Storage_StreamUnknownTypeError",
            Self::Write => "Storage_StreamWriteError",
        }
    }

    /// Builds the exception with a diagnostic message; `StreamException::Format.raise("...")`.
    pub fn raise(self, message: &str) -> Failure {
        Failure::with_message(self.type_name(), message)
    }

    /// The catch test for this class, which also catches derived classes;
    /// `StreamException::Read.is(&failure)`.
    pub fn is(self, failure: &Failure) -> bool {
        failure.is_kind(self.type_name())
    }
}
