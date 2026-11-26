// SPDX-License-Identifier: GPL-2.0-only
//! Completeness and uncertainty receipts for SSI (#38) and finite-domain 2D curve intersections (#568).

/// Search completeness state reported by an intersection query (#38, #568).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntersectionCompleteness {
    /// All intersection candidates inside the queried domain were certified.
    Complete,
    /// Search stopped or could not certify domain completeness.
    Inconclusive {
        /// Diagnostic reason for the inconclusive outcome.
        reason: String,
        /// True when a work or curve-count budget was exhausted.
        budget_exhausted: bool,
    },
}

/// Per-curve accuracy and near-tangent uncertainty receipt for SSI (#38).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub struct SsiCurveReceipt {
    /// 1-based intersection curve index.
    pub curve_index: usize,
    /// Measured maximum fitting residual against both surfaces.
    pub fitting_error: f64,
    /// Optional near-tangent angular/normal uncertainty when surfaces graze.
    pub near_tangent_uncertainty: Option<f64>,
}

/// Complete SSI receipt separating completeness, per-curve fitting error, and uncertainty (#38).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub struct SsiCompletenessReceipt {
    /// Overall completeness classification.
    pub completeness: IntersectionCompleteness,
    /// Per-curve fitting and uncertainty receipts in deterministic curve order.
    pub curves: Vec<SsiCurveReceipt>,
}

impl SsiCompletenessReceipt {
    /// Returns true only when completeness is [`IntersectionCompleteness::Complete`].
    pub fn is_complete(&self) -> bool {
        matches!(self.completeness, IntersectionCompleteness::Complete)
    }
}

/// Completeness receipt for finite-domain 2D curve intersection queries (#568).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Curve2dIntersectionReceipt {
    /// Overall completeness classification.
    pub completeness: IntersectionCompleteness,
    /// Number of isolated intersection points reported.
    pub point_count: usize,
    /// Number of overlap segments reported.
    pub segment_count: usize,
    /// Whether the finite query domain was certified without omitted roots.
    pub domain_certified: bool,
}

impl Curve2dIntersectionReceipt {
    /// Builds a receipt from solver completion state, counts, and a work budget (#568).
    pub fn from_solver_state(
        is_done: bool,
        point_count: usize,
        segment_count: usize,
        budget_exhausted: bool,
    ) -> Self {
        if !is_done {
            return Self {
                completeness: IntersectionCompleteness::Inconclusive {
                    reason: "2D curve intersection did not complete".to_owned(),
                    budget_exhausted,
                },
                point_count,
                segment_count,
                domain_certified: false,
            };
        }
        if budget_exhausted {
            return Self {
                completeness: IntersectionCompleteness::Inconclusive {
                    reason: "2D curve intersection exhausted its search budget".to_owned(),
                    budget_exhausted: true,
                },
                point_count,
                segment_count,
                domain_certified: false,
            };
        }
        Self {
            completeness: IntersectionCompleteness::Complete,
            point_count,
            segment_count,
            domain_certified: true,
        }
    }

    /// Returns true when the finite domain is certified complete for snap selection.
    pub fn is_complete(&self) -> bool {
        self.domain_certified && matches!(self.completeness, IntersectionCompleteness::Complete)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom_api::IntSS;
    use martensite_tkernel::standard::Handle;
    use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
    use martensite_tkmath::gp::{Axis3, Direction3, Point3};

    #[test]
    fn ssi_receipt_reports_fitting_error_and_budget_exhaustion_inconclusive() {
        let p1: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(Axis3::default())).cast();
        let p2: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(
            Axis3::new(Point3::new(0.0, 0.0, 0.0), Direction3::Y, Direction3::X).unwrap(),
        ))
        .cast();
        let ssi = IntSS::with_surfaces(&p1, &p2, 1e-7).unwrap();
        assert!(ssi.is_done());

        let complete = ssi.completeness_receipt(16);
        assert!(complete.is_complete());
        assert_eq!(complete.curves.len(), 1);
        assert!(complete.curves[0].fitting_error.is_finite());

        // Exhausted budget (0 allowed curves when 1 exists) must report Inconclusive (#38).
        let exhausted = ssi.completeness_receipt(0);
        assert!(!exhausted.is_complete());
        assert!(matches!(
            exhausted.completeness,
            IntersectionCompleteness::Inconclusive {
                budget_exhausted: true,
                ..
            }
        ));
    }

    #[test]
    fn curve2d_receipt_marks_budget_exhaustion_inconclusive() {
        let ok = Curve2dIntersectionReceipt::from_solver_state(true, 2, 0, false);
        assert!(ok.is_complete());
        let inc = Curve2dIntersectionReceipt::from_solver_state(true, 2, 0, true);
        assert!(!inc.is_complete());
        assert!(!inc.domain_certified);
    }
}
