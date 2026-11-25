// SPDX-License-Identifier: GPL-2.0-only
use super::{IncludedLineKey, IncludedLineVersion, IncludedSourceRevision, IncludedVertexName};
use crate::{EditFailure, LineId};
use martensite_tkbrep::brep::line_edge_capture::LineEdgeCaptureError;
use std::{error::Error, fmt};

/// Why a native Include Geometry edit did not publish.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum IncludeGeometryError {
    /// The original named edge has no current correspondence.
    Missing(IncludedLineVersion),
    /// More than one current edge corresponds to the same original name.
    Ambiguous(IncludedLineVersion),
    /// The resolved source identifies a different edge or occurrence path.
    WrongSource {
        /// Required original edge key.
        expected: IncludedLineKey,
        /// Observed edge key.
        actual: IncludedLineKey,
    },
    /// The resolved result has a different version than the pinned request.
    WrongRevision {
        /// Required pinned version.
        expected: IncludedLineVersion,
        /// Observed version.
        actual: IncludedLineVersion,
    },
    /// The destination line has no live include link.
    NoLink(LineId),
    /// The request was based on an earlier or otherwise different live link.
    StaleLink {
        /// Expected accepted version.
        expected: IncludedLineVersion,
        /// Currently accepted version.
        accepted: IncludedLineVersion,
    },
    /// Rebinding requires a newer revision within the original source incarnation.
    RebindRevision {
        /// Last accepted source revision.
        previous: IncludedSourceRevision,
        /// Proposed source revision.
        next: IncludedSourceRevision,
    },
    /// The native edge's endpoint names did not distinguish its two vertices.
    DuplicateVertex(IncludedVertexName),
    /// The newer trim has different source vertices.
    ChangedVertices {
        /// Accepted pair of original names.
        previous: [IncludedVertexName; 2],
        /// Observed pair of original names.
        next: [IncludedVertexName; 2],
    },
    /// The actual native line capture was incomplete or unsupported.
    Capture {
        /// Authoritative original version of the refused native input.
        source: IncludedLineVersion,
        /// Typed original read or proof cause.
        cause: Box<LineEdgeCaptureError>,
    },
    /// The complete edited sketch failed its ordinary geometry admission.
    Geometry(EditFailure),
    /// Operation-local cancellation.
    Cancelled,
}

impl fmt::Display for IncludeGeometryError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Capture { source, cause } => write!(output, "native source {source:?}: {cause}"),
            Self::Geometry(cause) => cause.fmt(output),
            other => write!(
                output,
                "native sketch inclusion needs a unique current edge and two named vertices, got {other:?}"
            ),
        }
    }
}

impl Error for IncludeGeometryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Capture { cause, .. } => Some(cause.as_ref()),
            Self::Geometry(cause) => Some(cause),
            _ => None,
        }
    }
}

impl From<EditFailure> for IncludeGeometryError {
    fn from(cause: EditFailure) -> Self {
        match cause {
            EditFailure::Cancelled(_) => Self::Cancelled,
            other => Self::Geometry(other),
        }
    }
}
