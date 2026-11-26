// SPDX-License-Identifier: GPL-2.0-only
use super::{
    LoftFailure as Failure, LoftOptions, LoftSection, LoftSectionId, LoftStage as Stage, poll,
};
use crate::spatial_prism::{
    SpatialPrismFailure,
    plane::{AuthoredPlane, certify_authored_plane},
};
use martensite_tkbo::body_check::ResolutionRejected;
use martensite_tkmath::predicates::{compare_tetrahedron_orientation_3d, positive_homothety_3d};
use martensite_tksketch3d::wire::{DetachedWire, WireClosure, WireFailure};
use std::cmp::Ordering;

pub(super) struct AdmittedPair<'a> {
    pub sections: [LoftSection<'a>; 2],
    pub planes: [AuthoredPlane; 2],
    pub station_sign: Ordering,
}

pub(super) fn admit<'a>(
    sections: [LoftSection<'a>; 2],
    options: LoftOptions,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<AdmittedPair<'a>, Failure> {
    poll(cancelled, Stage::Admission)?;
    options
        .resolution
        .admit()
        .map_err(|(field, value)| Failure::Resolution(ResolutionRejected { field, value }))?;
    if !options.source_trace_limit.is_finite() || options.source_trace_limit <= 0. {
        return Err(Failure::InvalidTraceLimit(options.source_trace_limit));
    }
    if sections[0].id == sections[1].id {
        return Err(Failure::RepeatedSection(sections[0].id));
    }
    let planes = [
        plane(sections[0], cancelled)?,
        plane(sections[1], cancelled)?,
    ];
    let station_sign = correspondence(&planes, sections.map(|section| section.id), cancelled)?;
    Ok(AdmittedPair {
        sections,
        planes,
        station_sign,
    })
}

fn plane(
    section: LoftSection<'_>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<AuthoredPlane, Failure> {
    if section.wire.closure() != WireClosure::Closed {
        return Err(Failure::OpenSection(section.id));
    }
    certify_authored_plane(section.wire, cancelled).map_err(|cause| match cause {
        SpatialPrismFailure::Cancelled(_) => Failure::Cancelled(Stage::Admission),
        source => Failure::SectionPlane {
            section: section.id,
            source,
        },
    })
}

fn correspondence(
    [first, last]: &[AuthoredPlane; 2],
    sections: [LoftSectionId; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Ordering, Failure> {
    let points =
        |plane: &AuthoredPlane| plane.points.iter().map(|point| point.1).collect::<Vec<_>>();
    poll(cancelled, Stage::Admission)?;
    positive_homothety_3d(&points(first), &points(last))
        .map_err(|cause| Failure::Incompatible { sections, cause })?;
    let [a, b, c] = first.anchors.map(|point| point.1);
    let station_sign = compare_tetrahedron_orientation_3d([a, b, c, last.anchors[0].1])
        .map_err(Failure::Predicate)?;
    if station_sign == Ordering::Equal {
        return Err(Failure::CoincidentPlanes);
    }
    poll(cancelled, Stage::Admission)?;
    Ok(station_sign)
}

pub(super) fn materialize(
    sections: [LoftSection<'_>; 2],
    limit: f64,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<[DetachedWire; 2], Failure> {
    Ok([
        detached(sections[0], limit, cancelled)?,
        detached(sections[1], limit, cancelled)?,
    ])
}

fn detached(
    section: LoftSection<'_>,
    limit: f64,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<DetachedWire, Failure> {
    poll(cancelled, Stage::Materialize)?;
    let detached = section
        .wire
        .materialize(&mut *cancelled)
        .map_err(|source| match source {
            WireFailure::Cancelled(_) => Failure::Cancelled(Stage::Materialize),
            source => Failure::Wire {
                section: section.id,
                source,
            },
        })?;
    poll(cancelled, Stage::Materialize)?;
    let bound = detached.validation.curve_error_bound;
    if !bound.is_finite() || bound > limit {
        return Err(Failure::InputTrace {
            section: section.id,
            bound,
            limit,
        });
    }
    Ok(detached)
}
