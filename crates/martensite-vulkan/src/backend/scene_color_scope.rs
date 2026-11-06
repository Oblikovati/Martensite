//! A zero-color-mask scope keeps its pipeline format but cannot replace a prior color resolve.
use super::render_target_record::{begin_colorless_draw_rendering, resume_color_draw_rendering};
use super::{frame::FrameResources, pipeline_variants::PipelineVariant, runtime::VulkanRuntime};
use ash::vk;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct SceneColorScope {
    active: bool,
    encoded: bool,
    colorless: bool,
}
impl SceneColorScope {
    pub fn initialized_srgb() -> Self {
        Self {
            active: true,
            ..Self::default()
        }
    }

    pub fn for_pipeline(encoded: bool, kind: PipelineVariant, ordinary: bool) -> Self {
        Self {
            active: ordinary,
            encoded: ordinary && encoded,
            colorless: ordinary && !super::pipeline_state::pipeline_writes_color(kind),
        }
    }
    pub fn advance(
        &mut self,
        runtime: &VulkanRuntime,
        command: vk::CommandBuffer,
        resources: &FrameResources,
        next: Self,
    ) {
        if *self == next {
            return;
        }
        *self = next;
        super::record::finish_rendering(runtime, command);
        super::record::layer_dependency(runtime, command);
        self.begin(runtime, command, resources);
    }
    fn begin(
        &self,
        runtime: &VulkanRuntime,
        command: vk::CommandBuffer,
        resources: &FrameResources,
    ) {
        if self.colorless {
            begin_colorless_draw_rendering(runtime, command, resources, self.encoded);
            return;
        }
        resume_color_draw_rendering(runtime, command, resources, self.encoded);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_zero_color_masks_select_colorless_without_changing_alias() {
        let mut kind = PipelineVariant::from_index(0, true);
        for encoded in [false, true] {
            kind.capping = crate::RenderCappingPass::MaskStart;
            require_colorless(kind, encoded);
            kind.capping = crate::RenderCappingPass::MaskContinue;
            require_colorless(kind, encoded);
            kind.capping = crate::RenderCappingPass::Surface;
            assert!(!SceneColorScope::for_pipeline(encoded, kind, true).colorless);
            kind.depth_pass = crate::depth_prepass::RenderDepthPass::DepthOnly;
            require_colorless(kind, encoded);
            kind.depth_pass = Default::default();
        }
    }
    fn require_colorless(kind: PipelineVariant, encoded: bool) {
        let expected = SceneColorScope {
            active: true,
            encoded,
            colorless: true,
        };
        assert_eq!(SceneColorScope::for_pipeline(encoded, kind, true), expected);
        assert_eq!(
            SceneColorScope::for_pipeline(encoded, kind, false),
            SceneColorScope::default()
        );
    }
    #[test]
    fn first_ordinary_draw_requires_an_explicit_color_scope() {
        let kind = PipelineVariant::from_index(0, false);
        let scope = SceneColorScope::for_pipeline(false, kind, true);
        assert_ne!(scope, SceneColorScope::default());
        assert_eq!(scope, SceneColorScope::initialized_srgb());
        assert_eq!(
            SceneColorScope::for_pipeline(false, kind, false),
            SceneColorScope::default()
        );
    }
    #[test]
    fn potentially_discarded_color_writers_keep_existing_resolve_contract() {
        let mut kind = PipelineVariant::from_index(0, true);
        for pass in [
            crate::RenderCappingPass::None,
            crate::RenderCappingPass::Surface,
        ] {
            kind.capping = pass;
            assert!(!SceneColorScope::for_pipeline(false, kind, true).colorless);
        }
    }
}
