// SPDX-License-Identifier: GPL-2.0-only
//! Adopt an actual native line edge as a read-only spatial sketch reference.
mod failure;
mod source;
pub use failure::IncludeGeometryError;
pub use source::{
    IncludedLineKey, IncludedLineObservation, IncludedLineResolution, IncludedLineVersion,
    IncludedSourceRevision, IncludedVertexName, IncludedVertexObservation,
};

use crate::{
    Edit, EditFailure, Entity, EntityId, LineId, PointId, PointRole, SketchLine, SketchPoint,
    Snapshot,
};
use martensite_tkbrep::brep::line_edge_capture::NativeLineEdgeCapture;
use std::collections::BTreeMap;

/// Fresh destination identities in the original occurrence direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IncludedLineIds {
    /// Destination sketch line.
    pub line: LineId,
    /// Two destination endpoint points in directed order.
    pub endpoints: [PointId; 2],
}

/// Live lineage of an included line, from its named original edge and vertices.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IncludedLine {
    /// Destination sketch identities currently in directed order.
    pub ids: IncludedLineIds,
    /// Original edge occurrence and accepted pinned source revision.
    pub source: IncludedLineVersion,
    /// Native endpoints in the same order as the destination endpoints.
    pub vertices: [IncludedVertexName; 2],
    /// Detached original retained trim, orientation, carrier, and endpoint evidence.
    pub native: NativeLineEdgeCapture,
}

impl IncludedLine {
    /// Whether this particular source change invalidates the accepted link.
    /// Example: `included.is_affected_by(changed_version)`.
    pub fn is_affected_by(self, changed: IncludedLineVersion) -> bool {
        self.source.key == changed.key
            && (self.source.revision.incarnation != changed.revision.incarnation
                || self.source.revision.revision < changed.revision.revision)
    }
}

/// Source of one endpoint locked by the included line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IncludedPointSource {
    /// Destination line whose link owns the read-only point.
    pub line: LineId,
    /// Original edge occurrence and pinned revision.
    pub edge: IncludedLineVersion,
    /// Original named endpoint vertex.
    pub vertex: IncludedVertexName,
}

/// Atomic optimistic replacement of one accepted source relation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IncludedLineRebind {
    /// Destination sketch line whose identity is retained.
    pub line: LineId,
    /// Exact accepted source version this update was made against.
    pub previous: IncludedLineVersion,
    /// Exact revised named source to admit.
    pub next: IncludedLineVersion,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct IncludedLines {
    lines: BTreeMap<LineId, IncludedLine>,
    endpoints: BTreeMap<PointId, LineId>,
}

impl IncludedLines {
    fn insert(&mut self, link: IncludedLine) {
        self.lines.insert(link.ids.line, link);
        for point in link.ids.endpoints {
            self.endpoints.insert(point, link.ids.line);
        }
    }

    pub(super) fn remove(&mut self, line: LineId) -> Option<IncludedLine> {
        let link = self.lines.remove(&line)?;
        for point in link.ids.endpoints {
            self.endpoints.remove(&point);
        }
        Some(link)
    }

    fn point(&self, point: PointId) -> Option<IncludedPointSource> {
        let line = self.endpoints.get(&point)?;
        let link = self.lines.get(line)?;
        let index = link.ids.endpoints.iter().position(|&end| end == point)?;
        Some(IncludedPointSource {
            line: *line,
            edge: link.source,
            vertex: link.vertices[index],
        })
    }
}

impl Snapshot {
    /// Observe a current native link and its source data. Example: `sketch.included_line(line)`.
    pub fn included_line(&self, line: LineId) -> Option<IncludedLine> {
        self.included.lines.get(&line).copied()
    }

    /// Observe a native endpoint's read-only source. Example: `sketch.included_point_source(point)`.
    pub fn included_point_source(&self, point: PointId) -> Option<IncludedPointSource> {
        self.included.point(point)
    }

    pub(super) fn point_is_read_only(&self, point: PointId) -> bool {
        self.frozen_source(point).is_some() || self.included.endpoints.contains_key(&point)
    }

    /// Include one resolved original native edge with fresh destination IDs.
    /// Example: `sketch.include_native_line(ids, source_version, resolved, || false)?`.
    pub fn include_native_line(
        &self,
        ids: IncludedLineIds,
        version: IncludedLineVersion,
        resolved: IncludedLineResolution<'_>,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<Self, IncludeGeometryError> {
        let (native, vertices) = source::capture(version, resolved, &mut cancelled)?;
        let link = IncludedLine {
            ids,
            source: version,
            vertices,
            native,
        };
        let mut draft = self.apply_targeted_edit_batch(&link.insertion(), &mut cancelled)?;
        draft.included.insert(link);
        poll(&mut cancelled)?;
        Ok(draft)
    }

