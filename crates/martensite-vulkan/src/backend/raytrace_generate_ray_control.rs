//! Bounded function-output replay of actual inputs, not the original whole fragment program.
use crate::backend::{
    runtime::VulkanRuntime,
    storage_descriptor::{StorageDescriptorLayout, UploadedStorage},
    submission::CommandSubmission,
};
use ash::vk;
use std::{path::Path, sync::Arc};
#[path = "raytrace_generate_ray_inputs.rs"]
mod inputs;
#[path = "raytrace_generate_ray_layout.rs"]
mod layout;
#[path = "raytrace_generate_ray_uniforms.rs"]
mod uniforms;
const RAY_BYTES: usize = 409 * 48 * 4;
#[test]
#[ignore = "requires source-reviewed immutable ray inputs, canonical GPU lease and actual AMD RADV"]
fn actual_uniform_ray_column() {
    let input_root = std::env::var("MARTENSITE_GENERATE_RAY_INPUT_DIR").unwrap();
    let expected = std::env::var("MARTENSITE_GENERATE_RAY_INPUT_SHA256").unwrap();
    let output = std::env::var("MARTENSITE_GENERATE_RAY_OUT").unwrap();
    let directory = Path::new(&output);
    assert!(Path::new(&input_root).is_absolute() && directory.is_absolute());
    let packets =
        inputs::load_inputs(&inputs::PreparedRayDirectory(input_root.into()), &expected).unwrap();
    std::fs::create_dir(directory).unwrap();
    let runtime = super::selected_runtime();
    for packet in packets {
        capture_ray_input(&runtime, directory, &packet, &expected);
    }
    assert!(runtime.validation.errors().unwrap().is_empty());
}
fn capture_ray_input(
    runtime: &Arc<VulkanRuntime>,
    directory: &Path,
    input: &inputs::RayProbeInput,
    manifest: &str,
) {
    let target = directory.join(&input.image);
    std::fs::create_dir(&target).unwrap();
    let uniform = uniforms::UniformInput::new(runtime, &input.uniforms);
    let (output_layout, output) = ray_probe_output(runtime);
    let pipeline = ray_probe_pipeline(runtime, input, &uniform, &output_layout);
    let bytes = execute_ray_probe(runtime, &pipeline, &[uniform.set, output.set], &output);
    std::fs::write(target.join("column.bin"), &bytes).unwrap();
    layout::require_ray_column(&bytes, input, 1);
    super::metadata(runtime, &target);
    input_metadata(&target, input, manifest);
}
fn ray_probe_output(runtime: &Arc<VulkanRuntime>) -> (StorageDescriptorLayout, UploadedStorage) {
    let layout = StorageDescriptorLayout::new(runtime, vk::ShaderStageFlags::FRAGMENT).unwrap();
    let output = UploadedStorage::new(runtime, &layout, &vec![0; RAY_BYTES]).unwrap();
    (layout, output)
}
fn ray_probe_pipeline(
    runtime: &Arc<VulkanRuntime>,
    input: &inputs::RayProbeInput,
    uniform: &uniforms::UniformInput,
    output: &StorageDescriptorLayout,
) -> super::pipeline::PixelPipeline {
    let data = probe_specialization(input, runtime);
    let entries: Vec<_> = (0..12).map(super::pipeline::specialization_entry).collect();
    let shader = include_bytes!("../../shaders/raytrace_controls/generate_ray.frag.spv");
    super::pipeline::PixelPipeline::with_fragment(
        runtime,
        &[uniform.layout, output.handle],
        shader,
        (&data, &entries),
    )
}

fn execute_ray_probe(
    runtime: &Arc<VulkanRuntime>,
    pipeline: &super::pipeline::PixelPipeline,
    sets: &[vk::DescriptorSet],
    output: &UploadedStorage,
) -> Vec<u8> {
    let commands = CommandSubmission::new(runtime).unwrap();
    let extent = vk::Extent2D {
        width: 409,
        height: 409,
    };
    super::begin(runtime, commands.command, extent);
    crate::backend::record::set_viewport(runtime, commands.command, extent);
    pipeline.draw_sets(commands.command, sets);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { runtime.device().cmd_end_rendering(commands.command) };
    super::host_barrier(runtime, commands.command);
    commands.finish().unwrap();
    output.test_readback(RAY_BYTES).unwrap()
}
fn probe_specialization(input: &inputs::RayProbeInput, runtime: &VulkanRuntime) -> Vec<u8> {
    input
        .constants
        .into_iter()
        .chain([1, runtime.attachment_coordinates.leading_rows(409)])
        .flat_map(u32::to_ne_bytes)
        .collect()
}
fn input_metadata(directory: &Path, input: &inputs::RayProbeInput, manifest: &str) {
    let metadata = serde_json::json!({"scope":"source-exact-GenerateRay-diagnostic-caller","qualification":false,
        "image":input.image,"uniform_bytes":input.uniforms.len(),"specialization_words":input.constants,
        "input_manifest_sha256":manifest,"record_words":48,"post_vertex_original_fragment_proof":false,
        "limit":"different caller/compiler context; no original ray/intersection/hit equivalence claim"});
    std::fs::write(
        directory.join("input.json"),
        serde_json::to_vec_pretty(&metadata).unwrap(),
    )
    .unwrap();
}
