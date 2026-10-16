use super::{InputBuffer, InputContext, InputFailure, InputOperation, UnformattedInput};
use martensite_tkernel::storage::state;
use std::io;

impl<S: InputBuffer + ?Sized> UnformattedInput for InputContext<'_, S> {
    fn get(&mut self, output: &mut u8) -> Result<(), InputFailure> {
        self.gcount = 0;
        let mut flags = 0;
        if self.sentry(InputOperation::Get)? {
            match self.peek() {
                Ok(Some(byte)) => {
                    self.consume();
                    *output = byte;
                    self.gcount = 1;
                }
                Ok(None) => flags |= state::EOF,
                Err(cause) => self.device_error(cause, InputOperation::Get)?,
            }
        }
        if self.gcount == 0 {
            flags |= state::FAIL;
        }
        self.set_state(flags, InputOperation::Get)
    }
    fn getline(&mut self, output: &mut [u8], delimiter: u8) -> Result<(), InputFailure> {
        self.gcount = 0;
        let mut written = 0;
        let mut flags = 0;
        if self.sentry(InputOperation::Line)? {
            flags = self.line_body(output, delimiter, &mut written)?;
        }
        if let Some(terminator) = output.get_mut(written) {
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
        match self.read_body(output) {
            Ok(count) => {
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
}

impl<S: InputBuffer + ?Sized> InputContext<'_, S> {
    fn line_body(
        &mut self,
        output: &mut [u8],
        delimiter: u8,
        written: &mut usize,
    ) -> Result<u8, InputFailure> {
        match self.scan_line(output, delimiter, written) {
            Ok(flags) => Ok(flags),
            Err(cause) => {
                self.device_error(cause, InputOperation::Line)?;
                Ok(0)
            }
        }
    }
    fn scan_line(
        &mut self,
        output: &mut [u8],
        delimiter: u8,
        written: &mut usize,
    ) -> io::Result<u8> {
        let mut next = self.peek()?;
        while self.gcount + 1 < output.len() && next.is_some_and(|byte| byte != delimiter) {
            output[*written] = next.unwrap_or_default();
            *written += 1;
            self.consume();
            // Preserve the count of consumed output when the following lookahead fails.
            self.gcount += 1;
            next = self.peek()?;
        }
        match next {
            None => Ok(state::EOF),
            Some(byte) if byte == delimiter => {
                self.consume();
                self.gcount += 1;
                Ok(0)
            }
            Some(_) => Ok(state::FAIL),
        }
    }
    fn read_body(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let mut count = 0;
        while count < output.len() {
            let next = self.copy_prefix(&mut output[count..])?;
            if next == 0 {
                break;
            }
            count += next;
        }
        Ok(count)
    }
}

impl<S: InputBuffer + ?Sized> InputContext<'_, S> {
    fn copy_prefix(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let available = self.source.buffer().fill_buf()?;
        let count = output.len().min(available.len());
        output[..count].copy_from_slice(&available[..count]);
        self.source.buffer().consume(count);
        self.extracted += count;
        Ok(count)
    }
}
