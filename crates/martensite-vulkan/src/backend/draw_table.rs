//! Draw tables (#594, instancing.md steps 3c and 3d). After the draws of a frame upload, a run
//! of consecutive plain lit surface draws whose recording state is equal, and which differ only
//! in their placement, geometry range and indexed material, records as one
//! `vkCmdDrawIndexedIndirect`. The run's tables preserve each draw's transform and material
//! under its original first instance, so the draw order and the captures do not change.
//!
//! A run is a shortcut, never the only form of its draws: every draw keeps its own push
//! constants and lighting block, so a pass that does not take the run (an OIT pass) records the
//! draws one by one as before.
use super::draw_material_table::{DrawMaterialTable, shared_lighting_key};
use super::host_upload_pages::{FrameUploadPages, UploadRange};
use super::pipeline::DrawPipeline;
use super::pipeline_variants::RasterPipelines;
use super::placement_visibility::PlacementCompaction;
use super::runtime::VulkanRuntime;
use super::storage_descriptor::DrawTableStorage;
use super::{frame::UploadedDraw, geometry_residency::IndexFormat};
use crate::{PrimitiveDraw, RenderError, RenderLighting};
use ash::vk;
use std::sync::Arc;

/// The bytes of a table entry: the clip, world and normal matrices (`DrawTableEntry`).
pub(super) const ENTRY_BYTES: usize = 192;
/// The size of `VkDrawIndexedIndirectCommand`.
pub(super) const COMMAND_BYTES: u32 = 20;
/// The most draws of one run, which bounds a table to 768 KiB of a frame page.
const MAX_RUN_DRAWS: usize = 4096;
/// The fewest draws of a run: a run costs a descriptor set, a table and a command range per
/// frame, which a few separately recorded draws do not.
const MIN_RUN_DRAWS: usize = 8;

#[cfg(test)]
thread_local! {
    /// Whether runs form, and how many formed: a test renders a frame with and without runs.
    static TEST_RUNS: std::cell::Cell<(bool, usize)> = const { std::cell::Cell::new((true, 0)) };
}

/// One indirect run: the twin pipeline, the table and the commands of its draws.
pub(super) struct DrawTableRun {
    pub pipeline: Arc<DrawPipeline>,
    table: DrawTableStorage,
    commands: DrawCommands,
    /// The frame draws of the run, the first of them included.
    pub count: usize,
    /// The commands of a placement run, by geometry buffers in draw order; empty for a run of
    /// frame draws, which draw from the buffers of the first (draw_instance_upload.rs).
    segments: Vec<RunSegment>,
    /// The template uploads of a placement run, kept until the frame retires.
    templates: Vec<UploadedDraw>,
}

pub(super) enum DrawCommands {
    Ordered(UploadRange),
    Compacted(Box<PlacementCompaction>),
}

/// Consecutive placement commands that bind the same geometry and shared lighting.
pub(super) struct RunSegment {
    pub vertices: vk::Buffer,
    pub indices: vk::Buffer,
    pub index_format: IndexFormat,
    pub template: usize,
    pub first: u32,
    pub count: u32,
}

impl DrawTableRun {
    /// The run of a placement frame draw; `DrawTableRun::placements(pipeline, ...)`.
    pub(super) fn placements(
        pipeline: Arc<DrawPipeline>,
        (table, commands): (DrawTableStorage, DrawCommands),
        segments: Vec<RunSegment>,
        templates: Vec<UploadedDraw>,
    ) -> Self {
        Self {
            pipeline,
            table,
            commands,
            count: 1,
            segments,
            templates,
        }
    }

    pub(super) fn record_compaction(
        &self,
        runtime: &VulkanRuntime,
        command: vk::CommandBuffer,
        extent: vk::Extent2D,
        head: &UploadedDraw,
    ) {
        let DrawCommands::Compacted(compaction) = &self.commands else {
            return;
        };
        compaction.record_occluders(command, |pipeline| {
            super::placement_hiz_raster::record(
                runtime,
                command,
                extent,
                head,
                (pipeline, self.table.set),
                &self.segments,
                compaction,
            );
        });
        compaction.record_compute(command);
    }

    fn publish_hiz(&mut self) {
        if let DrawCommands::Compacted(compaction) = &mut self.commands {
            compaction.publish_hiz();
        }
    }
}

