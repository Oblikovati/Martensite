//! The run of a retained placement frame draw (draw_instances.rs, #594). After the frame's draws
//! upload, each template uploads as an ordinary draw, the templates must record with one
//! pipeline and state, and the run writes one table entry and one indirect command per
//! placement, in draw order, segmented by the templates' geometry buffers.
use super::super::draw_material_table::{DrawMaterialTable, shared_lighting_key};
use super::super::draw_table::{
    COMMAND_BYTES, DrawCommands, DrawTableRun, ENTRY_BYTES, RunSegment, base_geometry,
    plain_uploaded, same_owner,
};
use super::super::host_upload_pages::FrameUploadPages;
use super::super::pipeline_variants::RasterPipelines;
use super::super::retained_submission::RetirementStore;
use super::super::runtime::VulkanRuntime;
use super::super::storage_descriptor::{DrawTableStorage, UploadedStorage};
use super::draw_upload::upload_draw;
use super::{FrameDrawUploads, UploadedDraw};
use crate::draw_instances::DrawInstances;
use crate::{RenderError, error::rejected};
use ash::vk;
use std::sync::Arc;

/// What the template uploads read: the runtime, pipelines, extent, samples and retirements.
pub(super) type TemplateUpload<'a> = (
    &'a Arc<VulkanRuntime>,
    &'a RasterPipelines,
    vk::Extent2D,
    vk::SampleCountFlags,
    &'a RetirementStore,
);

/// Attach its placement run to each frame draw that carries placements;
/// `attach_placement_runs(upload, sources, &mut draws, &mut uploads)?`.
pub(super) fn attach_placement_runs(
    upload: TemplateUpload<'_>,
    (sources, hiz_layer): (&[crate::PrimitiveDraw], Option<&crate::RenderLayerSpan>),
    draws: &mut [UploadedDraw],
    uploads: &mut FrameDrawUploads<'_>,
) -> Result<(), RenderError> {
    for (draw, source) in draws.iter_mut().zip(sources) {
        if let Some(instances) = &source.instances {
            draw.table_run = Some(Box::new(placement_run(
                upload,
                (draw, hiz_layer),
                instances,
                uploads,
            )?));
        }
    }
    Ok(())
}

/// The run of one placement draw, whose own upload is `head`'s.
fn placement_run(
    upload: TemplateUpload<'_>,
    (head, hiz_layer): (&UploadedDraw, Option<&crate::RenderLayerSpan>),
    instances: &DrawInstances,
    uploads: &mut FrameDrawUploads<'_>,
) -> Result<DrawTableRun, RenderError> {
    let templates = upload_templates(upload, instances, uploads)?;
    require_one_state(head, &templates)?;
    let (runtime, pipelines, extent, samples, _) = upload;
    let pipeline = pipelines.draw_table_twin(&head.pipeline)?.ok_or_else(|| {
        rejected(
            head.pipeline.kind,
            "a draw-table twin for placement templates",
        )
    })?;
    let layout = pipeline
        .draw_table_layout
        .as_ref()
        .ok_or_else(|| rejected("no table layout", "a draw-table twin pipeline"))?;
    let pages: &mut FrameUploadPages<'_> = &mut uploads.storage;
    let indexed = head.pipeline.uses_indexed_draw_materials();
    let materials = indexed
        .then(|| placement_materials(instances))
        .transpose()?;
    let table = DrawTableStorage::upload(
        runtime,
        layout,
        &table_entries(instances),
        materials.as_ref().map(DrawMaterialTable::bytes),
        pages,
    )?;
    let segments = segments(instances, &templates, &lighting_classes(instances, indexed));
    let hiz =
        super::super::placement_hiz::optional_hiz(super::super::placement_hiz_raster::request(
            runtime,
            pipelines,
            (extent, samples, hiz_layer),
            head,
            instances,
        ))?;
    let commands = draw_commands(
        (runtime, pipelines, extent, samples),
        instances,
        &templates,
        (&table, &segments, hiz),
        pages,
    )?;
    Ok(DrawTableRun::placements(
        pipeline,
        (table, commands),
        segments,
        templates,
    ))
}

