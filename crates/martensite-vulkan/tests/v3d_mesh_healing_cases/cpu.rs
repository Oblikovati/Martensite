//! Real original geometry supplies bounded CPU diagnostics before image qualification.
use super::*;
use crate::shape_mesh::{MeshRequest, TriangulationSummary, mesh_shape};

#[test]
fn healed_case_has_authored_area_and_validity() {
    let original = read_dataset_shape("OCC1360.brep").unwrap();
    let result = case_repair(original).unwrap();
    let mass = check_mass(&result, &MassCheck::near(MassKind::Area, 407.599)).unwrap();
    assert!((mass.mass() - 407.599).abs() <= 407.599 * 0.01);
    assert!(check_shape_valid(&result).unwrap());
}

#[test]
fn case_planar_counts_preserve_boolean_deflection_reference() {
    let result = read_dataset_shape("bug32471.brep").unwrap();
    mesh_shape(&result, &MeshRequest::new(0.01)).unwrap();
    let summary = TriangulationSummary::of(&result).unwrap();
    assert_eq!((summary.triangles, summary.nodes), (2, 4));
    let reference =
        crate::shape_mesh::CountExpectation::from_checktrinfo_reference("0", 0., 1e-7).unwrap();
    assert_eq!(reference, crate::shape_mesh::CountExpectation::Any);
}
