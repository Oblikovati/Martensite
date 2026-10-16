//! Root and minimum bracketing.
use super::utils::{evaluate_finite, is_valid, limited};
use super::{GOLDEN_RATIO, Status};
use crate::math::Function;
mod minimum;
mod options;
mod root;
pub use minimum::{MinBracketResult, bracket_minimum, bracket_minimum_iterations};
pub use options::MinBracketOptions;
pub use root::{BracketResult, bracket_root};

fn expansion(point: f64, previous: f64) -> f64 {
    point + GOLDEN_RATIO * (point - previous)
}
