//! Independent closed-form checks of BRepOffsetAPIMakeThickSolid.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods;
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
use martensite_tktopalgo::brep_gprop::BRepGProp;

/// The inward offset of the box of side 20 by -1 through its first face keeps sharp
/// inner corners (the concave edges are intersected), so the cavity is the box
/// 18 x 18 x 19 and the wall volume is 20^3 - 18 * 18 * 19 = 1844.
#[test]
fn hollow_box_wall_volume_is_the_closed_form() -> Result<(), Failure> {
    let a_box = BRepPrimAPIMakeBox::from_sizes(20.0, 20.0, 20.0)?
        .shape()?
        .clone();
    let face = Explorer::new(&a_box, ShapeEnum::Face, ShapeEnum::Shape)?
        .current()
        .clone();
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
    let result = maker.shape()?.clone();
    let mut props = GProps::new();
    BRepGProp::volume_properties(&result, &mut props, false, false, false)?;
    let wall = props.mass();
    assert!(
        (wall - 1844.0).abs() < 1.0e-6 * 1844.0,
        "wall volume {wall}"
    );
    Ok(())
}

#[test]
fn hollow_box_must_not_widen_entity_tolerances() -> Result<(), Failure> {
    let source = BRepPrimAPIMakeBox::from_sizes(20., 20., 20.)?
        .shape()?
        .clone();
    let opening = Explorer::new(&source, ShapeEnum::Face, ShapeEnum::Shape)?
        .current()
        .clone();
    let mut removed = List::default();
    removed.append(opening);
    let mut maker = BRepOffsetAPIMakeThickSolid::new();
    maker.make_thick_solid_by_join(
        &source,
        &removed,
        -1.,
        1e-7,
        MakeOffsetOptions::default(),
        &ProgressRange::default(),
    )?;
    let body = maker.shape()?.clone();
    let max_vertex = |shape: &topods::Shape| -> Result<f64, Failure> {
        let mut explorer = Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape)?;
        let mut maximum: f64 = 0.;
        while explorer.more() {
            let v = topods::vertex(explorer.current())?;
            maximum = maximum.max(BRepTool::vertex_tolerance(&v)?);
            explorer.next()?;
        }
        Ok(maximum)
    };
    let before = max_vertex(&source)?;
    let after = max_vertex(&body)?;
    assert!(
        after <= before,
        "shell vertex tolerance widened from {before:.17e} to {after:.17e}"
    );
    Ok(())
}
