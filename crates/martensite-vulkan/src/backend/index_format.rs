//! The width of an index stream in device memory.
use crate::{PrimitiveDraw, PrimitiveTopology, RenderCappingPass};
use ash::vk;
use std::ops::Range;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(in crate::backend) enum IndexFormat {
    U16,
    #[default]
    U32,
}

impl IndexFormat {
    pub(in crate::backend) fn of_draw(draw: &PrimitiveDraw) -> Self {
        if cfg!(any(
            feature = "command-state-diagnostics",
            feature = "vertex-output-diagnostics",
            feature = "lit-only-command-diagnostics"
        )) || draw.topology() != PrimitiveTopology::Triangles
            || draw.vertices.len() > usize::from(u16::MAX) + 1
            || draw.textured_style.is_some()
            || draw.user_program.is_some()
            || draw.capping_pass != RenderCappingPass::None
            || draw.polygon_edge_style.is_some()
            || draw.mesh_edges.is_some()
            || draw
                .lighting
                .as_ref()
                .is_some_and(crate::RenderLighting::is_fixed_lighting)
        {
            return Self::U32;
        }
        Self::U16
    }

    pub(in crate::backend) fn byte_width(self) -> u64 {
        match self {
            Self::U16 => 2,
            Self::U32 => 4,
        }
    }

    pub(in crate::backend) fn vk_type(self) -> vk::IndexType {
        match self {
            Self::U16 => vk::IndexType::UINT16,
            Self::U32 => vk::IndexType::UINT32,
        }
    }

    fn aligned_count(self, count: usize) -> Option<usize> {
        let indices_per_word = (4 / self.byte_width()) as usize;
        count.checked_next_multiple_of(indices_per_word)
    }

    pub(in crate::backend) fn encode(self, indices: &[u32]) -> Option<Vec<u8>> {
        let stride = self.byte_width() as usize;
        let size = self.aligned_count(indices.len())?.checked_mul(stride)?;
        let mut output = Vec::with_capacity(size);
        match self {
            Self::U16 => {
                for &index in indices {
                    output.extend_from_slice(&u16::try_from(index).ok()?.to_ne_bytes());
                }
                output.resize(size, 0);
            }
            Self::U32 => output.extend(indices.iter().flat_map(|index| index.to_ne_bytes())),
        }
        Some(output)
    }

    pub(in crate::backend) fn patch(
        self,
        indices: &[u32],
        range: Range<usize>,
        buffer_size: u64,
    ) -> Option<(u64, Vec<u8>)> {
        let stride = self.byte_width();
        let words = (4 / stride) as usize;
        let full_count = u64::try_from(self.aligned_count(indices.len())?).ok()?;
        if full_count.checked_mul(stride)? != buffer_size {
            return None;
        }
        indices.get(range.clone())?;
        if range.is_empty() {
            return Some((0, Vec::new()));
        }
        // Geometry transfers use whole words. Include neighbors from the current stream.
        let start = range.start / words * words;
        let end = self.aligned_count(range.end)?.min(indices.len());
        let offset = u64::try_from(start).ok()?.checked_mul(stride)?;
        Some((offset, self.encode(&indices[start..end])?))
    }
}

#[cfg(test)]
#[path = "index_format_tests.rs"]
mod tests;
