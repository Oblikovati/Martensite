// SPDX-License-Identifier: GPL-2.0-only
use super::{DirectedLineUse, WireClosure, WireFailure, WirePhase, poll};
use crate::{CurveId, LineGeometry, PointId, Snapshot};
use martensite_tkmath::predicates::{SegmentContact3d, exact_segment_contact_3d};
use std::collections::BTreeSet;

/// Owned occurrence and its authored segment, independent of the sketch snapshot.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WireSegment {
    /// Line identity and direction selected in the wire.
    pub source: DirectedLineUse,
    /// Authored endpoints, model coordinates and nominal line carrier.
    pub authored: LineGeometry,
}

impl WireSegment {
    /// Return the start and end point IDs in wire direction. Example: `segment.endpoints()`.
    pub fn endpoints(&self) -> [PointId; 2] {
        self.source
            .direction
            .endpoints(self.authored.line.endpoints)
    }
}

/// Evidence that every authored segment pair satisfies the ordered wire's incidence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WireRecipeValidation {
    /// Exact-predicate queries completed, including conservative disjoint-box exclusions.
    pub pair_checks: usize,
    /// Maximum pair checks requested by the caller.
    pub pair_limit: usize,
}

/// An immutable wire with certified authored incidence, ready for detached native export.
#[derive(Clone, Debug, PartialEq)]
pub struct WireRecipe {
    closure: WireClosure,
    segments: Vec<WireSegment>,
    validation: WireRecipeValidation,
}

impl Snapshot {
    /// Certify an explicitly ordered spatial wire. An empty or incomplete request returns a
    /// typed failure. Example: `snapshot.wire_recipe(&uses, WireClosure::Open, 64, || false)`.
    pub fn wire_recipe(
        &self,
        uses: &[DirectedLineUse],
        closure: WireClosure,
        max_pair_checks: usize,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<WireRecipe, WireFailure> {
        poll(&mut cancelled, WirePhase::Resolve)?;
        let segments = resolve(self, uses, &mut cancelled)?;
        check_connectivity(&segments, closure, &mut cancelled)?;
        let pair_checks = check_incidence(&segments, closure, max_pair_checks, &mut cancelled)?;
        Ok(WireRecipe {
            closure,
            segments,
            validation: WireRecipeValidation {
                pair_checks,
                pair_limit: max_pair_checks,
            },
        })
    }
}

impl WireRecipe {
    /// Return the required and verified closure. Example: `recipe.closure()`.
    pub fn closure(&self) -> WireClosure {
        self.closure
    }

    /// Return the occurrences in caller order. Example: `recipe.segments().len()`.
    pub fn segments(&self) -> &[WireSegment] {
        &self.segments
    }

    /// Return completed exact incidence evidence. Example: `recipe.validation().pair_checks`.
    pub fn validation(&self) -> WireRecipeValidation {
        self.validation
    }
}

fn resolve(
    snapshot: &Snapshot,
    uses: &[DirectedLineUse],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<WireSegment>, WireFailure> {
    let mut seen = BTreeSet::new();
    let mut segments = Vec::with_capacity(uses.len());
    for &source in uses {
        poll(cancelled, WirePhase::Resolve)?;
        if !seen.insert(source.line) {
            return Err(WireFailure::RepeatedLine(source.line));
        }
        let authored = snapshot
            .line(source.line)
            .ok_or(WireFailure::MissingLine(source.line))?;
        if snapshot.construction.contains(&CurveId::Line(source.line)) {
            return Err(WireFailure::ConstructionLine(source.line));
        }
        segments.push(WireSegment { source, authored });
    }
    Ok(segments)
}

fn check_connectivity(
    segments: &[WireSegment],
    closure: WireClosure,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), WireFailure> {
    let Some((first, last)) = segments.first().zip(segments.last()) else {
        return Err(WireFailure::Empty);
    };
    for pair in segments.windows(2) {
        poll(cancelled, WirePhase::Resolve)?;
        let points = [pair[0].endpoints()[1], pair[1].endpoints()[0]];
        if points[0] != points[1] {
            return Err(WireFailure::DisjointUses {
                lines: [pair[0].source.line, pair[1].source.line],
                points,
            });
        }
    }
    let points = [first.endpoints()[0], last.endpoints()[1]];
    if (points[0] == points[1]) != (closure == WireClosure::Closed) {
        return Err(WireFailure::WrongClosure {
            expected: closure,
            points,
        });
    }
    Ok(())
}

fn check_incidence(
    segments: &[WireSegment],
    closure: WireClosure,
    limit: usize,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<usize, WireFailure> {
    let mut checks = 0;
    for (left, first) in segments.iter().enumerate() {
        for (right, second) in segments.iter().enumerate().skip(left + 1) {
            poll(cancelled, WirePhase::Incidence)?;
            let lines = [first.source.line, second.source.line];
            if checks == limit {
                return Err(WireFailure::PairLimit { lines, limit });
            }
            let neighbors = right == left + 1
                || (left == 0 && right + 1 == segments.len() && closure == WireClosure::Closed);
            check_segment_pair([first, second], lines, neighbors)?;
            checks += 1;
        }
    }
    poll(cancelled, WirePhase::Incidence)?;
    Ok(checks)
}

fn check_segment_pair(
    segments: [&WireSegment; 2],
    lines: [crate::LineId; 2],
    adjacent: bool,
) -> Result<(), WireFailure> {
    let [first, second] =
        segments.map(|segment| segment.authored.points.map(|point| *point.coordinates()));
    let contact = exact_segment_contact_3d(first, second)
        .map_err(|source| WireFailure::IncidenceInput { lines, source })?;
    let expected = if adjacent {
        SegmentContact3d::Point
    } else {
        SegmentContact3d::Disjoint
    };
    if contact != expected {
        return Err(WireFailure::SegmentIncidence {
            lines,
            adjacent,
            contact,
        });
    }
    Ok(())
}
