use super::expand::selected_draw;
use crate::{
    LineStyle, PrimitiveDraw, RenderThinAxisCoverage, RenderThinLineCoverage, RenderVertex,
    identity_clip,
};
use ash::vk;

fn strip(points: &[[f32; 2]]) -> PrimitiveDraw {
    let vertices = points
        .iter()
        .map(|p| RenderVertex::new([p[0], p[1], 0.], [1.; 4]))
        .collect();
    let indices = (0..points.len() as u32)
        .collect::<Vec<_>>()
        .windows(2)
        .flat_map(|pair| [pair[0], pair[1]])
        .collect();
    let mut draw = PrimitiveDraw::segments(
        vertices,
        indices,
        identity_clip(),
        LineStyle::new(1., 0xffff, 1).unwrap(),
    )
    .unwrap();
    draw.source_line_strip = true;
    draw
}

fn lower(
    draw: &PrimitiveDraw,
    behavior: RenderThinLineCoverage,
) -> Result<Option<PrimitiveDraw>, crate::RenderError> {
    selected_draw(
        behavior,
        RenderThinAxisCoverage::HardwareV1,
        draw,
        vk::Extent2D {
            width: 32,
            height: 32,
        },
        true,
    )
}

fn pixels(draw: &PrimitiveDraw) -> Vec<[i32; 2]> {
    draw.vertices
        .as_chunks::<4>()
        .0
        .iter()
        .map(|quad| [0, 1].map(|axis| ((quad[0].position[axis] + 1.) * 16.).round() as i32))
        .collect()
}

#[test]
fn straight_strip_owns_joint_once_and_excludes_its_final_endpoint() {
    let draw = strip(&[[-0.75, 0.03125], [0.03125, 0.03125], [0.78125, 0.03125]]);
    let before = draw.clone();
    let selected = lower(&draw, RenderThinLineCoverage::DiamondExitFusedProjectionV4)
        .unwrap()
        .unwrap();
    assert_eq!(
        pixels(&selected),
        (3..28).map(|x| [x, 16]).collect::<Vec<_>>()
    );
    assert_eq!(draw, before);
    assert!(!selected.source_line_strip);
    assert!(selected.line_style.is_none());
}

#[test]
fn a_right_angle_joint_is_owned_by_the_outgoing_segment() {
    let draw = strip(&[[-0.75, 0.03125], [0.03125, 0.03125], [0.03125, 0.78125]]);
    let selected = lower(&draw, RenderThinLineCoverage::DiamondExitFusedProjectionV4)
        .unwrap()
        .unwrap();
    let expected = (3..16)
        .map(|x| [x, 16])
        .chain((16..28).map(|y| [16, y]))
        .collect::<Vec<_>>();
    assert_eq!(pixels(&selected), expected);
}

#[test]
fn zero_length_and_retraced_segments_preserve_order_and_multiplicity() {
    let draw = strip(&[
        [-0.75, 0.03125],
        [0.03125, 0.03125],
        [0.03125, 0.03125],
        [-0.75, 0.03125],
    ]);
    let selected = lower(&draw, RenderThinLineCoverage::DiamondExitFusedProjectionV4)
        .unwrap()
        .unwrap();
    let pixels = pixels(&selected);
    assert_eq!(pixels.iter().filter(|p| **p == [10, 16]).count(), 2);
    assert_eq!(pixels.iter().filter(|p| **p == [16, 16]).count(), 1);
}

#[test]
fn older_behaviors_keep_refusing_strips_and_hardware_keeps_them_unchanged() {
    let draw = strip(&[[-0.75, 0.03125], [0.75, 0.03125]]);
    for behavior in [
        RenderThinLineCoverage::DiamondExitV2,
        RenderThinLineCoverage::DiamondExitMajorAxisV3,
    ] {
        assert!(lower(&draw, behavior).is_err());
    }
    assert_eq!(
        lower(&draw, RenderThinLineCoverage::HardwareV1).unwrap(),
        None
    );
}

#[test]
fn stippled_disconnected_and_odd_strip_packets_stay_on_the_device_rasterizer() {
    let original = strip(&[[-0.75, 0.03125], [0., 0.03125], [0.75, 0.03125]]);
    let mut stippled = original.clone();
    stippled.line_style = Some(LineStyle::new(1., 0x00ff, 1).unwrap());
    let mut disconnected = original.clone();
    disconnected.indices = vec![0, 1, 2, 1];
    let mut odd = original;
    odd.indices.push(0);
    for draw in [stippled, disconnected, odd] {
        assert_eq!(
            lower(&draw, RenderThinLineCoverage::DiamondExitFusedProjectionV4).unwrap(),
            None
        );
    }
}

#[test]
fn straight_joints_in_every_octant_have_exact_half_open_coverage() {
    for direction in [
        [1, 0],
        [1, 1],
        [0, 1],
        [-1, 1],
        [-1, 0],
        [-1, -1],
        [0, -1],
        [1, -1],
    ] {
        let points =
            [-12, 0, 12].map(|step| direction.map(|d| (16.5 + (step * d) as f32) / 16. - 1.));
        let selected = lower(
            &strip(&points),
            RenderThinLineCoverage::DiamondExitFusedProjectionV4,
        )
        .unwrap()
        .unwrap();
        let mut expected = (0..24)
            .map(|step| direction.map(|d| 16 + (step - 12) * d))
            .collect::<Vec<_>>();
        let major = usize::from(direction[0] == 0);
        for segment in expected.chunks_mut(12) {
            segment.sort_by_key(|pixel| pixel[major]);
        }
        assert_eq!(pixels(&selected), expected, "direction={direction:?}");
    }
}

#[test]
fn a_closed_strip_keeps_each_square_corner_once() {
    let points = [
        [-0.46875, -0.46875],
        [0.53125, -0.46875],
        [0.53125, 0.53125],
        [-0.46875, 0.53125],
        [-0.46875, -0.46875],
    ];
    let selected = lower(
        &strip(&points),
        RenderThinLineCoverage::DiamondExitFusedProjectionV4,
    )
    .unwrap()
    .unwrap();
    let actual = pixels(&selected);
    assert_eq!(actual.len(), 64);
    for corner in [[8, 8], [24, 8], [24, 24], [8, 24]] {
        assert_eq!(actual.iter().filter(|p| **p == corner).count(), 1);
    }
}
