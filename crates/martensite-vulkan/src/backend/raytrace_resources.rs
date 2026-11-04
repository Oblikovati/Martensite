//! Descriptor set 0 of the ray-tracing passes (GlView::uploadRaytraceData and
//! bindRaytraceTextures): the RaytraceUniforms block, the ten scene texel buffers in the
//! formats of their GlTextureBuffer (RGBA32I nodes and triangles, RGB32F points, RGBA32F
//! transforms, materials and lights, RG32F texture coordinates), the environment maps and, for
//! the USE_TEXTURES programs, the material sampler array.
use super::{
    buffer::DeviceBuffer,
    memory_selection::HostBufferAccess,
    raytrace_scene_images::RaytraceSceneImages,
    runtime::{VulkanRuntime, vk_error},
};
use crate::RenderError;
use crate::raytrace::{MAX_TEX_NUMBER, RaytraceFrame};
use ash::vk;
use std::sync::Arc;

/// Binding of the environment map (`uEnvMapTexture`) after the block and the buffers.
const ENVIRONMENT_BINDING: u32 = 11;
/// Binding of the material sampler array (`uTextureSamplers`) of the USE_TEXTURES programs.
const TEXTURES_BINDING: u32 = 12;
/// Binding of the environment cube map (`uEnvCubeMapTexture`, BACKGROUND_CUBEMAP).
const CUBE_BINDING: u32 = 13;
/// The texel-buffer formats of bindings 1 to 10.
const TEXEL_FORMATS: [vk::Format; 10] = [
    vk::Format::R32G32B32A32_SINT,   // uSceneNodeInfoTexture
    vk::Format::R32G32B32_SFLOAT,    // uSceneMinPointTexture
    vk::Format::R32G32B32_SFLOAT,    // uSceneMaxPointTexture
    vk::Format::R32G32B32A32_SFLOAT, // uSceneTransformTexture
    vk::Format::R32G32B32_SFLOAT,    // uGeometryVertexTexture
    vk::Format::R32G32B32_SFLOAT,    // uGeometryNormalTexture
    vk::Format::R32G32_SFLOAT,       // uGeometryTexCrdTexture
    vk::Format::R32G32B32A32_SINT,   // uGeometryTriangTexture
    vk::Format::R32G32B32A32_SFLOAT, // uRaytraceMaterialTexture
    vk::Format::R32G32B32A32_SFLOAT, // uRaytraceLightSrcTexture
];

/// The set 0 layout, with the sampler array when `textures`;
/// `RaytraceLayout::new(&runtime, false)?`.
pub(super) struct RaytraceLayout {
    runtime: Arc<VulkanRuntime>,
    pub handle: vk::DescriptorSetLayout,
}
impl RaytraceLayout {
    pub fn new(runtime: &Arc<VulkanRuntime>, textures: bool) -> Result<Self, RenderError> {
        let binding = |index: u32, kind, count: usize| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(index)
                .descriptor_type(kind)
                .descriptor_count(count as u32)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT)
        };
        let sampler = vk::DescriptorType::COMBINED_IMAGE_SAMPLER;
        let mut bindings = vec![binding(0, vk::DescriptorType::UNIFORM_BUFFER, 1)];
        bindings.extend((1..=10).map(|i| binding(i, vk::DescriptorType::UNIFORM_TEXEL_BUFFER, 1)));
        bindings.push(binding(ENVIRONMENT_BINDING, sampler, 1));
        if textures {
            bindings.push(binding(TEXTURES_BINDING, sampler, MAX_TEX_NUMBER));
        }
        bindings.push(binding(CUBE_BINDING, sampler, 1));
        let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let handle = unsafe { runtime.device().create_descriptor_set_layout(&info, None) }
            .map_err(|e| vk_error("create ray-tracing layout", e))?;
        Ok(Self {
            runtime: runtime.clone(),
            handle,
        })
    }
}
impl Drop for RaytraceLayout {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            self.runtime
                .device()
                .destroy_descriptor_set_layout(self.handle, None)
        };
    }
}

