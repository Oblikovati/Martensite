//! The V3 no-covered-sample argument belongs only to this ordinary fill pipeline.
use crate::{
    RenderError, RenderPhongInterpolationBehavior as Behavior,
    RenderPhongInterpolationFailure as Failure,
};
use ash::vk;
#[cfg(test)]
pub(super) fn require_state(
    behavior: Behavior,
    selected: bool,
    raster: &vk::PipelineRasterizationStateCreateInfo<'_>,
    stages: usize,
    v4_geometry_partner: bool,
) -> Result<(), RenderError> {
    require_state_with_conservative(
        behavior,
        selected,
        raster,
        stages,
        v4_geometry_partner,
        None,
        false,
    )
}

pub(super) fn require_state_with_conservative(
    behavior: Behavior,
    selected: bool,
    raster: &vk::PipelineRasterizationStateCreateInfo<'_>,
    stages: usize,
    v4_geometry_partner: bool,
    conservative_node: Option<*const std::ffi::c_void>,
    exact_overestimate: bool,
) -> Result<(), RenderError> {
    if behavior != Behavior::UnsnappedV3 || !selected {
        return Ok(());
    }
    if ordinary_fill(
        raster,
        stages,
        v4_geometry_partner,
        conservative_node,
        exact_overestimate,
    ) {
        return Ok(());
    }
    Err(Failure::UnsupportedInterface { actual: format!(
        "V3 raster polygon={:?}, next={:?}, discard={}, stages={stages}, V4 geometry partner={v4_geometry_partner}; expected FILL, raster enabled, VS+FS or registered V4 VS+GS+FS with exact overestimate state", raster.polygon_mode, raster.p_next, raster.rasterizer_discard_enable
    ) }.into())
}

pub(super) fn exact_overestimate_state(
    state: &vk::PipelineRasterizationConservativeStateCreateInfoEXT<'_>,
) -> bool {
    state.s_type == vk::StructureType::PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT
        && state.p_next.is_null()
        && state.flags.as_raw() == 0
        && state.conservative_rasterization_mode
            == vk::ConservativeRasterizationModeEXT::OVERESTIMATE
        && state.extra_primitive_overestimation_size.to_bits() == 0
}

fn ordinary_fill(
    raster: &vk::PipelineRasterizationStateCreateInfo<'_>,
    stages: usize,
    v4_geometry_partner: bool,
    conservative_node: Option<*const std::ffi::c_void>,
    exact_overestimate: bool,
) -> bool {
    raster.polygon_mode == vk::PolygonMode::FILL
        && raster.rasterizer_discard_enable == vk::FALSE
        && supported_raster_extension(
            raster,
            stages,
            v4_geometry_partner,
            conservative_node,
            exact_overestimate,
        )
}

