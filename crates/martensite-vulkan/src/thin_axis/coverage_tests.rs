//! Closed-form cell sets are independent of the upload triangulation's diagonal.
use super::*;
use crate::{RenderLineGrid::*, RenderLineOrigin::*, RenderVertex};
use std::collections::BTreeSet;
fn frame_loop() -> PrimitiveDraw {
    // Derived near-integer input coordinates exercise the grid boundary, not a GPU-input claim.
    let corners = [
        [4.0009875, 10.000988],
        [26.000988, 10.000988],
        [26.000988, 356.001],
        [4.0009875, 356.001],
    ];
    let vertices =
        corners.map(|p| RenderVertex::new([p[0] / 200. - 1., p[1] / 200. - 1., 0.], [1.; 4]));
    PrimitiveDraw::segments(
        vertices.into(),
        vec![0, 1, 1, 2, 2, 3, 3, 0],
        crate::identity_clip(),
        crate::LineStyle::new(1., 0xffff, 1).unwrap(),
    )
    .unwrap()
}
fn cells(draw: &PrimitiveDraw) -> BTreeSet<(i32, i32)> {
    let mut result = BTreeSet::new();
    for quad in draw.vertices.as_chunks::<4>().0.iter() {
        let p: Vec<_> = quad
            .iter()
            .map(|v| [0, 1].map(|i| ((v.position[i] / v.weight + 1.) * 200.).round() as i32))
            .collect();
        let min = [0, 1].map(|i| p.iter().map(|v| v[i]).min().unwrap());
        let max = [0, 1].map(|i| p.iter().map(|v| v[i]).max().unwrap());
        for y in min[1]..max[1] {
            for x in min[0]..max[0] {
                result.insert((x, 399 - y));
            }
        }
    }
    result
}
fn reference_loop(grid: crate::RenderLineGrid) -> BTreeSet<(i32, i32)> {
    let (left, right, top, bottom) = match grid {
        Fractional8 => (3, 25, 44, 390),
        Fractional12 => (4, 26, 43, 389),
    };
    let mut cells = BTreeSet::new();
    for y in 44..390 {
        cells.extend([(left, y), (right, y)]);
    }
    for x in 4..26 {
        cells.extend([(x, top), (x, bottom)]);
    }
    cells
}
#[test]
fn closed_loop_has_exact_half_open_corners_without_blanket_translation() {
    for grid in [Fractional8, Fractional12] {
        let output = selected_draw(
            RenderThinAxisCoverage::FixedGridV2 {
                grid,
                origin: BottomUp,
            },
            &frame_loop(),
            [400, 400],
            true,
        )
        .unwrap()
        .unwrap();
        let expected = reference_loop(grid);
        assert_eq!(expected.len(), 735);
        assert_eq!(cells(&output), expected);
    }
}
#[test]
fn scissor_intersection_keeps_original_cell_ownership() {
    let source = frame_loop();
    let mode = RenderThinAxisCoverage::FixedGridV2 {
        grid: Fractional12,
        origin: BottomUp,
    };
    let output = selected_draw(mode, &source, [400, 400], true)
        .unwrap()
        .unwrap();
    let cropped = |set: BTreeSet<(i32, i32)>| {
        set.into_iter()
            .filter(|&(x, y)| (4..20).contains(&x) && (100..200).contains(&y))
            .collect::<BTreeSet<_>>()
    };
    assert_eq!(
        cropped(cells(&output)),
        (100..200).map(|y| (4, y)).collect()
    );
    assert_eq!(source, frame_loop());
}
#[test]
fn independent_projective_interpolation_matches_all_quad_corners() {
    let mut source = frame_loop();
    source.indices = vec![0, 3];
    source.vertices[0] = RenderVertex::homogeneous([-1.5, -1.6, -0.8, 2.], [0.; 4]);
    source.vertices[3] = RenderVertex::homogeneous([-3., 2.4, 1.6, 4.], [1.; 4]);
    let mode = RenderThinAxisCoverage::FixedGridV2 {
        grid: Fractional12,
        origin: BottomUp,
    };
    let output = selected_draw(mode, &source, [400, 400], true)
        .unwrap()
        .unwrap();
    for v in &output.vertices {
        let y = (v.position[1] as f64 / v.weight as f64 + 1.) * 200.;
        let screen_t = (y - 40.) / 280.;
        let parameter = 2. * screen_t / (4. - 2. * screen_t);
        assert!((f64::from(v.color[0]) - parameter).abs() < 1e-6);
        assert!((f64::from(v.position[2]) - (-0.8 + 2.4 * parameter)).abs() < 1e-6);
    }
}
