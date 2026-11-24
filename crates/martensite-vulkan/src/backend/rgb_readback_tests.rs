//! Packed RGB rows equal the RGBA readback without alpha, in both row orders (#594).
use crate::{PrimitiveDraw, RenderFrame, RenderVertex, identity_clip};

/// A 23 x 17 frame whose rows all differ: odd width pads the packed rows and odd height adds
/// the leading attachment row of the top-left convention.
fn gradient_frame() -> RenderFrame {
    let vertices = vec![
        RenderVertex::new([-1., -1., 0.], [1., 0., 0., 1.]),
        RenderVertex::new([1., -1., 0.], [0., 1., 0., 1.]),
        RenderVertex::new([0., 1., 0.], [0., 0., 1., 1.]),
    ];
    let draw = PrimitiveDraw::new(vertices, vec![0, 1, 2], identity_clip()).unwrap();
    RenderFrame::new(23, 17, [0.2, 0.4, 0.6, 1.], vec![draw]).unwrap()
}

fn expected_rows(rgba: &[u8], width: usize, flip: bool) -> Vec<Vec<u8>> {
    let mut rows: Vec<Vec<u8>> = rgba
        .chunks_exact(width * 4)
        .map(|row| {
            row.as_chunks::<4>()
                .0
                .iter()
                .flat_map(|pixel| pixel[..3].to_vec())
                .collect()
        })
        .collect();
    if flip {
        rows.reverse();
    }
    rows
}

#[test]
fn packed_rgb_rows_match_the_rgba_readback_in_both_orders() {
    let mut renderer = super::super::resource_lifecycle_test_support::create_resource_renderer();
    let frame = gradient_frame();
    let readback = renderer.render_deferred(&frame).unwrap();
    let full = renderer.read_frame(&readback).unwrap();
    assert!(
        full.rgba
            .as_chunks::<{ 23 * 4 }>()
            .0
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            > 8
    );
    for flip in [false, true] {
        let mut rows = Vec::new();
        let written = renderer
            .read_frame_rgb(&readback, flip, &mut |y, row| {
                assert_eq!(y, rows.len());
                rows.push(row.to_vec());
                true
            })
            .unwrap();
        assert!(written);
        assert_eq!(rows, expected_rows(&full.rgba, 23, flip), "flip {flip}");
    }
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn a_refused_row_stops_the_copy_and_reports_it() {
    let mut renderer = super::super::resource_lifecycle_test_support::create_resource_renderer();
    let readback = renderer.render_deferred(&gradient_frame()).unwrap();
    let mut calls = 0;
    let written = renderer
        .read_frame_rgb(&readback, false, &mut |_, _| {
            calls += 1;
            calls < 3
        })
        .unwrap();
    assert!(!written);
    assert_eq!(calls, 3);
}
