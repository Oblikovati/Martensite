//! Specialization storage remains alive through the existing synchronous pipeline factory.
use super::window_depth_parameters::WindowDepthPipelineParameters;
use crate::{RenderError, RenderWindowDepthFailure as Failure};
use ash::vk;

pub(super) fn with_specialization<T>(
    parameters: Option<WindowDepthPipelineParameters>,
    stages: &[vk::PipelineShaderStageCreateInfo<'_>],
    create: impl FnOnce(&[vk::PipelineShaderStageCreateInfo<'_>]) -> Result<T, RenderError>,
) -> Result<T, RenderError> {
    let Some(parameters) = parameters else {
        return create(stages);
    };
    require_geometry_stage(stages)?;
    let bytes: Vec<_> = parameters
        .specialization_words()
        .into_iter()
        .flat_map(u32::to_ne_bytes)
        .collect();
    let entries: [_; 7] = std::array::from_fn(|index| vk::SpecializationMapEntry {
        constant_id: 20 + index as u32,
        offset: 4 * index as u32,
        size: 4,
    });
    let specialization = vk::SpecializationInfo::default()
        .data(&bytes)
        .map_entries(&entries);
    let actual: Vec<_> = stages
        .iter()
        .map(|stage| {
            if stage.stage == vk::ShaderStageFlags::GEOMETRY
                || (parameters.clipped_fan() && stage.stage == vk::ShaderStageFlags::FRAGMENT)
            {
                stage.specialization_info(&specialization)
            } else {
                *stage
            }
        })
        .collect();
    create(&actual)
}

fn require_geometry_stage(
    stages: &[vk::PipelineShaderStageCreateInfo<'_>],
) -> Result<(), RenderError> {
    let geometry: Vec<_> = stages
        .iter()
        .filter(|stage| stage.stage == vk::ShaderStageFlags::GEOMETRY)
        .collect();
    if geometry.len() == 1 && geometry[0].p_specialization_info.is_null() {
        return Ok(());
    }
    Err(Failure::UnsupportedInterface {
        actual: "missing, ambiguous or already specialized geometry stage".into(),
    }
    .into())
}

#[cfg(test)]
#[path = "window_depth_pipeline_tests.rs"]
mod tests;
