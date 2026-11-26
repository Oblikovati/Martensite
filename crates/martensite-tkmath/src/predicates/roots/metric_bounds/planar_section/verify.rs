// SPDX-License-Identifier: GPL-2.0-only
//! Qualify the actual Section edge and any stored pcurves over their whole interval.
use super::{
    Checked, PlanarSectionError as Failure, PlanarSectionLineEvidence, PlanarSectionWitness,
    SectionLineSource, SectionUvEndpoints, SectionUvLine, SectionVertexSource,
    exact::{
        Chart, Vector, bounds, distance_bound, pcurve_endpoints, rounded, space_endpoints, sub,
        vertex,
    },
    poll,
};
use crate::predicates::roots::metric_bounds::placed_plane::{dot, exact_vector};
use std::cmp::Ordering;

pub(super) fn qualify(
    witnessed: &PlanarSectionWitness,
    line: SectionLineSource,
    vertices: [SectionVertexSource; 2],
    pcurves: [Option<SectionUvLine>; 2],
    limit: f64,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<PlanarSectionLineEvidence> {
    poll(cancelled)?;
    if !limit.is_finite() || limit < 0. {
        return Err(Failure::ErrorLimit(limit));
    }
    let reference = witnessed.hits.each_ref().map(|hit| hit.point.clone());
    let space = space_endpoints(line)?;
    let [a, b] = vertices;
    let vertex_points = [vertex(a)?, vertex(b)?];
    let vertex_error_bound = check_bound(distance_bound(&space, &vertex_points)?, limit)?;
    let pcurve_bounds = verify_pcurves(pcurves, &witnessed.charts, line, &space, limit, cancelled)?;
    let mut directed = space;
    let mut parameters = line.range;
    direct_pair(&reference, &mut directed, &mut parameters)?;
    let native_error_bound = check_bound(distance_bound(&reference, &directed)?, limit)?;
    let [a, b] = directed.each_ref().map(rounded);
    let endpoints = [a?, b?];
    if endpoints[0] == endpoints[1] {
        return Err(Failure::CandidateDomain);
    }
    let published = [
        exact_vector(endpoints[0], 0)?,
        exact_vector(endpoints[1], 1)?,
    ];
    let position_error_bound = check_bound(distance_bound(&reference, &published)?, limit)?;
    let [first, second] = &witnessed.charts;
    let parent_uv = [
        parent_uv(first, &reference)?,
        parent_uv(second, &reference)?,
    ];
    poll(cancelled)?;
    Ok(PlanarSectionLineEvidence {
        endpoints,
        native_parameters: parameters,
        parent_uv,
        anchors: witnessed.hits.each_ref().map(|hit| hit.anchor),
        native_error_bound,
        vertex_error_bound,
        position_error_bound,
        stored_pcurve_error_bounds: pcurve_bounds,
    })
}

fn direct_pair(
    reference: &[Vector; 2],
    candidate: &mut [Vector; 2],
    parameters: &mut [f64; 2],
) -> Checked<()> {
    let orient = dot(
        &sub(&reference[1], &reference[0]),
        &sub(&candidate[1], &candidate[0]),
    )
    .sign();
    match orient {
        Ordering::Equal => Err(Failure::CandidateDomain),
        Ordering::Greater => Ok(()),
        Ordering::Less => {
            candidate.swap(0, 1);
            parameters.swap(0, 1);
            Ok(())
        }
    }
}

fn verify_pcurves(
    pcurves: [Option<SectionUvLine>; 2],
    charts: &[Chart; 2],
    line: SectionLineSource,
    native: &[Vector; 2],
    limit: f64,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<[Option<f64>; 2]> {
    let mut attained = [None; 2];
    for (i, optional) in pcurves.into_iter().enumerate() {
        poll(cancelled)?;
        let Some(pcurve) = optional else {
            continue;
        };
        if pcurve.range != line.range {
            return Err(Failure::CandidateDomain);
        }
        let lifted = pcurve_endpoints(pcurve)?
            .each_ref()
            .map(|uv| charts[i].lift(uv));
        attained[i] = Some(check_bound(distance_bound(native, &lifted)?, limit)?);
    }
    Ok(attained)
}

fn parent_uv(chart: &Chart, reference: &[Vector; 2]) -> Checked<SectionUvEndpoints> {
    let endpoint = |point| -> Checked<_> {
        let [u, v] = chart.pullback(point);
        Ok([bounds(&u)?, bounds(&v)?])
    };
    Ok([endpoint(&reference[0])?, endpoint(&reference[1])?])
}

fn check_bound(upper: f64, limit: f64) -> Checked<f64> {
    if upper <= limit {
        return Ok(upper);
    }
    Err(Failure::Position { upper, limit })
}
