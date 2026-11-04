//! Every numeric input is copied from the already admitted same-snapshot fields.
use super::*;
use crate::face_validation::closed_extrusion::metrics;
pub(super) struct PairInputs {
    pub cap: GivenVertexNeighborhoodInput,
    pub seam: GivenVertexNeighborhoodInput,
    pub owner: usize,
}
pub(super) fn pair_inputs(
    carrier: &Carrier<'_>,
    cap: &EdgeUse,
    seam: &EdgeUse,
    class: usize,
) -> CheckResult<PairInputs> {
    let vertex = first_vertex(cap)?;
    let end = seam_endpoint(seam, vertex)?;
    let radius = radius(carrier, class, vertex.owner)?;
    let first = input(cap, vertex, class, radius, VertexNeighborhoodEnds::Both)?;
    let ends = endpoint_ends(end.side);
    Ok(PairInputs {
        cap: first,
        seam: input(seam, end, class, radius, ends)?,
        owner: vertex.owner,
    })
}
fn first_vertex(cap: &EdgeUse) -> CheckResult<&VertexUse> {
    cap.vertices
        .iter()
        .find(|v| v.side == 0)
        .ok_or_else(|| invalid(cap.owner, "cap first vertex", "absent"))
}

fn seam_endpoint<'a>(seam: &'a EdgeUse, vertex: &VertexUse) -> CheckResult<&'a VertexUse> {
    let matches: Vec<&VertexUse> = seam
        .vertices
        .iter()
        .filter(|v| v.owner == vertex.owner && v.location == vertex.location)
        .collect();
    if matches.len() != 1 {
        return Err(invalid(
            seam.owner,
            "one declared located seam endpoint",
            format!("{}", matches.len()),
        ));
    }
    Ok(matches[0])
}
fn radius(carrier: &Carrier<'_>, class: usize, owner: usize) -> CheckResult<f64> {
    let mut raw = vec![carrier.strip.tolerance];
    for (index, edge) in carrier.strip.edges.iter().enumerate() {
        let vertices: Vec<&VertexUse> = edge
            .vertices
            .iter()
            .filter(|v| usize::from(carrier.boundary.corners[index][v.side] >= 2) == class)
            .collect();
        if !vertices.is_empty() {
            raw.push(edge.tolerance);
        }
        raw.extend(vertices.iter().map(|v| v.tolerance));
    }
    metrics::minimum(&raw, owner)
}
fn input(
    edge: &EdgeUse,
    vertex: &VertexUse,
    class: usize,
    radius: f64,
    ends: VertexNeighborhoodEnds,
) -> CheckResult<GivenVertexNeighborhoodInput> {
    Ok(GivenVertexNeighborhoodInput {
        vertex: class,
        curve: curve(edge)?,
        interval: edge.range,
        placements: vec![placement(edge.placement)],
        point: vertex.point,
        point_placements: vec![placement(vertex.placement)],
        radius,
        ends,
    })
}
fn curve(edge: &EdgeUse) -> CheckResult<VertexNeighborhoodCurve> {
    match &edge.curve {
        SpaceCurve::Line(axis) => {
            let p = axis.location();
            let d = axis.direction();
            Ok(VertexNeighborhoodCurve::Line {
                origin: [p.x(), p.y(), p.z()],
                direction: [d.x(), d.y(), d.z()],
            })
        }
        SpaceCurve::Spline(curve) => Ok(VertexNeighborhoodCurve::Quadratic {
            poles: curve.poles().to_vec(),
            flat_knots: curve.flat_knots().to_vec(),
        }),
    }
}
fn placement(frame: MetricEmbedding<3>) -> VertexPlacement {
    VertexPlacement {
        matrix: frame.linear,
        scale: frame.scale,
        translation: frame.origin,
    }
}

fn endpoint_ends(side: usize) -> VertexNeighborhoodEnds {
    if side == 0 {
        VertexNeighborhoodEnds::First
    } else {
        VertexNeighborhoodEnds::Last
    }
}