/// What a frame forms its runs from: the uploaded draws' sources, from the first uploaded one,
/// the stencil plan of the whole frame and the layer ranges.
pub(super) struct TableFrame<'a> {
    pub runtime: &'a Arc<VulkanRuntime>,
    pub pipelines: &'a RasterPipelines,
    pub sources: &'a [PrimitiveDraw],
    pub first_draw: usize,
    pub stencil: &'a crate::stencil_plan::StencilPlan,
    pub layers: &'a [crate::RenderLayerSpan],
}

/// Form the runs of every layer and attach each to its first draw;
/// `form_runs(&frame, &mut draws, &mut pages)?`.
pub(super) fn form_runs(
    frame: &TableFrame<'_>,
    draws: &mut [UploadedDraw],
    pages: &mut FrameUploadPages<'_>,
) -> Result<(), RenderError> {
    if !frame.runtime.draw_tables || cfg!(feature = "lit-only-command-diagnostics") {
        return repair_borrowed_lighting(frame, draws, pages);
    }
    #[cfg(test)]
    if !TEST_RUNS.get().0 {
        return repair_borrowed_lighting(frame, draws, pages);
    }
    for range in layer_ranges(frame, draws.len()) {
        let mut start = range.start;
        while start < range.end {
            let end = run_end(frame, draws, start, range.end);
            if end - start >= MIN_RUN_DRAWS {
                let run = build_run(frame, start..end, draws, pages)?;
                #[cfg(test)]
                TEST_RUNS.set((true, TEST_RUNS.get().1 + usize::from(run.is_some())));
                draws[start].table_run = run.map(Box::new);
            }
            start = end;
        }
    }
    repair_borrowed_lighting(frame, draws, pages)
}

/// Bind its own lighting block for each draw that borrowed one and that no run draws: such a
/// draw records alone, and its vertex stage reads the block's world and normal matrices.
fn repair_borrowed_lighting(
    frame: &TableFrame<'_>,
    draws: &mut [UploadedDraw],
    pages: &mut FrameUploadPages<'_>,
) -> Result<(), RenderError> {
    let mut covered = 0;
    for (index, (draw, source)) in draws.iter_mut().zip(frame.sources).enumerate() {
        if let Some(run) = &draw.table_run {
            covered = index + run.count;
        }
        if index < covered || !draw.borrowed_lighting {
            continue;
        }
        let (Some(lighting), Some(layout)) = (
            source.lighting.as_ref(),
            draw.pipeline.lighting_layout.as_ref(),
        ) else {
            continue;
        };
        let bindings = &frame.pipelines.lighting_bindings;
        let own = bindings.bind_draw(frame.runtime, layout, lighting, pages, true)?;
        draw.bindings.lighting = Some(own);
        draw.borrowed_lighting = false;
    }
    Ok(())
}

/// The ranges of uploaded draw indices of each layer, without the draws before the first
/// uploaded one.
fn layer_ranges<'a>(
    frame: &'a TableFrame<'_>,
    count: usize,
) -> impl Iterator<Item = std::ops::Range<usize>> + 'a {
    frame.layers.iter().filter_map(move |layer| {
        let start = layer.draw_range.start.max(frame.first_draw) - frame.first_draw;
        let end = layer
            .draw_range
            .end
            .saturating_sub(frame.first_draw)
            .min(count);
        (start < end).then_some(start..end)
    })
}

/// The end of the run that starts at `start`: `start + 1` when the draw cannot start one.
fn run_end(frame: &TableFrame<'_>, draws: &[UploadedDraw], start: usize, limit: usize) -> usize {
    if !table_draw(frame, draws, start) {
        return start + 1;
    }
    let limit = limit.min(start + MAX_RUN_DRAWS);
    let mut end = start + 1;
    while end < limit && table_draw(frame, draws, end) && joins_run(frame, draws, start, end) {
        end += 1;
    }
    end
}

/// Whether uploaded draw `index` is a plain lit surface that a table can draw.
fn table_draw(frame: &TableFrame<'_>, draws: &[UploadedDraw], index: usize) -> bool {
    let draw = &draws[index];
    plain_uploaded(draw)
        && frame.sources[index]
            .lighting
            .as_ref()
            .is_some_and(|lighting| lighting.fixed.is_none() && !lighting.pbr)
        && base_geometry(draw).is_some()
}

