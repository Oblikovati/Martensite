//! Explicit ignored executable adapter: stdin is output-directory, declared row, device index.
use super::{
    frame::FrameResources,
    owned_sample_readback::{self, OwnedResult},
    pipeline_variants::RasterPipelines,
    retained_submission::RetirementStore,
    runtime::{VulkanOptions, VulkanRuntime},
    submission::CommandSubmission,
};
use crate::stencil_context::{StencilCompare, StencilOperation};
use crate::{RenderFrame, RenderStencilState};
use ash::vk;
use std::{
    io::Read,
    path::{Path, PathBuf},
    sync::Arc,
};
struct OwnedCaptureSession {
    runtime: Arc<VulkanRuntime>,
    pipelines: RasterPipelines,
    retirements: RetirementStore,
    frame: RenderFrame,
}
#[test]
#[ignore = "requires explicitly scheduled pinned-layer GPU execution and stdin request"]
fn owned_sample_capture() {
    let mut request = String::new();
    std::io::stdin().read_to_string(&mut request).unwrap();
    let (directory, row, device) = parse_request(&request).unwrap();
    run_capture(&directory, &row, device).unwrap();
    println!("OWNED_RAW_CAPTURE=complete");
}
fn parse_request(request: &str) -> OwnedResult<(PathBuf, String, usize)> {
    let fields: Vec<&str> = request.lines().collect();
    if fields.len() != 3 || fields.iter().any(|field| field.is_empty()) {
        return Err(format!(
            "request={request:?}; expected output directory, row, device on3lines"
        )
        .into());
    }
    let directory = PathBuf::from(fields[0]);
    if !directory.is_absolute() {
        return Err(format!("output={directory:?}; expected absolute directory").into());
    }
    Ok((directory, fields[1].to_owned(), fields[2].parse()?))
}
fn run_capture(directory: &Path, row: &str, device: usize) -> OwnedResult<()> {
    let frame = super::owned_sample_inputs::frame(row)?;
    std::fs::create_dir(directory)?;
    let session = OwnedCaptureSession::new(frame, device)?;
    super::owned_sample_metadata::write_driver(directory, row, &session.runtime)?;
    session.capture_phase(directory, "baseline")?;
    session.capture_phase(directory, "observer")?;
    check_validation(directory, &session.runtime)
}
impl OwnedCaptureSession {
    fn new(frame: RenderFrame, device: usize) -> OwnedResult<Self> {
        let options = VulkanOptions {
            validation: true,
            device_index: Some(device),
        };
        let runtime = VulkanRuntime::create_for_owned_v2(options)?;
        require_carrier(&runtime)?;
        let pipelines = RasterPipelines::new(&runtime, true)?;
        let retirements = RetirementStore::new()?;
        Ok(Self {
            runtime,
            pipelines,
            retirements,
            frame,
        })
    }
    fn capture_phase(&self, directory: &Path, phase: &str) -> OwnedResult<()> {
        let resources = self.resources(phase)?;
        require_single_draw(&resources, &self.frame)?;
        require_owned_winding(
            self.runtime.attachment_coordinates,
            resources.draws[0].presentation.mirrored,
        )?;
        let stencil = self.submit(&resources)?;
        self.write_phase(directory, phase, &resources, &stencil)
    }
    fn resources(&self, phase: &str) -> OwnedResult<FrameResources> {
        let alpha = Default::default();
        let stencil = stencil(phase);
        let resources = FrameResources::new(
            &self.runtime,
            &self.frame,
            &self.pipelines,
            alpha,
            stencil,
            &self.retirements,
        )?;
        Ok(resources)
    }
    fn submit(&self, resources: &FrameResources) -> OwnedResult<super::buffer::DeviceBuffer> {
        let stencil = owned_sample_readback::stencil_buffer(&self.runtime)?;
        let commands = CommandSubmission::new(&self.runtime)?;
        super::record::record_frame(&self.runtime, commands.command, resources, &self.frame);
        owned_sample_readback::copy_stencil(&self.runtime, commands.command, resources, &stencil);
        commands.finish()?;
        Ok(stencil)
    }
    fn write_phase(
        &self,
        directory: &Path,
        phase: &str,
        resources: &FrameResources,
        stencil: &super::buffer::DeviceBuffer,
    ) -> OwnedResult<()> {
        let phase_directory = directory.join(phase);
        owned_sample_readback::write_images(&phase_directory, resources, stencil)?;
        self.write_uploads(directory, phase, resources)?;
        let runtime = &self.runtime;
        super::owned_sample_metadata::write_events(
            &phase_directory,
            runtime,
            resources,
            &self.frame,
        )
    }
    fn write_uploads(
        &self,
        directory: &Path,
        phase: &str,
        resources: &FrameResources,
    ) -> OwnedResult<()> {
        let user = resources.draws[0]
            .user_program
            .as_ref()
            .ok_or("missing actual user draw")?;
        user.owned_write_uploads(&directory.join(format!("{phase}-submission")))?;
        Ok(())
    }
}
fn check_validation(directory: &Path, runtime: &VulkanRuntime) -> OwnedResult<()> {
    let messages = runtime
        .validation
        .errors()
        .map_err(|error| format!("validation observation={error:?}"))?;
    let encoded: Vec<String> = messages
        .iter()
        .map(|message| super::owned_sample_json::quote(message))
        .collect();
    std::fs::write(
        directory.join("validation.json"),
        format!("[{}]", encoded.join(",")),
    )?;
    if !messages.is_empty() {
        return Err(format!("validation={messages:?}; expected zero warnings/errors").into());
    }
    Ok(())
}
fn require_carrier(runtime: &VulkanRuntime) -> OwnedResult<()> {
    if runtime.depth.formats.stencil != vk::Format::D32_SFLOAT_S8_UINT {
        return Err(format!(
            "depth={:?}; expected explicit v2 D32F/S8 no fallback",
            runtime.depth.formats
        )
        .into());
    }
    super::image::check_format(runtime, vk::Format::D32_SFLOAT_S8_UINT, true)?;
    let properties = carrier_properties(runtime)?;
    require_single_sample(properties.sample_counts)?;
    Ok(())
}
fn require_single_sample(samples: vk::SampleCountFlags) -> OwnedResult<()> {
    if !samples.contains(vk::SampleCountFlags::TYPE_1) {
        return Err(format!("samples={samples:?}; expected v2 D32F/S8/sample1").into());
    }
    Ok(())
}
fn carrier_properties(runtime: &VulkanRuntime) -> OwnedResult<vk::ImageFormatProperties> {
    let usage = vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC;
    // The queried physical device is the same immutable selection used for allocation.
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
    Ok(properties)
}
fn require_single_draw(resources: &FrameResources, frame: &RenderFrame) -> OwnedResult<()> {
    let extent = resources.attachment_extent;
    require_shape(
        resources.draws.len(),
        frame.layers().len(),
        [extent.width, extent.height],
    )?;
    let correct_depth = resources.depth.format == vk::Format::D32_SFLOAT_S8_UINT;
    if !correct_depth
        || resources.multisample.is_some()
        || resources.depth.samples != vk::SampleCountFlags::TYPE_1
    {
        return Err(format!(
            "depth={:?},samples={:?}; expected v2 D32F/S8/sample1",
            resources.depth.format, resources.depth.samples
        )
        .into());
    }
    Ok(())
}
fn require_owned_winding(
    coordinates: crate::RenderCoordinateConvention,
    selector: bool,
) -> OwnedResult<()> {
    let actual = coordinates.front_face(selector);
    if coordinates != crate::RenderCoordinateConvention::TopLeftLogicalV2
        || actual != vk::FrontFace::CLOCKWISE
    {
        return Err(format!("coordinates={coordinates:?},selector={selector},front={actual:?}; expected v2 TopLeftLogicalV2/CW").into());
    }
    Ok(())
}
fn require_shape(draws: usize, layers: usize, extent: [u32; 2]) -> OwnedResult<()> {
    let actual = (draws, layers, extent);
    if actual != (1, 1, [64, 64]) {
        return Err(format!("draws/layers/extent={actual:?}; expected (1,1,[64,64])").into());
    }
    Ok(())
}
fn stencil(phase: &str) -> RenderStencilState {
    let observer = phase == "observer";
    let pass = if observer {
        StencilOperation::Replace
    } else {
        StencilOperation::Keep
    };
    RenderStencilState {
        enabled: observer,
        compare: StencilCompare::Always,
        reference: u32::from(observer),
        compare_mask: 1,
        write_mask: u32::from(observer),
        operations: [StencilOperation::Keep, StencilOperation::Keep, pass],
    }
}
#[test]
fn owned_request_rejects_missing_fields_and_relative_output() {
    assert!(parse_request("one\ntwo\n").is_err());
    assert!(parse_request("relative\ndirectional-existing\n0\n").is_err());
    assert!(parse_request("/nonexistent-owned-output-unique\ndirectional-existing\n0\n").is_ok());
}
#[test]
fn observer_only_changes_stencil_survival_recording() {
    let baseline = stencil("baseline").native_state_fields();
    let observer = stencil("observer").native_state_fields();
    assert_eq!(baseline, [0, 0x207, 0, 1, 0, 0x1E00, 0x1E00, 0x1E00]);
    assert_eq!(observer, [1, 0x207, 1, 1, 1, 0x1E00, 0x1E00, 0x1E01]);
}
#[test]
fn owned_capture_refuses_extra_draw_layers_and_cropped_extent() {
    assert!(require_shape(1, 1, [64, 64]).is_ok());
    for shape in [
        (2, 1, [64, 64]),
        (1, 2, [64, 64]),
        (1, 1, [63, 64]),
        (0, 1, [64, 64]),
    ] {
        assert!(require_shape(shape.0, shape.1, shape.2).is_err());
    }
}

#[test]
fn owned_carrier_requires_sample_one_without_accepting_multisample_only_support() {
    assert!(require_single_sample(vk::SampleCountFlags::TYPE_1).is_ok());
    assert!(require_single_sample(vk::SampleCountFlags::TYPE_4).is_err());
    assert!(require_single_sample(vk::SampleCountFlags::empty()).is_err());
}

#[test]
fn actual_dynamic_winding_rejects_the_failed_false_selector_mutant() {
    let convention = crate::RenderCoordinateConvention::TopLeftLogicalV2;
    assert!(require_owned_winding(convention, true).is_ok());
    assert!(require_owned_winding(convention, false).is_err());
    assert_eq!(convention.front_face(true), vk::FrontFace::CLOCKWISE);
    assert_eq!(
        convention.front_face(false),
        vk::FrontFace::COUNTER_CLOCKWISE
    );
}
