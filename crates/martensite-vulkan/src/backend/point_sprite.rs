//! Point sprites retain native size limits and immutable nearest-filtered texels.
use super::{
    runtime::VulkanRuntime,
    storage_descriptor::{StorageDescriptorLayout, UploadedStorage},
};
use crate::{RasterImage, RenderError, error::rejected};
use std::sync::{Arc, Mutex};

/// Every marker pipeline defines the same one-binding fragment storage layout.
#[derive(Default)]
pub(super) struct PlainPointSprite(Mutex<Option<Arc<UploadedStorage>>>);

impl PlainPointSprite {
    pub(super) fn binding(
        &self,
        runtime: &Arc<VulkanRuntime>,
        layout: &StorageDescriptorLayout,
        image: &RasterImage,
    ) -> Result<Arc<UploadedStorage>, RenderError> {
        // The plain marker currently carries a white texel. Keep other payloads independent.
        if image.pixels() != [[1.; 4]] {
            return upload_sprite(runtime, layout, image).map(Arc::new);
        }
        let mut stored = self
            .0
            .lock()
            .map_err(|error| rejected(error.to_string(), "unpoisoned plain point sprite"))?;
        if let Some(binding) = stored.as_ref() {
            return Ok(Arc::clone(binding));
        }
        let binding = Arc::new(upload_sprite(runtime, layout, image)?);
        *stored = Some(Arc::clone(&binding));
        Ok(binding)
    }
}

pub(super) fn upload_sprite(
    runtime: &Arc<VulkanRuntime>,
    layout: &StorageDescriptorLayout,
    image: &RasterImage,
) -> Result<UploadedStorage, RenderError> {
    UploadedStorage::new(runtime, layout, &sprite_upload_bytes(image))
}
fn sprite_upload_bytes(image: &RasterImage) -> Vec<u8> {
    // PointCoord follows raster rows in reverse order.
    let rows = image.pixels.chunks_exact(image.extent[0] as usize);
    encode_sprite_texels(rows.rev().flatten())
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
    fn point_rows_flip_and_the_source_remains_immutable() {
        let image = RasterImage::new([1, 2], vec![[0.25; 4], [0.75; 4]]).unwrap();
        let source = image.pixels().to_vec();
        let point = sprite_upload_bytes(&image);
        assert_eq!(&point[..4], &0.75_f32.to_ne_bytes());
        assert_eq!(&point[16..20], &0.25_f32.to_ne_bytes());
        assert_eq!(image.pixels(), &source);
    }
    #[cfg(not(feature = "command-state-diagnostics"))]
    #[test]
    fn plain_point_reuses_its_binding_and_preserves_pixels_in_flight() {
        let mut renderer =
            super::super::resource_lifecycle_test_support::create_resource_renderer();
        let frame = plain_point_frame();
        let reference = crate::RenderBackend::render(&mut renderer, &frame).unwrap();
        assert!(reference.rgba.as_chunks::<4>().0.contains(&[255; 4]));
        let before = renderer.work_ledger().totals();
        let first = renderer.render_deferred(&frame).unwrap();
        let later = renderer.render_deferred(&frame).unwrap();
        let work = renderer.work_ledger().totals().since(&before);
        assert_eq!(
            (work.buffers_created, work.uploaded_bytes),
            (0, 0),
            "{work:?}"
        );
        for readback in [first, later] {
            let capture = renderer.read_frame(&readback).unwrap();
            assert_eq!(capture.rgba, reference.rgba);
            assert_eq!(capture.depth, reference.depth);
        }
        assert!(renderer.validation_errors().unwrap().is_empty());
    }

    #[cfg(not(feature = "command-state-diagnostics"))]
    fn plain_point_frame() -> crate::RenderFrame {
        let mut style = crate::PointStyle::new(1., [1, 1], vec![[1.; 4]]).unwrap();
        style.color_policy = crate::point_style::PointColorPolicy::Plain;
        let point = crate::PrimitiveDraw::points(
            vec![crate::RenderVertex::new([0.125, 0.125, 0.5], [1.; 4])],
            vec![0],
            crate::identity_clip(),
            style,
        )
        .unwrap();
        crate::RenderFrame::new(8, 8, [0., 0., 0., 1.], vec![point]).unwrap()
    }

    #[test]
    fn point_sizes_respect_feature_and_range() {
        assert!(validate_point_size(1., false, [1., 64.]).is_ok());
        assert!(validate_point_size(3., false, [1., 64.]).is_err());
        assert!(validate_point_size(3., true, [1., 64.]).is_ok());
        assert!(validate_point_size(65., true, [1., 64.]).is_err());
    }
}