fn supported_raster_extension(
    raster: &vk::PipelineRasterizationStateCreateInfo<'_>,
    stages: usize,
    v4_geometry_partner: bool,
    conservative_node: Option<*const std::ffi::c_void>,
    exact_overestimate: bool,
) -> bool {
    if raster.p_next.is_null() {
        return stages == 2 || (v4_geometry_partner && stages == 3);
    }
    if !v4_geometry_partner || stages != 3 {
        return false;
    }
    conservative_node == Some(raster.p_next) && exact_overestimate
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordinary_fill_requires_supported_raster_extension_and_stage_shape() {
        let fill =
            vk::PipelineRasterizationStateCreateInfo::default().polygon_mode(vk::PolygonMode::FILL);
        assert!(require_state(Behavior::UnsnappedV3, true, &fill, 2, false).is_ok());
        assert!(require_state(Behavior::UnsnappedV3, true, &fill, 3, true).is_ok());
        assert!(require_state(Behavior::UnsnappedV3, true, &fill, 3, false).is_err());
        for mode in [
            vk::PolygonMode::LINE,
            vk::PolygonMode::POINT,
            vk::PolygonMode::FILL_RECTANGLE_NV,
        ] {
            assert!(
                require_state(
                    Behavior::UnsnappedV3,
                    true,
                    &fill.polygon_mode(mode),
                    2,
                    false
                )
                .is_err()
            );
        }
        let mut conservative = vk::PipelineRasterizationConservativeStateCreateInfoEXT::default();
        assert!(
            require_state(
                Behavior::UnsnappedV3,
                true,
                &fill.push_next(&mut conservative),
                2,
                false,
            )
            .is_err()
        );
        assert!(
            require_state(
                Behavior::UnsnappedV3,
                true,
                &fill.rasterizer_discard_enable(true),
                2,
                false,
            )
            .is_err()
        );
        assert!(
            require_state(
                Behavior::UnsnappedV2,
                true,
                &fill.polygon_mode(vk::PolygonMode::LINE),
                3,
                false,
            )
            .is_ok()
        );
    }

    #[test]
    fn v4_logical_coverage_discards_overestimate_before_phong_interpolation() {
        let source = include_str!("../../shaders/lighting.frag");
        let coverage = source
            .find("if (!triangleGridCoveredV2()) discard;")
            .unwrap();
        let axis = source.find("phongExactProjectedAxis()").unwrap();
        let shading = source.find("fragmentColor = litFragmentColor();").unwrap();
        assert!(coverage < axis && axis < shading);

        let registry = include_str!("world_clip_grid12_shader.rs");
        assert!(registry.contains("FRAGMENTS_V4_COMPACT"));
        assert!(registry.contains("fragment_v4_compact"));
        let routing = include_str!("pipeline_vertex_output.rs");
        assert!(routing.contains("world_clip_grid12_shader::fragment_v4_compact(stock)"));
        let creation = include_str!("pipeline_creation.rs");
        assert!(creation.contains("owner.has_v4_geometry_stage()"));
    }

    #[test]
    fn v4_overestimate_requires_three_stage_partner() {
        let fill = ordinary_raster();
        let mut state = conservative_overestimate_state();
        assert!(accepted(&fill, &mut state, 3, true));
        assert!(!accepted(&fill, &mut state, 2, false));
        assert!(!accepted(&fill, &mut state, 3, false));
    }

    #[test]
    fn v4_overestimate_refuses_other_mode_or_extra_size() {
        let fill = ordinary_raster();
        let mut under = vk::PipelineRasterizationConservativeStateCreateInfoEXT::default()
            .conservative_rasterization_mode(vk::ConservativeRasterizationModeEXT::UNDERESTIMATE);
        let mut extra = conservative_overestimate_state().extra_primitive_overestimation_size(0.25);
        assert!(!accepted(&fill, &mut under, 3, true));
        assert!(!accepted(&fill, &mut extra, 3, true));
    }

    #[test]
    fn v4_overestimate_refuses_flags_and_nested_extension() {
        let fill = ordinary_raster();
        let mut flags = conservative_overestimate_state()
            .flags(vk::PipelineRasterizationConservativeStateCreateFlagsEXT::from_raw(1));
        let child = vk::PipelineRasterizationDepthClipStateCreateInfoEXT::default();
        let mut chained = conservative_overestimate_state();
        chained.p_next = std::ptr::from_ref(&child).cast();
        assert!(!accepted(&fill, &mut flags, 3, true));
        assert!(!accepted(&fill, &mut chained, 3, true));
    }

    fn ordinary_raster() -> vk::PipelineRasterizationStateCreateInfo<'static> {
        vk::PipelineRasterizationStateCreateInfo::default().polygon_mode(vk::PolygonMode::FILL)
    }

    fn conservative_overestimate_state()
    -> vk::PipelineRasterizationConservativeStateCreateInfoEXT<'static> {
        vk::PipelineRasterizationConservativeStateCreateInfoEXT::default()
            .conservative_rasterization_mode(vk::ConservativeRasterizationModeEXT::OVERESTIMATE)
    }

    fn accepted(
        fill: &vk::PipelineRasterizationStateCreateInfo<'_>,
        state: &mut vk::PipelineRasterizationConservativeStateCreateInfoEXT<'_>,
        stages: usize,
        partner: bool,
    ) -> bool {
        let node = std::ptr::from_ref(&*state).cast();
        let exact = exact_overestimate_state(state);
        let raster = fill.push_next(state);
        require_state_with_conservative(
            Behavior::UnsnappedV3,
            true,
            &raster,
            stages,
            partner,
            Some(node),
            exact,
        )
        .is_ok()
    }
}
