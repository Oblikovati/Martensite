//! Validate authored identity and passive pcurve/face bounds before calling the classifier.
use super::{
    roles::GraphRoles,
    spec::{FaceWitness, FixtureSpec},
};
use martensite_tkbrep::{
    brep::BRepTool,
    brep_adaptor::{BRepAdaptorCurve, BRepAdaptorSurface},
    topexp::Explorer,
    topods::{Edge, Face},
};
use martensite_tkg3d::{
    adaptor3d::{Adaptor3dCurve, Adaptor3dSurface},
    top_abs::ShapeEnum,
};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

fn occurrences(edge: &Edge, face: &Face) -> usize {
    let shapes = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    shapes.filter(|found| found.is_same(edge)).count()
}

fn face_bounds(face: &Face, expected: FaceWitness) {
    let support = BRepAdaptorSurface::with_face(face, true).unwrap();
    let actual = [
        support.first_u_parameter().unwrap(),
        support.last_u_parameter().unwrap(),
        support.first_v_parameter().unwrap(),
        support.last_v_parameter().unwrap(),
    ];
    assert_eq!(
        actual, expected.bounds,
        "expected exact authored/native face bounds"
    );
    assert_eq!(support.get_type().unwrap() as i32, expected.kind);
}

fn pcurve(edge: &Edge, face: &Face, expected: FaceWitness, parameter: f64) {
    let curve = BRepTool::curve_on_face(edge, face).unwrap();
    assert!(!curve.pcurve.is_null(), "expected actual selected pcurve");
    assert!(
        curve.is_stored,
        "expected genuine stored pcurve in valid graph"
    );
    assert_eq!([curve.first, curve.last], expected.range);
    let uv = curve.pcurve.value(parameter).unwrap();
    assert_eq!([uv.x(), uv.y()], expected.uv);
    face_bounds(face, expected);
}

fn valid_roles(spec: &FixtureSpec, roles: &GraphRoles) {
    for shape in [&roles.complete, &*roles.first, &*roles.second] {
        assert!(
            BRepCheckAnalyzer::new(shape, true, false, false)
                .unwrap()
                .is_valid()
                .unwrap(),
            "{}: expected native-valid graph/face",
            spec.name
        );
    }
    let actual = [
        occurrences(&roles.edge, &roles.first),
        occurrences(&roles.edge, &roles.second),
    ];
    assert_eq!(actual, spec.occurrences);
    assert_eq!(roles.first.is_same(&roles.second), spec.same_face);
}

/// Validate complete topology, role membership and real pcurve selection; `check(spec, &roles)`.
pub(super) fn check(spec: &FixtureSpec, roles: &GraphRoles) {
    valid_roles(spec, roles);
    let curve = BRepAdaptorCurve::with_edge(&roles.edge).unwrap();
    let midpoint = 0.5 * (curve.first_parameter().unwrap() + curve.last_parameter().unwrap());
    pcurve(&roles.edge, &roles.first, spec.first, midpoint);
    let mut second_edge = roles.edge.clone();
    if spec.same_face {
        second_edge.reverse();
    }
    pcurve(&second_edge, &roles.second, spec.second, midpoint);
}
