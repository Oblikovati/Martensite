//! Queried device state is separated from production record-input derivations.
use super::owned_sample_json::{numeric as n, object, quote};
use super::owned_sample_readback::OwnedResult;
use super::{frame::FrameResources, runtime::VulkanRuntime};
use crate::RenderFrame;
use ash::vk;
use std::path::Path;

pub(super) fn write_driver(
    directory: &Path,
    row: &str,
    runtime: &VulkanRuntime,
) -> OwnedResult<()> {
    let json = object(&[
        ("row_id", quote(row)),
        ("fixture_version", n(2)),
        ("carrier_selection", carrier_selection(runtime)),
        ("depth_carrier", quote("D32_SFLOAT_S8_UINT")),
        ("driver", device_identity(runtime)),
        ("depth_format_features", n(depth_features(runtime))),
        ("depth_image_support", image_support(runtime)?),
        ("id_properties", id_properties(runtime)),
    ]);
    std::fs::write(directory.join("capture.json"), json)?;
    Ok(())
}
fn carrier_selection(runtime: &VulkanRuntime) -> String {
    let before = runtime
        .owned_v2_original_depth
        .expect("owned v2 constructor records selection before publication");
    object(&[
        ("original_plain", n(before.formats.plain.as_raw())),
        ("original_stencil", n(before.formats.stencil.as_raw())),
        ("original_clip_control", n(before.clip_control)),
        ("selected_plain", n(runtime.depth.formats.plain.as_raw())),
        (
            "selected_stencil",
            n(runtime.depth.formats.stencil.as_raw()),
        ),
        ("selected_clip_control", n(runtime.depth.clip_control)),
    ])
}
pub(super) fn device_identity(runtime: &VulkanRuntime) -> String {
    let identity =
        super::driver_identity::query_driver_identity(&runtime.instance, runtime.physical);
    object(&[
        ("vendor_id", n(identity.vendor_id)),
        ("device_id", n(identity.device_id)),
        ("driver_id", n(identity.driver_id)),
        ("driver_version", n(identity.driver_version)),
        ("device_name", quote(&identity.device_name)),
        ("driver_name", quote(&identity.driver_name)),
        ("driver_info", quote(&identity.driver_info)),
    ])
}
pub(super) fn id_properties(runtime: &VulkanRuntime) -> String {
    let mut identifiers = vk::PhysicalDeviceIDProperties::default();
    let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut identifiers);
    // All output structures and pNext links remain live through this selected-device query.
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    unsafe {
        runtime
            .instance
            .get_physical_device_properties2(runtime.physical, &mut properties);
    }
    let api_version = properties.properties.api_version;
    object(&[
        ("device_uuid", n(identifiers.device_uuid)),
        ("driver_uuid", n(identifiers.driver_uuid)),
        ("api_version", n(api_version)),
        (
            "subpixel_precision_bits",
            n(runtime.limits.sub_pixel_precision_bits),
        ),
    ])
}
fn depth_features(runtime: &VulkanRuntime) -> u32 {
    // Allocation separately requires support for this same carrier and sample count.
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    let properties = unsafe {
        runtime
            .instance
            .get_physical_device_format_properties(runtime.physical, vk::Format::D32_SFLOAT_S8_UINT)
    };
    properties.optimal_tiling_features.as_raw()
}
fn image_support(runtime: &VulkanRuntime) -> OwnedResult<String> {
    let usage = vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC;
    // SAFETY: the instance, physical device and device handles stay valid while the runtime holds them, and the call only reads through out-parameters it owns.
    let properties = unsafe {
        runtime
            .instance
            .get_physical_device_image_format_properties(
                runtime.physical,
                vk::Format::D32_SFLOAT_S8_UINT,
                vk::ImageType::TYPE_2D,
                vk::ImageTiling::OPTIMAL,
                usage,
                vk::ImageCreateFlags::empty(),
            )
    }?;
    Ok(encode_image_support(properties))
}
fn encode_image_support(properties: vk::ImageFormatProperties) -> String {
    let extent = properties.max_extent;
    object(&[
        ("max_extent", n([extent.width, extent.height, extent.depth])),
        ("max_mip_levels", n(properties.max_mip_levels)),
        ("max_array_layers", n(properties.max_array_layers)),
        ("sample_counts", n(properties.sample_counts.as_raw())),
        ("max_resource_size", n(properties.max_resource_size)),
    ])
}
pub(super) fn write_events(
    directory: &Path,
    runtime: &VulkanRuntime,
    resources: &FrameResources,
    frame: &RenderFrame,
) -> OwnedResult<()> {
    let json = object(&[
        (
            "observation_kind",
            quote("production-record-inputs-after-completed-submit"),
        ),
        ("sequence", event_sequence()),
        ("raster", raster_inputs(runtime, resources)),
        ("draw", draw_inputs(runtime, resources)),
        ("layer", layer_inputs(resources, frame)),
        ("readback", readback_semantics()),
    ]);
    std::fs::write(directory.join("events.json"), json)?;
    Ok(())
}
fn event_sequence() -> String {
    let events = [
        "begin-clearing-rendering",
        "apply-dynamic-state",
        "draw-indexed",
        "end-rendering",
        "transition-both-aspects-transfer-src",
        "copy-color-depth",
        "copy-stencil",
        "transfer-host-barrier",
        "completed-submit-fence",
        "host-read",
    ];
    let encoded: Vec<String> = events.iter().map(|event| quote(event)).collect();
    format!("[{}]", encoded.join(","))
}
fn raster_inputs(runtime: &VulkanRuntime, resources: &FrameResources) -> String {
    let full = vk::Rect2D {
        offset: vk::Offset2D::default(),
        extent: resources.extent,
    };
    let region = runtime
        .attachment_coordinates
        .raster_regions(resources.extent, full, full);
    let v = region.viewport;
    let s = region.scissor;
    let viewport = n([v.x, v.y, v.width, v.height, v.min_depth, v.max_depth]);
    let scissor = scissor_fields(s);
    object(&[
        ("viewport", viewport),
        ("scissor", scissor),
        ("sample_location", n([0.5, 0.5])),
    ])
}
fn scissor_fields(s: vk::Rect2D) -> String {
    let origin = [s.offset.x, s.offset.y];
    let extent = [s.extent.width as i32, s.extent.height as i32];
    n([origin[0], origin[1], extent[0], extent[1]])
}
fn draw_inputs(runtime: &VulkanRuntime, resources: &FrameResources) -> String {
    let draw = &resources.draws[0];
    let stencil = resources.stencil.during[0].native_state_fields();
    let clear = resources.stencil.clear_before[0];
    object(&[
        ("stencil", n(stencil)),
        ("stencil_clear_before", n(clear)),
        ("indexed_arguments", n([draw.count, 1, 0, 0, 0])),
        ("raster", draw_raster_inputs(runtime, resources)),
    ])
}
fn draw_raster_inputs(runtime: &VulkanRuntime, resources: &FrameResources) -> String {
    let draw = &resources.draws[0].presentation;
    let front = runtime
        .attachment_coordinates
        .front_face(draw.mirrored)
        .as_raw();
    let cull = format!("{:?}", draw.face_culling);
    let offset = format!("{:?}", draw.polygon_offset);
    object(&[
        ("front_face", n(front)),
        ("per_draw_raster_selector", n(draw.mirrored)),
        ("cull_mode", quote(&cull)),
        ("polygon_offset", quote(&offset)),
    ])
}

