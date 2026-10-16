//! Checked input over caller-owned buffers, with state and original failure causes.
mod context;
mod failure;
mod source;
pub use context::InputContext;
pub use failure::{InputCause, InputFailure, InputOperation};
pub use source::{BufferedInput, InputBuffer, PositionedBuffer, PositionedInput};
use std::{io, sync::Arc};

/// Consumer seam for checked byte, line and block extraction (#142).
/// An `Ok` result means no enabled exception; callers also inspect state/count.
/// For example, `consumer.read(&mut bytes)?; assert_eq!(consumer.gcount(), bytes.len());`.
pub trait UnformattedInput {
    /// Extract one byte, preserving output at EOF; `input.get(&mut byte)?`.
    fn get(&mut self, output: &mut u8) -> Result<(), InputFailure>;
    /// Store a NUL-terminated line and consume its delimiter; `input.getline(&mut line, b'>')?`.
    fn getline(&mut self, output: &mut [u8], delimiter: u8) -> Result<(), InputFailure>;
    /// Fill the requested block, retaining partial bytes; `input.read(&mut block)?`.
    fn read(&mut self, output: &mut [u8]) -> Result<(), InputFailure>;
    /// Query actual position when supported; `let offset = input.position()?`.
    fn position(&mut self) -> Result<Option<u64>, InputFailure>;
    /// Observe state bits; `assert_eq!(input.state(), 0)`.
    fn state(&self) -> u8;
    /// Observe the last published extraction count; `let count = input.gcount()`.
    fn gcount(&self) -> usize;
    /// Clear flags without moving or discarding the cause; `input.clear()?`.
    fn clear(&mut self) -> Result<(), InputFailure>;
    /// Observe the exception mask; `assert_eq!(input.exception_mask(), 0)`.
    fn exception_mask(&self) -> u8;
    /// Store a mask, then check existing state; `input.set_exception_mask(1)?`.
    fn set_exception_mask(&mut self, mask: u8) -> Result<(), InputFailure>;
    /// Borrow the retained original error; `let cause = input.last_device_error()`.
    fn last_device_error(&self) -> Option<&Arc<io::Error>>;
    /// Test all state bits; `assert!(input.good())`.
    fn good(&self) -> bool {
        self.state() == 0
    }
    /// Test end-of-input; `assert!(!input.eof())`.
    fn eof(&self) -> bool {
        self.state() & super::state::EOF != 0
    }
    /// Test bad or failed input; `assert!(!input.fail())`.
    fn fail(&self) -> bool {
        self.state() & (super::state::BAD | super::state::FAIL) != 0
    }
}
