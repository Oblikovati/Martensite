//! Per-view environment resources of the renderer: the background and IBL source cube maps
//! (GlView myCubeMapParams texture, mySkydomeTexture) and the PBR environment
//! (myPBREnvironment), updated as GlView::Render and updatePBREnvironment do.
use super::environment_cube::background::{BackgroundFrameDraw, BackgroundPipelineCache};
use super::environment_cube::skydome::SkydomePipeline;
use super::environment_cube::{CubeSamplerLayout, EnvironmentCube};
use super::pbr_environment::bake::BakingPipelines;
use super::pbr_environment::{PbrEnvironment, PbrEnvironmentSize};
use super::{frame::FrameResources, runtime::VulkanRuntime};
use crate::environment_frame::*;
use crate::{RenderError, RenderFrame};
use std::sync::{Arc, Weak};

/// Pipelines of the environment passes, created on first use.
#[derive(Default)]
pub(super) struct EnvironmentPipelines {
    created: Option<CreatedEnvironmentPipelines>,
    background: BackgroundPipelineCache,
    colored_quad: super::colored_quad_pass::ColoredQuadPipelineCache,
    stereo_pair: super::stereo_pair_pass::StereoPairPipelineCache,
}

struct CreatedEnvironmentPipelines {
    // Field order is drop order: pipelines before the descriptor layout they reference.
    baking: BakingPipelines,
    skydome: SkydomePipeline,
    layout: CubeSamplerLayout,
}

impl EnvironmentPipelines {
    fn created(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
    ) -> Result<&CreatedEnvironmentPipelines, RenderError> {
        if self.created.is_none() {
            let layout = CubeSamplerLayout::new(runtime)?;
            self.created = Some(CreatedEnvironmentPipelines {
                baking: BakingPipelines::new(runtime, layout.handle)?,
                skydome: SkydomePipeline::new(runtime)?,
                layout,
            });
        }
        Ok(self
            .created
            .as_ref()
            .expect("created environment pipelines"))
    }
}

/// The source cube maps a view sampled last, keyed by what they were made from.
#[derive(Clone, Debug, PartialEq)]
enum CubeKey {
    Image(Vec<u8>, usize),
    Skydome(SkydomeParams),
}

impl CubeKey {
    fn of(source: &EnvironmentCubeSource) -> Self {
        match source {
            EnvironmentCubeSource::Image(pixels) => Self::Image(pixels.id.clone(), pixels.revision),
            EnvironmentCubeSource::Skydome(params) => Self::Skydome(*params),
        }
    }
}

struct ViewEnvironment {
    view: Weak<ViewEnvironmentToken>,
    cubes: Vec<(CubeKey, Arc<EnvironmentCube>)>,
    pbr: Option<Arc<PbrEnvironment>>,
}

/// Every live view's environment; entries of dropped views are released on the next frame.
#[derive(Default)]
pub(super) struct ViewEnvironments {
    views: Vec<ViewEnvironment>,
    pipelines: EnvironmentPipelines,
}

/// The environment resources one frame binds.
#[derive(Default)]
pub(super) struct PreparedEnvironment {
    pub pbr: Option<Arc<PbrEnvironment>>,
    pub background: Option<(Arc<EnvironmentCube>, CubeBackground)>,
    /// The ray tracer's cube map when the background is no longer the cube map.
    pub raytrace_cube: Option<Arc<EnvironmentCube>>,
}

impl ViewEnvironments {
    /// Update the frame's view environment (skydome, cube uploads, PBR bake or clear) before
    /// recording. Example: `let prepared = environments.prepare(&runtime, frame)?`.
    pub fn prepare(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
        frame: &RenderFrame,
    ) -> Result<PreparedEnvironment, RenderError> {
        self.views.retain(|entry| entry.view.strong_count() > 0);
        let Some(environment) = &frame.environment else {
            return Ok(PreparedEnvironment::default());
        };
        let index = self.entry(&environment.view);
        let mut prepared = PreparedEnvironment::default();
        if let Some(background) = &environment.background {
            let cube = self.cube(runtime, index, &background.source)?;
            prepared.background = Some((cube, background.clone()));
        }
        if let Some(source) = &environment.raytrace_cube {
            prepared.raytrace_cube = Some(self.cube(runtime, index, source)?);
        }
        if let Some(pbr) = &environment.pbr {
            prepared.pbr = Some(self.pbr_environment(runtime, index, pbr)?);
        }
        Ok(prepared)
    }

    /// The background draw of a prepared frame; `environments.background_draw(rt, &res, bg)?`.
    pub fn background_draw(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
        resources: &FrameResources,
        (cube, background): (Arc<EnvironmentCube>, CubeBackground),
    ) -> Result<BackgroundFrameDraw, RenderError> {
        let layout = self.pipelines.created(runtime)?.layout.handle;
        BackgroundFrameDraw::new(
            runtime,
            &self.pipelines.background,
            layout,
            resources,
            cube,
            &background,
        )
    }

