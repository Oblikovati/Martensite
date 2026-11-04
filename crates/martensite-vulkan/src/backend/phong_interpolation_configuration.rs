//! Construct one immutable composition without changing any legacy constructor.
use super::*;
impl VulkanRenderer {
    /// Select interpolation independently; `Self::new_with_phong_interpolation(options, raster, coordinates, clipping, points, dots, interpolation)?`.
    pub fn new_with_phong_interpolation(
        options: VulkanOptions,
        raster: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
        clipping: crate::RenderLineClippingBehavior,
        points: crate::RenderPointRasterBehavior,
        dots: crate::RenderLightDotBehavior,
        interpolation: crate::RenderPhongInterpolationBehavior,
    ) -> Result<Self, RenderError> {
        let configuration = crate::RenderBehaviorConfiguration::default()
            .with_point_raster(points)
            .with_light_dots(dots)
            .with_phong_interpolation(interpolation);
        Self::new_with_render_behavior(options, raster, coordinates, clipping, configuration)
    }
    /// Inspect the requested replay version; `renderer.phong_interpolation_behavior()`.
    pub fn phong_interpolation_behavior(&self) -> crate::RenderPhongInterpolationBehavior {
        self.runtime.phong_interpolation
    }
}
