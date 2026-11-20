// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn cap(
    query: &ConicalActualCapQuery<'_>,
    sine: &Interval,
    cosine: &Interval,
    period: &Interval,
    bound_slant: bool,
    atlas: bool,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<CapCapture> {
    let points = cap_points(query, arithmetic)?;
    let u = numeric::hull(&points.iter().map(|p| p[0].clone()).collect::<Vec<_>>());
    let v = numeric::hull(&points.iter().map(|p| p[1].clone()).collect::<Vec<_>>());
    let r0 = arithmetic.scalar(query.carrier.radius)?;
    let z0 = arithmetic.scalar(query.carrier.axial_origin)?;
    let radius = filled_radius(&r0, &v, sine, arithmetic)?;
    numeric::positive("entire cap rectangle radius", &radius, arithmetic)?;
    pairs::width(&u, period, arithmetic)?;
    let radial = arithmetic.multiply(&r0, cosine)?;
    let axial = arithmetic.multiply(&z0, sine)?;
    let level = arithmetic.subtract(&radial, &axial)?;
    let extent = if atlas {
        CapExtent::Atlas(Box::new(super::atlas::cap_box(
            &u, &v, &radius, &z0, cosine, arithmetic,
        )?))
    } else if bound_slant {
        CapExtent::Slant(slant_coordinate(&r0, &z0, &v, sine, cosine, arithmetic)?)
    } else {
        CapExtent::Legacy
    };
    Ok(CapCapture {
        source: ConicalActualCapSnapshot::capture(query),
        u,
        level,
        extent,
    })
}
fn slant_coordinate(
    r0: &Interval,
    z0: &Interval,
    v: &Interval,
    sine: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    // In common carrier coordinates, r*sin(alpha)+z*cos(alpha) = S0+v.
    let radial = arithmetic.multiply(r0, sine)?;
    let axial = arithmetic.multiply(z0, cosine)?;
    let origin = arithmetic.add(&radial, &axial)?;
    arithmetic.add(&origin, v)
}
fn edge_points(
    edge: &ConicalActualUvEdge<'_>,
    points: &mut Vec<[Interval; 2]>,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<()> {
    if edge.poles.len() != edge.weights.len() || !(2..=4).contains(&edge.poles.len()) {
        return Err(error::shape(
            "actual cap curve controls",
            edge.poles.len(),
            "2..=4 controls and equal positive weight count",
        ));
    }
    for (point, weight) in edge.poles.iter().zip(edge.weights) {
        let u = arithmetic.scalar(point[0])?;
        let v = arithmetic.scalar(point[1])?;
        let weight = arithmetic.scalar(*weight)?;
        numeric::positive("actual cap weight", &weight, arithmetic)?;
        points.push([u, v]);
    }
    Ok(())
}

#[cfg(test)]
mod tests;

fn filled_radius(
    r0: &Interval,
    v: &Interval,
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let radial = arithmetic.multiply(v, sine)?;
    arithmetic.add(r0, &radial)
}

fn cap_points(
    query: &ConicalActualCapQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<Vec<[Interval; 2]>> {
    let mut points = Vec::new();
    for contour in query.contours {
        arithmetic.check_cancelled()?;
        for edge in contour.edges {
            edge_points(edge, &mut points, arithmetic)?;
        }
    }
    if points.is_empty() {
        return Err(error::shape(
            "actual cap UV controls",
            0,
            "nonempty selected boundary",
        ));
    }
    Ok(points)
}
