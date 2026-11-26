// SPDX-License-Identifier: GPL-2.0-only
//! Exact Trim side and Extend boundary selection, before rounding any position.
use super::{
    ExtendLineRequest, LineEditFailure, LineEndpoint, LineEndpointTouch, LineParameterLocation,
    Portion, TrimMiss, UnchangedLineEdit, poll,
};
use crate::{LineGeometry, LineId, Snapshot};
use martensite_tkmath::predicates::roots::metric_bounds::{
    DirectedLineContact, DirectedLineHit, DirectedLineParameter, directed_line_contact,
};
use std::{cmp::Ordering, collections::BTreeSet};

pub(super) enum Selection {
    Unchanged(UnchangedLineEdit),
    Hit {
        boundary: LineId,
        hit: Box<DirectedLineHit>,
        portion: Portion,
    },
}

type ProjectedEnd = (DirectedLineParameter, LineParameterLocation);

pub(super) fn trim_contact(
    source: LineGeometry,
    boundary: LineGeometry,
    pick: f64,
) -> Result<Selection, LineEditFailure> {
    match classify(source, boundary)? {
        DirectedLineContact::Parallel => Ok(miss(TrimMiss::Parallel)),
        DirectedLineContact::Skew => Ok(miss(TrimMiss::Skew)),
        DirectedLineContact::Collinear(parameters) => {
            collinear_trim(source.line.id, boundary.line.id, parameters)
        }
        DirectedLineContact::Transverse(hit) => {
            transverse_trim(source.line.id, boundary.line.id, hit, pick)
        }
    }
}

fn classify(
    source: LineGeometry,
    boundary: LineGeometry,
) -> Result<DirectedLineContact, LineEditFailure> {
    let coordinates = |line: LineGeometry| line.points.map(|point| *point.coordinates());
    Ok(directed_line_contact(
        coordinates(source),
        coordinates(boundary),
    )?)
}

fn transverse_trim(
    source: LineId,
    boundary: LineId,
    hit: Box<DirectedLineHit>,
    pick: f64,
) -> Result<Selection, LineEditFailure> {
    let [first, second] = hit
        .parameters()
        .each_ref()
        .map(DirectedLineParameter::location);
    if outside(first) || outside(second) {
        return Ok(miss(TrimMiss::AuthoredRanges));
    }
    if first != LineParameterLocation::Interior || second != LineParameterLocation::Interior {
        return Ok(Selection::Unchanged(UnchangedLineEdit::Touch(touch(
            boundary, first, second,
        ))));
    }
    let dropped = match hit.parameters()[0].compare_authored(pick)? {
        Ordering::Less => LineEndpoint::End,
        Ordering::Greater => LineEndpoint::Start,
        Ordering::Equal => return Err(LineEditFailure::PickAtCut { source, boundary }),
    };
    Ok(Selection::Hit {
        boundary,
        hit,
        portion: Portion::Trim(dropped),
    })
}

fn collinear_trim(
    source: LineId,
    boundary: LineId,
    parameters: [DirectedLineParameter; 2],
) -> Result<Selection, LineEditFailure> {
    use LineParameterLocation::{AfterEnd, BeforeStart, End, Start};
    let [low, high] = projected_bounds(parameters);
    let contact = match (low.0.location(), high.0.location()) {
        (_, BeforeStart) | (AfterEnd, _) => return Ok(miss(TrimMiss::AuthoredRanges)),
        (_, Start) => high,
        (End, _) => low,
        _ => return Err(LineEditFailure::Overlap { source, boundary }),
    };
    Ok(Selection::Unchanged(UnchangedLineEdit::Touch(touch(
        boundary,
        contact.0.location(),
        contact.1,
    ))))
}

fn projected_bounds(parameters: [DirectedLineParameter; 2]) -> [ProjectedEnd; 2] {
    let [first, second] = parameters;
    if first < second {
        [
            (first, LineParameterLocation::Start),
            (second, LineParameterLocation::End),
        ]
    } else {
        [
            (second, LineParameterLocation::End),
            (first, LineParameterLocation::Start),
        ]
    }
}

fn touch(
    boundary: LineId,
    source_location: LineParameterLocation,
    boundary_location: LineParameterLocation,
) -> LineEndpointTouch {
    LineEndpointTouch {
        boundary,
        source_location,
        boundary_location,
    }
}

fn miss(reason: TrimMiss) -> Selection {
    Selection::Unchanged(UnchangedLineEdit::Miss(reason))
}

fn outside(location: LineParameterLocation) -> bool {
    matches!(
        location,
        LineParameterLocation::BeforeStart | LineParameterLocation::AfterEnd
    )
}

impl Snapshot {
    pub(super) fn select_extension(
        &self,
        source: LineGeometry,
        request: ExtendLineRequest<'_>,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Selection, LineEditFailure> {
        if request.boundaries.is_empty() {
            return Ok(Selection::Unchanged(UnchangedLineEdit::EmptyBoundaries));
        }
        let boundaries = sorted_boundaries(request, cancelled)?;
        let mut scan = ForwardScan::default();
        for boundary in boundaries {
            poll(cancelled)?;
            let geometry = self.edit_boundary(request.source, boundary)?;
            scan.consider(boundary, request.endpoint, classify(source, geometry)?);
        }
        scan.finish(request.source, request.endpoint)
    }
}

fn sorted_boundaries(
    request: ExtendLineRequest<'_>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<BTreeSet<LineId>, LineEditFailure> {
    let requested = request.boundaries.len();
    if requested > request.max_boundary_checks {
        return Err(LineEditFailure::BoundaryLimit {
            requested,
            maximum: request.max_boundary_checks,
        });
    }
    let mut boundaries = BTreeSet::new();
    for &boundary in request.boundaries {
        poll(cancelled)?;
        if !boundaries.insert(boundary) {
            return Err(LineEditFailure::RepeatedBoundary(boundary));
        }
    }
    Ok(boundaries)
}

