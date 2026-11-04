use super::*;
use crate::{
    RenderClipping, RenderLighting, RenderProjectionFactors, RenderShading, identity_clip,
};

fn translated(z: f32) -> [f32; 16] {
    let mut matrix = identity_clip();
    matrix[14] = z;
    matrix
}
fn fixed_clipped_draw() -> PrimitiveDraw {
    let vertices = vec![RenderVertex::new([1., 0., 0.], [1.; 4]); 3];
    let mut draw = PrimitiveDraw::new(vertices, vec![0, 1, 2], identity_clip()).unwrap();
    draw.fixed_function_lighting = true;
    draw.clipping =
        Some(RenderClipping::new(vec![vec![[0., 0., 1., 0.]]], identity_clip()).unwrap());
    draw.fixed_function_factors = Some(
        RenderProjectionFactors::new(identity_clip(), translated(-2.), translated(5.)).unwrap(),
    );
    draw
}

#[test]
fn earlier_versions_and_programmable_draws_keep_their_source_draw() {
    let draw = fixed_clipped_draw();
    assert!(fixed_function(Behavior::FactorizedLinesV3, false, &draw).is_none());
    let mut programmable = draw;
    programmable.fixed_function_lighting = false;
    assert!(fixed_function(Behavior::FixedFunctionModelViewV4, true, &programmable).is_none());
}

#[test]
fn version_four_composes_view_with_model_and_keeps_world_planes() {
    let draw = fixed_clipped_draw();
    let selected = fixed_function(Behavior::FixedFunctionModelViewV4, false, &draw).unwrap();
    assert_eq!(selected.clip_transform[14], 3.);
    assert_eq!(selected.clipping, draw.clipping);
    assert_eq!(draw.clip_transform, identity_clip());
}

#[test]
fn stencil_masks_take_the_same_product() {
    let mut mask = fixed_clipped_draw();
    mask.fixed_function_lighting = false;
    mask.fixed_function_mask = true;
    let selected = fixed_function(Behavior::FixedFunctionModelViewV4, false, &mask).unwrap();
    assert_eq!(selected.clip_transform[14], 3.);
}

#[test]
fn clip_space_lowering_replaces_only_the_planes() {
    let draw = fixed_clipped_draw();
    let selected = fixed_function(Behavior::FactorizedLinesV3, true, &draw).unwrap();
    assert_eq!(selected.clip_transform, draw.clip_transform);
    let planes = selected.clipping.unwrap();
    assert_eq!(planes.chains()[0][0], [0., 0., 1., 2.]);
}

#[test]
fn a_draw_without_matrices_has_no_clip_space_planes() {
    let mut draw = fixed_clipped_draw();
    draw.fixed_function_factors = None;
    assert!(clip_space_clipping(&draw).is_none());
    assert!(clip_space_clipping(&fixed_clipped_draw()).is_some());
}

#[test]
fn version_four_uses_the_driver_inverse_for_fixed_normals() {
    let mut draw = fixed_clipped_draw();
    let mut lighting = RenderLighting::new(
        vec![[0., 0., 1.]; 3],
        crate::RenderMaterial::default(),
        identity_clip(),
        identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap();
    lighting.prepare_fixed_lighting().unwrap();
    lighting.world = [
        0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
    ];
    draw.lighting = Some(lighting);
    let selected = fixed_function(Behavior::FixedFunctionModelViewV4, false, &draw).unwrap();
    let normal = selected.lighting.unwrap().normal;
    assert_eq!(normal[1], 1.);
    assert_eq!(normal[4], 1.);
}
