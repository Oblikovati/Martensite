//! Frame stages retain draw-before-OIT failure cleanup and ordered target construction.
use super::super::frame_targets::{FrameTargetPool, RasterTargetLease};
use super::super::geometry_transfer::GeometryTransfers;
use super::super::host_upload_pages::FrameUploadPages;
use super::super::immediate_main_record::{MainLayerImages, MainLayerSnapshot, MainRestoreSource};
use super::super::{
    fragment_alpha_plan::FragmentAlphaPlan, oit_frame::OitFramePass, oit_plan::OitResourcePlan,
    retained_submission::RetirementStore,
};
use super::super::{pipeline_variants::RasterPipelines, runtime::VulkanRuntime};
use super::{FrameDrawUploads, FrameResources, UploadedDraw, check_extent, upload_draw};
use crate::immediate_main::{ImmediateMainPlan, MainSeed};
use crate::{RenderError, RenderFrame};
use ash::vk;
use std::sync::Arc;
pub(super) struct FrameSetup<'a> {
    pub runtime: &'a Arc<VulkanRuntime>,
    pub frame: &'a RenderFrame,
    pub pipeline: &'a RasterPipelines,
    pub retirements: &'a RetirementStore,
}
pub(super) struct PreparedFramePlan {
    fragment_alpha: FragmentAlphaPlan,
    stencil: crate::stencil_plan::StencilPlan,
    dimensions: FrameDimensions,
    raster: OitResourcePlan,
    restore: Option<PreparedMainRestore>,
    first_draw: usize,
    can_seed_main: bool,
}
struct PreparedMainRestore {
    start: usize,
    source: MainRestoreSource,
    destination: RasterTargetLease,
}
struct FrameDimensions {
    extent: vk::Extent2D,
    attachment: vk::Extent2D,
    bytes: u64,
}
pub(super) struct PreparedFrameUploads {
    draws: Vec<UploadedDraw>,
    meshlets: super::super::meshlet_draw::FrameMeshlets,
    geometry_transfers: GeometryTransfers,
    oit: Option<OitFramePass>,
}
impl FrameSetup<'_> {
    /// Plan before allocating targets; `setup.prepare(alpha, stencil)` preserves pruning order.
    pub(super) fn prepare(
        &self,
        alpha: crate::legacy_fragment_alpha::LegacyFragmentAlpha,
        initial_stencil: crate::RenderStencilState,
    ) -> Result<PreparedFramePlan, RenderError> {
        self.runtime.lifecycle.admit("allocate frame resources")?;
        super::super::v4_depth_admission::validate_frame(
            self.runtime.triangle_coverage,
            self.runtime.depth_bias_behavior,
            self.frame,
        )?;
        #[cfg(feature = "lit-only-command-diagnostics")]
        self.runtime
            .lit_only_commands
            .validate_frame(self.frame, initial_stencil)?;
        let stencil = crate::stencil_plan::StencilPlan::new(self.frame, initial_stencil);
        let clean_alpha = alpha == Default::default();
        let fragment_alpha = FragmentAlphaPlan::new(self.frame, alpha);
        let dimensions = self.prepare_dimensions()?;
        let raster = self.raster_plan();
        let (can_seed_main, restore) =
            self.prepare_immediate(clean_alpha, raster.samples, &dimensions)?;
        let first_draw = restore
            .as_ref()
            .map_or(0, |restore| self.first_immediate_draw(restore.start));
        Ok(PreparedFramePlan {
            stencil,
            fragment_alpha,
            dimensions,
            raster,
            restore,
            first_draw,
            can_seed_main,
        })
    }
    fn first_immediate_draw(&self, start: usize) -> usize {
        self.frame
            .layers
            .get(start)
            .map_or(self.frame.draws.len(), |layer| layer.draw_range.start)
    }
    fn prepare_immediate(
        &self,
        clean_alpha: bool,
        samples: vk::SampleCountFlags,
        dimensions: &FrameDimensions,
    ) -> Result<(bool, Option<PreparedMainRestore>), RenderError> {
        let Some(plan) = self.frame.immediate_main.as_ref().filter(|_| {
            !self.pipeline.stencil
                && clean_alpha
                && samples == vk::SampleCountFlags::TYPE_1
                && FrameTargetPool::can_reserve_main(dimensions.bytes)
        }) else {
            return self.refuse_elided().map(|()| (false, None));
        };
        let pool = &self.pipeline.frame_targets;
        let Some((start, ticket, source)) = plan.restore().and_then(|(start, ticket)| {
            pool.pin_main(ticket).map(|source| (start, ticket, source))
        }) else {
            return self.refuse_elided().map(|()| (true, None));
        };
        if let Some(destination) = self.optional_main_targets(dimensions.attachment, false)? {
            let restore = self.pipeline.main_restore.get(self.runtime)?;
            return Ok((
                false,
                Some(PreparedMainRestore {
                    start,
                    source: source.into_source(restore)?,
                    destination,
                }),
            ));
        }
        pool.revoke_main(ticket);
        self.refuse_elided().map(|()| (false, None))
    }
    /// An elided frame has no main draws to fall back on (immediate_elision.rs).
    fn refuse_elided(&self) -> Result<(), RenderError> {
        let elided = self
            .frame
            .immediate_main
            .as_ref()
            .is_some_and(ImmediateMainPlan::is_elided);
        match elided {
            true => Err(RenderError::main_restore_unavailable()),
            false => Ok(()),
        }
    }
    fn prepare_dimensions(&self) -> Result<FrameDimensions, RenderError> {
        self.pipeline.surface_images.prune()?;
        check_extent(self.runtime, self.frame)?;
        let extent = vk::Extent2D {
            width: self.frame.width,
            height: self.frame.height,
        };
        let attachment = self
            .runtime
            .attachment_coordinates
            .attachment_extent(extent);
        let bytes = u64::from(attachment.width) * u64::from(attachment.height) * 4;
        Ok(FrameDimensions {
            extent,
            attachment,
            bytes,
        })
    }
    fn raster_plan(&self) -> OitResourcePlan {
        super::super::frame_raster_plan::frame_raster_plan(
            self.runtime,
            self.frame,
            self.pipeline.stencil,
        )
    }

    /// Upload ordered draws after OIT setup; `setup.upload(&plan)` retains prior cleanup order.
    pub(super) fn upload(
        &self,
        plan: &PreparedFramePlan,
    ) -> Result<PreparedFrameUploads, RenderError> {
        let oit = self.prepare_oit(plan)?;
        self.pipeline
            .surface_images
            .observe_live_fixed_function(self.frame.raster.fixed_function);
        let compact_vertices = self.runtime.mesh_vertex_behavior
            == crate::RenderMeshVertexBehavior::Quantized16OctV2
            && plan.raster.samples == vk::SampleCountFlags::TYPE_1
            && !self.frame.raster.alpha_writes
            && self.frame.geometry_batch.is_none()
            && !self.frame.draws[plan.first_draw..]
                .iter()
                .any(|draw| draw.instances.is_some());
        let mut uploads = FrameDrawUploads {
            compact_vertices,
            geometry_batch: self.frame.geometry_batch.as_ref(),
            geometry: GeometryTransfers::new(&self.pipeline.geometry_residency),
            storage: FrameUploadPages::storage(&self.pipeline.frame_storage_pages),
            dynamic_lighting: super::super::storage_descriptor::uses_lighting_pages(
                &self.frame.draws[plan.first_draw..],
            ),
        };
        let mut draws = self.upload_draws(plan, &mut uploads)?;
        self.select_alpha_writes(&mut draws)?;
        self.select_multisample(&mut draws, plan)?;
        self.upload_oit(&mut draws, oit.as_ref())?;
        self.upload_alpha(&mut draws, plan, oit.as_ref())?;
        self.form_table_runs(&mut draws, plan, &mut uploads)?;
        let meshlets = super::super::meshlet_draw::prepare(
            self.runtime,
            self.frame,
            self.pipeline,
            plan.raster.samples,
            &draws,
            plan.first_draw,
        )?;
        super::super::window_depth_upload::require_scene_viewports(
            self.runtime,
            &draws,
            plan.dimensions.extent,
        )?;
        #[cfg(feature = "lit-only-command-diagnostics")]
        self.runtime
            .lit_only_commands
            .validate_uploads(self.frame, &draws, plan.raster.samples)?;
        Ok(PreparedFrameUploads {
            draws,
            meshlets,
            geometry_transfers: uploads.geometry,
            oit,
        })
    }
    /// Attach the indirect runs of the plain lit surfaces to their first draws (draw_table.rs).
    fn form_table_runs(
        &self,
        draws: &mut [UploadedDraw],
        plan: &PreparedFramePlan,
        uploads: &mut FrameDrawUploads<'_>,
    ) -> Result<(), RenderError> {
        let frame = crate::backend::draw_table::TableFrame {
            runtime: self.runtime,
            pipelines: self.pipeline,
            sources: &self.frame.draws[plan.first_draw..],
            first_draw: plan.first_draw,
            stencil: &plan.stencil,
            layers: &self.frame.layers,
        };
        let upload = (
            self.runtime,
            self.pipeline,
            plan.dimensions.extent,
            plan.raster.samples,
            self.retirements,
        );
        let sources = &self.frame.draws[plan.first_draw..];
        let hiz_layer = self.hiz_layer(plan, draws.len());
        super::draw_instance_upload::attach_placement_runs(
            upload,
            (sources, hiz_layer),
            draws,
            uploads,
        )?;
        crate::backend::draw_table::form_runs(&frame, draws, &mut uploads.storage)
    }
    fn hiz_layer(&self, plan: &PreparedFramePlan, draws: usize) -> Option<&crate::RenderLayerSpan> {
        let [layer] = self.frame.layers.as_slice() else {
            return None;
        };
        (draws == 1
            && plan.first_draw == 0
            && plan.restore.is_none()
            && !plan.stencil.during[0].enabled
            && plan.fragment_alpha.during[0].is_none()
            && self.frame.shader_grid.is_none()
            && self.frame.environment.is_none()
            && self.frame.shadow_maps.is_none()
            && self.frame.raytrace.is_none()
            && self.frame.stereo_pair.is_none()
            && self.frame.colored_quad.is_none()
            && !self.frame.raster.alpha_writes)
            .then_some(layer)
    }

    fn upload_draws(
        &self,
        plan: &PreparedFramePlan,
        uploads: &mut FrameDrawUploads<'_>,
    ) -> Result<Vec<UploadedDraw>, RenderError> {
        let sources = &self.frame.draws[plan.first_draw..];
        // An exact capacity: frames in flight keep these records until their fence (#594).
        let mut draws = Vec::with_capacity(sources.len());
        for draw in sources {
            draws.push(upload_draw(
                self.runtime,
                draw,
                self.pipeline,
                plan.dimensions.extent,
                plan.raster.samples,
                self.retirements,
                uploads,
            )?);
        }
        Ok(draws)
    }
    fn upload_oit(
        &self,
        draws: &mut [UploadedDraw],
        oit: Option<&OitFramePass>,
    ) -> Result<(), RenderError> {
        if let Some(pass) = oit {
            super::super::oit_upload::upload_oit_variants(
                self.runtime,
                self.pipeline,
                draws,
                pass,
            )?;
        }
        Ok(())
    }
    fn prepare_oit(&self, plan: &PreparedFramePlan) -> Result<Option<OitFramePass>, RenderError> {
        let mut oit = OitFramePass::new(
            self.runtime,
            self.frame,
            self.pipeline,
            plan.dimensions.attachment,
            plan.raster,
        )?;
        if let Some(pass) = &mut oit {
            pass.prepare_alpha(&plan.fragment_alpha)?;
        }
        Ok(oit)
    }
    /// Frames with native buffersOpaqueAlpha off draw with alpha-writing pipelines.
    fn select_alpha_writes(&self, draws: &mut [UploadedDraw]) -> Result<(), RenderError> {
        if !self.frame.raster.alpha_writes {
            return Ok(());
        }
        for draw in draws {
            draw.pipeline = self.pipeline.alpha_write_twin(&draw.pipeline)?;
        }
        Ok(())
    }
    fn select_multisample(
        &self,
        draws: &mut [UploadedDraw],
        plan: &PreparedFramePlan,
    ) -> Result<(), RenderError> {
        for draw in draws {
            let coverage = self.frame.raster.alpha_to_coverage && draw.sample_alpha_cutoff;
            draw.pipeline = self.pipeline.oit.multisample(
                self.runtime,
                &draw.pipeline,
                plan.raster.samples,
                coverage,
            )?;
            if let Some(user) = &mut draw.user_program {
                let mut fixed = user.fixed;
                (fixed.samples, fixed.alpha_coverage) = (plan.raster.samples, coverage);
                let cache = &self.pipeline.user_programs;
                user.pipeline = cache.pipeline(self.runtime, &user.modules, fixed)?;
                user.fixed = fixed;
            }
        }
        Ok(())
    }
    fn upload_alpha(
        &self,
        draws: &mut [UploadedDraw],
        plan: &PreparedFramePlan,
        oit: Option<&OitFramePass>,
    ) -> Result<(), RenderError> {
        super::super::fragment_alpha_upload::upload_fragment_alpha(
            self.runtime,
            self.pipeline,
            draws,
            &plan.fragment_alpha.during[plan.first_draw..],
            oit,
            plan.raster.samples,
            self.frame.raster.alpha_to_coverage,
        )
    }
    /// Allocate targets in original literal order; `setup.allocate(plan, uploads)` moves draws last.
    pub(super) fn allocate(
        &self,
        plan: PreparedFramePlan,
        uploads: PreparedFrameUploads,
    ) -> Result<FrameResources, RenderError> {
        uploads.allocate(self, plan)
    }

    fn snapshot_main(
        &self,
        start: usize,
        dimensions: &FrameDimensions,
    ) -> Result<Option<MainLayerSnapshot>, RenderError> {
        let Some(targets) = self.optional_main_targets(dimensions.attachment, true)? else {
            return Ok(None);
        };
        Ok(targets.main_publication.map(|ticket| MainLayerSnapshot {
            start,
            ticket,
            images: MainLayerImages::new(targets.color, targets.depth),
        }))
    }
    fn optional_main_targets(
        &self,
        extent: vk::Extent2D,
        reserve: bool,
    ) -> Result<Option<RasterTargetLease>, RenderError> {
        use super::super::device_lifecycle::contains_device_loss;
        let targets = self.pipeline.frame_targets.raster(
            self.runtime,
            extent,
            false,
            vk::SampleCountFlags::TYPE_1,
            [true; 2],
            reserve,
        );
        match self.runtime.lifecycle.observe(targets) {
            Ok(targets) => Ok(Some(targets)),
            Err(error) if contains_device_loss(&error) => Err(error),
            // This is optional storage. No rendering or upload was submitted for it.
            Err(_) => Ok(None),
        }
    }
    fn frame_targets(
        &self,
        plan: &PreparedFramePlan,
        seed: Option<&MainSeed>,
    ) -> Result<(RasterTargetLease, Option<MainLayerSnapshot>), RenderError> {
        if let Some(start) = seed.and_then(|seed| seed.snapshot_start)
            && let Some(targets) = self.optional_main_targets(plan.dimensions.attachment, false)?
        {
            return Ok((targets, self.snapshot_main(start, &plan.dimensions)?));
        }
        let targets = self.pipeline.frame_targets.raster(
            self.runtime,
            plan.dimensions.attachment,
            self.pipeline.stencil,
            plan.raster.samples,
            [!self.frame.subview_blits.is_empty(), false],
            seed.is_some_and(|seed| seed.snapshot_start.is_none()),
        )?;
        Ok((targets, None))
    }
}