/// Whether nothing of the draw besides its geometry and push constants needs a command of its
/// own: no line, point, window-depth, user-program, alpha or OIT state, and no binding besides
/// lighting, PBR environment and shadow maps.
pub(super) fn plain_uploaded(draw: &UploadedDraw) -> bool {
    let bindings = &draw.bindings;
    draw.transform.len() == 64
        && draw.instances == 1
        && draw.point_viewports.is_none()
        && draw.window_depth_status.is_none()
        && !draw.window_depth_expected
        && draw.user_program.is_none()
        && draw.native_alpha.is_none()
        && draw.oit_alpha.is_empty()
        && draw.oit.is_none()
        && (draw.line.width, draw.line.stipple) == (None, None)
        && draw.line.wide.is_none()
        && draw.presentation.capping == crate::RenderCappingPass::None
        && bindings.lighting.is_some()
        && bindings.clipping.is_none()
        && bindings.sprite.is_none()
        && bindings.surface_image.is_none()
        && bindings.material_maps.is_none()
        && bindings.sphere_map.is_none()
}

/// The first index and base vertex of a draw whose buffers bind at offset zero, as
/// `record_draw_bindings::bind_geometry` binds them; `None` for another binding.
pub(super) fn base_geometry(draw: &UploadedDraw) -> Option<(u32, i32)> {
    let ((vertex_offset, base), (index_offset, first)) =
        super::record_draw_bindings::geometry_locations(draw, &draw.pipeline);
    (vertex_offset == 0 && index_offset == 0).then_some((first, base))
}

/// Whether draw `index` records with exactly the state of the run's first draw `head`.
fn joins_run(frame: &TableFrame<'_>, draws: &[UploadedDraw], head: usize, index: usize) -> bool {
    let (first, draw) = (&draws[head], &draws[index]);
    let plan = frame.stencil;
    let at = |index: usize| index + frame.first_draw;
    Arc::ptr_eq(&first.pipeline, &draw.pipeline)
        && (first.vertices.handle, first.indices.handle)
            == (draw.vertices.handle, draw.indices.handle)
        && first.index_format == draw.index_format
        && first.presentation == draw.presentation
        && first.sample_alpha_cutoff == draw.sample_alpha_cutoff
        && same_owner(
            &first.bindings.pbr_environment,
            &draw.bindings.pbr_environment,
        )
        && same_owner(&first.bindings.shadow_maps, &draw.bindings.shadow_maps)
        && plan.during[at(head)] == plan.during[at(index)]
        && !plan.clear_before[at(index)]
        && (borrows_from_previous(draws, index)
            || shared_lighting(
                &frame.sources[head],
                &frame.sources[index],
                first.pipeline.uses_indexed_draw_materials(),
            ))
}

/// Whether the draw borrowed the very binding of the draw before it, whose block then differs
/// from its own only in the placement.
fn borrows_from_previous(draws: &[UploadedDraw], index: usize) -> bool {
    let (Some(previous), Some(lighting)) = (
        draws[index - 1].bindings.lighting.as_ref(),
        draws[index].bindings.lighting.as_ref(),
    ) else {
        return false;
    };
    draws[index].borrowed_lighting && Arc::ptr_eq(previous, lighting)
}

pub(super) fn same_owner<T>(left: &Option<Arc<T>>, right: &Option<Arc<T>>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(left), Some(right)) => Arc::ptr_eq(left, right),
        _ => false,
    }
}

/// The run shares every lighting word that its table does not replace.
fn shared_lighting(first: &PrimitiveDraw, draw: &PrimitiveDraw, indexed: bool) -> bool {
    let (Some(first), Some(draw)) = (&first.lighting, &draw.lighting) else {
        return false;
    };
    shared_lighting_values(first, draw, indexed)
}

fn shared_lighting_values(first: &RenderLighting, draw: &RenderLighting, indexed: bool) -> bool {
    let (mut left, mut right) = (Vec::new(), Vec::new());
    first.encode_into(&mut left);
    draw.encode_into(&mut right);
    shared_lighting_key(&mut left, indexed);
    shared_lighting_key(&mut right, indexed);
    left == right
}

/// Write the table and the commands of the draws in `range`; `None` when the pipeline has
/// no twin.
fn build_run(
    frame: &TableFrame<'_>,
    range: std::ops::Range<usize>,
    draws: &[UploadedDraw],
    pages: &mut FrameUploadPages<'_>,
) -> Result<Option<DrawTableRun>, RenderError> {
    let (sources, draws) = (&frame.sources[range.clone()], &draws[range]);
    let Some(pipeline) = frame.pipelines.draw_table_twin(&draws[0].pipeline)? else {
        return Ok(None);
    };
    let Some(layout) = pipeline.draw_table_layout.as_ref() else {
        return Ok(None);
    };
    let entries = table_entries(draws, sources);
    let materials = draws[0]
        .pipeline
        .uses_indexed_draw_materials()
        .then(|| draw_materials(sources))
        .transpose()?;
    let table = DrawTableStorage::upload(
        frame.runtime,
        layout,
        &entries,
        materials.as_ref().map(DrawMaterialTable::bytes),
        pages,
    )?;
    let commands = pages.write_storage(frame.runtime, &indirect_commands(draws))?;
    Ok(Some(DrawTableRun {
        pipeline,
        table,
        commands: DrawCommands::Ordered(commands),
        count: draws.len(),
        segments: Vec::new(),
        templates: Vec::new(),
    }))
}

