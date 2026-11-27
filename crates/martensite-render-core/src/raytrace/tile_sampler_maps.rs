//! The map updates of GlTileSampler: SetSize, GrabVarianceMap (from the read-back raw
//! error image), nextTileToSample and the map half of upload (the GPU upload is the caller's).
use super::{RaytraceTileOffset, RaytraceTileSampler, TileMap};
use crate::{RenderError, error::rejected};

impl RaytraceTileSampler {
    /// Size the maps for a `size` viewport with `tile_size` pixel tiles and `nb_tiles` shrunk
    /// offset tiles (RenderingParams RayTracingTileSize and NbRayTracingTiles); a non-positive
    /// size keeps the maps (SetSize). Example: `sampler.set_size(32, 256, [409, 409])`.
    pub fn set_size(&mut self, tile_size: i32, nb_tiles: i32, size: [i32; 2]) {
        if size[0] <= 0 || size[1] <= 0 {
            return;
        }
        self.view_size = size;
        let tile_size = tile_size.max(1);
        let count =
            |extent: i32| ((extent as f32 / tile_size as f32).ceil() as i32).max(1) as usize;
        let tiles = [count(size[0]), count(size[1])];
        if self.tile_size != tile_size || self.tiles != tiles {
            self.resize_tiles(tile_size, tiles);
        }
        let shrunk = shrunk_offsets(nb_tiles, tiles);
        if self.offsets_shrunk.size != shrunk {
            self.offsets_shrunk = TileMap::new(shrunk);
        }
    }

    fn resize_tiles(&mut self, tile_size: i32, tiles: [usize; 2]) {
        let area = tiles[0] * tiles[1];
        self.tile_size = tile_size;
        self.scale_factor = 1.0e6 * (1024. / (tile_size * tile_size) as f32);
        self.tiles = tiles;
        self.tile_counts = vec![1; area];
        self.tile_samples = vec![1; area];
        self.variance = vec![0.; area];
        self.offsets = TileMap::new(tiles);
        self.marginal = vec![0.; tiles[0]];
    }

    /// The pixels of tile (`x`, `y`), clipped by the viewport (tileArea).
    pub fn tile_area(&self, x: i32, y: i32) -> i32 {
        let size_x = self.tile_size.min(self.view_size[0] - x * self.tile_size);
        let size_y = self.tile_size.min(self.view_size[1] - y * self.tile_size);
        size_x * size_y
    }

    /// Rebuild the error distribution from the raw visual-error image (one non-negative
    /// quantized error per tile, row-major) (GrabVarianceMap after the image read-back).
    /// Example: `sampler.grab_variance_map(&raw)?`.
    pub fn grab_variance_map(&mut self, raw: &[i32]) -> Result<(), RenderError> {
        let [columns, rows] = self.tiles;
        if raw.len() != columns * rows {
            return Err(rejected(raw.len(), "one raw visual error per tile"));
        }
        if let Some(negative) = raw.iter().find(|value| **value < 0) {
            return Err(rejected(negative, "a non-negative quantized visual error"));
        }
        let factor = 1. / self.scale_factor;
        for column in 0..columns {
            for row in 0..rows {
                let mut tile = factor * raw[row * columns + column] as f32;
                tile *= 1. / self.tile_area(column as i32, row as i32) as f32;
                if row != 0 {
                    tile += self.variance[(row - 1) * columns + column];
                }
                self.variance[row * columns + column] = tile;
            }
        }
        let bottom = (rows - 1) * columns;
        for column in 0..columns {
            self.marginal[column] = self.variance[bottom + column];
            if column != 0 {
                self.marginal[column] += self.marginal[column - 1];
            }
        }
        Ok(())
    }

    /// The next tile by the error distribution (nextTileToSample).
    fn next_tile_to_sample(&mut self) -> RaytraceTileOffset {
        let [columns, rows] = self.tiles;
        let last = *self.marginal.last().expect("sized tile sampler");
        let ksi_x = self.sampler.sample(0, self.last_sample) * last;
        let mut x = 0;
        while x < columns - 1 && ksi_x > self.marginal[x] {
            x += 1;
        }
        let ksi_y =
            self.sampler.sample(1, self.last_sample) * self.variance[(rows - 1) * columns + x];
        let mut y = 0;
        while y < rows - 1 && ksi_y > self.variance[y * columns + x] {
            y += 1;
        }
        self.last_sample += 1;
        [x as i32, y as i32]
    }

    /// Redirect every offset tile (to itself, or drawn adaptively) and recount the samples;
    /// `None` before `set_size` (upload's map half). Returns the offsets map (row-major) for the
    /// tile-offsets image; `tile_samples` then holds the tile-samples image.
    pub fn update_offsets(&mut self, adaptive: bool) -> Option<&[RaytraceTileOffset]> {
        if self.tile_counts.is_empty() {
            return None;
        }
        self.tile_counts.fill(0);
        let size = self.map(adaptive).size;
        for row in 0..size[1] {
            for column in 0..size[0] {
                let redirect = match adaptive {
                    true => self.next_tile_to_sample(),
                    false => [column as i32, row as i32],
                };
                self.map_mut(adaptive).values[row * size[0] + column] = redirect;
                let tile = redirect[1] as usize * self.tiles[0] + redirect[0] as usize;
                self.tile_counts[tile] += 1;
            }
        }
        self.recount_tile_samples();
        Some(&self.map(adaptive).values)
    }

    fn recount_tile_samples(&mut self) {
        let columns = self.tiles[0];
        for index in 0..self.tile_counts.len() {
            let (x, y) = ((index % columns) as i32, (index / columns) as i32);
            self.tile_samples[index] = self.tile_area(x, y) * self.tile_counts[index] as i32;
        }
    }

    /// The tile-samples image of the last update (row-major, one per tile).
    pub fn tile_samples(&self) -> &[i32] {
        &self.tile_samples
    }

    fn map(&self, adaptive: bool) -> &TileMap {
        if adaptive {
            &self.offsets_shrunk
        } else {
            &self.offsets
        }
    }

    fn map_mut(&mut self, adaptive: bool) -> &mut TileMap {
        if adaptive {
            &mut self.offsets_shrunk
        } else {
            &mut self.offsets
        }
    }
}

/// The shrunk offset map: 8x8 doubled alternately in x and y until it holds `nb_tiles`
/// tiles; the full tile grid when `nb_tiles` is not positive.
fn shrunk_offsets(nb_tiles: i32, tiles: [usize; 2]) -> [usize; 2] {
    if nb_tiles <= 0 {
        return tiles;
    }
    let mut shrunk = [8usize, 8];
    let mut axis = 0;
    while ((shrunk[0] * shrunk[1]) as i32) < nb_tiles {
        shrunk[axis % 2] <<= 1;
        axis += 1;
    }
    shrunk
}
