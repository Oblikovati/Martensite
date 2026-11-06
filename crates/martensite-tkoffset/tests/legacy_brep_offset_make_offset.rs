//! The cases of `BRepOffsetMakeOffset` (the planar
//! wire offsets; the thick-solid cases are in legacy_brep_offset_make_offset_thick_solid.rs).
use martensite_tkbrep::topods::Wire;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::geom_abs::JoinType;
use martensite_tkmath::gp::{Axis2, Circle3, Direction3, Point3};
use martensite_tkoffset::brep_offset_api::BRepOffsetAPIMakeOffset;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire,
};

/// `MakeCircularWire(center, normal, radius)`.
fn make_circular_wire(center: Point3, normal: Direction3, radius: f64) -> Result<Wire, Failure> {
    let circle = Circle3::new(Axis2::new(center, normal)?, radius)?;
    let edge_maker = BRepBuilderAPIMakeEdge::from_circ(circle)?;
    if !edge_maker.is_done() {
        return Ok(Wire::default());
    }
    let wire_maker = BRepBuilderAPIMakeWire::from_edge(&edge_maker.edge()?)?;
    if !wire_maker.is_done() {
        return Ok(Wire::default());
    }
    wire_maker.wire()
}

/// `IsCircularWireOffsetDone(diameter, offset)`.
fn is_circular_wire_offset_done(diameter: f64, offset: f64) -> Result<bool, Failure> {
    let normal = Direction3::new(0.0, 0.0, 1.0)?;
    let wire = make_circular_wire(Point3::new(0.0, 0.0, 0.0), normal, diameter / 2.0)?;
    if wire.is_null() {
        return Ok(false);
    }
    let mut offset_maker = BRepOffsetAPIMakeOffset::new();
    offset_maker.init(JoinType::Intersection, false);
    offset_maker.add_wire(&wire);
    offset_maker.perform(offset, 0.0)?;
    Ok(offset_maker.is_done())
}

/// The offset of the 1-degree negative taper of a 3-long pad.
fn negative_taper_offset() -> f64 {
    let pad_length = 3.0;
    let taper_degrees: f64 = -1.0;
    martensite_cmath::tan(taper_degrees * std::f64::consts::PI / 180.0) * pad_length
}

#[test]
fn negative_taper_offset_on_small_circular_wire() -> Result<(), Failure> {
    assert!(is_circular_wire_offset_done(2.0, negative_taper_offset())?);
    Ok(())
}

#[test]
fn negative_taper_offset_above_small_circle_boundary() -> Result<(), Failure> {
    assert!(is_circular_wire_offset_done(2.01, negative_taper_offset())?);
    Ok(())
}

#[test]
fn negative_taper_offset_below_small_circle_boundary() -> Result<(), Failure> {
    assert!(is_circular_wire_offset_done(1.0, negative_taper_offset())?);
    Ok(())
}

#[test]
fn positive_offset_on_small_circular_wire() -> Result<(), Failure> {
    assert!(is_circular_wire_offset_done(2.0, 0.05)?);
    Ok(())
}

#[test]
fn collapsed_circular_wire_offset_is_not_done() -> Result<(), Failure> {
    assert!(!is_circular_wire_offset_done(2.0, -1.0)?);
    Ok(())
}
