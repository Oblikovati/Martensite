//! Bootstrap a frame-owned visibility mask or use its predecessor to rasterize a depth witness.
use super::*;

pub(in crate::backend) struct HiZRequest<'a> {
    pub(in crate::backend) history: Arc<HiZHistory>,
    pub(in crate::backend) view: HiZView,
    pub(in crate::backend) reduction: Arc<HiZPipeline>,
    pub(in crate::backend) occluder: Arc<DrawPipeline>,
    pub(in crate::backend) extent: vk::Extent2D,
    pub(in crate::backend) viewport: vk::Viewport,
    pub(in crate::backend) targets: &'a crate::backend::frame_targets::FrameTargetPool,
}

struct PendingHiZPublication {
    history: Arc<HiZHistory>,
    view: HiZView,
}

pub(super) struct CompactionHiZ {
    publication: Option<PendingHiZPublication>,
    occluder: Arc<DrawPipeline>,
    pub(super) warm: Option<WarmHiZ>,
    pub(super) output_size: usize,
    pub(super) first_word: u32,
}

pub(super) struct WarmHiZ {
    reduction: Arc<HiZPipeline>,
    pyramid: HiZPyramid,
    previous: UploadedStorage,
    previous_first_word: u32,
    pyramid_values: UploadedStorage,
    prepass: UploadedStorage,
    prepass_range: UploadRange,
    extent: vk::Extent2D,
    viewport: vk::Viewport,
}

impl CompactionHiZ {
    pub(super) fn prepare(
        pipeline: &PlacementVisibilityPipeline,
        request: Option<HiZRequest<'_>>,
        plan: &CompactionLayout,
        count: u32,
        pages: &mut FrameUploadPages<'_>,
    ) -> Result<Option<Self>, RenderError> {
        let Some(request) = request else {
            return Ok(None);
        };
        let Some((first_word, output_size)) = mask_layout(plan.size, count, &pipeline.runtime)
        else {
            return Ok(None);
        };
        let warm = match request.history.witness(&request.view) {
            Some(previous) => optional_hiz(WarmHiZ::prepare(
                pipeline, &request, previous, plan.size, pages,
            ))?,
            None => None,
        };
        Ok(Some(Self {
            publication: Some(PendingHiZPublication {
                history: request.history,
                view: request.view,
            }),
            occluder: request.occluder,
            warm,
            output_size,
            first_word,
        }))
    }
}

fn mask_layout(
    command_size: usize,
    count: u32,
    runtime: &crate::backend::runtime::VulkanRuntime,
) -> Option<(u32, usize)> {
    let first = u32::try_from(command_size / 4).ok()?;
    let total = u64::from(first.checked_add(count)?) * 4;
    if total > u64::from(runtime.limits.max_storage_buffer_range) {
        return None;
    }
    Some((first, usize::try_from(total).ok()?))
}

impl WarmHiZ {
    fn prepare(
        pipeline: &PlacementVisibilityPipeline,
        request: &HiZRequest<'_>,
        previous: HiZWitness,
        bytes: usize,
        pages: &mut FrameUploadPages<'_>,
    ) -> Result<Option<Self>, RenderError> {
        let Some(pyramid) = HiZPyramid::prepare(
            request.reduction.clone(),
            request.extent,
            request.targets,
            pages,
        )?
        else {
            return Ok(None);
        };
        let runtime = &pipeline.runtime;
        let layout = &pipeline.storage;
        let pyramid_values =
            UploadedStorage::bind_frame_range(runtime, layout, pyramid.range().clone(), pages)?;
        let prepass_range = pages.reserve_storage(runtime, bytes)?;
        let prepass =
            UploadedStorage::bind_frame_range(runtime, layout, prepass_range.clone(), pages)?;
        Ok(Some(Self {
            reduction: request.reduction.clone(),
            pyramid,
            previous: UploadedStorage::bind_frame_range(runtime, layout, previous.output, pages)?,
            previous_first_word: previous.first_word,
            pyramid_values,
            prepass,
            prepass_range,
            extent: request.extent,
            viewport: request.viewport,
        }))
    }

    pub(super) fn record(
        &self,
        compaction: &PlacementCompaction,
        command: vk::CommandBuffer,
        prepass: bool,
    ) {
        let output = if prepass {
            self.prepass.set
        } else {
            compaction.output.set
        };
        let sets = [
            compaction.candidates.set,
            output,
            compaction.table.set,
            self.previous.set,
            self.pyramid_values.set,
        ];
        let mask_word = if prepass {
            0
        } else {
            compaction.visibility_word()
        };
        let pipeline = &self.reduction.cull;
        pipeline.bind_compute(command, &sets);
        compaction.dispatch_segments(mask_word, |words, groups| {
            pipeline.dispatch(command, self.words(words, prepass), groups);
        });
    }

    fn words(&self, base: [u32; 7], prepass: bool) -> [u32; 15] {
        let [count, first, counts, depth, visibility, width, height] = base;
        [
            count,
            first,
            counts,
            depth,
            visibility,
            width,
            height,
            self.previous_first_word,
            if prepass { 1 } else { 2 },
            self.extent.width,
            self.extent.height,
            self.viewport.x.to_bits(),
            self.viewport.y.to_bits(),
            self.viewport.width.to_bits(),
            self.viewport.height.to_bits(),
        ]
    }
}

impl PlacementCompaction {
    pub(in crate::backend) fn record_occluders(
        &self,
        command: vk::CommandBuffer,
        draw: impl FnOnce(&DrawPipeline),
    ) {
        let Some(hiz) = &self.hiz else { return };
        let Some(warm) = &hiz.warm else { return };
        warm.record(self, command, true);
        self.output_barrier(
            command,
            &warm.prepass_range,
            vk::PipelineStageFlags2::DRAW_INDIRECT,
            vk::AccessFlags2::INDIRECT_COMMAND_READ,
        );
        warm.pyramid.begin_occluders(command);
        draw(&hiz.occluder);
        warm.pyramid.build_after_occluders(command);
    }

    pub(in crate::backend) fn draw_occluder_segment(
        &self,
        command: vk::CommandBuffer,
        index: usize,
    ) {
        if let Some(warm) = self.hiz.as_ref().and_then(|hiz| hiz.warm.as_ref()) {
            self.draw_segment_from(command, index, &warm.prepass_range);
        }
    }

    pub(in crate::backend) fn publish_hiz(&mut self) {
        let Some(hiz) = &mut self.hiz else { return };
        let Some(pending) = hiz.publication.take() else {
            return;
        };
        let witness = HiZWitness {
            output: self.commands.clone(),
            first_word: hiz.first_word,
        };
        pending.history.publish(pending.view, witness);
    }
}
