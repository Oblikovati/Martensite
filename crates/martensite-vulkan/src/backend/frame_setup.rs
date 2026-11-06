//! Frame stages retain draw-before-OIT failure cleanup and ordered target construction.
use super::super::{
    buffer::DeviceBuffer, image::TargetImage, memory_selection::HostBufferAccess,
    pipeline_variants::RasterPipelines, runtime::VulkanRuntime,
};
use super::super::{
    fragment_alpha_plan::FragmentAlphaPlan, oit_frame::OitFramePass, oit_plan::OitResourcePlan,
    retained_submission::RetirementStore,
};
use super::{FrameResources, UploadedDraw, check_extent, upload_draw};
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
}
struct FrameDimensions {
    extent: vk::Extent2D,
    attachment: vk::Extent2D,
    bytes: u64,
}
pub(super) struct PreparedFrameUploads {
    draws: Vec<UploadedDraw>,
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
        let fragment_alpha = FragmentAlphaPlan::new(self.frame, alpha);
        let dimensions = self.prepare_dimensions()?;
        let raster = self.raster_plan();
        Ok(PreparedFramePlan {
            stencil,
            fragment_alpha,
            dimensions,
            raster,
        })
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
        let mut draws = self.upload_draws(plan.dimensions.extent, plan.raster.samples)?;
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
        Ok(PreparedFrameUploads { draws, oit })
    }
    fn upload_draws(
        &self,
        extent: vk::Extent2D,
        samples: vk::SampleCountFlags,
    ) -> Result<Vec<UploadedDraw>, RenderError> {
        self.frame
            .draws
            .iter()
            .map(|draw| {
                upload_draw(
                    self.runtime,
                    draw,
                    self.pipeline,
                    extent,
                    samples,
                    self.retirements,
                )
            })
            .collect()
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
            &plan.fragment_alpha,
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

    fn multisample(
        &self,
        extent: vk::Extent2D,
        samples: vk::SampleCountFlags,
    ) -> Result<Option<super::super::multisample_targets::MultisampleTargets>, RenderError> {
        super::super::multisample_targets::MultisampleTargets::new(
            self.runtime,
            extent,
            self.pipeline.stencil,
            samples,
        )
    }
    /// The single-sample colour target; a frame with subview copies also receives transfers.
    fn color_target(&self, extent: vk::Extent2D) -> Result<TargetImage, RenderError> {
        let (format, mut usage, aspect) =
            super::super::image::attachment_properties(self.runtime.depth.formats, false, false);
        if !self.frame.subview_blits.is_empty() {
            usage |= vk::ImageUsageFlags::TRANSFER_DST;
        }
        let samples = vk::SampleCountFlags::TYPE_1;
        let spec = super::super::image::AttachmentSpec {
            format,
            usage,
            aspect,
            samples,
        };
        TargetImage::with_attachment(self.runtime, extent, spec)
    }
    fn depth_target(&self, extent: vk::Extent2D) -> Result<TargetImage, RenderError> {
        // Stencil selection belongs to the existing pipeline, not a caller-specific target policy.
        TargetImage::new(self.runtime, extent, true, self.pipeline.stencil)
    }
    fn readback(&self, bytes: u64) -> Result<DeviceBuffer, RenderError> {
        DeviceBuffer::new(
            self.runtime,
            bytes,
            vk::BufferUsageFlags::TRANSFER_DST,
            HostBufferAccess::Readback,
        )
    }
}

impl PreparedFrameUploads {
    fn allocate(
        self,
        setup: &FrameSetup<'_>,
        plan: PreparedFramePlan,
    ) -> Result<FrameResources, RenderError> {
        Ok(FrameResources {
            attachment_coordinates: setup.runtime.attachment_coordinates,
            fragment_alpha: plan.fragment_alpha,
            stencil: plan.stencil,
            stencil_clear_value: setup.frame.stencil_clear_value,
            multisample: setup.multisample(plan.dimensions.attachment, plan.raster.samples)?,
            oit: self.oit,
            extent: plan.dimensions.extent,
            attachment_extent: plan.dimensions.attachment,
            color: setup.color_target(plan.dimensions.attachment)?,
            depth: setup.depth_target(plan.dimensions.attachment)?,
            color_read: setup.readback(plan.dimensions.bytes)?,
            depth_read: setup.readback(plan.dimensions.bytes)?,
            draws: self.draws,
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
