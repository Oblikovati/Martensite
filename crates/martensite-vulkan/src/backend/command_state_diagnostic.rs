//! Temporary command-argument diagnostics for bug30412; remove after its cause is resolved.
//! These are command inputs, never post-vertex outputs or GPU fragment identities.
use ash::{vk, vk::Handle};
#[path = "lighting_command_diagnostic.rs"]
mod lighting;
pub(super) fn bound(
    location: (vk::CommandBuffer, vk::Pipeline, vk::PipelineLayout),
    set: vk::DescriptorSet,
) {
    lighting::bound(location, set);
}

pub(super) fn frame_start(command: vk::CommandBuffer, frame: &super::frame::FrameResources) {
    let target = frame.render_depth();
    emit_census(command, frame);
    eprintln!(
        "{}",
        frame_arguments_json(
            command.as_raw(),
            target.format.as_raw(),
            target.samples.as_raw(),
            frame.extent,
            frame.attachment_extent
        )
    );
}
fn emit_census(command: vk::CommandBuffer, frame: &super::frame::FrameResources) {
    eprintln!(
        "{}",
        extra_passes_json(
            command.as_raw(),
            frame.draws.len(),
            [
                frame.background.is_some(),
                frame.grid.is_some(),
                frame.raytrace.is_some(),
                frame.oit.is_some(),
                frame.subview_upload.is_some(),
                frame.multisample.is_some()
            ]
        )
    );
}
// Consumers must prove every recorded operation or refuse it; black pixels do not prove absence.
fn extra_passes_json(command: u64, draws: usize, present: [bool; 6]) -> String {
    format!(
        "{{\"kind\":\"frame-operation-census\",\"command\":{command},\"uploaded_draws\":{draws},\"background_grid_raytrace_oit_subview_msaa\":{present:?}}}"
    )
}
fn frame_arguments_json(
    command: u64,
    format: i32,
    samples: u32,
    logical: vk::Extent2D,
    attachment: vk::Extent2D,
) -> String {
    format!(
        "{{\"kind\":\"draw-state-frame\",\"command\":{},\"render_depth_format\":{},\"render_depth_samples\":{},\"logical_extent\":[{},{}],\"attachment_extent\":[{},{}]}}",
        command,
        format,
        samples,
        logical.width,
        logical.height,
        attachment.width,
        attachment.height
    )
}

pub(super) fn viewport(command: vk::CommandBuffer, viewport: vk::Viewport) {
    eprintln!(
        "{{\"kind\":\"draw-state-viewport\",\"command\":{},\"f32_bits\":{:?}}}",
        command.as_raw(),
        viewport_bits(viewport)
    );
}
fn viewport_bits(viewport: vk::Viewport) -> [u32; 6] {
    [
        viewport.x,
        viewport.y,
        viewport.width,
        viewport.height,
        viewport.min_depth,
        viewport.max_depth,
    ]
    .map(f32::to_bits)
}

pub(super) struct DrawRasterArguments {
    pub depth: (bool, bool),
    pub bias: [f32; 2],
    pub enabled: bool,
    pub user: bool,
    pub clipping: [crate::RenderWorldClippingBehavior; 2],
}
pub(super) fn before_draw(
    location: (vk::CommandBuffer, usize, vk::Pipeline),
    transform: &[u8],
    raster: DrawRasterArguments,
    lighting: Option<&super::storage_descriptor::UploadedStorage>,
) {
    let observation = DrawArguments {
        command: location.0.as_raw(),
        ordinal: location.1,
        pipeline: location.2.as_raw(),
        transform,
        depth: [raster.depth.0, raster.depth.1],
        bias: raster.bias.map(f32::to_bits),
        enabled: raster.enabled,
        user: raster.user,
        clipping: raster.clipping,
        lighting: lighting.map(lighting::DescriptorArguments::from_storage),
    };
    eprintln!("{}", observation.json());
}

