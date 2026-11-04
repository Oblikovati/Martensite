//! Immutable selection of versioned primitive rendering behavior.
/// Composes point, line, triangle coverage, text interpolation, light-dot, thin-axis, and Phong interpolation behavior.
/// Example: `RenderBehaviorConfiguration::default().with_light_dots(RenderLightDotBehavior::NumericMaxZeroV2)`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RenderBehaviorConfiguration {
    depth_bias: crate::RenderDepthBiasBehavior,
    window_depth: crate::RenderWindowDepthBehavior,
    point_raster: crate::RenderPointRasterBehavior,
    light_dots: crate::RenderLightDotBehavior,
    thin_axes: crate::RenderThinAxisCoverage,
    thin_lines: crate::RenderThinLineCoverage,
    phong_interpolation: crate::RenderPhongInterpolationBehavior,
    triangle_coverage: crate::RenderTriangleCoverageBehavior,
    text_interpolation: crate::RenderTextInterpolationBehavior,
    world_clipping: crate::RenderWorldClippingBehavior,
    world_clip_interpolation: crate::RenderWorldClipInterpolationBehavior,
    projection: crate::RenderProjectionBehavior,
}
impl RenderBehaviorConfiguration {
    /// Select window-depth reconstruction; `configuration.with_window_depth(RenderWindowDepthBehavior::HardwareV1)`.
    pub fn with_window_depth(mut self, behavior: crate::RenderWindowDepthBehavior) -> Self {
        self.window_depth = behavior;
        self
    }
    pub(crate) fn window_depth(self) -> crate::RenderWindowDepthBehavior {
        self.window_depth
    }
    /// Select an explicit triangle projection; `configuration.with_projection(RenderProjectionBehavior::FactorizedTrianglesV2)`.
    pub fn with_projection(mut self, behavior: crate::RenderProjectionBehavior) -> Self {
        self.projection = behavior;
        self
    }
    pub(crate) fn projection(self) -> crate::RenderProjectionBehavior {
        self.projection
    }
    /// Set point raster behavior; `configuration.with_point_raster(RenderPointRasterBehavior::FixedGridAliasedV2)`.
    pub fn with_point_raster(mut self, behavior: crate::RenderPointRasterBehavior) -> Self {
        self.point_raster = behavior;
        self
    }
    /// Set light-dot behavior; `configuration.with_light_dots(RenderLightDotBehavior::NumericMaxZeroV2)`.
    pub fn with_light_dots(mut self, behavior: crate::RenderLightDotBehavior) -> Self {
        self.light_dots = behavior;
        self
    }
    /// Set thin-axis coverage; `configuration.with_thin_axes(RenderThinAxisCoverage::HardwareV1)`.
    pub fn with_thin_axes(mut self, behavior: crate::RenderThinAxisCoverage) -> Self {
        self.thin_axes = behavior;
        self
    }
    /// Set independent thin-line coverage; `configuration.with_thin_lines(RenderThinLineCoverage::DiamondExitV2)`.
    pub fn with_thin_lines(mut self, behavior: crate::RenderThinLineCoverage) -> Self {
        self.thin_lines = behavior;
        self
    }
    /// Set Phong interpolation; `configuration.with_phong_interpolation(RenderPhongInterpolationBehavior::UnsnappedV2)`.
    pub fn with_phong_interpolation(
        mut self,
        behavior: crate::RenderPhongInterpolationBehavior,
    ) -> Self {
        self.phong_interpolation = behavior;
        self
    }
    /// Set triangle coverage; `configuration.with_triangle_coverage(RenderTriangleCoverageBehavior::HardwareV1)`.
    pub fn with_triangle_coverage(
        mut self,
        behavior: crate::RenderTriangleCoverageBehavior,
    ) -> Self {
        self.triangle_coverage = behavior;
        self
    }
    /// Set text interpolation; `configuration.with_text_interpolation(RenderTextInterpolationBehavior::ProjectedGrid12V2)`.
    pub fn with_text_interpolation(
        mut self,
        behavior: crate::RenderTextInterpolationBehavior,
    ) -> Self {
        self.text_interpolation = behavior;
        self
    }
    /// Select raster-stage world clipping; `configuration.with_world_clipping(RenderWorldClippingBehavior::RasterClipDistanceV3)`.
    pub fn with_world_clipping(mut self, behavior: crate::RenderWorldClippingBehavior) -> Self {
        self.world_clipping = behavior;
        self
    }
    /// Set fragment clip interpolation; `configuration.with_world_clip_interpolation(RenderWorldClipInterpolationBehavior::ProjectedGrid12V1)`.
    pub fn with_world_clip_interpolation(
        mut self,
        behavior: crate::RenderWorldClipInterpolationBehavior,
    ) -> Self {
        self.world_clip_interpolation = behavior;
        self
    }
    /// Select explicit depth-bias units; `configuration.with_depth_bias(RenderDepthBiasBehavior::Native24UnitsV2)`.
    pub fn with_depth_bias(mut self, behavior: crate::RenderDepthBiasBehavior) -> Self {
        self.depth_bias = behavior;
        self
    }
    pub(crate) fn depth_bias(self) -> crate::RenderDepthBiasBehavior {
        self.depth_bias
    }
    pub(crate) fn world_clipping(self) -> crate::RenderWorldClippingBehavior {
        self.world_clipping
    }
    pub(crate) fn world_clip_interpolation(self) -> crate::RenderWorldClipInterpolationBehavior {
        self.world_clip_interpolation
    }
    pub(crate) fn choices(
        self,
    ) -> (
        crate::RenderPointRasterBehavior,
        crate::RenderLightDotBehavior,
        crate::RenderThinAxisCoverage,
        crate::RenderThinLineCoverage,
        crate::RenderPhongInterpolationBehavior,
        crate::RenderTriangleCoverageBehavior,
        crate::RenderTextInterpolationBehavior,
    ) {
        (
            self.point_raster,
            self.light_dots,
            self.thin_axes,
            self.thin_lines,
            self.phong_interpolation,
            self.triangle_coverage,
            self.text_interpolation,
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defaults_select_all_hardware_v1_behaviors() {
        assert_eq!(
            RenderBehaviorConfiguration::default().choices(),
            hardware_choices()
        );
        assert_eq!(
            RenderBehaviorConfiguration::default().world_clip_interpolation(),
            crate::RenderWorldClipInterpolationBehavior::HardwareV1
        );
    }
    fn hardware_choices() -> (
        crate::RenderPointRasterBehavior,
        crate::RenderLightDotBehavior,
        crate::RenderThinAxisCoverage,
        crate::RenderThinLineCoverage,
        crate::RenderPhongInterpolationBehavior,
        crate::RenderTriangleCoverageBehavior,
        crate::RenderTextInterpolationBehavior,
    ) {
        (
            crate::RenderPointRasterBehavior::HardwareQuadV1,
            crate::RenderLightDotBehavior::DriverNativeV1,
            crate::RenderThinAxisCoverage::HardwareV1,
            crate::RenderThinLineCoverage::HardwareV1,
            crate::RenderPhongInterpolationBehavior::HardwareV1,
            crate::RenderTriangleCoverageBehavior::HardwareV1,
            crate::RenderTextInterpolationBehavior::HardwareV1,
        )
    }
    #[test]
    fn builders_preserve_nondefault_independent_choices() {
        let selected = RenderBehaviorConfiguration::default()
            .with_point_raster(crate::RenderPointRasterBehavior::FixedGridAliasedV2)
            .with_light_dots(crate::RenderLightDotBehavior::NumericMaxZeroV2)
            .with_thin_axes(crate::RenderThinAxisCoverage::FixedGridV2 {
                grid: crate::RenderLineGrid::Fractional12,
                origin: crate::RenderLineOrigin::BottomUp,
            })
            .with_thin_lines(crate::RenderThinLineCoverage::DiamondExitV2)
            .with_phong_interpolation(crate::RenderPhongInterpolationBehavior::UnsnappedV2)
            .with_triangle_coverage(crate::RenderTriangleCoverageBehavior::FixedGrid12V2 {
                origin: crate::RenderTriangleGridOrigin::TopDown,
            })
            .with_text_interpolation(crate::RenderTextInterpolationBehavior::ProjectedGrid12V2);
        let selected = selected.with_world_clip_interpolation(
            crate::RenderWorldClipInterpolationBehavior::ProjectedGrid12V1,
        );
        assert_eq!(
            selected.choices(),
            (
                crate::RenderPointRasterBehavior::FixedGridAliasedV2,
                crate::RenderLightDotBehavior::NumericMaxZeroV2,
                crate::RenderThinAxisCoverage::FixedGridV2 {
                    grid: crate::RenderLineGrid::Fractional12,
                    origin: crate::RenderLineOrigin::BottomUp,
                },
                crate::RenderThinLineCoverage::DiamondExitV2,
                crate::RenderPhongInterpolationBehavior::UnsnappedV2,
                crate::RenderTriangleCoverageBehavior::FixedGrid12V2 {
                    origin: crate::RenderTriangleGridOrigin::TopDown,
                },
                crate::RenderTextInterpolationBehavior::ProjectedGrid12V2,
            )
        );
        assert_eq!(
            selected.world_clip_interpolation(),
            crate::RenderWorldClipInterpolationBehavior::ProjectedGrid12V1
        );
    }
    #[test]
    fn unit12_point_behavior_composes_without_changing_other_defaults() {
        let selected = RenderBehaviorConfiguration::default()
            .with_point_raster(crate::RenderPointRasterBehavior::FixedGridUnit12V3);
        assert_eq!(
            selected.choices().0,
            crate::RenderPointRasterBehavior::FixedGridUnit12V3
        );
        assert_eq!(
            selected.choices().1,
            crate::RenderLightDotBehavior::DriverNativeV1
        );
    }
    #[test]
    fn raster_world_clip_is_opt_in_and_composes_independently() {
        use crate::RenderWorldClippingBehavior as World;
        let legacy = RenderBehaviorConfiguration::default();
        let selected = legacy.with_world_clipping(World::RasterClipDistanceV2);
        let lines = legacy.with_world_clipping(World::RasterClipDistanceV3);
        assert_eq!(legacy.world_clipping(), World::FragmentV1);
        assert_eq!(selected.world_clipping(), World::RasterClipDistanceV2);
        assert_eq!(lines.world_clipping(), World::RasterClipDistanceV3);
        assert_eq!(selected.choices(), legacy.choices());
        assert_eq!(lines.choices(), legacy.choices());
    }
    #[test]
    fn configuration_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<RenderBehaviorConfiguration>();
    }
}

#[cfg(test)]
mod depth_bias_tests {
    #[test]
    fn bias_choice_preserves_all_other_choices_and_default() {
        use crate::{RenderBehaviorConfiguration as C, RenderDepthBiasBehavior as B};
        let old =
            C::default().with_point_raster(crate::RenderPointRasterBehavior::FixedGridSprites12V4);
        assert_eq!(old.depth_bias(), B::HardwareFormatV1);
        let new = old.with_depth_bias(B::Native24UnitsV2);
        assert_eq!(new.depth_bias(), B::Native24UnitsV2);
        assert_eq!(new.choices(), old.choices());
    }
}

#[cfg(test)]
mod window_depth_tests {
    #[test]
    fn depth_plane_selection_preserves_projection_units_and_other_choices() {
        use crate::{RenderBehaviorConfiguration as C, RenderWindowDepthBehavior as D};
        let old = C::default()
            .with_projection(crate::RenderProjectionBehavior::FactorizedTrianglesV2)
            .with_depth_bias(crate::RenderDepthBiasBehavior::Native24UnitsV2);
        let selected = D::UnsnappedWindowDepthV2 {
            reference_grid: crate::RenderViewportGridPrecision::Fractional12,
            raster_grid: crate::RenderViewportGridPrecision::Fractional8,
        };
        let new = old.with_window_depth(selected);
        assert_eq!(old.window_depth(), D::HardwareV1);
        assert_eq!(new.window_depth(), selected);
        assert_eq!(new.projection(), old.projection());
        assert_eq!(new.depth_bias(), old.depth_bias());
        assert_eq!(new.choices(), old.choices());
    }
}