#[derive(Default)]
struct ForwardScan {
    closest: Option<Closest>,
    overlap: Option<ForwardOverlap>,
    touch: Option<LineEndpointTouch>,
}

struct Closest {
    boundary: LineId,
    hit: Box<DirectedLineHit>,
    tied: Option<LineId>,
}

struct ForwardOverlap {
    boundary: LineId,
    entry: Option<DirectedLineParameter>,
}

impl ForwardScan {
    fn consider(&mut self, boundary: LineId, endpoint: LineEndpoint, contact: DirectedLineContact) {
        match contact {
            DirectedLineContact::Transverse(hit) => {
                self.consider_transverse(boundary, endpoint, hit)
            }
            DirectedLineContact::Collinear(parameters) => {
                self.consider_collinear(boundary, endpoint, parameters);
            }
            DirectedLineContact::Parallel | DirectedLineContact::Skew => {}
        }
    }

    fn consider_collinear(
        &mut self,
        boundary: LineId,
        endpoint: LineEndpoint,
        parameters: [DirectedLineParameter; 2],
    ) {
        let [low, high] = projected_bounds(parameters);
        let (furthest, nearest) = match endpoint {
            LineEndpoint::Start => (low, high),
            LineEndpoint::End => (high, low),
        };
        if !forward(endpoint, furthest.0.location()) {
            self.record_touch(boundary, endpoint, furthest.0.location(), furthest.1);
            return;
        }
        // An overlap reaching the source endpoint has no isolated forward entry.
        let entry = forward(endpoint, nearest.0.location()).then_some(nearest.0);
        if self.overlap.as_ref().is_none_or(|old| {
            compare_overlap_entry(endpoint, entry.as_ref(), old.entry.as_ref()) == Ordering::Less
        }) {
            self.overlap = Some(ForwardOverlap { boundary, entry });
        }
    }

    fn consider_transverse(
        &mut self,
        boundary: LineId,
        endpoint: LineEndpoint,
        hit: Box<DirectedLineHit>,
    ) {
        let [first, second] = hit
            .parameters()
            .each_ref()
            .map(DirectedLineParameter::location);
        if outside(second) {
            return;
        }
        if forward(endpoint, first) {
            self.record_forward(boundary, endpoint, hit);
        } else {
            self.record_touch(boundary, endpoint, first, second);
        }
    }

    fn record_touch(
        &mut self,
        boundary: LineId,
        endpoint: LineEndpoint,
        source_location: LineParameterLocation,
        boundary_location: LineParameterLocation,
    ) {
        let endpoint_location = match endpoint {
            LineEndpoint::Start => LineParameterLocation::Start,
            LineEndpoint::End => LineParameterLocation::End,
        };
        if source_location == endpoint_location && self.touch.is_none() {
            self.touch = Some(touch(boundary, source_location, boundary_location));
        }
    }

    fn record_forward(
        &mut self,
        boundary: LineId,
        endpoint: LineEndpoint,
        hit: Box<DirectedLineHit>,
    ) {
        let Some(closest) = &mut self.closest else {
            self.closest = Some(Closest {
                boundary,
                hit,
                tied: None,
            });
            return;
        };
        let order = compare_forward(endpoint, &hit.parameters()[0], &closest.hit.parameters()[0]);
        match order {
            Ordering::Less => {
                *closest = Closest {
                    boundary,
                    hit,
                    tied: None,
                }
            }
            Ordering::Equal => {
                closest.tied.get_or_insert(boundary);
            }
            Ordering::Greater => {}
        }
    }

    fn finish(self, source: LineId, endpoint: LineEndpoint) -> Result<Selection, LineEditFailure> {
        if let Some(overlap) = self.overlap
            && self.closest.as_ref().is_none_or(|candidate| {
                compare_overlap_entry(
                    endpoint,
                    overlap.entry.as_ref(),
                    Some(&candidate.hit.parameters()[0]),
                ) != Ordering::Greater
            })
        {
            return Err(LineEditFailure::Overlap {
                source,
                boundary: overlap.boundary,
            });
        }
        let Some(closest) = self.closest else {
            let reason = self
                .touch
                .map_or(UnchangedLineEdit::NoForwardHit, UnchangedLineEdit::Touch);
            return Ok(Selection::Unchanged(reason));
        };
        if let Some(tied) = closest.tied {
            return Err(LineEditFailure::AmbiguousExtension {
                source,
                boundaries: [closest.boundary, tied],
            });
        }
        Ok(Selection::Hit {
            boundary: closest.boundary,
            hit: closest.hit,
            portion: Portion::Extend(endpoint),
        })
    }
}

fn compare_overlap_entry(
    endpoint: LineEndpoint,
    first: Option<&DirectedLineParameter>,
    second: Option<&DirectedLineParameter>,
) -> Ordering {
    match (first, second) {
        (Some(first), Some(second)) => compare_forward(endpoint, first, second),
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
    }
}

fn compare_forward(
    endpoint: LineEndpoint,
    first: &DirectedLineParameter,
    second: &DirectedLineParameter,
) -> Ordering {
    let order = first.cmp(second);
    if endpoint == LineEndpoint::Start {
        order.reverse()
    } else {
        order
    }
}

fn forward(endpoint: LineEndpoint, location: LineParameterLocation) -> bool {
    matches!(
        (endpoint, location),
        (LineEndpoint::Start, LineParameterLocation::BeforeStart)
            | (LineEndpoint::End, LineParameterLocation::AfterEnd)
    )
}
