//! Additive opt-in leaves existing option and pipeline-policy struct literals unchanged.
use super::*;

impl Graphic3dRenderDriver {
    /// Select line clipping independently of presentation widths and attachment coordinates.
    /// Example: `Self::with_line_clipping_configuration(options, policies, coordinates, widths, RenderLineClippingBehavior::PlatformNativeV2)?`.
    pub fn with_line_clipping_configuration(
        options: VulkanOptions,
        selection: RenderPipelinePolicies,
        coordinates: crate::RenderCoordinateConvention,
        line_widths: crate::RenderLineWidthPolicy,
        behavior: crate::RenderLineClippingBehavior,
    ) -> Result<Handle<Self>, RenderError> {
        selection.unlit_faces.validate(selection.lighting)?;
        let renderer = VulkanRenderer::new_with_line_clipping(
            options,
            selection.raster,
            coordinates,
            behavior,
        )?;
        Ok(Self::build(DriverBuildInputs {
            limit: renderer.capture_limit(),
            backend: DriverBackend::Vulkan(Box::new(renderer)),
            text_source: Arc::new(crate::FreeTypeTextSource::default()),
            selection,
            line_widths,
        }))
    }
}
