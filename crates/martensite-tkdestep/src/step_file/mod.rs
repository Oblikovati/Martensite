//! `StepFile`: the reading of the text of a STEP file: a scanner and a parser that build the
//! records of a `StepDataStepReaderData`. The flex scanner and bison parser are
//! replaced by [`scanner`] and [`parser`], which accept the same language and report the same
//! errors, recovery included.
mod parser;
mod read;
mod read_data;
mod scanner;

pub use read::{step_file_interrupt, step_file_read};
pub use read_data::StepFileReadData;
