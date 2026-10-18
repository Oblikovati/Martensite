//! Owned checked-input failures retain the original typed cause.
use std::{io, sync::Arc};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Operation whose state or device failed; `InputOperation::Read`.
pub enum InputOperation {
    /// Byte extraction.
    Get,
    /// Delimited line extraction.
    Line,
    /// Block extraction.
    Read,
    /// Actual position query.
    Position,
    /// Exception-mask update.
    Mask,
    /// State clearing.
    Clear,
}
#[derive(Clone, Debug)]
/// Distinguish an original device error from a generated state failure.
pub enum InputCause {
    /// The retained original typed error.
    Device(Arc<io::Error>),
    /// State intersects the configured exception mask.
    State,
}
#[derive(Clone, Debug)]
/// Owned diagnostic snapshot; `failure.state` records state at failure.
pub struct InputFailure {
    /// The operation that failed.
    pub operation: InputOperation,
    /// Observed state bits.
    pub state: u8,
    /// Active exception bits.
    pub mask: u8,
    /// Last published input count.
    pub gcount: usize,
    /// Physical bytes consumed by this context.
    pub extracted: usize,
    /// Original error or generated state classification.
    pub cause: InputCause,
}

impl std::fmt::Display for InputFailure {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            output,
            "input {:?}: state {}, mask {}, count {}, extracted {}",
            self.operation, self.state, self.mask, self.gcount, self.extracted
        )?;
        match &self.cause {
            InputCause::Device(cause) => write!(output, ": {cause}"),
            InputCause::State => output.write_str("; expected state bits outside mask"),
        }
    }
}
impl std::error::Error for InputFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.cause {
            InputCause::Device(cause) => Some(cause.as_ref()),
            InputCause::State => None,
        }
    }
}
