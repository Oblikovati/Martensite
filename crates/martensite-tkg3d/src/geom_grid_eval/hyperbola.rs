//! `GeomGridEvalHyperbola`: `P(u) = C + MajR * cosh(u) * X + MinR * sinh(u) * Y` on a grid.
use super::conic::{
    ConicTerms, conic_grid, conic_grid_d1, conic_grid_d2, conic_grid_d3, conic_grid_dn,
    conic_grid_members,
};
use super::frame::{GridFrame3, RadiiFrame3};
use super::{GeomGridEvalCurveD1, GeomGridEvalCurveD2, GeomGridEvalCurveD3};
use crate::geom::GeomHyperbola;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d_grid_eval::grid_loops::empty;
use martensite_tkmath::gp::{Hyperbola3, Point3, Vector3};

/// Batch evaluator of a `GeomHyperbola` (`GeomGridEvalHyperbola`).
///
/// The hyperbola is read once per call (`Hypr()`); a null hyperbola or empty
/// parameters give an empty result; the derivatives alternate between the
/// `(sinh, cosh)` and `(cosh, sinh)` combinations;
/// `GeomGridEvalHyperbola::new(hyperbola).evaluate_grid(&params)?`.
pub struct GeomGridEvalHyperbola {
    geom: Handle<GeomHyperbola>,
}

// The hyperbolic formulas on the radii frame.
struct HyperbolaTerms(RadiiFrame3);

impl ConicTerms for HyperbolaTerms {
    type Local = (f64, f64);
    fn local(&self, u: f64) -> (f64, f64) {
        (martensite_cmath::cosh(u), martensite_cmath::sinh(u))
    }
    fn point(&self, (ch, sh): (f64, f64)) -> Point3 {
        self.0.point(ch, sh)
    }
    fn d1(&self, (ch, sh): (f64, f64)) -> Vector3 {
        self.0.vector(sh, ch)
    }
    fn d2(&self, (ch, sh): (f64, f64)) -> Vector3 {
        self.0.vector(ch, sh)
    }
    fn d3(&self, local: (f64, f64)) -> Vector3 {
        self.d1(local)
    }
    // Period 2: odd orders are D1, even orders D2 (`(theN % 2) == 1`).
    fn dn(&self, u: f64, n: i32) -> Vector3 {
        let local: (f64, f64) = self.local(u);
        if n % 2 == 1 {
            self.d1(local)
        } else {
            self.d2(local)
        }
    }
}

impl GeomGridEvalHyperbola {
    /// Evaluator of `hyperbola` (`GeomGridEvalHyperbola(theHyperbola)`); a null handle
    /// is accepted; `GeomGridEvalHyperbola::new(hyperbola)`.
    pub fn new(hyperbola: Handle<GeomHyperbola>) -> Self {
        Self { geom: hyperbola }
    }
    fn terms(&self, params: &Array1<'_, f64>) -> Result<Option<HyperbolaTerms>, Failure> {
        let Some(hyperbola) = self.geom.get() else {
            return Ok(None);
        };
        if params.is_empty() {
            return Ok(None);
        }
        let hypr: Hyperbola3 = hyperbola.hypr()?;
        let (x_dir, y_dir) = (hypr.x_axis().direction(), hypr.y_axis().direction());
        Ok(Some(HyperbolaTerms(RadiiFrame3 {
            frame: GridFrame3::new(hypr.location(), x_dir, y_dir),
            major: hypr.major_radius(),
            minor: hypr.minor_radius(),
        })))
    }
    conic_grid_members!(GeomHyperbola);
}
