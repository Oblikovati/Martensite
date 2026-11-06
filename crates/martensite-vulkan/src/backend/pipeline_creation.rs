//! Keep linked Vulkan descriptions local until the synchronous pipeline creation call.
use super::super::pipeline_shader_stages::shader_stages;
use super::super::{oit_pipeline, pipeline_state, pipeline_variants::PipelineVariant};
use super::{DrawPipeline, create_pipeline};
use crate::RenderError;
use ash::vk;

pub(super) struct PipelineCreationRequest {
    pub(super) kind: PipelineVariant,
    pub(super) oit: Option<oit_pipeline::OitPipelineTarget>,
    pub(super) samples: vk::SampleCountFlags,
    pub(super) alpha_coverage: bool,
}

struct PipelineCreationState {
    stages: Vec<vk::PipelineShaderStageCreateInfo<'static>>,
    state: pipeline_state::PipelineState,
    formats: Vec<vk::Format>,
    blends: Option<Vec<vk::PipelineColorBlendAttachmentState>>,
}

/// Assemble one owned request; `create(owner, request)?` keeps linked nodes on the stack.
pub(super) fn create(
    owner: &DrawPipeline,
    request: PipelineCreationRequest,
) -> Result<vk::Pipeline, RenderError> {
    let prepared = prepare_creation(owner, &request);
    create_prepared(owner, &request, &prepared)
}

fn prepare_creation(
    owner: &DrawPipeline,
    request: &PipelineCreationRequest,
) -> PipelineCreationState {
    let stages = shader_stages(&owner.shaders);
    let mut state = pipeline_state::PipelineState::new(request.kind);
    state.set_multisampling(request.samples, request.alpha_coverage);
    state.set_flat_text(owner.runtime.text_interpolation, request.kind.text);
    let formats = color_formats(request);
    let blends = request
        .oit
        .map(|target| oit_pipeline::geometry_blend_attachments(target, request.kind));
    PipelineCreationState {
        stages,
        state,
        formats,
        blends,
    }
}

fn create_prepared(
    owner: &DrawPipeline,
    request: &PipelineCreationRequest,
    prepared: &PipelineCreationState,
) -> Result<vk::Pipeline, RenderError> {
    let blend_info = vk::PipelineColorBlendStateCreateInfo::default()
        .attachments(prepared.blends.as_deref().unwrap_or(&[]));
    let depth = owner.runtime.depth.formats;
    let mut rendering = rendering_description(request.kind, depth, &prepared.formats);
    let mut info = prepared
        .state
        .description(owner.layout, &prepared.stages)
        .push_next(&mut rendering);
    if request.oit.is_some() {
        info = info.color_blend_state(&blend_info);
    }
    super::super::window_depth_pipeline::with_specialization(
        request.kind.window_depth,
        &prepared.stages,
        |stages| {
            create_with_raster(
                owner,
                request.kind,
                request.samples,
                info.stages(stages),
                &prepared.state,
            )
        },
    )
}

fn color_formats(request: &PipelineCreationRequest) -> Vec<vk::Format> {
    let ordinary = vec![
        if request.kind.text_logic_xor || request.kind.encoded_surface {
            vk::Format::R8G8B8A8_UNORM
        } else {
            vk::Format::R8G8B8A8_SRGB
        },
    ];
    request
        .oit
        .map(|target| target.formats[..target.count].to_vec())
        .unwrap_or(ordinary)
}

fn rendering_description(
    kind: PipelineVariant,
    depth: super::super::depth_equivalence::DepthFormats,
    formats: &[vk::Format],
) -> vk::PipelineRenderingCreateInfo<'_> {
    vk::PipelineRenderingCreateInfo::default()
        .color_attachment_formats(formats)
        .depth_attachment_format(kind.depth_format(depth))
        .stencil_attachment_format(if kind.stencil {
            kind.depth_format(depth)
        } else {
            vk::Format::UNDEFINED
        })
}

fn create_with_raster(
    owner: &DrawPipeline,
    kind: PipelineVariant,
    samples: vk::SampleCountFlags,
    info: vk::GraphicsPipelineCreateInfo<'_>,
    state: &pipeline_state::PipelineState,
) -> Result<vk::Pipeline, RenderError> {
    if kind.line {
        owner
            .runtime
            .lines
            .validate_multisampling(samples, kind.fixed_line_stipple)?;
    }
    let mut line_state = line_description(kind.fixed_line_stipple, samples);
    let mut conservative_state = overestimate_state();
    let conservative_node = std::ptr::from_ref(&conservative_state).cast();
    let exact_overestimate =
        super::super::phong_projected_fill::exact_overestimate_state(&conservative_state);
    let selected = selected_raster(kind, state, &mut line_state, &mut conservative_state);
    // Only the later known provoking-vertex node may extend this already checked raster state.
    super::super::phong_projected_fill::require_state_with_conservative(
        owner.runtime.phong_interpolation,
        owner.phong_interpolation_shader,
        &selected,
        info.stage_count as usize,
        owner.has_v4_geometry_stage(),
        Some(conservative_node),
        exact_overestimate,
    )?;
    super::super::text_flat_corners_layout::require_raster(
        owner.runtime.text_interpolation,
        owner.text_grid_interpolation_shader,
        &selected,
        info.stage_count,
    )?;
    let info = if kind.line || kind.triangle_grid_coverage {
        info.rasterization_state(&selected)
    } else {
        info
    };
    if kind.fixed_grid_point {
        return super::super::fixed_grid_pipeline::create_fixed_grid_pipeline(
            &owner.runtime,
            info,
            &selected,
            kind.clamp,
        );
    }
    create_pipeline(&owner.runtime, info, &selected)
}

