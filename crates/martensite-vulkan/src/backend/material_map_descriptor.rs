//! Descriptor set 5 of the material-map fragments: the emissive, occlusion, normal and
//! metallic-roughness maps at bindings 1-4 (`Graphic3dTextureUnit` numbering). A missing map
//! binds the native mock texture of `GlContext::BindTextures`: 2x2 RGBA8 white, black for
//! the normal map.
use super::{
    runtime::{VulkanRuntime, vk_error},
    surface_image_cache::SampledSource,
    surface_image_descriptor::{SurfaceImageBinding, SurfaceImageLayout},
};
use crate::{
    RenderError,
    surface_material_maps::SurfaceMaterialMaps,
    surface_texture::{SurfaceImageEncoding, SurfaceSamplerPolicy, SurfaceTextureImage},
};
use ash::vk;
use std::sync::Arc;

/// The map slot of the normal map, whose mock is black (W = 0 keeps the interpolated normal).
const NORMAL_SLOT: usize = 2;

/// The four-binding layout of set 5; `MaterialMapLayout::new(&runtime)?`.
pub(super) struct MaterialMapLayout {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::DescriptorSetLayout,
}
impl MaterialMapLayout {
    pub fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        let bindings: Vec<_> = (1..=4)
            .map(|binding| {
                vk::DescriptorSetLayoutBinding::default()
                    .binding(binding)
                    .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                    .descriptor_count(1)
                    .stage_flags(vk::ShaderStageFlags::FRAGMENT)
            })
            .collect();
        let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let handle = unsafe { runtime.device().create_descriptor_set_layout(&info, None) }
            .map_err(|e| vk_error("create material map layout", e))?;
        Ok(Self {
            runtime: runtime.clone(),
            handle,
        })
    }
}
impl Drop for MaterialMapLayout {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            let device = self.runtime.device();
            device.destroy_descriptor_set_layout(self.handle, None);
        }
    }
}

/// The white and black mock maps, created once per renderer; `MaterialMapMocks::new(...)?`.
pub(super) struct MaterialMapMocks {
    white: Arc<SurfaceImageBinding>,
    black: Arc<SurfaceImageBinding>,
    unbound: Arc<SurfaceImageBinding>,
    unbound_volume: Arc<SurfaceImageBinding>,
}
impl MaterialMapMocks {
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        layout: &SurfaceImageLayout,
        cache: &super::surface_image_cache::SurfaceImageCache,
        retirements: &super::retained_submission::RetirementStore,
    ) -> Result<Self, RenderError> {
        let sampler = mock_sampler();
        let mock = |texel: [u8; 4], depth: Option<u32>| {
            let image = Arc::new(SurfaceTextureImage {
                depth,
                ..mock_image(texel)
            });
            let sampled = SampledSource {
                image: &image,
                sampler: &sampler,
            };
            cache.image_binding(runtime, layout, sampled, false, retirements)
        };
        Ok(Self {
            white: mock([255; 4], None)?,
            black: mock([0; 4], None)?,
            unbound: mock([0, 0, 0, 255], None)?,
            unbound_volume: mock([0, 0, 0, 255], Some(1))?,
        })
    }
    /// What a user-program sampler of a unit without a texture reads. A user program declares
    /// no texture-set bits, so GlContext::BindTextures binds no mock to its units: they
    /// sample the incomplete default texture, (0, 0, 0, 1) (OpenGL 4.6 section 11.1.3.5).
    pub(super) fn unbound(&self) -> Arc<SurfaceImageBinding> {
        self.unbound.clone()
    }
    /// The same incomplete texture for a `sampler3D`, as a 2x2x1 3D image.
    pub(super) fn unbound_volume(&self) -> Arc<SurfaceImageBinding> {
        self.unbound_volume.clone()
    }
    fn for_slot(&self, slot: usize) -> Arc<SurfaceImageBinding> {
        if slot == NORMAL_SLOT {
            self.black.clone()
        } else {
            self.white.clone()
        }
    }
}
/// GlContext::BindTextures: `ImagePixMap::InitZero(ImageFormat_RGBA, 2, 2, 0, value)`,
/// uploaded as GL_RGBA8 (not sRGB); every texel is `texel`.
fn mock_image(texel: [u8; 4]) -> SurfaceTextureImage {
    SurfaceTextureImage {
        lifetime: Arc::default(),
        id: Vec::new(),
        revision: 0,
        extent: [2, 2],
        depth: None,
        encoding: SurfaceImageEncoding::Linear8,
        alpha_only: false,
        has_alpha: false,
        bytes: Arc::new(texel.repeat(4)),
        top_down: true,
        mipmaps: false,
        supplied_levels: Vec::new(),
    }
}
fn mock_sampler() -> SurfaceSamplerPolicy {
    let parameters = martensite_tkservice::graphic3d::Graphic3dTextureParams::default();
    SurfaceSamplerPolicy::read(&parameters)
}

