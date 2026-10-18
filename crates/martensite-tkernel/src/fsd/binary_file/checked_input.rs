//! Checked header input reuses the owned reader's scalar and field decoding.
use super::{
    BinaryFile, FileHeader, checked_source::CheckedReadSource, complete_info, input_fields,
};
use crate::standard::Failure;
use crate::standard::Handle;
use crate::storage::{
    Data,
    checked_input::{InputFailure, UnformattedInput},
};
use std::{io, sync::Arc};

/// Owned input, incomplete-field, domain or position failure (#142).
/// Match `CheckedReadError::Input(fault)` to inspect input state and masks.
#[derive(Clone, Debug)]
pub enum CheckedReadError {
    /// A configured input exception, retaining its original typed cause.
    Input(InputFailure),
    /// A typed storage-domain failure, distinct from input and field errors.
    Domain(Failure),
    /// Section dispatch requires an actual caller offset; none was available.
    PositionUnavailable {
        /// Input flags after the position query.
        state: u8,
        /// Active exception bits when the query could not supply an offset.
        mask: u8,
        /// Published input count, unchanged by the position query.
        gcount: usize,
        /// Original device cause, when the query set bad state.
        cause: Option<Arc<io::Error>>,
    },
    /// A field did not receive its required bytes.
    Format {
        /// Typed format failure describing the observed and expected count.
        failure: Failure,
        /// Original device cause when bad state caused the incomplete field.
        cause: Option<Arc<io::Error>>,
    },
}
impl std::fmt::Display for CheckedReadError {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input(failure) => failure.fmt(output),
            Self::Domain(failure) => failure.fmt(output),
            Self::PositionUnavailable { state, mask, .. } => write!(
                output,
                "binary container position unavailable with state {state} and mask {mask}; expected actual caller offset"
            ),
            Self::Format { failure, .. } => failure.fmt(output),
        }
    }
}
impl std::error::Error for CheckedReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Input(failure) => Some(failure),
            Self::Domain(failure) => Some(failure),
            Self::PositionUnavailable {
                cause: Some(cause), ..
            } => Some(cause.as_ref()),
            Self::PositionUnavailable { cause: None, .. } => None,
            Self::Format {
                cause: Some(cause), ..
            } => Some(cause.as_ref()),
            Self::Format { failure, .. } => Some(failure),
        }
    }
}
impl From<Failure> for CheckedReadError {
    fn from(failure: Failure) -> Self {
        Self::Domain(failure)
    }
}
impl BinaryFile {
    /// Read host-order header fields without requiring owned or writable input (#142).
    ///
    /// ```
    /// use martensite_tkernel::fsd::{BinaryFile, CheckedReadError, FileHeader};
    /// use martensite_tkernel::storage::checked_input::{BufferedInput, InputContext};
    /// let mut bytes = [0_u8; 52].as_slice();
    /// let mut source = BufferedInput(&mut bytes);
    /// let mut input = InputContext::new(&mut source);
    /// let mut header = FileHeader::default();
    /// BinaryFile::read_header_checked(&mut input, &mut header)?;
    /// assert_eq!(header.fields(), [0; 13]);
    /// # Ok::<(), CheckedReadError>(())
    /// ```
    pub fn read_header_checked(
        input: &mut dyn UnformattedInput,
        header: &mut FileHeader,
    ) -> Result<(), CheckedReadError> {
        input_fields::header(&mut CheckedReadSource::header(input), header)
    }
}

impl BinaryFile {
    /// Read container metadata through caller-owned checked input (#142).
    ///
    /// Section dispatch needs an actual position capability; this operation does
    /// not reposition or buffer the complete source. A complete header precedes dataset
    /// allocation, and complete info precedes header metadata publication.
    ///
    /// ```
    /// use martensite_tkernel::fsd::{BinaryFile, CheckedReadError, MemoryFileSystem};
    /// use martensite_tkernel::standard::Handle;
    /// use martensite_tkernel::storage::{Data, checked_input::UnformattedInput};
    /// use std::rc::Rc;
    /// fn read_info(input: &mut dyn UnformattedInput) -> Result<Handle<Data>, CheckedReadError> {
    ///     let mut driver = BinaryFile::new(Rc::new(MemoryFileSystem::default()));
    ///     let mut set = Handle::default();
    ///     driver.read_complete_info_checked(input, &mut set)?;
    ///     Ok(set)
    /// }
    /// # let _ = read_info;
    /// ```
    pub fn read_complete_info_checked(
        &mut self,
        input: &mut dyn UnformattedInput,
        set: &mut Handle<Data>,
    ) -> Result<(), CheckedReadError> {
        complete_info::read_container(&mut CheckedReadSource::container(input), set)
    }
}
