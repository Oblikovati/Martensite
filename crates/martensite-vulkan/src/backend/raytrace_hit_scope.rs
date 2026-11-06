//! Narrow actual frame admission precedes diagnostic pipeline selection and allocation.
use super::{frame::FrameResources, runtime::VulkanRuntime};
use crate::{RenderError, RenderFrame, error::rejected};
use ash::vk;
pub(super) fn require_hit_frame(
    runtime: &VulkanRuntime,
    frame: &RenderFrame,
    resources: &FrameResources,
) -> Result<(), RenderError> {
    let [ordinary, flags] = hit_frame_flags(frame)?;
    let dimensions = require_hit_extents(
        frame.extent(),
        resources.extent,
        resources.attachment_extent,
    );
    let capability = runtime.fragment_stores
        && runtime.attachment_coordinates == crate::RenderCoordinateConvention::TopLeftLogicalV2
        && resources.render_color().samples == vk::SampleCountFlags::TYPE_1;
    require_hit_domain([ordinary, flags, dimensions, capability])
}
fn hit_frame_flags(frame: &RenderFrame) -> Result<[bool; 2], RenderError> {
    let ray = frame
        .raytrace
        .as_ref()
        .ok_or_else(|| rejected("missing raytrace", "ordinary raytrace frame"))?;
    let ordinary = !ray.antialiasing && ray.path_tracing.is_none() && ray.eye_targets == 0;
    let flags = !ray.constants.depth_of_field
        && !ray.constants.textures
        && !frame.flip_output
        && frame.stereo_pair.is_none();
    Ok([ordinary, flags])
}
fn require_hit_extents(logical: [u32; 2], render: vk::Extent2D, attachment: vk::Extent2D) -> bool {
    let render = [render.width, render.height];
    let attachment = [attachment.width, attachment.height];
    logical == [409, 409] && render == [409, 409] && attachment == [409, 410]
}
fn require_hit_domain(conditions: [bool; 4]) -> Result<(), RenderError> {
    if conditions != [true; 4] {
        return Err(rejected(
            format!("ordinary/flags/dimensions/capability={conditions:?}"),
            "ordinary mono no-FSAA/DOF/textures/flip/stereo, actual409x410 TopLeftV2 single-sample with fragment stores",
        ));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn each_domain_requirement_refuses_independently() {
        assert!(require_hit_domain([true; 4]).is_ok());
        for index in 0..4 {
            let mut conditions = [true; 4];
            conditions[index] = false;
            assert!(require_hit_domain(conditions).is_err());
        }
    }
    #[test]
    fn extents_require_the_real_leading_attachment_row() {
        let cases = [
            ([409, 409], [409, 409], [409, 410], true),
            ([409, 409], [409, 409], [409, 409], false),
            ([409, 409], [409, 410], [409, 410], false),
            ([410, 409], [409, 409], [409, 410], false),
        ];
        for (logical, render, attachment, expected) in cases {
            assert_eq!(
                require_hit_extents(logical, extent(render), extent(attachment)),
                expected
            );
        }
    }
    fn extent(dimensions: [u32; 2]) -> vk::Extent2D {
        vk::Extent2D {
            width: dimensions[0],
            height: dimensions[1],
        }
    }
}
