//! CPU command-state and closed-form depth contracts, not GPU execution evidence.
use super::*;
use crate::{RenderCappingPass, RenderFaceCulling, depth_prepass::RenderDepthPass};

fn presentation() -> DrawPresentation {
    DrawPresentation {
        annotation: false,
        stencil_rect: false,
        text_shadow_bias: false,
        text_logic_xor: false,
        encoded_surface: false,
        polygon_offset: None,
        polygon: true,
        polygon_edge: false,
        surface_blend: false,
        capping: RenderCappingPass::None,
        mirrored: false,
        face_culling: RenderFaceCulling::None,
    }
}
fn ordinary_kind() -> PipelineVariant {
    PipelineVariant::from_index(0, false)
}
#[test]
fn disabled_layer_comparison_retains_requested_depth_write() {
    let state = resolve_layer_draw_depth((false, true), false, &presentation(), ordinary_kind());
    assert_eq!(
        state,
        DepthCommands {
            test: true,
            write: true,
            compare: vk::CompareOp::ALWAYS
        }
    );
    let no_write =
        resolve_layer_draw_depth((false, false), false, &presentation(), ordinary_kind());
    assert!(!no_write.test && !no_write.write);
    assert_eq!(no_write.compare, vk::CompareOp::ALWAYS);
}
#[test]
fn ordinary_depth_tests_keep_the_pipeline_comparison() {
    let mut kind = ordinary_kind();
    let ordinary = resolve_layer_draw_depth((true, true), true, &presentation(), kind);
    assert_eq!(ordinary.compare, vk::CompareOp::LESS_OR_EQUAL);
    assert!(ordinary.test && ordinary.write);
    kind.depth_pass = RenderDepthPass::DepthEqual;
    let equal = resolve_layer_draw_depth((true, false), true, &presentation(), kind);
    assert_eq!(equal.compare, vk::CompareOp::EQUAL);
    assert!(equal.test && !equal.write);
}
#[test]
fn annotation_and_stencil_disable_testing_without_enabling_writes() {
    let mut draw = presentation();
    draw.annotation = true;
    let annotation = resolve_layer_draw_depth((false, true), false, &draw, ordinary_kind());
    assert!(!annotation.test);
    assert_eq!(annotation.compare, vk::CompareOp::LESS_OR_EQUAL);
    draw.annotation = false;
    draw.stencil_rect = true;
    assert!(!resolve_layer_draw_depth((false, true), false, &draw, ordinary_kind()).test);
}
#[test]
fn capping_keeps_masks_disabled_and_surface_less_comparison() {
    let mut draw = presentation();
    let mut kind = ordinary_kind();
    draw.capping = RenderCappingPass::MaskStart;
    kind.capping = draw.capping;
    let mask = resolve_layer_draw_depth((false, false), false, &draw, kind);
    assert!(!mask.test && !mask.write);
    draw.capping = RenderCappingPass::Surface;
    kind.capping = draw.capping;
    let surface = resolve_layer_draw_depth((true, true), true, &draw, kind);
    assert_eq!(surface.compare, vk::CompareOp::LESS);
    assert!(surface.test && surface.write);
}

struct FakeDepthAttachment {
    value: f32,
}
impl FakeDepthAttachment {
    fn draw(&mut self, state: DepthCommands, fragment: f32) -> bool {
        let passes = !state.test
            || match state.compare {
                vk::CompareOp::ALWAYS => true,
                vk::CompareOp::LESS_OR_EQUAL => fragment <= self.value,
                _ => panic!(
                    "comparison {:?}; expected ALWAYS or LESS_OR_EQUAL",
                    state.compare
                ),
            };
        if passes && state.test && state.write {
            self.value = fragment;
        }
        passes
    }
}
#[test]
fn later_three_dimensional_fragment_is_occluded_by_comparison_disabled_layer() {
    let first = resolve_layer_draw_depth((false, true), false, &presentation(), ordinary_kind());
    let later = resolve_layer_draw_depth((true, true), true, &presentation(), ordinary_kind());
    let mut native_contract = FakeDepthAttachment { value: 1.0 };
    assert!(native_contract.draw(first, 0.25));
    assert_eq!(native_contract.value, 0.25);
    assert!(!native_contract.draw(later, 0.5));
    let old_mapping = DepthCommands {
        test: false,
        write: true,
        compare: vk::CompareOp::LESS_OR_EQUAL,
    };
    let mut old = FakeDepthAttachment { value: 1.0 };
    assert!(old.draw(old_mapping, 0.25));
    assert_eq!(old.value, 1.0);
    assert!(old.draw(later, 0.5));
}
#[test]
fn disabled_writes_preserve_the_attachment_value() {
    let state = resolve_layer_draw_depth((false, false), false, &presentation(), ordinary_kind());
    let mut attachment = FakeDepthAttachment { value: 0.75 };
    assert!(attachment.draw(state, 0.25));
    assert_eq!(attachment.value, 0.75);
}
