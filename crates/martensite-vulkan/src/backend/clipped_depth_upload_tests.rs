use super::*;
fn triangle() -> PrimitiveDraw {
    PrimitiveDraw::new(
        vec![crate::RenderVertex::new([0.; 3], [1.; 4]); 3],
        vec![0, 1, 2],
        crate::identity_clip(),
    )
    .unwrap()
}
#[test]
fn selection_is_only_the_explicit_fixed_clipped_depth_writing_route() {
    let mut draw = triangle();
    assert!(!applies(&draw));
    draw.fixed_function_lighting = true;
    draw.clipping = Some(
        crate::RenderClipping::new(vec![vec![[0., 1., 0., 0.]]], crate::identity_clip()).unwrap(),
    );
    assert!(
        !applies(&draw),
        "live fixed capability alone is not a fixed lighting payload"
    );
    draw.lighting = Some(super::super::fixed_vertex_sources::fixed_test_lighting(
        3,
        crate::RenderShading::Phong,
    ));
    assert!(applies(&draw));
    draw.capping_pass = crate::RenderCappingPass::MaskStart;
    assert!(!applies(&draw));
}
#[test]
fn canonical_bounds_refuse_unproved_generated_frustum_vertices_and_bad_w() {
    assert!(require_canonical_bounds(0, [[0., 0.]; 4]).is_err());
    assert!(require_canonical_bounds(0, [[-0.5, 0.5], [-0.5, 0.5], [-0.5, 0.5], [1., 1.]]).is_ok());
    for range in [[-2., 0.], [0., 2.], [f64::NAN, 0.]] {
        assert!(require_canonical_bounds(3, [range, [0., 0.], [0., 0.], [1., 1.]]).is_err());
    }
}
#[test]
fn pipeline_refuses_alternate_geometry_and_sample_compositions() {
    let mut kind = super::super::pipeline_variants::PipelineVariant::from_index(0, false);
    assert!(require_pipeline(kind, false, vk::SampleCountFlags::TYPE_8).is_err());
    kind.lighting = true;
    kind.fixed_lighting = true;
    kind.clipped = true;
    kind.world_clip_v2 = true;
    assert!(require_pipeline(kind, false, vk::SampleCountFlags::TYPE_8).is_ok());
    assert!(require_pipeline(kind, true, vk::SampleCountFlags::TYPE_8).is_err());
    assert!(require_pipeline(kind, false, vk::SampleCountFlags::TYPE_4).is_err());
    kind.triangle_grid_coverage = true;
    assert!(require_pipeline(kind, false, vk::SampleCountFlags::TYPE_8).is_err());
}
