//! Exact rectangle endpoints bind the full finite support domain.
use super::{budget::*, decode::*, placements::identity};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::predicates::roots::metric_bounds::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Side {
    UFirst,
    ULast,
    VFirst,
    VLast,
}
pub(super) struct Boundary {
    pub bounds: [f64; 4],
    pub sides: [Side; 4],
    pub corners: [[usize; 2]; 4],
}
pub(super) fn certify<F: FnMut() -> bool>(
    strip: &Strip,
    budget: &mut Budget<'_, F>,
) -> CheckResult<Boundary> {
    let [u0, u1] = strip.basis.domain();
    let [v0, v1] = vertical_interval(strip)?;
    let bounds = [u0, u1, v0, v1];
    let targets = [[u0, v0], [u1, v0], [u1, v1], [u0, v1]];
    let (sides, corners) = classify_edges(strip, &targets, budget)?;
    cycle(strip, &corners)?;
    seam(strip, &sides)?;
    Ok(Boundary {
        bounds,
        sides,
        corners,
    })
}
fn vertical_interval(strip: &Strip) -> CheckResult<[f64; 2]> {
    let mut values = Vec::new();
    for edge in &strip.edges {
        let d = edge.pcurve.direction();
        if d.y() == 0. && d.x() != 0. {
            values.push(edge.pcurve.location().y());
        }
    }
    if values.len() != 2 || values.iter().any(|v| !v.is_finite()) || values[0] == values[1] {
        return Err(unsupported(
            0,
            "cap levels",
            format!("{values:?}"),
            "two distinct finite constant-V maps",
        ));
    }
    Ok([values[0].min(values[1]), values[0].max(values[1])])
}
fn corner<F: FnMut() -> bool>(
    edge: &EdgeUse,
    parameter: f64,
    targets: &[[f64; 2]; 4],
    budget: &mut Budget<'_, F>,
) -> CheckResult<usize> {
    let mut matches = Vec::new();
    for (index, target) in targets.iter().enumerate() {
        if endpoint_matches(edge, parameter, *target, budget)? {
            matches.push(index);
        }
    }
    if matches.len() != 1 {
        return Err(invalid(
            edge.owner,
            "one exact trim corner",
            format!("matched={matches:?}"),
        ));
    }
    Ok(matches[0])
}
fn endpoint_matches<F: FnMut() -> bool>(
    edge: &EdgeUse,
    parameter: f64,
    target: [f64; 2],
    budget: &mut Budget<'_, F>,
) -> CheckResult<bool> {
    budget.query()?;
    let first = endpoint_chain(edge, parameter);
    let second = corner_chain(target);
    compare_curve_chains(first, second, &mut *budget.cancel)
        .map(|d| d == CurveIdentityDecision::Identical)
        .map_err(|e| {
            super::types::ClosedFaceCompletion::Incomplete(
                super::types::ClosedFaceIncomplete::Metric(e),
            )
        })
}
fn side(pair: [usize; 2], owner: usize) -> CheckResult<Side> {
    match (pair[0].min(pair[1]), pair[0].max(pair[1])) {
        (0, 3) => Ok(Side::UFirst),
        (1, 2) => Ok(Side::ULast),
        (0, 1) => Ok(Side::VFirst),
        (2, 3) => Ok(Side::VLast),
        _ => Err(invalid(owner, "rectangle side", format!("{pair:?}"))),
    }
}
fn cycle(strip: &Strip, corners: &[[usize; 2]; 4]) -> CheckResult<()> {
    let mut incoming = [0; 4];
    let mut outgoing = [0; 4];
    for (edge, pair) in strip.edges.iter().zip(corners) {
        let pair = directed_pair(edge, *pair)?;
        outgoing[pair[0]] += 1;
        incoming[pair[1]] += 1;
    }
    if incoming != [1; 4] || outgoing != [1; 4] {
        return Err(invalid(
            0,
            "single directed rectangle",
            format!("incoming={incoming:?}, outgoing={outgoing:?}"),
        ));
    }
    Ok(())
}
fn seam(strip: &Strip, sides: &[Side; 4]) -> CheckResult<()> {
    require_sides(sides)?;
    let a = sides
        .iter()
        .position(|s| *s == Side::UFirst)
        .ok_or_else(|| invalid(0, "first seam", "missing"))?;
    let b = sides
        .iter()
        .position(|s| *s == Side::ULast)
        .ok_or_else(|| invalid(0, "last seam", "missing"))?;
    let (first, last) = (&strip.edges[a], &strip.edges[b]);
    require_seam_pair(first, last)?;
    require_unique_edges(strip, [a, b])
}

