//! Fixed state for the opaque unlit acceptance slice; unsupported styles fail before upload.
use crate::depth_prepass::RenderDepthPass::{DepthEqual, DepthOnly};
use ash::vk;
const BINDINGS: [vk::VertexInputBindingDescription; 1] = [vk::VertexInputBindingDescription {
    binding: 0,
    stride: 32,
    input_rate: vk::VertexInputRate::VERTEX,
}];
const ATTRIBUTES: [vk::VertexInputAttributeDescription; 3] = [
    vk::VertexInputAttributeDescription {
        location: 0,
        binding: 0,
        format: vk::Format::R32G32B32A32_SFLOAT,
        offset: 0,
    },
    vk::VertexInputAttributeDescription {
        location: 1,
        binding: 0,
        format: vk::Format::R32G32B32A32_SFLOAT,
        offset: 16,
    },
    vk::VertexInputAttributeDescription {
        location: 2,
        binding: 0,
        format: vk::Format::R32G32_SFLOAT,
        offset: 32,
    },
];
const BLEND: [vk::PipelineColorBlendAttachmentState; 1] = [vk::PipelineColorBlendAttachmentState {
    blend_enable: vk::FALSE,
    src_color_blend_factor: vk::BlendFactor::ZERO,
    dst_color_blend_factor: vk::BlendFactor::ZERO,
    color_blend_op: vk::BlendOp::ADD,
    src_alpha_blend_factor: vk::BlendFactor::ZERO,
    dst_alpha_blend_factor: vk::BlendFactor::ZERO,
    alpha_blend_op: vk::BlendOp::ADD,
    // Native buffersOpaqueAlpha keeps the cleared attachment alpha even for opaque draws.
    color_write_mask: vk::ColorComponentFlags::from_raw(0x7),
}];
const POINT_BLEND: [vk::PipelineColorBlendAttachmentState; 1] =
    [vk::PipelineColorBlendAttachmentState {
        blend_enable: vk::TRUE,
        src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
        dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
        color_blend_op: vk::BlendOp::ADD,
        src_alpha_blend_factor: vk::BlendFactor::SRC_ALPHA,
        dst_alpha_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
        alpha_blend_op: vk::BlendOp::ADD,
        // GlContext::SetColorMask at honors buffersOpaqueAlpha=true.
        color_write_mask: vk::ColorComponentFlags::from_raw(0x7),
    }];
/// The ordinary blends with alpha writes: GlContext::SetColorMask with buffersOpaqueAlpha off.
const BLEND_ALPHA: [vk::PipelineColorBlendAttachmentState; 1] = [with_alpha_writes(BLEND[0])];
const POINT_BLEND_ALPHA: [vk::PipelineColorBlendAttachmentState; 1] =
    [with_alpha_writes(POINT_BLEND[0])];
