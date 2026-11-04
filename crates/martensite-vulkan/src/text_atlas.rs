//! Pack unique glyph bitmaps with transparent gutters, preserving native glyph rectangles.
use crate::{GlyphLayout, RasterImage, RenderError, error::rejected};
use martensite_tkservice::font::*;
use std::collections::BTreeMap;
struct GlyphBitmap {
    rect: FontRect,
    extent: [u32; 2],
    alpha: Vec<f32>,
}
struct GlyphInstance {
    symbol: u32,
    offset: [f32; 2],
}
/// Texel columns of a font's glyph tiles, allocated once per glyph in first-use order for the
/// font's whole lifetime, as GlFont::renderGlyph fills its texture row: each tile starts
/// three zero texels right of the previous one. Sampling precision depends on these texel
/// coordinates, so equal columns give equal filtered coverage.
#[derive(Debug, Default)]
pub(crate) struct NativeTileColumns {
    right: u32,
    lefts: BTreeMap<u32, u32>,
}
impl NativeTileColumns {
    const GAP: u32 = 3;
    /// The tile column of `symbol`, allocating it after the last tile when new.
    /// Example: `columns.left_of(65, 12)`.
    fn left_of(&mut self, symbol: u32, width: u32) -> u32 {
        if let Some(left) = self.lefts.get(&symbol) {
            return *left;
        }
        let left = self.right + Self::GAP;
        self.right = left + width;
        self.lefts.insert(symbol, left);
        left
    }
}
pub(crate) fn build_glyph_layout(
    font: &FontFTFont,
    formatter: &FontTextFormatter,
    columns: &mut NativeTileColumns,
) -> Result<Option<GlyphLayout>, RenderError> {
    let (glyphs, instances) = collect_glyphs(font, formatter)?;
    if instances.is_empty() {
        return Ok(None);
    }
    let order: Vec<u32> = instances.iter().map(|instance| instance.symbol).collect();
    let (atlas, tiles) = pack_glyphs(&glyphs, &order, columns)?;
    let mut layout = initial_native_glyph_layout(font, formatter, atlas);
    append_native_instances(&mut layout, &glyphs, &tiles, instances);
    Ok(Some(layout))
}
fn append_native_instances(
    layout: &mut GlyphLayout,
    glyphs: &BTreeMap<u32, GlyphBitmap>,
    tiles: &BTreeMap<u32, [u32; 2]>,
    instances: Vec<GlyphInstance>,
) {
    for instance in instances {
        append_quad(
            layout,
            &glyphs[&instance.symbol],
            tiles[&instance.symbol],
            instance.offset,
        );
    }
}
// Subtitle rectangles use formatter advance/line bounds, never the union of rasterized glyph ink.
fn native_formatter_rectangle(formatter: &FontTextFormatter) -> FontRect {
    let mut rectangle = FontRect::default();
    formatter.bnd_box(&mut rectangle);
    rectangle
}
fn initial_native_glyph_layout(
    font: &FontFTFont,
    formatter: &FontTextFormatter,
    atlas: RasterImage,
) -> GlyphLayout {
    GlyphLayout {
        positions: vec![],
        uv: vec![],
        indices: vec![],
        atlas,
        formatter_bounds: native_formatter_rectangle(formatter),
        point_size: font.point_size(),
    }
}
fn collect_glyphs(
    font: &FontFTFont,
    formatter: &FontTextFormatter,
) -> Result<(BTreeMap<u32, GlyphBitmap>, Vec<GlyphInstance>), RenderError> {
    let mut glyphs = BTreeMap::new();
    let mut instances = Vec::new();
    let mut it = FontTextFormatterIterator::new(
        formatter,
        FontTextFormatterIterationFilter::ExcludeInvisible,
    );
    while it.more() {
        if let std::collections::btree_map::Entry::Vacant(slot) = glyphs.entry(it.symbol()) {
            slot.insert(copy_glyph(font, it.symbol())?);
        }
        let offset = formatter
            .bottom_left(it.symbol_position())
            .map_err(|e| rejected(e, "a formatted glyph position"))?;
        instances.push(GlyphInstance {
            symbol: it.symbol(),
            offset: [offset.x, offset.y],
        });
        it.next();
    }
    Ok((glyphs, instances))
}
fn copy_glyph(font: &FontFTFont, symbol: u32) -> Result<GlyphBitmap, RenderError> {
    if !font.render_glyph(symbol) {
        return Err(rejected(symbol, "a renderable font glyph"));
    }
    let image = font.glyph_image();
    let extent = [image.width() as u32, image.height() as u32];
    let alpha = (0..extent[1])
        .flat_map(|y| (0..extent[0]).map(move |x| (x, y)))
        .map(|(x, y)| crate::raster_image::coverage_pixel(image, x, y))
        .collect::<Result<Vec<_>, _>>()?;
    let mut rect = FontRect::default();
    font.glyph_rect(&mut rect);
    Ok(GlyphBitmap {
        rect,
        extent,
        alpha,
    })
}
fn pack_glyphs(
    glyphs: &BTreeMap<u32, GlyphBitmap>,
    order: &[u32],
    columns: &mut NativeTileColumns,
) -> Result<(RasterImage, BTreeMap<u32, [u32; 2]>), RenderError> {
    let tiles = tile_positions(glyphs, order, columns);
    let used = || tiles.iter().map(|(symbol, tile)| (tile, &glyphs[symbol]));
    let right = used()
        .map(|(tile, g)| tile[0] + g.extent[0])
        .max()
        .unwrap_or(0);
    let tallest = used().map(|(_, g)| g.extent[1]).max().unwrap_or(0);
    // Zero texels stay right of and below every tile, as in the native texture.
    let width = (right + NativeTileColumns::GAP).next_power_of_two();
    let height = (tallest + 1).next_power_of_two();
    let mut pixels = vec![[1., 1., 1., 0.]; (width * height) as usize];
    for (tile, glyph) in used() {
        copy_tile(&mut pixels, width, *tile, glyph);
    }
    Ok((RasterImage::new([width, height], pixels)?, tiles))
}
/// Tiles on the texture's top row, so the top edge samples clamped edge texels as the
/// native first tile row does; columns come from the font's allocation in first-use order.
fn tile_positions(
    glyphs: &BTreeMap<u32, GlyphBitmap>,
    order: &[u32],
    columns: &mut NativeTileColumns,
) -> BTreeMap<u32, [u32; 2]> {
    let mut tiles = BTreeMap::new();
    for symbol in order {
        let width = glyphs[symbol].extent[0];
        tiles.insert(*symbol, [columns.left_of(*symbol, width), 0]);
    }
    tiles
}
fn copy_tile(pixels: &mut [[f32; 4]], width: u32, origin: [u32; 2], glyph: &GlyphBitmap) {
    for y in 0..glyph.extent[1] {
        for x in 0..glyph.extent[0] {
            pixels[((origin[1] + y) * width + origin[0] + x) as usize][3] =
                glyph.alpha[(y * glyph.extent[0] + x) as usize];
        }
    }
}
fn append_quad(layout: &mut GlyphLayout, glyph: &GlyphBitmap, origin: [u32; 2], offset: [f32; 2]) {
    let start = layout.positions.len() as u32;
    let r = glyph.rect;
    layout.positions.extend(
        [
            [r.right, r.top],
            [r.left, r.top],
            [r.left, r.bottom],
            [r.right, r.bottom],
        ]
        .map(|p| [(p[0] + offset[0]).floor(), (p[1] + offset[1]).floor()]),
    );
    let [w, h] = layout.atlas.extent();
    let [x, y] = origin;
    let [gw, gh] = glyph.extent;
    layout.uv.extend(
        [[x + gw, y], [x, y], [x, y + gh], [x + gw, y + gh]]
            .map(|p| [p[0] as f32 / w as f32, p[1] as f32 / h as f32]),
    );
    layout.indices.extend([0, 1, 2, 3, 0, 2].map(|i| i + start));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_tile_columns_keep_first_use_order_and_zero_gaps() {
        let glyph = GlyphBitmap {
            rect: FontRect {
                left: -1.,
                right: 129.,
                top: 2.,
                bottom: 0.,
            },
            extent: [130, 2],
            alpha: vec![0.5; 260],
        };
        let glyphs = BTreeMap::from([
            (65, glyph),
            (
                66,
                GlyphBitmap {
                    rect: FontRect::default(),
                    extent: [130, 2],
                    alpha: vec![1.; 260],
                },
            ),
        ]);
        let mut columns = NativeTileColumns::default();
        let (atlas, tiles) = pack_glyphs(&glyphs, &[66, 65, 66], &mut columns).unwrap();
        // 3 + 130 + 3 + 130 + 3 texels wide, one zero row below the 2-texel glyphs.
        assert_eq!(atlas.extent(), [512, 4]);
        assert_eq!(tiles[&66], [3, 0]);
        assert_eq!(tiles[&65], [136, 0]);
        assert_eq!(atlas.pixels()[2][3], 0.);
        assert_eq!(atlas.pixels()[3][3], 1.);
        assert_eq!(atlas.pixels()[136][3], 0.5);
        assert_eq!(atlas.pixels()[2 * 512 + 3][3], 0.);
        // A later text reuses the columns allocated first.
        let (_, again) = pack_glyphs(&glyphs, &[65], &mut columns).unwrap();
        assert_eq!(again[&65], [136, 0]);
        let mut layout = GlyphLayout {
            positions: vec![],
            uv: vec![],
            indices: vec![],
            atlas,
            formatter_bounds: martensite_tkservice::font::FontRect::default(),
            point_size: 16,
        };
        append_quad(&mut layout, &glyphs[&65], tiles[&65], [0.5, -0.5]);
        assert_eq!(
            layout.positions,
            vec![[129., 1.], [-1., 1.], [-1., -1.], [129., -1.]]
        );
        assert_eq!(layout.uv[1], [136. / 512., 0.]);
        assert_eq!(layout.indices, vec![0, 1, 2, 3, 0, 2]);
    }
}

#[cfg(test)]
#[path = "subtitle_bounds_tests.rs"]
mod subtitle_bounds_tests;
