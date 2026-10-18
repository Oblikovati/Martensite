use super::{InputBuffer, InputContext, InputFailure, InputOperation, UnformattedInput};
use crate::storage::{
    state,
    stream::input_engine::{self, LineEnd, LineOutput},
};

impl<S: InputBuffer + ?Sized> UnformattedInput for InputContext<'_, S> {
    fn get(&mut self, output: &mut u8) -> Result<(), InputFailure> {
        self.gcount = 0;
        let mut flags = 0;
        if self.sentry(InputOperation::Get)? {
            let mut count = 0;
            let outcome = input_engine::scan_byte(self, output, &mut count);
            self.gcount = count;
            match outcome {
                Ok(()) if count == 0 => flags |= state::EOF,
                Err(cause) => self.device_error(cause, InputOperation::Get)?,
                _ => {}
            }
        }
        if self.gcount == 0 {
            flags |= state::FAIL;
        }
        self.set_state(flags, InputOperation::Get)
    }
    fn getline(&mut self, output: &mut [u8], delimiter: u8) -> Result<(), InputFailure> {
        self.gcount = 0;
        let mut line = SliceLine { output, written: 0 };
        let mut flags = 0;
        if self.sentry(InputOperation::Line)? {
            flags = self.checked_line(&mut line, delimiter)?;
        }
        if let Some(terminator) = line.output.get_mut(line.written) {
            *terminator = 0;
        }
        if self.gcount == 0 {
            flags |= state::FAIL;
        }
        self.set_state(flags, InputOperation::Line)
    }
    fn read(&mut self, output: &mut [u8]) -> Result<(), InputFailure> {
        self.gcount = 0;
        if !self.sentry(InputOperation::Read)? {
            return Ok(());
        }
        let mut count = 0;
        match input_engine::scan_block(self, output, &mut count) {
            Ok(()) => {
                self.gcount = count;
                if count != output.len() {
                    self.set_state(state::EOF | state::FAIL, InputOperation::Read)?;
                }
            }
            Err(cause) => self.device_error(cause, InputOperation::Read)?,
        }
        Ok(())
    }
    fn position(&mut self) -> Result<Option<u64>, InputFailure> {
        if !self.sentry(InputOperation::Position)? {
            return Ok(None);
        }
        match self.source.position() {
            Ok(position) => Ok(position),
            Err(cause) => {
                self.device_error(cause, InputOperation::Position)?;
                Ok(None)
            }
        }
    }
    fn state(&self) -> u8 {
        self.state
    }
    fn gcount(&self) -> usize {
        self.gcount
    }
    fn clear(&mut self) -> Result<(), InputFailure> {
        InputContext::clear(self)
    }
    fn exception_mask(&self) -> u8 {
        self.mask
    }
    fn set_exception_mask(&mut self, mask: u8) -> Result<(), InputFailure> {
        InputContext::set_exception_mask(self, mask)
    }
    fn last_device_error(&self) -> Option<&std::sync::Arc<std::io::Error>> {
        self.last_error.as_ref()
    }
}

impl<S: InputBuffer + ?Sized> InputContext<'_, S> {
    fn checked_line(
        &mut self,
        output: &mut SliceLine<'_>,
        delimiter: u8,
    ) -> Result<u8, InputFailure> {
        let mut count = 0;
        let outcome = input_engine::scan_line(self, output, delimiter, &mut count);
        self.gcount = count;
        match outcome {
            Ok(LineEnd::Delimiter) => Ok(0),
            Ok(LineEnd::Eof) => Ok(state::EOF),
            Ok(LineEnd::Capacity) => Ok(state::FAIL),
            Err(cause) => {
                self.device_error(cause, InputOperation::Line)?;
                Ok(0)
            }
        }
    }
}
struct SliceLine<'a> {
    output: &'a mut [u8],
    written: usize,
}
impl LineOutput for SliceLine<'_> {
    fn has_room(&self, count: usize) -> bool {
        count + 1 < self.output.len()
    }
    fn append(&mut self, byte: u8) {
        self.output[self.written] = byte;
        self.written += 1;
    }
}
