//! Frame-owned buffers and command spans for stable placement compaction.
use super::{PLACEMENTS_PER_GROUP, PlacementVisibilityPipeline};
use crate::backend::{
    draw_table::{COMMAND_BYTES, ENTRY_BYTES},
    host_upload_pages::{FrameUploadPages, UploadRange},
    pipeline::DrawPipeline,
    placement_hiz::{HiZPipeline, HiZPyramid, optional_hiz},
    placement_hiz_history::{HiZHistory, HiZView, HiZWitness},
    storage_descriptor::UploadedStorage,
};
use crate::{RenderError, error::rejected};
use ash::vk;
use std::sync::Arc;

#[path = "placement_hiz_compaction.rs"]
mod hiz;
use hiz::CompactionHiZ;
pub(in crate::backend) use hiz::HiZRequest;

/// Two std430 vec4 AABB corners followed by four indexed-draw words.
pub(super) const CANDIDATE_BYTES: usize = 48;
const COMMAND_WORDS: u32 = COMMAND_BYTES / 4;

struct Segment {
    first: u32,
    count: u32,
    first_count_word: u32,
}

struct CompactionLayout {
    segments: Vec<Segment>,
    size: usize,
}

impl CompactionLayout {
    fn new(count: u32, segments: &[(u32, u32)]) -> Option<Self> {
        let mut command = 0_u32;
        let mut word = count.checked_mul(COMMAND_WORDS)?;
        let mut layout = Vec::with_capacity(segments.len());
        for &(first, count) in segments {
            if first != command || count == 0 {
                return None;
            }
            layout.push(Segment {
                first,
                count,
                first_count_word: word,
            });
            command = command.checked_add(count)?;
            word = word.checked_add(count.div_ceil(PLACEMENTS_PER_GROUP))?;
        }
        if command != count || count == 0 {
            return None;
        }
        Some(Self {
            segments: layout,
            size: usize::try_from(u64::from(word) * 4).ok()?,
        })
    }
}

fn candidate_count(candidates: &[u8], table_size: u64) -> Result<u32, RenderError> {
    let length = candidates.len();
    let count = u32::try_from(length / CANDIDATE_BYTES)
        .ok()
        .filter(|count| *count != 0 && length.is_multiple_of(CANDIDATE_BYTES))
        .filter(|count| u64::from(*count) * ENTRY_BYTES as u64 == table_size)
        .ok_or_else(|| {
            rejected(
                (length, table_size),
                "one packed candidate for every draw-table entry",
            )
        })?;
    let (records, _) = candidates.as_chunks::<CANDIDATE_BYTES>();
    if !records.iter().all(|candidate| {
        let [a, b, c, d] = candidate[44..48] else {
            return false;
        };
        u32::from_ne_bytes([a, b, c, d]) < count
    }) {
        return Err(rejected(
            count,
            "candidate placement indices within the draw table",
        ));
    }
    Ok(count)
}

fn prepare_output(
    pipeline: &PlacementVisibilityPipeline,
    plan: &CompactionLayout,
    hiz: Option<CompactionHiZ>,
    pages: &mut FrameUploadPages<'_>,
) -> Result<(UploadRange, UploadedStorage, Option<CompactionHiZ>), RenderError> {
    if let Some(hiz) = hiz {
        let output = optional_hiz(allocate_output(pipeline, hiz.output_size, pages).map(Some))?;
        if let Some((commands, output)) = output {
            return Ok((commands, output, Some(hiz)));
        }
    }
    let (commands, output) = allocate_output(pipeline, plan.size, pages)?;
    Ok((commands, output, None))
}

fn allocate_output(
    pipeline: &PlacementVisibilityPipeline,
    bytes: usize,
    pages: &mut FrameUploadPages<'_>,
) -> Result<(UploadRange, UploadedStorage), RenderError> {
    let runtime = &pipeline.runtime;
    let commands = pages.reserve_storage(runtime, bytes)?;
    let output =
        UploadedStorage::bind_frame_range(runtime, &pipeline.storage, commands.clone(), pages)?;
    Ok((commands, output))
}

