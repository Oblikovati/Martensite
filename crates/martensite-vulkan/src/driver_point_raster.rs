//! Additive opt-in leaves existing option and pipeline-policy struct literals unchanged.
use super::*;

impl Graphic3dRenderDriver {
    /// Select point coverage without changing existing policy struct literals.
    /// Example: `Self::with_point_raster_configuration(options, policies, coordinates, widths, RenderLineClippingBehavior::PlatformNativeV2, RenderPointRasterBehavior::FixedGridAliasedV2)?`.
    pub fn with_point_raster_configuration(
        options: VulkanOptions,
        selection: RenderPipelinePolicies,
        coordinates: crate::RenderCoordinateConvention,
        line_widths: crate::RenderLineWidthPolicy,
        behavior: crate::RenderLineClippingBehavior,
        points: crate::RenderPointRasterBehavior,
    ) -> Result<Handle<Self>, RenderError> {
        Self::with_point_and_light_dot_configuration(
            options,
            selection,
            coordinates,
            line_widths,
            behavior,
            points,
            Default::default(),
        )
    }

    /// Compose independent immutable coverage and lighting choices.
    /// Example: `Self::with_point_and_light_dot_configuration(options, policies, coordinates, widths, clipping, points, dots)?`.
    pub fn with_point_and_light_dot_configuration(
        options: VulkanOptions,
        selection: RenderPipelinePolicies,
        coordinates: crate::RenderCoordinateConvention,
        line_widths: crate::RenderLineWidthPolicy,
        behavior: crate::RenderLineClippingBehavior,
        points: crate::RenderPointRasterBehavior,
        dots: crate::RenderLightDotBehavior,
    ) -> Result<Handle<Self>, RenderError> {
        Self::with_coverage_configuration(
            options,
            selection,
            coordinates,
            line_widths,
            behavior,
            (points, dots, Default::default()),
        )
    }
    /// Compose point, light-dot and thin-axis coverage with the same Vulkan driver owner.
    /// Example: `Self::with_coverage_configuration(options, policies, coordinates, widths, clipping, (points, dots, axes))?`.
    pub fn with_coverage_configuration(
        options: VulkanOptions,
        selection: RenderPipelinePolicies,
        coordinates: crate::RenderCoordinateConvention,
        line_widths: crate::RenderLineWidthPolicy,
        clipping: crate::RenderLineClippingBehavior,
        choices: (
            crate::RenderPointRasterBehavior,
            crate::RenderLightDotBehavior,
            crate::RenderThinAxisCoverage,
        ),
    ) -> Result<Handle<Self>, RenderError> {
        selection.unlit_faces.validate(selection.lighting)?;
        let renderer = VulkanRenderer::new_with_coverage_configuration(
            options,
            selection.raster,
            coordinates,
            clipping,
            choices,
        )?;
        Ok(Self::publish_renderer(renderer, selection, line_widths))
    }
}