    /// The corner gradient draw of a frame; `environments.colored_quad_draw(rt, &res, &quad)?`.
    pub fn colored_quad_draw(
        &self,
        runtime: &Arc<VulkanRuntime>,
        resources: &FrameResources,
        quad: &crate::colored_quad_background::ColoredQuadBackground,
    ) -> Result<super::colored_quad_pass::ColoredQuadFrameDraw, RenderError> {
        let cache = &self.pipelines.colored_quad;
        super::colored_quad_pass::ColoredQuadFrameDraw::new(runtime, cache, resources, quad)
    }

    /// The stereo pair draw of a frame;
    /// `environments.stereo_pair_draw(rt, &res, &pair, &retirements)?`.
    pub fn stereo_pair_draw(
        &self,
        runtime: &Arc<VulkanRuntime>,
        resources: &FrameResources,
        pair: &crate::stereo_pair::StereoPairComposite,
        retirements: &super::retained_submission::RetirementStore,
    ) -> Result<super::stereo_pair_pass::StereoPairFrameDraw, RenderError> {
        let cache = &self.pipelines.stereo_pair;
        super::stereo_pair_pass::StereoPairFrameDraw::new(
            runtime,
            cache,
            resources,
            pair,
            retirements,
        )
    }

    fn entry(&mut self, view: &Arc<ViewEnvironmentToken>) -> usize {
        let found = self
            .views
            .iter()
            .position(|entry| std::ptr::eq(entry.view.as_ptr(), Arc::as_ptr(view)));
        found.unwrap_or_else(|| {
            self.views.push(ViewEnvironment {
                view: Arc::downgrade(view),
                cubes: vec![],
                pbr: None,
            });
            self.views.len() - 1
        })
    }

    /// The view's cube for `source`, uploaded or generated when its key is new; the view keeps
    /// at most the background and the IBL source.
    fn cube(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
        index: usize,
        source: &EnvironmentCubeSource,
    ) -> Result<Arc<EnvironmentCube>, RenderError> {
        let key = CubeKey::of(source);
        if let Some((_, cube)) = self.views[index]
            .cubes
            .iter()
            .find(|(cached, _)| *cached == key)
        {
            return Ok(cube.clone());
        }
        let pipelines = self.pipelines.created(runtime)?;
        let cube = match source {
            EnvironmentCubeSource::Image(pixels) => {
                EnvironmentCube::uploaded(runtime, &pipelines.layout, pixels)?
            }
            EnvironmentCubeSource::Skydome(params) => {
                EnvironmentCube::skydome(runtime, &pipelines.layout, &pipelines.skydome, params)?
            }
        };
        let cubes = &mut self.views[index].cubes;
        if cubes.len() >= 2 {
            cubes.remove(0);
        }
        cubes.push((key, cube.clone()));
        Ok(cube)
    }

    /// GlView::Render "process PBR environment" and updatePBREnvironment.
    fn pbr_environment(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
        index: usize,
        frame: &FramePbrEnvironment,
    ) -> Result<Arc<PbrEnvironment>, RenderError> {
        let size = PbrEnvironmentSize::new(frame.pow2_size, frame.spec_levels);
        let current = self.views[index].pbr.clone();
        let resized = current.as_ref().is_some_and(|env| env.size() != size);
        // SizesAreDifferent releases the environment and raises myPBREnvRequest; a new
        // (NONEXISTENT) environment without a request stays cleared.
        let environment = match (current.filter(|_| !resized), frame.request || resized) {
            (_, true) => self.updated_pbr_environment(runtime, index, size, frame)?,
            (Some(environment), false) => environment,
            (None, false) => PbrEnvironment::cleared(runtime, size)?,
        };
        self.views[index].pbr = Some(environment.clone());
        Ok(environment)
    }

    /// Bake from the IBL source, or clear without one.
    fn updated_pbr_environment(
        &mut self,
        runtime: &Arc<VulkanRuntime>,
        index: usize,
        size: PbrEnvironmentSize,
        frame: &FramePbrEnvironment,
    ) -> Result<Arc<PbrEnvironment>, RenderError> {
        let Some(source) = &frame.source else {
            return PbrEnvironment::cleared(runtime, size);
        };
        let cube = self.cube(runtime, index, source)?;
        let pipelines = self.pipelines.created(runtime)?;
        PbrEnvironment::baked(
            runtime,
            size,
            &pipelines.baking,
            &cube,
            source.coefficients(),
            frame.samples,
        )
    }
}
