//! Shared owned geometry and original expected length for retained curve cases.
use martensite_tkernel::standard::{Handle, RuntimeTyped};
/// Original input/result geometry paired with its independently supplied reference.
pub struct CurveLengthCase<C: ?Sized + RuntimeTyped> {
    pub curve: Handle<C>,
    pub expected: f64,
    pub name: &'static str,
}
impl<C: ?Sized + RuntimeTyped> CurveLengthCase<C> {
    /// Keep source case identity with its original result; `CurveLengthCase::new("case", curve, 5.0)`.
    pub fn new(name: &'static str, curve: Handle<C>, expected: f64) -> Self {
        Self {
            curve,
            expected,
            name,
        }
    }
}
