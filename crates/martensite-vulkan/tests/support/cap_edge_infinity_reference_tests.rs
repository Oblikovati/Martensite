//! The cap edge reference accepts faithful roundings of its analytic image and rejects the rest.
use super::*;

const BACKGROUND: [u8; 4] = [0, 0, 255, 255];
/// Two texels alternate by column; with an alpha table red is 64/255 and green 128/255.
const TEXELS: [[u8; 4]; 2] = [[255, 0, 0, 255], [0, 255, 0, 255]];
const BEHIND: [u8; 4] = [128, 128, 128, 255];
const MASK_TABLE: [([u8; 3], f64); 2] = [([255, 0, 0], 64. / 255.), ([0, 255, 0], 128. / 255.)];

fn inputs(skip_first: bool) -> CapEdgeInputs {
    CapEdgeInputs {
        plane: [0.6, 0., -0.8, -2.],
        edge: [0., 1., 1.],
        edge_alpha: 1.,
        width: 3.,
        feather: 1.,
        skip_first,
        alpha: CapAlphaPolicy::opaque(),
    }
}

fn with_alpha(edge_alpha: f64, cutoff: Option<f64>, blend: bool, table: bool) -> CapEdgeInputs {
    let mut inputs = inputs(false);
    inputs.edge_alpha = edge_alpha;
    inputs.alpha = CapAlphaPolicy {
        cutoff,
        blend,
        depth_write: !blend || cutoff.is_some(),
        texel_alpha: if table {
            MASK_TABLE.to_vec()
        } else {
            Vec::new()
        },
    };
    inputs
}

/// A control frame: `cap` on the analytic cap at depth `depth`, background elsewhere.
fn control(cap: impl Fn(usize) -> [u8; 4], depth: f32) -> FrameCapture {
    let fan = CapFan::new(&inputs(false));
    let mut rgba = BACKGROUND.repeat(PIXELS * PIXELS);
    let mut depths = vec![1.; PIXELS * PIXELS];
    for index in 0..PIXELS * PIXELS {
        if fan.covers(index % PIXELS, index / PIXELS) {
            rgba[index * 4..index * 4 + 4].copy_from_slice(&cap(index));
            depths[index] = depth;
        }
    }
    FrameCapture {
        width: PIXELS as u32,
        height: PIXELS as u32,
        rgba,
        depth: depths,
    }
}

fn source() -> FrameCapture {
    control(|index| TEXELS[index % 2], 0.5)
}
fn behind() -> FrameCapture {
    control(|_| BEHIND, 0.75)
}

/// The controls' composite at every cap pixel, rounded to bytes (Either draws).
fn rounded(inputs: &CapEdgeInputs) -> FrameCapture {
    let (source, behind) = (source(), behind());
    let fan = CapFan::new(inputs);
    let mut capture = source.clone();
    for index in 0..PIXELS * PIXELS {
        if !fan.covers(index % PIXELS, index / PIXELS) {
            continue;
        }
        let pixel = PixelControls {
            source: &source.rgba[index * 4..index * 4 + 3],
            behind: &behind.rgba[index * 4..index * 4 + 3],
        };
        let distance = fan.edge_distance(index % PIXELS, index / PIXELS);
        let range = match alpha::outcome(&inputs.alpha, &fan.style(), &pixel, distance) {
            Outcome::Discarded => {
                capture.rgba[index * 4..index * 4 + 4].copy_from_slice(&BEHIND);
                capture.depth[index] = behind.depth[index];
                continue;
            }
            Outcome::Drawn(range) | Outcome::Either(range) => range,
        };
        for (byte, (low, high)) in capture.rgba[index * 4..index * 4 + 3].iter_mut().zip(range) {
            *byte = ((low + high) / 2.).round() as u8;
        }
        if !inputs.alpha.depth_write {
            capture.depth[index] = behind.depth[index];
        }
    }
    capture
}

fn violation(capture: &FrameCapture, inputs: &CapEdgeInputs) -> Option<String> {
    let (source, behind) = (source(), behind());
    let controls = CapEdgeControls {
        face: &source,
        source: &source,
        behind: &behind,
    };
    cap_edge_violation(capture, &controls, inputs)
}

fn set_pixel(capture: &mut FrameCapture, (column, row): (usize, usize), rgba: [u8; 4]) {
    let index = row * PIXELS + column;
    capture.rgba[index * 4..index * 4 + 4].copy_from_slice(&rgba);
}

