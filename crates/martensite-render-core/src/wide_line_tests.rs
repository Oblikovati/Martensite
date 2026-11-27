use super::*;
use crate::{LineStyle, RenderVertex};

const IDENTITY: [f32; 16] = [
    1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
];

fn segments(points: &[[f32; 3]], transform: [f32; 16]) -> PrimitiveDraw {
    let vertices = points
        .iter()
        .map(|&p| RenderVertex::new(p, [1.; 4]))
        .collect();
    let indices = (0..points.len() as u32).collect();
    let style = LineStyle::new(9., 0xffff, 1).unwrap();
    PrimitiveDraw::segments(vertices, indices, transform, style).unwrap()
}

#[test]
fn segments_split_by_window_major_axis_with_x_major_first() {
    // Steep (y-major), shallow (x-major) and 45 degree (x-major: |dx| >= |dy|) segments.
    let draw = segments(
        &[
            [0., 0., 0.],
            [0.1, 0.5, 0.],
            [0., 0., 0.],
            [0.5, 0.1, 0.],
            [0., 0., 0.],
            [0.5, 0.5, 0.],
        ],
        IDENTITY,
    );
    let replay = WideLineReplay::new(&draw, [64, 64], 9.);
    assert_eq!(replay.indices, [2, 3, 4, 5, 0, 1]);
    assert_eq!(replay.x_major, 4);
}

#[test]
fn framebuffer_aspect_decides_the_major_axis() {
    // Equal NDC deltas are x-major on a wide framebuffer and y-major on a tall one.
    let draw = segments(&[[0., 0., 0.], [0.4, 0.5, 0.]], IDENTITY);
    assert_eq!(WideLineReplay::new(&draw, [128, 64], 9.).x_major, 2);
    assert_eq!(WideLineReplay::new(&draw, [64, 128], 9.).x_major, 0);
}

#[test]
fn replicas_follow_the_gl_column_offsets() {
    let draw = segments(&[[0., 0., 0.], [1., 0., 0.]], IDENTITY);
    let offsets = |width| -> Vec<f32> {
        WideLineReplay::new(&draw, [64, 64], width)
            .offsets()
            .collect()
    };
    assert_eq!(offsets(9.), [-4., -3., -2., -1., 0., 1., 2., 3., 4.]);
    // GL shifts an even width by -(w - 1) / 2, a half pixel, before widening.
    assert_eq!(offsets(12.).first(), Some(&-5.5));
    assert_eq!(offsets(12.).last(), Some(&5.5));
    assert_eq!(offsets(9.4).len(), 9);
    assert_eq!(offsets(0.2), [0.]);
}

#[test]
fn segment_ending_on_the_eye_plane_uses_its_visible_slope() {
    // Perspective: w = -z, z_clip = -z - 2, so the near plane z + w = 0 lies at z = -1.
    let mut transform = IDENTITY;
    transform[10] = -1.;
    transform[14] = -2.;
    transform[11] = -1.;
    transform[15] = 0.;
    // The far end has w = 0: dividing it gives infinite deltas that would read as x-major.
    let draw = segments(&[[0.1, 0.1, -2.], [0.3, 3., 0.]], transform);
    assert_eq!(WideLineReplay::new(&draw, [64, 64], 9.).x_major, 0);
}
