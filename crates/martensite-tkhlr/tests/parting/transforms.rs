// SPDX-License-Identifier: GPL-2.0-only
use super::{
    coords,
    fixtures::{Model, cylinder},
    selected,
};
use martensite_tkbrep::topods::Solid;
use martensite_tkhlr::parting::{PartingOptions, PartingOrigin, PartingResult, PartingSource};
use martensite_tkmath::{
    gp::{Direction3, Point3, Transform3},
    toploc::Location,
};

#[test]
fn seam_coincident_generator_is_reported_once() {
    let model = Model::new(cylinder(5., 10.));
    let curves = selected(&model, Direction3::Y, PartingOptions::default());
    assert_eq!(curves.len(), 2);
    let mut sides = Vec::new();
    for curve in &curves {
        let PartingOrigin::Smooth { uv_ends, .. } = curve.origin() else {
            panic!("expected smooth cylinder generator");
        };
        let mut ends = curve.endpoints().map(coords);
        ends.sort_by(|a, b| a[2].total_cmp(&b[2]));
        let side = ends[0][0].signum() * 5.;
        for (actual, z) in ends.into_iter().zip([0., 10.]) {
            assert_point(actual, [side, 0., z]);
        }
        sides.push((side, uv_ends[0][0]));
    }
    sides.sort_by(|a, b| a.0.total_cmp(&b.0));
    assert_eq!(sides[0].0, -5.);
    assert_eq!(sides[1].0, 5.);
    assert!(sides[1].1 == 0. || sides[1].1 == std::f64::consts::TAU);
}

#[test]
fn located_cylinder_retains_world_generators_and_native_face_scope() {
    let mut transform = Transform3::default();
    transform
        .set_values([[0., 0., 1., 20.], [0., 1., 0., 30.], [-1., 0., 0., 40.]])
        .unwrap();
    let native = cylinder(5., 10.);
    let located = Solid::unchecked(native.moved(&Location::new(transform), true).unwrap());
    let pull = Direction3::new(0., 0., -1.).unwrap();
    let curves = selected(&Model::new(located), pull, PartingOptions::default());
    assert_eq!(curves.len(), 2);
    for curve in &curves {
        let mut ends = curve.endpoints().map(coords);
        ends.sort_by(|a, b| a[0].total_cmp(&b[0]));
        let y = if ends[0][1] < 30. { -5. } else { 5. };
        for (actual, z) in ends.into_iter().zip([0., 10.]) {
            let expected = Point3::new(0., y, z).transformed(transform);
            assert_point(actual, coords(expected));
        }
    }
}

#[test]
fn detached_result_and_stable_read_request_can_cross_threads() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<PartingResult>();
    assert_send_sync::<PartingSource<'static>>();
}

fn assert_point(actual: [f64; 3], expected: [f64; 3]) {
    for (value, expected) in actual.into_iter().zip(expected) {
        assert!(
            (value - expected).abs() <= 2e-14,
            "{value} versus {expected}"
        );
    }
}
