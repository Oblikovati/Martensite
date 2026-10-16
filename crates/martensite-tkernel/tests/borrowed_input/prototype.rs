//! Private executable design candidate, not an exported foundation API.
#[path = "failure.rs"]
mod failure;
#[path = "operations.rs"]
mod operations;
#[path = "source.rs"]
mod source;
pub(crate) use failure::{InputCause, InputFailure, InputOperation};
use martensite_tkernel::storage::state;
pub(crate) use source::{BufferedInput, InputBuffer, PositionedInput};
use std::{io, sync::Arc};

pub(crate) trait UnformattedInput {
    fn get(&mut self, output: &mut u8) -> Result<(), InputFailure>;
    fn getline(&mut self, output: &mut [u8], delimiter: u8) -> Result<(), InputFailure>;
    fn read(&mut self, output: &mut [u8]) -> Result<(), InputFailure>;
    fn position(&mut self) -> Result<Option<u64>, InputFailure>;
    fn state(&self) -> u8;
    fn gcount(&self) -> usize;
}

pub(crate) struct InputContext<'a, S: InputBuffer + ?Sized> {
    source: &'a mut S,
    state: u8,
    mask: u8,
    gcount: usize,
    extracted: usize,
    last_error: Option<Arc<io::Error>>,
}
impl<'a, S: InputBuffer + ?Sized> InputContext<'a, S> {
    pub(crate) fn new(source: &'a mut S) -> Self {
        Self {
            source,
            state: 0,
            mask: 0,
            gcount: 0,
            extracted: 0,
            last_error: None,
        }
    }
    pub(crate) fn exception_mask(&self) -> u8 {
        self.mask
    }
    pub(crate) fn last_device_error(&self) -> Option<&Arc<io::Error>> {
        self.last_error.as_ref()
    }
    pub(crate) fn set_exception_mask(&mut self, mask: u8) -> Result<(), InputFailure> {
        self.mask = mask;
        self.set_state(0, InputOperation::Mask)
    }
    pub(crate) fn clear(&mut self) -> Result<(), InputFailure> {
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
    fn peek(&mut self) -> io::Result<Option<u8>> {
        self.source
            .buffer()
            .fill_buf()
            .map(|bytes| bytes.first().copied())
    }
    fn consume(&mut self) {
        self.source.buffer().consume(1);
        self.extracted += 1;
    }
}
