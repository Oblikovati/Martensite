//! The stereo pair pass of GlView::drawStereoPair: the two eye images bound as
//! uLeftSampler and uRightSampler (bilinear, clamped as the forced GlFrameBuffer colour
//! texture filter) and the getStdProgramStereo program over the blit quad, drawn first in the
//! frame as its own rendering scope that clears the attachments.
use super::environment_pipeline::{EnvironmentPipeline, EnvironmentPipelineSpec};
use super::runtime::{VulkanRuntime, vk_error};
use super::sampled_surface_image::SampledSurfaceImage;
use super::{frame::FrameResources, shader_grid_pass::GridTarget};
use crate::RenderError;
use crate::stereo_pair::{StereoPairComposite, StereoProgram};
use crate::surface_texture::SurfaceSamplerPolicy;
use ash::vk;
use martensite_tkservice::graphic3d::{
    Graphic3dLevelOfTextureAnisotropy, Graphic3dTypeOfTextureFilter,
};
use std::sync::{Arc, Mutex};

const STEREO_VERTEX: &[u8] = include_bytes!("../../shaders/stereo_pair.vert.spv");
/// StereoConstants: uMultL and uMultR, or uTexOffset, the window size and the flip flag.
const STEREO_CONSTANTS: usize = 128;

/// The fragment binary of a program; `linear` selects the pow pair of a non-sRGB target.
fn fragment_shader(program: StereoProgram, linear: bool) -> &'static [u8] {
    match (program, linear) {
        (StereoProgram::Anaglyph, false) => {
            include_bytes!("../../shaders/stereo_pair.anaglyph.frag.spv")
        }
        (StereoProgram::Anaglyph, true) => {
            include_bytes!("../../shaders/stereo_pair.anaglyph_linear.frag.spv")
        }
        (StereoProgram::RowInterlaced, _) => {
            include_bytes!("../../shaders/stereo_pair.row_interlaced.frag.spv")
        }
        (StereoProgram::ColumnInterlaced, _) => {
            include_bytes!("../../shaders/stereo_pair.column_interlaced.frag.spv")
        }
        (StereoProgram::ChessBoard, _) => {
            include_bytes!("../../shaders/stereo_pair.chess_board.frag.spv")
        }
        (StereoProgram::SideBySide, _) => {
            include_bytes!("../../shaders/stereo_pair.side_by_side.frag.spv")
        }
        (StereoProgram::OverUnder, _) => {
            include_bytes!("../../shaders/stereo_pair.over_under.frag.spv")
        }
        (StereoProgram::RedCyan, _) => {
            include_bytes!("../../shaders/stereo_pair.red_cyan.frag.spv")
        }
    }
}

/// Whether a colour attachment encodes sRGB (GlContext::ToRenderSRGB).
fn srgb_target(format: vk::Format) -> bool {
    matches!(
        format,
        vk::Format::R8G8B8A8_SRGB | vk::Format::B8G8R8A8_SRGB | vk::Format::A8B8G8R8_SRGB_PACK32
    )
}

/// The two-sampler descriptor layout of the stereo programs.
struct StereoPairLayout {
    runtime: Arc<VulkanRuntime>,
    handle: vk::DescriptorSetLayout,
}

impl StereoPairLayout {
    fn new(runtime: &Arc<VulkanRuntime>) -> Result<Self, RenderError> {
        let binding = |index: u32| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(index)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT)
        };
        let bindings = [binding(0), binding(1)];
        let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        let handle = unsafe { runtime.device().create_descriptor_set_layout(&info, None) }
            .map_err(|e| vk_error("create stereo pair layout", e))?;
        Ok(Self {
            runtime: runtime.clone(),
            handle,
        })
    }
}

impl Drop for StereoPairLayout {
    fn drop(&mut self) {
        let device = self.runtime.device();
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe { device.destroy_descriptor_set_layout(self.handle, None) };
    }
}

type StereoPipelineKey = (GridTarget, StereoProgram, bool);

/// The layout and the pipelines per attachment configuration and program, created on first use.
#[derive(Default)]
pub(super) struct StereoPairPipelineCache {
    // Field order is drop order: pipelines before the layout they reference.
    pipelines: Mutex<Vec<(StereoPipelineKey, Arc<EnvironmentPipeline>)>>,
    layout: Mutex<Option<Arc<StereoPairLayout>>>,
}

fn poisoned(error: impl ToString) -> RenderError {
    crate::error::rejected(error.to_string(), "an unpoisoned stereo pair cache")
}

