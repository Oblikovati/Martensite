//! Input state belongs to the context; the source buffer remains borrowed.
#[path = "operations.rs"]
mod operations;
use super::{InputBuffer, InputCause, InputFailure, InputOperation, UnformattedInput};
use crate::storage::{state, stream::input_engine::InputBytes};
use std::{io, sync::Arc};

/// Borrowed input with independent state, mask and retained causes.
///
/// ```
/// use martensite_tkernel::storage::checked_input::{BufferedInput, InputContext, UnformattedInput};
/// let mut bytes = b"x>tail".as_slice();
/// let mut source = BufferedInput(&mut bytes);
/// let mut input = InputContext::new(&mut source);
/// let mut line = [0; 8];
/// input.getline(&mut line, b'>')?;
/// assert_eq!(&line[..2], b"x\0");
/// # Ok::<(), martensite_tkernel::storage::checked_input::InputFailure>(())
/// ```
pub struct InputContext<'a, S: InputBuffer + ?Sized> {
    source: &'a mut S,
    state: u8,
    mask: u8,
    gcount: usize,
    extracted: usize,
    last_error: Option<Arc<io::Error>>,
}
impl<'a, S: InputBuffer + ?Sized> InputContext<'a, S> {
    /// Start with good state and no exceptions; `InputContext::new(&mut source)`.
    pub fn new(source: &'a mut S) -> Self {
        Self {
            source,
            state: 0,
            mask: 0,
            gcount: 0,
            extracted: 0,
            last_error: None,
        }
    }
    /// Observe the active exception bits; `input.exception_mask()`.
    pub fn exception_mask(&self) -> u8 {
        self.mask
    }
    /// Borrow the original retained device cause; `input.last_device_error()`.
    pub fn last_device_error(&self) -> Option<&Arc<io::Error>> {
        self.last_error.as_ref()
    }
    /// Store the new mask before checking state; `input.set_exception_mask(1)?`.
    pub fn set_exception_mask(&mut self, mask: u8) -> Result<(), InputFailure> {
        self.mask = mask;
        self.set_state(0, InputOperation::Mask)
    }
    /// Clear flags, keeping position, counts and cause; `input.clear()?`.
    pub fn clear(&mut self) -> Result<(), InputFailure> {
        self.state = 0;
        self.set_state(0, InputOperation::Clear)
    }
    fn set_state(&mut self, bits: u8, operation: InputOperation) -> Result<(), InputFailure> {
        self.state |= bits;
        if self.mask & self.state != 0 {
            return Err(self.failure(operation, InputCause::State));
        }
        Ok(())
    }
    fn failure(&self, operation: InputOperation, cause: InputCause) -> InputFailure {
        InputFailure {
            operation,
            state: self.state,
            mask: self.mask,
            gcount: self.gcount,
            extracted: self.extracted,
            cause,
        }
    }
    fn device_error(
        &mut self,
        cause: io::Error,
        operation: InputOperation,
    ) -> Result<(), InputFailure> {
        let cause = Arc::new(cause);
        self.last_error = Some(cause.clone());
        self.state |= state::BAD;
        if self.mask & state::BAD != 0 {
            return Err(self.failure(operation, InputCause::Device(cause)));
        }
        Ok(())
    }
    fn sentry(&mut self, operation: InputOperation) -> Result<bool, InputFailure> {
        if self.state == 0 {
            return Ok(true);
        }
        self.set_state(state::FAIL, operation)?;
        Ok(false)
    }
}

impl<S: InputBuffer + ?Sized> InputBytes for InputContext<'_, S> {
    fn peek(&mut self) -> io::Result<Option<u8>> {
        self.source
            .buffer()
            .fill_buf()
            .map(|bytes| bytes.first().copied())
    }
    fn consume(&mut self, count: usize) {
        self.source.buffer().consume(count);
        self.extracted += count;
    }
    fn copy_prefix(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let available = self.source.buffer().fill_buf()?;
        let count = output.len().min(available.len());
        output[..count].copy_from_slice(&available[..count]);
        self.consume(count);
        Ok(count)
    }
}