fn validate_cache(edge: &EdgeUse, expected: [[f64; 2]; 2]) -> CheckResult<()> {
    if edge.cached_endpoints != expected {
        return Err(invalid(
            edge.owner,
            "stored UV endpoint cache",
            format!("actual={:?}, expected={expected:?}", edge.cached_endpoints),
        ));
    }
    Ok(())
}

fn classify_edges<F: FnMut() -> bool>(
    strip: &Strip,
    targets: &[[f64; 2]; 4],
    budget: &mut Budget<'_, F>,
) -> CheckResult<([Side; 4], [[usize; 2]; 4])> {
    let mut sides = [Side::UFirst; 4];
    let mut corners = [[0; 2]; 4];
    for (index, edge) in strip.edges.iter().enumerate() {
        let pair = [
            corner(edge, edge.range[0], targets, budget)?,
            corner(edge, edge.range[1], targets, budget)?,
        ];
        validate_cache(edge, pair.map(|corner| targets[corner]))?;
        corners[index] = pair;
        sides[index] = side(pair, edge.owner)?;
    }
    Ok((sides, corners))
}

fn endpoint_chain(edge: &EdgeUse, parameter: f64) -> MetricCurveChain<'static, 2> {
    let p = edge.pcurve.location();
    let d = edge.pcurve.direction();
    MetricCurveChain {
        curve: MetricCurve::Line {
            origin: [parameter, 0.],
            direction: [0.; 2],
        },
        embedding: MetricEmbedding {
            linear: [[d.x(), 0.], [d.y(), 0.], [0., 0.]],
            origin: [p.x(), p.y(), 0.],
            scale: 1.,
        },
        placements: &[],
    }
}

fn corner_chain(target: [f64; 2]) -> MetricCurveChain<'static, 3> {
    MetricCurveChain {
        curve: MetricCurve::Line {
            origin: [target[0], target[1], 0.],
            direction: [0.; 3],
        },
        embedding: identity(),
        placements: &[],
    }
}

fn directed_pair(edge: &EdgeUse, mut pair: [usize; 2]) -> CheckResult<[usize; 2]> {
    match edge.orientation {
        Orientation::Forward => {}
        Orientation::Reversed => pair.swap(0, 1),
        other => {
            return Err(invalid(
                edge.owner,
                "edge orientation",
                format!("{other:?}"),
            ));
        }
    }
    Ok(pair)
}

fn require_sides(sides: &[Side; 4]) -> CheckResult<()> {
    for side in [Side::UFirst, Side::ULast, Side::VFirst, Side::VLast] {
        if sides.iter().filter(|s| **s == side).count() != 1 {
            return Err(invalid(0, "one occurrence per side", format!("{sides:?}")));
        }
    }
    Ok(())
}

fn require_seam_pair(first: &EdgeUse, last: &EdgeUse) -> CheckResult<()> {
    if first.owner != last.owner
        || first.location != last.location
        || !first.seam
        || !last.seam
        || first.branch == last.branch
        || first.orientation == last.orientation
    {
        return Err(invalid(
            first.owner,
            "genuine paired seam",
            "identity, branch or orientation mismatch",
        ));
    }
    Ok(())
}

fn require_unique_edges(strip: &Strip, seam: [usize; 2]) -> CheckResult<()> {
    let [a, b] = seam;
    for left in 0..4 {
        for right in left + 1..4 {
            if (left, right) != (a.min(b), a.max(b))
                && strip.edges[left].owner == strip.edges[right].owner
                && strip.edges[left].location == strip.edges[right].location
            {
                return Err(invalid(
                    0,
                    "only seam aliases",
                    "extra identical edge occurrences",
                ));
            }
        }
    }
    Ok(())
}
