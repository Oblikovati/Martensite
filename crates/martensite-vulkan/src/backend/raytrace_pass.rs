//! The ray-tracing pass of a frame (GlView::runRaytrace), after the raster passes of the
//! ray-traced layers: without FSAA program 0 draws into the frame, depth-tested LEQUAL with the
//! hit depth; with FSAA the image goes through backend/raytrace_fsaa.rs first.
use super::{
    frame::FrameResources,
    raytrace_adaptive::{AdaptiveTargets, AdaptiveTargetsKey},
    raytrace_adaptive_record::{AdaptiveDraw, record_adaptive},
    raytrace_environment::BlackEnvironmentCache,
    raytrace_fsaa::{FSAA_COLOR, FSAA_DEPTH, FsaaTargets, fsaa_offset},
    raytrace_fsaa_record::{begin_pass, begin_targets, finish_targets, swap_targets},
    raytrace_pipeline::{RaytracePipeline, RaytracePipelineCache, RaytraceProgram},
    raytrace_resources::RaytraceResources,
    raytrace_scene_images::RaytraceSceneImages,
    runtime::VulkanRuntime,
    shader_grid_pass::GridTarget,
};
use crate::RenderError;
use ash::vk;
use std::sync::Arc;

/// The renderer's ray-tracing pipelines and black environment.
#[derive(Default)]
pub(super) struct RaytraceCaches {
    pipelines: RaytracePipelineCache,
    environment: BlackEnvironmentCache,
    /// The view's FSAA targets per eye (`aFBOIdx`), kept between frames as GlView keeps
    /// its FBOs.
    fsaa: std::sync::Mutex<[Option<Arc<FsaaTargets>>; 2]>,
    /// The view's adaptive-sampling images per eye, kept between frames like GlView's
    /// textures.
    adaptive: std::sync::Mutex<[Option<Arc<AdaptiveTargets>>; 2]>,
}
impl RaytraceCaches {
    fn fsaa_targets(
        &self,
        runtime: &Arc<VulkanRuntime>,
        (extent, eye): (vk::Extent2D, usize),
    ) -> Result<Arc<FsaaTargets>, RenderError> {
        let mut slots = self.fsaa.lock().map_err(|e| {
            crate::error::rejected(e.to_string(), "unpoisoned ray-tracing FSAA targets")
        })?;
        let slot = &mut slots[eye];
        if let Some(targets) = slot.as_ref().filter(|targets| targets.extent == extent) {
            return Ok(targets.clone());
        }
        let targets = Arc::new(FsaaTargets::new(runtime, extent)?);
        *slot = Some(targets.clone());
        Ok(targets)
    }
    fn adaptive_targets(
        &self,
        runtime: &Arc<VulkanRuntime>,
        (key, eye): (AdaptiveTargetsKey, usize),
    ) -> Result<Arc<AdaptiveTargets>, RenderError> {
        let mut slots = self.adaptive.lock().map_err(|e| {
            crate::error::rejected(e.to_string(), "unpoisoned adaptive-sampling targets")
        })?;
        let slot = &mut slots[eye];
        if let Some(targets) = slot.as_ref().filter(|targets| targets.key == key) {
            return Ok(targets.clone());
        }
        let targets = Arc::new(AdaptiveTargets::new(runtime, key)?);
        *slot = Some(targets.clone());
        Ok(targets)
    }
}

/// The smoothing chain of one frame: its targets and the programs that use them.
pub(super) struct FsaaDraw {
    targets: Arc<FsaaTargets>,
    smooth: Arc<RaytracePipeline>,
    display: Arc<RaytracePipeline>,
    window: [i32; 2],
}

/// The uploaded ray-tracing pass of one frame.
pub(super) struct RaytraceFrameDraw {
    pub span: usize,
    render: Arc<RaytracePipeline>,
    #[cfg(feature = "raytrace-hit-diagnostics")]
    hit: Option<super::raytrace_hit_frame::RayHitFrame>,
    resources: RaytraceResources,
    fsaa: Option<FsaaDraw>,
    pathtrace: Option<super::raytrace_pathtrace::PathtraceDraw>,
    adaptive: Option<AdaptiveDraw>,
}
impl RaytraceFrameDraw {
    /// After the frame completed: hand the adaptive visual error to the view.
    /// Example: `draw.publish_feedback()?`.
    pub fn publish_feedback(&self) -> Result<(), RenderError> {
        #[cfg(feature = "raytrace-hit-diagnostics")]
        if let Some(hit) = &self.hit {
            hit.completed()?;
        }
        match &self.adaptive {
            Some(adaptive) => adaptive.publish_visual_error(),
            None => Ok(()),
        }
    }
}

