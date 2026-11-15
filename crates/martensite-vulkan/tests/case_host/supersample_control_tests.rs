use super::*;
use martensite_vulkan::{LineStyle, RenderVertex, identity_clip};

fn vertex(x: f32) -> RenderVertex {
    RenderVertex {
        position: [x, 0., 0.],
        weight: 1.,
        color: [1.; 4],
    }
}

#[test]
fn centre_samples_take_the_middle_large_pixel() {
    // A 2x1 image at factor 3: large pixels (1,1) and (4,1) are the centres.
    let (bpp, stride) = (1, 6);
    let large: Vec<u8> = (0..18).collect();
    assert_eq!(sample_centres(&large, stride, bpp, [2, 1], 3), [7, 10]);
    let rgb: Vec<u8> = (0..54).collect();
    assert_eq!(
        sample_centres(&rgb, 18, 3, [2, 1], 3),
        [21, 22, 23, 30, 31, 32]
    );
}

#[test]
fn record_names_dependencies_and_frame_changes() {
    let solid = LineStyle::new(1., u16::MAX, 1).unwrap();
    let line = PrimitiveDraw::segments(
        vec![vertex(0.), vertex(1.)],
        vec![0, 1],
        identity_clip(),
        solid,
    )
    .unwrap();
    let json = control_record_json(
        std::slice::from_ref(&line),
        std::slice::from_ref(&line),
        false,
        [4, 3],
    );
    assert!(
        json.contains("\"pixel_size_dependencies\": [\"lines\"]"),
        "{json}"
    );
    assert!(
        json.contains("\"draws_equal\": true") && json.contains("\"width\": 4"),
        "{json}"
    );
    let json = control_record_json(&[line], &[], true, [4, 3]);
    assert!(json.contains("\"draws_equal\": false") && json.contains("\"shader_grid\": true"));
}
