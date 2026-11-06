//! BRepFillShapeLaw and BRepFillNSections: section laws of wires.
use martensite_tkbool::brep_fill::{BRepFillNSections, BRepFillSectionLaw, BRepFillShapeLaw};
use martensite_tkbrep::topods::{self, Shape, Wire};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakePolygon;

/// The closed square of half side `half` at height `z`.
fn square(z: f64, half: f64) -> Result<Wire, Failure> {
    let mut maker = BRepBuilderAPIMakePolygon::new();
    for (x, y) in [(-half, -half), (half, -half), (half, half), (-half, half)] {
        maker.add_point(Point3::new(x, y, z))?;
    }
    maker.close()?;
    maker.wire()
}

#[test]
fn a_square_law_has_one_law_per_edge_and_c0_corners() -> Result<(), Failure> {
    let wire = square(0., 1.)?;
    let law = BRepFillShapeLaw::with_wire(&wire, true)?;
    assert!(law.is_done() && law.is_constant() && !law.is_vertex());
    assert!(law.is_u_closed() && law.is_v_closed());
    assert_eq!(law.nb_law()?, 4);
    assert_eq!(law.continuity(1, 1e-2)?, Continuity::C0);
    assert_eq!(
        law.vertex_tol(0, 0.)?,
        martensite_tkernel::precision::confusion()
    );
    let first = law.edge(1)?;
    assert_eq!(law.index_of_edge(&first)?, 1);
    law.init(&wire)?;
    assert!(law.current_edge()?.is_same(&first));
    Ok(())
}

#[test]
fn nsections_through_two_squares_give_four_laws() -> Result<(), Failure> {
    let sections: Vec<Shape> = vec![square(0., 1.)?.into_shape(), square(1., 0.5)?.into_shape()];
    let law = BRepFillNSections::new(&sections, true)?;
    assert!(law.is_done() && !law.is_constant());
    assert_eq!(law.nb_law()?, 4);
    let section = topods::wire(&law.d0(0.5)?)?;
    assert!(!section.is_null());
    Ok(())
}
