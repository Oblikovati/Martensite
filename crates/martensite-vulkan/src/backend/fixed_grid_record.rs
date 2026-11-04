//! Expanded point coverage restores the caller's dynamic viewport and facing state.
use super::{frame::UploadedDraw, runtime::VulkanRuntime};
use ash::vk;

pub(super) fn fixed_grid_draw_culling(
    draw: &UploadedDraw,
    logical_facing: bool,
) -> (crate::RenderFaceCulling, bool) {
    selected_culling(
        draw.presentation.face_culling,
        draw.presentation.mirrored,
        draw.point_viewports.is_some(),
        logical_facing,
    )
}

pub(super) fn logical_facing_active(
    coverage: crate::RenderTriangleCoverageBehavior,
    selected_pipeline: bool,
) -> bool {
    selected_pipeline
        && matches!(
            coverage,
            crate::RenderTriangleCoverageBehavior::NativeGrid12V4 { .. }
        )
}

fn selected_culling(
    culling: crate::RenderFaceCulling,
    mirrored: bool,
    expanded_point: bool,
    logical_facing: bool,
) -> (crate::RenderFaceCulling, bool) {
    if expanded_point || logical_facing {
        return (crate::RenderFaceCulling::None, false);
    }
    (culling, mirrored)
}

pub(super) fn begin_fixed_grid_draw(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    draw: &UploadedDraw,
) {
    let Some(viewports) = draw.point_viewports else {
        return;
    };
    super::fixed_grid_viewport::record_viewport(
        runtime,
        command,
        viewports[1],
        super::fixed_grid_viewport::coverage_front_face(viewports[0]),
    );
}

pub(super) fn restore_fixed_grid_draw(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    draw: &UploadedDraw,
    mirrored: bool,
) {
    let Some(viewports) = draw.point_viewports else {
        return;
    };
    super::fixed_grid_viewport::record_viewport(
        runtime,
        command,
        viewports[0],
        runtime.attachment_coordinates.front_face(mirrored),
    );
}

pub(super) fn fixed_grid_bias_enabled(
    expanded_point: bool,
    ordinary_bias: bool,
    shader_depth: bool,
) -> bool {
    !expanded_point && !shader_depth && ordinary_bias
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn v4_disables_hardware_culling_for_shader_logical_facing() {
        assert_eq!(
            selected_culling(crate::RenderFaceCulling::Back, true, false, true),
            (crate::RenderFaceCulling::None, false)
        );
    }

    #[test]
    fn logical_facing_is_limited_to_draws_with_the_selected_coverage_shader() {
        let v4 = crate::RenderTriangleCoverageBehavior::NativeGrid12V4 {
            origin: crate::RenderTriangleGridOrigin::TopDown,
        };
        let v2 = crate::RenderTriangleCoverageBehavior::FixedGrid12V2 {
            origin: crate::RenderTriangleGridOrigin::TopDown,
        };
        assert!(logical_facing_active(v4, true));
        assert!(!logical_facing_active(v4, false));
        assert!(!logical_facing_active(v2, true));
    }

    #[test]
    fn legacy_culling_and_expanded_point_culling_keep_existing_routes() {
        assert_eq!(
            selected_culling(crate::RenderFaceCulling::Back, true, false, false),
            (crate::RenderFaceCulling::Back, true)
        );
        assert_eq!(
            selected_culling(crate::RenderFaceCulling::Back, true, true, false),
            (crate::RenderFaceCulling::None, false)
        );
    }

    #[test]
    fn shader_written_v4_depth_disables_fixed_function_bias() {
        assert!(!fixed_grid_bias_enabled(false, true, true));
        assert!(!fixed_grid_bias_enabled(false, false, true));
    }

    #[test]
    fn semantic_point_expansion_does_not_adopt_polygon_or_shadow_bias() {
        assert!(!fixed_grid_bias_enabled(true, true, false));
        assert!(!fixed_grid_bias_enabled(true, false, false));
        assert!(fixed_grid_bias_enabled(false, true, false));
        assert!(!fixed_grid_bias_enabled(false, false, false));
    }
}