impl PreparedFrameUploads {
    fn allocate(
        self,
        setup: &FrameSetup<'_>,
        mut plan: PreparedFramePlan,
    ) -> Result<FrameResources, RenderError> {
        let (targets, immediate_source, main_snapshot) = match plan.restore.take() {
            Some(restore) => (
                restore.destination,
                Some((restore.start, restore.source)),
                None,
            ),
            None => {
                let seed = setup
                    .frame
                    .immediate_main
                    .as_ref()
                    .filter(|_| plan.can_seed_main)
                    .and_then(ImmediateMainPlan::seed);
                let (targets, snapshot) = setup.frame_targets(&plan, seed)?;
                (targets, None, snapshot)
            }
        };
        Ok(FrameResources {
            secondary_layers: super::super::secondary_layer_commands::FrameSecondaryLayers::new(
                setup.runtime,
            ),
            attachment_coordinates: setup.runtime.attachment_coordinates,
            fragment_alpha: plan.fragment_alpha,
            stencil: plan.stencil,
            stencil_clear_value: setup.frame.stencil_clear_value,
            multisample: targets.multisample,
            oit: self.oit,
            extent: plan.dimensions.extent,
            attachment_extent: plan.dimensions.attachment,
            color: targets.color,
            depth: targets.depth,
            immediate_source,
            main_publication: targets.main_publication,
            main_snapshot,
            draws: self.draws,
            meshlets: self.meshlets,
            submitted_draws: Default::default(),
            first_draw: plan.first_draw,
            geometry_transfers: self.geometry_transfers,
            grid: None,
            raytrace: None,
            background: None,
            subview_upload: None,
        })
    }
}

#[cfg(test)]
#[path = "clip_msaa_plan_device_tests.rs"]
mod clip_msaa_plan_device_tests;
