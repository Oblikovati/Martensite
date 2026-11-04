//! The uploaded part of one user-program draw: its interleaved vertices in the native
//! attribute layout, its indices, the default uniform block and the descriptor sets, and the
//! recording of the draw with the program's pipeline.
use super::{
    buffer::DeviceBuffer,
    memory_selection::HostBufferAccess,
    oit_pipeline::{OitGeometryPhase, OitPipelineTarget},
    runtime::{VulkanRuntime, vk_error},
    surface_image_descriptor::SurfaceImageBinding,
    user_program_pipeline::{UserPipelineState, UserProgramPipeline, UserSampler},
};
use crate::{PrimitiveDraw, RenderError, error::rejected, user_program::UserProgramDraw};
use ash::vk;
use std::sync::Arc;

/// GL's current generic attribute for an attribute array the mesh does not supply.
const GENERIC_ATTRIBUTE: [f32; 4] = [0., 0., 0., 1.];

/// The textures a user program can sample: the draw's texture set by `Graphic3dTextureUnit`
/// (base colour, emissive, occlusion, normal, metallic-roughness) and the 2D and 3D bindings of
/// a unit without a texture of the sampler's dimensionality.
/// Example: `UserProgramTextures { units, unbound }.sampled(&sampler)`.
pub(super) struct UserProgramTextures {
    pub units: [Option<Arc<SurfaceImageBinding>>; 5],
    /// The incomplete texture of a `sampler2D`, then of a `sampler3D`.
    pub unbound: [Arc<SurfaceImageBinding>; 2],
}
impl UserProgramTextures {
    /// The binding `sampler` reads. A GL unit holds one texture per target: a `sampler3D` sees
    /// only a 3D texture of its unit and a `sampler2D` only a 2D one; otherwise the target is
    /// unbound and reads the incomplete texture (0, 0, 0, 1).
    fn sampled(&self, sampler: &UserSampler) -> &Arc<SurfaceImageBinding> {
        let bound = sampler.unit.and_then(|unit| self.units.get(unit)?.as_ref());
        let bound = bound.filter(|binding| binding.is_volume() == sampler.volume);
        bound.unwrap_or(&self.unbound[usize::from(sampler.volume)])
    }
}

/// One draw's buffers and sets; dropped after the frame retires.
pub(super) struct UploadedUserProgram {
    runtime: Arc<VulkanRuntime>,
    pub pipeline: Arc<UserProgramPipeline>,
    /// The fixed state `pipeline` was linked with.
    pub fixed: UserPipelineState,
    /// The registered stages, to relink for another fixed state.
    pub modules: Arc<crate::user_program::UserProgramModules>,
    /// The initial and peeling pipelines of a depth-peeling frame.
    peeling: Option<[Arc<UserProgramPipeline>; 2]>,
    /// Whether native writes the program's colour into every draw buffer (`gl_FragColor`).
    broadcasts_color: bool,
    pool: vk::DescriptorPool,
    sets: [vk::DescriptorSet; 2],
    vertices: DeviceBuffer,
    indices: DeviceBuffer,
    count: u32,
    _uniforms: DeviceBuffer,
    /// The uploaded default block, patched for the weighted-OIT pass.
    block: Vec<u8>,
    textures: UserProgramTextures,
    #[cfg(all(test, target_os = "linux"))]
    owned_resources: Option<owned_capture::OwnedResourceCapture>,
}

