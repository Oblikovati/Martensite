//! Bind installed cap traces to the identical certified native rail and 2D control bits.
use super::assembly::CylinderDatum;
use super::*;
use martensite_tkbrep::brep::{BRepTool, access::edge_tshape};
use martensite_tkg3d::geom::GeomBezierCurve;
use martensite_tkmath::predicates::roots::metric_bounds::{CylindricalTrimQuery, MetricBezier};

pub(in super::super) struct Trace {
    pub radius: f64,
    pub poles: Vec<[f64; 3]>,
    pub weights: Vec<f64>,
    pub uv: Vec<[f64; 2]>,
    pub proof: Arc<CertifiedCylindricalTrim>,
    pub edge: topods::Edge,
    pub face: Face,
}
impl Trace {
    pub(super) fn read(
        attempt: Attempt,
        input: &Admitted,
        face: &Face,
        shape: &Shape,
        proof: Arc<CertifiedCylindricalTrim>,
        depth: usize,
    ) -> Checked<Self> {
        let datum = CylinderDatum::read(attempt, face)?;
        let edge = attempt.native(VALIDATION, "cap boundary edge", topods::edge(shape))?;
        let curve = attempt.native(
            VALIDATION,
            "actual cap rail and placement",
            BRepTool::curve_with_location(&edge),
        )?;
        let origin =
            assembly::exact_translation(attempt, [0.; 3], curve.location.transformation())?;
        let bezier = curve.curve.downcast::<GeomBezierCurve>();
        require(
            attempt,
            VALIDATION,
            datum.origin == input.support.origin
                && origin == datum.origin
                && !bezier.is_null()
                && !bezier.has_eval_representation()
                && [curve.first, curve.last] == [0., 1.]
                && (2..=10).contains(&bezier.nb_poles()),
            "same placed cylinder and complete literal rail domain",
        )?;
        let (_, uv) = assembly::polynomial_trace(attempt, face, &edge)?;
        let uv = uv
            .into_iter()
            .map(|[u, v]| [u, datum.height_sign * v])
            .collect();
        let (poles, weights) = rail_words(attempt, &bezier)?;
        let trace = Self {
            radius: datum.radius,
            poles,
            weights,
            uv,
            proof,
            edge: edge.clone(),
            face: face.clone(),
        };
        let raw = attempt
            .native(
                VALIDATION,
                "unchanged raw cap rail tolerance",
                edge_tshape(&edge, "cylinder cap rail"),
            )?
            .tolerance();
        let limit = if depth == 0 {
            input.resolution.sew.min(raw)
        } else {
            raw.min(attempt.native(
                VALIDATION,
                "retained far cap tolerance",
                BRepTool::face_tolerance(face),
            )?)
        };
        require(
            attempt,
            VALIDATION,
            trace.proof.matches_source(trace.query())
                && trace.proof.spatial_residual_bound() <= limit,
            "exact cap trace source within the actual sewing or far-cap tolerance",
        )?;
        Ok(trace)
    }

    pub(in super::super) fn query(&self) -> CylindricalTrimQuery<'_> {
        CylindricalTrimQuery {
            curve: MetricBezier {
                poles: &self.poles,
                weights: Some(&self.weights),
            },
            pcurve: &self.uv,
            radius: self.radius,
            spatial_limit: self.proof.requested_spatial_limit(),
            options: self.proof.requested_options(),
        }
    }
}

fn rail_words(attempt: Attempt, curve: &GeomBezierCurve) -> Checked<(Vec<[f64; 3]>, Vec<f64>)> {
    let mut poles = Vec::new();
    let mut weights = Vec::new();
    for index in 1..=curve.nb_poles() {
        poles.push(
            *attempt
                .native(VALIDATION, "retained cap rail pole", curve.pole(index))?
                .coordinates(),
        );
        weights.push(attempt.native(
            VALIDATION,
            "retained cap rail weight",
            curve.weight(index),
        )?);
    }
    Ok((poles, weights))
}
