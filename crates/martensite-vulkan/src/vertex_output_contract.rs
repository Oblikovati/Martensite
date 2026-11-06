//! Explicit diagnostic contracts never infer vertex identity from TF slot or coordinates.
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// Packed clip4/normal3/direct-index record size in bytes.
pub const VERTEX_OUTPUT_STRIDE: usize = 32;
#[path = "point_vertex_output_contract.rs"]
mod point_contract;
pub use point_contract::{PointVertexOutputRecord, TextVertexOutputRecord, VertexOutputFormat};
trait DirectVertexOutput: PartialEq {
    fn direct_index(&self) -> u32;
}
#[derive(Clone, Debug, PartialEq, Eq)]
/// An incomplete observation, including the offending counts or identity.
pub enum VertexOutputFailure {
    /// A requested exact source was never observed.
    MissingSource {
        /// Requested semantic source.
        source: String,
    },
    /// More than one actual binding matched an exact source request.
    AmbiguousSource {
        /// Requested semantic source.
        source: String,
        /// Actual matching bindings.
        matches: usize,
    },
    /// Requested indices do not fit the bounded triangle-record allocation.
    Capacity {
        /// Number of requested index invocations.
        indices: usize,
        /// Caller-provided allocation limit.
        maximum_bytes: usize,
    },
    /// Written or needed primitive count differs from the original command.
    QueryCount {
        /// Primitives actually written to the buffer.
        written: u64,
        /// Primitives the command needed to write.
        needed: u64,
        /// Required complete count.
        expected: usize,
    },
    /// The complete packed record sequence is unavailable.
    ByteCount {
        /// Observed value.
        actual: usize,
        /// Required complete count.
        expected: usize,
    },
    /// The shader exported an index absent from the original EBO.
    UnexpectedVertex {
        /// Actual exported direct vertex index.
        index: u32,
    },
    /// Repeated direct IDs produced different output bits.
    ConflictingVertex {
        /// Actual exported direct vertex index.
        index: u32,
    },
    /// Some original vertex IDs have no captured record.
    VertexDomain {
        /// Observed value.
        actual: usize,
        /// Required complete count.
        expected: usize,
    },
    /// The request crosses an unsupported diagnostic boundary.
    Unsupported {
        /// Boundary requiring refusal.
        operation: &'static str,
        /// Observed value.
        actual: String,
    },
}
impl fmt::Display for VertexOutputFailure {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            output,
            "vertex-output diagnostic {self:?}; expected complete bounded original VS outputs under the declared primitive/record format"
        )
    }
}
impl std::error::Error for VertexOutputFailure {}

