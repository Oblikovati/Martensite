//! Implements `martensite-render-core` backend traits for `VulkanRenderer` and `VulkanOptions` (#633).
use super::frame::GeometryBatch;
use super::frame_readback::FrameReadback;
use super::placement_hiz_history::HiZHistory;
use super::window_surface::WindowPresenter;
use super::{VulkanOptions, VulkanRenderer};
use crate::driver_options::WindowFrameQueue;
use crate::view_diagnostics::RenderDiagnosticFacts;
use crate::{
    CapturePlanes, FrameCapture, FrameWorkLedger, PrimitiveDraw, RenderAssemblyStreamingProgress,
    RenderBehaviorConfiguration, RenderCoordinateConvention, RenderDeferredFrame,
    RenderDeviceBackend, RenderDeviceLimits, RenderDeviceMemoryReport, RenderDeviceOptions,
    RenderDriverIdentity, RenderError, RenderFrame, RenderGeometryBatch, RenderGuiSurfacePolicy,
    RenderHiZHistory, RenderImageMemory, RenderLineClippingBehavior, RenderMemoryQueryError,
    RenderRasterPolicy, RenderUserProgramLibrary, RenderValidationObservationFailure,
    RenderWindowPresenter, RenderedFrame, error::rejected,
};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectWindow;
use std::any::Any;
use std::sync::Arc;

impl RenderGeometryBatch for GeometryBatch {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
        self
    }
}

impl RenderHiZHistory for HiZHistory {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync> {
        self
    }
}

impl RenderDeferredFrame for FrameReadback {
    fn logical_size(&self) -> [u32; 2] {
        FrameReadback::logical_size(self)
    }

