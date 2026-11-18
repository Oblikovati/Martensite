// SPDX-License-Identifier: GPL-2.0-only
//! Exact bounded query binding without allocating during comparison.
use super::angle::*;
use super::boundary::*;
use super::*;

#[derive(Debug)]
pub(super) struct BoundarySnapshot {
    pub(super) bits: Vec<u64>,
}
impl BoundarySnapshot {
    pub fn capture(query: &ConicalBoundaryQuery<'_>) -> Self {
        let mut bits = Vec::new();
        visit(query, |value| bits.push(value));
        Self { bits }
    }
    pub fn matches(&self, query: &ConicalBoundaryQuery<'_>) -> bool {
        let mut expected = self.bits.iter().copied();
        let mut same = true;
        visit(query, |value| {
            same &= expected.next() == Some(value);
        });
        same && expected.next().is_none()
    }
}
pub(super) fn visit(query: &ConicalBoundaryQuery<'_>, mut put: impl FnMut(u64)) {
    put(query.source.len() as u64);
    for point in query.source {
        for value in point {
            put(value.to_bits());
        }
    }
    visit_original(query.original, &mut put);
    values(query.angular_fit, &mut put);
    values(query.slant_numerator, &mut put);
    values(query.slant_weights, &mut put);
    visit_support(query, &mut put);
    for edge in &query.edges {
        visit_edge(edge, &mut put);
    }
}
pub(super) fn visit_original(original: Option<ConicalOriginalSpan<'_>>, put: &mut impl FnMut(u64)) {
    let Some(original) = original else {
        put(0);
        return;
    };
    put(1);
    put(original.poles.len() as u64);
    for point in original.poles {
        for value in point {
            put(value.to_bits());
        }
    }
    put(original.index);
    put(u64::from(original.level));
}
pub(super) fn visit_support(query: &ConicalBoundaryQuery<'_>, put: &mut impl FnMut(u64)) {
    for value in [
        query.semi_angle,
        query.chart_origin,
        query.fit_center,
        query.depths[0],
        query.depths[1],
        query.spatial_limit,
    ] {
        put(value.to_bits());
    }
    put(query.developed_turn as i64 as u64);
    put(query.physical_turn as i64 as u64);
    put(u64::from(query.outward));
}
pub(super) fn visit_edge(edge: &ConicalStorageQuery<'_>, put: &mut impl FnMut(u64)) {
    values(edge.tangent, put);
    values(edge.numerator, put);
    values(edge.weights, put);
    for value in [edge.semi_angle, edge.center, edge.depth] {
        put(value.to_bits());
    }
    put(u64::from(edge.outward));
    put(edge.stored_poles.len() as u64);
    for point in edge.stored_poles {
        for value in point {
            put(value.to_bits());
        }
    }
    values(edge.stored_weights, put);
}
pub(super) fn values(values: &[f64], put: &mut impl FnMut(u64)) {
    put(values.len() as u64);
    for value in values {
        put(value.to_bits());
    }
}
