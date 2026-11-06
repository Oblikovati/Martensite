//! Mandatory V3 completion uses existing owned coherent storage and the submission fence.
use super::{
    frame::{FrameResources, UploadedDraw},
    pipeline::DrawPipeline,
    runtime::VulkanRuntime,
    storage_descriptor::UploadedStorage,
    window_depth_status_protocol as protocol,
    window_depth_status_state::InvocationState,
};
use crate::{PrimitiveDraw, RenderError, RenderWindowDepthFailure as Failure};
use ash::vk;
use std::sync::Arc;

pub(super) struct WindowDepthStatus {
    storage: UploadedStorage,
    indices: Vec<u32>,
    reference_tail: Vec<u8>,
    prefix_bytes: usize,
    invocations: InvocationState,
}
impl protocol::StatusReader for UploadedStorage {
    fn read_status(&self, size: usize) -> Result<Vec<u8>, RenderError> {
        self.read_after_fence(size)
    }
}
impl WindowDepthStatus {
    pub(super) fn prepare(
        runtime: &Arc<VulkanRuntime>,
        draw: &PrimitiveDraw,
        pipeline: &DrawPipeline,
    ) -> Result<Option<Self>, RenderError> {
        if !pipeline
            .kind
            .window_depth
            .is_some_and(|p| p.fragment_depth())
        {
            return Ok(None);
        }
        Self::allocate(runtime, draw, pipeline).map(Some)
    }
    fn allocate(
        runtime: &Arc<VulkanRuntime>,
        draw: &PrimitiveDraw,
        pipeline: &DrawPipeline,
    ) -> Result<Self, RenderError> {
        let layout = pipeline
            .window_depth_status_layout
            .as_ref()
            .ok_or_else(|| Failure::UnsupportedInterface {
                actual: "depth pipeline without mandatory status descriptor".into(),
            })?;
        let (bytes, prefix_bytes, reference_tail) = initial_status(runtime, draw, pipeline)?;
        Ok(Self {
            storage: UploadedStorage::new(runtime, layout, &bytes)?,
            indices: draw.indices.clone(),
            reference_tail,
            prefix_bytes,
            invocations: InvocationState::default(),
        })
    }
    fn validate(&self) -> Result<(), RenderError> {
        let expected = self.invocations.expected(self.indices.len() as u32)?;
        protocol::validate(&self.storage, &self.indices, expected)?;
        self.validate_reference_tail()?;
        #[cfg(feature = "command-state-diagnostics")]
        super::replay_resource_diagnostic::status_accepted(
            self.storage.set,
            &self.indices,
            expected,
        );
        Ok(())
    }
    fn validate_reference_tail(&self) -> Result<(), RenderError> {
        if self.reference_tail.is_empty() {
            return Ok(());
        }
        let bytes = self
            .storage
            .read_after_fence(self.prefix_bytes + self.reference_tail.len())?;
        let tail = bytes.get(self.prefix_bytes..).ok_or(Failure::GpuReadback {
            expected_bytes: self.prefix_bytes + self.reference_tail.len(),
            observed_bytes: bytes.len(),
        })?;
        super::clipped_depth_uniforms::require_tail(&self.reference_tail, tail)
    }
    #[cfg(feature = "command-state-diagnostics")]
    fn report_incidence(&self, draw: usize) -> Result<(), RenderError> {
        if self.reference_tail.is_empty() {
            return Ok(());
        }
        let size = self.storage.diagnostic_storage().1.len();
        let bytes = self.storage.read_after_fence(size)?;
        let report = super::clipped_depth_incidence::checked_report(
            draw,
            &bytes,
            self.prefix_bytes,
            &self.reference_tail,
            &self.indices,
        )?;
        eprintln!("{report}");
        Ok(())
    }
    pub(super) fn dispatch(&self, indices: u32, instances: u32) {
        self.invocations
            .dispatch(self.indices.len() as u32, indices, instances);
    }
}
impl super::window_depth_completion::DepthDrawCompletion for WindowDepthStatus {
    fn validate_draw(&self) -> Result<(), RenderError> {
        self.validate()
    }
    #[cfg(feature = "command-state-diagnostics")]
    fn diagnostic_failure(&self, draw: usize) {
        if let Err(error) = self.report_incidence(draw) {
            super::clipped_depth_incidence::unavailable(draw, &error);
        }
    }
}