    fn assembly_streaming(&self) -> Option<RenderAssemblyStreamingProgress> {
        self.assembly_streaming
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl RenderWindowPresenter for WindowPresenter {
    fn observed_extent(&self) -> Result<[u32; 2], RenderError> {
        WindowPresenter::observed_extent(self)
    }

    fn cancel_assembly_streaming(&mut self) {
        WindowPresenter::cancel_assembly_streaming(self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl RenderDeviceOptions for VulkanOptions {
    fn create_device(
        self,
        raster: RenderRasterPolicy,
        coordinates: RenderCoordinateConvention,
        clipping: RenderLineClippingBehavior,
        configuration: RenderBehaviorConfiguration,
    ) -> Result<Box<dyn RenderDeviceBackend>, RenderError> {
        let renderer = VulkanRenderer::new_with_render_behavior(
            self,
            raster,
            coordinates,
            clipping,
            configuration,
        )?;
        Ok(Box::new(renderer))
    }

    #[cfg(feature = "vertex-output-diagnostics")]
    fn create_device_with_vertex_output(
        self,
        raster: RenderRasterPolicy,
        coordinates: RenderCoordinateConvention,
        clipping: RenderLineClippingBehavior,
        configuration: RenderBehaviorConfiguration,
        request: crate::VertexOutputRequest,
    ) -> Result<Box<dyn RenderDeviceBackend>, RenderError> {
        let renderer = VulkanRenderer::new_with_vertex_output_behavior(
            self,
            raster,
            coordinates,
            clipping,
            configuration,
            request,
        )?;
        Ok(Box::new(renderer))
    }
}

impl RenderDeviceBackend for VulkanRenderer {
    fn capture_limit(&self) -> u32 {
        VulkanRenderer::capture_limit(self)
    }

    fn register_builtin_user_programs(
        &self,
        library: &RenderUserProgramLibrary,
    ) -> Result<(), RenderError> {
        crate::vulkan_user_program::register_samples(library)
    }

    fn prepare_batch(
        &self,
        draws: Vec<PrimitiveDraw>,
    ) -> Result<Option<Arc<dyn RenderGeometryBatch>>, RenderError> {
        VulkanRenderer::prepare_batch(self, draws)
            .map(|batch| batch.map(|b| Arc::new(b) as Arc<dyn RenderGeometryBatch>))
    }

    fn create_hiz_history(&self) -> Option<Arc<dyn RenderHiZHistory>> {
        Some(Arc::new(HiZHistory::default()))
    }

    fn supports_hiz(&self) -> bool {
        VulkanRenderer::supports_hiz(self)
    }

    fn supports_placement_compaction(&self) -> bool {
        VulkanRenderer::supports_placement_compaction(self)
    }

    fn supports_placements(
        &self,
        templates: &[PrimitiveDraw],
        placement_count: usize,
    ) -> Result<bool, RenderError> {
        VulkanRenderer::supports_placements(self, templates, placement_count)
    }

    fn supports_phong_batching(&self) -> bool {
        VulkanRenderer::supports_phong_batching(self)
    }

    fn supports_mesh_lod(&self) -> bool {
        VulkanRenderer::supports_mesh_lod(self)
    }

    fn allows_main_copy(&self) -> bool {
        VulkanRenderer::allows_main_copy(self)
    }

    fn device_limits(&self) -> Option<RenderDeviceLimits> {
        Some(VulkanRenderer::device_limits(self))
    }

    fn diagnostic_facts(&self) -> Option<RenderDiagnosticFacts> {
        Some(VulkanRenderer::diagnostic_facts(self))
    }

    fn image_memory(&self) -> RenderImageMemory {
        VulkanRenderer::image_memory(self)
    }

    fn work_ledger(&self) -> Option<Arc<FrameWorkLedger>> {
        Some(VulkanRenderer::work_ledger(self))
    }

    fn render_frame(&mut self, frame: &RenderFrame) -> Result<FrameCapture, RenderError> {
        <Self as crate::RenderBackend>::render(self, frame)
    }

    fn render_deferred(&mut self, frame: &RenderFrame) -> Result<RenderedFrame, RenderError> {
        VulkanRenderer::render_deferred(self, frame)
            .map(|readback| RenderedFrame::Deferred(Box::new(readback)))
    }

    fn finish_frames(&mut self) -> Result<(), RenderError> {
        VulkanRenderer::finish_frames(self)
    }

    fn read_frame_rgb(
        &mut self,
        frame: &dyn RenderDeferredFrame,
        flip: bool,
        write_row: &mut dyn FnMut(usize, &[u8]) -> bool,
    ) -> Result<bool, RenderError> {
        let Some(readback) = frame.as_any().downcast_ref::<FrameReadback>() else {
            return Err(rejected(
                "foreign deferred frame",
                "a Vulkan FrameReadback from this device",
            ));
        };
        VulkanRenderer::read_frame_rgb(self, readback, flip, write_row)
    }

    fn read_frame(
        &mut self,
        frame: &dyn RenderDeferredFrame,
        planes: CapturePlanes,
    ) -> Result<FrameCapture, RenderError> {
        let Some(readback) = frame.as_any().downcast_ref::<FrameReadback>() else {
            return Err(rejected(
                "foreign deferred frame",
                "a Vulkan FrameReadback from this device",
            ));
        };
        VulkanRenderer::read_frame_planes(self, readback, planes)
    }

    fn raw_instance(&self) -> Option<u64> {
        Some(VulkanRenderer::raw_instance(self))
    }

    fn validation_errors(&self) -> Result<Vec<String>, RenderValidationObservationFailure> {
        VulkanRenderer::validation_errors(self)
    }

    fn driver_identity(&self) -> Option<RenderDriverIdentity> {
        Some(VulkanRenderer::driver_identity(self))
    }

    fn memory_report(&self) -> Result<RenderDeviceMemoryReport, RenderMemoryQueryError> {
        VulkanRenderer::memory_report(self)
    }

    fn raster_policy(&self) -> Option<RenderRasterPolicy> {
        Some(VulkanRenderer::raster_policy(self))
    }

    fn coordinate_convention(&self) -> Option<RenderCoordinateConvention> {
        Some(VulkanRenderer::coordinate_convention(self))
    }

    fn device_name(&self) -> &str {
        VulkanRenderer::device_name(self)
    }

    fn window_presenter(
        &self,
        window: &Handle<dyn AspectWindow>,
    ) -> Result<Box<dyn RenderWindowPresenter>, RenderError> {
        VulkanRenderer::window_presenter(self, window)
            .map(|presenter| Box::new(presenter) as Box<dyn RenderWindowPresenter>)
    }

    fn render_window(
        &mut self,
        frame: &RenderFrame,
        target: &mut dyn RenderWindowPresenter,
        frame_queue: WindowFrameQueue,
        policy: RenderGuiSurfacePolicy,
    ) -> Result<RenderedFrame, RenderError> {
        let Some(presenter) = target.as_any_mut().downcast_mut::<WindowPresenter>() else {
            return Err(rejected(
                "foreign window presenter",
                "a Vulkan WindowPresenter from this device",
            ));
        };
        VulkanRenderer::render_window(self, frame, presenter, frame_queue, policy)
            .map(|readback| RenderedFrame::Deferred(Box::new(readback)))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
