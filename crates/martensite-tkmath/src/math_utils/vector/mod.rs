//! Vector functors.
//! Explicit dimension metadata adapts variable-size functors to the shared Rust objective trait.
use crate::math::{
    FunctionSet, Matrix, MultipleVarFunction, MultipleVarFunctionWithGradient, Vector,
};
use crate::math_utils::{EULER_NUMBER, TWO_PI};
mod linear;
mod planar;
mod system;
mod variable;
pub use linear::*;
pub use planar::*;
pub use system::*;
pub use variable::*;
