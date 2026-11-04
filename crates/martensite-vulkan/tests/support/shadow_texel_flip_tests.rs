//! Rejecting tests of the sub-texel selection and caster-silhouette bounds
//! (support/shadow_flip_reference.rs `admits_selection_flip`, support/shadow_silhouette_reference.rs).
use super::shadow_flip_reference::*;
use super::shadow_flip_reference_tests::overhead;
use super::shadow_silhouette_reference::*;

/// The overhead scene with a caster at z = 1 over x >= 0 (light texels 8..16 of 16), listed
/// first: floor taps in texel 8 or above are shadowed, below are lit, so the decision changes
/// across the border at light texel coordinate 8 (world x = 0).
fn half_covered() -> ShadowFlipScene {
    // A bias of five depth units keeps the floor's own taps lit without a depth tie.
    let mut scene = overhead(5.);
    let caster = FlatFace {
        corners: [[0., -5., 1.], [5., -5., 1.], [5., 5., 1.], [0., 5., 1.]],
        normal: [0., 0., 1.],
    };
    scene.faces.insert(0, caster);
    scene
}

/// The scene with the view moved so the closest decision-changing tap of window pixel (31, 32)
/// lies `distance` texels from the border; world x maps to 1.6 light texels per unit.
fn with_tie_at(distance: f64) -> ShadowFlipScene {
    let mut scene = half_covered();
    let tie = scene.closest_selection_tie(31, 32).unwrap();
    // The closest tap lies below the border, so moving the view towards +x shortens it.
    scene.view_inverse[12] += (tie - distance) / 1.6;
    let moved = scene.closest_selection_tie(31, 32).unwrap();
    assert!(
        (moved - distance).abs() < 1e-9,
        "tie {moved}, expected {distance}"
    );
    scene
}

#[test]
fn a_tap_within_the_sub_texel_precision_of_a_deciding_border_is_admitted() {
    let scene = with_tie_at(1. / 512.);
    assert!(scene.admits_selection_flip(31, 32, 1. / 256.));
}

#[test]
fn a_tap_one_sixty_fourth_of_a_texel_from_the_border_is_rejected() {
    let scene = with_tie_at(1. / 64.);
    assert!(!scene.admits_selection_flip(31, 32, 1. / 256.));
    // Not a depth tie either: the caster stores 0.45 against the floor's 0.5.
    assert!(!scene.admits_flip(31, 32));
}

#[test]
fn a_border_between_equally_decided_texels_is_no_selection_tie() {
    // Without the caster no tap decision changes anywhere.
    let scene = overhead(5.);
    assert_eq!(scene.closest_selection_tie(31, 32), None);
    assert!(!scene.admits_selection_flip(31, 32, 1. / 256.));
}

/// A 64x64 RGB image of `colour` everywhere except `pixel`, which is `at_pixel`.
fn image(colour: u8, pixel: (usize, usize), at_pixel: u8) -> Vec<u8> {
    let mut rgb = vec![colour; 64 * 64 * 3];
    let at = 3 * (pixel.1 * 64 + pixel.0);
    rgb[at..at + 3].fill(at_pixel);
    rgb
}

#[test]
fn a_pixel_a_shifted_capture_reproduces_is_admitted() {
    let native = image(0, (31, 32), 200);
    let controls = [image(0, (31, 32), 90), image(0, (31, 32), 200)];
    assert!(admits_shifted_raster_flip(
        (31, 32),
        (&native, &controls),
        64
    ));
}

#[test]
fn a_pixel_no_shifted_capture_reproduces_is_rejected() {
    let native = image(0, (31, 32), 200);
    let controls = [image(0, (31, 32), 90), image(0, (31, 32), 201)];
    assert!(!admits_shifted_raster_flip(
        (31, 32),
        (&native, &controls),
        64
    ));
    assert!(!admits_shifted_raster_flip((31, 32), (&native, &[]), 64));
}

#[test]
fn silhouette_texels_are_found_only_where_a_tap_reads_one() {
    let scene = half_covered();
    assert!(reads_caster_silhouette(&scene, (31, 32), 1));
    // Window column 16 is world x = -4.84: every tap reads floor texels 0..2.
    assert!(!reads_caster_silhouette(&scene, (16, 32), 1));
}

#[test]
fn control_shifts_stay_within_half_a_snap_step_along_one_axis() {
    for shift in CONTROL_SHIFTS {
        assert!(shift.iter().filter(|s| **s != 0.).count() == 1, "{shift:?}");
        assert!(shift.iter().all(|s| s.abs() <= HALF_SNAP_STEP), "{shift:?}");
    }
}