fn draw_materials(sources: &[PrimitiveDraw]) -> Result<DrawMaterialTable, RenderError> {
    let mut materials = DrawMaterialTable::new(sources.len());
    for (index, draw) in sources.iter().enumerate() {
        let offset = materials.intern(draw)?;
        materials.set_draw(index, offset);
    }
    Ok(materials)
}

/// The table entries of a run: each draw's pushed clip matrix and its lighting block's world
/// and normal matrices, the bytes the single draw would read.
fn table_entries(draws: &[UploadedDraw], sources: &[PrimitiveDraw]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(draws.len() * ENTRY_BYTES);
    for (draw, source) in draws.iter().zip(sources) {
        bytes.extend_from_slice(&draw.transform);
        if let Some(lighting) = &source.lighting {
            bytes.extend_from_slice(lighting.world.map(f32::to_ne_bytes).as_flattened());
            bytes.extend_from_slice(lighting.normal.map(f32::to_ne_bytes).as_flattened());
        }
    }
    bytes
}

/// One `VkDrawIndexedIndirectCommand` per draw, with the draw's place in the table as its
/// first instance.
fn indirect_commands(draws: &[UploadedDraw]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(draws.len() * COMMAND_BYTES as usize);
    for (instance, draw) in draws.iter().enumerate() {
        let (first_index, base_vertex) = base_geometry(draw).unwrap_or_default();
        let words = [
            draw.count,
            1,
            first_index,
            base_vertex as u32,
            instance as u32,
        ];
        bytes.extend_from_slice(words.map(u32::to_ne_bytes).as_flattened());
    }
    bytes
}

/// Bind the run's table and record its indirect draw in place of its draws;
/// `record_run(runtime, command, &run, &mut slots)`.
pub(super) fn record_run(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    run: &DrawTableRun,
    slots: &mut super::record_draw_bindings::BindingSlots,
) {
    let layout = run.pipeline.layout;
    if slots.needs_set(layout, 7, run.table.set) {
        super::record_draw_bindings::bind_descriptor(runtime, command, layout, 7, run.table.set);
    }
    if run.segments.is_empty() {
        indirect(runtime, command, run, (0, run.count as u32));
        return;
    }
    for (segment_index, segment) in run.segments.iter().enumerate() {
        slots.bind_buffers(
            runtime,
            command,
            (segment.vertices, segment.indices, segment.index_format),
        );
        let template = &run.templates[segment.template];
        super::record_draw_bindings::bind_lighting(
            runtime,
            command,
            template,
            &run.pipeline,
            &mut slots.lighting,
        );
        match &run.commands {
            DrawCommands::Ordered(_) => {
                indirect(runtime, command, run, (segment.first, segment.count))
            }
            DrawCommands::Compacted(compaction) => compaction.draw_segment(command, segment_index),
        }
    }
}

/// Record the run's commands `[first, first + count)`.
fn indirect(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    run: &DrawTableRun,
    (first, count): (u32, u32),
) {
    let DrawCommands::Ordered(commands) = &run.commands else {
        return;
    };
    for (first, count) in indirect_chunks(first, count, runtime.limits.max_draw_indirect_count) {
        let offset = commands.offset + u64::from(first) * u64::from(COMMAND_BYTES);
        // SAFETY: the frame owns this command range through completion. The device
        // enabled both indirect features, and each call stays within its draw-count limit.
        unsafe {
            runtime.device().cmd_draw_indexed_indirect(
                command,
                commands.buffer.handle,
                offset,
                count,
                COMMAND_BYTES,
            );
        }
    }
}

fn indirect_chunks(first: u32, count: u32, limit: u32) -> impl Iterator<Item = (u32, u32)> {
    let limit = limit.max(1);
    (0..count)
        .step_by(limit as usize)
        .map(move |skip| (first + skip, (count - skip).min(limit)))
}

#[cfg(test)]
#[path = "draw_table_tests.rs"]
mod tests;

pub(super) fn publish_hiz(draws: &mut [UploadedDraw]) {
    if let [draw] = draws
        && let Some(run) = &mut draw.table_run
    {
        run.publish_hiz();
    }
}
