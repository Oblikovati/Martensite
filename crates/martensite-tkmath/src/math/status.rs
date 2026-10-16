//! Status discriminants.
/// Algorithm status independent of a solver's Done flag; `Status::TooManyIterations`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum Status {
    /// Ok.
    Ok = 0,
    /// TooManyIterations.
    TooManyIterations = 1,
    /// Math_FunctionError.
    FunctionError = 2,
    /// Math_DirectionSearchError.
    DirectionSearchError = 3,
    /// NotBracketed.
    NotBracketed = 4,
}