fn layer_inputs(resources: &FrameResources, frame: &RenderFrame) -> String {
    let layer = &frame.layers()[0];
    object(&[
        ("depth_test", n(layer.depth_test)),
        ("depth_write", n(layer.depth_write)),
        ("clear_depth", n(layer.clear_depth)),
        ("color_clear", n(frame.clear_color())),
        ("stencil_clear", n(resources.stencil_clear_value)),
        ("depth_clear_value", n(1.0)),
        ("color_format", n(resources.color.format.as_raw())),
        ("depth_format", n(resources.depth.format.as_raw())),
        ("layers", n(frame.layers().len())),
        ("later_depth_clear_count", n(0)),
    ])
}
fn readback_semantics() -> String {
    object(&[
        (
            "depth_transfer",
            quote(
                "raw exact IEEE f32 depth bits; identical logical f32bits.u32le, no mask or quantization",
            ),
        ),
        (
            "color_readback",
            quote("RGBA8 top-left attachment rows; sRGB encoded"),
        ),
        (
            "stencil_readback",
            quote("all4096 STENCIL-aspect u8 samples, no crop"),
        ),
    ])
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn readback_and_event_contract_never_claims_post_vertex_gpu_query() {
        assert!(readback_semantics().contains("no mask or quantization"));
        assert!(event_sequence().contains("completed-submit-fence"));
        assert!(!event_sequence().contains("post-vertex-query"));
    }
    #[test]
    fn image_support_serializes_limits_and_sample_count() {
        let properties = vk::ImageFormatProperties {
            max_extent: vk::Extent3D {
                width: 64,
                height: 64,
                depth: 1,
            },
            max_mip_levels: 1,
            max_array_layers: 1,
            sample_counts: vk::SampleCountFlags::TYPE_1,
            max_resource_size: 4096,
        };
        let json = encode_image_support(properties);
        assert!(json.contains("\"max_extent\":[64, 64, 1]"));
        assert!(json.contains("\"sample_counts\":1"));
    }
}
