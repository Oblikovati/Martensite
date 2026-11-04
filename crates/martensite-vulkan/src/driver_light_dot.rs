//! Additive configuration keeps existing public option/policy struct literals valid.
use super::*;
impl Graphic3dRenderDriver {
    /// Compose versioned light-dot semantics with existing independent presentation policies.
    /// Example: `Self::with_light_dot_configuration(options, policies, coordinates, widths, clipping, dots)?`.
    pub fn with_light_dot_configuration(
        options: VulkanOptions,
        selection: RenderPipelinePolicies,
        coordinates: crate::RenderCoordinateConvention,
        widths: crate::RenderLineWidthPolicy,
        clipping: crate::RenderLineClippingBehavior,
        dots: crate::RenderLightDotBehavior,
    ) -> Result<Handle<Self>, RenderError> {
        selection.unlit_faces.validate(selection.lighting)?;
        let create = VulkanRenderer::new_with_light_dot_configuration;
        let renderer = create(options, selection.raster, coordinates, clipping, dots)?;
        Ok(Self::build(DriverBuildInputs {
            limit: renderer.capture_limit(),
            backend: DriverBackend::Vulkan(Box::new(renderer)),
            text_source: Arc::new(crate::FreeTypeTextSource::default()),
            selection,
            line_widths: widths,
        }))
    }
}

#[cfg(feature = "vertex-output-diagnostics")]
impl Graphic3dRenderDriver {
    /// Inject the request into the same ordinary driver; `Self::with_vertex_output_configuration(options, policies, coordinates, widths, clipping, dots, request)?`.
    pub fn with_vertex_output_configuration(
        options: VulkanOptions,
        selection: RenderPipelinePolicies,
        coordinates: crate::RenderCoordinateConvention,
        widths: crate::RenderLineWidthPolicy,
        clipping: crate::RenderLineClippingBehavior,
        dots: crate::RenderLightDotBehavior,
        request: crate::VertexOutputRequest,
    ) -> Result<Handle<Self>, RenderError> {
        let configuration = crate::RenderBehaviorConfiguration::default().with_light_dots(dots);
        Self::with_vertex_output_behavior(
            options,
            selection,
            coordinates,
            widths,
            clipping,
            configuration,
            request,
        )
    }
    /// Compose observation with explicit operation behavior; `Self::with_vertex_output_behavior(options, policies, coordinates, widths, clipping, configuration, request)?`.
    pub fn with_vertex_output_behavior(
        options: VulkanOptions,
        selection: RenderPipelinePolicies,
        coordinates: crate::RenderCoordinateConvention,
        widths: crate::RenderLineWidthPolicy,
        clipping: crate::RenderLineClippingBehavior,
        configuration: crate::RenderBehaviorConfiguration,
        request: crate::VertexOutputRequest,
    ) -> Result<Handle<Self>, RenderError> {
        selection.unlit_faces.validate(selection.lighting)?;
        let renderer = VulkanRenderer::new_with_vertex_output_behavior(
            options,
            selection.raster,
            coordinates,
            clipping,
            configuration,
            request,
        )?;
        Ok(Self::publish_renderer(renderer, selection, widths))
    }
}
