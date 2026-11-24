//! Lossless material records indexed by the first instance of an indirect draw.
use crate::{PrimitiveDraw, RenderError, RenderLighting, error::rejected};
use std::collections::HashMap;

const MATERIAL_WORDS: usize = 8 * 4;
const WORD_BYTES: usize = std::mem::size_of::<u32>();
const MATERIAL_START: usize = super::storage_descriptor::PLACEMENT_BYTES + 2 * 16;
const MATERIAL_END: usize = MATERIAL_START + MATERIAL_WORDS * WORD_BYTES;

/// Replay tooling uses the legacy draw table and compares a one-buffer descriptor payload.
pub(super) const INDEX_MATERIALS: bool = !cfg!(feature = "command-state-diagnostics");

/// Strip the indexed coefficients and transforms from an encoded segment lighting key.
pub(super) fn shared_lighting_key(bytes: &mut Vec<u8>, indexed: bool) {
    if indexed && bytes.len() >= MATERIAL_END {
        bytes.drain(MATERIAL_START..MATERIAL_END);
    }
    bytes.drain(..super::storage_descriptor::PLACEMENT_BYTES.min(bytes.len()));
}

/// The prefix holds absolute word offsets by draw. Records follow in first-seen order.
pub(super) struct DrawMaterialTable {
    bytes: Vec<u8>,
    offsets: HashMap<[u32; MATERIAL_WORDS], u32>,
}

impl DrawMaterialTable {
    pub(super) fn new(draw_count: usize) -> Self {
        Self {
            bytes: vec![0; draw_count * WORD_BYTES],
            offsets: HashMap::new(),
        }
    }

    pub(super) fn intern(&mut self, draw: &PrimitiveDraw) -> Result<u32, RenderError> {
        let lighting = draw
            .lighting
            .as_ref()
            .ok_or_else(|| rejected(draw.indices.len(), "lighting on an indexed material draw"))?;
        let words = material_words(lighting);
        let offset = u32::try_from(self.bytes.len() / WORD_BYTES)
            .map_err(|_| rejected(self.bytes.len(), "a material word offset within u32"))?;
        Ok(*self.offsets.entry(words).or_insert_with(|| {
            self.bytes
                .extend_from_slice(words.map(u32::to_ne_bytes).as_flattened());
            offset
        }))
    }

    pub(super) fn set_draw(&mut self, draw: usize, offset: u32) {
        let start = draw * WORD_BYTES;
        self.bytes[start..start + WORD_BYTES].copy_from_slice(&offset.to_ne_bytes());
    }

    pub(super) fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

fn material_words(lighting: &RenderLighting) -> [u32; MATERIAL_WORDS] {
    let records = lighting
        .packed_material_records()
        .map(|slot| slot.map(f32::to_bits));
    std::array::from_fn(|word| records[word / 4][word % 4])
}