const fn with_alpha_writes(
    mut state: vk::PipelineColorBlendAttachmentState,
) -> vk::PipelineColorBlendAttachmentState {
    state.color_write_mask = vk::ColorComponentFlags::RGBA;
    state
}
/// The colour attachment state of a variant; capping masks and depth-only passes write no colour.
fn color_blend(
    kind: super::pipeline_variants::PipelineVariant,
) -> &'static [vk::PipelineColorBlendAttachmentState] {
    let blended =
        kind.point_blend || (kind.text && kind.text_blend) || kind.surface_blend || kind.line_blend;
    match (
        kind.capping.is_mask() || kind.depth_pass == DepthOnly,
        blended,
        kind.alpha_writes,
    ) {
        (true, _, _) => &super::capping_state::MASK_BLEND,
        (false, true, false) => &POINT_BLEND,
        (false, true, true) => &POINT_BLEND_ALPHA,
        (false, false, false) => &BLEND,
        (false, false, true) => &BLEND_ALPHA,
    }
}
/// Query actual blend masks; `pipeline_writes_color(kind)` includes depth-only and capping.
pub(super) fn pipeline_writes_color(kind: super::pipeline_variants::PipelineVariant) -> bool {
    color_blend(kind)
        .iter()
        .any(|attachment| !attachment.color_write_mask.is_empty())
}
pub(super) struct PipelineState {
    vertex: vk::PipelineVertexInputStateCreateInfo<'static>,
    assembly: vk::PipelineInputAssemblyStateCreateInfo<'static>,
    viewport: vk::PipelineViewportStateCreateInfo<'static>,
    raster: vk::PipelineRasterizationStateCreateInfo<'static>,
    samples: vk::PipelineMultisampleStateCreateInfo<'static>,
    depth: vk::PipelineDepthStencilStateCreateInfo<'static>,
    blend: vk::PipelineColorBlendStateCreateInfo<'static>,
    dynamic: vk::PipelineDynamicStateCreateInfo<'static>,
}
impl PipelineState {
    pub(super) fn set_flat_text(
        &mut self,
        behavior: crate::RenderTextInterpolationBehavior,
        is_text: bool,
    ) {
        if is_text
            && behavior == crate::RenderTextInterpolationBehavior::ProjectedGrid12FlatCornersV4
        {
            self.vertex = super::text_flat_corners_layout::vertex_state();
        }
    }
    pub fn set_multisampling(&mut self, samples: vk::SampleCountFlags, alpha_coverage: bool) {
        self.samples = self
            .samples
            .rasterization_samples(samples)
            .alpha_to_coverage_enable(alpha_coverage);
    }
    pub fn new(kind: super::pipeline_variants::PipelineVariant) -> Self {
        let super::pipeline_variants::PipelineVariant {
            clamp: depth_clamp,
            line,
            point,
            text,
            ..
        } = kind;
        let mut state = Self {
            vertex: if kind.fixed_lighting {
                super::fixed_vertex_layout::fixed_vertex_layout(
                    kind.surface_texture || kind.surface_alpha || kind.mesh_edges,
                )
            } else if kind.surface_texture || kind.surface_alpha || kind.mesh_edges {
                super::surface_texture_pipeline::surface_vertex_state(
                    kind.lighting || kind.sphere_map,
                )
            } else {
                vertex_state(text, kind.lighting)
            },
            assembly: vk::PipelineInputAssemblyStateCreateInfo::default().topology(
                if point && !kind.fixed_grid_point {
                    vk::PrimitiveTopology::POINT_LIST
                } else if line && !kind.polygon_edges {
                    if kind.fixed_line_strip {
                        vk::PrimitiveTopology::LINE_STRIP
                    } else {
                        vk::PrimitiveTopology::LINE_LIST
                    }
                } else {
                    vk::PrimitiveTopology::TRIANGLE_LIST
                },
            ),
            viewport: vk::PipelineViewportStateCreateInfo::default()
                .viewport_count(1)
                .scissor_count(1),
            raster: raster_state(depth_clamp).polygon_mode(kind.polygon_raster_mode()),
            samples: vk::PipelineMultisampleStateCreateInfo::default()
                .rasterization_samples(vk::SampleCountFlags::TYPE_1),
            depth: depth_pass_state(kind),
            blend: vk::PipelineColorBlendStateCreateInfo::default()
                .logic_op_enable(kind.text_logic_xor)
                .logic_op(vk::LogicOp::XOR)
                .attachments(color_blend(kind)),
            dynamic: vk::PipelineDynamicStateCreateInfo::default()
                .dynamic_states(dynamic_states(line, kind.fixed_line_stipple)),
        };
        if kind.fixed_grid_point {
            state.vertex = super::fixed_grid_pipeline::fixed_grid_instance_input(state.vertex);
        }
        state
    }
    /// Copy the authored raster state for admission; `state.raster_description()` preserves its fields.
    pub(super) fn raster_description(&self) -> vk::PipelineRasterizationStateCreateInfo<'static> {
        // The local policy chain must not mutate the authored state.
        self.raster
    }
    #[cfg(test)]
    pub(super) fn assembly_topology(&self) -> vk::PrimitiveTopology {
        self.assembly.topology
    }
    pub fn description<'a>(
        &'a self,
        layout: vk::PipelineLayout,
        stages: &'a [vk::PipelineShaderStageCreateInfo<'a>],
    ) -> vk::GraphicsPipelineCreateInfo<'a> {
        vk::GraphicsPipelineCreateInfo::default()
            .stages(stages)
            .vertex_input_state(&self.vertex)
            .input_assembly_state(&self.assembly)
            .viewport_state(&self.viewport)
            .rasterization_state(&self.raster)
            .multisample_state(&self.samples)
            .depth_stencil_state(&self.depth)
            .color_blend_state(&self.blend)
            .dynamic_state(&self.dynamic)
            .layout(layout)
    }
}
fn vertex_state(text: bool, lighting: bool) -> vk::PipelineVertexInputStateCreateInfo<'static> {
    const TEXT_BINDING: [vk::VertexInputBindingDescription; 1] =
        [vk::VertexInputBindingDescription {
            binding: 0,
            stride: 40,
            input_rate: vk::VertexInputRate::VERTEX,
        }];
    const LIT_BINDING: [vk::VertexInputBindingDescription; 1] =
        [vk::VertexInputBindingDescription {
            binding: 0,
            stride: 44,
            input_rate: vk::VertexInputRate::VERTEX,
        }];
    const LIT_ATTRIBUTES: [vk::VertexInputAttributeDescription; 3] = [
        ATTRIBUTES[0],
        ATTRIBUTES[1],
        vk::VertexInputAttributeDescription {
            location: 2,
            binding: 0,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: 32,
        },
    ];
    vk::PipelineVertexInputStateCreateInfo::default()
        .vertex_binding_descriptions(if lighting {
            &LIT_BINDING
        } else if text {
            &TEXT_BINDING
        } else {
            &BINDINGS
        })
        .vertex_attribute_descriptions(if lighting {
            &LIT_ATTRIBUTES
        } else {
            &ATTRIBUTES[..if text { 3 } else { 2 }]
        })
}
pub(super) fn raster_state(depth_clamp: bool) -> vk::PipelineRasterizationStateCreateInfo<'static> {
    vk::PipelineRasterizationStateCreateInfo::default()
        .depth_clamp_enable(depth_clamp)
        .depth_bias_enable(false)
        .polygon_mode(vk::PolygonMode::FILL)
        .cull_mode(vk::CullModeFlags::NONE)
        .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
        .line_width(1.0)
}

