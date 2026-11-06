// SPDX-License-Identifier: GPL-2.0-only
//! #386: nonzero trim contributions must not disappear below a parameter cutoff.
use martensite_tkbrep::{
    topexp,
    topods::{Shape, face},
    toptools::IndexedMapOfShape,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Point3, Vector3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakePrism;
use martensite_tktopalgo::{
    brep_builder_api::{
        BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
    },
    brep_check::BRepCheckAnalyzer,
    brep_gprop::{BRepGProp, VolumeGKFlags},
};

fn skew_polygon(origin: [f64; 3], scale: f64, skew: f64) -> ([Point3; 4], f64) {
    let points = [[0., 0.], [4., 0.], [4., 3.], [0., 3.]].map(|[x, y]| {
        Point3::new(
            (origin[0] + (x * scale) * (1. + skew)) + (y * scale) * skew,
            origin[1] + y * scale,
            origin[2],
        )
    });
    (points, polygon_area(points, origin) * (2. * scale))
}

fn polygon_area(points: [Point3; 4], origin: [f64; 3]) -> f64 {
    let local = points.map(|point| [point.x() - origin[0], point.y() - origin[1]]);
    (0..4)
        .map(|i| {
            let [a, b] = [local[i], local[(i + 1) % 4]];
            a[0] * b[1] - a[1] * b[0]
        })
        .sum::<f64>()
        * 0.5
}

fn skew_prism(points: [Point3; 4], scale: f64) -> Shape {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for point in points {
        polygon.add_point(point).unwrap();
    }
    polygon.close().unwrap();
    let wire = polygon.wire().unwrap();
    let face = BRepBuilderAPIMakeFace::from_wire(&wire, true)
        .unwrap()
        .face()
        .unwrap();
    let mut prism =
        BRepPrimAPIMakePrism::new(&face, Vector3::new(0., 0., 2. * scale), true, true).unwrap();
    prism.shape().unwrap().clone()
}

fn assert_skew_volume(origin: [f64; 3], scale: f64, skew: f64) {
    let (shape, expected) = checked_skew_prism(origin, scale, skew);
    let (properties, achieved) = skew_volume_properties(&shape);
    assert!(
        (0. ..=1e-12).contains(&achieved),
        "origin={origin:?}, scale={scale}, skew={skew}, achieved={achieved}"
    );
    let relative = (properties.mass() - expected).abs() / expected;
    assert!(
        relative <= 1e-12,
        "origin={origin:?}, scale={scale}, skew={skew}, actual={}, expected={expected}, relative={relative}, achieved={achieved}",
        properties.mass()
    );
}

fn checked_skew_prism(origin: [f64; 3], scale: f64, skew: f64) -> (Shape, f64) {
    let (points, expected) = skew_polygon(origin, scale, skew);
    let shape = skew_prism(points, scale);
    assert!(
        BRepCheckAnalyzer::new(&shape, true, false, true)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    (shape, expected)
}

fn sorted_reference_face_areas(points: [Point3; 4], scale: f64) -> Vec<f64> {
    let origin = [points[0].x(), points[0].y(), points[0].z()];
    let area = polygon_area(points, origin);
    let mut areas = vec![area, area];
    for index in 0..4 {
        areas.push(points[index].distance(&points[(index + 1) % 4]) * (2. * scale));
    }
    areas.sort_by(f64::total_cmp);
    areas
}

fn assert_independent_face_areas(shape: &Shape, expected: &[f64]) {
    let mut faces = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Face, &mut faces).unwrap();
    assert_eq!(faces.extent(), 6);
    let mut areas = Vec::new();
    for index in 1..=faces.extent() {
        areas.push(trimmed_face_area(faces.find_key(index).unwrap()));
    }
    areas.sort_by(f64::total_cmp);
    for (actual, expected) in areas.into_iter().zip(expected) {
        assert!(
            (actual - expected).abs() <= expected * 1e-11,
            "face area {actual}, expected {expected}"
        );
    }
}

fn trimmed_face_area(shape: &Shape) -> f64 {
    let mut properties = GProps::new();
    BRepGProp::surface_properties(&face(shape).unwrap(), &mut properties, false, false).unwrap();
    properties.mass()
}

fn skew_volume_properties(shape: &Shape) -> (GProps, f64) {
    let mut properties = GProps::new();
    let flags = VolumeGKFlags::default();
    let achieved =
        BRepGProp::volume_properties_gk(shape, &mut properties, 1e-12, true, flags).unwrap();
    (properties, achieved)
}

#[test]
fn small_skewed_prism_retains_all_trimmed_volume_contributions() {
    assert_skew_volume([0.; 3], 0.01, 5e-10);
    let (points, _) = skew_polygon([0.; 3], 0.01, 5e-10);
    assert_independent_face_areas(
        &skew_prism(points, 0.01),
        &sorted_reference_face_areas(points, 0.01),
    );
}

#[test]
fn default_full_properties_retain_the_small_skewed_prism_volume() {
    let (points, expected) = skew_polygon([0.; 3], 0.01, 5e-10);
    let mut properties = GProps::new();
    BRepGProp::volume_properties(
        &skew_prism(points, 0.01),
        &mut properties,
        true,
        false,
        false,
    )
    .unwrap();
    assert!((properties.mass() - expected).abs() <= expected * 1e-12);
}

#[test]
fn nonzero_trim_contributions_survive_sign_scale_and_translation() {
    for origin in [[0.; 3], [10., -20., 30.]] {
        for (scale, skew) in [
            (0.01, -5e-10),
            (0.01, 0.),
            (0.01, 1e-12),
            (0.01, 1e-10),
            (0.01, 5e-10),
            (0.01, 1e-8),
            (1., 5e-10),
            (100., 5e-10),
        ] {
            assert_skew_volume(origin, scale, skew);
        }
    }
}
