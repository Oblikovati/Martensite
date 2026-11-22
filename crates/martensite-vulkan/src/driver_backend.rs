//! Device injection keeps viewer tests independent from Vulkan availability.
use crate::{FrameCapture, RenderBackend, RenderError, RenderFrame, VulkanRenderer};
use martensite_tkernel::standard::SharedState;
use std::sync::Arc;
pub(crate) type SharedRenderDevice = Arc<SharedState<DriverBackend>>;
/// A redraw's result: pixels already on the host, or device images a capture reads (#594).
pub(crate) enum RenderedFrame {
    Ready(FrameCapture),
    Deferred(crate::backend::FrameReadback),
}
pub(crate) enum DriverBackend {
    Vulkan(Box<VulkanRenderer>),
    Injected(Box<dyn RenderBackend + Send + Sync>),
}
impl DriverBackend {
    pub(crate) fn prepare_batch(
        &self,
        draws: Vec<crate::PrimitiveDraw>,
    ) -> Result<Option<crate::backend::GeometryBatch>, RenderError> {
        match self {
            Self::Vulkan(renderer) => renderer.prepare_batch(draws),
            Self::Injected(_) => Ok(None),
        }
    }

    pub(crate) fn supports_phong_batching(&self) -> bool {
        matches!(self, Self::Vulkan(renderer) if renderer.supports_phong_batching())
    }

    pub(super) fn allows_main_copy(&self) -> bool {
        matches!(self, Self::Vulkan(renderer) if renderer.allows_main_copy())
    }

    /// Selected device capacities; injected backends have no Vulkan device.
    pub fn device_limits(&self) -> Option<crate::RenderDeviceLimits> {
        match self {
            Self::Vulkan(device) => Some(device.device_limits()),
            Self::Injected(_) => None,
        }
    }
    /// Device facts of the diagnostic dictionary; injected backends have no Vulkan device.
    pub fn diagnostic_facts(&self) -> Option<crate::view_diagnostics::RenderDiagnosticFacts> {
        match self {
            Self::Vulkan(device) => Some(device.diagnostic_facts()),
            Self::Injected(_) => None,
        }
    }
    /// Image bytes for frame statistics; injected backends own no device images.
    /// Example: `backend.image_memory().sampled_image_bytes`.
    pub fn image_memory(&self) -> crate::RenderImageMemory {
        match self {
            Self::Vulkan(device) => device.image_memory(),
            Self::Injected(_) => crate::RenderImageMemory::default(),
        }
    }
    /// The work counters of the Vulkan device; injected backends count nothing.
    /// Example: `backend.work_ledger().map(|ledger| ledger.totals())`.
    pub fn work_ledger(&self) -> Option<Arc<crate::backend::FrameWorkLedger>> {
        match self {
            Self::Vulkan(device) => Some(device.work_ledger()),
            Self::Injected(_) => None,
        }
    }
    pub fn render(&mut self, frame: &RenderFrame) -> Result<FrameCapture, RenderError> {
        match self {
            Self::Vulkan(device) => device.render(frame),
            Self::Injected(device) => device.render(frame),
        }
    }
    /// Render a view frame; a Vulkan frame keeps its images on the device until a capture
    /// reads them. Example: `backend.render_view_frame(&frame)?`.
    pub fn render_view_frame(&mut self, frame: &RenderFrame) -> Result<RenderedFrame, RenderError> {
        match self {
            Self::Vulkan(device) => device.render_deferred(frame).map(RenderedFrame::Deferred),
            Self::Injected(device) => device.render(frame).map(RenderedFrame::Ready),
        }
    }
    /// Wait for every frame a Vulkan redraw left on the GPU; injected backends draw at once.
    /// Example: `backend.finish_frames()?`.
    pub fn finish_frames(&mut self) -> Result<(), RenderError> {
        match self {
            Self::Vulkan(device) => device.finish_frames(),
            Self::Injected(_) => Ok(()),
        }
    }
    /// Copy a deferred frame's images to the host; `backend.read_frame(&readback)?`.
    pub fn read_frame(
        &mut self,
        readback: &crate::backend::FrameReadback,
    ) -> Result<FrameCapture, RenderError> {
        match self {
            Self::Vulkan(device) => device.read_frame(readback),
            Self::Injected(_) => Err(crate::error::rejected(
                "injected backend",
                "the Vulkan device that rendered the frame",
            )),
        }
    }
    /// Testing only: the raw `VkInstance` handle, None for injected backends;
    /// `backend.raw_instance()`.
    pub fn raw_instance(&self) -> Option<u64> {
        match self {
            Self::Vulkan(device) => Some(device.raw_instance()),
            Self::Injected(_) => None,
        }
    }
    /// Forward actual coverage; `backend.validation_errors()?` refuses injected backends instead of inventing clean evidence.
    pub fn validation_errors(
        &self,
    ) -> Result<Vec<String>, crate::RenderValidationObservationFailure> {
        match self {
            Self::Vulkan(device) => device.validation_errors(),
            Self::Injected(_) => Err(crate::RenderValidationObservationFailure::unavailable()),
        }
    }
    /// Return selected Vulkan provenance; injected backends have no Vulkan identity.
    /// Example: `backend.driver_identity().is_none()`.
    pub fn driver_identity(&self) -> Option<crate::RenderDriverIdentity> {
        match self {
            Self::Vulkan(device) => Some(device.driver_identity()),
            Self::Injected(_) => None,
        }
    }
    /// Query the real backend or explicit unavailable provider; for example backend.memory_report().
    pub fn memory_report(
        &self,
    ) -> Result<crate::RenderDeviceMemoryReport, crate::RenderMemoryQueryError> {
        match self {
            Self::Vulkan(device) => device.memory_report(),
            Self::Injected(_) => Ok(crate::RenderDeviceMemoryReport::unavailable()),
        }
    }
    /// Inspect actual Vulkan policy; `backend.raster_policy()` never invents one for a fake.
    pub fn raster_policy(&self) -> Option<crate::RenderRasterPolicy> {
        match self {
            Self::Vulkan(device) => Some(device.raster_policy()),
            Self::Injected(_) => None,
        }
    }
    /// Inspect the real coordinate choice; `backend.coordinate_convention()` leaves fakes explicit.
    pub fn coordinate_convention(&self) -> Option<crate::RenderCoordinateConvention> {
        match self {
            Self::Vulkan(device) => Some(device.coordinate_convention()),
            Self::Injected(_) => None,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Self::Vulkan(device) => device.device_name(),
            Self::Injected(_) => "injected Martensite render backend",
        }
    }
}

impl DriverBackend {
    pub fn window_presenter(
        &self,
        window: &martensite_tkernel::standard::Handle<
            dyn martensite_tkservice::aspect::AspectWindow,
        >,
    ) -> Result<crate::backend::WindowPresenter, RenderError> {
        match self {
            Self::Vulkan(device) => device.window_presenter(window),
            Self::Injected(_) => Err(crate::error::rejected(
                "injected backend",
                "a Vulkan device for native window presentation",
            )),
        }
    }
    pub fn render_window(
        &mut self,
        frame: &RenderFrame,
        target: &mut crate::backend::WindowPresenter,
        vsync: bool,
        policy: crate::RenderGuiSurfacePolicy,
    ) -> Result<RenderedFrame, RenderError> {
        match self {
            Self::Vulkan(device) => device
                .render_window(frame, target, vsync, policy)
                .map(RenderedFrame::Deferred),
            Self::Injected(_) => Err(crate::error::rejected(
                "injected backend",
                "a Vulkan window presenter",
            )),
        }
    }
}