struct DrawArguments<'a> {
    command: u64,
    ordinal: usize,
    pipeline: u64,
    transform: &'a [u8],
    depth: [bool; 2],
    bias: [u32; 2],
    enabled: bool,
    user: bool,
    clipping: [crate::RenderWorldClippingBehavior; 2],
    lighting: Option<lighting::DescriptorArguments<'a>>,
}
impl DrawArguments<'_> {
    fn json(&self) -> String {
        format!(
            "{{\"kind\":\"draw-state-before-draw\",\"command\":{},\"ordinal\":{},\"pipeline\":{},\"transform_bytes\":{:?},\"depth_test_write\":{:?},\"bias_units_factor_bits\":{:?},\"bias_enabled\":{},\"user_program\":{},\"world_clipping_requested\":\"{:?}\",\"world_clipping_effective\":\"{:?}\",\"post_vertex_observed\":false,\"shader_identity\":\"unknown\",\"lighting_descriptor\":{},\"lighting_binding_status\":\"{}\"}}",
            self.command,
            self.ordinal,
            self.pipeline,
            self.transform,
            self.depth,
            self.bias,
            self.enabled,
            self.user,
            self.clipping[0],
            self.clipping[1],
            lighting::optional_json(self.lighting.as_ref()),
            lighting::binding_status(self.user, self.lighting.is_some())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extra_pass_absence_is_observed_instead_of_inferred_from_color() {
        let empty = extra_passes_json(3, 13, [false; 6]);
        let raytrace = extra_passes_json(3, 13, [false, false, true, false, false, false]);
        assert!(empty.contains("\"uploaded_draws\":13"));
        assert!(empty.contains("[false, false, false, false, false, false]"));
        assert!(raytrace.contains("[false, false, true, false, false, false]"));
    }
    #[test]
    fn actual_negative_height_and_depth_range_are_not_defaulted() {
        let viewport = vk::Viewport::default()
            .x(3.)
            .y(8.)
            .width(11.)
            .height(-7.)
            .min_depth(0.25)
            .max_depth(0.75);
        assert_eq!(
            viewport_bits(viewport),
            [3., 8., 11., -7., 0.25, 0.75].map(f32::to_bits)
        );
    }
    #[test]
    fn multisample_attachment_and_logical_extent_remain_distinct() {
        let logical = vk::Extent2D {
            width: 7,
            height: 11,
        };
        let attachment = vk::Extent2D {
            width: 13,
            height: 19,
        };
        let json = frame_arguments_json(
            91,
            vk::Format::D32_SFLOAT.as_raw(),
            vk::SampleCountFlags::TYPE_4.as_raw(),
            logical,
            attachment,
        );
        assert!(json.contains("\"render_depth_samples\":4"));
        assert!(json.contains("\"render_depth_format\":126"));
        assert!(json.contains("\"logical_extent\":[7,11]"));
        assert!(json.contains("\"attachment_extent\":[13,19]"));
    }
    fn changed_command_arguments() -> DrawArguments<'static> {
        DrawArguments {
            command: 91,
            ordinal: 7,
            pipeline: 53,
            transform: &[1, 0, 255],
            depth: [false, true],
            bias: [(-0.0f32).to_bits(), 1.25f32.to_bits()],
            enabled: false,
            user: true,
            clipping: [
                crate::RenderWorldClippingBehavior::FragmentV1,
                crate::RenderWorldClippingBehavior::RasterClipDistanceV2,
            ],
            lighting: None,
        }
    }
    #[test]
    fn actual_argument_values_are_retained_without_float_json_coercion() {
        let json = changed_command_arguments().json();
        assert!(json.contains("\"command\":91,\"ordinal\":7,\"pipeline\":53"));
        assert!(json.contains("\"transform_bytes\":[1, 0, 255]"));
        assert!(json.contains("\"depth_test_write\":[false, true]"));
        assert!(json.contains("\"bias_units_factor_bits\":[2147483648, 1067450368]"));
        assert!(json.contains("\"bias_enabled\":false,\"user_program\":true"));
        assert!(json.contains("\"post_vertex_observed\":false"));
        assert!(json.contains("\"world_clipping_requested\":\"FragmentV1\""));
        assert!(json.contains("\"world_clipping_effective\":\"RasterClipDistanceV2\""));
    }
    #[test]
    fn absent_lighting_is_explicit_in_the_actual_draw_arguments() {
        let json = changed_command_arguments().json();
        assert!(json.contains("\"lighting_descriptor\":null"));
    }
}
