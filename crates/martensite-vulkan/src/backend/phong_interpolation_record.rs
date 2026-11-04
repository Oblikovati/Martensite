//! Window interpolation uses the same viewport as the actual draw.
use super::{pipeline::DrawPipeline, runtime::VulkanRuntime};
use ash::vk;
pub(super) fn push_viewport(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    pipeline: &DrawPipeline,
    draw: &super::frame::UploadedDraw,
    extent: vk::Extent2D,
    bias: [f32; 2],
    bias_enabled: bool,
) {
    if !pipeline.phong_interpolation_shader {
        return;
    }
    let bytes = viewport_payload(runtime, pipeline, draw, extent, bias, bias_enabled);
    push_fragment_payload(runtime, command, pipeline, &bytes);
}
fn viewport_payload(
    runtime: &VulkanRuntime,
    pipeline: &DrawPipeline,
    draw: &super::frame::UploadedDraw,
    extent: vk::Extent2D,
    bias: [f32; 2],
    bias_enabled: bool,
) -> Vec<u8> {
    let coverage = pipeline
        .triangle_grid_coverage_shader
        .then_some(runtime.triangle_coverage);
    let state = v4_fragment_state(
        runtime,
        draw,
        extent,
        bias,
        bias_enabled,
        pipeline.triangle_grid_coverage_shader,
    );
    viewport_bytes(runtime.attachment_coordinates, extent, coverage, state)
}

fn push_fragment_payload(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    pipeline: &DrawPipeline,
    bytes: &[u8],
) {
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_push_constants(
            command,
            pipeline.layout,
            vk::ShaderStageFlags::FRAGMENT,
            64,
            bytes,
        );
    }
}

pub(super) fn push_text_grid_viewport(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    pipeline: &DrawPipeline,
    extent: vk::Extent2D,
) {
    if !pipeline.text_grid_interpolation_shader {
        return;
    }
    let bytes = viewport_bytes(runtime.attachment_coordinates, extent, None, None);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_push_constants(
            command,
            pipeline.layout,
            pipeline.push_stages,
            80,
            &bytes,
        );
    }
}

fn viewport_bytes(
    coordinates: crate::RenderCoordinateConvention,
    extent: vk::Extent2D,
    coverage: Option<crate::RenderTriangleCoverageBehavior>,
    v4_state: Option<V4FragmentState>,
) -> Vec<u8> {
    let mut bytes: Vec<_> = super::phong_interpolation_upload::viewport(coordinates, extent)
        .into_iter()
        .flat_map(f32::to_ne_bytes)
        .collect();
    if let Some(origin) = coverage_origin(coverage) {
        bytes.extend(origin.to_ne_bytes());
    }
    if let Some(state) = v4_state {
        state.append_bytes(&mut bytes);
    }
    bytes
}

#[derive(Clone, Copy)]
struct V4FragmentState {
    front_face_ccw: u32,
    cull_mode: u32,
    negative_one_to_one: u32,
    depth_minimum: f32,
    depth_maximum: f32,
    bias_factor: f32,
    bias_units: f32,
    bias_unit_scale: f32,
}

impl V4FragmentState {
    fn from_draw(
        runtime: &VulkanRuntime,
        draw: &super::frame::UploadedDraw,
        viewport: vk::Viewport,
        [bias_units, bias_factor]: [f32; 2],
    ) -> Self {
        Self {
            front_face_ccw: v4_front_face_ccw(
                runtime.attachment_coordinates,
                draw.presentation.mirrored,
            ),
            cull_mode: cull_code(draw.presentation.face_culling),
            negative_one_to_one: u32::from(runtime.depth.clip_control),
            depth_minimum: viewport.min_depth,
            depth_maximum: viewport.max_depth,
            bias_factor,
            bias_units,
            bias_unit_scale: native24_unit_scale(runtime.depth_bias_behavior),
        }
    }

    fn append_bytes(self, output: &mut Vec<u8>) {
        output.extend(self.front_face_ccw.to_ne_bytes());
        output.extend(self.cull_mode.to_ne_bytes());
        output.extend(self.negative_one_to_one.to_ne_bytes());
        for value in [
            self.depth_minimum,
            self.depth_maximum,
            self.bias_factor,
            self.bias_units,
            self.bias_unit_scale,
        ] {
            output.extend(value.to_ne_bytes());
        }
    }
}

fn v4_front_face_ccw(coordinates: crate::RenderCoordinateConvention, mirrored: bool) -> u32 {
    u32::from(coordinates.front_face(mirrored) == vk::FrontFace::COUNTER_CLOCKWISE)
}

fn selected_bias(enabled: bool, [units, factor]: [f32; 2]) -> [f32; 2] {
    if enabled { [units, factor] } else { [0., 0.] }
}

fn v4_fragment_state(
    runtime: &VulkanRuntime,
    draw: &super::frame::UploadedDraw,
    extent: vk::Extent2D,
    bias: [f32; 2],
    enabled: bool,
    selected_coverage_pipeline: bool,
) -> Option<V4FragmentState> {
    if !super::fixed_grid_record::logical_facing_active(
        runtime.triangle_coverage,
        selected_coverage_pipeline,
    ) {
        return None;
    }
    let viewport = viewport_state(runtime, extent);
    Some(V4FragmentState::from_draw(
        runtime,
        draw,
        viewport,
        selected_bias(enabled, bias),
    ))
}