type InitialWindowDepthStatus = (Vec<u8>, usize, Vec<u8>);
fn initial_status(
    runtime: &VulkanRuntime,
    draw: &PrimitiveDraw,
    pipeline: &DrawPipeline,
) -> Result<InitialWindowDepthStatus, RenderError> {
    require_capacity(draw.indices.len(), runtime.limits.max_storage_buffer_range)?;
    let words = protocol::initial_words(draw.indices.len() / 3)?;
    let mut bytes: Vec<_> = words.into_iter().flat_map(u32::to_ne_bytes).collect();
    let prefix = bytes.len();
    let tail = if pipeline.kind.window_depth.is_some_and(|p| p.clipped_fan()) {
        if pipeline
            .kind
            .window_depth
            .is_some_and(|p| p.shared_incidence())
        {
            super::clipped_depth_uniforms::encode_incidence(draw)?
        } else {
            super::clipped_depth_uniforms::encode(draw)?
        }
    } else {
        Vec::new()
    };
    require_total_capacity(prefix, tail.len(), runtime.limits.max_storage_buffer_range)?;
    bytes.extend_from_slice(&tail);
    #[cfg(feature = "command-state-diagnostics")]
    if !tail.is_empty() {
        let triangles = draw.indices.len() / 3;
        let size = super::clipped_depth_incidence::byte_count(triangles)?;
        require_total_capacity(bytes.len(), size, runtime.limits.max_storage_buffer_range)?;
        bytes.extend(super::clipped_depth_incidence::initial(triangles)?);
    }
    Ok((bytes, prefix, tail))
}

fn require_total_capacity(prefix: usize, tail: usize, maximum: u32) -> Result<(), RenderError> {
    if prefix
        .checked_add(tail)
        .is_some_and(|bytes| bytes <= maximum as usize)
    {
        return Ok(());
    }
    Err(Failure::UnsupportedInterface {
        actual: format!(
            "V4 status prefix={prefix},tail={tail}; expected checked total <= {maximum}"
        ),
    }
    .into())
}

fn require_capacity(indices: usize, maximum: u32) -> Result<(), RenderError> {
    let bytes = indices
        .checked_div(3)
        .and_then(|n| n.checked_mul(2))
        .and_then(|n| n.checked_add(4))
        .and_then(|n| n.checked_mul(4));
    if indices.is_multiple_of(3)
        && u32::try_from(indices).is_ok()
        && bytes.is_some_and(|n| n <= maximum as usize)
    {
        return Ok(());
    }
    Err(Failure::UnsupportedInterface{actual:format!("status indices={indices}, bytes={bytes:?}; expected triangle triples and storage <= {maximum}")}.into())
}

pub(super) fn begin(resources: &FrameResources) {
    for status in resources
        .draws
        .iter()
        .filter_map(|d| d.window_depth_status.as_ref())
    {
        status.invocations.begin();
    }
}
pub(super) fn validate(resources: &FrameResources) -> Result<(), RenderError> {
    super::window_depth_completion::validate(resources.draws.iter().enumerate().map(
        |(draw, loaded)| {
            super::window_depth_completion::SubmittedDepthCompletion {
                draw,
                required: loaded.window_depth_expected,
                actual_pipeline: loaded
                    .pipeline
                    .kind
                    .window_depth
                    .is_some_and(|p| p.fragment_depth()),
                status: loaded
                    .window_depth_status
                    .as_ref()
                    .map(|s| s as &dyn super::window_depth_completion::DepthDrawCompletion),
            }
        },
    ))
}

pub(super) fn bind(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    draw: &UploadedDraw,
    pipeline: &DrawPipeline,
) {
    let Some(status) = &draw.window_depth_status else {
        return;
    };
    // Upload and actual-pipeline admission require the compatible set7 layout.
    super::record_draw_bindings::bind_descriptor(
        runtime,
        command,
        pipeline.layout,
        7,
        status.storage.set,
    );
}
pub(super) fn host_barrier(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
) {
    if !resources
        .draws
        .iter()
        .any(|draw| draw.window_depth_status.is_some())
    {
        return;
    }
    let barrier = vk::MemoryBarrier2::default()
        .src_stage_mask(
            vk::PipelineStageFlags2::GEOMETRY_SHADER | vk::PipelineStageFlags2::FRAGMENT_SHADER,
        )
        .src_access_mask(vk::AccessFlags2::SHADER_STORAGE_WRITE)
        .dst_stage_mask(vk::PipelineStageFlags2::HOST)
        .dst_access_mask(vk::AccessFlags2::HOST_READ);
    // SAFETY: recorded before submission completion; every status allocation remains frame-owned.
    unsafe {
        runtime.device().cmd_pipeline_barrier2(
            command,
            &vk::DependencyInfo::default().memory_barriers(&[barrier]),
        )
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn status_allocation_is_checked_and_bounded_before_initialization() {
        assert!(require_capacity(0, 16).is_ok());
        assert!(require_capacity(6, 32).is_ok());
        for (indices, maximum) in [(6, 31), (1, 24), (usize::MAX, u32::MAX)] {
            assert!(require_capacity(indices, maximum).is_err());
        }
    }
    #[test]
    fn appended_constants_share_the_same_checked_allocation_limit() {
        assert!(require_total_capacity(16, 144, 160).is_ok());
        assert!(require_total_capacity(16, 144, 159).is_err());
        assert!(require_total_capacity(usize::MAX, 144, u32::MAX).is_err());
    }
}
