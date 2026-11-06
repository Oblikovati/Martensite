// SPDX-License-Identifier: GPL-2.0-only
//! Shared normalized weights make the coefficient residual a convex combination.
use super::{capture_curves::unsupported, exact, types::*};
use martensite_tkmath::predicates::roots::{DyadicBound, metric_bounds::*};

pub(super) fn check(
    packet: &OwnedFacePacket,
    roles: [BoundaryRole; 4],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<StripPrerequisites, BindingFailure> {
    let mut conformance = Vec::new();
    for (index, role) in roles.iter().enumerate() {
        let edge = &packet.edges[index];
        let limit = existing_limit(packet.face_tolerance, edge.tolerance)?;
        let residuals = match role {
            BoundaryRole::UFirst | BoundaryRole::ULast => seam_inputs(packet, edge, *role)?,
            BoundaryRole::VFirst | BoundaryRole::VLast => cap_inputs(packet, edge, *role)?,
        };
        conformance.push(certify_edge(index, residuals, limit, cancelled)?);
    }
    Ok(StripPrerequisites { roles, conformance })
}
fn existing_limit(face: f64, edge: f64) -> Result<f64, BindingFailure> {
    if !face.is_finite() || !edge.is_finite() || face < 0. || edge < 0. {
        return Err(unsupported(
            "recorded spatial tolerances",
            format!("face={face}, edge={edge}; expected finite nonnegative"),
        ));
    }
    let limit = face.min(edge);
    Ok(if limit == 0. { 0. } else { limit })
}
fn seam_inputs(
    packet: &OwnedFacePacket,
    edge: &OwnedEdgeUse,
    role: BoundaryRole,
) -> Result<Vec<ResidualInput>, BindingFailure> {
    let OwnedSpaceCurve::Line(line) = &edge.curve else {
        return Err(unsupported(
            "seam 3D curve",
            "B-spline; expected built-in line",
        ));
    };
    let pole = seam_pole(packet, role);
    Ok(vec![ResidualInput::new(
        packet,
        edge,
        line.clone(),
        pole,
        OwnedLine {
            origin: [edge.pcurve.origin[1], 0.],
            direction: [edge.pcurve.direction[1], 0.],
        },
        edge.range,
    )])
}

fn seam_pole(packet: &OwnedFacePacket, role: BoundaryRole) -> [f64; 3] {
    if role == BoundaryRole::UFirst {
        return packet.basis.poles[0];
    }
    *packet.basis.poles.last().expect("validated nonempty basis")
}

fn cap_inputs(
    packet: &OwnedFacePacket,
    edge: &OwnedEdgeUse,
    role: BoundaryRole,
) -> Result<Vec<ResidualInput>, BindingFailure> {
    let OwnedSpaceCurve::Spline(curve) = &edge.curve else {
        return Err(unsupported(
            "cap 3D curve",
            "line; expected matching built-in B-spline",
        ));
    };
    matching_basis(&packet.basis, curve)?;
    let v = require_cap_map(packet, edge, role)?;
    Ok(curve
        .poles
        .iter()
        .zip(&packet.basis.poles)
        .map(|(actual, support)| cap_coefficient(packet, edge, *actual, *support, v))
        .collect())
}

fn cap_coefficient(
    packet: &OwnedFacePacket,
    edge: &OwnedEdgeUse,
    actual: [f64; 3],
    support: [f64; 3],
    v: f64,
) -> ResidualInput {
    ResidualInput::new(
        packet,
        edge,
        OwnedLine {
            origin: actual,
            direction: [0.; 3],
        },
        support,
        OwnedLine {
            origin: [v, 0.],
            direction: [0.; 2],
        },
        [0., 1.],
    )
}

fn require_cap_map(
    packet: &OwnedFacePacket,
    edge: &OwnedEdgeUse,
    role: BoundaryRole,
) -> Result<f64, BindingFailure> {
    let v = if role == BoundaryRole::VFirst {
        packet.uv_bounds[2]
    } else {
        packet.uv_bounds[3]
    };
    if edge.pcurve.origin != [0., v]
        || edge.pcurve.direction != [1., 0.]
        || edge.range != packet.basis.domain
    {
        return Err(unsupported(
            "cap same-parameter map",
            format!(
                "origin={:?}, direction={:?}, range={:?}; expected (t,v) on full basis domain",
                edge.pcurve.origin, edge.pcurve.direction, edge.range
            ),
        ));
    }
    Ok(v)
}

fn matching_basis(source: &OwnedSpline, actual: &OwnedSpline) -> Result<(), BindingFailure> {
    let layout = source.degree == actual.degree
        && source.knots == actual.knots
        && source.multiplicities == actual.multiplicities
        && source.domain == actual.domain
        && source.periodic == actual.periodic
        && source.poles.len() == actual.poles.len();
    // Positive weights alone are insufficient: both normalized basis functions must be identical.
    if !layout
        || source.weights != actual.weights
        || actual.poles.iter().flatten().any(|v| !v.is_finite())
    {
        return Err(unsupported(
            "cap coefficient correspondence",
            format!(
                "layout_equal={layout}, weights_equal={}; expected same complete knots/degree/weights/domain",
                source.weights == actual.weights
            ),
        ));
    }
    Ok(())
}
fn certify_edge(
    index: usize,
    residuals: Vec<ResidualInput>,
    limit: f64,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<EdgeConformance, BindingFailure> {
    let squared_limit = exact::squared_limit(limit, cancelled)?;
    let (upper, queries) = coefficient_bound(index, &residuals, limit, cancelled)?;
    if upper > squared_limit {
        return Err(unsupported(
            "certified squared error",
            format!(
                "upper={upper}, permitted={squared_limit}; expected upper <= existing limit squared"
            ),
        ));
    }
    Ok(EdgeConformance {
        occurrence: index,
        existing_spatial_limit: limit,
        proven_squared_upper: upper,
        coefficient_count: residuals.len(),
        bound_queries: queries,
        existing_squared_limit: squared_limit,
    })
}
fn coefficient_bound(
    index: usize,
    residuals: &[ResidualInput],
    limit: f64,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(DyadicBound, usize), BindingFailure> {
    let mut upper = DyadicBound::from_f64(0.).expect("finite zero");
    let mut queries = 0;
    for input in residuals {
        let (bound, work) = super::refine::certified_upper(input, index, limit, cancelled)?;
        upper = upper.max(bound);
        queries += work;
    }
    Ok((upper, queries))
}

#[derive(Clone)]
pub(super) struct ResidualInput {
    pub curve: OwnedLine<3>,
    pub curve_placement: AffinePlacement,
    pub support_origin: [f64; 3],
    pub direction: [f64; 3],
    pub support_placement: AffinePlacement,
    pub v_map: OwnedLine<2>,
    pub interval: [f64; 2],
}
impl ResidualInput {
    fn new(
        packet: &OwnedFacePacket,
        edge: &OwnedEdgeUse,
        curve: OwnedLine<3>,
        support_origin: [f64; 3],
        v_map: OwnedLine<2>,
        interval: [f64; 2],
    ) -> Self {
        Self {
            curve,
            curve_placement: edge.placement.clone(),
            support_origin,
            direction: packet.direction,
            support_placement: packet.support_placement.clone(),
            v_map,
            interval,
        }
    }

    pub(super) fn query(&self, limit: f64) -> PlanarResidualQuery<'_> {
        PlanarResidualQuery {
            curve: MetricCurve::Line {
                origin: self.curve.origin,
                direction: self.curve.direction,
            },
            curve_embedding: self.curve_placement.embedding(),
            pcurve: MetricCurve::Line {
                origin: self.v_map.origin,
                direction: self.v_map.direction,
            },
            plane_embedding: MetricEmbedding {
                linear: std::array::from_fn(|i| [self.direction[i], 0.]),
                origin: self.support_origin,
                scale: 1.,
            },
            plane_placement: self.support_placement.embedding(),
            interval: self.interval,
            spatial_limit: limit,
            options: martensite_tkmath::predicates::roots::RootIsolationOptions::new(1.),
        }
    }
}