#[derive(Clone, Debug)]
/// Bounded opt-in request tied to original indices and a caller-owned source token.
pub struct VertexOutputRequest {
    captures: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    pub(crate) source_token: Option<std::num::NonZeroU64>,
    pub(crate) indices: Vec<u32>,
    pub(crate) unique: BTreeSet<u32>,
    pub(crate) capacity_bytes: usize,
    pub(crate) format: VertexOutputFormat,
}
impl VertexOutputRequest {
    /// Refuse a completed operation with no selected capture; `request.finish()?`.
    pub fn finish(&self) -> Result<(), VertexOutputFailure> {
        let actual = self.captures.load(std::sync::atomic::Ordering::Relaxed);
        if actual > 0 {
            return Ok(());
        }
        Err(VertexOutputFailure::MissingSource {
            source: format!(
                "source_token={:?}, captures={actual}; expected at least one complete selected capture",
                self.source_token
            ),
        })
    }
    pub(crate) fn record_capture(&self) {
        self.captures
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    /// Bind a caller-owned actual source identity; `request.for_source_token(token)`.
    pub fn for_source_token(mut self, token: std::num::NonZeroU64) -> Self {
        self.source_token = Some(token);
        self
    }

    /// Bind capacity and the input-index domain; `Self::new(vec![0,1,2], 96)?`.
    pub fn new(indices: Vec<u32>, maximum_bytes: usize) -> Result<Self, VertexOutputFailure> {
        Self::with_format(
            indices,
            maximum_bytes,
            VertexOutputFormat::TrianglesClipNormalId32,
        )
    }
    fn with_format(
        indices: Vec<u32>,
        maximum_bytes: usize,
        format: VertexOutputFormat,
    ) -> Result<Self, VertexOutputFailure> {
        let capacity = indices.len().checked_mul(format.stride());
        let valid = !indices.is_empty()
            && indices
                .len()
                .is_multiple_of(format.vertices_per_primitive());
        let Some(capacity_bytes) = capacity.filter(|bytes| valid && *bytes <= maximum_bytes) else {
            return Err(VertexOutputFailure::Capacity {
                indices: indices.len(),
                maximum_bytes,
            });
        };
        Ok(Self {
            captures: Default::default(),
            source_token: None,
            unique: indices.iter().copied().collect(),
            indices,
            capacity_bytes,
            format,
        })
    }
    pub(crate) fn require_actual_indices(&self, bytes: &[u8]) -> Result<(), VertexOutputFailure> {
        let expected = self
            .indices
            .iter()
            .copied()
            .flat_map(u32::to_ne_bytes)
            .collect::<Vec<_>>();
        if bytes == expected {
            return Ok(());
        }
        Err(VertexOutputFailure::Unsupported {
            operation: "actual uploaded original EBO",
            actual: format!("{bytes:?}; expected {expected:?}"),
        })
    }
    /// Require both written and needed primitive counts, including overflow; `request.counts(1,1)?`.
    pub fn counts(&self, written: u64, needed: u64) -> Result<(), VertexOutputFailure> {
        let expected = self.indices.len() / self.format.vertices_per_primitive();
        if written != expected as u64 || needed != expected as u64 {
            return Err(VertexOutputFailure::QueryCount {
                written,
                needed,
                expected,
            });
        }
        Ok(())
    }
    /// Decode every record unchanged, then validate direct IDs; `request.decode(&bytes)?`.
    pub fn decode(&self, bytes: &[u8]) -> Result<Vec<VertexOutputRecord>, VertexOutputFailure> {
        self.require_format(VertexOutputFormat::TrianglesClipNormalId32)?;
        if bytes.len() != self.capacity_bytes {
            return Err(VertexOutputFailure::ByteCount {
                actual: bytes.len(),
                expected: self.capacity_bytes,
            });
        }
        let records = bytes
            .chunks_exact(VERTEX_OUTPUT_STRIDE)
            .map(VertexOutputRecord::decode)
            .collect::<Vec<_>>();
        self.require_identity(&records)?;
        Ok(records)
    }
    fn require_identity<T: DirectVertexOutput>(
        &self,
        records: &[T],
    ) -> Result<(), VertexOutputFailure> {
        let mut seen = BTreeMap::new();
        for record in records {
            if !self.unique.contains(&record.direct_index()) {
                return Err(VertexOutputFailure::UnexpectedVertex {
                    index: record.direct_index(),
                });
            }
            if seen
                .insert(record.direct_index(), record)
                .is_some_and(|prior| prior != record)
            {
                return Err(VertexOutputFailure::ConflictingVertex {
                    index: record.direct_index(),
                });
            }
        }
        if seen.len() != self.unique.len() {
            return Err(VertexOutputFailure::VertexDomain {
                actual: seen.len(),
                expected: self.unique.len(),
            });
        }
        require_multiplicity(&self.indices, records)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
/// Raw output bits retain NaN payloads and signed zero without conversion.
pub struct VertexOutputRecord {
    /// Existing gl_Position float bits, including its API depth convention.
    pub clip_bits: [u32; 4],
    /// Existing worldNormal float bits.
    pub normal_bits: [u32; 3],
    /// Unsigned gl_VertexIndex metadata from the original invocation.
    pub vertex_index: u32,
}
impl VertexOutputRecord {
    fn decode(bytes: &[u8]) -> Self {
        let words = bytes
            .chunks_exact(4)
            .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
            .collect::<Vec<_>>();
        Self {
            clip_bits: words[..4].try_into().unwrap(),
            normal_bits: words[4..7].try_into().unwrap(),
            vertex_index: words[7],
        }
    }
}
impl DirectVertexOutput for VertexOutputRecord {
    fn direct_index(&self) -> u32 {
        self.vertex_index
    }
}
fn require_multiplicity<T: DirectVertexOutput>(
    indices: &[u32],
    records: &[T],
) -> Result<(), VertexOutputFailure> {
    let mut expected = BTreeMap::<u32, usize>::new();
    let mut actual = BTreeMap::<u32, usize>::new();
    for index in indices {
        *expected.entry(*index).or_default() += 1;
    }
    for record in records {
        *actual.entry(record.direct_index()).or_default() += 1;
    }
    if actual != expected {
        return Err(VertexOutputFailure::Unsupported {
            operation: "TF index multiplicities",
            actual: format!("{actual:?}; expected {expected:?}"),
        });
    }
    Ok(())
}
#[cfg(test)]
#[path = "vertex_output_contract_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "point_vertex_output_contract_tests.rs"]
mod point_tests;