impl UploadedUserProgram {
    /// Upload `program` for `draw`; each `occSamplerN` reads unit N of `textures`;
    /// `UploadedUserProgram::new(rt, pipeline, ..)?`.
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        (pipeline, fixed): (Arc<UserProgramPipeline>, UserPipelineState),
        draw: &PrimitiveDraw,
        program: &UserProgramDraw,
        textures: UserProgramTextures,
    ) -> Result<Self, RenderError> {
        let count = u32::try_from(draw.indices.len())
            .map_err(|_| rejected(draw.indices.len(), "a 32-bit index count"))?;
        let vertices = upload(
            runtime,
            &user_vertices(draw, &program.tex_coords),
            vk::BufferUsageFlags::VERTEX_BUFFER,
        )?;
        let index_bytes: Vec<u8> = draw.indices.iter().flat_map(|i| i.to_ne_bytes()).collect();
        let indices = upload(runtime, &index_bytes, vk::BufferUsageFlags::INDEX_BUFFER)?;
        // A zero-sized buffer is invalid; a program without uniforms binds 16 unread bytes.
        let uniforms = program.uniform_bytes(draw.lighting.as_ref().map(|l| &l.lights));
        let block = if uniforms.is_empty() {
            vec![0; 16]
        } else {
            uniforms
        };
        let uniforms = upload(runtime, &block, vk::BufferUsageFlags::UNIFORM_BUFFER)?;
        let pool = create_pool(runtime, pipeline.samplers.len() as u32)?;
        let mut uploaded = Self {
            runtime: runtime.clone(),
            pipeline,
            fixed,
            modules: program.modules.clone(),
            peeling: None,
            broadcasts_color: program.broadcasts_color,
            pool,
            sets: [vk::DescriptorSet::null(); 2],
            vertices,
            indices,
            count,
            _uniforms: uniforms,
            block,
            textures,
            #[cfg(all(test, target_os = "linux"))]
            owned_resources: None,
        };
        uploaded.write_sets(uploaded.block.len() as u64)?;
        Ok(uploaded)
    }
    /// Draw in the weighted-OIT pass (a user program in a Graphic3dRTM_BLEND_OIT view):
    /// relink for the OIT `target` and set occOitDepthFactor to `depth_factor`. A program with
    /// the weighted output (occFragCoverage at location 1) writes colour and coverage; a GLSL
    /// 1.10 program writes its one colour into both targets as native `gl_FragColor` does; any
    /// other program is refused: native leaves its coverage output undefined.
    /// Example: `user.select_weighted_oit(rt, cache, target, 0.5)?`.
    pub fn select_weighted_oit(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
        cache: &super::user_program_pipeline::UserProgramPipelineCache,
        target: OitPipelineTarget,
        depth_factor: f32,
    ) -> Result<(), RenderError> {
        let weighted = writes_weighted_oit(&self.modules);
        if !weighted && !self.broadcasts_color {
            return Err(rejected(
                "a blended user program without the weighted OIT output in an OIT frame",
                "user programs created in a weighted-OIT view (two outputs, Graphic3dRTM_BLEND_OIT) or GLSL 1.10 programs with one output",
            ));
        }
        let fixed = self.oit_state(target, !weighted);
        self.pipeline = cache.pipeline(runtime, &self.modules, fixed)?;
        self.fixed = fixed;
        if let Some(offset) = member_offset(&self.modules, "occOitDepthFactor") {
            self.block[offset..offset + 4].copy_from_slice(&depth_factor.to_ne_bytes());
            self._uniforms.write(&self.block)?;
        }
        Ok(())
    }
    /// Draw in the depth-peeling passes (GlLayerList::renderTransparent): native gives
    /// a user program no peeling output, so only a program whose `gl_FragColor` GL writes into
    /// every target draws: its colour lands in the depth, front and back targets alike under
    /// the passes' MAX blending. `targets` are the initial and peeling targets;
    /// `user.select_depth_peeling(rt, cache, [initial, peeling])?`.
    pub fn select_depth_peeling(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
        cache: &super::user_program_pipeline::UserProgramPipelineCache,
        targets: [OitPipelineTarget; 2],
    ) -> Result<(), RenderError> {
        if !self.broadcasts_color {
            return Err(rejected(
                "a blended user program with declared fragment outputs in a depth-peeling frame",
                "GLSL 1.10 programs with one output, whose gl_FragColor native writes into every peeling target",
            ));
        }
        let [initial, peeling] = targets.map(|target| self.oit_state(target, true));
        self.peeling = Some([
            cache.pipeline(runtime, &self.modules, initial)?,
            cache.pipeline(runtime, &self.modules, peeling)?,
        ]);
        self.fixed = peeling;
        Ok(())
    }
    fn oit_state(&self, target: OitPipelineTarget, broadcast: bool) -> UserPipelineState {
        let mut fixed = self.fixed;
        (fixed.oit, fixed.broadcast) = (Some(target), broadcast);
        fixed
    }
    /// The pipeline of an OIT geometry `phase`, or of the ordinary pass for `None`.
    pub fn pipeline_for(&self, phase: Option<OitGeometryPhase>) -> &UserProgramPipeline {
        match (&self.peeling, phase) {
            (Some([initial, _]), Some(OitGeometryPhase::Initial)) => initial,
            (Some([_, peeling]), Some(OitGeometryPhase::Peeling)) => peeling,
            _ => &self.pipeline,
        }
    }
    fn write_sets(&mut self, block_size: u64) -> Result<(), RenderError> {
        let layouts = self.pipeline.set_layouts;
        let info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.pool)
            .set_layouts(&layouts);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let sets = unsafe { self.runtime.device().allocate_descriptor_sets(&info) }
            .map_err(|e| vk_error("allocate user program sets", e))?;
        self.sets = [sets[0], sets[1]];
        let images: Vec<[vk::DescriptorImageInfo; 1]> = self
            .pipeline
            .samplers
            .iter()
            .map(|sampler| [self.textures.sampled(sampler).image_info()])
            .collect();
        let buffer = [vk::DescriptorBufferInfo::default()
            .buffer(self._uniforms.handle)
            .range(block_size)];
        let mut writes: Vec<_> = self
            .pipeline
            .samplers
            .iter()
            .zip(&images)
            .map(|(sampler, image)| {
                vk::WriteDescriptorSet::default()
                    .dst_set(self.sets[0])
                    .dst_binding(sampler.binding)
                    .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                    .image_info(image)
            })
            .collect();
        writes.push(
            vk::WriteDescriptorSet::default()
                .dst_set(self.sets[1])
                .dst_binding(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info(&buffer),
        );
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        unsafe { self.runtime.device().update_descriptor_sets(&writes, &[]) };
        #[cfg(all(test, target_os = "linux"))]
        {
            self.owned_resources = Some(owned_capture::OwnedResourceCapture::new(
                &writes[writes.len() - 1],
                &buffer[0],
            ));
        }
        Ok(())
    }
    /// Bind the sets and buffers and draw; `pipeline` (from `pipeline_for`) is already bound.
    pub fn record(&self, command: vk::CommandBuffer, pipeline: &UserProgramPipeline) {
        let device = self.runtime.device();
        // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
        unsafe {
            device.cmd_bind_descriptor_sets(
                command,
                vk::PipelineBindPoint::GRAPHICS,
                pipeline.layout,
                0,
                &self.sets,
                &[],
            );
            device.cmd_bind_vertex_buffers(command, 0, &[self.vertices.handle], &[0]);
            device.cmd_bind_index_buffer(command, self.indices.handle, 0, vk::IndexType::UINT32);
            device.cmd_draw_indexed(command, self.count, 1, 0, 0, 0);
        }
    }
}

