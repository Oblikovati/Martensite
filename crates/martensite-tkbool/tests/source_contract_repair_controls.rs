//! Causal controls pin native line parameters, geometry-only dispatch and unique topology identity.
use martensite_boolean_cases::common_operation;
#[path = "support/bug28054_geometry.rs"]
mod source_geometry;
#[path = "support/bug29573_counts.rs"]
mod source_topology;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{
    geom::{GeomCurve, GeomLine},
    top_abs::ShapeEnum,
};
use martensite_tkmath::gp::{Direction3, Point3};

fn assert_case_line_basis_28054(curve: &Handle<dyn GeomCurve>) {
    let basis: Handle<GeomLine> = curve.cast();
    assert!(
        !basis.is_null(),
        "native mkedge: stored castNull={}, expected GeomLine basis",
        basis.is_null()
    );
    assert_eq!(
        curve.value(0.).unwrap(),
        Point3::default(),
        "native basis line must retain its origin"
    );
}

#[test]
fn trimmed_line_keeps_case_center_and_minus100_plus100_parameters() {
    let shape = source_geometry::case_line_edge_28054(
        Point3::default(),
        Direction3::new(1., 0., 1.).unwrap(),
    )
    .unwrap();
    let edge = topods::edge(&shape).unwrap();
    let (curve, first, last) = BRepTool::curve(&edge).unwrap();
    assert_eq!(
        [first, last],
        [-100., 100.],
        "native trimmed-line edge must preserve its parameter chart"
    );
    assert_case_line_basis_28054(&curve);
}

#[test]
fn plain_geometry_rotate_retains_db_rep_shape_copy_identity() {
    let ([first, second, copied_first, copied_second], _) =
        source_geometry::case_inputs_28054().unwrap();
    assert!(
        first.is_equal(&copied_first),
        "native plain rotate must ignore copied topology-command f3"
    );
    assert!(
        second.is_equal(&copied_second),
        "native plain rotate must ignore copied topology-command f4"
    );
}

#[test]
fn closed_shared_vertex_cannot_satisfy_case_two_unique_vertex_predicate() {
    let bytes = include_bytes!("fixtures/source-contract-repair/closed_shared_vertex.brep");
    let shape = common_operation::restored_shape(bytes, ShapeEnum::Edge);
    let edge = topods::edge(&shape).unwrap();
    let counts = source_topology::source_edge_counts_29573(&edge);
    assert_ne!(
        counts,
        [2, 1],
        "same vertex occurring twice must not pass original uniqueV2/E1 predicate"
    );
    assert_eq!(
        counts,
        [1, 1],
        "independent native closed edge has one unique vertex and one edge"
    );
}
