use super::*;
use crate::shape_checks::{MassCheck, MassKind, check_mass};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkg3d::top_abs::ShapeEnum;

fn explored_faces(shape: &Shape) -> Vec<Shape> {
    let explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    explorer.collect()
}

fn assert_source_mesh_clean(shape: &Shape) {
    let topology = crate::mesh_topology_check::check_mesh_topology(shape).unwrap();
    assert!(topology.is_clean());
}

#[test]
fn actual_nested_boxes_general_fuse_has_two_solids_and_one_shared_face() {
    let model = geometry::actual_shared_solids().unwrap();
    assert_eq!(model.boxes.len(), 2);
    let volume = check_mass(&model.result, &MassCheck::near(MassKind::Volume, 2000.))
        .unwrap()
        .mass();
    assert!((volume - 2000.).abs() < 1e-10);
    let first = explored_faces(&model.parts[0]);
    let second = explored_faces(&model.parts[1]);
    assert_eq!((first.len(), second.len()), (6, 6));
    let shared = first
        .iter()
        .filter(|face| second.iter().any(|other| face.is_same(other)))
        .count();
    assert_eq!(shared, 1);
}

#[test]
fn sequential_shared_face_meshes_and_actual_null_shading_remesh_keep_topology_clean() {
    let model = geometry::actual_shared_solids().unwrap();
    geometry::sequential_mesh_checks(&model.parts).unwrap();
    assert_source_mesh_clean(&model.parts[0]);
    remove_triangulation(&model.result).unwrap();
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    show_source_parts(&mut session, &model.parts).unwrap();
    assert_source_mesh_clean(&model.parts[0]);
}

#[test]
fn both_actual_full_argument_reports_retain_copied_second_and_no_fault_bindings() {
    let model = geometry::actual_shared_solids().unwrap();
    let diagnostics = full_source_diagnostics(&model).unwrap();
    assert!(diagnostics.bindings.is_empty());
    assert!(diagnostics.reports[0].copied_second.is_none());
    assert!(diagnostics.reports[1].copied_second.is_some());
    for report in diagnostics.reports {
        assert_eq!(report.output, "Shape(s) seem(s) to be valid for BOP.\n");
    }
}