/// A frame holds every descriptor and output byte range until the last indirect draw retires.
pub(in crate::backend) struct PlacementCompaction {
    candidates: UploadedStorage,
    table: UploadedStorage,
    output: UploadedStorage,
    commands: UploadRange,
    segments: Vec<Segment>,
    include_depth_planes: bool,
    size_extent: [u32; 2],
    pipeline: Arc<PlacementVisibilityPipeline>,
    hiz: Option<Box<CompactionHiZ>>,
}

impl PlacementCompaction {
    /// Lease buffer ranges and descriptor sets for the candidates and their ordered segments.
    #[cfg(test)]
    pub(in crate::backend) fn prepare(
        pipeline: Arc<PlacementVisibilityPipeline>,
        candidate_bytes: &[u8],
        segments: &[(u32, u32)],
        table: &UploadedStorage,
        (include_depth_planes, size_extent): (bool, [u32; 2]),
        pages: &mut FrameUploadPages<'_>,
    ) -> Result<Self, RenderError> {
        Self::prepare_with_hiz(
            pipeline,
            candidate_bytes,
            segments,
            table,
            (include_depth_planes, size_extent, None),
            pages,
        )
    }

    pub(in crate::backend) fn prepare_with_hiz(
        pipeline: Arc<PlacementVisibilityPipeline>,
        candidate_bytes: &[u8],
        segments: &[(u32, u32)],
        table: &UploadedStorage,
        (include_depth_planes, size_extent, request): (bool, [u32; 2], Option<HiZRequest<'_>>),
        pages: &mut FrameUploadPages<'_>,
    ) -> Result<Self, RenderError> {
        let runtime = &pipeline.runtime;
        let table = table
            .frame_range()
            .ok_or_else(|| rejected("dedicated storage", "a frame-owned draw table"))?;
        let count = candidate_count(candidate_bytes, table.size)?;
        let plan = CompactionLayout::new(count, segments).ok_or_else(|| {
            rejected(
                segments,
                "ordered nonempty segments covering the candidate range",
            )
        })?;
        if plan.size as u64 > u64::from(runtime.limits.max_storage_buffer_range) {
            return Err(rejected(
                plan.size,
                "compacted commands and counts within maxStorageBufferRange",
            ));
        }
        let candidates =
            UploadedStorage::for_frame(runtime, &pipeline.storage, candidate_bytes, pages)?;
        let table =
            UploadedStorage::bind_frame_range(runtime, &pipeline.storage, table.clone(), pages)?;
        let hiz = CompactionHiZ::prepare(&pipeline, request, &plan, count, pages)?;
        let (commands, output, hiz) = prepare_output(&pipeline, &plan, hiz, pages)?;
        Ok(Self {
            candidates,
            table,
            output,
            commands,
            segments: plan.segments,
            include_depth_planes,
            size_extent,
            pipeline,
            hiz: hiz.map(Box::new),
        })
    }

    /// Dispatch before dynamic rendering, then make shader writes visible to indirect draws.
    pub(in crate::backend) fn record_compute(&self, command: vk::CommandBuffer) {
        if let Some(hiz) = self.hiz.as_ref().and_then(|hiz| hiz.warm.as_ref()) {
            hiz.record(self, command, false);
        } else {
            let sets = [self.candidates.set, self.output.set, self.table.set];
            self.pipeline.bind_compute(command, &sets);
            self.dispatch_segments(self.visibility_word(), |words, groups| {
                self.pipeline.dispatch(command, words, groups);
            });
        }
        let stages =
            vk::PipelineStageFlags2::DRAW_INDIRECT | vk::PipelineStageFlags2::COMPUTE_SHADER;
        let access =
            vk::AccessFlags2::INDIRECT_COMMAND_READ | vk::AccessFlags2::SHADER_STORAGE_READ;
        #[cfg(test)]
        let (stages, access) = (
            stages | vk::PipelineStageFlags2::HOST,
            access | vk::AccessFlags2::HOST_READ,
        );
        self.output_barrier(command, &self.commands, stages, access);
    }

    fn visibility_word(&self) -> u32 {
        self.hiz.as_ref().map_or(0, |hiz| hiz.first_word)
    }

    fn dispatch_segments(&self, mask_word: u32, mut dispatch: impl FnMut([u32; 7], u32)) {
        for segment in &self.segments {
            self.dispatch_segment(segment, mask_word, &mut dispatch);
        }
    }

    fn dispatch_segment(
        &self,
        segment: &Segment,
        mask_word: u32,
        dispatch: &mut impl FnMut([u32; 7], u32),
    ) {
        let max_groups = self.pipeline.runtime.limits.max_compute_work_group_count[0];
        let depth = u32::from(self.include_depth_planes);
        let mut done = 0;
        while done < segment.count {
            let remaining = segment.count - done;
            let groups = remaining.div_ceil(PLACEMENTS_PER_GROUP).min(max_groups);
            let candidates = remaining.min(groups.saturating_mul(PLACEMENTS_PER_GROUP));
            dispatch(
                [
                    candidates,
                    segment.first + done,
                    segment.first_count_word + done / PLACEMENTS_PER_GROUP,
                    depth,
                    mask_word,
                    self.size_extent[0],
                    self.size_extent[1],
                ],
                groups,
            );
            done += candidates;
        }
    }

    /// Consume one segment with its geometry, material, and graphics pipeline already bound.
    pub(in crate::backend) fn draw_segment(
        &self,
        command: vk::CommandBuffer,
        segment_index: usize,
    ) {
        self.draw_segment_from(command, segment_index, &self.commands);
    }

    fn draw_segment_from(
        &self,
        command: vk::CommandBuffer,
        segment_index: usize,
        range: &UploadRange,
    ) {
        let segment = &self.segments[segment_index];
        let buffer = range.buffer.handle;
        let device = self.pipeline.runtime.device();
        for block in 0..segment.count.div_ceil(PLACEMENTS_PER_GROUP) {
            let start = block * PLACEMENTS_PER_GROUP;
            let max_count = (segment.count - start).min(PLACEMENTS_PER_GROUP);
            let commands =
                range.offset + u64::from(segment.first + start) * u64::from(COMMAND_BYTES);
            let count = range.offset + u64::from(segment.first_count_word + block) * 4;
            // SAFETY: compaction wrote the per-block count and commands before the indirect barrier. Both buffer ranges and the currently bound geometry are owned by this frame.
            unsafe {
                device.cmd_draw_indexed_indirect_count(
                    command,
                    buffer,
                    commands,
                    buffer,
                    count,
                    max_count,
                    COMMAND_BYTES,
                )
            };
        }
    }

    fn output_barrier(
        &self,
        command: vk::CommandBuffer,
        range: &UploadRange,
        stages: vk::PipelineStageFlags2,
        access: vk::AccessFlags2,
    ) {
        let barriers = [vk::BufferMemoryBarrier2::default()
            .buffer(range.buffer.handle)
            .offset(range.offset)
            .size(range.size)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .src_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
            .src_access_mask(vk::AccessFlags2::SHADER_STORAGE_WRITE)
            .dst_stage_mask(stages)
            .dst_access_mask(access)];
        let dependency = vk::DependencyInfo::default().buffer_memory_barriers(&barriers);
        // SAFETY: the command buffer is recording outside rendering and the frame owns this output range.
        unsafe {
            self.pipeline
                .runtime
                .device()
                .cmd_pipeline_barrier2(command, &dependency)
        };
    }
}

#[cfg(test)]
#[path = "placement_visibility_tests.rs"]
mod tests;
