//! Rejecting tests of the shadow-flip bound (support/shadow_flip_reference.rs).
use super::shadow_flip_reference::*;

/// The square `-5..5` at z = 0 facing +Z (BRepPreviewAPI_MakeBox with zero height).
fn floor() -> FlatFace {
    FlatFace {
        corners: [[-5., -5., 0.], [5., -5., 0.], [5., 5., 0.], [-5., 5., 0.]],
        normal: [0., 0., 1.],
    }
}

/// A light looking down -Z over `-5..5` that stores the floor at window depth 0.5, and a view
/// over `-10..10` looking down -Z; `bias` in depth units.
pub(super) fn overhead(bias_units: f64) -> ShadowFlipScene {
    let mut light = [0.; 16];
    (light[0], light[5], light[10], light[15]) = (0.2, 0.2, -0.1, 1.);
    let mut view = [0.; 16];
    (view[0], view[5], view[10], view[15]) = (10., 10., -10., 1.);
    ShadowFlipScene {
        faces: vec![floor()],
        view_inverse: view,
        view_depth_range: [-1., 1.],
        size: (64, 64),
        // The bias is ShadowMapBias * max(1 - N.L, 0.1) = 0.1 * ShadowMapBias here.
        lights: vec![FlipLight {
            matrix: light,
            position: [0., 0., 1.],
        }],
        rendered: Vec::new(),
        resolution: 16,
        bias: 10. * bias_units * DEPTH_UNIT,
        // A floor facing the light has no slope, so the offset changes nothing here.
        slope_factor: 1.,
        view_triangles: Vec::new(),
    }
}

#[test]
fn a_tap_within_one_depth_unit_of_the_stored_depth_is_a_proven_flip() {
    let scene = overhead(0.5);
    let margin = scene.closest_margin(32, 32).unwrap();
    assert!((margin - 0.5).abs() < 1e-6, "margin {margin}");
    assert!(scene.admits_flip(32, 32));
}

#[test]
fn a_flip_with_a_depth_margin_of_two_units_is_rejected() {
    let scene = overhead(2.);
    let margin = scene.closest_margin(32, 32).unwrap();
    assert!((margin - 2.).abs() < 1e-6, "margin {margin}");
    assert!(!scene.admits_flip(32, 32));
}

#[test]
fn pixels_off_the_faces_or_outside_the_light_are_rejected() {
    let scene = overhead(0.);
    // Window column 2 maps to x = -9.06: off the floor.
    assert!(scene.surface(2, 32).is_none());
    assert!(!scene.admits_flip(2, 32));
    let mut away = scene.clone();
    away.lights[0].matrix[12] = 3.; // Shift the light frustum off the floor.
    assert!(away.closest_margin(32, 32).is_none());
}

#[test]
fn texels_without_a_face_store_the_clear_depth() {
    let mut scene = overhead(0.);
    scene.faces[0].corners = [[0., 0., 0.], [5., 0., 0.], [5., 5., 0.], [0., 5., 0.]];
    let light = scene.lights[0];
    assert_eq!(scene.stored_depth(&light, 0, 0), 1.);
    assert!((scene.stored_depth(&light, 15, 15) - 0.5).abs() < 1e-12);
}

#[test]
fn sloped_faces_store_their_polygon_offset() {
    let mut scene = overhead(0.);
    // Tilt the floor to z = x / 10: the light depth -z / 20 + 0.5 falls x / 200 per unit, and
    // one texel of the 16-texel map spans 10 / 16 units.
    for corner in &mut scene.faces[0].corners {
        corner[2] = corner[0] / 10.;
    }
    let light = scene.lights[0];
    let plain = 0.5 - (-5. + 10.5 * 10. / 16.) / 200.;
    let step = 10. / 16. / 200.;
    assert!((scene.stored_depth(&light, 10, 3) - (plain + step)).abs() < 1e-12);
    scene.slope_factor = 0.;
    assert!((scene.stored_depth(&light, 10, 3) - plain).abs() < 1e-12);
}

#[test]
fn box_faces_carry_outward_normals_and_invert_round_trips() {
    let faces = box_faces([0., 0., 0.], [1., 2., 3.]);
    assert_eq!(faces.len(), 6);
    for face in &faces {
        let centre = [0, 1, 2].map(|i| face.corners.iter().map(|c| c[i]).sum::<f64>() / 4.);
        let outward = [0, 1, 2].map(|i| centre[i] - [0.5, 1., 1.5][i]);
        assert!((0..3).map(|i| outward[i] * face.normal[i]).sum::<f64>() > 0.);
    }
    let matrix = [
        2., 0., 0., 0., 0., 4., 0., 1., 0., 0., 8., 0., 1., 2., 3., 1.,
    ];
    let inverse = invert(&invert(&matrix));
    assert!(
        matrix
            .iter()
            .zip(inverse)
            .all(|(a, b)| (a - b).abs() < 1e-12)
    );
}
