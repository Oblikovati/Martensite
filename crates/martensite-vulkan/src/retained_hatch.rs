//! Window-anchored polygon coverage owns the actual Graphic3d bitmap snapshot.
use crate::{PrimitiveDraw, RenderError, error::rejected};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::{aspect::AspectInteriorStyle, graphic3d::*};

/// Snapshot the current polygon mask; `apply_hatch(&mut draws, &aspects)?`.
pub(crate) fn apply_hatch(
    draws: &mut [PrimitiveDraw],
    aspects: &Handle<dyn Graphic3dAspectsSource>,
) -> Result<(), RenderError> {
    let rows = source_hatch_rows(&**aspects)?;
    for draw in draws {
        if draw.topology() == crate::PrimitiveTopology::Triangles {
            draw.hatch_rows = rows;
        }
    }
    Ok(())
}

fn source_hatch_rows(
    aspects: &dyn Graphic3dAspectsSource,
) -> Result<Option<[u32; 32]>, RenderError> {
    if aspects.interior_style() != AspectInteriorStyle::Hatch {
        return Ok(None);
    }
    let style = aspects.hatch_style();
    style
        .get()
        .filter(|s| s.hatch_type() != 0)
        .map(hatch_snapshot)
        .transpose()
}

fn hatch_snapshot(style: &Graphic3dHatchStyle) -> Result<[u32; 32], RenderError> {
    let bytes = style
        .pattern()
        .ok_or_else(|| rejected(style.hatch_type(), "a hatch bitmap"))?;
    if bytes.len() < 128 {
        return Err(rejected(bytes.len(), "at least 128 packed hatch bytes"));
    }
    // glPolygonStipple uses BITMAP unpacking with default MSB-first bytes, not host word bits.
    Ok(std::array::from_fn(|row| {
        u32::from_le_bytes(std::array::from_fn(|column| {
            bytes[row * 4 + column].reverse_bits()
        }))
    }))
}

/// Share the existing fragment descriptor only when needed; `needs_fragment_mask(&draw)`.
pub(crate) fn needs_fragment_mask(draw: &PrimitiveDraw) -> bool {
    // Hatch remains fragment coverage even without modelling clip planes.
    draw.clipping.is_some() || polygon_hatch(draw).is_some()
}

fn polygon_hatch(draw: &PrimitiveDraw) -> Option<[u32; 32]> {
    // PolygonMode LINE is governed by line stipple, never polygon stipple.
    draw.hatch_rows.filter(|_| {
        draw.line_style.is_none() && draw.polygon_edge_style.is_none() && draw.point_style.is_none()
    })
}

/// Encode one immutable clip/mask block; `fragment_mask_bytes(&draw, height)`.
pub(crate) fn fragment_mask_bytes(draw: &PrimitiveDraw, height: u32) -> Option<Vec<u8>> {
    if !needs_fragment_mask(draw) {
        return None;
    }
    let empty =
        crate::RenderClipping::new(vec![], crate::identity_clip()).expect("identity affine");
    let mut bytes = draw.clipping.as_ref().unwrap_or(&empty).encode();
    bytes[68..72].copy_from_slice(&u32::from(polygon_hatch(draw).is_some()).to_ne_bytes());
    bytes[72..76].copy_from_slice(&height.to_ne_bytes());
    let rows = polygon_hatch(draw).unwrap_or([u32::MAX; 32]);
    bytes.splice(80..80, rows.into_iter().flat_map(u32::to_ne_bytes));
    Some(bytes)
}

/// Keep hatch state at its legacy offsets while reserving the plane count for the vertex stage.
pub(crate) fn raster_world_clip_fragment_mask_bytes(
    draw: &PrimitiveDraw,
    height: u32,
) -> Option<Vec<u8>> {
    let mut bytes = fragment_mask_bytes(draw, height)?;
    move_plane_count_to_vertex_header(&mut bytes);
    Some(bytes)
}

fn move_plane_count_to_vertex_header(bytes: &mut [u8]) {
    let count = u32::from_ne_bytes(bytes[64..68].try_into().expect("clip count header"));
    bytes[64..68].copy_from_slice(&0u32.to_ne_bytes());
    bytes[76..80].copy_from_slice(&count.to_ne_bytes());
}

