//! The cap-at-infinity reference accepts faithful roundings and rejects anything beyond its bound.
use super::*;

fn fixture_inputs() -> CapInfinityInputs {
    CapInfinityInputs {
        plane: [0.6, 0., -0.8, -2.],
        light_position: [0., 0., 12.],
        attenuation: (0.5, 0.125),
        light_color: [0.75, 0.5, 0.25],
        scene_ambient: [0.5, 0.25, 0.125],
        emission: [0.3382, 0.3880, 0.3880],
        ambient: [0.4706, 0.5371, 0.4706],
        diffuse: [0.6445, 0.3880, 0.1937],
        texture_scale: 0.0625,
        texels: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.], [1., 1., 1.]],
    }
}

/// A capture holding the rounded exact values; `rounded_capture(&inputs)`.
fn rounded_capture(inputs: &CapInfinityInputs) -> FrameCapture {
    let frame = CapFrame::new(inputs);
    let mut capture = FrameCapture {
        width: PIXELS as u32,
        height: PIXELS as u32,
        rgba: BACKGROUND.repeat(PIXELS * PIXELS),
        depth: vec![1.; PIXELS * PIXELS],
    };
    for index in 0..PIXELS * PIXELS {
        let Some(point) = frame.cap_point(index % PIXELS, index / PIXELS) else {
            continue;
        };
        let color = frame.color(point);
        let rgba = [
            color[0].round() as u8,
            color[1].round() as u8,
            color[2].round() as u8,
            255,
        ];
        capture.rgba[index * 4..index * 4 + 4].copy_from_slice(&rgba);
        capture.depth[index] = ((EYE_Z - point[2] - Z_RANGE.0) / (Z_RANGE.1 - Z_RANGE.0)) as f32;
    }
    capture
}

fn first_cap_pixel(capture: &FrameCapture) -> usize {
    (0..PIXELS * PIXELS)
        .find(|i| capture.rgba[i * 4..i * 4 + 4] != BACKGROUND)
        .unwrap()
}

#[test]
fn exact_roundings_pass_and_cover_a_lit_centre() {
    let inputs = fixture_inputs();
    let capture = rounded_capture(&inputs);
    assert_eq!(cap_infinity_violation(&capture, &inputs), None);
    let frame = CapFrame::new(&inputs);
    assert!(
        frame.centre_color[0] > frame.infinite[0],
        "{:?}",
        frame.centre_color
    );
}

#[test]
fn a_two_level_channel_error_is_rejected() {
    let inputs = fixture_inputs();
    let mut capture = rounded_capture(&inputs);
    let index = first_cap_pixel(&capture);
    capture.rgba[index * 4] = capture.rgba[index * 4].wrapping_add(2);
    assert!(cap_infinity_violation(&capture, &inputs).is_some());
}

#[test]
fn coverage_alpha_and_depth_errors_are_rejected() {
    let inputs = fixture_inputs();
    let index = first_cap_pixel(&rounded_capture(&inputs));
    let mut uncovered = rounded_capture(&inputs);
    uncovered.rgba[index * 4..index * 4 + 4].copy_from_slice(&BACKGROUND);
    let mut alpha = rounded_capture(&inputs);
    alpha.rgba[index * 4 + 3] = 254;
    let mut depth = rounded_capture(&inputs);
    depth.depth[index] += 4e-6;
    for capture in [uncovered, alpha, depth] {
        assert!(cap_infinity_violation(&capture, &inputs).is_some());
    }
}

#[test]
fn defined_colour_falls_off_from_the_lit_centre() {
    // The point light reaches only the finite centre vertex, so its term fades towards the border.
    let frame = CapFrame::new(&fixture_inputs());
    // Both samples lie in the red texel (u > 0, v > 0) on the plane z = 0.75x - 2.5.
    let [x, y, z] = frame.centre;
    let near = frame.color([x - 0.04, y + 0.04, z - 0.03])[0];
    let far = frame.color([-4.5, 5.5, (0.6 * -4.5 - 2.) / 0.8])[0];
    assert!(near > far + 20., "near {near} far {far}");
}
