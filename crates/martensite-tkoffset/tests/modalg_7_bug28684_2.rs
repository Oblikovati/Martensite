//! Reference replay of bug25298: a circular planar-wire offset preserves its expected result.

use martensite_tkbrep::topods::Wire;
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::{
    geom_abs::JoinType,
    gp::{Axis2, Circle3, Direction3, Point3},
};
use martensite_tkoffset::brep_offset_api::BRepOffsetAPIMakeOffset;
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire},
    brep_check::BRepCheckAnalyzer,
    brep_gprop::BRepGProp,
};

fn source_wire() -> Result<Wire, Failure> {
    let circle = Circle3::new(
        Axis2::new(Point3::default(), Direction3::new(0.0, 0.0, 1.0)?)?,
        100.0,
    )?;
    let edge = BRepBuilderAPIMakeEdge::from_circ(circle)?.edge()?;
    BRepBuilderAPIMakeWire::from_edge(&edge)?.wire()
}

/// `tests/bugs/modalg_7/bug28684_2`: inward offset of a 100-radius circle.
#[test]
#[allow(non_snake_case)]
fn Modalg7__Bug28684_2() -> Result<(), Failure> {
    let wire = source_wire()?;
    let mut maker = BRepOffsetAPIMakeOffset::from_wire(&wire, JoinType::Arc, false);
    maker.perform(-10.0, 0.0)?;
    assert!(maker.is_done(), "25298: mkoffset did not complete");
    let result = maker.shape()?;
    assert!(BRepCheckAnalyzer::new(result, true, false, false)?.is_valid()?);
    let mut props = GProps::default();
    BRepGProp::linear_properties(result, &mut props, false, false)?;
    let expected = 565.487;
    assert!(
        (props.mass() - expected).abs() / expected <= 0.01,
        "25298: length {}, expected {expected}",
        props.mass()
    );
    Ok(())
}
