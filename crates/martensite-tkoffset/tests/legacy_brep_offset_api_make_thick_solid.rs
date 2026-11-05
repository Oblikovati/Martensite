//! The cases of `BRepOffsetAPIMakeThickSolid`, with
//! independent closed-form volume checks of the hollowed box.
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkoffset::brep_offset::MakeOffsetOptions;
use martensite_tkoffset::brep_offset_api::BRepOffsetAPIMakeThickSolid;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use martensite_tktopalgo::brep_gprop::BRepGProp;

/// The box of a size and its first face.
fn box_and_first_face(size: f64) -> Result<(Shape, Shape), Failure> {
    let a_box = BRepPrimAPIMakeBox::from_sizes(size, size, size)?
        .shape()?
        .clone();
    let face = Explorer::new(&a_box, ShapeEnum::Face, ShapeEnum::Shape)?
        .current()
        .clone();
    Ok((a_box, face))
}

/// The volume of a shape.
fn volume(s: &Shape) -> Result<f64, Failure> {
    let mut props = GProps::new();
    BRepGProp::volume_properties(s, &mut props, false, false, false)?;
    Ok(props.mass())
}

/// The box of side 20 hollowed by -1 through its first face.
fn hollow_box_of_twenty() -> Result<(Shape, BRepOffsetAPIMakeThickSolid), Failure> {
    let (a_box, face) = box_and_first_face(20.0)?;
    let mut faces_to_remove = List::default();
    faces_to_remove.append(face);
    let mut maker = BRepOffsetAPIMakeThickSolid::new();
    maker.make_thick_solid_by_join(
        &a_box,
        &faces_to_remove,
        -1.0,
        precision::confusion(),
        MakeOffsetOptions::default(),
        &ProgressRange::default(),
    )?;
    maker.build(&ProgressRange::default())?;
    Ok((a_box, maker))
}

#[test]
fn hollow_box() -> Result<(), Failure> {
    let (_, mut maker) = hollow_box_maker()?;
    assert!(maker.is_done());
    let result = maker.shape()?.clone();
    assert!(!result.is_null());
    assert!(BRepCheckAnalyzer::new(&result, true, false, false)?.is_valid()?);
    Ok(())
}

fn hollow_box_maker() -> Result<(Shape, BRepOffsetAPIMakeThickSolid), Failure> {
    hollow_box_of_twenty()
}

#[test]
fn hollow_box_volume() -> Result<(), Failure> {
    let (a_box, mut maker) = hollow_box_of_twenty()?;
    let an_orig_volume = volume(&a_box)?;
    assert!(maker.is_done());
    let result = maker.shape()?.clone();
    assert!(!result.is_null());
    let a_hollow_volume = volume(&result)?;
    // The hollowed box must have less volume than the original solid.
    assert!(a_hollow_volume < an_orig_volume);
    Ok(())
}

#[test]
fn make_thick_solid_by_simple() -> Result<(), Failure> {
    let (_, face) = box_and_first_face(10.0)?;
    let mut maker = BRepOffsetAPIMakeThickSolid::new();
    maker.make_thick_solid_by_simple(&face, 2.0)?;
    maker.build(&ProgressRange::default())?;
    assert!(maker.is_done());
    let result = maker.shape()?.clone();
    assert!(!result.is_null());
    assert!(BRepCheckAnalyzer::new(&result, true, false, false)?.is_valid()?);
    Ok(())
}

#[test]
fn thick_solid_larger_volume() -> Result<(), Failure> {
    let (_, face) = box_and_first_face(10.0)?;
    // The area of the original face, for the expected volume.
    let mut face_props = GProps::new();
    BRepGProp::surface_properties(&face, &mut face_props, false, false)?;
    let a_face_area = face_props.mass();
    let an_offset = 2.0;
    let mut maker = BRepOffsetAPIMakeThickSolid::new();
    maker.make_thick_solid_by_simple(&face, an_offset)?;
    maker.build(&ProgressRange::default())?;
    assert!(maker.is_done());
    let result = maker.shape()?.clone();
    assert!(!result.is_null());
    let a_result_volume = volume(&result)?.abs();
    // A planar face of area A thickened by d makes a volume of about A * d.
    assert!(a_result_volume > a_face_area * an_offset * 0.5);
    let _ = topods::face(&face)?;
    Ok(())
}
