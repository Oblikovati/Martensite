//! Query and buffer owners follow the frame's existing fence/retirement lifetime.
use super::super::{
    buffer::DeviceBuffer,
    frame::UploadedDraw,
    memory_selection::HostBufferAccess,
    runtime::{VulkanRuntime, vk_error},
};
use crate::{RenderError, VertexOutputRequest};
use ash::{vk, vk::Handle};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

pub(in crate::backend) struct VertexOutputResources {
    command: AtomicU64,
    invocations: super::invocation::InvocationCount,
    runtime: Arc<VulkanRuntime>,
    output: DeviceBuffer,
    query: vk::QueryPool,
    request: Arc<VertexOutputRequest>,
}
struct VertexOutputReadback {
    counts: [u64; 2],
    query_result: Result<(), vk::Result>,
    bytes: Vec<u8>,
}
impl VertexOutputResources {
    pub(in crate::backend) fn new(
        runtime: &Arc<VulkanRuntime>,
        request: Arc<VertexOutputRequest>,
    ) -> Result<Self, RenderError> {
        let output = output_buffer(runtime, request.capacity_bytes)?;
        let query = query_pool(runtime)?;
        Ok(Self {
            command: AtomicU64::new(0),
            invocations: Default::default(),
            runtime: runtime.clone(),
            output,
            query,
            request,
        })
    }
    pub(in crate::backend) fn recorded_invocations(&self) -> usize {
        self.invocations.actual()
    }
    fn extension(&self) -> ash::ext::transform_feedback::Device {
        ash::ext::transform_feedback::Device::new(&self.runtime.instance, self.runtime.device())
    }
    pub(in crate::backend) fn reset(&self, runtime: &VulkanRuntime, command: vk::CommandBuffer) {
        // Reset occurs outside the rendering scope; only this frame owns the pool.
        unsafe {
            runtime
                .device()
                .cmd_reset_query_pool(command, self.query, 0, 1)
        };
    }
    pub(in crate::backend) fn begin(&self, command: vk::CommandBuffer) {
        if !self.invocations.begin() {
            return;
        }
        self.command.store(command.as_raw(), Ordering::Relaxed);
        let extension = self.extension();
        self.bind_output(command, &extension);
        self.begin_query(command, &extension);
        begin_feedback(command, &extension);
    }
    fn bind_output(
        &self,
        command: vk::CommandBuffer,
        extension: &ash::ext::transform_feedback::Device,
    ) {
        unsafe {
            (extension.fp().cmd_bind_transform_feedback_buffers_ext)(
                command,
                0,
                1,
                &self.output.handle,
                &0,
                &(self.request.capacity_bytes as u64),
            );
        }
    }
    fn begin_query(
        &self,
        command: vk::CommandBuffer,
        extension: &ash::ext::transform_feedback::Device,
    ) {
        unsafe {
            (extension.fp().cmd_begin_query_indexed_ext)(
                command,
                self.query,
                0,
                vk::QueryControlFlags::empty(),
                0,
            );
        }
    }
    pub(in crate::backend) fn end(&self, command: vk::CommandBuffer) {
        if !self.invocations.ending() {
            return;
        }
        let extension = self.extension();
        end_feedback(command, &extension);
        unsafe {
            (extension.fp().cmd_end_query_indexed_ext)(command, self.query, 0, 0);
        }
    }
    pub(in crate::backend) fn completed(
        &self,
        ordinal: usize,
        draw: &UploadedDraw,
    ) -> Result<(), RenderError> {
        let readback = self.readback()?;
        self.emit_receipt(ordinal, draw, &readback);
        self.validate_readback(&readback)?;
        self.request.record_capture();
        Ok(())
    }
    fn readback(&self) -> Result<VertexOutputReadback, RenderError> {
        let (counts, query_result) = self.query_results();
        let bytes = self.output.read(self.request.capacity_bytes)?;
        Ok(VertexOutputReadback {
            counts,
            query_result,
            bytes,
        })
    }
    fn query_results(&self) -> ([u64; 2], Result<(), vk::Result>) {
        let mut query = [[0u64; 2]; 1];
        let result = unsafe {
            self.runtime.device().get_query_pool_results(
                self.query,
                0,
                &mut query,
                vk::QueryResultFlags::TYPE_64,
            )
        };
        (query[0], result)
    }
    fn validate_readback(&self, readback: &VertexOutputReadback) -> Result<(), RenderError> {
        readback
            .query_result
            .map_err(|e| vk_error("read fenced vertex-output query", e))?;
        if self.invocations.actual() != 1 {
            return Err(super::unsupported(
                "exactly one original draw invocation",
                self.invocations.actual(),
            ));
        }
        self.request
            .counts(readback.counts[0], readback.counts[1])?;
        self.request.validate_records(&readback.bytes)?;
        Ok(())
    }
    fn emit_receipt(&self, ordinal: usize, draw: &UploadedDraw, readback: &VertexOutputReadback) {
        let VertexOutputReadback {
            counts,
            query_result,
            bytes,
        } = readback;
        // Retain raw records even when counts or the direct-ID join subsequently refuse.
        eprintln!(
            "{{\"kind\":\"actual-vertex-output\",\"source_token\":{},\"command\":{},\"draw_ordinal\":{ordinal},\"pipeline\":{},\"vertex_buffer\":{},\"index_buffer\":{},\"query_status\":\"{query_result:?}\",\"written_needed\":{counts:?},\"stride\":{},\"format\":\"{:?}\",\"native_endian\":true,\"bytes\":{bytes:?}}}",
            self.request.source_token.unwrap(),
            self.command.load(Ordering::Relaxed),
            draw.pipeline.handle.as_raw(),
            draw.vertices.handle.as_raw(),
            draw.indices.handle.as_raw(),
            self.request.format().stride(),
            self.request.format()
        );
    }
}
fn output_buffer(
    runtime: &Arc<VulkanRuntime>,
    capacity_bytes: usize,
) -> Result<DeviceBuffer, RenderError> {
    let output = DeviceBuffer::new(
        runtime,
        capacity_bytes as u64,
        vk::BufferUsageFlags::TRANSFORM_FEEDBACK_BUFFER_EXT,
        HostBufferAccess::Readback,
    )?;
    output.write(&vec![0; capacity_bytes])?;
    Ok(output)
}
fn query_pool(runtime: &VulkanRuntime) -> Result<vk::QueryPool, RenderError> {
    let info = vk::QueryPoolCreateInfo::default()
        .query_count(1)
        .query_type(vk::QueryType::TRANSFORM_FEEDBACK_STREAM_EXT);
    unsafe { runtime.device().create_query_pool(&info, None) }
        .map_err(|e| vk_error("create vertex-output query", e))
}
fn begin_feedback(command: vk::CommandBuffer, extension: &ash::ext::transform_feedback::Device) {
    unsafe {
        (extension.fp().cmd_begin_transform_feedback_ext)(
            command,
            0,
            0,
            std::ptr::null(),
            std::ptr::null(),
        );
    }
}
fn end_feedback(command: vk::CommandBuffer, extension: &ash::ext::transform_feedback::Device) {
    unsafe {
        (extension.fp().cmd_end_transform_feedback_ext)(
            command,
            0,
            0,
            std::ptr::null(),
            std::ptr::null(),
        );
    }
}
impl Drop for VertexOutputResources {
    fn drop(&mut self) {
        // The frame owner cannot drop before fence completion or retained retirement.
        unsafe { self.runtime.device().destroy_query_pool(self.query, None) };
    }
}
