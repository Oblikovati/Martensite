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

#[test]
fn noncardinal_box_uses_the_exact_source_placement_signs() {
    use martensite_tkmath::gp::{Axis1, Plane3};
    let original = Model::new(super::block());
    let axis = Axis1::new(Point3::default(), Direction3::new(1., 2., 3.).unwrap());
    let transform = Transform3::rotation(axis, 0.3).unwrap();
    let pull = Plane3::default()
        .transformed(transform)
        .position()
        .x_direction();
    let located = Solid::unchecked(
        original
            .solid
            .moved(&Location::new(transform), true)
            .unwrap(),
    );
    let model = Model::new(located);
    let expected = exact_box_edge_ids(&original, transform, pull);
    let actual: Vec<_> = selected(&model, pull, PartingOptions::default())
        .iter()
        .map(|curve| {
            let PartingOrigin::Shared { edge, .. } = curve.origin() else {
                panic!("expected native edge");
            };
            edge
        })
        .collect();
    assert_eq!(actual, expected);
    assert!(!actual.is_empty());
}

fn exact_box_edge_ids(
    model: &Model,
    transform: Transform3,
    pull: Direction3,
) -> Vec<martensite_tkhlr::parting::PartingEdgeId> {
    use martensite_tkbrep::topexp::Explorer;
    use martensite_tkg3d::top_abs::ShapeEnum;
    let signs: Vec<_> = model
        .faces
        .iter()
        .map(|face| placed_box_face_sign(face, transform, pull))
        .collect();
    let (_, edges) = model.bindings();
    edges
        .into_iter()
        .filter_map(|edge| {
            let signs: Vec<_> = model
                .faces
                .iter()
                .zip(&signs)
                .filter_map(|(face, &sign)| {
                    let incident = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape)
                        .unwrap()
                        .any(|child| child.is_same(edge.occurrence));
                    incident.then_some(sign)
                })
                .collect();
            assert_eq!(signs.len(), 2);
            (signs[0] != signs[1]).then_some(edge.id)
        })
        .collect()
}

fn placed_box_face_sign(
    face: &martensite_tkbrep::topods::Face,
    transform: Transform3,
    pull: Direction3,
) -> std::cmp::Ordering {
    use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
    use martensite_tkg3d::{adaptor3d::Adaptor3dSurface, top_abs::Orientation};
    use martensite_tkmath::predicates::compare_scalar_triple_product_3d;
    let plane = BRepAdaptorSurface::with_face(face, false)
        .unwrap()
        .plane()
        .unwrap();
    let frame = plane.position();
    let column = |direction: Direction3| {
        let basis = *direction.coordinates();
        let axis = basis
            .iter()
            .position(|component| component.abs() == 1.)
            .unwrap();
        assert!(
            basis
                .iter()
                .enumerate()
                .all(|(index, &component)| index == axis || component == 0.)
        );
        [1, 2, 3].map(|row| basis[axis] * transform.value(row, axis as i32 + 1).unwrap())
    };
    let sign = compare_scalar_triple_product_3d(
        *pull.coordinates(),
        column(frame.x_direction()),
        column(frame.y_direction()),
    )
    .unwrap();
    if face.orientation() == Orientation::Reversed {
        sign.reverse()
    } else {
        sign
    }
}
