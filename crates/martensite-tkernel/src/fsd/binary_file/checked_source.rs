//! Checked source policy retains failures without replacing field decoding.
use super::{CheckedReadError, input_source::BinaryReadSource};
use crate::storage::{StreamException, checked_input::UnformattedInput, state};
use std::{io, sync::Arc};

pub(super) struct CheckedReadSource<'a> {
    input: &'a mut dyn UnformattedInput,
    integer_context: &'static str,
}
impl<'a> CheckedReadSource<'a> {
    pub(super) fn header(input: &'a mut dyn UnformattedInput) -> Self {
        Self {
            input,
            integer_context: "header field",
        }
    }
    pub(super) fn container(input: &'a mut dyn UnformattedInput) -> Self {
        Self {
            input,
            integer_context: "container integer",
        }
    }
    fn device_cause(&self) -> Option<Arc<io::Error>> {
        if self.input.state() & state::BAD != 0 {
            return self.input.last_device_error().cloned();
        }
        None
    }
    fn incomplete(&self, kind: StreamException, message: String) -> CheckedReadError {
        CheckedReadError::Format {
            failure: kind.raise(&message),
            cause: self.device_cause(),
        }
    }
}
impl BinaryReadSource for CheckedReadSource<'_> {
    type Error = CheckedReadError;
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), CheckedReadError> {
        self.input.read(bytes).map_err(CheckedReadError::Input)
    }
    fn count(&self) -> usize {
        self.input.gcount()
    }
    fn good(&self) -> bool {
        self.input.good()
    }
    fn eof(&self) -> bool {
        self.input.eof()
    }
    fn position(&mut self) -> Result<i32, CheckedReadError> {
        if let Some(position) = self.input.position().map_err(CheckedReadError::Input)? {
            return Ok(position as i32);
        }
        Err(CheckedReadError::PositionUnavailable {
            state: self.input.state(),
            mask: self.input.exception_mask(),
            gcount: self.input.gcount(),
            cause: self.device_cause(),
        })
    }
    fn incomplete_integer(&self) -> CheckedReadError {
        self.incomplete(
            StreamException::TypeMismatch,
            format!(
                "binary {} received {} bytes; expected 4",
                self.integer_context,
                self.count()
            ),
        )
    }
    fn incomplete_block(&self, _member: &str, expected: usize) -> CheckedReadError {
        self.incomplete(
            StreamException::Read,
            format!(
                "binary string field received {} bytes; expected {expected}",
                self.count()
            ),
        )
    }
}
