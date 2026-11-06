//! Independent immutable numerical policy does not change earlier constructors.
use super::*;
impl VulkanRenderer {
    /// Select Phong light-dot semantics independently of raster/attachment/line behavior.
    /// Example: `Self::new_with_light_dot_configuration(options, raster, coordinates, clipping, dots)?`.
    pub fn new_with_light_dot_configuration(
        options: VulkanOptions,
        raster: crate::RenderRasterPolicy,
        coordinates: crate::RenderCoordinateConvention,
        clipping: crate::RenderLineClippingBehavior,
        dots: crate::RenderLightDotBehavior,
    ) -> Result<Self, RenderError> {
        let runtime =
            VulkanRuntime::create_with_light_dots(options, raster, coordinates, clipping, dots)?;
        Self::initialized_renderer(runtime)
    }
    /// Inspect the requested built-in Phong policy (custom/PBR/fixed remain unapplied); `renderer.light_dot_behavior()`.
    pub fn light_dot_behavior(&self) -> crate::RenderLightDotBehavior {
        self.runtime.light_dot_behavior
    }
}