/// Preserve pre-pass EQUAL and capping LESS; `depth_pass_state(kind).depth_compare_op`.
pub(super) fn depth_pass_state(
    kind: super::pipeline_variants::PipelineVariant,
) -> vk::PipelineDepthStencilStateCreateInfo<'static> {
    let state = super::capping_state::depth_state(kind.capping);
    if kind.depth_pass == DepthEqual && kind.capping == crate::RenderCappingPass::None {
        return state.depth_compare_op(vk::CompareOp::EQUAL);
    }
    state
}
const RASTER_DYNAMIC_STATES: [vk::DynamicState; 16] = [
    vk::DynamicState::VIEWPORT,
    vk::DynamicState::SCISSOR,
    vk::DynamicState::DEPTH_TEST_ENABLE,
    vk::DynamicState::DEPTH_WRITE_ENABLE,
    vk::DynamicState::DEPTH_COMPARE_OP,
    vk::DynamicState::CULL_MODE,
    vk::DynamicState::FRONT_FACE,
    vk::DynamicState::DEPTH_BIAS,
    vk::DynamicState::DEPTH_BIAS_ENABLE,
    vk::DynamicState::STENCIL_TEST_ENABLE,
    vk::DynamicState::STENCIL_OP,
    vk::DynamicState::STENCIL_COMPARE_MASK,
    vk::DynamicState::STENCIL_WRITE_MASK,
    vk::DynamicState::STENCIL_REFERENCE,
    vk::DynamicState::LINE_WIDTH,
    vk::DynamicState::LINE_STIPPLE_KHR,
];
fn dynamic_states(line: bool, stipple: bool) -> &'static [vk::DynamicState] {
    &RASTER_DYNAMIC_STATES[..if stipple {
        16
    } else if line {
        15
    } else {
        14
    }]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn polygon_points_keep_triangle_assembly_and_point_raster() {
        let mut kind = super::super::pipeline_variants::PipelineVariant::from_index(0, false);
        kind.polygon_points = true;
        let state = PipelineState::new(kind);
        assert_eq!(
            state.assembly.topology,
            vk::PrimitiveTopology::TRIANGLE_LIST
        );
        assert_eq!(state.raster.polygon_mode, vk::PolygonMode::POINT);
    }
    #[test]
    fn default_opaque_buffer_preserves_alpha_during_annotation_blending() {
        assert!(
            !POINT_BLEND[0]
                .color_write_mask
                .contains(vk::ColorComponentFlags::A)
        );
        assert!(
            POINT_BLEND[0]
                .color_write_mask
                .contains(vk::ColorComponentFlags::R)
        );
        assert_eq!(
            POINT_BLEND[0].src_color_blend_factor,
            vk::BlendFactor::SRC_ALPHA
        );
    }
    #[test]
    fn triangle_pipelines_do_not_require_dynamic_line_width() {
        assert!(!dynamic_states(false, false).contains(&vk::DynamicState::LINE_WIDTH));
        assert!(dynamic_states(true, false).contains(&vk::DynamicState::LINE_WIDTH));
        assert_eq!(
            &dynamic_states(true, false)[..14],
            dynamic_states(false, false)
        );
    }
}

#[cfg(test)]
mod xor_tests {
    use super::*;
    #[test]
    fn xor_uses_integer_logic_and_native_opaque_alpha_mask() {
        let mut kind = super::super::pipeline_variants::text_xor_variant(40, false);
        let state = PipelineState::new(kind);
        assert_eq!(state.blend.logic_op_enable, vk::TRUE);
        assert_eq!(state.blend.logic_op, vk::LogicOp::XOR);
        // SAFETY: the pointer was set by this scope's live Vulkan structure chain, and the structure type names what it points at.
        let attachment = unsafe { &*state.blend.p_attachments };
        assert_eq!(attachment.blend_enable, vk::FALSE);
        assert_eq!(attachment.color_write_mask.as_raw(), 7);
        kind.text_logic_xor = false;
        assert_eq!(PipelineState::new(kind).blend.logic_op_enable, vk::FALSE);
    }
}

#[cfg(all(test, feature = "lit-only-command-diagnostics"))]
#[test]
fn lit_only_ordinary_point_pipeline_has_no_active_alpha_to_coverage() {
    let kind = super::pipeline_variants::PipelineVariant::from_index(4, false);
    let state = PipelineState::new(kind);
    assert_eq!(
        state.samples.rasterization_samples,
        vk::SampleCountFlags::TYPE_1
    );
    assert_eq!(state.samples.alpha_to_coverage_enable, vk::FALSE);
}
