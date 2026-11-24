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
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex, PoisonError, Weak},
};

const MAX_RETAINED_ATLASES: usize = 2048;
const MAX_RETAINED_BYTES: u64 = 64 << 20;

type AtlasKey = (usize, [u32; 2]);

#[derive(Default)]
pub(super) struct GlyphAtlasCache(Mutex<AtlasEntries>);

#[derive(Default)]
struct AtlasEntries {
    atlases: HashMap<AtlasKey, CachedAtlas>,
    order: VecDeque<AtlasKey>,
    bytes: u64,
}

struct CachedAtlas {
    // Keeps the identity unique without retaining the source texel allocation.
    source: Weak<Vec<[f32; 4]>>,
    binding: Arc<SurfaceImageBinding>,
    bytes: u64,
}

impl GlyphAtlasCache {
    /// Binds a live immutable atlas. A submitted frame owns its binding independently.
    pub(super) fn binding(
        &self,
        runtime: &Arc<VulkanRuntime>,
        layout: &SurfaceImageLayout,
        atlas: &RasterImage,
        retirements: &RetirementStore,
    ) -> Result<Arc<SurfaceImageBinding>, RenderError> {
        let key = (Arc::as_ptr(&atlas.pixels).addr(), atlas.extent());
        if !cfg!(feature = "command-state-diagnostics")
            && let Some(binding) = self.cached(key)
        {
            return Ok(binding);
        }
        let (binding, bytes) = upload_glyph_atlas(runtime, layout, atlas, retirements)?;
        if !cfg!(feature = "command-state-diagnostics") {
            self.0
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .insert(
                    key,
                    CachedAtlas {
                        source: Arc::downgrade(&atlas.pixels),
                        binding: Arc::clone(&binding),
                        bytes,
                    },
                );
        }
        Ok(binding)
    }

    fn cached(&self, key: AtlasKey) -> Option<Arc<SurfaceImageBinding>> {
        let entries = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        entries
            .atlases
            .get(&key)
            .map(|atlas| Arc::clone(&atlas.binding))
    }
}

impl AtlasEntries {
    fn insert(&mut self, key: AtlasKey, atlas: CachedAtlas) {
        if atlas.bytes > MAX_RETAINED_BYTES || self.atlases.contains_key(&key) {
            return;
        }
        while self.should_evict(atlas.bytes) {
            let Some(oldest) = self.order.pop_front() else {
                break;
            };
            if let Some(expired) = self.atlases.remove(&oldest) {
                self.bytes -= expired.bytes;
            }
        }
        self.bytes += atlas.bytes;
        self.atlases.insert(key, atlas);
        self.order.push_back(key);
    }

    fn should_evict(&self, new_bytes: u64) -> bool {
        self.atlases.len() >= MAX_RETAINED_ATLASES
            || self.bytes > MAX_RETAINED_BYTES - new_bytes
            || self
                .order
                .front()
                .and_then(|key| self.atlases.get(key))
                .is_some_and(|atlas| atlas.source.strong_count() == 0)
    }
}

/// Upload `atlas` as an 8-bit RGBA sampled image bound for bilinear sampling.
/// Example: `upload_glyph_atlas(runtime, layout, &style.image, retirements)?`.
fn upload_glyph_atlas(
    runtime: &Arc<VulkanRuntime>,
    layout: &SurfaceImageLayout,
    atlas: &RasterImage,
    retirements: &RetirementStore,
) -> Result<(Arc<SurfaceImageBinding>, u64), RenderError> {
    let source = glyph_atlas_source(atlas);
    let image =
        SampledSurfaceImage::new(runtime, &source, vk::Format::R8G8B8A8_UNORM, retirements)?;
    let bytes = image.allocation_bytes();
    SurfaceImageBinding::new(&image, layout, &glyph_sampler_policy())
        .map(|binding| (binding, bytes))
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

    #[cfg(not(feature = "command-state-diagnostics"))]
    #[test]
    fn shared_atlas_keeps_its_binding_without_another_device_upload() {
        let renderer = super::super::resource_lifecycle_test_support::create_resource_renderer();
        let layout = SurfaceImageLayout::new(&renderer.runtime).unwrap();
        let atlas = RasterImage::new([1, 1], vec![[1.; 4]]).unwrap();
        let cache = GlyphAtlasCache::default();
        let rendered = cache
            .binding(&renderer.runtime, &layout, &atlas, &renderer.retirements)
            .unwrap();
        let before = renderer.runtime.work.totals();
        let retained = cache
            .binding(
                &renderer.runtime,
                &layout,
                &atlas.clone(),
                &renderer.retirements,
            )
            .unwrap();
        let work = renderer.runtime.work.totals().since(&before);
        assert!(Arc::ptr_eq(&rendered, &retained));
        assert_eq!(
            (work.images_created, work.uploaded_bytes, work.submissions),
            (0, 0, 0)
        );
        assert!(renderer.validation_errors().unwrap().is_empty());
    }

    #[test]
    fn atlas_sampler_is_clamped_bilinear_without_anisotropy() {
        let policy = glyph_sampler_policy();
        assert!(!policy.repeat);
        assert_eq!(policy.filter, Graphic3dTypeOfTextureFilter::Bilinear);
        assert_eq!(policy.anisotropy, Graphic3dLevelOfTextureAnisotropy::Off);
    }
}
