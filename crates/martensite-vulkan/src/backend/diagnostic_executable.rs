//! Test-only introspection borrows the pipeline of the actual completed ordinary submission.
mod admission;
mod api;
#[cfg(test)]
mod controls;
mod fake_api;
mod fixture;
mod metadata;
mod output;
mod properties;
mod query;
mod raw;
mod raw_output;
mod records;
mod retention_controls;
use super::{VulkanRenderer, runtime::VulkanRuntime};
use crate::{FrameCapture, RenderFrame, VulkanOptions};
use ash::{vk, vk::Handle};
pub(crate) use records::{ExecutableGap, ExecutableModuleInput, ExecutableWitness};

/// Bind admission to one selected device; `profile.physical` is never substituted.
#[derive(Clone, Copy, Debug)]
pub(super) struct CaptureProfile {
    pub physical: vk::PhysicalDevice,
}
/// Own the capture and observed identity; `result.pipeline` is an operation-local API join.
pub(crate) struct CapturedExecutableFrame {
    pub capture: FrameCapture,
    pub witness: Option<ExecutableWitness>,
    pub pipeline: u64,
    pub device: u64,
    pub physical: u64,
    pub modules: Vec<ExecutableModuleInput>,
    pub attachment: CapturedAttachment,
}
/// Retain actual target/convention values; `result.attachment.color` names the API format.
pub(crate) struct CapturedAttachment {
    pub color: i32,
    pub depth: i32,
    pub samples: u32,
    pub raster: crate::RenderRasterPolicy,
    pub coordinates: crate::RenderCoordinateConvention,
}
/// Construct the same renderer with an immutable private profile; `renderer(options, true)?`.
pub(crate) fn renderer(
    options: VulkanOptions,
    enabled: bool,
) -> Result<VulkanRenderer, ExecutableGap> {
    if !enabled {
        return Ok(VulkanRenderer::new(options)?);
    }
    let runtime = VulkanRuntime::create_for_executable(options)?;
    Ok(VulkanRenderer::initialized_renderer(runtime)?)
}
/// One operation family owns its total IR budget; `let budget = CaptureByteBudget::default();`.
#[derive(Default)]
pub(crate) struct CaptureByteBudget {
    used: usize,
}
impl CaptureByteBudget {
    fn charge(&mut self, bytes: usize) -> Result<(), ExecutableGap> {
        let total = self.used.checked_add(bytes).ok_or_else(|| {
            records::contract("aggregate IR", bytes, "nonoverflowing phase bytes")
        })?;
        if total > 16 * 1024 * 1024 {
            return Err(records::contract(
                "aggregate IR",
                total,
                "<=16MiB for this operation family",
            ));
        }
        self.used = total;
        Ok(())
    }
}
/// Query only after real completion/capture; `capture(&mut renderer, &frame, &mut budget)?`.
pub(crate) fn capture(
    renderer: &mut VulkanRenderer,
    frame: &RenderFrame,
    budget: &mut CaptureByteBudget,
) -> Result<CapturedExecutableFrame, ExecutableGap> {
    let (resources, capture) = completed_capture(renderer, frame)?;
    let mut owned = unselected_capture(&renderer.runtime, &resources, capture);
    let selected = match fixture::selected(frame, &resources) {
        Ok(selected) => selected,
        Err(cause) => return Err(captured_gap(owned, cause)),
    };
    owned.pipeline = selected.handle.as_raw();
    let owned = attach_modules(selected, owned)?;
    finish_query(&renderer.runtime, selected.handle, owned, budget)
}
fn captured_gap(frame: CapturedExecutableFrame, cause: ExecutableGap) -> ExecutableGap {
    // The frame and actual creation inputs survive every later query refusal.
    ExecutableGap::CapturedFrame {
        frame: Box::new(frame),
        cause: Box::new(cause),
    }
}
fn attach_modules(
    selected: &super::pipeline::DrawPipeline,
    mut owned: CapturedExecutableFrame,
) -> Result<CapturedExecutableFrame, ExecutableGap> {
    match selected.executable_modules() {
        Ok(modules) => {
            owned.modules = modules;
            Ok(owned)
        }
        Err(cause) => Err(captured_gap(owned, cause)),
    }
}
fn finish_query(
    runtime: &VulkanRuntime,
    pipeline: vk::Pipeline,
    owned: CapturedExecutableFrame,
    budget: &mut CaptureByteBudget,
) -> Result<CapturedExecutableFrame, ExecutableGap> {
    settle_query(owned, query_selected(runtime, pipeline, budget))
}
fn settle_query(
    mut owned: CapturedExecutableFrame,
    outcome: Result<Option<ExecutableWitness>, ExecutableGap>,
) -> Result<CapturedExecutableFrame, ExecutableGap> {
    match outcome {
        Ok(witness) => {
            owned.witness = witness;
            Ok(owned)
        }
        Err(cause) => Err(captured_gap(owned, cause)),
    }
}

fn completed_capture(
    renderer: &mut VulkanRenderer,
    frame: &RenderFrame,
) -> Result<(super::frame::FrameResources, FrameCapture), ExecutableGap> {
    renderer.runtime.lifecycle.admit("render frame")?;
    let outcome = renderer
        .completed_owned_frame(frame)
        .and_then(|resources| resources.capture().map(|capture| (resources, capture)));
    Ok(renderer.runtime.lifecycle.observe(outcome)?)
}
fn unselected_capture(
    runtime: &VulkanRuntime,
    resources: &super::frame::FrameResources,
    capture: FrameCapture,
) -> CapturedExecutableFrame {
    CapturedExecutableFrame {
        capture,
        witness: None,
        pipeline: 0,
        device: runtime.device().handle().as_raw(),
        physical: runtime.physical.as_raw(),
        modules: Vec::new(),
        attachment: captured_attachment(runtime, resources),
    }
}

