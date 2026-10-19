//! Carries a retained exception raised inside a `Function` callback across a math
//! solver.
//!
//! Natively the exception unwinds through the solver to the caller's `catch`. The
//! Rust solvers take `bool`-returning callbacks, so a callback that fails records
//! its [`Failure`], answers `false`, and the caller checks [`FailureCapture::take`]
//! as soon as the solver returns: the solver's result is then discarded exactly as
//! the retained unwinding discards it. Only the first failure is kept, since the
//! retained unwinding stops at it.
use martensite_tkernel::standard::Failure;

/// The first failure raised by the callbacks of one solver run;
/// `let mut capture = FailureCapture::default();`.
#[derive(Clone, Default)]
pub struct FailureCapture {
    failure: Option<Failure>,
}

impl FailureCapture {
    /// The value of a fallible evaluation, or `None` after recording its failure;
    /// `capture.keep(curve.value(u))`.
    pub fn keep<T>(&mut self, value: Result<T, Failure>) -> Option<T> {
        match value {
            Ok(value) => Some(value),
            Err(failure) => {
                self.failure.get_or_insert(failure);
                None
            }
        }
    }
    /// The recorded failure, if any, leaving the capture empty; `capture.take()?`.
    pub fn take(&mut self) -> Result<(), Failure> {
        match self.failure.take() {
            Some(failure) => Err(failure),
            None => Ok(()),
        }
    }
}