fn floats<const N: usize>(values: &[[f32; N]]) -> Vec<u8> {
    values
        .iter()
        .flatten()
        .flat_map(|v| v.to_ne_bytes())
        .collect()
}
fn ints(values: &[[i32; 4]]) -> Vec<u8> {
    values
        .iter()
        .flatten()
        .flat_map(|v| v.to_ne_bytes())
        .collect()
}

/// The bytes of bindings 1 to 10; an empty buffer keeps one zero texel.
fn texel_contents(frame: &RaytraceFrame) -> [Vec<u8>; 10] {
    let buffers = &frame.buffers;
    let materials: Vec<[f32; 4]> = frame.materials.iter().flat_map(|m| m.records).collect();
    let lights: Vec<[f32; 4]> = frame.lights.sources.iter().flatten().copied().collect();
    [
        ints(&buffers.node_info),
        floats(&buffers.min_points),
        floats(&buffers.max_points),
        floats(&buffers.transforms),
        floats(&buffers.vertices),
        floats(&buffers.normals),
        floats(&buffers.tex_coords),
        ints(&buffers.triangles),
        floats(&materials),
        floats(&lights),
    ]
}

/// One frame's uploaded scene and its bound set; kept alive until the frame retires.
pub(super) struct RaytraceResources {
    runtime: Arc<VulkanRuntime>,
    _uniforms: DeviceBuffer,
    buffers: Vec<DeviceBuffer>,
    views: Vec<vk::BufferView>,
    images: RaytraceSceneImages,
    textured: bool,
    pool: vk::DescriptorPool,
    pub set: vk::DescriptorSet,
    #[cfg(feature = "raytrace-input-diagnostics")]
    pub observation: Option<Arc<crate::raytrace_input_observation::RayInputObservation>>,
    #[cfg(feature = "raytrace-input-diagnostics")]
    pub uniform_bytes: Vec<u8>,
    #[cfg(feature = "raytrace-input-diagnostics")]
    pub uniform_handle: vk::Buffer,
    #[cfg(feature = "raytrace-input-diagnostics")]
    pub uniform_layout: vk::DescriptorSetLayout,
    #[cfg(feature = "raytrace-hit-diagnostics")]
    pub hit_scene: Vec<super::raytrace_hit_scene::HitTexelUpload>,
}
impl RaytraceResources {
    /// Upload `frame` and write its set; `RaytraceResources::new(rt, &layout, frame, images)?`.
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        layout: &RaytraceLayout,
        frame: &RaytraceFrame,
        images: RaytraceSceneImages,
    ) -> Result<Self, RenderError> {
        let uniform_bytes = frame.uniforms.bytes();
        let uniforms = upload(
            runtime,
            &uniform_bytes,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
        )?;
        #[cfg(feature = "raytrace-input-diagnostics")]
        let uniform_handle = uniforms.handle;
        let mut resources = Self {
            runtime: runtime.clone(),
            _uniforms: uniforms,
            buffers: Vec::new(),
            views: Vec::new(),
            images,
            textured: frame.constants.textures,
            pool: vk::DescriptorPool::null(),
            set: vk::DescriptorSet::null(),
            #[cfg(feature = "raytrace-input-diagnostics")]
            observation: None,
            #[cfg(feature = "raytrace-input-diagnostics")]
            uniform_bytes: uniform_bytes.to_vec(),
            #[cfg(feature = "raytrace-input-diagnostics")]
            uniform_handle,
            #[cfg(feature = "raytrace-input-diagnostics")]
            uniform_layout: layout.handle,
            #[cfg(feature = "raytrace-hit-diagnostics")]
            hit_scene: Vec::new(),
        };
        for (bytes, format) in texel_contents(frame).iter().zip(TEXEL_FORMATS) {
            resources.add_texel_buffer(bytes, format)?;
        }
        resources.pool = create_pool(runtime, resources.textured)?;
        resources.set = resources.allocate_set(layout)?;
        Ok(resources)
    }
    fn add_texel_buffer(&mut self, bytes: &[u8], format: vk::Format) -> Result<(), RenderError> {
        // 48 bytes hold whole texels of every format.
        let padded = if bytes.is_empty() {
            vec![0; 48]
        } else {
            bytes.to_vec()
        };
        let buffer = upload(
            &self.runtime,
            &padded,
            vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER,
        )?;
        let info = vk::BufferViewCreateInfo::default()
            .buffer(buffer.handle)
            .format(format)
            .range(vk::WHOLE_SIZE);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let view = unsafe { self.runtime.device().create_buffer_view(&info, None) }
            .map_err(|e| vk_error("create ray-tracing texel buffer view", e))?;
        #[cfg(feature = "raytrace-hit-diagnostics")]
        if self.runtime.ray_hit_control.is_some() {
            self.hit_scene
                .push(super::raytrace_hit_scene::HitTexelUpload::new(
                    buffer.handle,
                    view,
                    format,
                    &padded,
                ));
        }
        self.buffers.push(buffer);
        self.views.push(view);
        Ok(())
    }
    fn allocate_set(&self, layout: &RaytraceLayout) -> Result<vk::DescriptorSet, RenderError> {
        let layouts = [layout.handle];
        let info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.pool)
            .set_layouts(&layouts);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let set = unsafe { self.runtime.device().allocate_descriptor_sets(&info) }
            .map_err(|e| vk_error("allocate ray-tracing set", e))?[0];
        let block = [vk::DescriptorBufferInfo::default()
            .buffer(self._uniforms.handle)
            .range(vk::WHOLE_SIZE)];
        let image = [self.images.environment_info()];
        let cube = [self.images.cube_info()];
        let array = self.images.sampler_array();
        let mut writes = vec![
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info(&block),
            sampled_write(set, ENVIRONMENT_BINDING, &image),
            sampled_write(set, CUBE_BINDING, &cube),
        ];
        if self.textured {
            writes.push(sampled_write(set, TEXTURES_BINDING, &array));
        }
        let views: Vec<[vk::BufferView; 1]> = self.views.iter().map(|view| [*view]).collect();
        for (index, view) in views.iter().enumerate() {
            writes.push(
                vk::WriteDescriptorSet::default()
                    .dst_set(set)
                    .dst_binding(index as u32 + 1)
                    .descriptor_type(vk::DescriptorType::UNIFORM_TEXEL_BUFFER)
                    .texel_buffer_view(view),
            );
        }
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        unsafe { self.runtime.device().update_descriptor_sets(&writes, &[]) };
        Ok(set)
    }
}
impl Drop for RaytraceResources {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            let device = self.runtime.device();
            if self.pool != vk::DescriptorPool::null() {
                device.destroy_descriptor_pool(self.pool, None);
            }
            for view in self.views.drain(..) {
                device.destroy_buffer_view(view, None);
            }
        }
    }
}

