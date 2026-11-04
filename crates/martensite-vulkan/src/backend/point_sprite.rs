//! Point sprites retain native size limits and immutable nearest-filtered texels.
use super::{
    runtime::VulkanRuntime,
    storage_descriptor::{StorageDescriptorLayout, UploadedStorage},
};
use crate::{RasterImage, RenderError, error::rejected};
use std::sync::Arc;
pub(super) fn upload_sprite(
    runtime: &Arc<VulkanRuntime>,
    layout: &StorageDescriptorLayout,
    image: &RasterImage,
    point_coordinates: bool,
) -> Result<UploadedStorage, RenderError> {
    UploadedStorage::new(
        runtime,
        layout,
        &sprite_upload_bytes(image, point_coordinates),
    )
}
fn sprite_upload_bytes(image: &RasterImage, point_coordinates: bool) -> Vec<u8> {
    // PointCoord follows raster rows; UV-addressed glyphs retain their authored atlas order.
    if point_coordinates {
        let rows = image.pixels.chunks_exact(image.extent[0] as usize);
        return encode_sprite_texels(rows.rev().flatten());
    }
    encode_sprite_texels(image.pixels.iter())
}
fn encode_sprite_texels<'a>(pixels: impl Iterator<Item = &'a [f32; 4]>) -> Vec<u8> {
    pixels
        .flatten()
        .flat_map(|value| value.to_ne_bytes())
        .collect()
}
pub(super) fn validate_point_size(
    size: f32,
    large: bool,
    range: [f32; 2],
) -> Result<(), RenderError> {
    if (!large && size != 1.) || !(range[0]..=range[1]).contains(&size) {
        return Err(rejected(
            (size, large, range),
            "point size within enabled device capabilities",
        ));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_rows_flip_while_glyph_uv_rows_remain_immutable() {
        let image = RasterImage::new([1, 2], vec![[0.25; 4], [0.75; 4]]).unwrap();
        let source = image.pixels().to_vec();
        let point = sprite_upload_bytes(&image, true);
        let glyph = sprite_upload_bytes(&image, false);
        assert_eq!(&point[..4], &0.75_f32.to_ne_bytes());
        assert_eq!(&glyph[..4], &0.25_f32.to_ne_bytes());
        assert_eq!(image.pixels(), &source);
    }
    #[test]
    fn point_sizes_respect_feature_and_range() {
        assert!(validate_point_size(1., false, [1., 64.]).is_ok());
        assert!(validate_point_size(3., false, [1., 64.]).is_err());
        assert!(validate_point_size(3., true, [1., 64.]).is_ok());
        assert!(validate_point_size(65., true, [1., 64.]).is_err());
    }
}
