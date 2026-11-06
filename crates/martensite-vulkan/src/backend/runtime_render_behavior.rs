//! Compose immutable behavior before ordinary or diagnostic device admission.
use super::*;
impl VulkanRuntime {
    pub fn create_with_point_raster(
        options: VulkanOptions,
        policy: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
        line_clipping: crate::RenderLineClippingBehavior,
        point_raster: crate::RenderPointRasterBehavior,
        light_dots: crate::RenderLightDotBehavior,
    ) -> Result<Arc<Self>, RenderError> {
        Self::create_with_coverage_configuration(
            options,
            policy,
            coordinates,
            line_clipping,
            (point_raster, light_dots, Default::default()),
        )
    }

    pub fn create(options: VulkanOptions) -> Result<Arc<Self>, RenderError> {
        let policy = crate::RenderRasterPolicy::default();
        Self::create_with_raster_policy(options, policy)
    }

    pub(in crate::backend) fn create_with_render_behavior(
        options: VulkanOptions,
        raster: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
        clipping: crate::RenderLineClippingBehavior,
        configuration: crate::RenderBehaviorConfiguration,
    ) -> Result<Arc<Self>, RenderError> {
        let mut runtime = Self::configured_runtime(options, raster, coordinates)?;
        runtime.line_clipping = clipping;
        runtime.apply_render_behavior(configuration);
        runtime.initialize_device(options.device_index)?;
        Ok(Arc::new(runtime))
    }
    fn apply_render_behavior(&mut self, configuration: crate::RenderBehaviorConfiguration) {
        let (points, dots, axes, lines, interpolation, triangles, text) = configuration.choices();
        self.depth_bias_behavior = configuration.depth_bias();
        self.projection_behavior = configuration.projection();
        self.window_depth_behavior = configuration.window_depth();
        self.point_raster = points;
        self.light_dot_behavior = dots;
        self.thin_axis_coverage = axes;
        self.thin_line_coverage = lines;
        self.phong_interpolation = interpolation;
        self.triangle_coverage = triangles;
        self.text_interpolation = text;
        self.world_clipping = configuration.world_clipping();
        self.world_clip_interpolation = configuration.world_clip_interpolation();
    }
    #[cfg(feature = "vertex-output-diagnostics")]
    pub(in crate::backend) fn create_with_vertex_output(
        options: VulkanOptions,
        policy: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
        clipping: crate::RenderLineClippingBehavior,
        configuration: crate::RenderBehaviorConfiguration,
        request: crate::VertexOutputRequest,
    ) -> Result<Arc<Self>, RenderError> {
        let mut runtime = Self::configured_runtime(options, policy, coordinates)?;
        runtime.line_clipping = clipping;
        runtime.apply_render_behavior(configuration);
        runtime.vertex_output = Some(Arc::new(request));
        runtime.initialize_device(options.device_index)?;
        Ok(Arc::new(runtime))
    }
}
