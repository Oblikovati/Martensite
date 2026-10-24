//! Shared face identity must survive an ASCII archive with repeated locations (#237).
use martensite_tkbrep::{
    brep::BRepBuilder, brep_tools, topexp, topods::Shape, toptools::IndexedMapOfShape,
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkg3d::top_abs::ShapeEnum;

fn restored_assembly() -> Shape {
    let bytes = include_bytes!("fixtures/repeated-location-solids.brep");
    let header_end = bytes.iter().position(u8::is_ascii_whitespace).unwrap();
    let mut stream = IoStream::from_bytes(bytes[header_end..].to_vec());
    let mut shape = Shape::default();
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .unwrap();
    shape
}

fn unique_count(shape: &Shape, kind: ShapeEnum, located: bool) -> usize {
    let mut unique = IndexedMapOfShape::default();
    for entity in topexp::Explorer::new(shape, kind, ShapeEnum::Shape).unwrap() {
        let key = if located {
            entity
        } else {
            entity
                .located(&martensite_tkmath::toploc::Location::default(), false)
                .unwrap()
        };
        unique.add(key);
    }
    unique.extent()
}

#[test]
fn archive_preserves_fourteen_face_definitions_and_three_solids() {
    let shape = restored_assembly();
    for (kind, expected) in [
        (ShapeEnum::Face, 14),
        (ShapeEnum::Shell, 3),
        (ShapeEnum::Solid, 3),
    ] {
        assert_eq!(
            unique_count(&shape, kind, false),
            expected,
            "unlocated {kind:?} identity count"
        );
    }
}

#[test]
fn repeated_face_placements_remain_distinct_instances() {
    let shape = restored_assembly();
    for (kind, expected) in [
        (ShapeEnum::Face, 15),
        (ShapeEnum::Shell, 3),
        (ShapeEnum::Solid, 3),
    ] {
        assert_eq!(
            unique_count(&shape, kind, true),
            expected,
            "located {kind:?} identity count"
        );
    }
}
