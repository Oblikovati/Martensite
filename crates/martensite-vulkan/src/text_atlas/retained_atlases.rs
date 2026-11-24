//! Index equivalent native tiles without keeping pixels alive after layouts retire.
use super::GlyphBitmap;
use crate::{RasterImage, RenderError};
use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, Weak},
};

const MAX_ATLAS_ENTRIES: usize = 512;
const MAX_INDEXED_TILES: usize = 128;

#[derive(Default)]
pub(crate) struct RetainedGlyphAtlases(HashMap<AtlasKey, Weak<Vec<[f32; 4]>>>);

#[derive(Hash, PartialEq, Eq)]
struct AtlasKey {
    font: usize,
    extent: [u32; 2],
    tiles: Vec<AtlasTile>,
}

#[derive(Hash, PartialEq, Eq)]
struct AtlasTile {
    symbol: u32,
    origin: [u32; 2],
    extent: [u32; 2],
}

impl RetainedGlyphAtlases {
    pub(super) fn get_or_pack(
        &mut self,
        font: usize,
        extent: [u32; 2],
        glyphs: &BTreeMap<u32, GlyphBitmap>,
        tiles: &BTreeMap<u32, [u32; 2]>,
        pack: impl FnOnce() -> Result<RasterImage, RenderError>,
    ) -> Result<RasterImage, RenderError> {
        if tiles.len() > MAX_INDEXED_TILES {
            return pack();
        }
        let key = AtlasKey::new(font, extent, glyphs, tiles);
        if let Some(pixels) = self.0.get(&key).and_then(Weak::upgrade) {
            return Ok(RasterImage { extent, pixels });
        }
        let image = pack()?;
        self.remember(key, &image);
        Ok(image)
    }

    fn remember(&mut self, key: AtlasKey, image: &RasterImage) {
        if self.0.len() >= MAX_ATLAS_ENTRIES {
            self.0.retain(|_, pixels| pixels.strong_count() > 0);
        }
        if self.0.len() < MAX_ATLAS_ENTRIES {
            self.0.insert(key, Arc::downgrade(&image.pixels));
        }
    }
}

impl AtlasKey {
    fn new(
        font: usize,
        extent: [u32; 2],
        glyphs: &BTreeMap<u32, GlyphBitmap>,
        tiles: &BTreeMap<u32, [u32; 2]>,
    ) -> Self {
        // The owning font is held for the lifetime of this index. Its face parameters and
        // fallback faces are never replaced, and the native column map only appends.
        let tiles = tiles
            .iter()
            .map(|(&symbol, &origin)| AtlasTile {
                symbol,
                origin,
                extent: glyphs[&symbol].extent,
            })
            .collect();
        Self {
            font,
            extent,
            tiles,
        }
    }
}
