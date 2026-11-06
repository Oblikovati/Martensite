//! Additive construction keeps existing option literals and independent coverage choices.
use super::*;
type CoverageChoices = (
    crate::RenderPointRasterBehavior,
    crate::RenderLightDotBehavior,
    crate::RenderThinAxisCoverage,
);
impl VulkanRenderer {
    /// Compose all versioned raster behaviors before device initialization.
    /// Example: `Self::new_with_render_behavior(options, raster, coordinates, clipping, configuration)?`.
    pub fn new_with_render_behavior(
        options: VulkanOptions,
        raster: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
        clipping: crate::RenderLineClippingBehavior,
        configuration: crate::RenderBehaviorConfiguration,
    ) -> Result<Self, RenderError> {
        let runtime = VulkanRuntime::create_with_render_behavior(
            options,
            raster,
            coordinates,
            clipping,
            configuration,
        )?;
        Self::initialized_renderer(runtime)
    }
    /// Select thin-axis coverage with legacy point/light behavior.
    /// Example: `Self::new_with_thin_axis_coverage(options, raster, coordinates, coverage)?`.
    pub fn new_with_thin_axis_coverage(
        options: VulkanOptions,
        raster: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
        coverage: crate::RenderThinAxisCoverage,
    ) -> Result<Self, RenderError> {
        Self::new_with_coverage_configuration(
            options,
            raster,
            coordinates,
            Default::default(),
            (Default::default(), Default::default(), coverage),
        )
    }
    /// Compose point, light-dot and thin-axis behaviors without overwriting independent choices.
    /// Example: `Self::new_with_coverage_configuration(options, raster, coordinates, clipping, (points, dots, axes))?`.
    pub fn new_with_coverage_configuration(
        options: VulkanOptions,
        raster: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
        clipping: crate::RenderLineClippingBehavior,
        choices: CoverageChoices,
    ) -> Result<Self, RenderError> {
        let configuration = crate::RenderBehaviorConfiguration::default()
            .with_point_raster(choices.0)
            .with_light_dots(choices.1)
            .with_thin_axes(choices.2);
        Self::new_with_render_behavior(options, raster, coordinates, clipping, configuration)
    }
    /// Record the immutable coverage choice; `renderer.thin_axis_coverage()`.
    pub fn thin_axis_coverage(&self) -> crate::RenderThinAxisCoverage {
        self.runtime.thin_axis_coverage
    }
    /// Record selected independent line coverage; `renderer.thin_line_coverage()`.
    pub fn thin_line_coverage(&self) -> crate::RenderThinLineCoverage {
        self.runtime.thin_line_coverage
    }
    /// Inspect the selected triangle coverage policy; `renderer.triangle_coverage_behavior()`.
    pub fn triangle_coverage_behavior(&self) -> crate::RenderTriangleCoverageBehavior {
        self.runtime.triangle_coverage
    }
}
impl VulkanRuntime {
    pub(super) fn create_with_coverage_configuration(
        options: VulkanOptions,
        raster: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
        clipping: crate::RenderLineClippingBehavior,
        (points, dots, axes): CoverageChoices,
    ) -> Result<std::sync::Arc<Self>, RenderError> {
        let mut runtime = Self::configured_runtime(options, raster, coordinates)?;
        runtime.line_clipping = clipping;
        runtime.point_raster = points;
        runtime.light_dot_behavior = dots;
        runtime.thin_axis_coverage = axes;
        runtime.initialize_device(options.device_index)?;
        Ok(std::sync::Arc::new(runtime))
    }
}
