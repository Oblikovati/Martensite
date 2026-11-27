//! Device injection keeps viewer tests independent from GPU availability (#633).
use crate::device_backend::{
    CapturePlanes, RenderDeferredFrame, RenderDeviceBackend, RenderGeometryBatch, RenderHiZHistory,
    RenderWindowPresenter, RenderedFrame,
};
use crate::frame_work_ledger::FrameWorkLedger;
use crate::{FrameCapture, RenderBackend, RenderError, RenderFrame};
use martensite_tkernel::standard::SharedState;
use std::sync::Arc;

/// Shared handle to a driver's active backend; `driver.render_device()`.
pub type SharedRenderDevice = Arc<SharedState<DriverBackend>>;

/// Active backend behind a [`crate::Graphic3dRenderDriver`].
pub enum DriverBackend {
    /// Concrete GPU backend (`VulkanRenderer`, `WebGpuRenderer`, etc.).
    Device(Box<dyn RenderDeviceBackend>),
    /// Injected test/recording backend.
    Injected(Box<dyn RenderBackend + Send + Sync>),
}

impl DriverBackend {
    /// Access the concrete GPU device backend, or `None` for an injected test backend.
    pub fn as_device(&self) -> Option<&dyn RenderDeviceBackend> {
        match self {
            Self::Device(device) => Some(device.as_ref()),
            Self::Injected(_) => None,
        }
    }

    /// Mutably access the concrete GPU device backend, or `None` for an injected test backend.
    pub fn as_device_mut(&mut self) -> Option<&mut dyn RenderDeviceBackend> {
        match self {
            Self::Device(device) => Some(device.as_mut()),
            Self::Injected(_) => None,
        }
    }

    pub fn prepare_batch(
        &self,
        draws: Vec<crate::PrimitiveDraw>,
    ) -> Result<Option<Arc<dyn RenderGeometryBatch>>, RenderError> {
        match self {
            Self::Device(renderer) => renderer.prepare_batch(draws),
            Self::Injected(_) => Ok(None),
        }
    }

    pub fn create_hiz_history(&self) -> Option<Arc<dyn RenderHiZHistory>> {
        match self {
            Self::Device(renderer) => renderer.create_hiz_history(),
            Self::Injected(_) => None,
        }
    }

    pub fn supports_hiz(&self) -> bool {
        matches!(self, Self::Device(renderer) if renderer.supports_hiz())
    }

    pub fn supports_placement_compaction(&self) -> bool {
        matches!(self, Self::Device(renderer) if renderer.supports_placement_compaction())
    }

    pub fn supports_placements(
        &self,
        templates: &[crate::PrimitiveDraw],
        placement_count: usize,
    ) -> Result<bool, RenderError> {
        match self {
            Self::Device(renderer) => renderer.supports_placements(templates, placement_count),
            Self::Injected(_) => Ok(false),
        }
    }

    pub fn supports_phong_batching(&self) -> bool {
        matches!(self, Self::Device(renderer) if renderer.supports_phong_batching())
    }

    pub fn supports_mesh_lod(&self) -> bool {
        match self {
            Self::Device(renderer) => renderer.supports_mesh_lod(),
            Self::Injected(_) => true,
        }
    }

    pub fn allows_main_copy(&self) -> bool {
        matches!(self, Self::Device(renderer) if renderer.allows_main_copy())
    }

    /// Selected device capacities; injected backends have no GPU device.
    pub fn device_limits(&self) -> Option<crate::RenderDeviceLimits> {
        match self {
            Self::Device(device) => device.device_limits(),
            Self::Injected(_) => None,
        }
    }

    /// Device facts of the diagnostic dictionary; injected backends have no GPU device.
    pub fn diagnostic_facts(&self) -> Option<crate::view_diagnostics::RenderDiagnosticFacts> {
        match self {
            Self::Device(device) => device.diagnostic_facts(),
            Self::Injected(_) => None,
        }
    }

    /// Image bytes for frame statistics; injected backends own no device images.
    pub fn image_memory(&self) -> crate::RenderImageMemory {
        match self {
            Self::Device(device) => device.image_memory(),
            Self::Injected(_) => crate::RenderImageMemory::default(),
        }
    }

    /// The work counters of the GPU device; injected backends count nothing.
    pub fn work_ledger(&self) -> Option<Arc<FrameWorkLedger>> {
        match self {
            Self::Device(device) => device.work_ledger(),
            Self::Injected(_) => None,
        }
    }

    /// Render a frame synchronously into host pixels.
    pub fn render(&mut self, frame: &RenderFrame) -> Result<FrameCapture, RenderError> {
        match self {
            Self::Device(device) => device.render_frame(frame),
            Self::Injected(device) => device.render(frame),
        }
    }

