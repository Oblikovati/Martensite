//! Additive construction composes interpolation with existing independent policies.
use super::*;
impl Graphic3dRenderDriver {
    /// Compose every versioned raster behavior before device initialization.
    /// Example: `Self::with_render_behavior_configuration(options, policies, coordinates, widths, clipping, configuration)?`.
    pub fn with_render_behavior_configuration(
        options: VulkanOptions,
        selection: RenderPipelinePolicies,
        coordinates: crate::RenderCoordinateConvention,
        widths: crate::RenderLineWidthPolicy,
        clipping: crate::RenderLineClippingBehavior,
        configuration: crate::RenderBehaviorConfiguration,
    ) -> Result<Handle<Self>, RenderError> {
        selection.unlit_faces.validate(selection.lighting)?;
        let renderer = VulkanRenderer::new_with_render_behavior(
            options,
            selection.raster,
            coordinates,
            clipping,
            configuration,
        )?;
        Ok(Self::publish_renderer(renderer, selection, widths))
    }
    /// Compose coverage and interpolation choices before device initialization.
    /// Example: `Self::with_phong_interpolation_and_coverage_configuration(options, policies, coordinates, widths, clipping, (points, dots, axes), interpolation)?`.
    pub fn with_phong_interpolation_and_coverage_configuration(
        options: VulkanOptions,
        selection: RenderPipelinePolicies,
        coordinates: crate::RenderCoordinateConvention,
        widths: crate::RenderLineWidthPolicy,
        clipping: crate::RenderLineClippingBehavior,
        choices: (
            crate::RenderPointRasterBehavior,
            crate::RenderLightDotBehavior,
            crate::RenderThinAxisCoverage,
        ),
        interpolation: crate::RenderPhongInterpolationBehavior,
    ) -> Result<Handle<Self>, RenderError> {
        let configuration = crate::RenderBehaviorConfiguration::default()
            .with_point_raster(choices.0)
            .with_light_dots(choices.1)
            .with_thin_axes(choices.2)
            .with_phong_interpolation(interpolation);
        Self::with_render_behavior_configuration(
            options,
            selection,
            coordinates,
            widths,
            clipping,
            configuration,
        )
    }
    /// Choose interpolation once for the operation; `Self::with_phong_interpolation_configuration(options, selection, coordinates, widths, clipping, points, dots, interpolation)?`.
    #[allow(clippy::too_many_arguments)] // Each argument is a distinct Vulkan or behaviour input.
    pub fn with_phong_interpolation_configuration(
        options: VulkanOptions,
        selection: RenderPipelinePolicies,
        coordinates: crate::RenderCoordinateConvention,
        widths: crate::RenderLineWidthPolicy,
        clipping: crate::RenderLineClippingBehavior,
        points: crate::RenderPointRasterBehavior,
        dots: crate::RenderLightDotBehavior,
        interpolation: crate::RenderPhongInterpolationBehavior,
    ) -> Result<Handle<Self>, RenderError> {
        selection.unlit_faces.validate(selection.lighting)?;
        let create = VulkanRenderer::new_with_phong_interpolation;
        let renderer = create(
            options,
            selection.raster,
            coordinates,
            clipping,
            points,
            dots,
            interpolation,
        )?;
        Ok(Self::publish_renderer(renderer, selection, widths))
    }
}