pub(super) fn fsaa_target() -> GridTarget {
    GridTarget {
        color: FSAA_COLOR,
        depth: FSAA_DEPTH,
        stencil: false,
        samples: vk::SampleCountFlags::TYPE_1,
    }
}

/// The sampled-image sources of the scene set.
pub(super) struct RaytraceImageSources<'a> {
    /// The raster pipelines' sampled-image cache.
    pub pipelines: &'a super::pipeline_variants::RasterPipelines,
    pub retirements: &'a super::retained_submission::RetirementStore,
    /// The frame's uploaded cubemap background.
    pub cube: Option<Arc<super::environment_cube::EnvironmentCube>>,
}

/// Prepare the frame's ray-tracing pass when the frame has one; `prepare_raytrace(..)?`.
pub(super) fn prepare_raytrace(
    runtime: &Arc<VulkanRuntime>,
    caches: &RaytraceCaches,
    frame: &crate::RenderFrame,
    (resources, sources): (&FrameResources, RaytraceImageSources<'_>),
) -> Result<Option<RaytraceFrameDraw>, RenderError> {
    let Some(raytrace) = frame.raytrace.as_deref() else {
        return Ok(None);
    };
    #[cfg(feature = "raytrace-hit-diagnostics")]
    let hit = super::raytrace_hit_frame::RayHitFrame::new(runtime, frame, resources)?;
    require_ray_tracing_textures(raytrace.constants.textures, runtime.raytrace_textures)?;
    let pipeline_with = |program, target, alpha| {
        let key = (program, target, raytrace.constants, alpha);
        caches.pipelines.pipeline(runtime, key)
    };
    let pipeline = |program, target| pipeline_with(program, target, frame.raster.alpha_writes);
    let main = GridTarget::of(resources);
    let render_target = if raytrace.antialiasing {
        fsaa_target()
    } else {
        main
    };
    let render = pipeline(RaytraceProgram::Render, render_target)?;
    let eye_targets = (resources.attachment_extent, raytrace.eye_targets);
    let images = scene_images(runtime, caches, raytrace, sources)?;
    let uploaded = RaytraceResources::new(runtime, &render.scene, raytrace, images)?;
    #[cfg(feature = "raytrace-input-diagnostics")]
    let uploaded = super::raytrace_input_diagnostic::uploaded(uploaded, frame);
    let fsaa = match raytrace.antialiasing {
        false => None,
        true => Some(FsaaDraw {
            targets: caches.fsaa_targets(runtime, eye_targets)?,
            smooth: pipeline(RaytraceProgram::Smooth, fsaa_target())?,
            display: pipeline(RaytraceProgram::Display, main)?,
            window: raytrace.uniforms.window,
        }),
    };
    let adaptive = match raytrace.path_tracing.as_ref() {
        Some(path) if path.adaptive.is_some() => {
            let pipelines = |atomic| -> Result<_, RenderError> {
                let render = match atomic {
                    true => RaytraceProgram::RenderAdaptiveAtomic,
                    false => RaytraceProgram::RenderAdaptive,
                };
                let render = pipeline_with(render, fsaa_target(), true)?;
                Ok((render, pipeline(RaytraceProgram::DisplayAdaptive, main)?))
            };
            let eye = raytrace.eye_targets;
            Some(adaptive_draw(runtime, (caches, eye), path, pipelines)?)
        }
        _ => None,
    };
    let pathtrace = match raytrace.path_tracing.clone() {
        Some(path) if path.adaptive.is_none() => Some(super::raytrace_pathtrace::PathtraceDraw {
            targets: caches.fsaa_targets(runtime, eye_targets)?,
            // runPathtrace forces writes into all components, alpha included.
            render: pipeline_with(RaytraceProgram::RenderPathtrace, fsaa_target(), true)?,
            display: pipeline(RaytraceProgram::DisplayPathtrace, main)?,
            frame: path,
        }),
        _ => None,
    };
    let span = raytrace.span;
    Ok(Some(RaytraceFrameDraw {
        span,
        render,
        resources: uploaded,
        #[cfg(feature = "raytrace-hit-diagnostics")]
        hit,
        fsaa,
        pathtrace,
        adaptive,
    }))
}

/// The images of the scene set: the material textures, the environment texture or the cubemap
/// background, and black stand-ins for the unused bindings.
fn scene_images(
    runtime: &Arc<VulkanRuntime>,
    caches: &RaytraceCaches,
    raytrace: &crate::raytrace::RaytraceFrame,
    sources: RaytraceImageSources<'_>,
) -> Result<RaytraceSceneImages, RenderError> {
    let (pipelines, retirements) = (sources.pipelines, sources.retirements);
    let environment = match &raytrace.environment.map {
        Some(map) => Some(pipelines.raytrace_image(runtime, map, retirements)?),
        None => None,
    };
    let cube = match raytrace.environment.cube {
        true => Some(sources.cube.ok_or_else(|| {
            crate::error::rejected("no cube", "the uploaded cubemap background of the frame")
        })?),
        false => None,
    };
    Ok(RaytraceSceneImages {
        black: caches.environment.get(runtime)?,
        materials: pipelines.raytrace_material_images(runtime, raytrace, retirements)?,
        environment,
        cube,
    })
}

/// Textured materials need non-uniform sampler indexing; the scene only collects textures on
/// devices reporting HasRayTracingTextures, so only a bypassed device check reaches the refusal.
fn require_ray_tracing_textures(textures: bool, supported: bool) -> Result<(), RenderError> {
    if textures && !supported {
        let feature = "shaderSampledImageArrayNonUniformIndexing";
        return Err(crate::RenderPortabilityFailure::MissingDeviceFeature { feature }.into());
    }
    Ok(())
}

type AdaptivePipelines = (Arc<RaytracePipeline>, Arc<RaytracePipeline>);

/// The adaptive pass of `path`; its atomic variant requires float image atomics (the view's
/// flags were admitted against the device, so only a bypassed admission reaches the refusal).
fn adaptive_draw(
    runtime: &Arc<VulkanRuntime>,
    (caches, eye): (&RaytraceCaches, usize),
    path: &crate::raytrace::PathtraceFrame,
    pipelines: impl Fn(bool) -> Result<AdaptivePipelines, RenderError>,
) -> Result<AdaptiveDraw, RenderError> {
    let adaptive = path.adaptive.as_ref().expect("adaptive frame");
    require_float_atomics(adaptive.atomic, runtime.image_float_atomic_add)?;
    let passes = adaptive.passes.as_ref().ok_or_else(|| {
        crate::error::rejected(
            path.accum_frames,
            "adaptive passes from the view's accumulation",
        )
    })?;
    let key = AdaptiveTargetsKey::of(passes, adaptive.atomic);
    let targets = caches.adaptive_targets(runtime, (key, eye))?;
    AdaptiveDraw::new(runtime, targets, pipelines(adaptive.atomic)?, path.clone())
}

/// The atomic variant needs `shaderImageFloat32AtomicAdd`; refuse it on devices without.
fn require_float_atomics(atomic: bool, supported: bool) -> Result<(), RenderError> {
    if atomic && !supported {
        let feature = "shaderImageFloat32AtomicAdd";
        return Err(crate::RenderPortabilityFailure::MissingDeviceFeature { feature }.into());
    }
    Ok(())
}

pub(super) fn bind(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    pipeline: &RaytracePipeline,
    sets: &[vk::DescriptorSet],
    push: &[u8],
) {
    let device = runtime.device();
    let (graphics, fragment) = (
        vk::PipelineBindPoint::GRAPHICS,
        vk::ShaderStageFlags::FRAGMENT,
    );
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        device.cmd_bind_pipeline(command, graphics, pipeline.handle);
        device.cmd_bind_descriptor_sets(command, graphics, pipeline.layout, 0, sets, &[]);
        if !push.is_empty() {
            device.cmd_push_constants(command, pipeline.layout, fragment, 0, push);
        }
        device.cmd_draw(command, 6, 1, 0, 0);
    }
}

