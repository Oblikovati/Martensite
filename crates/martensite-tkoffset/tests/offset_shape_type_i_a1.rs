//! Reference replay of `tests/offset/shape_type_i/A1`.

use martensite_tkbrep::{topexp::Explorer, topods::Face};
use martensite_tkernel::{message::ProgressRange, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::geom_abs::JoinType;
use martensite_tkoffset::brep_offset::{BRepOffsetMakeOffset, BRepOffsetMode, MakeOffsetOptions};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};

fn volume(shape: &martensite_tkbrep::topods::Shape) -> Result<f64, Failure> {
    let mut properties = GProps::default();
    BRepGProp::volume_properties(shape, &mut properties, false, false, false)?;
    Ok(properties.mass())
}

fn assert_positive_face_areas(
    source_name: &str,
    shape: &martensite_tkbrep::topods::Shape,
) -> Result<(), Failure> {
    for face in Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)? {
        let area = face_area(&Face::unchecked(face))?;
        assert!(
            area > 0.0,
            "{source_name}: result face has non-positive area"
        );
    }
    Ok(())
}

fn face_area(face: &Face) -> Result<f64, Failure> {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(&face.clone().into_shape(), &mut properties, false, false)?;
    Ok(properties.mass())
}

fn offset_case(source_name: &str, distance: f64, expected_volume: f64) -> Result<(), Failure> {
    let source = BRepPrimAPIMakeCylinder::new(5.0, 15.0).shape()?.clone();
    let mut offset = BRepOffsetMakeOffset::new();
    offset.initialize(
        &source,
        distance,
        1.0e-7,
        MakeOffsetOptions {
            mode: BRepOffsetMode::Skin,
            intersection: true,
            self_inter: false,
            join: JoinType::Intersection,
            thickening: false,
            remove_int_edges: false,
        },
    );
    offset.make_offset_shape(&ProgressRange::default())?;
    assert!(offset.is_done(), "{source_name}: offsetperform completion");
    let result = offset.shape();
    assert!(BRepCheckAnalyzer::new(result, true, false, false)?.is_valid()?);
    assert_positive_face_areas(source_name, result)?;
    assert!(
        (volume(result)? - expected_volume).abs() <= 1.0e-2,
        "{source_name}: checkprops result -v {expected_volume}"
    );
    Ok(())
}

#[test]
#[allow(non_snake_case)]
fn OffsetShapeTypeI__A1() -> Result<(), Failure> {
    offset_case("tests/offset/shape_type_i/A1", 1.0, 1922.65)
}

#[test]
#[allow(non_snake_case)]
fn OffsetShapeTypeI__A2() -> Result<(), Failure> {
    offset_case("tests/offset/shape_type_i/A2", -1.0, 653.451)
}