fn viewport_state(runtime: &VulkanRuntime, extent: vk::Extent2D) -> vk::Viewport {
    let full = vk::Rect2D {
        offset: vk::Offset2D::default(),
        extent,
    };
    runtime
        .attachment_coordinates
        .raster_regions(extent, full, full)
        .viewport
}

fn native24_unit_scale(behavior: crate::RenderDepthBiasBehavior) -> f32 {
    if behavior == crate::RenderDepthBiasBehavior::Native24UnitsV2 {
        1. / 8_388_608.
    } else {
        0.
    }
}

fn cull_code(culling: crate::RenderFaceCulling) -> u32 {
    match culling {
        crate::RenderFaceCulling::None => 0,
        crate::RenderFaceCulling::Back => 1,
        crate::RenderFaceCulling::Front => 2,
    }
}

fn coverage_origin(coverage: Option<crate::RenderTriangleCoverageBehavior>) -> Option<u32> {
    match coverage? {
        crate::RenderTriangleCoverageBehavior::HardwareV1 => None,
        crate::RenderTriangleCoverageBehavior::FixedGrid12V2 { origin }
        | crate::RenderTriangleCoverageBehavior::FixedGrid12V3 { origin }
        | crate::RenderTriangleCoverageBehavior::NativeGrid12V4 { origin } => Some(match origin {
            crate::RenderTriangleGridOrigin::BottomUp => 0,
            crate::RenderTriangleGridOrigin::TopDown => 1,
        }),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fragment_push_bytes_are_the_actual_selected_viewport() {
        let coordinates = crate::RenderCoordinateConvention::default();
        let extent = vk::Extent2D {
            width: 409,
            height: 311,
        };
        let expected: Vec<_> =
            super::super::phong_interpolation_upload::viewport(coordinates, extent)
                .into_iter()
                .flat_map(f32::to_ne_bytes)
                .collect();
        assert_eq!(viewport_bytes(coordinates, extent, None, None), expected);
        assert_eq!(expected.len(), 16);
    }

    #[test]
    fn coverage_viewport_push_appends_the_declared_origin_word() {
        assert_eq!(
            coverage_origin_payload(),
            expected_coverage_origin_payload()
        );
    }

    fn coverage_origin_payload() -> Vec<u8> {
        viewport_bytes(
            crate::RenderCoordinateConvention::default(),
            vk::Extent2D {
                width: 4,
                height: 5,
            },
            Some(crate::RenderTriangleCoverageBehavior::FixedGrid12V2 {
                origin: crate::RenderTriangleGridOrigin::TopDown,
            }),
            None,
        )
    }

    fn expected_coverage_origin_payload() -> Vec<u8> {
        let mut bytes = viewport_bytes(
            crate::RenderCoordinateConvention::default(),
            vk::Extent2D {
                width: 4,
                height: 5,
            },
            None,
            None,
        );
        bytes.extend(1u32.to_ne_bytes());
        bytes
    }

    #[test]
    fn v4_payload_encodes_facing_culling_depth_range_and_bias() {
        let mut bytes = Vec::new();
        v4_test_state().append_bytes(&mut bytes);
        assert_v4_payload_fields(&bytes);
    }

    fn v4_test_state() -> V4FragmentState {
        V4FragmentState {
            front_face_ccw: 1,
            cull_mode: 2,
            negative_one_to_one: 0,
            depth_minimum: 0.,
            depth_maximum: 1.,
            bias_factor: 1.5,
            bias_units: -2.,
            bias_unit_scale: 1. / 8_388_608.,
        }
    }

    fn assert_v4_payload_fields(bytes: &[u8]) {
        assert_eq!(bytes.len(), 32);
        assert_eq!(
            &bytes[..12],
            &[1u32.to_ne_bytes(), 2u32.to_ne_bytes(), 0u32.to_ne_bytes()].concat()
        );
        assert_eq!(f32::from_ne_bytes(bytes[20..24].try_into().unwrap()), 1.5);
        assert_eq!(f32::from_ne_bytes(bytes[24..28].try_into().unwrap()), -2.);
        assert_eq!(
            f32::from_ne_bytes(bytes[28..32].try_into().unwrap()),
            1. / 8_388_608.
        );
    }

    #[test]
    fn text_viewport_payload_keeps_the_existing_four_float_abi() {
        assert_eq!(text_viewport_payload(), expected_text_viewport_payload());
    }

    fn text_viewport_payload() -> Vec<u8> {
        viewport_bytes(
            crate::RenderCoordinateConvention::default(),
            vk::Extent2D {
                width: 17,
                height: 23,
            },
            None,
            None,
        )
    }

    fn expected_text_viewport_payload() -> Vec<u8> {
        super::super::phong_interpolation_upload::viewport(
            crate::RenderCoordinateConvention::default(),
            vk::Extent2D {
                width: 17,
                height: 23,
            },
        )
        .into_iter()
        .flat_map(f32::to_ne_bytes)
        .collect()
    }
}
