//! This supplemental bow-tie input stays unchanged in native ShapeFixFace::perform;
//! the separate `shape_fix_face_loop_wire` suite covers direct loop reconstruction.
use martensite_tkbrep::topexp::Explorer;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkshhealing::shape_extend::ShapeExtendStatus;
use martensite_tkshhealing::shape_fix::ShapeFixFace;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon};

#[test]
fn bowtie_perform_matches_native_no_change() {
    let source = bowtie_face();
    let source_shape = source.clone().into_shape();
    let fixer = ShapeFixFace::new(source_shape.clone());

    let changed = fixer.perform().unwrap();
    let result = fixer.result();
    let actual = [
        i32::from(changed),
        i32::from(fixer.status(ShapeExtendStatus::Done)),
        i32::from(fixer.status(ShapeExtendStatus::Done1)),
        i32::from(fixer.status(ShapeExtendStatus::Done7)),
        result.shape_type().unwrap() as i32,
        Explorer::new(&result, ShapeEnum::Face, ShapeEnum::Shape)
            .unwrap()
            .count() as i32,
        i32::from(result.is_same(&source_shape)),
    ];
    let expected = include_str!("fixtures/behavior/shape_fix_face_perform_bowtie.expected")
        .split_whitespace()
        .skip(1)
        .map(|value| value.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(actual.as_slice(), expected);
}

fn bowtie_face() -> martensite_tkbrep::topods::Face {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    polygon.add_point(Point3::new(0.0, 0.0, 0.0)).unwrap();
    polygon.add_point(Point3::new(1.0, 1.0, 0.0)).unwrap();
    polygon.add_point(Point3::new(1.0, 0.0, 0.0)).unwrap();
    polygon.add_point(Point3::new(0.0, 1.0, 0.0)).unwrap();
    polygon.close().unwrap();
    BRepBuilderAPIMakeFace::from_wire(&polygon.wire().unwrap(), true)
        .unwrap()
        .face()
        .unwrap()
}
