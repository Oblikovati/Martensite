//! Device injection keeps viewer tests independent from Vulkan availability.
use crate::{FrameCapture, RenderBackend, RenderError, RenderFrame, VulkanRenderer};
use martensite_tkernel::standard::SharedState;
use std::sync::Arc;
pub(crate) type SharedRenderDevice = Arc<SharedState<DriverBackend>>;
pub(crate) enum DriverBackend {
    Vulkan(Box<VulkanRenderer>),
    Injected(Box<dyn RenderBackend + Send + Sync>),
}
impl DriverBackend {
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
    pub fn render(&mut self, frame: &RenderFrame) -> Result<FrameCapture, RenderError> {
        match self {
            Self::Vulkan(device) => device.render(frame),
            Self::Injected(device) => device.render(frame),
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
    ) -> Result<FrameCapture, RenderError> {
        match self {
            Self::Vulkan(device) => device.render_window(frame, target, vsync, policy),
            Self::Injected(_) => Err(crate::error::rejected(
                "injected backend",
                "a Vulkan window presenter",
            )),
        }
    }
}
