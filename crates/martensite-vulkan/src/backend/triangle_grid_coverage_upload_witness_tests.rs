use super::*;

#[test]
fn witnessed_bug29791_triangle_nine_passes_with_full_q12_intervals() {
    let draw = super::bug29791_triangle_nine();
    let viewport = [0.0, 410.0, 409.0, -409.0];
    let clips = projected_clips(&draw);
    assert_eq!(
        clips
            .iter()
            .map(|clip| clip.map(f32::to_bits))
            .collect::<Vec<_>>(),
        [
            [1045933892, 1059860473, 1031135183, 1065353216],
            [1045934012, 1059860833, 1031125773, 1065353216],
            [1045820567, 1059254569, 1035063445, 1065353216],
        ]
    );
    assert_logical_interval_and_coverage(&draw, clips, viewport);
}

fn assert_logical_interval_and_coverage(
    draw: &PrimitiveDraw,
    clips: Vec<[f32; 4]>,
    viewport: [f32; 4],
) {
    let intervals = crate::backend::triangle_grid_projection_intervals::projected_q12_intervals(
        draw,
        &[0, 1, 2],
        viewport,
    )
    .unwrap();
    assert_eq!(
        intervals.map(crate::triangle_grid_coverage::GridVertexInterval::coordinates),
        [
            [[1014062, 1014062], [278330, 278332]],
            [[1014063, 1014064], [278312, 278314]],
            [[1012647, 1012648], [308581, 308583]],
        ]
    );
    assert!(crate::triangle_grid_coverage::interval_area_excludes_zero(
        intervals
    ));
    let logical_grid = grid_triangle(clips.try_into().unwrap(), viewport).unwrap();
    assert_eq!(logical_grid.signed_area(), 35050);
    assert_no_center_samples(logical_grid);
    assert!(
        admit(
            native_grid_behavior(),
            interpolation(),
            draw,
            viewport,
            samples(),
            8
        )
        .is_ok()
    );
}

fn assert_no_center_samples(grid: crate::triangle_grid_coverage::GridTriangle) {
    for y in 66..=76 {
        for x in 246..=249 {
            assert!(!grid.covers([x, y], crate::RenderTriangleGridOrigin::TopDown));
        }
    }
}
