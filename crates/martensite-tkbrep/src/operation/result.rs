// SPDX-License-Identifier: GPL-2.0-only
//! Sealed, immutable, `Send + Sync` operation result owner (#27) and
//! deterministic content-addressed geometry revision (#28).
use super::{HistoryGraph, OperationError, OperationKind};
use crate::brep_tools;
use crate::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::storage::IoStream;
use std::fmt;
use std::io::{Read, Seek, SeekFrom};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::{self, ThreadId};

static NEXT_OPERATION_REVISION: AtomicU64 = AtomicU64::new(1);

/// Deterministic, content-addressed 128-bit digest of a sealed shape, operation kind,
/// and option bytes (#28).
///
/// Independent of process-global counters, pointer addresses, and thread IDs: identical
/// inputs and options always produce the exact same [`GeometryRevision`].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeometryRevision {
    low: u64,
    high: u64,
}

impl GeometryRevision {
    /// Computes a deterministic 128-bit revision from `operation`, serialized `shape`,
    /// and canonical `options_bytes` (#28).
    pub fn compute(operation: OperationKind, shape: &Shape, options_bytes: &[u8]) -> Self {
        let mut stream = IoStream::from_bytes(Vec::new());
        let _ = brep_tools::write(shape, &mut stream, &ProgressRange::default());
        let mut bytes = Vec::new();
        if let Some(mut device) = stream.into_device()
            && device.seek(SeekFrom::Start(0)).is_ok()
        {
            let _ = device.read_to_end(&mut bytes);
        }
        Self::from_canonical_bytes(operation, &bytes, options_bytes)
    }

    /// Computes a deterministic 128-bit revision from pre-serialized canonical bytes.
    pub fn from_canonical_bytes(
        operation: OperationKind,
        shape_bytes: &[u8],
        options_bytes: &[u8],
    ) -> Self {
        let op_tag = format!("{operation:?}");
        let mut lane_a: u64 = 0xcbf2_9ce4_8422_2325;
        let mut lane_b: u64 = 0xaf63_dc4c_8601_ec8c;
        fnv_mix(&mut lane_a, &mut lane_b, op_tag.as_bytes());
        fnv_mix(&mut lane_a, &mut lane_b, &[0xff]);
        fnv_mix(&mut lane_a, &mut lane_b, options_bytes);
        fnv_mix(&mut lane_a, &mut lane_b, &[0xfe]);
        fnv_mix(&mut lane_a, &mut lane_b, shape_bytes);
        Self {
            low: lane_a,
            high: lane_b,
        }
    }

    /// Returns the 128-bit digest as a big-endian 16-byte array.
    pub fn as_bytes(&self) -> [u8; 16] {
        let mut out = [0u8; 16];
        out[..8].copy_from_slice(&self.high.to_be_bytes());
        out[8..].copy_from_slice(&self.low.to_be_bytes());
        out
    }
}

fn fnv_mix(lane_a: &mut u64, lane_b: &mut u64, bytes: &[u8]) {
    for &byte in bytes {
        *lane_a ^= u64::from(byte);
        *lane_a = lane_a.wrapping_mul(0x0000_0100_0000_01b3);
        *lane_b ^= u64::from(byte.rotate_left(3));
        *lane_b = lane_b.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    }
}

impl fmt::Debug for GeometryRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "GeometryRevision({:016x}{:016x})", self.high, self.low)
    }
}

impl fmt::Display for GeometryRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:016x}{:016x}", self.high, self.low)
    }
}

/// Single sealed, immutable result owner returned by public modelling operations (#27).
///
/// No mutable internal handle escapes: callers read the detached [`Shape`], its
/// unified [`HistoryGraph`], its deterministic [`GeometryRevision`], and its originating thread.
///
/// ```
/// use martensite_tkbrep::operation::{HistoryGraph, OperationKind, OperationResult};
/// use martensite_tkbrep::brep::BRepBuilder;
/// use martensite_tkbrep::topods::Vertex;
/// let mut vertex = Vertex::default();
/// BRepBuilder::default().make_vertex(&mut vertex);
/// let result = OperationResult::seal(OperationKind::Copy, vertex.into_shape(), HistoryGraph::empty()).unwrap();
/// assert!(result.revision() >= 1);
/// ```
#[derive(Clone, Debug)]
pub struct OperationResult {
    operation: OperationKind,
    shape: Shape,
    history: HistoryGraph,
    revision: u64,
    content_revision: GeometryRevision,
    owner_thread: ThreadId,
}

