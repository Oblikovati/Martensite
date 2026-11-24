//! An RGB dump packs the frame's colour rows on the GPU and copies them into the caller's
//! rows, without an RGBA capture on the host (#594). The colour image is copied into a
//! device-local buffer, `rgb_pack` writes word-aligned RGB rows into a host buffer, and the
//! host hands each row to the caller.
use super::{
    VulkanRenderer, WorkPhase,
    frame_readback::{FrameReadback, ReadbackBuffers},
    rgb_pack::{RgbPackPipeline, RgbPackRows},
    submission::CommandSubmission,
};
use crate::RenderError;
use ash::vk;

impl VulkanRenderer {
    /// Read the RGB rows of a completed frame into `write_row(y, rgb)`; `flip` reverses the
    /// rows for a flipped output. Returns whether every row was written;
    /// `renderer.read_frame_rgb(&readback, false, &mut |y, row| copy(y, row))?`.
    pub(crate) fn read_frame_rgb(
        &mut self,
        readback: &FrameReadback,
        flip: bool,
        write_row: &mut dyn FnMut(usize, &[u8]) -> bool,
    ) -> Result<bool, RenderError> {
        self.runtime.lifecycle.admit("read frame")?;
        // The thread may hold the frame being read; settling also reports its error (#594).
        self.settle_presentation()?;
        let outcome = self.read_admitted_rgb(readback, flip, write_row);
        self.runtime.lifecycle.observe(outcome)
    }
    fn read_admitted_rgb(
        &mut self,
        readback: &FrameReadback,
        flip: bool,
        write_row: &mut dyn FnMut(usize, &[u8]) -> bool,
    ) -> Result<bool, RenderError> {
        let _readback = self.runtime.work.enter(WorkPhase::Readback);
        if self.rgb_pack.is_none() {
            self.rgb_pack = Some(RgbPackPipeline::new(&self.runtime)?);
        }
        let rows = readback.rgb_rows(flip);
        let source = self
            .rgb_pack_sources
            .take(&self.runtime, readback.attachment_bytes() as u64)?;
        let packed = self
            .readback_buffers
            .take(&self.runtime, rows.packed_bytes() as u64)?;
        let commands = CommandSubmission::new(&self.runtime)?;
        self.record_rgb_pack(
            commands.command,
            readback,
            [source.handle, packed.handle],
            rows,
        );
        let buffers = ReadbackBuffers {
            color: packed,
            depth: Some(source),
        };
        if let Err(primary) = commands.finish() {
            return Err(self.keep_failed_rgb(commands, readback, buffers, primary));
        }
        let written = copy_rows(&buffers, rows, write_row);
        self.return_rgb_buffers(buffers);
        written
    }
    fn record_rgb_pack(
        &self,
        command: vk::CommandBuffer,
        readback: &FrameReadback,
        [source, packed]: [vk::Buffer; 2],
        rows: RgbPackRows,
    ) {
        super::record::record_attachment_reads(
            &self.runtime,
            command,
            [Some((&*readback.color, source)), None],
            readback.attachment_extent(),
        );
        memory_barrier(&self.runtime, command, PackStage::Copied);
        if let Some(pack) = &self.rgb_pack {
            pack.record(command, source, packed, rows);
        }
        memory_barrier(&self.runtime, command, PackStage::Packed);
    }
    fn keep_failed_rgb(
        &self,
        commands: CommandSubmission,
        readback: &FrameReadback,
        buffers: ReadbackBuffers,
        primary: RenderError,
    ) -> RenderError {
        if !commands.retirement_incomplete() {
            return primary;
        }
        self.retirements
            .keep_readback(commands, readback.clone(), buffers, primary)
    }
    fn return_rgb_buffers(&self, buffers: ReadbackBuffers) {
        self.readback_buffers.give(buffers.color);
        if let Some(source) = buffers.depth {
            self.rgb_pack_sources.give(source);
        }
    }
}

fn copy_rows(
    buffers: &ReadbackBuffers,
    rows: RgbPackRows,
    write_row: &mut dyn FnMut(usize, &[u8]) -> bool,
) -> Result<bool, RenderError> {
    let (stride, width) = (rows.row_stride(), rows.width as usize * 3);
    buffers.color.with_mapped(rows.packed_bytes(), |bytes| {
        bytes
            .chunks_exact(stride)
            .enumerate()
            .all(|(y, row)| write_row(y, &row[..width]))
    })
}

#[cfg(test)]
#[path = "rgb_readback_tests.rs"]
mod tests;

/// The two points the RGB pack synchronizes: after the colour copy and after the pack.
#[derive(Clone, Copy)]
enum PackStage {
    Copied,
    Packed,
}

fn memory_barrier(
    runtime: &super::runtime::VulkanRuntime,
    command: vk::CommandBuffer,
    stage: PackStage,
) {
    let barrier = match stage {
        PackStage::Copied => vk::MemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::TRANSFER)
            .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
            .dst_access_mask(vk::AccessFlags2::SHADER_STORAGE_READ),
        PackStage::Packed => vk::MemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
            .src_access_mask(vk::AccessFlags2::SHADER_STORAGE_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::HOST)
            .dst_access_mask(vk::AccessFlags2::HOST_READ),
    };
    let barriers = [barrier];
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_pipeline_barrier2(
            command,
            &vk::DependencyInfo::default().memory_barriers(&barriers),
        );
    }
}
