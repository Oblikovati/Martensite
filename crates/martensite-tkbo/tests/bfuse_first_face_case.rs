//! Preserve first-face exploration order and command-host plain-copy alias semantics.
#[path = "boolean_unconditional_case.rs"]
pub(super) mod boolean_unconditional_case;
use martensite_tkbrep::{topexp::Explorer, topods::Shape, toptools::MapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;
const FIXTURES: [&[u8]; 4] = [
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro15203a.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro15203b.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro15203c.rle"),
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/pro15203d.rle"),
];
fn restored(index: usize) -> Shape {
    boolean_unconditional_case::restored_shape(FIXTURES[index], ShapeEnum::Solid)
}
fn first_exploded_face(shape: &Shape) -> Shape {
    let mut seen = MapOfShape::default();
    seen.add(shape.clone());
    let faces: Vec<Shape> = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)
        .expect("explode: expected face exploration")
        .filter(|face| seen.add(face.clone()))
        .collect();
    // Native explode captures independently establish one face for all four inputs.
    assert_eq!(
        faces.len(),
        1,
        "pro15203 fixture: expected one exploded face"
    );
    faces.into_iter().next().expect("source first face")
}
fn drawable_shape_alias(face: &Shape) -> Shape {
    // TopologyDrawableShape::Copy constructs its drawable with the existing myShape.
    let alias = face.clone();
    assert!(
        alias.is_equal(face),
        "plain copy: expected shared shape and orientation"
    );
    alias
}
fn fused_faces(first: &Shape, second: &Shape) -> Shape {
    boolean_unconditional_case::checked_fuse_shapes(first, second, 570.047)
}
fn restored_pair(first_index: usize, second_index: usize) -> Shape {
    let first = restored(first_index);
    let second = restored(second_index);
    let first_face = first_exploded_face(&first);
    let second_face = first_exploded_face(&second);
    fused_faces(&first_face, &second_face)
}
fn interleaved_copy(second_index: usize) -> Shape {
    let first = restored(1);
    let first_face = first_exploded_face(&first);
    let first_alias = drawable_shape_alias(&first_face);
    let second = restored(second_index);
    let second_face = first_exploded_face(&second);
    let second_alias = drawable_shape_alias(&second_face);
    fused_faces(&first_alias, &second_alias)
}
fn restored_pair_copy(second_index: usize) -> Shape {
    let first = restored(1);
    let second = restored(second_index);
    let first_face = first_exploded_face(&first);
    let first_alias = drawable_shape_alias(&first_face);
    let second_face = first_exploded_face(&second);
    let second_alias = drawable_shape_alias(&second_face);
    fused_faces(&first_alias, &second_alias)
}
/// Execute the original command order; `checked_result("P2")`.
pub fn checked_result(case: &str) -> Shape {
    match case {
        "O1" => restored_pair(0, 2),
        "O2" => restored_pair(0, 3),
        "O3" => restored_pair(1, 2),
        "O4" => restored_pair(1, 3),
        "P2" => interleaved_copy(2),
        "P3" => interleaved_copy(3),
        "S3" => restored_pair_copy(2),
        "S4" => restored_pair_copy(3),
        _ => panic!("case {case}: expected O1/O2/O3/O4/P2/P3/S3/S4"),
    }
}
