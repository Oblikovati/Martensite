// SPDX-License-Identifier: GPL-2.0-only
//! Positively oriented convex trim and exact half-plane classification.
use super::{
    Checked, PLANAR_SECTION_MAX_EDGES, PlanarSectionAnchor, PlanarSectionEmpty,
    PlanarSectionError as Failure, PlanarSectionTrim, PlanarSectionWitness, SectionBorder,
    SectionChartSource,
    exact::{
        Chart, Hit, Scalar, Uv, Vector, bounds, interpolate, orientation, pcurve_endpoints,
        space_endpoints, sub, vertex, zero,
    },
    poll,
};
use crate::predicates::roots::metric_bounds::placed_plane::{cross, dot};
use std::cmp::Ordering;

pub(super) fn classify(
    sources: [SectionChartSource; 2],
    reverse_face: bool,
    border: &[SectionBorder],
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<PlanarSectionTrim> {
    poll(cancelled)?;
    if !(3..=PLANAR_SECTION_MAX_EDGES).contains(&border.len()) {
        return Err(Failure::BorderCount(border.len()));
    }
    let charts = [
        Chart::capture(sources[0], 0)?,
        Chart::capture(sources[1], 1)?,
    ];
    let points = validate_trim(&charts[0], border, cancelled)?;
    let direction = cross(&charts[0].normal, &charts[1].normal);
    if direction.iter().all(zero) {
        return Ok(if zero(&charts[0].separation(&charts[1])) {
            PlanarSectionTrim::CoincidentSupports
        } else {
            PlanarSectionTrim::Empty(PlanarSectionEmpty::ParallelDistinct)
        });
    }
    transverse(charts, direction, reverse_face, border, points, cancelled)
}

fn validate_trim(
    chart: &Chart,
    border: &[SectionBorder],
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Vec<Vector>> {
    let mut lines = Vec::with_capacity(border.len());
    for (index, use_edge) in border.iter().enumerate() {
        poll(cancelled)?;
        lines.push(validate_edge(chart, use_edge, index)?);
    }
    let mut polygon = Vec::with_capacity(lines.len());
    let mut points = Vec::with_capacity(lines.len());
    for (index, points_on_edge) in lines.iter().enumerate() {
        if points_on_edge[1] != lines[(index + 1) % lines.len()][0] {
            return Err(Failure::InconsistentBorder(index));
        }
        polygon.push(chart.pullback(&points_on_edge[0]));
        points.push(points_on_edge[0].clone());
    }
    require_convex(&polygon, cancelled)?;
    Ok(points)
}

pub(super) fn validate_edge(
    chart: &Chart,
    use_edge: &SectionBorder,
    index: usize,
) -> Checked<[Vector; 2]> {
    let inconsistent = Failure::InconsistentBorder(index);
    let mut actual = space_endpoints(use_edge.space).map_err(|cause| {
        if cause == Failure::CandidateDomain {
            inconsistent
        } else {
            cause
        }
    })?;
    if actual[0] == actual[1] || actual.iter().any(|point| !zero(&chart.dist(point))) {
        return Err(inconsistent);
    }
    if let Some(pcurve) = use_edge.pcurve
        && (pcurve.range != use_edge.space.range
            || pcurve_endpoints(pcurve)?
                .each_ref()
                .map(|uv| chart.lift(uv))
                != actual)
    {
        return Err(inconsistent);
    }
    if use_edge.reversed {
        actual.swap(0, 1);
    }
    let [a, b] = use_edge.vertices;
    if [vertex(a)?, vertex(b)?] != actual {
        return Err(inconsistent);
    }
    Ok(actual)
}

fn require_convex(points: &[Uv], cancelled: &mut impl FnMut() -> bool) -> Checked<()> {
    for (i, start) in points.iter().enumerate() {
        let next = (i + 1) % points.len();
        for (_, point) in points
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i && *j != next)
        {
            poll(cancelled)?;
            if orientation(start, &points[next], point) != Ordering::Greater {
                return Err(Failure::NonConvexTrim);
            }
        }
    }
    Ok(())
}

fn transverse(
    charts: [Chart; 2],
    direction: Vector,
    reverse_face: bool,
    border: &[SectionBorder],
    points: Vec<Vector>,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<PlanarSectionTrim> {
    let distances: Vec<_> = points.iter().map(|point| charts[1].dist(point)).collect();
    let mut hits = Vec::with_capacity(2);
    for (index, current) in distances.iter().enumerate() {
        poll(cancelled)?;
        let next = (index + 1) % distances.len();
        let following = &distances[next];
        if zero(current) && zero(following) {
            return Ok(PlanarSectionTrim::BoundaryOverlap(index));
        }
        if let Some(hit) = cut(
            index,
            [current, following],
            [&points[index], &points[next]],
            border[index],
        )? {
            hits.push(hit);
        }
    }
    finish(charts, direction, reverse_face, hits)
}

fn cut(
    index: usize,
    [d0, d1]: [&Scalar; 2],
    [start, end]: [&Vector; 2],
    border: SectionBorder,
) -> Checked<Option<Hit>> {
    if zero(d0) {
        return Ok(Some(Hit {
            point: start.clone(),
            anchor: PlanarSectionAnchor::Vertex(index),
        }));
    }
    if zero(d1) || d0.sign() == d1.sign() {
        return Ok(None);
    }
    let fraction = d0.divide(&d0.subtract(d1));
    let point = interpolate(start, end, &fraction);
    let mut range = border.space.range.map(Scalar::from_finite);
    if border.reversed {
        range.swap(0, 1);
    }
    let [parameter] = interpolate(&[range[0].clone()], &[range[1].clone()], &fraction);
    let anchor = PlanarSectionAnchor::Edge {
        index,
        parameter: bounds(&parameter)?,
    };
    Ok(Some(Hit { point, anchor }))
}

fn finish(
    charts: [Chart; 2],
    direction: Vector,
    reverse_face: bool,
    hits: Vec<Hit>,
) -> Checked<PlanarSectionTrim> {
    let mut hits: [Hit; 2] = match hits.try_into() {
        Ok(pair) => pair,
        Err(hits) if hits.is_empty() => {
            return Ok(PlanarSectionTrim::Empty(PlanarSectionEmpty::TrimMiss));
        }
        Err(hits) if hits.len() == 1 => {
            let PlanarSectionAnchor::Vertex(index) = hits[0].anchor else {
                return Err(Failure::NonConvexTrim);
            };
            return Ok(PlanarSectionTrim::PointOnly(index));
        }
        Err(_) => return Err(Failure::NonConvexTrim),
    };
    let sign = dot(&direction, &sub(&hits[1].point, &hits[0].point)).sign();
    if sign == Ordering::Equal {
        return Err(Failure::NonConvexTrim);
    }
    if (sign == Ordering::Less) != reverse_face {
        hits.swap(0, 1);
    }
    Ok(PlanarSectionTrim::Segment(Box::new(PlanarSectionWitness {
        charts,
        hits,
    })))
}
