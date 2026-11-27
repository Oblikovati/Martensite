//! GlTileSampler: the tile maps of adaptive screen sampling. The view is
//! split into square tiles; each frame redirects every offset tile to a tile drawn from the
//! visual-error distribution (Halton samples of the marginal and conditional maps), and the
//! samples map counts how often each tile is sampled. The GPU images (visual error, tile
//! offsets, tile samples) are filled by the path-tracing pass from `samples` and `offsets`.
use super::halton_sampler::RaytraceHaltonSampler;

/// One redirected tile: (column, row); (-1, -1) before the first upload.
pub type RaytraceTileOffset = [i32; 2];

/// The tile maps of one view; `RaytraceTileSampler::default()`, then `set_size`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RaytraceTileSampler {
    /// Tiles per row and per column.
    tiles: [usize; 2],
    /// Samples per tile in the last upload (row-major, top row first).
    tile_counts: Vec<u32>,
    /// Samples of all pixels of each tile (tile area times its count).
    tile_samples: Vec<i32>,
    /// Visual error per tile, accumulated down each column.
    variance: Vec<f32>,
    offsets: TileMap,
    offsets_shrunk: TileMap,
    /// Running sum of the bottom row of `variance`.
    marginal: Vec<f32>,
    sampler: RaytraceHaltonSampler,
    last_sample: u32,
    scale_factor: f32,
    tile_size: i32,
    view_size: [i32; 2],
}

/// A row-major map of tile redirections.
#[derive(Clone, Debug, Default, PartialEq)]
struct TileMap {
    size: [usize; 2],
    values: Vec<RaytraceTileOffset>,
}

impl TileMap {
    fn new(size: [usize; 2]) -> Self {
        Self {
            size,
            values: vec![[-1, -1]; size[0] * size[1]],
        }
    }
}

impl RaytraceTileSampler {
    /// The square tile size in pixels (GlTileSampler::TileSize).
    pub fn tile_size(&self) -> [i32; 2] {
        [self.tile_size; 2]
    }
    /// The quantization factor of the visual error (VarianceScaleFactor).
    pub fn variance_scale_factor(&self) -> f32 {
        self.scale_factor
    }
    /// Tiles per row (NbTilesX).
    pub fn nb_tiles_x(&self) -> i32 {
        self.tiles[0] as i32
    }
    /// Tiles per column (NbTilesY).
    pub fn nb_tiles_y(&self) -> i32 {
        self.tiles[1] as i32
    }
    /// All tiles (NbTiles).
    pub fn nb_tiles(&self) -> i32 {
        (self.tiles[0] * self.tiles[1]) as i32
    }
    /// The ray-tracing viewport (ViewSize).
    pub fn view_size(&self) -> [i32; 2] {
        self.view_size
    }
    /// The offset map size: shrunk for adaptive sampling (NbOffsetTiles).
    pub fn nb_offset_tiles(&self, adaptive: bool) -> [i32; 2] {
        let map = if adaptive {
            &self.offsets_shrunk
        } else {
            &self.offsets
        };
        map.size.map(|value| value as i32)
    }
    /// The larger offset map extent of both modes (NbOffsetTilesMax).
    pub fn nb_offset_tiles_max(&self) -> [i32; 2] {
        let (shrunk, full) = (self.nb_offset_tiles(true), self.nb_offset_tiles(false));
        [shrunk[0].max(full[0]), shrunk[1].max(full[1])]
    }
    /// The viewport the offset tiles cover (OffsetTilesViewport).
    pub fn offset_tiles_viewport(&self, adaptive: bool) -> [i32; 2] {
        self.nb_offset_tiles(adaptive)
            .map(|tiles| tiles * self.tile_size)
    }
    /// The larger offset viewport of both modes (OffsetTilesViewportMax).
    pub fn offset_tiles_viewport_max(&self) -> [i32; 2] {
        self.nb_offset_tiles_max()
            .map(|tiles| tiles * self.tile_size)
    }
    /// The largest sample count of a tile in the last upload (MaxTileSamples).
    pub fn max_tile_samples(&self) -> i32 {
        self.tile_counts.iter().copied().max().unwrap_or(0) as i32
    }
    /// Restart the Halton sequence (Reset).
    pub fn reset(&mut self) {
        self.last_sample = 0;
    }
}

#[path = "tile_sampler_maps.rs"]
mod maps;

#[cfg(test)]
#[path = "tile_sampler_tests.rs"]
mod tests;