#[cfg(test)]
#[path = "../tests/support/hatch_depth_control.rs"]
mod depth_control;

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkservice::aspect::AspectHatchStyle;

    #[test]
    fn grid_snapshot_uses_bitmap_msb_order_and_four_row_period() {
        let style = Graphic3dHatchStyle::predefined(AspectHatchStyle::GridWide);
        let rows = hatch_snapshot(&style).unwrap();
        for (y, row) in rows.into_iter().enumerate() {
            for x in 0usize..32 {
                assert_eq!(
                    row & (1 << x) != 0,
                    y.is_multiple_of(4) || x.is_multiple_of(8)
                );
            }
        }
    }

    fn triangle() -> PrimitiveDraw {
        let vertex = crate::RenderVertex::new([0., 0., 0.], [1.; 4]);
        PrimitiveDraw::new(vec![vertex; 3], vec![0, 1, 2], crate::identity_clip()).unwrap()
    }

    #[test]
    fn mask_header_preserves_original_plane_bytes_and_real_height() {
        let mut draw = triangle();
        draw.clipping = Some(
            crate::RenderClipping::new(vec![vec![[1., 0., 0., -2.]]], crate::identity_clip())
                .unwrap(),
        );
        draw.hatch_rows = Some(std::array::from_fn(|i| 1 << i));
        let original = draw.clipping.as_ref().unwrap().encode();
        let bytes = fragment_mask_bytes(&draw, 409).unwrap();
        assert_eq!(&bytes[208..], &original[80..]);
        assert_eq!(&bytes[68..72], &1u32.to_ne_bytes());
        assert_eq!(&bytes[72..76], &409u32.to_ne_bytes());
        assert_eq!(&bytes[80 + 31 * 4..208], &(1u32 << 31).to_ne_bytes());
    }

    #[test]
    fn raster_clip_header_hides_fragment_planes_and_preserves_vertex_count() {
        let mut draw = triangle();
        draw.clipping = Some(
            crate::RenderClipping::new(vec![vec![[1., 0., 0., -0.25]]], crate::identity_clip())
                .unwrap(),
        );
        draw.hatch_rows = Some([0x1234; 32]);
        let legacy = fragment_mask_bytes(&draw, 64).unwrap();
        let raster = raster_world_clip_fragment_mask_bytes(&draw, 64).unwrap();
        assert_eq!(&raster[64..68], &0u32.to_ne_bytes());
        assert_eq!(&raster[68..76], &legacy[68..76]);
        assert_eq!(&raster[76..80], &1u32.to_ne_bytes());
        assert_eq!(&raster[80..], &legacy[80..]);
    }

    #[test]
    fn ordinary_unmasked_draw_keeps_descriptor_absent() {
        let draw = triangle();
        assert!(!needs_fragment_mask(&draw));
        assert_eq!(fragment_mask_bytes(&draw, 64), None);
    }
    #[test]
    fn live_aspect_changes_replace_the_mask_without_mutating_prior_snapshots() {
        let style: Handle<dyn Graphic3dAspectsSource> =
            Handle::new(Graphic3dAspects::default()).cast();
        style.set_interior_style(AspectInteriorStyle::Hatch);
        style.set_hatch_style_type(AspectHatchStyle::GridWide);
        let mut draws = vec![triangle()];
        apply_hatch(&mut draws, &style).unwrap();
        let prior = draws[0].clone();
        style.set_hatch_style_type(AspectHatchStyle::Horizontal);
        apply_hatch(&mut draws, &style).unwrap();
        assert_ne!(prior.hatch_rows, draws[0].hatch_rows);
        assert_eq!(prior.hatch_rows.unwrap()[1], 0x01010101);
        style.set_interior_style(AspectInteriorStyle::Solid);
        apply_hatch(&mut draws, &style).unwrap();
        assert!(draws[0].hatch_rows.is_none());
    }

    #[test]
    fn polygon_line_mode_retains_its_independent_line_coverage() {
        let mut draw = triangle();
        draw.hatch_rows = Some([0; 32]);
        draw.polygon_edge_style = Some(crate::LineStyle::new(1., 0xffff, 1).unwrap());
        assert!(!needs_fragment_mask(&draw));
        assert!(fragment_mask_bytes(&draw, 64).is_none());
    }
}
