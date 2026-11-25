//! Reuse an already certified full-domain distance on the same native edge/face use.
use martensite_tkbrep::brep::BRepCurveRepresentation;
use martensite_tkbrep::topods::{Edge, Face, Shape};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dBezierCurve, Geom2dCurve};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::geom::{GeomBezierCurve, GeomSurface};
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkmath::gp::Transform3;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedCylindricalTrim, CylindricalTrimQuery, MetricBezier,
};
use std::collections::HashMap;
use std::sync::Arc;

mod placement;
#[cfg(test)]
mod tests;

/// Candidate full-domain curve-on-cylinder bounds for one native validation operation.
///
/// A bound cannot supply a checker status. The analyzer rebinds it to the actual
/// curve, pcurve, carrier, domain, placement and native tolerance before it uses it.
/// Example: `let bounds = CylindricalCurveBounds::default();`.
#[derive(Default)]
pub struct CylindricalCurveBounds {
    by_edge: HashMap<usize, Vec<BoundUse>>,
}

const _: fn() = || {
    fn assert_thread_safe<T: Send + Sync>() {}
    assert_thread_safe::<CylindricalCurveBounds>();
};

struct BoundUse {
    edge: Edge,
    face: Face,
    proof: Arc<CertifiedCylindricalTrim>,
}

impl CylindricalCurveBounds {
    /// Add a prospective bound for one edge occurrence in one face occurrence.
    /// Example: `bounds.insert(edge, face, proof);`.
    pub fn insert(&mut self, edge: Edge, face: Face, proof: Arc<CertifiedCylindricalTrim>) {
        self.by_edge
            .entry(edge.tshape_address())
            .or_default()
            .push(BoundUse { edge, face, proof });
    }

    pub(super) fn find(&self, edge: &Shape, face: &Shape) -> Option<&CertifiedCylindricalTrim> {
        self.by_edge
            .get(&edge.tshape_address())?
            .iter()
            .find(|item| item.edge.is_same(edge) && item.face.is_same(face))
            .map(|item| item.proof.as_ref())
    }
}

pub(super) struct CylinderMetricSource<'a> {
    pub reference: &'a dyn BRepCurveRepresentation,
    pub loaded_reference: &'a Handle<dyn Adaptor3dCurve>,
    pub surface: &'a Handle<dyn GeomSurface>,
    pub moved_surface: &'a Handle<dyn GeomSurface>,
    pub trace: &'a Handle<dyn Geom2dCurve>,
    pub reference_transform: Transform3,
    pub surface_transform: Transform3,
}

impl CylinderMetricSource<'_> {
    pub(super) fn matches(&self, proof: &CertifiedCylindricalTrim, edge_limit: f64) -> bool {
        if !edge_limit.is_finite() || proof.spatial_residual_bound() >= edge_limit {
            return false;
        }
        self.matches_source(proof, edge_limit).unwrap_or(false)
    }

    fn matches_source(&self, proof: &CertifiedCylindricalTrim, edge_limit: f64) -> Option<bool> {
        let datum = placement::admitted(self)?;
        let pcurve = polynomial_chart(self.trace, datum.height_sign)?;
        let rail = self.reference.curve3d().ok()?.downcast::<GeomBezierCurve>();
        let rail = GeomBezierCurve::copy_of(rail.get()?).ok()?;
        let (poles, weights) = curve_controls(&rail)?;
        let location_error = loaded_curve_shift_error(self, &poles, &weights)?;
        if (proof.spatial_residual_bound() + location_error).next_up() >= edge_limit {
            return Some(false);
        }
        Some(proof.matches_source(CylindricalTrimQuery {
            curve: MetricBezier {
                poles: &poles,
                weights: Some(&weights),
            },
            pcurve: &pcurve,
            radius: datum.radius,
            spatial_limit: proof.requested_spatial_limit(),
            options: proof.requested_options(),
        }))
    }
}

fn polynomial_chart(trace: &Handle<dyn Geom2dCurve>, height_sign: f64) -> Option<Vec<[f64; 2]>> {
    let trace = trace.downcast::<Geom2dBezierCurve>();
    let trace = Geom2dBezierCurve::copy_of(trace.get()?).ok()?;
    if trace.has_eval_representation()
        || trace.is_rational()
        || !(2..=4).contains(&trace.nb_poles())
    {
        return None;
    }
    Some(
        trace
            .poles()
            .iter()
            .map(|p| [p.x(), height_sign * p.y()])
            .collect(),
    )
}

type CurveControls = (Vec<[f64; 3]>, Vec<f64>);

fn curve_controls(rail: &GeomBezierCurve) -> Option<CurveControls> {
    if rail.has_eval_representation() || !(2..=10).contains(&rail.nb_poles()) {
        return None;
    }
    let (poles, weights) = rail.poles_weights_snapshot();
    let weights = weights.map_or_else(|| vec![1.; poles.len()], |w| w.as_slice().to_vec());
    if weights.len() != poles.len() {
        return None;
    }
    Some((
        poles.iter().map(|point| *point.coordinates()).collect(),
        weights,
    ))
}

fn loaded_curve_shift_error(
    source: &CylinderMetricSource<'_>,
    poles: &[[f64; 3]],
    weights: &[f64],
) -> Option<f64> {
    let reference = source.loaded_reference.downcast::<GeomAdaptorCurve>();
    let curve = reference.get()?.curve().downcast::<GeomBezierCurve>();
    let (loaded, loaded_weights) = curve_controls(curve.get()?)?;
    if weights != loaded_weights || poles.len() != loaded.len() {
        return None;
    }
    let offset = *source.reference_transform.translation_part().coordinates();
    // With positive weights the translated rational curve is a convex combination
    // of control points, so it inherits their maximum placement error.
    poles
        .iter()
        .zip(loaded)
        .try_fold(0_f64, |error, (&raw, moved)| {
            let displacement = placement::pole_shift_error(raw, moved, offset)?;
            Some(error.max(displacement))
        })
}
