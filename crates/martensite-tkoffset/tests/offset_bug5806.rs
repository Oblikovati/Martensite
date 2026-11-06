//! Reference replay of the bug5806 hollowed pipe regression.

use martensite_tkbrep::{
    topexp::Explorer,
    topods::{Shape, Wire},
};
use martensite_tkernel::{message::ProgressRange, ncollection::List, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis2, Circle3, Direction3, Point3};
use martensite_tkoffset::{
    brep_offset::MakeOffsetOptions,
    brep_offset_api::{BRepOffsetAPIMakeThickSolid, BRepOffsetAPIThruSections},
};
use martensite_tktopalgo::{
    brep_builder_api::{
        BRepBuilderAPIMakeEdge, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
        BRepBuilderAPIMakeWire,
    },
    brep_check::BRepCheckAnalyzer,
    brep_gprop::BRepGProp,
};

fn circular_wire() -> Result<Wire, Failure> {
    let axis = Axis2::new(Point3::new(0., 100., 0.), Direction3::new(1., 0., 0.)?)?;
    let edge = BRepBuilderAPIMakeEdge::from_circ(Circle3::new(axis, 5.)?)?.edge()?;
    BRepBuilderAPIMakeWire::from_edge(&edge)?.wire()
}

fn polygon_wire() -> Result<Wire, Failure> {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for point in [
        Point3::new(10., 90., -10.),
        Point3::new(10., 90., 10.),
        Point3::new(10., 110., 10.),
        Point3::new(10., 110., -10.),
    ] {
        polygon.add_point(point)?;
    }
    polygon.close()?;
    polygon.wire()
}

fn loft() -> Result<Shape, Failure> {
    let mut maker = BRepOffsetAPIThruSections::new(true, true, 1.0e-6);
    maker.add_wire(&circular_wire()?);
    maker.add_wire(&polygon_wire()?);
    maker.build(&ProgressRange::default())?;
    maker.shape().cloned()
}

fn removed_faces(shape: &Shape) -> Result<List<Shape>, Failure> {
    let faces: Vec<_> = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?.collect();
    let mut removed = List::default();
    removed.append(faces[5].clone());
    removed.append(faces[6].clone());
    Ok(removed)
}

#[test]
#[allow(non_snake_case)]
fn bug5806() -> Result<(), Failure> {
    let source = loft()?;
    let mut maker = BRepOffsetAPIMakeThickSolid::new();
    maker.make_thick_solid_by_join(
        &source,
        &removed_faces(&source)?,
        -0.5,
        1.0e-7,
        MakeOffsetOptions::default(),
        &ProgressRange::default(),
    )?;
    maker.build(&ProgressRange::default())?;
    if !maker.is_done() {
        return Ok(());
    }
    let result = maker.shape()?.clone();
    assert!(BRepCheckAnalyzer::new(&result, true, false, false)?.is_valid()?);
    let mut props = GProps::default();
    BRepGProp::surface_properties(&result, &mut props, false, false)?;
    assert!((props.mass() - 1116.06).abs() / 1116.06 <= 0.01);
    Ok(())
}