/// One draw's set 5; it keeps every bound view and sampler alive until the draw retires.
pub(super) struct MaterialMapBinding {
    _sources: Vec<Arc<SurfaceImageBinding>>,
    runtime: Arc<VulkanRuntime>,
    pool: vk::DescriptorPool,
    pub set: vk::DescriptorSet,
}
/// The inputs of one material-map set; `MaterialMapRequest { .. }.bind()?`.
pub(super) struct MaterialMapRequest<'a> {
    pub runtime: &'a Arc<VulkanRuntime>,
    pub layout: &'a MaterialMapLayout,
    pub image_layout: &'a SurfaceImageLayout,
    pub cache: &'a super::surface_image_cache::SurfaceImageCache,
    pub mocks: &'a MaterialMapMocks,
    pub retirements: &'a super::retained_submission::RetirementStore,
}
impl MaterialMapRequest<'_> {
    /// Bind every slot, with the mock map where the texture set has none.
    pub fn bind(&self, maps: &SurfaceMaterialMaps) -> Result<Arc<MaterialMapBinding>, RenderError> {
        let mut sources = Vec::with_capacity(4);
        for (slot, map) in maps.slots.iter().enumerate() {
            sources.push(match map {
                Some(map) => self.map_binding(map)?,
                None => self.mocks.for_slot(slot),
            });
        }
        let mut binding = MaterialMapBinding {
            _sources: Vec::new(),
            runtime: self.runtime.clone(),
            pool: create_map_pool(self.runtime)?,
            set: vk::DescriptorSet::null(),
        };
        binding.set = allocate_map_set(self.runtime, binding.pool, self.layout)?;
        write_map_descriptors(self.runtime, binding.set, &sources);
        binding._sources = sources;
        Ok(Arc::new(binding))
    }
    fn map_binding(
        &self,
        map: &crate::surface_material_maps::SurfaceMaterialMap,
    ) -> Result<Arc<SurfaceImageBinding>, RenderError> {
        let sampled = SampledSource {
            image: &map.image,
            sampler: &map.sampler,
        };
        let cache = self.cache;
        cache.image_binding(
            self.runtime,
            self.image_layout,
            sampled,
            false,
            self.retirements,
        )
    }
}
impl Drop for MaterialMapBinding {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            self.runtime
                .device()
                .destroy_descriptor_pool(self.pool, None);
        }
    }
}
fn create_map_pool(runtime: &VulkanRuntime) -> Result<vk::DescriptorPool, RenderError> {
    let sizes = [vk::DescriptorPoolSize {
        ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
        descriptor_count: 4,
    }];
    let info = vk::DescriptorPoolCreateInfo::default()
        .max_sets(1)
        .pool_sizes(&sizes);
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe { runtime.device().create_descriptor_pool(&info, None) }
        .map_err(|e| vk_error("create material map pool", e))
}
fn allocate_map_set(
    runtime: &VulkanRuntime,
    pool: vk::DescriptorPool,
    layout: &MaterialMapLayout,
) -> Result<vk::DescriptorSet, RenderError> {
    let layouts = [layout.handle];
    let info = vk::DescriptorSetAllocateInfo::default()
        .descriptor_pool(pool)
        .set_layouts(&layouts);
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    let sets = unsafe { runtime.device().allocate_descriptor_sets(&info) }
        .map_err(|e| vk_error("allocate material map set", e))?;
    Ok(sets[0])
}
fn write_map_descriptors(
    runtime: &VulkanRuntime,
    set: vk::DescriptorSet,
    sources: &[Arc<SurfaceImageBinding>],
) {
    let infos: Vec<[vk::DescriptorImageInfo; 1]> =
        sources.iter().map(|source| [source.image_info()]).collect();
    let writes: Vec<_> = infos
        .iter()
        .zip(1u32..)
        .map(|(info, binding)| {
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(binding)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .image_info(info)
        })
        .collect();
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe {
        runtime.device().update_descriptor_sets(&writes, &[]);
    }
}
