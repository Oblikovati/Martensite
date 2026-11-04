//! Additive construction leaves existing option and policy literals unchanged.
use super::*;
impl VulkanRenderer {
    /// Select fixed-grid point coverage explicitly; `Self::new_with_point_raster(options, raster, coordinates, clipping, points)?`.
    pub fn new_with_point_raster(
        options: VulkanOptions,
        raster: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
        clipping: crate::RenderLineClippingBehavior,
        points: crate::RenderPointRasterBehavior,
    ) -> Result<Self, RenderError> {
        Self::new_with_point_and_light_dot_configuration(
            options,
            raster,
            coordinates,
            clipping,
            points,
            Default::default(),
        )
    }

    /// Compose immutable point and lighting choices; `Self::new_with_point_and_light_dot_configuration(options, raster, coordinates, clipping, points, dots)?`.
    pub fn new_with_point_and_light_dot_configuration(
        options: VulkanOptions,
        raster: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
        clipping: crate::RenderLineClippingBehavior,
        points: crate::RenderPointRasterBehavior,
        dots: crate::RenderLightDotBehavior,
    ) -> Result<Self, RenderError> {
        let runtime = VulkanRuntime::create_with_point_raster(
            options,
            raster,
            coordinates,
            clipping,
            points,
            dots,
        )?;
        Self::initialized_renderer(runtime)
    }

    /// Read the immutable replay choice; `renderer.point_raster_behavior()`.
    pub fn point_raster_behavior(&self) -> crate::RenderPointRasterBehavior {
        self.runtime.point_raster
    }
}