/// Record the pass before the span at its index.
pub(super) fn record_raytrace(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    clear: [f32; 4],
    draw: &RaytraceFrameDraw,
) {
    let first = draw.span == 0 && resources.background.is_none();
    if let Some(adaptive) = &draw.adaptive {
        let inputs = (resources, clear, first);
        record_adaptive(runtime, command, inputs, draw.resources.set, adaptive);
        return;
    }
    if let Some(path) = &draw.pathtrace {
        let scene = draw.resources.set;
        super::raytrace_pathtrace::record_pathtrace(
            runtime,
            command,
            (resources, clear, first),
            scene,
            path,
        );
        return;
    }
    if let Some(fsaa) = &draw.fsaa {
        record_smoothing(runtime, command, resources, draw, fsaa);
    }
    if !first {
        super::record::layer_dependency(runtime, command);
    }
    super::record::begin_rendering(runtime, command, resources, clear, first, false, false);
    match &draw.fsaa {
        None => {
            let sets = [draw.resources.set];
            #[cfg(feature = "raytrace-hit-diagnostics")]
            if let Some(hit) = &draw.hit {
                hit.bind(runtime, command, draw.render.layout);
            }
            bind(runtime, command, &draw.render, &sets, &[]);
            #[cfg(feature = "raytrace-input-diagnostics")]
            super::raytrace_input_diagnostic::dispatched(
                command,
                &draw.render,
                &sets,
                &draw.resources,
            );
        }
        // Display.fs without path tracing reads no push constant; the block stays zero.
        Some(fsaa) => bind(
            runtime,
            command,
            &fsaa.display,
            &[fsaa.targets.outputs[1]],
            &[0; 16],
        ),
    }
    super::record::finish_rendering(runtime, command);
    #[cfg(feature = "raytrace-hit-diagnostics")]
    if draw.hit.is_some() {
        super::raytrace_hit_frame::host_barrier(runtime, command);
    }
}

