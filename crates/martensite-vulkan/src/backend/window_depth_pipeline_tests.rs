use super::*;

#[derive(Default)]
struct RecordingDepthPipelineFactory {
    stages: Vec<(vk::ShaderStageFlags, Option<Vec<u8>>)>,
}
impl RecordingDepthPipelineFactory {
    fn create(
        &mut self,
        stages: &[vk::PipelineShaderStageCreateInfo<'_>],
    ) -> Result<(), RenderError> {
        self.stages = stages
            .iter()
            .map(|stage| {
                // The factory runs synchronously while with_specialization owns every linked slice.
                // SAFETY: the pointer comes from a live Vulkan structure of this scope, and Vulkan's structure rules make it valid to read here.
                let bytes = unsafe { stage.p_specialization_info.as_ref() }.map(|info| {
                    // SAFETY: the pointer and length describe a live allocation or Vulkan output array of this scope, and the view does not outlive it.
                    unsafe { std::slice::from_raw_parts(info.p_data.cast::<u8>(), info.data_size) }
                        .to_vec()
                });
                (stage.stage, bytes)
            })
            .collect();
        Ok(())
    }
}
fn parameters() -> WindowDepthPipelineParameters {
    use crate::{RenderViewportGridPrecision as Grid, RenderWindowDepthBehavior as Behavior};
    WindowDepthPipelineParameters::selected(
        Behavior::UnsnappedWindowDepthV2 {
            reference_grid: Grid::Fractional12,
            raster_grid: Grid::Fractional8,
        },
        vk::Viewport {
            x: 0.,
            y: 0.,
            width: 409.,
            height: 409.,
            min_depth: 0.,
            max_depth: 1.,
        },
        false,
    )
    .unwrap()
    .unwrap()
}
fn stages() -> [vk::PipelineShaderStageCreateInfo<'static>; 3] {
    [
        vk::ShaderStageFlags::VERTEX,
        vk::ShaderStageFlags::FRAGMENT,
        vk::ShaderStageFlags::GEOMETRY,
    ]
    .map(|stage| {
        vk::PipelineShaderStageCreateInfo::default()
            .stage(stage)
            .name(c"main")
    })
}

#[test]
fn geometry_constants_are_exact_and_do_not_replace_vertex_specialization() {
    let mut factory = RecordingDepthPipelineFactory::default();
    let value = 0_u32.to_ne_bytes();
    let old = vk::SpecializationInfo::default().data(&value);
    let mut inputs = stages();
    inputs[0] = inputs[0].specialization_info(&old);
    with_specialization(Some(parameters()), &inputs, |stages| factory.create(stages)).unwrap();
    assert_eq!(factory.stages[0].1, Some(value.to_vec()));
    assert!(factory.stages[1].1.is_none());
    let expected: Vec<_> = parameters()
        .specialization_words()
        .into_iter()
        .flat_map(u32::to_ne_bytes)
        .collect();
    assert_eq!(factory.stages[2].1, Some(expected));
}

#[test]
fn missing_duplicate_or_pre_specialized_geometry_refuses_before_factory() {
    let mut factory = RecordingDepthPipelineFactory::default();
    assert!(
        with_specialization(Some(parameters()), &stages()[..2], |stages| factory
            .create(stages))
        .is_err()
    );
    let mut duplicate = stages().to_vec();
    duplicate.push(stages()[2]);
    assert!(
        with_specialization(Some(parameters()), &duplicate, |stages| factory
            .create(stages))
        .is_err()
    );
    let existing = vk::SpecializationInfo::default();
    let mut inputs = stages();
    inputs[2] = inputs[2].specialization_info(&existing);
    assert!(
        with_specialization(Some(parameters()), &inputs, |stages| factory.create(stages)).is_err()
    );
    assert!(factory.stages.is_empty());
}

#[test]
fn default_route_preserves_two_stage_pipeline_without_new_requirements() {
    let mut factory = RecordingDepthPipelineFactory::default();
    with_specialization(None, &stages()[..2], |stages| factory.create(stages)).unwrap();
    assert_eq!(factory.stages.len(), 2);
    assert!(
        factory
            .stages
            .iter()
            .all(|(_, specialization)| specialization.is_none())
    );
}

#[test]
fn clipped_fan_fragment_and_geometry_share_the_exact_grid_specialization() {
    use crate::{RenderViewportGridPrecision as Grid, RenderWindowDepthBehavior as Behavior};
    let selected = WindowDepthPipelineParameters::selected(
        Behavior::ClippedFanFramebufferV4 {
            fan_grid: Grid::Fractional8,
            raster_grid: Grid::Fractional8,
        },
        parameters().viewport(),
        false,
    )
    .unwrap();
    let mut factory = RecordingDepthPipelineFactory::default();
    with_specialization(selected, &stages(), |actual| factory.create(actual)).unwrap();
    assert!(factory.stages[0].1.is_none());
    assert_eq!(factory.stages[1].1, factory.stages[2].1);
    assert!(factory.stages[1].1.is_some());
}