fn query_selected(
    runtime: &VulkanRuntime,
    pipeline: vk::Pipeline,
    budget: &mut CaptureByteBudget,
) -> Result<Option<ExecutableWitness>, ExecutableGap> {
    let Some(profile) = runtime.executable_profile else {
        return Ok(None);
    };
    require_physical(runtime, profile)?;
    let mut api = api::BoundExecutableApi::new(runtime);
    let outcome = query::capture(&mut api, pipeline, budget);
    observe_query_failure(runtime, &outcome);
    outcome
        .map(Some)
        .map_err(|cause| ExecutableGap::BoundPipeline {
            pipeline: pipeline.as_raw(),
            cause: Box::new(cause),
        })
}
fn require_physical(runtime: &VulkanRuntime, profile: CaptureProfile) -> Result<(), ExecutableGap> {
    if profile.physical != runtime.physical {
        return Err(records::contract(
            "admitted physical",
            (profile.physical, runtime.physical),
            "same actual runtime physical device",
        ));
    }
    Ok(())
}
fn observe_query_failure(
    runtime: &VulkanRuntime,
    outcome: &Result<ExecutableWitness, ExecutableGap>,
) {
    let Err(gap) = outcome else {
        return;
    };
    if let Some((operation, result)) = api_failure(gap) {
        let failure = super::runtime::vk_error(operation, result);
        let _ = runtime.lifecycle.observe::<()>(Err(failure));
    }
}
fn api_failure(gap: &ExecutableGap) -> Option<(&'static str, vk::Result)> {
    match gap {
        ExecutableGap::Api {
            operation, result, ..
        } => Some((operation, *result)),
        ExecutableGap::PartialQuery { cause, .. } | ExecutableGap::BoundPipeline { cause, .. } => {
            api_failure(cause)
        }
        _ => None,
    }
}

/// Preserve raw module/IR bytes under the caller's artifact directory; `retain(&result, path)?`.
pub(crate) fn retain(
    result: &CapturedExecutableFrame,
    directory: &std::path::Path,
) -> std::io::Result<()> {
    std::fs::create_dir_all(directory)?;
    output::retain(result, directory)
}
pub(super) fn admit_profile(
    instance: &ash::Instance,
    physical: vk::PhysicalDevice,
) -> Result<CaptureProfile, ExecutableGap> {
    // The returned profile is owned by this runtime and cannot switch physical devices.
    let profile = admission::query(instance, physical)?;
    println!(
        "{{\"event\":\"executable-capability\",\"physical\":{},\"extension\":true,\"feature\":true}}",
        physical.as_raw()
    );
    Ok(profile)
}
pub(super) fn creation_flags(
    enabled: bool,
    current: vk::PipelineCreateFlags,
) -> vk::PipelineCreateFlags {
    if enabled {
        return current | vk::PipelineCreateFlags::CAPTURE_INTERNAL_REPRESENTATIONS_KHR;
    }
    // All normal constructors keep their original flags, including empty flags.
    current
}

/// Refuse missing module identity rather than truncating a zip; `module_count_gap(&handles, &inputs)`.
pub(super) fn module_count_gap(
    handles: &[vk::ShaderModule],
    inputs: &[&'static [u8]],
) -> ExecutableGap {
    let cause = records::contract(
        "module creation inputs",
        (handles.len(), inputs.len()),
        "one retained input for every actual module",
    );
    let evidence = raw::RawModuleInputs {
        handles: handles.iter().map(|handle| handle.as_raw()).collect(),
        inputs: inputs.to_vec(),
    };
    ExecutableGap::ModuleInputs {
        evidence: Box::new(evidence),
        cause: Box::new(cause),
    }
}

fn captured_attachment(
    runtime: &VulkanRuntime,
    resources: &super::frame::FrameResources,
) -> CapturedAttachment {
    CapturedAttachment {
        color: resources.color.format.as_raw(),
        depth: resources.depth.format.as_raw(),
        samples: resources.color.samples.as_raw(),
        raster: runtime.raster_policy,
        coordinates: runtime.attachment_coordinates,
    }
}

/// Retain the complete owned error evidence without a second budget charge; `retain_gap(error, path)?`.
pub(crate) fn retain_gap(
    gap: &ExecutableGap,
    directory: &std::path::Path,
) -> Result<(), ExecutableGap> {
    output::retain_gap(gap, directory).map_err(|error| {
        records::contract(
            "gap evidence",
            (directory, error),
            "complete owned diagnostic evidence",
        )
    })
}

impl std::fmt::Debug for CapturedExecutableFrame {
    fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        output
            .debug_struct("CapturedExecutableFrame")
            .field("pipeline", &self.pipeline)
            .field("device", &self.device)
            .field("physical", &self.physical)
            .field("extent", &[self.capture.width, self.capture.height])
            .field("rgba_bytes", &self.capture.rgba.len())
            .field("depth_values", &self.capture.depth.len())
            .field("module_count", &self.modules.len())
            .finish()
    }
}
fn missing_witness(gap: &ExecutableGap) -> Option<&ExecutableWitness> {
    match gap {
        ExecutableGap::MissingAssembly(witness) => Some(witness),
        ExecutableGap::PartialQuery { cause, .. }
        | ExecutableGap::BoundPipeline { cause, .. }
        | ExecutableGap::CapturedFrame { cause, .. }
        | ExecutableGap::ModuleInputs { cause, .. } => missing_witness(cause),
        _ => None,
    }
}
