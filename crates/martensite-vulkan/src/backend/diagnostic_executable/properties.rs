//! Driver stage flags describe reported executables; they are not a universal one-to-one proof.
use super::{
    api::ExecutableApi,
    metadata::bounded_text,
    records::{ExecutableGap, ExecutableProperty, contract},
};
use ash::{vk, vk::Handle};
const PROPERTY_LIMIT: usize = 16;

/// Retain all bounded properties; `query(api, pipeline, &mut raw)?`.
pub(super) fn query(
    api: &mut dyn ExecutableApi,
    pipeline: vk::Pipeline,
    raw: &mut super::raw::RawQueryEvidence,
) -> Result<Vec<ExecutableProperty>, ExecutableGap> {
    let mut slots = vec![vk::PipelineExecutablePropertiesKHR::default(); PROPERTY_LIMIT];
    let (status, count) = api.properties(pipeline, &mut slots);
    raw.calls
        .push(super::raw::property_call(status, count, &slots));
    require_property_status(pipeline, status, count)?;
    slots[..count as usize]
        .iter()
        .enumerate()
        .map(|(index, row)| property(index, row))
        .collect()
}
fn require_property_status(
    pipeline: vk::Pipeline,
    status: vk::Result,
    count: u32,
) -> Result<(), ExecutableGap> {
    require_complete(
        "executable properties",
        pipeline,
        status,
        count,
        PROPERTY_LIMIT,
    )
}
fn property(
    index: usize,
    row: &vk::PipelineExecutablePropertiesKHR<'_>,
) -> Result<ExecutableProperty, ExecutableGap> {
    Ok(ExecutableProperty {
        index: index as u32,
        stages: row.stages.as_raw(),
        name: bounded_text(&row.name)?,
        description: bounded_text(&row.description)?,
        subgroup_size: row.subgroup_size,
    })
}
/// Require one reported fragment executable; `fragment_index(&properties)?`.
pub(super) fn fragment_index(rows: &[ExecutableProperty]) -> Result<u32, ExecutableGap> {
    let candidates: Vec<_> = rows
        .iter()
        .filter(|row| row.stages & vk::ShaderStageFlags::FRAGMENT.as_raw() != 0)
        .collect();
    if candidates.len() != 1 || candidates[0].stages != vk::ShaderStageFlags::FRAGMENT.as_raw() {
        return Err(ExecutableGap::AmbiguousStage(rows.to_vec()));
    }
    Ok(candidates[0].index)
}
/// Refuse prefixes and original API errors; `require_complete(op, pipeline, status, count, capacity)?`.
pub(super) fn require_complete(
    operation: &'static str,
    pipeline: vk::Pipeline,
    status: vk::Result,
    count: u32,
    capacity: usize,
) -> Result<(), ExecutableGap> {
    require_status(operation, pipeline, status)?;
    if count == 0 || count as usize > capacity {
        return Err(contract(
            "written count",
            (operation, count, capacity),
            "positive count within fixed capacity",
        ));
    }
    Ok(())
}
fn require_status(
    operation: &'static str,
    pipeline: vk::Pipeline,
    status: vk::Result,
) -> Result<(), ExecutableGap> {
    if status != vk::Result::SUCCESS {
        return Err(ExecutableGap::Api {
            operation,
            pipeline: pipeline.as_raw(),
            result: status,
        });
    }
    Ok(())
}
