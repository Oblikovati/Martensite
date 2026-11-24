//! `Law_BSpFunc`: a law evaluated on a `Law_BSpline` over a parameter range.
use super::bsp_func_intervals::IntervalKnots;
use super::{LawBSpline, LawFunction};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{
    Failure, Handle, HandleUpcast, RuntimeType, RuntimeTyped, SharedState, Transient,
};
use martensite_tkernel::{impl_transient, impl_transient_interface_target};
use martensite_tkg2d::geom2d::arrays::put;
use martensite_tkg2d::geom2d::errors;
use martensite_tkmath::geom_abs::Shape;

/// The fields of a B-spline law: the shared curve and the range.
#[derive(Clone, Default)]
struct BSpFuncFields {
    curv: Handle<LawBSpline>,
    first: f64,
    last: f64,
}

/// A law defined by a scalar B-spline between two parameters (`Law_BSpFunc`); the
/// curve is shared with the handle it was given.
///
/// `let law = LawBSpFunc::with_curve(&curve, 0.0, 1.0); law.value(0.5)?`.
#[derive(Default)]
pub struct LawBSpFunc {
    state: BSpFuncState,
}
impl_transient!(LawBSpFunc, "Law_BSpFunc", dyn LawFunction, interfaces: [dyn LawFunction, dyn BSplineLaw]);

/// B-spline laws share the `Law_BSpFunc` runtime class and its curve access.
/// Use `law.cast::<dyn BSplineLaw>()` for an owning view of a `Law_S` or `Law_Interpol`.
pub trait BSplineLaw: LawFunction {
    /// The curve shared by the law; `law.curve()`.
    fn curve(&self) -> Handle<LawBSpline>;

    /// Replace the curve and adopt its range; `law.set_curve(&curve)?`.
    fn set_curve(&self, curve: &Handle<LawBSpline>) -> Result<(), Failure>;
}

impl RuntimeTyped for dyn BSplineLaw {
    const TYPE_NAME: &'static str = LawBSpFunc::TYPE_NAME;

    fn static_type() -> &'static RuntimeType {
        LawBSpFunc::static_type()
    }
}
impl_transient_interface_target!(dyn BSplineLaw);

impl HandleUpcast<dyn LawFunction> for dyn BSplineLaw {
    fn upcast_arc(owner: std::sync::Arc<Self>) -> std::sync::Arc<dyn LawFunction> {
        owner
    }
}

#[derive(Default)]
pub(super) struct BSpFuncState {
    fields: SharedState<BSpFuncFields>,
}

pub(super) trait BSpFuncOwner: Transient {
    fn bsp_func_state(&self) -> &BSpFuncState;
}

/// `PosTol`: half of `Precision::PConfusion`.
fn pos_tol() -> f64 {
    precision::p_confusion() / 2.0
}

impl LawBSpFunc {
    /// A law without curve on `[0, 0]` (`Law_BSpFunc()`); its evaluators raise
    /// Standard_NullObject until a curve is set. `LawBSpFunc::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// The law of `c` on `[first, last]` (`Law_BSpFunc(C, First, Last)`).
    /// `LawBSpFunc::with_curve(&curve, 0.0, 1.0)`.
    pub fn with_curve(c: &Handle<LawBSpline>, first: f64, last: f64) -> Self {
        let law = Self::default();
        *law.state.fields.write() = BSpFuncFields {
            curv: c.clone(),
            first,
            last,
        };
        law
    }

    /// The curve (`Curve`), possibly null; `law.curve()`.
    pub fn curve(&self) -> Handle<LawBSpline> {
        self.state.curve()
    }

    /// Sets the curve and takes its parameter range (`SetCurve`);
    /// Standard_NullObject for a null curve. `law.set_curve(&curve)?`.
    pub fn set_curve(&self, c: &Handle<LawBSpline>) -> Result<(), Failure> {
        self.state.set_curve(c)
    }
}

impl BSpFuncOwner for LawBSpFunc {
    fn bsp_func_state(&self) -> &BSpFuncState {
        &self.state
    }
}

impl BSpFuncState {
    fn curve(&self) -> Handle<LawBSpline> {
        self.fields.read().curv.clone()
    }

    fn set_curve(&self, c: &Handle<LawBSpline>) -> Result<(), Failure> {
        let curve: &LawBSpline = live_curve(c)?;
        let (first, last) = (curve.first_parameter(), curve.last_parameter());
        *self.fields.write() = BSpFuncFields {
            curv: c.clone(),
            first,
            last,
        };
        Ok(())
    }

    /// A copy of the fields, so that no lock is held while the curve evaluates.
    fn snapshot(&self) -> BSpFuncFields {
        self.fields.read().clone()
    }
}

/// The curve behind a handle, or the Standard_NullObject of its native dereference.
pub(crate) fn live_curve(curv: &Handle<LawBSpline>) -> Result<&LawBSpline, Failure> {
    curv.get()
        .ok_or_else(|| errors::null_object("Law_BSpFunc: the law has no Law_BSpline curve"))
}

