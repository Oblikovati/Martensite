//! Reference replay of bug32964: an open right-angle wire remains two offset edges.

use martensite_tkbrep::{
    topexp,
    topods::{Shape, Wire},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::{geom_abs::JoinType, gp::Point3};
use martensite_tkoffset::brep_offset_api::BRepOffsetAPIMakeOffset;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire,
};

fn count(shape: &Shape, kind: ShapeEnum) -> Result<usize, Failure> {
    let mut found = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, kind, &mut found)?;
    Ok(found.extent())
}

fn source_wire() -> Result<Wire, Failure> {
    let points = [
        Point3::new(-90.0, 40.0, 0.0),
        Point3::new(40.0, 40.0, 0.0),
        Point3::new(40.0, -90.0, 0.0),
    ];
    let mut maker = BRepBuilderAPIMakeWire::new();
    for pair in points.windows(2) {
        maker.add_edge(&BRepBuilderAPIMakeEdge::from_points(pair[0], pair[1])?.edge()?)?;
    }
    maker.wire()
}

/// `tests/bugs/modalg_8/bug32964`: 2D open offset must not split straight edges.
#[test]
#[allow(non_snake_case)]
fn Modalg8__Bug32964() -> Result<(), Failure> {
    let wire = source_wire()?;
    let mut maker = BRepOffsetAPIMakeOffset::from_wire(&wire, JoinType::Arc, true);
    maker.perform(1.0, 40.0)?;
    assert!(maker.is_done(), "32964: open offset did not complete");
    let result = maker.shape()?.clone();
    assert_eq!(count(&result, ShapeEnum::Vertex)?, 3);
    assert_eq!(count(&result, ShapeEnum::Edge)?, 2);
    assert_eq!(count(&result, ShapeEnum::Wire)?, 1);
    assert_eq!(
        count(&result, ShapeEnum::Vertex)?
            + count(&result, ShapeEnum::Edge)?
            + count(&result, ShapeEnum::Wire)?,
        6
    );
    Ok(())
}