#[test]
fn oblique_fan_draws_the_lines_through_the_centre_along_both_cap_axes() {
    let fan = CapFan::new(&inputs(false));
    // Centre (1.2, 0) maps to pixel (35.84, 32); local X is horizontal and local Z vertical.
    assert!((fan.centre[0] - 35.84).abs() < 1e-12 && (fan.centre[1] - 32.).abs() < 1e-12);
    assert!((fan.edge_distance(20, 31) - 0.5).abs() < 1e-12);
    assert!((fan.edge_distance(35, 10) - 0.34).abs() < 1e-9);
    // The section spans x in (-14/3, 6]: columns 17..=50 and rows 13..=50.
    assert!(!fan.covers(16, 31) && fan.covers(17, 31) && fan.covers(50, 50));
    assert!(!fan.covers(51, 31) && !fan.covers(30, 12));
}

#[test]
fn skip_first_keeps_one_ray_per_fan_triangle() {
    let fan = CapFan::new(&inputs(true));
    // Local X points left on screen and local Z up: (C,-X,+Z) and (C,+X,-Z) keep the vertical
    // Z ray, (C,+Z,+X) and (C,-Z,-X) the horizontal X ray.
    let distances = [(50, 20), (20, 44), (20, 20), (50, 44)].map(|(c, r)| fan.edge_distance(c, r));
    assert!((distances[0] - 14.66).abs() < 1e-9 && (distances[1] - 15.34).abs() < 1e-9);
    assert!((distances[2] - 11.5).abs() < 1e-12 && (distances[3] - 12.5).abs() < 1e-12);
}

#[test]
fn rounded_analytic_frames_pass_for_every_alpha_policy() {
    let mut cases = vec![inputs(false), inputs(true)];
    cases.push(with_alpha(0.25, Some(0.285), false, false));
    cases.push(with_alpha(-0.25, Some(0.285), false, false));
    cases.push(with_alpha(1., Some(0.5), false, true));
    cases.push(with_alpha(1., Some(0.5), true, true));
    cases.push(with_alpha(1., None, true, true));
    for inputs in cases {
        assert_eq!(violation(&rounded(&inputs), &inputs), None, "{inputs:?}");
    }
}

#[test]
fn a_two_level_error_on_the_edge_ramp_is_rejected() {
    let inputs = inputs(false);
    let mut capture = rounded(&inputs);
    // Pixel (20, 34) lies 2.5 px from the horizontal ray, at the start of the ramp.
    let index = 34 * PIXELS + 20;
    capture.rgba[index * 4 + 1] = capture.rgba[index * 4 + 1].saturating_sub(2);
    assert!(violation(&capture, &inputs).is_some());
}

#[test]
fn the_undefined_native_band_layout_is_rejected() {
    // A frame without the left ray (the earlier w = 0 division) differs by whole channels.
    let inputs = inputs(false);
    let mut capture = rounded(&inputs);
    set_pixel(&mut capture, (20, 31), TEXELS[0]);
    assert!(violation(&capture, &inputs).is_some());
}

#[test]
fn a_changed_background_pixel_is_rejected() {
    let inputs = inputs(false);
    let mut capture = rounded(&inputs);
    capture.depth[0] = 0.9;
    assert!(violation(&capture, &inputs).is_some());
}

#[test]
fn a_translucent_edge_kept_below_the_cutoff_is_rejected() {
    // Edge alpha 0.25 on the ray (t = 0) is below 0.285: the pixel must show what is behind.
    let inputs = with_alpha(0.25, Some(0.285), false, false);
    let mut capture = rounded(&inputs);
    set_pixel(&mut capture, (20, 31), [0, 255, 255, 255]);
    assert!(violation(&capture, &inputs).is_some());
}

#[test]
fn a_hollow_interior_that_is_drawn_is_rejected() {
    // Negative edge alpha draws only the band; at (45, 20) t = 1 and alpha 0 discards.
    let inputs = with_alpha(-0.25, Some(0.285), false, false);
    let mut capture = rounded(&inputs);
    let index = 20 * PIXELS + 45;
    capture.depth[index] = 0.5;
    assert!(violation(&capture, &inputs).is_some());
}

#[test]
fn an_unblended_translucent_texel_is_rejected() {
    // Blend: the red texel (alpha 64/255) at (44, 20) must mix with the grey behind it.
    let inputs = with_alpha(1., None, true, true);
    let mut capture = rounded(&inputs);
    set_pixel(&mut capture, (44, 20), TEXELS[0]);
    assert!(violation(&capture, &inputs).is_some());
}

#[test]
fn a_depth_write_from_the_transparent_pass_is_rejected() {
    let inputs = with_alpha(1., None, true, true);
    let mut capture = rounded(&inputs);
    capture.depth[20 * PIXELS + 44] = 0.5;
    assert!(violation(&capture, &inputs).is_some());
}

#[test]
fn a_masked_texel_that_is_drawn_is_rejected() {
    // Mask with cutoff 0.5: the interior red texel (alpha 64/255) at (44, 20) is discarded.
    let inputs = with_alpha(1., Some(0.5), false, true);
    let mut capture = rounded(&inputs);
    set_pixel(&mut capture, (44, 20), TEXELS[0]);
    assert!(violation(&capture, &inputs).is_some());
}
