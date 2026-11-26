//! `GeomAPI_IntSS`: the intersection curves of two Geom surfaces, the approximated
//! 3d curves of `GeomInt_IntSS` without their 2d curves.
use crate::geom_int::{GeomIntIntSS, GeomIntIntSSApprox};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};

/// `GeomAPI_IntSS`: the intersection of two surfaces with approximated walking lines
/// and no 2d curves. For most cases the tolerance `1.0e-7` is recommended.
///
/// ```
/// use martensite_tkgeomalgo::geom_api::IntSS;
/// let inter = IntSS::new();
/// assert!(!inter.is_done());
/// assert_eq!(inter.nb_lines(), 0);
/// assert!(inter.line(1).is_err());
/// ```
#[derive(Default)]
pub struct IntSS {
    my_intersec: GeomIntIntSS,
}

/// The flags of `GeomInt_IntSS` that `GeomAPI_IntSS` passes (`Approx` true only).
fn approx_flags() -> GeomIntIntSSApprox {
    GeomIntIntSSApprox {
        approx: true,
        approx_s1: false,
        approx_s2: false,
    }
}

impl IntSS {
    /// `GeomAPI_IntSS()`: empty; `IntSS::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// `GeomAPI_IntSS(S1, S2, Tol)`: the intersection curves of `s1` and `s2` computed
    /// with the precision `tol`; `IntSS::with_surfaces(&s1, &s2, 1.0e-7)?`.
    pub fn with_surfaces(
        s1: &Handle<dyn GeomSurface>,
        s2: &Handle<dyn GeomSurface>,
        tol: f64,
    ) -> Result<Self, Failure> {
        Ok(Self {
            my_intersec: GeomIntIntSS::with_surfaces(s1, s2, tol, approx_flags())?,
        })
    }

    /// `Perform(S1, S2, Tol)`: computes the intersection curves again;
    /// `inter.perform(&s1, &s2, 1.0e-7)?`.
    pub fn perform(
        &mut self,
        s1: &Handle<dyn GeomSurface>,
        s2: &Handle<dyn GeomSurface>,
        tol: f64,
    ) -> Result<(), Failure> {
        self.my_intersec.perform(s1, s2, tol, approx_flags())
    }

    /// `IsDone()`: whether the intersection was successful; `inter.is_done()`.
    pub fn is_done(&self) -> bool {
        self.my_intersec.is_done()
    }

    /// Returns a structured completeness and per-curve fitting-error receipt (#38).
    ///
    /// When `nb_lines()` exceeds `max_curves_budget` or `!is_done()`, returns
    /// [`crate::completeness::IntersectionCompleteness::Inconclusive`].
    pub fn completeness_receipt(
        &self,
        max_curves_budget: usize,
    ) -> crate::completeness::SsiCompletenessReceipt {
        use crate::completeness::{
            IntersectionCompleteness, SsiCompletenessReceipt, SsiCurveReceipt,
        };
        if !self.is_done() {
            return SsiCompletenessReceipt {
                completeness: IntersectionCompleteness::Inconclusive {
                    reason: "IntSS did not complete".to_owned(),
                    budget_exhausted: false,
                },
                curves: Vec::new(),
            };
        }
        let count = self.nb_lines().max(0) as usize;
        let tol = self.my_intersec.tol_reached3d();
        let curves: Vec<SsiCurveReceipt> = (1..=count.min(max_curves_budget))
            .map(|curve_index| SsiCurveReceipt {
                curve_index,
                fitting_error: tol,
                near_tangent_uncertainty: None,
            })
            .collect();
        let completeness = if count > max_curves_budget {
            IntersectionCompleteness::Inconclusive {
                reason: format!("SSI curve count {count} exceeded budget {max_curves_budget}"),
                budget_exhausted: true,
            }
        } else {
            IntersectionCompleteness::Complete
        };
        SsiCompletenessReceipt {
            completeness,
            curves,
        }
    }

    /// `NbLines()`: the number of intersection curves (not checked for a failed
    /// computation, as natively); `inter.nb_lines()`.
    pub fn nb_lines(&self) -> i32 {
        self.my_intersec.nb_lines()
    }

    /// `Line(Index)`: the intersection curve `index` (StdFail_NotDone when the
    /// computation failed, Standard_OutOfRange outside `1..=NbLines`);
    /// `inter.line(1)?`.
    pub fn line(&self, index: i32) -> Result<Handle<dyn GeomCurve>, Failure> {
        self.my_intersec.line(index)
    }
}
