//! Reference replays of `tests/offset/faces_type_i/A1` and `A2`.

use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkernel::{message::ProgressRange, ncollection::List, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::geom_abs::JoinType;
use martensite_tkoffset::{
    brep_offset::MakeOffsetOptions, brep_offset_api::BRepOffsetAPIMakeThickSolid,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};

fn cylinder() -> Result<Shape, Failure> {
    Ok(BRepPrimAPIMakeCylinder::new(5.0, 15.0).shape()?.clone())
}

fn face(shape: &Shape, index: usize) -> Result<Shape, Failure> {
    let mut faces = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Face, &mut faces)?;
    Ok(faces.find_key(index)?.clone())
}

fn volume(shape: &Shape) -> Result<f64, Failure> {
    let mut properties = GProps::default();
    BRepGProp::volume_properties(shape, &mut properties, false, false, false)?;
    Ok(properties.mass())
}

fn thick_solid(distance: f64) -> Result<Shape, Failure> {
    let source = cylinder()?;
    let mut removed = List::default();
    removed.append(face(&source, 2)?);
    let mut maker = BRepOffsetAPIMakeThickSolid::new();
    maker.make_thick_solid_by_join(
        &source,
        &removed,
        distance,
        1.0e-7,
        MakeOffsetOptions {
            intersection: true,
            join: JoinType::Intersection,
            ..MakeOffsetOptions::default()
        },
        &ProgressRange::default(),
    )?;
    maker.build(&ProgressRange::default())?;
    maker.shape().cloned()
}

fn assert_end_checks(source: &str, result: &Shape, expected_volume: f64) -> Result<(), Failure> {
    assert!(BRepCheckAnalyzer::new(result, true, false, false)?.is_valid()?);
    for face in topexp::Explorer::new(result, ShapeEnum::Face, ShapeEnum::Shape)? {
        let mut area = GProps::default();
        BRepGProp::surface_properties(&face, &mut area, false, false)?;
        assert!(area.mass() >= 0.0, "{source}: negative result face area");
    }
    let measured = volume(result)?;
    assert!(
        (measured - expected_volume).abs() / expected_volume <= 1.0e-2,
        "{source}: checkprops result -v {expected_volume}; got {measured}"
    );
    Ok(())
}

#[test]
#[allow(non_snake_case)]
fn OffsetFacesTypeI__A1() -> Result<(), Failure> {
    assert_end_checks("tests/offset/faces_type_i/A1", &thick_solid(1.0)?, 631.46)
}

#[test]
#[allow(non_snake_case)]
fn OffsetFacesTypeI__A2() -> Result<(), Failure> {
    assert_end_checks("tests/offset/faces_type_i/A2", &thick_solid(-1.0)?, 474.38)
}
