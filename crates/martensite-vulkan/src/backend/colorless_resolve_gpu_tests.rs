//! Configured zero-color-write capping passes may change stencil, never resolved RGB.
use super::encoded_resolve_gpu_tests::{assert_capture_equal, encoded_frame, validated_renderer};
use crate::stencil_program::{StencilCommand, StencilProgram};
use crate::{PrimitiveDraw, RenderBackend, RenderCappingPass, RenderFrame, RenderLayer};

#[test]
fn colorless_capping_sequence_preserves_encoded_multisample_resolve() {
    let mut renderer = validated_renderer();
    let baseline = capped_frame(false, false);
    let plan = super::frame_raster_plan::frame_raster_plan(&renderer.runtime, &baseline, true);
    assert_eq!(plan.samples, ash::vk::SampleCountFlags::TYPE_4);
    let before = renderer.render(&baseline).unwrap();
    assert!(
        before
            .rgba
            .as_chunks::<4>()
            .0
            .iter()
            .any(|p| p[0] > 0 && p[0] < 255)
    );
    for split in [false, true] {
        let after = renderer.render(&capped_frame(true, split)).unwrap();
        assert_capture_equal(&before, &after);
    }
    assert!(!renderer.stencil.is_enabled());
    assert!(renderer.validation_errors().unwrap().is_empty());
}
fn capped_frame(with_mask: bool, split: bool) -> RenderFrame {
    let layers = capping_layers(with_mask, split);
    let mut frame = RenderFrame::with_layers(8, 8, [0., 0., 0., 1.], layers).unwrap();
    frame.raster.requested_samples = 4;
    pin_submitted_depth_only_caps(&mut frame);
    if with_mask {
        require_submitted_colorless_caps(&frame);
    }
    let stencil = crate::stencil_plan::StencilPlan::new(&frame, Default::default());
    assert!(stencil.clear_after_layer[usize::from(split)]);
    frame
}
fn capping_layers(with_mask: bool, split: bool) -> Vec<RenderLayer> {
    let mut layers = Vec::new();
    let mut program = StencilProgram::default();
    let draw = encoded_frame(4, false).draws[0].clone();
    if split {
        layers.push(RenderLayer::new(vec![draw], true, true, false));
    } else {
        program.geometry(vec![draw]);
    }
    append_capping_program(&mut program, with_mask);
    layers.push(RenderLayer::from_stencil_program(
        program, true, true, false,
    ));
    layers
}
fn append_capping_program(program: &mut StencilProgram, with_mask: bool) {
    if with_mask {
        program.geometry(colorless_capping_draws());
    }
    program.command(StencilCommand::CappingRestore);
}
fn colorless_capping_draws() -> Vec<PrimitiveDraw> {
    let mut source = encoded_frame(4, false).draws[0].clone();
    source.compatibility_color_target = false;
    let mask = source
        .clone()
        .with_capping_pass(RenderCappingPass::MaskStart)
        .unwrap();
    let surface = source
        .with_capping_pass(RenderCappingPass::Surface)
        .unwrap();
    // The complete capping sequence remains submitted; its configured color masks are zero.
    assert!(
        super::capping_state::MASK_BLEND[0]
            .color_write_mask
            .is_empty()
    );
    vec![mask, surface]
}

fn pin_submitted_depth_only_caps(frame: &mut RenderFrame) {
    // Flattening applies layer defaults, so define this owned low-level input afterward.
    for draw in &mut frame.draws {
        if draw.capping_pass == RenderCappingPass::Surface {
            draw.depth_pass = crate::depth_prepass::RenderDepthPass::DepthOnly;
        }
    }
}
fn require_submitted_colorless_caps(frame: &RenderFrame) {
    for draw in frame
        .draws
        .iter()
        .filter(|draw| draw.capping_pass != RenderCappingPass::None)
    {
        require_submitted_colorless_cap(draw);
    }
}
fn require_submitted_colorless_cap(draw: &PrimitiveDraw) {
    let mut kind = super::pipeline_variants::PipelineVariant::from_index(0, true);
    kind.capping = draw.capping_pass;
    kind.depth_pass = draw.depth_pass;
    let writes = super::pipeline_state::pipeline_writes_color(kind);
    print_submitted_cap_state(draw, writes);
    assert!(
        !writes,
        "submitted capping={:?},depth={:?}; expected zero color mask",
        draw.capping_pass, draw.depth_pass
    );
}
fn print_submitted_cap_state(draw: &PrimitiveDraw, writes: bool) {
    println!(
        r#"{{"event":"owned_submitted_cap_state","capping":"{:?}","depth_pass":"{:?}","color_writes":{},"vertices":{},"indices":{}}}"#,
        draw.capping_pass,
        draw.depth_pass,
        writes,
        draw.vertices.len(),
        draw.indices.len()
    );
}
#[test]
fn colorless_fixture_has_zero_actual_capping_masks_after_layer_flattening() {
    for split in [false, true] {
        let frame = capped_frame(true, split);
        assert_eq!(frame.draws.len(), 3);
        let surface = frame
            .draws
            .iter()
            .find(|draw| draw.capping_pass == RenderCappingPass::Surface)
            .unwrap();
        assert_eq!(
            surface.depth_pass,
            crate::depth_prepass::RenderDepthPass::DepthOnly
        );
    }
}
