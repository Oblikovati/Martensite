//! Per-submission first-hit storage survives until the real fence and owned frame capture.
use super::raytrace_hit_records::{
    HIT_STORAGE_BYTES, initial_hit_storage, require_hit_records, require_hit_storage,
};
use super::{
    runtime::VulkanRuntime,
    storage_descriptor::{StorageDescriptorLayout, UploadedStorage},
};
use crate::{RenderError, RenderFrame, error::rejected};
use ash::{vk, vk::Handle};
use std::{io::Write, sync::Arc};
pub(super) struct RayHitFrame {
    _layout: StorageDescriptorLayout,
    pub storage: UploadedStorage,
    token: Arc<crate::raytrace_input_observation::RayInputObservation>,
    control: super::raytrace_hit_control::RayHitControl,
}
impl RayHitFrame {
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        frame: &RenderFrame,
        resources: &super::frame::FrameResources,
    ) -> Result<Option<Self>, RenderError> {
        let Some(control) = &runtime.ray_hit_control else {
            return Ok(None);
        };
        super::raytrace_hit_scope::require_hit_frame(runtime, frame, resources)?;
        let token = frame
            .raytrace_observation
            .clone()
            .ok_or_else(|| rejected("missing ray input token", "observed actual frame"))?;
        Self::allocate(runtime, token, control.clone()).map(Some)
    }
    fn allocate(
        runtime: &Arc<VulkanRuntime>,
        token: Arc<crate::raytrace_input_observation::RayInputObservation>,
        control: super::raytrace_hit_control::RayHitControl,
    ) -> Result<Self, RenderError> {
        let layout = StorageDescriptorLayout::new(runtime, vk::ShaderStageFlags::FRAGMENT)?;
        let storage = UploadedStorage::new(runtime, &layout, &initial_hit_storage(control.mode)?)?;
        Ok(Self {
            _layout: layout,
            storage,
            token,
            control,
        })
    }
    pub fn bind(
        &self,
        runtime: &VulkanRuntime,
        command: vk::CommandBuffer,
        layout: vk::PipelineLayout,
    ) {
        unsafe {
            runtime.device().cmd_bind_descriptor_sets(
                command,
                vk::PipelineBindPoint::GRAPHICS,
                layout,
                2,
                &[self.storage.set],
                &[],
            )
        };
        eprintln!("{}", self.binding_marker(command.as_raw(), layout.as_raw()));
    }
    fn binding_marker(&self, command: u64, layout: u64) -> String {
        format!(
            "{{\"kind\":\"ray-hit-set-binding\",\"view_address\":{},\"view_id\":{},\"frame_generation\":{},\"command\":{command},\"layout\":{layout},\"hit_set\":{},\"mode\":{}}}",
            self.token.owner,
            self.token.view_id,
            self.token.generation,
            self.storage.set.as_raw(),
            self.control.mode
        )
    }
    pub fn completed(&self) -> Result<(), RenderError> {
        self.control.require_directory()?;
        let bytes = self.storage.read_after_fence(HIT_STORAGE_BYTES)?;
        let name = format!(
            "{}-{}-{}",
            self.token.owner, self.token.view_id, self.token.generation
        );
        let target = self.control.directory.join(name);
        std::fs::create_dir(&target)
            .map_err(|e| rejected(e, "fresh unique completed-frame directory"))?;
        exclusive_hit_write(&target.join("raw-storage.bin"), &bytes)?;
        let rows = require_hit_storage(&bytes, self.control.mode)?;
        exclusive_hit_write(&target.join("column.bin"), rows)?;
        require_hit_records(rows, self.control.mode)?;
        exclusive_hit_write(&target.join("frame.json"), self.metadata().as_bytes())
    }
    fn metadata(&self) -> String {
        format!(
            "{{\"scope\":\"completed-frame-first-hit\",\"qualification\":false,\"accepted_publication\":false,\"mode\":{},\"view_address\":{},\"view_id\":{},\"frame_generation\":{},\"record_words\":32}}",
            self.control.mode, self.token.owner, self.token.view_id, self.token.generation
        )
    }
}
fn exclusive_hit_write(path: &std::path::Path, bytes: &[u8]) -> Result<(), RenderError> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| rejected(e, "exclusive owned diagnostic file"))?;
    file.write_all(bytes)
        .map_err(|e| rejected(e, "complete diagnostic bytes"))
}

pub(super) fn host_barrier(runtime: &VulkanRuntime, command: vk::CommandBuffer) {
    let barriers = [vk::MemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::FRAGMENT_SHADER)
        .src_access_mask(vk::AccessFlags2::SHADER_STORAGE_WRITE)
        .dst_stage_mask(vk::PipelineStageFlags2::HOST)
        .dst_access_mask(vk::AccessFlags2::HOST_READ)];
    unsafe {
        runtime.device().cmd_pipeline_barrier2(
            command,
            &vk::DependencyInfo::default().memory_barriers(&barriers),
        )
    };
}
