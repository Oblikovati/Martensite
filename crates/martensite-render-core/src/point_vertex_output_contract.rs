//! The point format exports no normal because the plain shader has none.
use super::*;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Explicit primitive and record format; `VertexOutputFormat::PointsClipId20`.
pub enum VertexOutputFormat {
    #[default]
    /// Existing triangle capture with clip4, normal3 and direct ID.
    TrianglesClipNormalId32,
    /// Ordinary unexpanded points with clip4 and direct ID only.
    PointsClipId20,
    /// Text triangles with final clip4, UV2 and direct vertex ID; `VertexOutputFormat::TextClipUvId28`.
    TextClipUvId28,
}
impl VertexOutputFormat {
    /// Exact packed output bytes; `VertexOutputFormat::PointsClipId20.stride()`.
    pub const fn stride(self) -> usize {
        match self {
            Self::TrianglesClipNormalId32 => 32,
            Self::PointsClipId20 => 20,
            Self::TextClipUvId28 => 28,
        }
    }
    pub(crate) const fn vertices_per_primitive(self) -> usize {
        match self {
            Self::TrianglesClipNormalId32 => 3,
            Self::PointsClipId20 => 1,
            Self::TextClipUvId28 => 3,
        }
    }
}
impl VertexOutputRequest {
    /// Bind the actual indexed point domain; `Self::points(vec![2,7],40)?`.
    pub fn points(indices: Vec<u32>, maximum_bytes: usize) -> Result<Self, VertexOutputFailure> {
        Self::with_format(indices, maximum_bytes, VertexOutputFormat::PointsClipId20)
    }
    /// Bind indexed text triangle invocations; `Self::text(vec![0,1,2], 96)?`.
    pub fn text(indices: Vec<u32>, maximum_bytes: usize) -> Result<Self, VertexOutputFailure> {
        Self::with_format(indices, maximum_bytes, VertexOutputFormat::TextClipUvId28)
    }
    /// Read the immutable output contract; `request.format()`.
    pub fn format(&self) -> VertexOutputFormat {
        self.format
    }
    pub(super) fn require_format(
        &self,
        expected: VertexOutputFormat,
    ) -> Result<(), VertexOutputFailure> {
        if self.format == expected {
            return Ok(());
        }
        Err(VertexOutputFailure::Unsupported {
            operation: "explicit vertex-output format",
            actual: format!("{:?}; expected {expected:?}", self.format),
        })
    }
    /// Decode actual point outputs, preserving signed zero and NaN bits; `request.decode_points(&bytes)?`.
    pub fn decode_points(
        &self,
        bytes: &[u8],
    ) -> Result<Vec<PointVertexOutputRecord>, VertexOutputFailure> {
        self.require_format(VertexOutputFormat::PointsClipId20)?;
        if bytes.len() != self.capacity_bytes {
            return Err(VertexOutputFailure::ByteCount {
                actual: bytes.len(),
                expected: self.capacity_bytes,
            });
        }
        let records = bytes
            .chunks_exact(self.format.stride())
            .map(PointVertexOutputRecord::decode)
            .collect::<Vec<_>>();
        self.require_identity(&records)?;
        Ok(records)
    }
    pub(crate) fn validate_records(&self, bytes: &[u8]) -> Result<(), VertexOutputFailure> {
        match self.format {
            VertexOutputFormat::TrianglesClipNormalId32 => self.decode(bytes).map(|_| ()),
            VertexOutputFormat::PointsClipId20 => self.decode_points(bytes).map(|_| ()),
            VertexOutputFormat::TextClipUvId28 => self.decode_text(bytes).map(|_| ()),
        }
    }
}
impl VertexOutputRequest {
    /// Decode exact clip and UV bits with direct vertex IDs; `request.decode_text(&bytes)?`.
    pub fn decode_text(
        &self,
        bytes: &[u8],
    ) -> Result<Vec<TextVertexOutputRecord>, VertexOutputFailure> {
        self.require_format(VertexOutputFormat::TextClipUvId28)?;
        if bytes.len() != self.capacity_bytes {
            return Err(VertexOutputFailure::ByteCount {
                actual: bytes.len(),
                expected: self.capacity_bytes,
            });
        }
        let records = bytes
            .chunks_exact(self.format.stride())
            .map(TextVertexOutputRecord::decode)
            .collect::<Vec<_>>();
        self.require_identity(&records)?;
        Ok(records)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
/// Observed text vertex outputs from the original indexed draw.
pub struct TextVertexOutputRecord {
    /// Final Vulkan clip-position float bits.
    pub clip_bits: [u32; 4],
    /// Interpolated fragment-input UV float bits before interpolation.
    pub uv_bits: [u32; 2],
    /// Original indexed vertex invocation.
    pub vertex_index: u32,
}
impl TextVertexOutputRecord {
    fn decode(bytes: &[u8]) -> Self {
        let word =
            |offset: usize| u32::from_ne_bytes(bytes[offset..offset + 4].try_into().unwrap());
        Self {
            clip_bits: [word(0), word(4), word(8), word(12)],
            uv_bits: [word(16), word(20)],
            vertex_index: word(24),
        }
    }
}
impl DirectVertexOutput for TextVertexOutputRecord {
    fn direct_index(&self) -> u32 {
        self.vertex_index
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
/// Observed clip bits and direct invocation index, with no invented varyings.
pub struct PointVertexOutputRecord {
    /// Existing final gl_Position float bits under the actual depth convention.
    pub clip_bits: [u32; 4],
    /// Unsigned gl_VertexIndex from the original invocation.
    pub vertex_index: u32,
}
impl PointVertexOutputRecord {
    fn decode(bytes: &[u8]) -> Self {
        let words = bytes
            .chunks_exact(4)
            .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
            .collect::<Vec<_>>();
        Self {
            clip_bits: words[..4].try_into().unwrap(),
            vertex_index: words[4],
        }
    }
}
impl DirectVertexOutput for PointVertexOutputRecord {
    fn direct_index(&self) -> u32 {
        self.vertex_index
    }
}
const _: () = {
    fn assert_send_sync<T: Send + Sync>() {}
    let _ = assert_send_sync::<VertexOutputFormat>;
    let _ = assert_send_sync::<PointVertexOutputRecord>;
    let _ = assert_send_sync::<TextVertexOutputRecord>;
};
