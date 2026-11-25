// SPDX-License-Identifier: GPL-2.0-only
//! Coherent source resolution and pinned native authority.
use super::{IncludeGeometryError as Error, poll};
use martensite_tkbrep::{
    brep::line_edge_capture::{LineEdgeCaptureError, NativeLineEdgeCapture, capture_line_edge},
    topods::{Edge, Vertex},
};
use std::num::NonZeroU64;

/// Stable caller-owned path and edge occurrence used to resolve one original native source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IncludedLineKey {
    /// Original source document.
    pub document: [u8; 16],
    /// Named path of the assembly or feature instance containing the edge.
    pub path: NonZeroU64,
    /// Stable edge occurrence identity in that path.
    pub occurrence: NonZeroU64,
}

/// Authoritative version of one source scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IncludedSourceRevision {
    /// Caller-owned identity of the originating source incarnation.
    pub incarnation: u64,
    /// Revision within that incarnation.
    pub revision: u64,
}

/// One exact pinned observation of an original named edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IncludedLineVersion {
    /// Original document, path, and edge occurrence.
    pub key: IncludedLineKey,
    /// Original source revision.
    pub revision: IncludedSourceRevision,
}

/// Stable endpoint identity in the original edge's source scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IncludedVertexName(pub NonZeroU64);

/// An original vertex name and its resolved located BRep occurrence.
#[derive(Clone, Copy, Debug)]
pub struct IncludedVertexObservation<'a> {
    /// Source-owner vertex identity.
    pub name: IncludedVertexName,
    /// Actual located native vertex.
    pub vertex: &'a Vertex,
}

/// Unique actual original native source with both independently named endpoint occurrences.
/// The caller pins this observation's revision during capture.
#[derive(Clone, Copy, Debug)]
pub struct IncludedLineObservation<'a> {
    /// Resolved source path, occurrence, and revision.
    pub version: IncludedLineVersion,
    /// Actual located, oriented edge.
    pub edge: &'a Edge,
    /// Actual located endpoints, in either order.
    pub vertices: [IncludedVertexObservation<'a>; 2],
}

/// Result supplied by the model owner's same-name resolver.
#[derive(Clone, Copy, Debug)]
pub enum IncludedLineResolution<'a> {
    /// The exact original named edge was not found.
    Missing,
    /// More than one correspondence remains after resolving the original name.
    Ambiguous,
    /// Exactly one original located edge with its vertex names was resolved.
    Unique(IncludedLineObservation<'a>),
}

pub(super) fn capture(
    expected: IncludedLineVersion,
    resolved: IncludedLineResolution<'_>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(NativeLineEdgeCapture, [IncludedVertexName; 2]), Error> {
    poll(cancelled)?;
    let observation = match resolved {
        IncludedLineResolution::Unique(observation) => observation,
        IncludedLineResolution::Missing => return Err(Error::Missing(expected)),
        IncludedLineResolution::Ambiguous => return Err(Error::Ambiguous(expected)),
    };
    observation.check(expected)?;
    let native = capture_line_edge(
        observation.edge,
        observation.vertices.map(|v| v.vertex),
        cancelled,
    )
    .map_err(|cause| match cause {
        LineEdgeCaptureError::Cancelled => Error::Cancelled,
        other => Error::Capture {
            source: observation.version,
            cause: Box::new(other),
        },
    })?;
    let names = native
        .vertices
        .map(|index| observation.vertices[index].name);
    Ok((native.geometry, names))
}

impl IncludedLineObservation<'_> {
    fn check(self, expected: IncludedLineVersion) -> Result<(), Error> {
        if expected.key != self.version.key {
            return Err(Error::WrongSource {
                expected: expected.key,
                actual: self.version.key,
            });
        }
        if expected != self.version {
            return Err(Error::WrongRevision {
                expected,
                actual: self.version,
            });
        }
        if self.vertices[0].name == self.vertices[1].name {
            return Err(Error::DuplicateVertex(self.vertices[0].name));
        }
        Ok(())
    }
}

const _: fn() = || {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<(
        IncludedLineKey,
        IncludedSourceRevision,
        IncludedLineVersion,
        IncludedVertexName,
        IncludedVertexObservation<'static>,
        IncludedLineObservation<'static>,
        IncludedLineResolution<'static>,
    )>();
};
