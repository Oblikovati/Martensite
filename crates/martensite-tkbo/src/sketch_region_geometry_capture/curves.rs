// SPDX-License-Identifier: GPL-2.0-only
//! Capture real immutable coefficient values, never rounded line endpoints.
use super::CapturedGeometryCause;
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg2d::geom2d::{Geom2dBezierCurve, Geom2dCurve, Geom2dLine};
use martensite_tkg3d::geom::{GeomBezierCurve, GeomCurve, GeomLine};
use martensite_tkmath::predicates::roots::metric_bounds::{MetricBezier, MetricCurve};
#[derive(Clone)]
pub(crate) enum CapturedMetricCurve<const DIM: usize> {
    Bezier {
        poles: Vec<[f64; DIM]>,
        weights: Option<Vec<f64>>,
    },
    Line {
        origin: [f64; DIM],
        direction: [f64; DIM],
    },
}
impl<const DIM: usize> CapturedMetricCurve<DIM> {
    pub(crate) fn borrow(&self) -> MetricCurve<'_, DIM> {
        match self {
            Self::Bezier { poles, weights } => MetricCurve::Bezier(MetricBezier {
                poles,
                weights: weights.as_deref(),
            }),
            Self::Line { origin, direction } => MetricCurve::Line {
                origin: *origin,
                direction: *direction,
            },
        }
    }
}
pub(crate) fn capture_space_curve(
    curve: &Handle<dyn GeomCurve>,
) -> Result<CapturedMetricCurve<3>, CapturedGeometryCause> {
    if let Some(bezier) = curve.downcast::<GeomBezierCurve>().get() {
        return Ok(space_bezier(bezier));
    }
    if let Some(line) = curve.downcast::<GeomLine>().get() {
        let frame = line.position();
        let p = frame.location();
        let d = frame.direction();
        return Ok(CapturedMetricCurve::Line {
            origin: [p.x(), p.y(), p.z()],
            direction: [d.x(), d.y(), d.z()],
        });
    }
    unsupported_curve(curve, "3D curve")
}
pub(crate) fn capture_plane_curve(
    curve: &Handle<dyn Geom2dCurve>,
) -> Result<CapturedMetricCurve<2>, CapturedGeometryCause> {
    if let Some(bezier) = curve.downcast::<Geom2dBezierCurve>().get() {
        return Ok(plane_bezier(bezier));
    }
    if let Some(line) = curve.downcast::<Geom2dLine>().get() {
        let p = line.location();
        let d = line.direction();
        return Ok(CapturedMetricCurve::Line {
            origin: [p.x(), p.y()],
            direction: [d.x(), d.y()],
        });
    }
    unsupported_curve(curve, "pcurve")
}
fn unsupported_curve<
    T: ?Sized + martensite_tkernel::standard::Transient + martensite_tkernel::standard::RuntimeTyped,
    const DIM: usize,
>(
    curve: &Handle<T>,
    representation: &'static str,
) -> Result<CapturedMetricCurve<DIM>, CapturedGeometryCause> {
    let Some(curve) = curve.get() else {
        return Err(CapturedGeometryCause::MissingCurve { representation });
    };
    Err(CapturedGeometryCause::Unsupported {
        representation,
        dynamic_type: curve.dynamic_type().name().to_owned(),
    })
}
fn space_bezier(curve: &GeomBezierCurve) -> CapturedMetricCurve<3> {
    let (poles, weights) = curve.poles_weights_snapshot();
    capture_bezier_coefficients(poles, weights, |pole| [pole.x(), pole.y(), pole.z()])
}
fn plane_bezier(curve: &Geom2dBezierCurve) -> CapturedMetricCurve<2> {
    let (poles, weights) = curve.poles_weights_snapshot();
    capture_bezier_coefficients(poles, weights, |pole| [pole.x(), pole.y()])
}
fn capture_bezier_coefficients<const DIM: usize, P: Copy>(
    poles: Array1<'_, P>,
    weights: Option<Array1<'_, f64>>,
    coordinates: impl FnMut(P) -> [f64; DIM],
) -> CapturedMetricCurve<DIM> {
    CapturedMetricCurve::Bezier {
        poles: poles.iter().copied().map(coordinates).collect(),
        weights: weights.map(|values| values.as_slice().to_vec()),
    }
}