fn draw_commands(
    (runtime, pipelines, extent, samples): (
        &Arc<VulkanRuntime>,
        &RasterPipelines,
        vk::Extent2D,
        vk::SampleCountFlags,
    ),
    instances: &DrawInstances,
    templates: &[UploadedDraw],
    (table, segments, hiz): (
        &UploadedStorage,
        &[RunSegment],
        Option<super::super::placement_visibility::HiZRequest<'_>>,
    ),
    pages: &mut FrameUploadPages<'_>,
) -> Result<DrawCommands, RenderError> {
    let Some(visibility) = &instances.visibility else {
        return pages
            .write_storage(runtime, &placement_commands(instances, templates))
            .map(DrawCommands::Ordered);
    };
    let candidates = visibility_candidates(instances, templates, visibility)?;
    let ranges: Vec<_> = segments
        .iter()
        .map(|segment| (segment.first, segment.count))
        .collect();
    let Some(pipeline) = pipelines.placement_visibility()? else {
        return Err(rejected(
            "device feature unavailable",
            "placement compaction enabled during view admission",
        ));
    };
    let size_extent = if samples == vk::SampleCountFlags::TYPE_1
        && runtime.limits.sub_pixel_precision_bits >= 4
    {
        [extent.width, extent.height]
    } else {
        [0; 2]
    };
    super::super::placement_visibility::PlacementCompaction::prepare_with_hiz(
        pipeline,
        &candidates,
        &ranges,
        table,
        (!templates[0].pipeline.kind.clamp, size_extent, hiz),
        pages,
    )
    .map(|compaction| DrawCommands::Compacted(Box::new(compaction)))
}

fn visibility_candidates(
    instances: &DrawInstances,
    templates: &[UploadedDraw],
    visibility: &crate::draw_instances::InstanceVisibility,
) -> Result<Vec<u8>, RenderError> {
    if visibility.visible.len() != instances.placements.len()
        || visibility.bounds.len() != templates.len()
    {
        return Err(rejected(
            visibility.visible.len(),
            "one visibility flag per placement and one bound per template",
        ));
    }
    let mut bytes = Vec::with_capacity(instances.placements.len() * 48);
    for (index, (placement, &shown)) in instances
        .placements
        .iter()
        .zip(&visibility.visible)
        .enumerate()
    {
        let template_index = placement.template as usize;
        let template = &templates[template_index];
        let (first_index, base_vertex) = base_geometry(template).unwrap_or_default();
        bytes.extend_from_slice(&visibility_words(visibility.bounds[template_index]));
        let command = [
            if shown { template.count } else { 0 },
            first_index,
            base_vertex as u32,
            index as u32,
        ];
        bytes.extend_from_slice(command.map(u32::to_ne_bytes).as_flattened());
    }
    Ok(bytes)
}

fn visibility_words(bounds: Option<crate::draw_instances::InstanceBounds>) -> [u8; 32] {
    let Some(bounds) = bounds else {
        return [0; 32];
    };
    let [min_x, min_y, min_z] = bounds.minimum;
    let [max_x, max_y, max_z] = bounds.maximum;
    [min_x, min_y, min_z, 1., max_x, max_y, max_z, 0.]
        .map(f32::to_ne_bytes)
        .as_flattened()
        .try_into()
        .unwrap_or_default()
}

/// Each template uploaded as an ordinary draw of this frame.
fn upload_templates(
    (runtime, pipelines, extent, samples, retirements): TemplateUpload<'_>,
    instances: &DrawInstances,
    uploads: &mut FrameDrawUploads<'_>,
) -> Result<Vec<UploadedDraw>, RenderError> {
    instances
        .templates
        .iter()
        .map(|template| {
            upload_draw(
                runtime,
                template,
                pipelines,
                extent,
                samples,
                retirements,
                uploads,
            )
        })
        .collect()
}

