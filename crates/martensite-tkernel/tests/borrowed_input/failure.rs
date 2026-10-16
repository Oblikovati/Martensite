//! Test-owned error vocabulary for checked input experiments.
use std::{io, sync::Arc};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum InputOperation {
    Get,
    Line,
    Read,
    Position,
    Mask,
    Clear,
}
#[derive(Clone, Debug)]
pub(crate) enum InputCause {
    Device(Arc<io::Error>),
    State,
}
#[derive(Clone, Debug)]
pub(crate) struct InputFailure {
    pub operation: InputOperation,
    pub state: u8,
    pub mask: u8,
    pub gcount: usize,
    pub extracted: usize,
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
