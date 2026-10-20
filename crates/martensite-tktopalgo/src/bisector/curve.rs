//! `Bisector_Curve`: the abstract bisector curve, a `Geom2d_Curve` with a parameter
//! of a point, extensions and intervals of continuity.
use martensite_tkernel::impl_transient_interface;
use martensite_tkernel::standard::Failure;
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkmath::gp::Point2;

/// `Bisector_Curve`: the members every bisector curve adds to `Geom2d_Curve`. A trimmed curve whose basis is a bisector
/// gives it back with `trimmed.basis_curve().cast::<dyn BisectorCurve>()`.
/// ```
/// use martensite_tktopalgo::bisector::{BisecAna, BisectorCurve};
/// use martensite_tkernel::standard::Handle;
/// let bisector: Handle<dyn BisectorCurve> = Handle::new(BisecAna::new()).cast();
/// assert!(!bisector.is_extend_at_start() && bisector.nb_intervals() == 1);
/// ```
pub trait BisectorCurve: Geom2dCurve {
    /// The parameter of the point `p` of the bisector (`Parameter`); `bis.parameter(p)?`.
    fn parameter(&self, p: Point2) -> Result<f64, Failure>;
    /// True when the start of the curve is an extension (`IsExtendAtStart`).
    fn is_extend_at_start(&self) -> bool;
    /// True when the end of the curve is an extension (`IsExtendAtEnd`).
    fn is_extend_at_end(&self) -> bool;
    /// The number of intervals of continuity C1 (`NbIntervals`); `bis.nb_intervals()`.
    fn nb_intervals(&self) -> i32;
    /// The first parameter of the interval `index` (`IntervalFirst`); `bis.interval_first(1)?`.
    fn interval_first(&self, index: i32) -> Result<f64, Failure>;
    /// The last parameter of the interval `index` (`IntervalLast`); `bis.interval_last(1)?`.
    fn interval_last(&self, index: i32) -> Result<f64, Failure>;
}
impl_transient_interface!(dyn BisectorCurve, "Bisector_Curve", dyn Geom2dCurve);
