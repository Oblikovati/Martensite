use super::*;

pub(in crate::backend) fn phong_triangle(points: [[f32; 3]; 3]) -> PrimitiveDraw {
    let vertices = points.map(|point| crate::RenderVertex::new(point, [1.0; 4]));
    let draw =
        PrimitiveDraw::new(vertices.to_vec(), vec![0, 1, 2], crate::identity_clip()).unwrap();
    let lighting = crate::RenderLighting::new(
        vec![[0.0, 0.0, 1.0]; 3],
        crate::RenderMaterial::default(),
        crate::identity_clip(),
        crate::identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap();
    draw.with_lighting(lighting).unwrap()
}

fn selected_behavior() -> Behavior {
    Behavior::FixedGrid12V2 {
        origin: crate::RenderTriangleGridOrigin::TopDown,
    }
}

fn native_grid_behavior() -> Behavior {
    Behavior::NativeGrid12V4 {
        origin: crate::RenderTriangleGridOrigin::TopDown,
    }
}

#[test]
fn v4_uses_q12_admission_when_the_device_grid_is_eight_bits() {
    let draw = q12_stable_eight_bit_collapsed_triangle();
    assert_v2_refuses_eight_bit_collapse(&draw);
    assert_v4_admits_twelve_bit_stability(&draw);
}

#[path = "triangle_grid_coverage_upload_witness_tests.rs"]
mod witness_tests;

pub(in crate::backend) fn bug29791_triangle_nine() -> PrimitiveDraw {
    let mut draw = phong_triangle([
        [7.445_753, 0.900_423_3, 51.276_653],
        [7.445_816_5, 0.899_897_46, 51.277_412],
        [7.386_058_3, 1.302_361_4, 50.0],
    ]);
    draw.clip_transform = [
        0.028288545,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.066666305,
        0.0,
        0.0,
        0.028288545,
        0.0,
        0.0,
        0.0,
        -0.77793497,
        0.0,
        1.0,
    ];
    draw
}

fn assert_v2_refuses_eight_bit_collapse(draw: &PrimitiveDraw) {
    let error = admit(
        selected_behavior(),
        interpolation(),
        draw,
        viewport(),
        samples(),
        8,
    )
    .unwrap_err();
    assert!(matches!(
        error.triangle_coverage_failure(),
        Some(Failure::RasterGridDegenerate { .. })
    ));
}

fn assert_v4_admits_twelve_bit_stability(draw: &PrimitiveDraw) {
    assert!(
        admit(
            native_grid_behavior(),
            interpolation(),
            draw,
            viewport(),
            samples(),
            8
        )
        .is_ok()
    );
}

fn interpolation() -> RenderPhongInterpolationBehavior {
    RenderPhongInterpolationBehavior::UnsnappedV2
}

fn viewport() -> [f32; 4] {
    [0.0, 0.0, 4.0, 4.0]
}

fn samples() -> vk::SampleCountFlags {
    vk::SampleCountFlags::TYPE_1
}

fn q12_stable_eight_bit_collapsed_triangle() -> PrimitiveDraw {
    let clip = |coordinate: i32| coordinate as f32 / 8192.0;
    phong_triangle([
        [clip(-7), clip(-7), 0.0],
        [clip(7), clip(-7), 0.0],
        [clip(-7), clip(7), 0.0],
    ])
}

#[test]
fn v4_admits_small_triangle_with_a_proven_q12_area_interval() {
    let draw = q12_small_triangle();
    let intervals = super::super::triangle_grid_projection_intervals::projected_q12_intervals(
        &draw,
        &[0, 1, 2],
        viewport(),
    )
    .unwrap();
    assert!(crate::triangle_grid_coverage::interval_area_excludes_zero(
        intervals
    ));
    assert!(
        admit(
            native_grid_behavior(),
            interpolation(),
            &draw,
            viewport(),
            samples(),
            8,
        )
        .is_ok()
    );
}

#[test]
fn v4_keeps_nominal_rounding_when_q12_area_interval_crosses_zero() {
    let draw = q12_uncertain_triangle();
    let clips = projected_clips(&draw);
    let grid = grid_triangle(clips.clone().try_into().unwrap(), viewport()).unwrap();
    assert_ne!(grid.signed_area(), 0);
    let intervals = super::super::triangle_grid_projection_intervals::projected_q12_intervals(
        &draw,
        &[0, 1, 2],
        viewport(),
    )
    .unwrap();
    assert!(!crate::triangle_grid_coverage::interval_area_excludes_zero(
        intervals
    ));
    assert!(
        admit(
            native_grid_behavior(),
            interpolation(),
            &draw,
            viewport(),
            samples(),
            8,
        )
        .is_ok()
    );
}

fn q12_uncertain_triangle() -> PrimitiveDraw {
    let clip = |coordinate: i32| coordinate as f32 / 16384.0;
    phong_triangle([
        [clip(1), clip(1), 0.0],
        [clip(3), clip(1), 0.0],
        [clip(1), clip(3), 0.0],
    ])
}

fn q12_small_triangle() -> PrimitiveDraw {
    let clip = |coordinate: i32| coordinate as f32 / 8192.0;
    phong_triangle([
        [clip(-1), clip(-1), 0.0],
        [clip(1), clip(-1), 0.0],
        [clip(-1), clip(1), 0.0],
    ])
}

#[test]
fn viewport_bound_keeps_fragment_centers_inside_signed_edge_range() {
    assert!(viewport_in_int64_domain([0.0, 410.0, 409.0, -409.0]));
    assert!(!viewport_in_int64_domain([0.0, 0.0, 262_143.0, 1.0]));
    assert!(!viewport_in_int64_domain([f32::NAN, 0.0, 1.0, 1.0]));
}

#[test]
fn only_programmable_phong_triangles_select_the_coverage_carrier() {
    let mut draw = phong_triangle([[-0.75, -0.75, 0.0], [0.75, -0.75, 0.0], [-0.75, 0.75, 0.0]]);
    assert!(selected_phong(&draw));
    draw.lighting.as_mut().unwrap().shading = RenderShading::Gouraud;
    assert!(!selected_phong(&draw));
}

#[test]
fn v4_pipeline_selection_is_limited_to_programmable_phong_draws() {
    let behavior = native_grid_behavior();
    let mut phong = phong_triangle([[-0.75, -0.75, 0.0], [0.75, -0.75, 0.0], [-0.75, 0.75, 0.0]]);
    assert!(v4_selected(behavior, &phong));
    phong.lighting.as_mut().unwrap().shading = RenderShading::Gouraud;
    assert!(!v4_selected(behavior, &phong));
    assert!(!v4_selected(behavior, &unlit_triangle()));
}

fn v4_selected(behavior: Behavior, draw: &PrimitiveDraw) -> bool {
    super::super::fixed_grid_record::logical_facing_active(behavior, selected_phong(draw))
}

fn unlit_triangle() -> PrimitiveDraw {
    let vertices = [
        crate::RenderVertex::new([-0.75, -0.75, 0.0], [1.0; 4]),
        crate::RenderVertex::new([0.75, -0.75, 0.0], [1.0; 4]),
        crate::RenderVertex::new([-0.75, 0.75, 0.0], [1.0; 4]),
    ];
    PrimitiveDraw::new(vertices.to_vec(), vec![0, 1, 2], crate::identity_clip()).unwrap()
}

#[test]
fn compatibility_surface_state_stays_on_the_device_rasterizer() {
    let mut draw = phong_triangle([[-0.75, -0.75, 0.0], [0.75, -0.75, 0.0], [-0.75, 0.75, 0.0]]);
    draw.compatibility_color_target = true;
    assert!(!selected_phong(&draw));
    assert!(
        admit(
            selected_behavior(),
            RenderPhongInterpolationBehavior::UnsnappedV2,
            &draw,
            [0.0, 0.0, 4.0, 4.0],
            vk::SampleCountFlags::TYPE_1,
            8,
        )
        .is_ok()
    );
}

#[test]
fn one_complete_four_by_four_phong_triangle_keeps_exact_half_open_coverage() {
    let draw = phong_triangle([[-0.75, -0.75, 0.0], [0.75, -0.75, 0.0], [-0.75, 0.75, 0.0]]);
    let original = draw.clone();
    assert!(
        admit(
            selected_behavior(),
            RenderPhongInterpolationBehavior::UnsnappedV2,
            &draw,
            [0.0, 0.0, 4.0, 4.0],
            vk::SampleCountFlags::TYPE_1,
            8,
        )
        .is_ok()
    );
    assert_coverage(draw, original);
}

fn assert_coverage(draw: PrimitiveDraw, original: PrimitiveDraw) {
    let projected = projected_clips(&draw);
    let triangle = grid_triangle(
        [projected[0], projected[1], projected[2]],
        [0.0, 0.0, 4.0, 4.0],
    )
    .unwrap();
    for y in 0..4 {
        for x in 0..4 {
            assert_eq!(
                triangle.covers([x, y], crate::RenderTriangleGridOrigin::TopDown),
                x + y < 3,
                "sample ({x},{y})",
            );
        }
    }
    assert_eq!(draw, original);
}

#[test]
fn selected_coverage_refuses_msaa_and_partially_clipped_triangles() {
    assert_msaa_refused();
    assert_user_clipping_refused();
    assert_partial_clip_refused();
    assert_uncertain_clip_boundary_refused();
}

fn assert_msaa_refused() {
    let draw = phong_triangle([[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [-1.0, 1.0, 0.0]]);
    let msaa = admit(
        selected_behavior(),
        RenderPhongInterpolationBehavior::UnsnappedV2,
        &draw,
        [0.0, 0.0, 4.0, 4.0],
        vk::SampleCountFlags::TYPE_4,
        8,
    )
    .unwrap_err();
    assert!(matches!(
        msaa.triangle_coverage_failure(),
        Some(Failure::UnsupportedInterface { .. })
    ));
}

fn assert_partial_clip_refused() {
    let clipped = phong_triangle([[-1.0, -1.0, 0.0], [2.0, -1.0, 0.0], [-1.0, 1.0, 0.0]]);
    let error = admit(
        selected_behavior(),
        RenderPhongInterpolationBehavior::UnsnappedV2,
        &clipped,
        [0.0, 0.0, 4.0, 4.0],
        vk::SampleCountFlags::TYPE_1,
        8,
    )
    .unwrap_err();
    assert!(matches!(
        error.triangle_coverage_failure(),
        Some(Failure::InvalidTriangle { .. })
    ));
}

fn assert_user_clipping_refused() {
    let clipping =
        crate::RenderClipping::new(vec![vec![[1.0, 0.0, 0.0, -2.0]]], crate::identity_clip())
            .unwrap();
    let draw = phong_triangle([[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [-1.0, 1.0, 0.0]])
        .with_clipping(clipping);
    let error = admit(
        selected_behavior(),
        RenderPhongInterpolationBehavior::UnsnappedV2,
        &draw,
        [0.0, 0.0, 4.0, 4.0],
        vk::SampleCountFlags::TYPE_1,
        8,
    )
    .unwrap_err();
    assert!(matches!(
        error.triangle_coverage_failure(),
        Some(Failure::UnsupportedInterface { .. })
    ));
}

fn assert_uncertain_clip_boundary_refused() {
    let mut boundary = phong_triangle([[-1.0, -0.75, 0.0], [0.0, -0.75, 0.0], [-0.75, 0.75, 0.0]]);
    boundary.clip_transform[4] = martensite_cmath::powi(f64::from(2.0_f32), -25) as f32;
    let error = admit(
        selected_behavior(),
        RenderPhongInterpolationBehavior::UnsnappedV2,
        &boundary,
        [0.0, 0.0, 4.0, 4.0],
        vk::SampleCountFlags::TYPE_1,
        8,
    )
    .unwrap_err();
    assert!(matches!(
        error.triangle_coverage_failure(),
        Some(Failure::InvalidTriangle { .. })
    ));
}

#[test]
fn captured_bug25044_triangle_two_has_proven_q12_winding() {
    let mut draw = bug25044_triangle_two();
    draw.clip_transform = bug25044_clip_matrix();
    assert!(
        admit(
            native_grid_behavior(),
            interpolation(),
            &draw,
            [0.0, 410.0, 409.0, -409.0],
            samples(),
            8
        )
        .is_ok()
    );
}

fn bug25044_triangle_two() -> PrimitiveDraw {
    phong_triangle([
        [-132.706_04, 152.23132, 7.215_387],
        [-132.708_51, 152.146_24, 7.209_042],
        [-132.706_42, 152.18864, 7.212_439],
    ])
}

fn bug25044_clip_matrix() -> [f32; 16] {
    [
        13.712_413,
        -7.916_866,
        -12.925_124,
        0.0,
        13.712_413,
        7.916_866,
        12.925_124,
        0.0,
        0.0,
        15.833_732,
        -12.925_124,
        0.0,
        -267.528_3,
        -2_369.930_7,
        -3_589.445,
        1.0,
    ]
}