/// The `uFsaaOffset, uSamples` push constants of smoothing pass `iteration`.
fn smoothing_constants(iteration: u32, window: [i32; 2]) -> [u8; 12] {
    let offset = fsaa_offset(iteration, window);
    let mut push = [0u8; 12];
    push[..4].copy_from_slice(&offset[0].to_ne_bytes());
    push[4..8].copy_from_slice(&offset[1].to_ne_bytes());
    push[8..].copy_from_slice(&(iteration as i32 + 1).to_ne_bytes());
    push
}

/// Program 0 into target 1 with a cleared depth, then the three smoothing passes ending in
/// target 2; both targets and the depth are left shader-readable.
fn record_smoothing(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    draw: &RaytraceFrameDraw,
    fsaa: &FsaaDraw,
) {
    let targets = &fsaa.targets;
    let device = runtime.device();
    begin_targets(runtime, command, targets);
    begin_pass(
        runtime,
        command,
        resources,
        &targets.color[0],
        Some(&targets.depth),
    );
    bind(runtime, command, &draw.render, &[draw.resources.set], &[]);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { device.cmd_end_rendering(command) };
    for iteration in 1..=3u32 {
        // Odd passes write target 2 from target 1, even passes the reverse.
        let (source, target) = if iteration % 2 == 1 { (0, 1) } else { (1, 0) };
        swap_targets(runtime, command, targets, source);
        begin_pass(runtime, command, resources, &targets.color[target], None);
        let sets = [draw.resources.set, targets.inputs[source]];
        let push = smoothing_constants(iteration, fsaa.window);
        bind(runtime, command, &fsaa.smooth, &sets, &push);
        // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
        unsafe { device.cmd_end_rendering(command) };
    }
    finish_targets(runtime, command, targets);
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_atomic_variant_is_refused_without_float_image_atomics() {
        let error = super::require_float_atomics(true, false).unwrap_err();
        let feature = "shaderImageFloat32AtomicAdd";
        let expected = crate::RenderPortabilityFailure::MissingDeviceFeature { feature };
        assert_eq!(error.portability_failure(), Some(&expected));
        assert!(super::require_float_atomics(true, true).is_ok());
        assert!(super::require_float_atomics(false, false).is_ok());
    }
    #[test]
    fn textured_materials_are_refused_without_non_uniform_sampler_indexing() {
        let error = super::require_ray_tracing_textures(true, false).unwrap_err();
        let feature = "shaderSampledImageArrayNonUniformIndexing";
        let expected = crate::RenderPortabilityFailure::MissingDeviceFeature { feature };
        assert_eq!(error.portability_failure(), Some(&expected));
        assert!(super::require_ray_tracing_textures(true, true).is_ok());
        assert!(super::require_ray_tracing_textures(false, false).is_ok());
    }
    #[test]
    fn smoothing_passes_accumulate_two_three_and_four_samples() {
        let samples = |i| {
            i32::from_ne_bytes(
                super::smoothing_constants(i, [8, 8])[8..]
                    .try_into()
                    .unwrap(),
            )
        };
        assert_eq!([samples(1), samples(2), samples(3)], [2, 3, 4]);
    }
}