impl StereoPairPipelineCache {
    fn layout(&self, runtime: &Arc<VulkanRuntime>) -> Result<Arc<StereoPairLayout>, RenderError> {
        let mut layout = self.layout.lock().map_err(poisoned)?;
        if layout.is_none() {
            *layout = Some(Arc::new(StereoPairLayout::new(runtime)?));
        }
        Ok(layout.clone().expect("created stereo pair layout"))
    }
    fn pipeline(
        &self,
        runtime: &Arc<VulkanRuntime>,
        layout: &StereoPairLayout,
        key: StereoPipelineKey,
    ) -> Result<Arc<EnvironmentPipeline>, RenderError> {
        let mut pipelines = self.pipelines.lock().map_err(poisoned)?;
        if let Some((_, pipeline)) = pipelines.iter().find(|(found, _)| *found == key) {
            return Ok(pipeline.clone());
        }
        let pipeline = Arc::new(EnvironmentPipeline::new(runtime, spec(layout, key))?);
        pipelines.push((key, pipeline.clone()));
        Ok(pipeline)
    }
}

fn spec(
    layout: &StereoPairLayout,
    (target, program, linear): StereoPipelineKey,
) -> EnvironmentPipelineSpec<'static> {
    EnvironmentPipelineSpec {
        shaders: [STEREO_VERTEX, fragment_shader(program, linear)],
        topology: vk::PrimitiveTopology::TRIANGLE_STRIP,
        color: target.color,
        depth: target.depth,
        stencil: if target.stencil {
            target.depth
        } else {
            vk::Format::UNDEFINED
        },
        samples: target.samples,
        push: vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)
            .size(STEREO_CONSTANTS as u32),
        set_layout: Some(layout.handle),
    }
}

/// The eye images of one frame, their views and samplers, and the set binding them.
struct StereoPairSet {
    layout: Arc<StereoPairLayout>,
    images: Vec<Arc<SampledSurfaceImage>>,
    views: Vec<vk::ImageView>,
    samplers: Vec<vk::Sampler>,
    pool: vk::DescriptorPool,
    set: vk::DescriptorSet,
}

/// `GlFrameBuffer` colour textures with the bilinear filter drawStereoPair forces.
fn eye_sampler() -> SurfaceSamplerPolicy {
    SurfaceSamplerPolicy {
        repeat: false,
        filter: Graphic3dTypeOfTextureFilter::Bilinear,
        anisotropy: Graphic3dLevelOfTextureAnisotropy::Off,
        first_level: 0,
        last_level: 0,
    }
}

impl StereoPairSet {
    fn new(
        runtime: &Arc<VulkanRuntime>,
        layout: Arc<StereoPairLayout>,
        composite: &StereoPairComposite,
        retirements: &super::retained_submission::RetirementStore,
    ) -> Result<Self, RenderError> {
        let mut set = Self {
            layout,
            images: Vec::new(),
            views: Vec::new(),
            samplers: Vec::new(),
            pool: vk::DescriptorPool::null(),
            set: vk::DescriptorSet::null(),
        };
        for eye in &composite.samplers {
            let format = super::surface_image_format::sampled_format(eye.encoding);
            let image = SampledSurfaceImage::new(runtime, eye, format, retirements)?;
            set.views.push(image.view(0, 1)?);
            let sampler =
                super::surface_sampler::create_surface_sampler(&image, &eye_sampler(), 1)?;
            set.samplers.push(sampler);
            set.images.push(image);
        }
        set.allocate()?;
        Ok(set)
    }
    fn allocate(&mut self) -> Result<(), RenderError> {
        let device = self.layout.runtime.device();
        let sizes = [vk::DescriptorPoolSize {
            ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            descriptor_count: 2,
        }];
        let info = vk::DescriptorPoolCreateInfo::default()
            .max_sets(1)
            .pool_sizes(&sizes);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.pool = unsafe { device.create_descriptor_pool(&info, None) }
            .map_err(|e| vk_error("create stereo pair pool", e))?;
        let layouts = [self.layout.handle];
        let info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.pool)
            .set_layouts(&layouts);
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        self.set = unsafe { device.allocate_descriptor_sets(&info) }
            .map_err(|e| vk_error("allocate stereo pair set", e))?[0];
        self.write();
        Ok(())
    }
    fn write(&self) {
        let infos: Vec<[vk::DescriptorImageInfo; 1]> = (0..2)
            .map(|eye| {
                [vk::DescriptorImageInfo::default()
                    .sampler(self.samplers[eye])
                    .image_view(self.views[eye])
                    .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)]
            })
            .collect();
        let writes: Vec<vk::WriteDescriptorSet> = infos
            .iter()
            .enumerate()
            .map(|(binding, info)| {
                vk::WriteDescriptorSet::default()
                    .dst_set(self.set)
                    .dst_binding(binding as u32)
                    .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                    .image_info(info)
            })
            .collect();
        // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
        unsafe {
            self.layout
                .runtime
                .device()
                .update_descriptor_sets(&writes, &[])
        };
    }
}

