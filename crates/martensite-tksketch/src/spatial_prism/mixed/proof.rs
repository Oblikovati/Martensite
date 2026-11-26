// SPDX-License-Identifier: GPL-2.0-only
//! Certify the complete retained normal sweep and integrate its checked cap chart.
mod cap;
mod side;

use super::{
    Failure, Stage,
    images::{Images, PointImages},
    native::{self, Subject},
    poll,
    source::{Source, SourcePoint},
};
use crate::spatial_prism::volume::interval::Span;
use martensite_tkbo::profile_extrusion::{IntegrationMethod, LoopUse, Measured};
use martensite_tksketch3d::PointId;
use std::collections::BTreeMap;

pub(super) fn certify(
    source: &Source,
    images: &Images<'_>,
    outer: &[LoopUse],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(Measured, Measured), Failure> {
    let laterals = bind_points(source, images, cancelled)?;
    let twice_area = cap::verify(source, images, outer, cancelled)?;
    side::verify(source, images, &laterals, cancelled)?;
    // Each source section is the same simple closed trim in a disjoint
    // parallel plane. The bounded quarter-end gap moves as a ruled closing
    // strip, so its entire volume contribution is already in the cap enclosure.
    let area = twice_area * Span::at(0.5);
    let volume = area * Span::at(source.chart.height.abs());
    Ok((measure(area)?, measure(volume)?))
}

fn bind_points(
    source: &Source,
    images: &Images<'_>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<BTreeMap<PointId, [f64; 2]>, Failure> {
    source
        .points
        .iter()
        .map(|(&id, &point)| {
            poll(cancelled, Stage::Sweep)?;
            let range = bind_point(point, &images.points[&id])?;
            Ok((id, range))
        })
        .collect()
}

fn bind_point(point: SourcePoint, images: &PointImages<'_>) -> Result<[f64; 2], Failure> {
    let subject = Subject(None);
    native::vertex(images.caps[0], point.xyz[0], subject)?;
    native::vertex(images.caps[1], point.xyz[1], subject)?;
    native::edge_vertices(images.lateral, images.caps, subject)?;
    native::line_xyz(images.lateral, point.xyz, subject)
}

fn measure(enclosure: Span) -> Result<Measured, Failure> {
    let subject = Subject(None);
    subject.require(
        enclosure.finite() && enclosure.lo > 0.,
        "a finite positive analytic area or volume enclosure",
    )?;
    let value = enclosure.lo + (enclosure.hi - enclosure.lo) * 0.5;
    let error_bound = (value - enclosure.lo)
        .abs()
        .max((enclosure.hi - value).abs())
        .next_up();
    subject.require(
        value.is_finite() && error_bound.is_finite(),
        "a representable area or volume receipt",
    )?;
    Ok(Measured {
        value,
        error_bound,
        method: IntegrationMethod::CertifiedEnclosure,
    })
}