    /// Rebind to a unique newer revision of the same named original edge and vertices.
    /// Example: `sketch.rebind_included_line(request, resolved, || false)?`.
    pub fn rebind_included_line(
        &self,
        request: IncludedLineRebind,
        resolved: IncludedLineResolution<'_>,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<Self, IncludeGeometryError> {
        poll(&mut cancelled)?;
        let prior = self
            .included_line(request.line)
            .ok_or(IncludeGeometryError::NoLink(request.line))?;
        prior.check_revision(request)?;
        let (native, vertices) = source::capture(request.next, resolved, &mut cancelled)?;
        let link = prior.rebound(request.next, native, vertices)?;
        self.apply_included_rebind(link, &mut cancelled)
    }

    /// Break the whole native link, keeping ids and geometry and returning last accepted lineage.
    /// Example: `let (editable, lineage) = sketch.detach_included_line(line, || false)?;`.
    pub fn detach_included_line(
        &self,
        line: LineId,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<(Self, IncludedLine), IncludeGeometryError> {
        poll(&mut cancelled)?;
        let mut result = self.clone();
        let lineage = result
            .included
            .remove(line)
            .ok_or(IncludeGeometryError::NoLink(line))?;
        poll(&mut cancelled)?;
        Ok((result, lineage))
    }

    fn apply_included_rebind(
        &self,
        link: IncludedLine,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Self, IncludeGeometryError> {
        poll(cancelled)?;
        let mut draft = self.edit_draft();
        for (point, position) in link.ids.endpoints.into_iter().zip(link.native.points) {
            draft.move_owned_point(point, position)?;
        }
        draft.set_included_line(link.ids)?;
        draft.included.insert(link);
        Ok(self.complete_edit(draft, &[], cancelled)?)
    }

    fn set_included_line(&mut self, ids: IncludedLineIds) -> Result<(), EditFailure> {
        let existing = self.entities.get_mut(&ids.line.get());
        let Some(Entity::Line(line)) = existing else {
            return Err(EditFailure::MissingEditTarget(EntityId::Line(ids.line)));
        };
        line.endpoints = ids.endpoints;
        Ok(())
    }
}

impl IncludedLine {
    fn insertion(self) -> [Edit; 3] {
        let [first, last] = std::array::from_fn(|index| {
            Edit::InsertPoint(SketchPoint {
                id: self.ids.endpoints[index],
                role: PointRole::Defining,
                position: self.native.points[index],
            })
        });
        [
            first,
            last,
            Edit::InsertLine(SketchLine {
                id: self.ids.line,
                endpoints: self.ids.endpoints,
            }),
        ]
    }

    fn check_revision(self, request: IncludedLineRebind) -> Result<(), IncludeGeometryError> {
        if self.source != request.previous {
            return Err(IncludeGeometryError::StaleLink {
                expected: request.previous,
                accepted: self.source,
            });
        }
        if self.source.key != request.next.key {
            return Err(IncludeGeometryError::WrongSource {
                expected: self.source.key,
                actual: request.next.key,
            });
        }
        let [previous, next] = [self.source.revision, request.next.revision];
        if previous.incarnation != next.incarnation || previous.revision >= next.revision {
            return Err(IncludeGeometryError::RebindRevision { previous, next });
        }
        Ok(())
    }

    fn rebound(
        mut self,
        source: IncludedLineVersion,
        native: NativeLineEdgeCapture,
        vertices: [IncludedVertexName; 2],
    ) -> Result<Self, IncludeGeometryError> {
        if vertices != self.vertices {
            if vertices != [self.vertices[1], self.vertices[0]] {
                return Err(IncludeGeometryError::ChangedVertices {
                    previous: self.vertices,
                    next: vertices,
                });
            }
            self.ids.endpoints.swap(0, 1);
        }
        Ok(Self {
            source,
            native,
            vertices,
            ..self
        })
    }
}

pub(super) fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), IncludeGeometryError> {
    if cancelled() {
        return Err(IncludeGeometryError::Cancelled);
    }
    Ok(())
}

const _: fn() = || {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<(
        IncludedLine,
        IncludedLineIds,
        IncludedPointSource,
        IncludedLineRebind,
        IncludeGeometryError,
    )>();
};