impl BSpFuncFields {
    /// The knot span `(Ideb, Ifin)` used at the range ends: `LocateU` of the end,
    /// widened to a non-empty span inside the knots.
    fn end_span(&self, curve: &LawBSpline, x: f64) -> Result<(i32, i32), Failure> {
        let (mut ideb, mut ifin): (i32, i32) = (0, 0);
        if x == self.first {
            (ideb, ifin) = curve.locate_u(self.first, pos_tol(), false)?;
            ideb = ideb.max(1);
            if ideb >= ifin {
                ifin = ideb + 1;
            }
        }
        if x == self.last {
            (ideb, ifin) = curve.locate_u(self.last, pos_tol(), false)?;
            ifin = ifin.min(curve.nb_knots());
            if ideb >= ifin {
                ideb = ifin - 1;
            }
        }
        Ok((ideb, ifin))
    }

    fn at_end(&self, x: f64) -> bool {
        x == self.first || x == self.last
    }

    pub(crate) fn value(&self, x: f64) -> Result<f64, Failure> {
        let curve: &LawBSpline = live_curve(&self.curv)?;
        if !self.at_end(x) {
            return curve.value(x);
        }
        let (ideb, ifin) = self.end_span(curve, x)?;
        curve.local_value(x, ideb, ifin)
    }

    pub(crate) fn d1(&self, x: f64) -> Result<(f64, f64), Failure> {
        let curve: &LawBSpline = live_curve(&self.curv)?;
        if !self.at_end(x) {
            return curve.d1(x);
        }
        let (ideb, ifin) = self.end_span(curve, x)?;
        curve.local_d1(x, ideb, ifin)
    }

    pub(crate) fn d2(&self, x: f64) -> Result<(f64, f64, f64), Failure> {
        let curve: &LawBSpline = live_curve(&self.curv)?;
        if !self.at_end(x) {
            return curve.d2(x);
        }
        let (ideb, ifin) = self.end_span(curve, x)?;
        curve.local_d2(x, ideb, ifin)
    }

    pub(crate) fn nb_intervals(&self, s: Shape) -> Result<i32, Failure> {
        let curve: &LawBSpline = live_curve(&self.curv)?;
        match IntervalKnots::compute(curve, self.first, self.last, s)? {
            Some(knots) => Ok(knots.count()),
            None => Ok(1),
        }
    }

    pub(crate) fn intervals(&self, t: &mut Array1<'_, f64>, s: Shape) -> Result<(), Failure> {
        let curve: &LawBSpline = live_curve(&self.curv)?;
        let mut nb_intervals: i32 = 1;
        if let Some(knots) = IntervalKnots::compute(curve, self.first, self.last, s)? {
            nb_intervals = knots.store(t)?;
        }
        let lower: i32 = t.lower();
        put(t, lower, self.first)?;
        put(t, lower + nb_intervals, self.last)
    }
}

impl<T: BSpFuncOwner> LawFunction for T {
    /// The continuity of the curve.
    fn continuity(&self) -> Result<Shape, Failure> {
        Ok(live_curve(&self.bsp_func_state().snapshot().curv)?.continuity())
    }
    /// The number of knot spans of the curve inside the range where it is less than
    /// C`s`; Standard_DomainError for G1 and G2.
    fn nb_intervals(&self, s: Shape) -> Result<i32, Failure> {
        self.bsp_func_state().snapshot().nb_intervals(s)
    }
    fn intervals(&self, t: &mut Array1<'_, f64>, s: Shape) -> Result<(), Failure> {
        self.bsp_func_state().snapshot().intervals(t, s)
    }
    /// The curve value; at the range ends it evaluates on the span inside the range.
    fn value(&self, x: f64) -> Result<f64, Failure> {
        self.bsp_func_state().snapshot().value(x)
    }
    fn d1(&self, x: f64) -> Result<(f64, f64), Failure> {
        self.bsp_func_state().snapshot().d1(x)
    }
    fn d2(&self, x: f64) -> Result<(f64, f64, f64), Failure> {
        self.bsp_func_state().snapshot().d2(x)
    }
    /// A `Law_BSpFunc` on the same curve and the new range.
    fn trim(
        &self,
        p_first: f64,
        p_last: f64,
        _tol: f64,
    ) -> Result<Handle<dyn LawFunction>, Failure> {
        let curve: Handle<LawBSpline> = self.bsp_func_state().snapshot().curv;
        Ok(Handle::new(LawBSpFunc::with_curve(&curve, p_first, p_last)).into_upcast())
    }
    fn bounds(&self) -> Result<(f64, f64), Failure> {
        let fields: BSpFuncFields = self.bsp_func_state().snapshot();
        Ok((fields.first, fields.last))
    }
}

impl<T: BSpFuncOwner> BSplineLaw for T {
    fn curve(&self) -> Handle<LawBSpline> {
        self.bsp_func_state().curve()
    }

    fn set_curve(&self, curve: &Handle<LawBSpline>) -> Result<(), Failure> {
        self.bsp_func_state().set_curve(curve)
    }
}
