//! Additive opt-in leaves existing option and pipeline-policy struct literals unchanged.
use super::*;

impl Graphic3dRenderDriver {
    /// Select line clipping independently of presentation widths and attachment coordinates.
    /// Example: `Self::with_line_clipping_configuration(options, policies, coordinates, widths, RenderLineClippingBehavior::PlatformNativeV2)?`.
    pub fn with_line_clipping_configuration(
        options: impl RenderDeviceOptions,
        selection: RenderPipelinePolicies,
        coordinates: crate::RenderCoordinateConvention,
        line_widths: crate::RenderLineWidthPolicy,
        behavior: crate::RenderLineClippingBehavior,
    ) -> Result<Handle<Self>, RenderError> {
        selection.unlit_faces.validate(selection.lighting)?;
        let renderer =
            options.create_device(selection.raster, coordinates, behavior, Default::default())?;
        Self::publish_renderer(renderer, selection, line_widths)
    }
}
