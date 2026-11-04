use super::*;
use crate::{RenderLighting, RenderMaterial, RenderVertex};

fn phong_draw() -> PrimitiveDraw {
    let vertices = vec![RenderVertex::new([0.; 3], [1.; 4]); 3];
    let mut draw = PrimitiveDraw::new(vertices, vec![0, 1, 2], crate::identity_clip()).unwrap();
    let light = RenderLighting::new(
        vec![[0., 0., 1.]; 3],
        RenderMaterial::default(),
        crate::identity_clip(),
        crate::identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap();
    draw = draw.with_lighting(light).unwrap();
    draw
}

fn selected_device() -> WorldClipDevice {
    WorldClipDevice::observed(Behavior::RasterClipDistanceV2, true, 8, 8)
}

fn selected_device_v3() -> WorldClipDevice {
    WorldClipDevice::observed(Behavior::RasterClipDistanceV3, true, 8, 8)
}

fn fixed_phong_draw() -> PrimitiveDraw {
    let mut draw = PrimitiveDraw::new(
        vec![RenderVertex::new([0.; 3], [1.; 4]); 4],
        vec![0, 1, 2, 0, 2, 3],
        crate::identity_clip(),
    )
    .unwrap();
    draw.lighting = Some(crate::backend::fixed_vertex_sources::fixed_test_lighting(
        4,
        RenderShading::Phong,
    ));
    draw.fixed_function_lighting = true;
    draw.clipping = Some(
        crate::RenderClipping::new(vec![vec![[1., 0., 0., -0.25]]], crate::identity_clip())
            .unwrap(),
    );
    draw
}

fn clipped_phong_with_model(model: [f32; 16]) -> PrimitiveDraw {
    let mut draw = phong_draw();
    draw.clipping =
        Some(crate::RenderClipping::new(vec![vec![[1., 0., 0., -0.25]]], model).unwrap());
    draw
}

fn clipped_solid_line() -> PrimitiveDraw {
    let mut draw = PrimitiveDraw::segments(
        vec![RenderVertex::new([0.; 3], [1.; 4]); 2],
        vec![0, 1],
        crate::identity_clip(),
        crate::LineStyle::new(2., 0xffff, 1).unwrap(),
    )
    .unwrap();
    draw.clipping = Some(
        crate::RenderClipping::new(vec![vec![[0., 1., 0., -0.5]]], crate::identity_clip()).unwrap(),
    );
    draw
}

#[test]
fn default_replay_never_requires_clip_distance_support() {
    let device = WorldClipDevice::observed(Behavior::FragmentV1, false, 0, 0);
    assert!(device.admit_device(Behavior::FragmentV1).is_ok());
}

#[test]
fn raster_v2_accepts_opaque_phong_with_single_plane_and_chains() {
    let mut draw = phong_draw();
    draw.clipping = Some(
        crate::RenderClipping::new(
            vec![vec![[1., 0., 0., -0.25]], vec![[0., 1., 0., -0.5]]],
            crate::identity_clip(),
        )
        .unwrap(),
    );
    assert_eq!(
        selected_device().admit_draw(Behavior::RasterClipDistanceV2, &draw, true),
        Ok(true)
    );
}

#[test]
fn raster_v2_accepts_the_fixed_function_phong_microcase() {
    assert_eq!(
        selected_device().admit_draw(Behavior::RasterClipDistanceV2, &fixed_phong_draw(), true),
        Ok(true)
    );
}

#[test]
fn raster_v2_refuses_or_chains_and_more_than_eight_planes() {
    let mut draw = phong_draw();
    draw.clipping = Some(
        crate::RenderClipping::new(
            vec![vec![[1., 0., 0., 0.], [0., 1., 0., 0.]]],
            crate::identity_clip(),
        )
        .unwrap(),
    );
    assert!(
        selected_device()
            .admit_draw(Behavior::RasterClipDistanceV2, &draw, true)
            .is_err()
    );
    draw.clipping = Some(
        crate::RenderClipping::new(
            (0..9).map(|_| vec![[1., 0., 0., 0.]]).collect(),
            crate::identity_clip(),
        )
        .unwrap(),
    );
    assert!(
        selected_device()
            .admit_draw(Behavior::RasterClipDistanceV2, &draw, true)
            .is_err()
    );
}

#[test]
fn raster_v2_refusal_is_typed_for_unsupported_device_and_draw() {
    let unsupported = WorldClipDevice::observed(Behavior::RasterClipDistanceV2, false, 8, 8);
    let error = unsupported
        .admit_device(Behavior::RasterClipDistanceV2)
        .unwrap_err();
    assert_eq!(
        error.world_clipping_failure().unwrap().reason,
        Requirement::ClipDistanceCapacity
    );
    let mut draw = phong_draw();
    draw.line_style = Some(crate::LineStyle::new(1., 0xffff, 1).unwrap());
    draw.clipping = Some(
        crate::RenderClipping::new(vec![vec![[1., 0., 0., 0.]]], crate::identity_clip()).unwrap(),
    );
    let error = selected_device()
        .admit_draw(Behavior::RasterClipDistanceV2, &draw, true)
        .unwrap_err();
    assert!(matches!(
        error.world_clipping_failure().unwrap().reason,
        Requirement::UnsupportedDraw { .. }
    ));
}

#[test]
fn raster_v3_accepts_solid_native_width_lines_and_v2_keeps_its_refusal() {
    let draw = clipped_solid_line();
    assert_eq!(
        selected_device_v3().admit_draw(Behavior::RasterClipDistanceV3, &draw, true),
        Ok(true)
    );
    assert!(
        selected_device()
            .admit_draw(Behavior::RasterClipDistanceV2, &draw, true)
            .is_err()
    );
}

#[test]
fn raster_v3_refuses_stippled_or_blended_line_draws() {
    let mut draw = clipped_solid_line();
    draw.line_style = Some(crate::LineStyle::new(2., 0xcccc, 1).unwrap());
    assert_typed_v3_refusal(&draw, true);
    draw.line_style = Some(
        crate::LineStyle::new(2., 0xffff, 1)
            .unwrap()
            .with_alpha_blending(true),
    );
    assert_typed_v3_refusal(&draw, true);
}

fn assert_typed_v3_refusal(draw: &PrimitiveDraw, compatible: bool) {
    let error = selected_device_v3()
        .admit_draw(Behavior::RasterClipDistanceV3, draw, compatible)
        .unwrap_err();
    assert!(matches!(
        error.world_clipping_failure().unwrap().reason,
        Requirement::UnsupportedDraw { .. }
    ));
}

#[test]
fn raster_v2_refuses_nonpositive_or_nonfinite_homogeneous_weights() {
    let mut draw = clipped_phong_with_model(crate::identity_clip());
    for weight in [0., -1., f32::from_bits(1), f32::NAN] {
        draw.vertices[1].weight = weight;
        let error = selected_device()
            .admit_draw(Behavior::RasterClipDistanceV2, &draw, true)
            .unwrap_err();
        assert!(matches!(
            error.world_clipping_failure().unwrap().reason,
            Requirement::UnsupportedDraw { ref actual } if actual.contains("expected positive normal finite W")
        ));
    }
}

#[test]
fn raster_v2_accepts_positive_nonunit_homogeneous_vertices() {
    let mut draw = clipped_phong_with_model(crate::identity_clip());
    for vertex in &mut draw.vertices {
        vertex.weight = 2.;
    }
    assert_eq!(
        selected_device().admit_draw(Behavior::RasterClipDistanceV2, &draw, true),
        Ok(true)
    );
}

#[test]
fn raster_v2_refuses_nonopaque_material_and_vertex_sources() {
    let mut material = phong_draw();
    material.lighting.as_mut().unwrap().material.set_alpha(0.5);
    material.clipping = clipped_phong_with_model(crate::identity_clip()).clipping;
    assert_typed_draw_refusal(&material);

    let mut vertex = clipped_phong_with_model(crate::identity_clip());
    vertex.source_color_array = true;
    vertex.vertices[0].color[3] = 0.75;
    assert_typed_draw_refusal(&vertex);

    let mut fixed = fixed_phong_draw();
    fixed.lighting.as_mut().unwrap().fixed_source_opacity = Some([1., 0.5]);
    assert_typed_draw_refusal(&fixed);
}

fn assert_typed_draw_refusal(draw: &PrimitiveDraw) {
    let error = selected_device()
        .admit_draw(Behavior::RasterClipDistanceV2, draw, true)
        .unwrap_err();
    assert!(matches!(
        error.world_clipping_failure().unwrap().reason,
        Requirement::UnsupportedDraw { .. }
    ));
}

#[test]
fn raster_v2_refuses_world_or_plane_arithmetic_that_can_overflow() {
    let mut model = crate::identity_clip();
    model[0] = f32::MAX;
    let mut draw = clipped_phong_with_model(model);
    draw.vertices[0].position[0] = 2.;
    assert_typed_draw_refusal(&draw);

    let mut draw = clipped_phong_with_model(crate::identity_clip());
    draw.clipping = Some(
        crate::RenderClipping::new(vec![vec![[f32::MAX, 0., 0., 0.]]], crate::identity_clip())
            .unwrap(),
    );
    draw.vertices[0].position[0] = 2.;
    assert_typed_draw_refusal(&draw);
}

#[test]
fn fragment_policy_is_unselected_even_when_v4_bootstrapped_clip_capability() {
    let mut draw = fixed_phong_draw();
    draw.lighting = None;
    let device = selected_device();
    assert!(
        !device
            .admit_draw(Behavior::FragmentV1, &draw, false)
            .unwrap()
    );
    assert!(
        device
            .admit_draw(Behavior::RasterClipDistanceV2, &draw, true)
            .is_err()
    );
    assert!(
        selected_device_v3()
            .admit_draw(Behavior::RasterClipDistanceV3, &draw, true)
            .is_err()
    );
}

fn fixed_mask_draw() -> PrimitiveDraw {
    let mut draw = fixed_phong_draw();
    draw.lighting = None;
    draw.fixed_function_lighting = false;
    draw.fixed_function_mask = true;
    draw.capping_pass = crate::RenderCappingPass::MaskStart;
    draw
}

#[test]
fn version_four_selects_fixed_function_surfaces_and_their_stencil_masks() {
    assert!(fixed_function_triangles(&fixed_phong_draw()));
    assert!(fixed_function_triangles(&fixed_mask_draw()));
    let device = WorldClipDevice::observed(Behavior::RasterFixedFunctionV4, true, 8, 8);
    assert!(device.enabled());
    for draw in [fixed_phong_draw(), fixed_mask_draw()] {
        assert_eq!(
            device.admit_draw(Behavior::RasterFixedFunctionV4, &draw, false),
            Ok(true)
        );
    }
}

#[test]
fn version_four_leaves_programmable_unplaned_and_chained_draws_on_the_discard() {
    let mut programmable = fixed_phong_draw();
    programmable.fixed_function_lighting = false;
    assert!(!fixed_function_triangles(&programmable));
    let mut unplaned = fixed_phong_draw();
    unplaned.clipping = None;
    assert!(!fixed_function_triangles(&unplaned));
    let mut chained = fixed_phong_draw();
    chained.clipping = Some(
        crate::RenderClipping::new(
            vec![vec![[0., 0., 1., 0.], [1., 0., 0., 0.]]],
            crate::identity_clip(),
        )
        .unwrap(),
    );
    assert!(!fixed_function_triangles(&chained));
}