    /// Render a view frame; GPU backends may keep images on the device until capture.
    pub fn render_view_frame(&mut self, frame: &RenderFrame) -> Result<RenderedFrame, RenderError> {
        match self {
            Self::Device(device) => device.render_deferred(frame),
            Self::Injected(device) => device.render(frame).map(RenderedFrame::Ready),
        }
    }

    /// Wait for every frame a redraw left in flight on the GPU.
    pub fn finish_frames(&mut self) -> Result<(), RenderError> {
        match self {
            Self::Device(device) => device.finish_frames(),
            Self::Injected(_) => Ok(()),
        }
    }

    /// Write the RGB rows of a deferred frame through `write_row`.
    pub fn read_frame_rgb(
        &mut self,
        readback: &dyn RenderDeferredFrame,
        flip: bool,
        write_row: &mut dyn FnMut(usize, &[u8]) -> bool,
    ) -> Result<bool, RenderError> {
        match self {
            Self::Device(device) => device.read_frame_rgb(readback, flip, write_row),
            Self::Injected(_) => Err(crate::error::rejected(
                "injected backend",
                "the GPU device that rendered the frame",
            )),
        }
    }

    /// Copy the chosen images of a deferred frame to the host.
    pub fn read_frame(
        &mut self,
        readback: &dyn RenderDeferredFrame,
        planes: CapturePlanes,
    ) -> Result<FrameCapture, RenderError> {
        match self {
            Self::Device(device) => device.read_frame(readback, planes),
            Self::Injected(_) => Err(crate::error::rejected(
                "injected backend",
                "the GPU device that rendered the frame",
            )),
        }
    }

    /// Testing only: raw native instance handle, `None` for injected backends.
    pub fn raw_instance(&self) -> Option<u64> {
        match self {
            Self::Device(device) => device.raw_instance(),
            Self::Injected(_) => None,
        }
    }

    /// Forward actual validation coverage; refuses injected backends.
    pub fn validation_errors(
        &self,
    ) -> Result<Vec<String>, crate::RenderValidationObservationFailure> {
        match self {
            Self::Device(device) => device.validation_errors(),
            Self::Injected(_) => Err(crate::RenderValidationObservationFailure::unavailable()),
        }
    }

    /// Return selected GPU driver provenance; injected backends have none.
    pub fn driver_identity(&self) -> Option<crate::RenderDriverIdentity> {
        match self {
            Self::Device(device) => device.driver_identity(),
            Self::Injected(_) => None,
        }
    }

    /// Query the GPU memory report or return an explicit unavailable report.
    pub fn memory_report(
        &self,
    ) -> Result<crate::RenderDeviceMemoryReport, crate::RenderMemoryQueryError> {
        match self {
            Self::Device(device) => device.memory_report(),
            Self::Injected(_) => Ok(crate::RenderDeviceMemoryReport::unavailable()),
        }
    }

    /// Inspect effective raster policy; injected backends return `None`.
    pub fn raster_policy(&self) -> Option<crate::RenderRasterPolicy> {
        match self {
            Self::Device(device) => device.raster_policy(),
            Self::Injected(_) => None,
        }
    }

    /// Inspect effective coordinate convention; injected backends return `None`.
    pub fn coordinate_convention(&self) -> Option<crate::RenderCoordinateConvention> {
        match self {
            Self::Device(device) => device.coordinate_convention(),
            Self::Injected(_) => None,
        }
    }

    /// Human-readable backend name.
    pub fn name(&self) -> &str {
        match self {
            Self::Device(device) => device.device_name(),
            Self::Injected(_) => "injected Martensite render backend",
        }
    }

    /// Create a window presenter bound to `window`.
    pub fn window_presenter(
        &self,
        window: &martensite_tkernel::standard::Handle<
            dyn martensite_tkservice::aspect::AspectWindow,
        >,
    ) -> Result<Box<dyn RenderWindowPresenter>, RenderError> {
        match self {
            Self::Device(device) => device.window_presenter(window),
            Self::Injected(_) => Err(crate::error::rejected(
                "injected backend",
                "a GPU device for window presentation",
            )),
        }
    }

    /// Render and present a frame to `target`.
    pub fn render_window(
        &mut self,
        frame: &RenderFrame,
        target: &mut dyn RenderWindowPresenter,
        frame_queue: crate::driver_options::WindowFrameQueue,
        policy: crate::RenderGuiSurfacePolicy,
    ) -> Result<RenderedFrame, RenderError> {
        match self {
            Self::Device(device) => device.render_window(frame, target, frame_queue, policy),
            Self::Injected(_) => Err(crate::error::rejected(
                "injected backend",
                "a GPU window presenter",
            )),
        }
    }
}
