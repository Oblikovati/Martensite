//! `GeomFill_TgtField`: the abstract field of tangents along a boundary.
use crate::law::LawBSpline;
use martensite_tkernel::impl_transient_interface;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient, Transient};
use martensite_tkmath::gp::Vector3;

/// A field of tangents along a boundary of a constrained filling
/// (`GeomFill_TgtField`), reached through `Handle<dyn GeomFillTgtField>`. The
/// evaluators return `Result`, because the boundaries behind a field raise natively;
/// the two `D1` overloads are `d1` and `d1_with_value`.
/// `let (t, dt) = field.d1_with_value(0.5)?;`
pub trait GeomFillTgtField: Transient {
    /// `IsScalable()`: false unless overridden.
    fn is_scalable(&self) -> bool {
        false
    }
    /// `Scale(Func)`: does nothing unless overridden.
    fn scale(&self, _func: &Handle<LawBSpline>) -> Result<(), Failure> {
        Ok(())
    }
    /// `Value(W)`: the tangent at `w`.
    fn value(&self, w: f64) -> Result<Vector3, Failure>;
    /// `D1(W)`: the derivative of the tangent at `w`.
    fn d1(&self, w: f64) -> Result<Vector3, Failure>;
    /// `D1(W, V, DV)`: the tangent and its derivative at `w`.
    fn d1_with_value(&self, w: f64) -> Result<(Vector3, Vector3), Failure>;
}
impl_transient_interface!(dyn GeomFillTgtField, "GeomFill_TgtField", StandardTransient);