impl Drop for StereoPairSet {
    fn drop(&mut self) {
        let device = self.layout.runtime.device();
        // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
        unsafe {
            if self.pool != vk::DescriptorPool::null() {
                device.destroy_descriptor_pool(self.pool, None);
            }
            self.samplers
                .iter()
                .for_each(|s| device.destroy_sampler(*s, None));
            self.views
                .iter()
                .for_each(|v| device.destroy_image_view(*v, None));
        }
    }
}

/// The stereo pair draw of one frame.
pub(super) struct StereoPairFrameDraw {
    pipeline: Arc<EnvironmentPipeline>,
    set: StereoPairSet,
    constants: [u8; STEREO_CONSTANTS],
}

impl StereoPairFrameDraw {
    /// Upload the eye images and select the program's pipeline for the frame's attachments.
    /// Example: `StereoPairFrameDraw::new(rt, &cache, &resources, &composite, &retirements)?`.
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        cache: &StereoPairPipelineCache,
        resources: &FrameResources,
        composite: &StereoPairComposite,
        retirements: &super::retained_submission::RetirementStore,
    ) -> Result<Self, RenderError> {
        let target = GridTarget::of(resources);
        let layout = cache.layout(runtime)?;
        let key = (target, composite.program, !srgb_target(target.color));
        Ok(Self {
            pipeline: cache.pipeline(runtime, &layout, key)?,
            set: StereoPairSet::new(runtime, layout, composite, retirements)?,
            constants: stereo_constants(composite),
        })
    }

    /// Record the pair as the frame's first rendering scope (it clears the attachments).
    /// Example: `draw.record(runtime, command, &resources, clear)`.
    pub fn record(
        &self,
        runtime: &VulkanRuntime,
        command: vk::CommandBuffer,
        resources: &FrameResources,
        clear: [f32; 4],
    ) {
        super::record::begin_rendering(runtime, command, resources, clear, true, true, false);
        let device = runtime.device();
        let stages = vk::ShaderStageFlags::FRAGMENT;
        self.pipeline
            .bind(device, command, Some(self.set.set), stages, &self.constants);
        // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
        unsafe {
            device.cmd_draw(command, 4, 1, 0, 0);
            device.cmd_end_rendering(command);
        }
    }
}

/// The push constants of the program: the anaglyph filters, otherwise uTexOffset, the window
/// size and the flip flag.
fn stereo_constants(composite: &StereoPairComposite) -> [u8; STEREO_CONSTANTS] {
    let words: Vec<[u8; 4]> = if composite.program == StereoProgram::Anaglyph {
        let [left, right] = composite.filters;
        left.iter().chain(&right).map(|v| v.to_ne_bytes()).collect()
    } else {
        let floats = composite.tex_offset.iter().chain(&composite.view_size);
        let flip = i32::from(composite.flip_output).to_ne_bytes();
        floats.map(|v| v.to_ne_bytes()).chain([flip]).collect()
    };
    super::environment_pipeline::push_constant_bytes(words)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interlaced_constants_hold_offset_size_then_flip() {
        let mut composite = crate::stereo_pair::tests_support::composite(StereoProgram::ChessBoard);
        composite.tex_offset = [0.25, -0.5];
        composite.view_size = [4., 2.];
        composite.flip_output = true;
        let bytes = stereo_constants(&composite);
        let float = |at: usize| f32::from_ne_bytes(bytes[at..at + 4].try_into().unwrap());
        assert_eq!(
            [float(0), float(4), float(8), float(12)],
            [0.25, -0.5, 4., 2.]
        );
        assert_eq!(i32::from_ne_bytes(bytes[16..20].try_into().unwrap()), 1);
    }

    #[test]
    fn anaglyph_constants_hold_the_left_then_the_right_filter() {
        let mut composite = crate::stereo_pair::tests_support::composite(StereoProgram::Anaglyph);
        composite.filters[0][0] = 0.5;
        composite.filters[1][15] = 2.;
        let bytes = stereo_constants(&composite);
        let float = |at: usize| f32::from_ne_bytes(bytes[at..at + 4].try_into().unwrap());
        assert_eq!((float(0), float(124)), (0.5, 2.));
    }

    #[test]
    fn only_srgb_targets_skip_the_pow_pair() {
        assert!(srgb_target(vk::Format::R8G8B8A8_SRGB));
        assert!(!srgb_target(vk::Format::R8G8B8A8_UNORM));
        assert_ne!(
            fragment_shader(StereoProgram::Anaglyph, true),
            fragment_shader(StereoProgram::Anaglyph, false)
        );
    }
}