impl Drop for UploadedUserProgram {
    fn drop(&mut self) {
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            self.runtime
                .device()
                .destroy_descriptor_pool(self.pool, None)
        }
    }
}

/// Whether the fragment stage writes the weighted coverage (occFragCoverage at location 1).
fn writes_weighted_oit(modules: &crate::user_program::UserProgramModules) -> bool {
    let fragment = modules.stage(crate::user_program::UserStage::Fragment);
    fragment.is_some_and(|stage| stage.outputs.get("occFragCoverage") == Some(&1))
}

/// The byte offset of the default-block member `name`, when the program declares it.
fn member_offset(modules: &crate::user_program::UserProgramModules, name: &str) -> Option<usize> {
    let block = modules
        .stages
        .iter()
        .find_map(|(reflection, _)| reflection.uniform_block.as_ref())?;
    let member = block.2.iter().find(|member| member.name == name)?;
    Some(member.offset as usize)
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

fn create_pool(runtime: &VulkanRuntime, samplers: u32) -> Result<vk::DescriptorPool, RenderError> {
    let mut sizes = vec![vk::DescriptorPoolSize {
        ty: vk::DescriptorType::UNIFORM_BUFFER,
        descriptor_count: 1,
    }];
    if samplers > 0 {
        sizes.push(vk::DescriptorPoolSize {
            ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            descriptor_count: samplers,
        });
    }
    let info = vk::DescriptorPoolCreateInfo::default()
        .max_sets(2)
        .pool_sizes(&sizes);
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    unsafe { runtime.device().create_descriptor_pool(&info, None) }
        .map_err(|e| vk_error("create user program descriptor pool", e))
}

/// The interleaved native attributes of every vertex: occVertex (position, weight),
/// occNormal (padded to 16 bytes), occTexCoord (`tex_coords`, untransformed) and occVertColor;
/// arrays the mesh lacks read GL's generic attribute (0, 0, 0, 1).
pub(super) fn user_vertices(draw: &PrimitiveDraw, tex_coords: &[[f32; 4]]) -> Vec<u8> {
    let normals = draw.lighting.as_ref().map(|lighting| &lighting.normals);
    let uvs = (!tex_coords.is_empty()).then_some(tex_coords);
    let mut bytes = Vec::with_capacity(draw.vertices.len() * 64);
    for (index, vertex) in draw.vertices.iter().enumerate() {
        let [x, y, z] = vertex.position;
        let normal = normals.map_or([0.; 3], |normals| normals[index]);
        let uv = uvs.map_or(GENERIC_ATTRIBUTE, |uvs| uvs[index]);
        let color = if draw.source_color_array {
            vertex.color
        } else {
            GENERIC_ATTRIBUTE
        };
        let words = [
            [x, y, z, vertex.weight],
            [normal[0], normal[1], normal[2], 0.],
            uv,
            color,
        ];
        bytes.extend(words.iter().flatten().flat_map(|value| value.to_ne_bytes()));
    }
    bytes
}

#[cfg(all(test, target_os = "linux"))]
#[path = "owned_user_capture.rs"]
mod owned_capture;
