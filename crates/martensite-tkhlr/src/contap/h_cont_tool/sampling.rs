//! the original process-wide sampled range retained as an explicit historical adapter.
use super::ContapHContTool;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle;
use martensite_tkmath::geom_abs::SurfaceType;
use std::sync::{PoisonError, RwLock};
pub(in crate::contap) mod range;
use range::{SamplingBounds, live_surface, spline_samples};
// Native file statics are observable across surfaces; retain them for compatibility.
static SAMPLED_RANGE: RwLock<[f64; 4]> = RwLock::new([0.; 4]);
struct HistoricalBounds;
impl SamplingBounds for HistoricalBounds {
    fn sampled_range(&self) -> [f64; 4] {
        *SAMPLED_RANGE.read().unwrap_or_else(PoisonError::into_inner)
    }
    fn store_bound(&mut self, index: usize, value: f64) {
        SAMPLED_RANGE
            .write()
            .unwrap_or_else(PoisonError::into_inner)[index] = value;
    }
    fn replace_bounds(&mut self, bounds: [f64; 4]) {
        *SAMPLED_RANGE
            .write()
            .unwrap_or_else(PoisonError::into_inner) = bounds;
    }
}
impl ContapHContTool {
    /// NbSamplesU(S, u1, u2): the U sample count by surface type (the range is not
    /// used); `ContapHContTool::nb_samples_u(&s, 0.0, 1.0)?`.
    pub fn nb_samples_u(s: &Adaptor3dSurfaceHandle, _u1: f64, _u2: f64) -> Result<i32, Failure> {
        Ok(match live_surface(s)?.get_type()? {
            SurfaceType::Plane => 2,
            SurfaceType::BezierSurface => 3 + s.nb_u_poles()?,
            SurfaceType::BSplineSurface => spline_samples(s.nb_u_knots()?, s.u_degree()?),
            SurfaceType::Torus => 20,
            _ => 10,
        })
    }

    /// NbSamplesV(S, v1, v2): the V sample count by surface type (the range is not
    /// used); `ContapHContTool::nb_samples_v(&s, 0.0, 1.0)?`.
    pub fn nb_samples_v(s: &Adaptor3dSurfaceHandle, _v1: f64, _v2: f64) -> Result<i32, Failure> {
        Ok(match live_surface(s)?.get_type()? {
            SurfaceType::Plane => 2,
            SurfaceType::BezierSurface => 3 + s.nb_v_poles()?,
            SurfaceType::BSplineSurface => spline_samples(s.nb_v_knots()?, s.v_degree()?),
            SurfaceType::Cylinder
            | SurfaceType::Cone
            | SurfaceType::Sphere
            | SurfaceType::Torus
            | SurfaceType::SurfaceOfRevolution
            | SurfaceType::SurfaceOfExtrusion => 15,
            _ => 10,
        })
    }

    /// NbSamplePoints(S): keeps the parametric range of `s` made finite (in the
    /// process-wide statics) and returns the number of sample points: 5, or for a
    /// B-spline the product of the thirds of the sample counts when it is larger;
    /// `ContapHContTool::nb_sample_points(&s)?`.
    pub fn nb_sample_points(s: &Adaptor3dSurfaceHandle) -> Result<i32, Failure> {
        range::sample_count(s, &mut HistoricalBounds)
    }

    /// SamplePoint(S, Index, U, V): the sample point `index` of the range kept by the
    /// last `NbSamplePoints` (of any surface): a grid point of a B-spline with more
    /// than 5 cells, else one of five fixed points of the range;
    /// `ContapHContTool::sample_point(&s, 1)?`.
    pub fn sample_point(s: &Adaptor3dSurfaceHandle, index: i32) -> Result<(f64, f64), Failure> {
        range::sample_point(s, index, &HistoricalBounds)
    }
}