fn selected_raster<'a>(
    kind: PipelineVariant,
    state: &pipeline_state::PipelineState,
    line_state: &'a mut vk::PipelineRasterizationLineStateCreateInfoKHR<'static>,
    conservative_state: &'a mut vk::PipelineRasterizationConservativeStateCreateInfoEXT<'static>,
) -> vk::PipelineRasterizationStateCreateInfo<'a> {
    let raster = linked_raster(kind, line_state, conservative_state);
    if kind.line || kind.triangle_grid_coverage {
        return raster;
    }
    state.raster_description()
}

fn line_description(
    stippled: bool,
    samples: vk::SampleCountFlags,
) -> vk::PipelineRasterizationLineStateCreateInfoKHR<'static> {
    // OpenGL 3.1 section 3.5.4 uses rectangular sample coverage when MSAA is enabled.
    let mode = if samples == vk::SampleCountFlags::TYPE_1 {
        vk::LineRasterizationModeKHR::BRESENHAM
    } else {
        vk::LineRasterizationModeKHR::RECTANGULAR
    };
    vk::PipelineRasterizationLineStateCreateInfoKHR::default()
        .line_rasterization_mode(mode)
        .stippled_line_enable(stippled)
        .line_stipple_factor(1)
        .line_stipple_pattern(0xffff)
}

fn linked_raster<'a>(
    kind: PipelineVariant,
    line_state: &'a mut vk::PipelineRasterizationLineStateCreateInfoKHR<'static>,
    conservative_state: &'a mut vk::PipelineRasterizationConservativeStateCreateInfoEXT<'static>,
) -> vk::PipelineRasterizationStateCreateInfo<'a> {
    let mut raster =
        pipeline_state::raster_state(kind.clamp).polygon_mode(kind.polygon_raster_mode());
    if kind.line {
        raster = raster.push_next(line_state);
    }
    if kind.triangle_grid_coverage {
        raster = raster.push_next(conservative_state);
    }
    raster
}

fn overestimate_state() -> vk::PipelineRasterizationConservativeStateCreateInfoEXT<'static> {
    vk::PipelineRasterizationConservativeStateCreateInfoEXT::default()
        .conservative_rasterization_mode(vk::ConservativeRasterizationModeEXT::OVERESTIMATE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authored_msaa_lines_use_rectangular_coverage_and_keep_stipple() {
        let ordinary = line_description(false, vk::SampleCountFlags::TYPE_1);
        assert_eq!(
            ordinary.line_rasterization_mode,
            vk::LineRasterizationModeKHR::BRESENHAM
        );
        for stippled in [false, true] {
            let sampled = line_description(stippled, vk::SampleCountFlags::TYPE_8);
            assert_eq!(
                sampled.line_rasterization_mode,
                vk::LineRasterizationModeKHR::RECTANGULAR
            );
            assert_eq!(sampled.stippled_line_enable, u32::from(stippled));
        }
    }

    #[test]
    fn coverage_carrier_uses_overestimate_mode_without_extra_inflation() {
        let state = overestimate_state();
        assert_eq!(
            state.conservative_rasterization_mode,
            vk::ConservativeRasterizationModeEXT::OVERESTIMATE
        );
        assert_eq!(state.extra_primitive_overestimation_size, 0.0);
    }

    #[test]
    fn coverage_selection_survives_coordinate_and_provoking_policies() {
        for coordinates in [
            crate::RenderCoordinateConvention::LegacyBottomUpV1,
            crate::RenderCoordinateConvention::TopLeftLogicalV2,
        ] {
            assert_policy_preserves_coverage(coordinates, false);
            assert_policy_preserves_coverage(coordinates, true);
        }
    }

    fn assert_policy_preserves_coverage(
        coordinates: crate::RenderCoordinateConvention,
        provoking: bool,
    ) {
        let kind = coverage_kind();
        let state = pipeline_state::PipelineState::new(kind);
        let mut line = line_description(false, vk::SampleCountFlags::TYPE_1);
        let mut conservative = overestimate_state();
        let linked = selected_raster(kind, &state, &mut line, &mut conservative);
        let mut last_vertex = vk::PipelineRasterizationProvokingVertexStateCreateInfoEXT::default();
        let policy = super::super::super::raster_pipeline_policy::select_raster(
            &linked,
            coordinates,
            provoking,
            &mut last_vertex,
        );
        let refused = super::super::super::phong_projected_fill::require_state(
            crate::RenderPhongInterpolationBehavior::UnsnappedV3,
            true,
            &policy,
            2,
            false,
        );
        assert!(refused.is_err());
    }

    fn coverage_kind() -> PipelineVariant {
        PipelineVariant {
            lighting: true,
            triangle_grid_coverage: true,
            ..PipelineVariant::from_index(1, false)
        }
    }
}
