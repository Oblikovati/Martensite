// SPDX-License-Identifier: GPL-2.0-only
//! Immutable, checked mirror copies of selected authored points and lines.
mod failure;
mod plane;

use crate::{
    CurveId, CurveRole, Edit, EntityId, LineId, PointId, SketchLine, SketchPoint, Snapshot,
};
pub use failure::MirrorFailure;
use martensite_tkmath::gp::Axis2;
use plane::MirrorPlane;
use std::{
    collections::{BTreeMap, btree_map::Entry},
    num::NonZeroU64,
};

/// One reflected copy and its source in the input snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorDerivation {
    /// A point and its fresh reflected identity.
    Point {
        /// Original point.
        source: PointId,
        /// Reflected point.
        copy: PointId,
    },
    /// A line and its fresh reflected identity.
    Line {
        /// Original directed line.
        source: LineId,
        /// Reflected directed line.
        copy: LineId,
    },
}

impl MirrorDerivation {
    fn ids(self) -> [EntityId; 2] {
        match self {
            Self::Point { source, copy } => [EntityId::Point(source), EntityId::Point(copy)],
            Self::Line { source, copy } => [EntityId::Line(source), EntityId::Line(copy)],
        }
    }
}

/// A complete reflected copy operation.
#[derive(Clone, Debug, PartialEq)]
pub struct MirrorResult {
    /// The admitted source geometry and its new copies.
    pub snapshot: Snapshot,
    /// The derived point and line bindings in ascending source-key order.
    pub history: Vec<MirrorDerivation>,
    /// The supplied and certified upper bound on Euclidean position error in model units.
    pub position_error_bound: f64,
}

type Selection = BTreeMap<NonZeroU64, MirrorDerivation>;

impl Snapshot {
    /// Reflect points and directed lines across a native plane in one atomic edit.
    ///
    /// Each pair maps an owned source to a fresh identity from the caller. Both endpoints
    /// of every selected line require point mappings. Point roles and line participation
    /// are kept. A reflected frozen point is authored at the reflected location, and its
    /// original source stamp stays with the original point. Coordinates round once
    /// from the exact reflection. The inclusive error limit applies along each
    /// authored segment because reflection is affine.
    /// Example: `snapshot.mirror_copies(plane, &[(from, to)], &[], 1e-8, || false)?`.
    pub fn mirror_copies(
        &self,
        plane: Axis2,
        points: &[(PointId, PointId)],
        lines: &[(LineId, LineId)],
        position_error_limit: f64,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<MirrorResult, MirrorFailure> {
        poll(&mut cancelled)?;
        let plane = MirrorPlane::new(plane, position_error_limit)?;
        let selected = self.mirror_selection(points, lines, &mut cancelled)?;
        let edits = self.stage_mirror(&selected, &plane, &mut cancelled)?;
        let snapshot = self.apply(&edits, &mut cancelled)?;
        let history = selected.into_values().collect();
        poll(&mut cancelled)?;
        Ok(MirrorResult {
            snapshot,
            history,
            position_error_bound: position_error_limit,
        })
    }

    fn mirror_selection(
        &self,
        points: &[(PointId, PointId)],
        lines: &[(LineId, LineId)],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Selection, MirrorFailure> {
        if points.is_empty() && lines.is_empty() {
            return Err(MirrorFailure::EmptySelection);
        }
        let points = points
            .iter()
            .map(|&(source, copy)| MirrorDerivation::Point { source, copy });
        let lines = lines
            .iter()
            .map(|&(source, copy)| MirrorDerivation::Line { source, copy });
        let mut selection = Selection::new();
        let mut targets = BTreeMap::new();
        for pair in points.chain(lines) {
            poll(cancelled)?;
            self.select_mirror_pair(pair, &mut selection, &mut targets)?;
        }
        Ok(selection)
    }

    fn select_mirror_pair(
        &self,
        pair: MirrorDerivation,
        selection: &mut Selection,
        targets: &mut BTreeMap<NonZeroU64, EntityId>,
    ) -> Result<(), MirrorFailure> {
        let [source, copy] = pair.ids();
        if self.entities.get(&source.raw()).map(|entity| entity.id()) != Some(source) {
            return Err(MirrorFailure::MissingSource(source));
        }
        let Entry::Vacant(selected) = selection.entry(source.raw()) else {
            return Err(MirrorFailure::RepeatedSource(source));
        };
        let existing = self
            .entities
            .get(&copy.raw())
            .map(|entity| entity.id())
            .or_else(|| targets.get(&copy.raw()).copied());
        if let Some(existing) = existing {
            return Err(MirrorFailure::DuplicateTarget {
                existing,
                attempted: copy,
            });
        }
        targets.insert(copy.raw(), copy);
        selected.insert(pair);
        Ok(())
    }

    fn stage_mirror(
        &self,
        selected: &Selection,
        plane: &MirrorPlane,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Vec<Edit>, MirrorFailure> {
        let mut edits = Vec::new();
        // Resolve each directed line's endpoint mappings before doing point arithmetic.
        for &pair in selected.values() {
            if let MirrorDerivation::Line { source, copy } = pair {
                poll(cancelled)?;
                self.stage_mirrored_line(source, copy, selected, &mut edits)?;
            }
        }
        for &pair in selected.values() {
            if let MirrorDerivation::Point { source, copy } = pair {
                poll(cancelled)?;
                edits.push(self.mirrored_point(source, copy, plane)?);
            }
        }
        Ok(edits)
    }

    fn mirrored_point(
        &self,
        source: PointId,
        copy: PointId,
        plane: &MirrorPlane,
    ) -> Result<Edit, MirrorFailure> {
        let point = self
            .point(source)
            .ok_or(MirrorFailure::MissingSource(EntityId::Point(source)))?;
        let position = plane.reflect(source, point.position)?;
        Ok(Edit::InsertPoint(SketchPoint {
            id: copy,
            role: point.role,
            position,
        }))
    }

    fn stage_mirrored_line(
        &self,
        source: LineId,
        copy: LineId,
        selected: &Selection,
        edits: &mut Vec<Edit>,
    ) -> Result<(), MirrorFailure> {
        let line = self
            .line(source)
            .ok_or(MirrorFailure::MissingSource(EntityId::Line(source)))?;
        let [a, b] = line.line.endpoints;
        let endpoints = [
            mirrored_endpoint(source, a, selected)?,
            mirrored_endpoint(source, b, selected)?,
        ];
        edits.push(Edit::InsertLine(SketchLine {
            id: copy,
            endpoints,
        }));
        if self.curve_role(CurveId::Line(source)) == Some(CurveRole::Construction) {
            edits.push(Edit::SetCurveRole {
                curve: CurveId::Line(copy),
                role: CurveRole::Construction,
            });
        }
        Ok(())
    }
}

fn mirrored_endpoint(
    line: LineId,
    point: PointId,
    selected: &Selection,
) -> Result<PointId, MirrorFailure> {
    match selected.get(&point.get()) {
        Some(MirrorDerivation::Point { copy, .. }) => Ok(*copy),
        _ => Err(MirrorFailure::UnmappedEndpoint { line, point }),
    }
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), MirrorFailure> {
    if cancelled() {
        return Err(MirrorFailure::Cancelled);
    }
    Ok(())
}
