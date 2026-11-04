//! Additive constructors keep public policy struct literals source-compatible.
use super::*;
use crate::{RenderCoordinateConvention, RenderLineWidthPolicy};

impl Graphic3dRenderDriver {
    /// Compose line raster widths independently of lighting, texture and coordinates.
    /// Example: `Self::with_line_width_configuration(options, policies, coordinates, RenderLineWidthPolicy::SinglePixelV1)?`.
    pub fn with_line_width_configuration(
        options: VulkanOptions,
        selection: RenderPipelinePolicies,
        coordinates: RenderCoordinateConvention,
        line_widths: RenderLineWidthPolicy,
    ) -> Result<Handle<Self>, RenderError> {
        let (renderer, limit) = validated_renderer(options, &selection, coordinates)?;
        Ok(Self::build(DriverBuildInputs {
            backend: DriverBackend::Vulkan(Box::new(renderer)),
            limit,
            text_source: Arc::new(crate::FreeTypeTextSource::default()),
            selection,
            line_widths,
        }))
    }

    /// Inject a device while retaining an explicit immutable line-width selection.
    /// Example: `Self::with_backend_and_line_width_configuration(device, 4096, fonts, policies, RenderLineWidthPolicy::SinglePixelV1)?`.
    pub fn with_backend_and_line_width_configuration(
        backend: impl RenderBackend + Send + Sync + 'static,
        limit: u32,
        fonts: Arc<dyn crate::TextLayoutSource>,
        selection: RenderPipelinePolicies,
        line_widths: RenderLineWidthPolicy,
    ) -> Result<Handle<Self>, RenderError> {
        validate_capture_limit(limit)?;
        Ok(Self::build(DriverBuildInputs {
            backend: DriverBackend::Injected(Box::new(backend)),
            limit,
            text_source: fonts,
            selection,
            line_widths,
        }))
    }

    /// Read the immutable line rasterization selection; `driver.line_width_policy()`.
    pub fn line_width_policy(&self) -> RenderLineWidthPolicy {
        self.line_widths
    }
}
