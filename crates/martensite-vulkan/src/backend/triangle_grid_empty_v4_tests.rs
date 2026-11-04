use super::super::*;

fn actual_bug29791_triangle_zero() -> PrimitiveDraw {
    let positions = [
        [1089470464, 2802081024, 1112014848],
        [1089356912, 1063668542, 1112349891],
        [1089356958, 1063665490, 1112350067],
    ];
    let mut vertices = vec![vertex([0, 0, 0]); 82];
    for (index, position) in [41, 80, 81].into_iter().zip(positions) {
        vertices[index] = vertex(position);
    }
    let transform = [
        1021820257, 0, 0, 0, 0, 0, 1032357976, 0, 0, 1021820257, 0, 0, 0, 3209111231, 0, 1065353216,
    ]
    .map(f32::from_bits);
    let draw = PrimitiveDraw::new(vertices, vec![41, 80, 81], transform).unwrap();
    let lighting = crate::RenderLighting::new(
        vec![[0.0, 0.0, 1.0]; 82],
        crate::RenderMaterial::default(),
        crate::identity_clip(),
        crate::identity_clip(),
        false,
        crate::RenderShading::Phong,
    )
    .unwrap();
    let clipped = draw.with_lighting(lighting).unwrap();
    let clipping =
        crate::RenderClipping::new(vec![vec![[1.0, 0.0, 0.0, -20.0]]], crate::identity_clip())
            .unwrap();
    clipped.with_clipping(clipping)
}

fn vertex(position: [u32; 3]) -> crate::RenderVertex {
    crate::RenderVertex::new(position.map(f32::from_bits), [1.0; 4])
}

fn v4_behavior() -> crate::RenderTriangleCoverageBehavior {
    crate::RenderTriangleCoverageBehavior::NativeGrid12V4 {
        origin: crate::RenderTriangleGridOrigin::TopDown,
    }
}

fn prepare_v4(draw: &PrimitiveDraw) -> Result<Option<PrimitiveDraw>, crate::RenderError> {
    prepare(
        v4_behavior(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
        draw,
        [0.0, 410.0, 409.0, -409.0],
        ash::vk::SampleCountFlags::TYPE_1,
        8,
        true,
        false,
    )
}

#[test]
fn bug29791_triangle_zero_uses_interval_rectangle_empty_carrier() {
    let original = actual_bug29791_triangle_zero();
    let intervals =
        super::super::super::triangle_grid_projection_intervals::projected_q12_intervals(
            &original,
            &[41, 80, 81],
            [0.0, 410.0, 409.0, -409.0],
        )
        .unwrap();
    assert_eq!(
        intervals.map(crate::triangle_grid_coverage::GridVertexInterval::coordinates),
        [
            [[1015347, 1015348], [308581, 308583]],
            [[1014064, 1014065], [278296, 278298]],
            [[1014064, 1014065], [278280, 278282]],
        ]
    );
    assert!(!crate::triangle_grid_coverage::interval_area_excludes_zero(
        intervals
    ));
    assert!(super::sample::q12_interval_box_empty(intervals));

    let selected = prepare_v4(&original).unwrap().unwrap();
    assert_eq!(selected.indices, [41, 41, 41]);
    assert_eq!(selected.vertices, original.vertices);
    assert_eq!(selected.clip_transform, original.clip_transform);
    assert_eq!(selected.clipping, original.clipping);
    assert_eq!(original.indices, [41, 80, 81]);
}

#[test]
fn v4_uses_the_nominal_q12_area_when_interval_box_reaches_pixel_center() {
    let center = 0.25_f32;
    let delta = 1.0_f32 / 16384.0;
    let triangle = [
        [center - delta, center - delta, 0.0],
        [center + delta, center - delta, 0.0],
        [center - delta, center + delta, 0.0],
    ];
    let draw = crate::backend::triangle_grid_coverage_upload::tests::phong_triangle(triangle);
    let intervals =
        super::super::super::triangle_grid_projection_intervals::projected_q12_intervals(
            &draw,
            &[0, 1, 2],
            [0.0, 0.0, 4.0, 4.0],
        )
        .unwrap();
    assert!(!super::sample::q12_interval_box_empty(intervals));
    let selected = prepare(
        v4_behavior(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
        &draw,
        [0.0, 0.0, 4.0, 4.0],
        ash::vk::SampleCountFlags::TYPE_1,
        8,
        false,
        true,
    );
    assert!(
        selected.unwrap().is_some(),
        "a triangle whose nominal Q12 area is zero covers no sample on the 12-bit grid"
    );
}

#[test]
fn v4_collapses_a_repeated_vertex_beyond_the_clip_volume_to_a_carrier() {
    let mut draw = crate::backend::triangle_grid_coverage_upload::tests::phong_triangle([
        [-0.5, 0.0, 2.0],
        [-0.5, 0.0, 2.0],
        [-0.5, 0.0, 2.0],
    ]);
    draw.clipping = None;
    let selected = prepare(
        v4_behavior(),
        crate::RenderPhongInterpolationBehavior::UnsnappedV3,
        &draw,
        [0.0, 0.0, 4.0, 4.0],
        ash::vk::SampleCountFlags::TYPE_1,
        8,
        false,
        true,
    )
    .unwrap();
    assert!(
        selected.is_some(),
        "a sample-empty triangle becomes a zero-area carrier"
    );
}