impl OperationResult {
    /// Seals a detached shape and its history into a new revision (#27).
    pub fn seal(
        operation: OperationKind,
        shape: Shape,
        history: HistoryGraph,
    ) -> Result<Self, OperationError> {
        Self::seal_with_options(operation, shape, history, &[])
    }

    /// Seals a detached shape, history, and canonical option bytes into a deterministic
    /// content-addressed [`GeometryRevision`] (#28).
    pub fn seal_with_options(
        operation: OperationKind,
        shape: Shape,
        history: HistoryGraph,
        options_bytes: &[u8],
    ) -> Result<Self, OperationError> {
        if shape.is_null() {
            return Err(OperationError::kernel_failure(
                operation,
                "cannot seal a null result shape",
            ));
        }
        let content_revision = GeometryRevision::compute(operation, &shape, options_bytes);
        let revision = NEXT_OPERATION_REVISION.fetch_add(1, Ordering::Relaxed);
        Ok(Self {
            operation,
            shape,
            history,
            revision,
            content_revision,
            owner_thread: thread::current().id(),
        })
    }

    /// Operation that produced this result.
    pub fn operation(&self) -> OperationKind {
        self.operation
    }

    /// Immutable reference to the sealed result shape.
    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    /// Unified many-to-many history graph for this operation.
    pub fn history(&self) -> &HistoryGraph {
        &self.history
    }

    /// Monotonically increasing seal sequence number.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Deterministic content-addressed revision over topology, geometry, and options (#28).
    pub fn content_revision(&self) -> GeometryRevision {
        self.content_revision
    }

    /// Thread on which this result was sealed.
    pub fn owner_thread(&self) -> ThreadId {
        self.owner_thread
    }
}

#[cfg(test)]
mod geometry_revision_tests {
    use super::*;
    use crate::brep::BRepBuilder;
    use crate::topods::Vertex;
    use martensite_tkmath::gp::Point3;

    fn make_vertex_shape(x: f64, y: f64, z: f64, tol: f64) -> Shape {
        let mut vertex = Vertex::default();
        BRepBuilder::default()
            .make_vertex_at(&mut vertex, Point3::new(x, y, z), tol)
            .expect("make vertex");
        vertex.into_shape()
    }

    #[test]
    fn identical_inputs_produce_identical_content_revision_across_threads() {
        let first = OperationResult::seal_with_options(
            OperationKind::Boolean,
            make_vertex_shape(1.0, 2.0, 3.0, 1e-6),
            HistoryGraph::empty(),
            b"tol=1e-6",
        )
        .expect("seal first");

        let second = std::thread::spawn(|| {
            OperationResult::seal_with_options(
                OperationKind::Boolean,
                make_vertex_shape(1.0, 2.0, 3.0, 1e-6),
                HistoryGraph::empty(),
                b"tol=1e-6",
            )
            .expect("seal second")
        })
        .join()
        .expect("join thread");

        assert_eq!(first.content_revision(), second.content_revision());
        assert_eq!(
            first.content_revision().as_bytes(),
            second.content_revision().as_bytes()
        );
    }

    #[test]
    fn geometry_or_option_change_produces_distinct_content_revision() {
        let base = OperationResult::seal_with_options(
            OperationKind::Boolean,
            make_vertex_shape(1.0, 2.0, 3.0, 1e-6),
            HistoryGraph::empty(),
            b"tol=1e-6",
        )
        .expect("seal base");
        let moved = OperationResult::seal_with_options(
            OperationKind::Boolean,
            make_vertex_shape(1.0, 2.0, 3.5, 1e-6),
            HistoryGraph::empty(),
            b"tol=1e-6",
        )
        .expect("seal moved");
        let re_optioned = OperationResult::seal_with_options(
            OperationKind::Boolean,
            make_vertex_shape(1.0, 2.0, 3.0, 1e-6),
            HistoryGraph::empty(),
            b"tol=1e-5",
        )
        .expect("seal re_optioned");

        assert_ne!(base.content_revision(), moved.content_revision());
        assert_ne!(base.content_revision(), re_optioned.content_revision());
    }
}
