//! Pinned normal vectors, oriented directions, output retention and native exceptions.
use martensite_tkbool::top_ope_brep_tool::{min_duv, nggeom_f, nt};
use martensite_tkbrep::{brep::BRepBuilder, topods::Face};
use martensite_tkernel::{
    ncollection::Array2,
    standard::{Failure, Handle},
};
use martensite_tkg3d::{
    geom::{
        GeomBezierSurface, GeomConicalSurface, GeomCylindricalSurface, GeomPlane,
        GeomSphericalSurface, GeomSurface, GeomToroidalSurface,
    },
    top_abs::Orientation,
};
use martensite_tkmath::gp::{Axis3, Direction3, Point2, Point3, Vector3};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
use std::f64::consts::{FRAC_PI_2, PI};
#[path = "face_normals/analytic.rs"]
mod analytic;
#[test]
fn generic_periodic_surface_normals_match_closed_forms() {
    let samples = [
        (
            cylinder_face(),
            Point2::new(0.4, 0.2),
            [0.921_060_994_002_885_1, 0.389_418_342_308_650_5, 0.],
        ),
        (
            torus_face(),
            Point2::new(0.4, 0.2),
            [
                0.902_701_096_375_46,
                0.381_655_902_095_048_3,
                0.198_669_330_795_061_22,
            ],
        ),
    ];
    for (face, uv, expected) in samples {
        assert_geometric_and_oriented_normal(&face, uv, expected);
    }
}

#[test]
fn native_face_normals_match_analytic_singular_and_failed_cases() {
    let mut keys = std::collections::BTreeSet::new();
    for line in include_str!("fixtures/face_normals.expected").lines() {
        let expected = parse_observation(line);
        let key = (
            expected[0] as i32,
            expected[1] as usize,
            expected[2] as usize,
        );
        assert!(keys.insert(key), "duplicate baseline row: {line}");
        compare_observation(observe(key.0, key.1, key.2), &expected[3..], line);
    }
    assert_eq!(keys.len(), 80);
}
fn cylinder_face() -> Face {
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomCylindricalSurface::new(Axis3::default(), 2.).unwrap()).cast();
    BRepBuilderAPIMakeFace::from_surface_bounds(&surface, (0., 2. * PI, -1., 1.), 1e-7)
        .unwrap()
        .face()
        .unwrap()
}

fn torus_face() -> Face {
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomToroidalSurface::new(Axis3::default(), 5., 2.).unwrap()).cast();
    BRepBuilderAPIMakeFace::from_surface_bounds(&surface, (0., 2. * PI, -PI, PI), 1e-7)
        .unwrap()
        .face()
        .unwrap()
}

fn assert_geometric_and_oriented_normal(face: &Face, uv: Point2, expected: [f64; 3]) {
    let mut vector = Vector3::default();
    let mut oriented = Direction3::X;
    assert!(nggeom_f(uv, face, &mut vector).unwrap());
    assert!(nt(uv, face, &mut oriented).unwrap());
    assert_vector_close(vector, expected);
    assert_vector_close(Vector3::from(oriented), expected);
}

fn assert_vector_close(actual: Vector3, expected: [f64; 3]) {
    for (actual, expected) in [actual.x(), actual.y(), actual.z()]
        .into_iter()
        .zip(expected)
    {
        assert!((actual - expected).abs() < 1e-12, "{actual} != {expected}");
    }
}

fn parse_observation(line: &str) -> Vec<f64> {
    let expected: Vec<f64> = line
        .split_whitespace()
        .map(|field| field.parse().unwrap())
        .collect();
    assert_eq!(
        expected.len(),
        12,
        "baseline shape must be 3 keys + 9 observations"
    );
    expected
}
fn compare_observation(actual: [f64; 9], expected: &[f64], label: &str) {
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert!(
            (actual - expected).abs() < 1e-12,
            "{label}: field {index}, {actual} != {expected}"
        );
    }
}

fn observe(kind: i32, orientation: usize, point: usize) -> [f64; 9] {
    let mut face = fixture(kind);
    face.set_orientation(
        [
            Orientation::Forward,
            Orientation::Reversed,
            Orientation::Internal,
            Orientation::External,
        ][orientation],
    );
    let points = [
        Point2::new(0.3, 0.2),
        Point2::new(0., 0.),
        Point2::new(0.4, FRAC_PI_2),
        Point2::new(0.4, -FRAC_PI_2),
    ];
    let uv = if kind == 2 && point == 2 {
        Point2::new(0.4, 0.)
    } else {
        points[point]
    };
    let mut vector = Vector3::new(7., 8., 9.);
    let mut normal = Direction3::new(1., 0., 0.).unwrap();
    let a = normal_status(nggeom_f(uv, &face, &mut vector));
    let b = normal_status(nt(uv, &face, &mut normal));
    [
        a,
        b,
        min_duv(&face).unwrap(),
        vector.x(),
        vector.y(),
        vector.z(),
        normal.x(),
        normal.y(),
        normal.z(),
    ]
}
fn normal_status(result: Result<bool, Failure>) -> f64 {
    match result {
        Ok(done) => f64::from(done),
        Err(failure) => {
            assert_eq!(failure.exception_type(), "Standard_ConstructionError");
            -1.
        }
    }
}
fn fixture(kind: i32) -> Face {
    if kind >= 3 {
        return degenerate_fixture(kind);
    }
    let surface: Handle<dyn GeomSurface> = match kind {
        0 => Handle::new(GeomPlane::new(Axis3::default())).cast(),
        1 => Handle::new(GeomSphericalSurface::new(Axis3::default(), 2.).unwrap()).cast(),
        _ => Handle::new(GeomConicalSurface::new(Axis3::default(), 0.35, 0.).unwrap()).cast(),
    };
    let (first, last) = if kind == 1 {
        (-FRAC_PI_2, FRAC_PI_2)
    } else {
        (-1., 1.)
    };
    BRepBuilderAPIMakeFace::from_surface_bounds(&surface, (0., 2. * PI, first, last), 1e-7)
        .unwrap()
        .face()
        .unwrap()
}
fn degenerate_fixture(kind: i32) -> Face {
    let mut poles = Array2::with_bounds(1, 2, 1, 2).unwrap();
    for i in 1..=2 {
        for j in 1..=2 {
            *poles.change_value(i, j).unwrap() = if kind == 3 {
                Point3::default()
            } else {
                Point3::new(f64::from(i) * 1e-9, f64::from(j) * 1e-9, 0.)
            };
        }
    }
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomBezierSurface::new(&poles).unwrap()).cast();
    let mut face = Face::default();
    BRepBuilder::default()
        .make_face_on_surface(&mut face, &surface, 1e-7)
        .unwrap();
    face
}
