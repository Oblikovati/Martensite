//! AppCont: the continuous least-squares approximation of a function by a Bezier
//! MultiCurve — the user function ([`Function`]), the fit ([`LeastSquare`]) and the
//! precomputed Bernstein matrices it reads ([`cont_matrices`]).
pub mod cont_matrices;
mod cont_tables;
mod function;
mod least_square;

pub use function::{Function, FunctionBase};
pub use least_square::{LeastSquare, PeriodicityInfo};
