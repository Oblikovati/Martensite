//! Isolated depth attachment and ordered max reduction for a Hi-Z frame.
use super::*;

const WORKGROUP_SIDE: u32 = 8;
pub(super) const DEPTH_USAGE: vk::ImageUsageFlags = vk::ImageUsageFlags::from_raw(
    vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT.as_raw() | vk::ImageUsageFlags::SAMPLED.as_raw(),
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Level {
    extent: vk::Extent2D,
    offset: u32,
}

fn levels_for(extent: vk::Extent2D) -> Option<(Vec<Level>, usize)> {
    if extent.width == 0 || extent.height == 0 {
        return None;
    }
    let mut levels = Vec::new();
    let mut offset = u32::BITS - 1;
    let mut extent = half_extent(extent);
    loop {
        levels.push(Level { extent, offset });
        offset = offset.checked_add(extent.width.checked_mul(extent.height)?)?;
        if extent.width == 1 && extent.height == 1 {
            break;
        }
        extent = half_extent(extent);
    }
    Some((levels, usize::try_from(u64::from(offset) * 4).ok()?))
}

fn half_extent(extent: vk::Extent2D) -> vk::Extent2D {
    vk::Extent2D {
        width: extent.width.div_ceil(2),
        height: extent.height.div_ceil(2),
    }
}

pub(in crate::backend) struct HiZPyramid {
    source: TargetSampleSet,
    storage: UploadedStorage,
    depth: Arc<TargetImage>,
    extent: vk::Extent2D,
    values: UploadRange,
    levels: Vec<Level>,
    pipeline: Arc<HiZPipeline>,
}

impl HiZPyramid {
    /// Allocate the draw-only depth witness and its per-frame reduction ranges.
    pub(in crate::backend) fn prepare(
        pipeline: Arc<HiZPipeline>,
        extent: vk::Extent2D,
        targets: &crate::backend::frame_targets::FrameTargetPool,
        pages: &mut FrameUploadPages<'_>,
    ) -> Result<Option<Self>, RenderError> {
        let runtime = &pipeline.runtime;
        let Some((levels, size)) = levels_for(extent) else {
            return Ok(None);
        };
        if size as u64 > u64::from(runtime.limits.max_storage_buffer_range) {
            return Ok(None);
        }
        let depth = targets.hiz_depth(runtime, extent, depth_spec(runtime))?;
        let source = TargetSampleSet::new(runtime, pipeline.source_layout, depth.view)?;
        let values = pages.reserve_storage(runtime, size)?;
        let offsets: Vec<_> = levels
            .iter()
            .flat_map(|level| level.offset.to_ne_bytes())
            .collect();
        values.buffer.write_at(values.offset as usize, &offsets)?;
        let storage =
            UploadedStorage::bind_frame_range(runtime, &pipeline.storage, values.clone(), pages)?;
        Ok(Some(Self {
            source,
            storage,
            depth,
            extent,
            values,
            levels,
            pipeline,
        }))
    }

    /// Enter an isolated, cleared depth-only rendering scope.
    pub(in crate::backend) fn begin_occluders(&self, command: vk::CommandBuffer) {
        let depth = super::super::render_target_record::render_attachment(
            self.depth.view,
            super::super::render_target_record::depth_clear(),
            true,
        );
        let area = vk::Rect2D {
            offset: vk::Offset2D::default(),
            extent: self.extent,
        };
        let info = vk::RenderingInfo::default()
            .render_area(area)
            .layer_count(1)
            .depth_attachment(&depth);
        self.transition_depth(command, false);
        // SAFETY: this fresh image is in attachment layout and the frame retains it through command completion.
        unsafe {
            self.pipeline
                .runtime
                .device()
                .cmd_begin_rendering(command, &info)
        };
    }

    fn transition_depth(&self, command: vk::CommandBuffer, sample: bool) {
        let barrier = vk::ImageMemoryBarrier2::default()
            .image(self.depth.handle)
            .subresource_range(self.depth.range())
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED);
        let stages = vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS
            | vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS;
        let write = vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE;
        let barrier = if sample {
            barrier
                .old_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
                .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .src_stage_mask(stages)
                .src_access_mask(write)
                .dst_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
                .dst_access_mask(vk::AccessFlags2::SHADER_SAMPLED_READ)
        } else {
            barrier
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::ATTACHMENT_OPTIMAL)
                .dst_stage_mask(stages)
                .dst_access_mask(write | vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_READ)
        };
        let barriers = [barrier];
        let info = vk::DependencyInfo::default().image_memory_barriers(&barriers);
        // SAFETY: the frame owns the image and transitions occur outside the rendering scope.
        unsafe {
            self.pipeline
                .runtime
                .device()
                .cmd_pipeline_barrier2(command, &info)
        };
    }

    /// End the depth scope and build every level with a shader-write-to-read barrier.
    pub(in crate::backend) fn build_after_occluders(&self, command: vk::CommandBuffer) {
        let device = self.pipeline.runtime.device();
        // SAFETY: this command buffer started the matching occluder scope and owns its descriptors.
        unsafe { device.cmd_end_rendering(command) };
        self.transition_depth(command, true);
        // SAFETY: the frame owns the layouts, pipeline, and these descriptors until retirement.
        unsafe {
            device.cmd_bind_pipeline(
                command,
                vk::PipelineBindPoint::COMPUTE,
                self.pipeline.pipeline,
            );
            device.cmd_bind_descriptor_sets(
                command,
                vk::PipelineBindPoint::COMPUTE,
                self.pipeline.layout,
                0,
                &[self.source.handle, self.storage.set],
                &[],
            );
        }
        for (index, &target) in self.levels.iter().enumerate() {
            let base = Level {
                extent: self.extent,
                offset: 0,
            };
            let source = index
                .checked_sub(1)
                .map_or(base, |previous| self.levels[previous]);
            self.reduce(command, source, target, index == 0);
            self.storage_barrier(
                command,
                vk::PipelineStageFlags2::COMPUTE_SHADER,
                vk::AccessFlags2::SHADER_STORAGE_READ | vk::AccessFlags2::SHADER_STORAGE_WRITE,
            );
        }
    }

    fn reduce(&self, command: vk::CommandBuffer, source: Level, target: Level, base: bool) {
        let words = [
            source.extent.width,
            source.extent.height,
            target.extent.width,
            target.extent.height,
            source.offset,
            target.offset,
            u32::from(base),
        ];
        let push = words.map(u32::to_ne_bytes);
        // SAFETY: the compatible reduction pipeline is bound and both level ranges are inside its owned descriptor.
        unsafe {
            let device = self.pipeline.runtime.device();
            device.cmd_push_constants(
                command,
                self.pipeline.layout,
                vk::ShaderStageFlags::COMPUTE,
                0,
                push.as_flattened(),
            );
            device.cmd_dispatch(
                command,
                target.extent.width.div_ceil(WORKGROUP_SIDE),
                target.extent.height.div_ceil(WORKGROUP_SIDE),
                1,
            );
        }
    }

    fn storage_barrier(
        &self,
        command: vk::CommandBuffer,
        stage: vk::PipelineStageFlags2,
        access: vk::AccessFlags2,
    ) {
        let barriers = [vk::BufferMemoryBarrier2::default()
            .buffer(self.values.buffer.handle)
            .offset(self.values.offset)
            .size(self.values.size)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .src_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
            .src_access_mask(vk::AccessFlags2::SHADER_STORAGE_WRITE)
            .dst_stage_mask(stage)
            .dst_access_mask(access)];
        let info = vk::DependencyInfo::default().buffer_memory_barriers(&barriers);
        // SAFETY: all accesses name this frame's range outside a rendering scope.
        unsafe {
            self.pipeline
                .runtime
                .device()
                .cmd_pipeline_barrier2(command, &info)
        };
    }

    /// An alias for the occlusion compute shader to bind the completed pyramid.
    pub(in crate::backend) fn range(&self) -> &UploadRange {
        &self.values
    }
}

fn depth_spec(runtime: &VulkanRuntime) -> AttachmentSpec {
    AttachmentSpec {
        format: runtime.depth.formats.plain,
        usage: DEPTH_USAGE,
        aspect: vk::ImageAspectFlags::DEPTH,
        samples: vk::SampleCountFlags::TYPE_1,
    }
}

#[cfg(test)]
#[path = "placement_hiz_pyramid_tests.rs"]
mod tests;