/// Refuse templates that would not record as the head draw does: another pipeline, state or
/// binding, or geometry that does not bind at offset zero.
fn require_one_state(head: &UploadedDraw, templates: &[UploadedDraw]) -> Result<(), RenderError> {
    let first = templates
        .first()
        .ok_or_else(|| rejected("no templates", "at least one placement template"))?;
    let same = |template: &UploadedDraw| {
        Arc::ptr_eq(&template.pipeline, &first.pipeline)
            && plain_uploaded(template)
            && base_geometry(template).is_some()
            && template.presentation == head.presentation
            && template.sample_alpha_cutoff == head.sample_alpha_cutoff
            && same_owner(
                &template.bindings.pbr_environment,
                &head.bindings.pbr_environment,
            )
            && same_owner(&template.bindings.shadow_maps, &head.bindings.shadow_maps)
    };
    if plain_uploaded(head) && templates.iter().all(same) {
        return Ok(());
    }
    Err(rejected(
        templates.len(),
        "placement templates that record with one state",
    ))
}

/// One entry per placement: the clip matrix of this frame, then the world and normal
/// matrices, the bytes its separate draw pushes and binds.
fn table_entries(instances: &DrawInstances) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(instances.placements.len() * ENTRY_BYTES);
    for placement in instances.placements.iter() {
        for matrix in [instances.clip(placement), placement.world, placement.normal] {
            bytes.extend_from_slice(matrix.map(f32::to_ne_bytes).as_flattened());
        }
    }
    bytes
}

/// One `VkDrawIndexedIndirectCommand` per placement, with its place as the first instance.
fn placement_commands(instances: &DrawInstances, templates: &[UploadedDraw]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(instances.placements.len() * COMMAND_BYTES as usize);
    for (instance, placement) in instances.placements.iter().enumerate() {
        let template = &templates[placement.template as usize];
        let (first_index, base_vertex) = base_geometry(template).unwrap_or_default();
        let words = [
            template.count,
            1,
            first_index,
            base_vertex as u32,
            instance as u32,
        ];
        bytes.extend_from_slice(words.map(u32::to_ne_bytes).as_flattened());
    }
    bytes
}

/// A template may share the fragment lighting block only when its non-placement bytes match.
fn lighting_classes(instances: &DrawInstances, indexed: bool) -> Vec<usize> {
    let mut classes = std::collections::HashMap::new();
    instances
        .templates
        .iter()
        .map(|template| {
            let mut bytes = Vec::new();
            if let Some(lighting) = &template.lighting {
                lighting.encode_into(&mut bytes);
            }
            shared_lighting_key(&mut bytes, indexed);
            let next = classes.len();
            *classes.entry(bytes).or_insert(next)
        })
        .collect()
}

fn placement_materials(instances: &DrawInstances) -> Result<DrawMaterialTable, RenderError> {
    let mut materials = DrawMaterialTable::new(instances.placements.len());
    let offsets = instances
        .templates
        .iter()
        .map(|draw| materials.intern(draw))
        .collect::<Result<Vec<_>, _>>()?;
    for (index, placement) in instances.placements.iter().enumerate() {
        materials.set_draw(index, offsets[placement.template as usize]);
    }
    Ok(materials)
}

/// The commands in draw-order runs that bind the same geometry buffers and fragment lighting.
fn segments(
    instances: &DrawInstances,
    templates: &[UploadedDraw],
    lighting: &[usize],
) -> Vec<RunSegment> {
    let mut segments: Vec<RunSegment> = Vec::new();
    for (index, placement) in instances.placements.iter().enumerate() {
        let template_index = placement.template as usize;
        let template = &templates[template_index];
        let buffers = (
            template.vertices.handle,
            template.indices.handle,
            template.index_format,
        );
        match segments.last_mut() {
            Some(last)
                if (last.vertices, last.indices, last.index_format) == buffers
                    && lighting[last.template] == lighting[template_index] =>
            {
                last.count += 1
            }
            _ => segments.push(RunSegment {
                vertices: buffers.0,
                indices: buffers.1,
                index_format: buffers.2,
                template: template_index,
                first: index as u32,
                count: 1,
            }),
        }
    }
    segments
}

#[cfg(test)]
#[path = "draw_instance_upload_tests.rs"]
mod tests;