/// A combined image sampler write of `binding`.
fn sampled_write<'a>(
    set: vk::DescriptorSet,
    binding: u32,
    infos: &'a [vk::DescriptorImageInfo],
) -> vk::WriteDescriptorSet<'a> {
    vk::WriteDescriptorSet::default()
        .dst_set(set)
        .dst_binding(binding)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .image_info(infos)
}

fn upload(
    runtime: &Arc<VulkanRuntime>,
    bytes: &[u8],
    usage: vk::BufferUsageFlags,
) -> Result<DeviceBuffer, RenderError> {
    let buffer = DeviceBuffer::new(runtime, bytes.len() as u64, usage, HostBufferAccess::Upload)?;
    buffer.write(bytes)?;
    Ok(buffer)
}

fn create_pool(runtime: &VulkanRuntime, textures: bool) -> Result<vk::DescriptorPool, RenderError> {
    let samplers = 2 + if textures { MAX_TEX_NUMBER as u32 } else { 0 };
    let size = |ty, descriptor_count| vk::DescriptorPoolSize {
        ty,
        descriptor_count,
    };
    let sizes = [
        size(vk::DescriptorType::UNIFORM_BUFFER, 1),
        size(vk::DescriptorType::UNIFORM_TEXEL_BUFFER, 10),
        size(vk::DescriptorType::COMBINED_IMAGE_SAMPLER, samplers),
    ];
    let info = vk::DescriptorPoolCreateInfo::default()
        .max_sets(1)
        .pool_sizes(&sizes);
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe { runtime.device().create_descriptor_pool(&info, None) }
        .map_err(|e| vk_error("create ray-tracing pool", e))
}
