use super::*;
use crate::primitive_shapes::{box_at, compound_of};
use crate::shape_mesh::{MeshRequest, mesh_shape};
#[test]
fn grid_has_all_900_independently_created_boxes() {
    let boxes = geometry::grid_boxes().unwrap();
    assert_eq!(boxes.len(), 900);
    for (index, shape) in boxes.iter().enumerate() {
        assert!(
            boxes[..index]
                .iter()
                .all(|previous| !shape.is_same(previous))
        );
    }
    assert!(crate::shape_checks::check_shape_valid(&compound_of(&boxes).unwrap()).unwrap());
}
#[test]
fn actual_intersecting_box_meshes_report_real_overlapping_faces() {
    let boxes = [
        box_at([0.; 3], [2.; 3]).unwrap(),
        box_at([1.; 3], [2.; 3]).unwrap(),
    ];
    for shape in &boxes {
        mesh_shape(shape, &MeshRequest::new(0.1)).unwrap();
    }
    let compound = compound_of(&boxes).unwrap();
    let result = geometry::actual_self_overlap_faces(&compound).unwrap();
    assert!(!result.is_null());
    assert_eq!(overlapping_face_count(&result), 6);
}

fn overlapping_face_count(shape: &Shape) -> usize {
    martensite_tkbrep::topexp::Explorer::new(
        shape,
        martensite_tkg3d::top_abs::ShapeEnum::Face,
        martensite_tkg3d::top_abs::ShapeEnum::Shape,
    )
    .unwrap()
    .count()
}
