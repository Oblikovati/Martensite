//! Frame stages retain draw-before-OIT failure cleanup and ordered target construction.
use super::super::frame_targets::{FrameTargetPool, RasterTargetLease};
use super::super::geometry_transfer::GeometryTransfers;
use super::super::host_upload_pages::FrameUploadPages;
use super::super::immediate_main_record::{MainLayerImages, MainLayerSnapshot};
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
    source: MainLayerImages,
    destination: RasterTargetLease,
}
struct FrameDimensions {
    extent: vk::Extent2D,
    attachment: vk::Extent2D,
    bytes: u64,
}
pub(super) struct PreparedFrameUploads {
    draws: Vec<UploadedDraw>,
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
            return Ok((false, None));
        };
        let pool = &self.pipeline.frame_targets;
        let Some((start, ticket, source)) = plan.restore().and_then(|(start, ticket)| {
            pool.pin_main(ticket).map(|source| (start, ticket, source))
        }) else {
            return Ok((true, None));
        };
        if let Some(destination) = self.optional_main_targets(dimensions.attachment, false)? {
            return Ok((
                false,
                Some(PreparedMainRestore {
                    start,
                    source,
                    destination,
                }),
            ));
        }
        pool.revoke_main(ticket);
        Ok((false, None))
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
        let mut uploads = FrameDrawUploads {
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
            geometry_transfers: uploads.geometry,
            oit,
        })
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
                    .map(ImmediateMainPlan::seed);
                let (targets, snapshot) = setup.frame_targets(&plan, seed)?;
                (targets, None, snapshot)
            }
        };
        Ok(FrameResources {
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
