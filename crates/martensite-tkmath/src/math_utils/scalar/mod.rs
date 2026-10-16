//! Scalar functors.
use crate::math::{Function, FunctionWithDerivative, Vector};
mod arithmetic;
mod elementary;
mod lambda;
mod polynomial;
pub use arithmetic::*;
pub use elementary::*;
pub use lambda::*;
pub use polynomial::*;

mod power_gaussian;
pub use power_gaussian::*;
