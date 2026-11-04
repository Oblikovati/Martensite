//! Glyph atlases are sampled images with bilinear filtering, as GlFont binds its
//! alpha texture (Graphic3dTOTF_BILINEAR, no repeat, no mipmaps); the device sampler then
//! weights texels exactly as the native driver's texture unit does.
use super::{
    retained_submission::RetirementStore,
    runtime::VulkanRuntime,
    sampled_surface_image::SampledSurfaceImage,
    surface_image_descriptor::{SurfaceImageBinding, SurfaceImageLayout},
};
use crate::{
    RasterImage, RenderError,
    surface_texture::{SurfaceImageEncoding, SurfaceSamplerPolicy, SurfaceTextureImage},
};
use ash::vk;
use martensite_tkservice::graphic3d::{
    Graphic3dLevelOfTextureAnisotropy, Graphic3dTextureParams, Graphic3dTypeOfTextureFilter,
};
use std::sync::Arc;

/// Upload `atlas` as an 8-bit RGBA sampled image bound for bilinear sampling.
/// Example: `upload_glyph_atlas(runtime, layout, &style.image, retirements)?`.
pub(super) fn upload_glyph_atlas(
    runtime: &Arc<VulkanRuntime>,
    layout: &SurfaceImageLayout,
    atlas: &RasterImage,
    retirements: &RetirementStore,
) -> Result<Arc<SurfaceImageBinding>, RenderError> {
    let source = glyph_atlas_source(atlas);
    let image =
        SampledSurfaceImage::new(runtime, &source, vk::Format::R8G8B8A8_UNORM, retirements)?;
    SurfaceImageBinding::new(&image, layout, &glyph_sampler_policy())
}

/// Atlas texels in upload order; coverage came from 8-bit FreeType bitmaps, so rounding
/// restores the original byte. Example: `glyph_atlas_source(&atlas).bytes.len()`.
fn glyph_atlas_source(atlas: &RasterImage) -> SurfaceTextureImage {
    let bytes = atlas
        .pixels()
        .iter()
        .flatten()
        .map(|value| (value.clamp(0., 1.) * 255.).round() as u8)
        .collect();
    SurfaceTextureImage {
        lifetime: Arc::default(),
        id: Vec::new(),
        revision: 0,
        extent: atlas.extent(),
        depth: None,
        encoding: SurfaceImageEncoding::Linear8,
        alpha_only: false,
        has_alpha: false,
        bytes: Arc::new(bytes),
        top_down: true,
        mipmaps: false,
        supplied_levels: Vec::new(),
    }
}

/// Clamped bilinear sampling of the single atlas level.
fn glyph_sampler_policy() -> SurfaceSamplerPolicy {
    let mut policy = SurfaceSamplerPolicy::read(&Graphic3dTextureParams::default());
    policy.repeat = false;
    policy.filter = Graphic3dTypeOfTextureFilter::Bilinear;
    policy.anisotropy = Graphic3dLevelOfTextureAnisotropy::Off;
    policy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atlas_coverage_rounds_back_to_bytes() {
        let atlas =
            RasterImage::new([2, 1], vec![[1., 1., 1., 0.], [1., 1., 1., 128. / 255.]]).unwrap();
        let source = glyph_atlas_source(&atlas);
        assert_eq!(*source.bytes, vec![255, 255, 255, 0, 255, 255, 255, 128]);
        assert_eq!(source.extent, [2, 1]);
        assert!(!source.mipmaps);
    }

    #[test]
    fn atlas_sampler_is_clamped_bilinear_without_anisotropy() {
        let policy = glyph_sampler_policy();
        assert!(!policy.repeat);
        assert_eq!(policy.filter, Graphic3dTypeOfTextureFilter::Bilinear);
        assert_eq!(policy.anisotropy, Graphic3dLevelOfTextureAnisotropy::Off);
    }
}
