//! Checked header input reuses the owned reader's scalar and field decoding.
use super::{BinaryFile, FileHeader, header_input};
use crate::standard::Failure;
use crate::storage::{
    StreamException,
    checked_input::{InputFailure, UnformattedInput},
    state,
};
use std::{io, sync::Arc};

/// An owned checked-input failure or an incomplete binary field (#142).
/// Match `CheckedReadError::Input(fault)` to inspect input state and masks.
#[derive(Clone, Debug)]
pub enum CheckedReadError {
    /// A configured input exception, retaining its original typed cause.
    Input(InputFailure),
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
            Self::Format { failure, .. } => failure.fmt(output),
        }
    }
}
impl std::error::Error for CheckedReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Input(failure) => Some(failure),
            Self::Format {
                cause: Some(cause), ..
            } => Some(cause.as_ref()),
            Self::Format { failure, .. } => Some(failure),
        }
    }
}
fn incomplete_field(input: &dyn UnformattedInput) -> CheckedReadError {
    let message = format!(
        "binary header field received {} bytes; expected 4",
        input.gcount()
    );
    let cause = if input.state() & state::BAD != 0 {
        input.last_device_error().cloned()
    } else {
        None
    };
    CheckedReadError::Format {
        failure: StreamException::TypeMismatch.raise(&message),
        cause,
    }
}
fn checked_integer(
    input: &mut dyn UnformattedInput,
    value: &mut i32,
) -> Result<(), CheckedReadError> {
    header_input::read_integer(value, |bytes| {
        input.read(bytes).map_err(CheckedReadError::Input)?;
        if input.gcount() != 4 {
            return Err(incomplete_field(input));
        }
        Ok(())
    })
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
        header_input::read_fields(header, |field| checked_integer(input, field))
    }
}
