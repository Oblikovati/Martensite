//! BRepFillSection and BRepFillSectionPlacement.
use martensite_tkbool::brep_fill::{
    BRepFillEdge3DLaw, BRepFillLocationLaw, BRepFillSection, BRepFillSectionPlacement,
};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Shape, Vertex};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkgeomalgo::geom_fill::{
    GeomFillCorrectedFrenet, GeomFillCurveAndTrihedron, GeomFillLocationLaw, GeomFillTrihedronLaw,
};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakePolygon;

#[test]
fn a_vertex_section_is_a_punctual_wire() -> Result<(), Failure> {
    let mut v = Vertex::default();
    BRepBuilder::default().make_vertex_at(&mut v, Point3::new(1., 2., 3.), 1e-7)?;
    let section =
        BRepFillSection::with_profile(&v.clone().into_shape(), &Vertex::default(), true, false)?;
    assert!(section.is_punctual() && section.with_contact() && !section.with_correction());
    assert!(section.wire().closed()?);
    assert!(!section.modified_shape(&v.into_shape())?.is_null());
    Ok(())
}

#[test]
fn a_section_at_the_start_of_a_straight_path_needs_no_move() -> Result<(), Failure> {
    let mut path = BRepBuilderAPIMakePolygon::new();
    path.add_point(Point3::new(0., 0., 0.))?;
    path.add_point(Point3::new(0., 0., 3.))?;
    let path = path.wire()?;
    let mut square = BRepBuilderAPIMakePolygon::new();
    for (x, y) in [(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5), (-0.5, 0.5)] {
        square.add_point(Point3::new(x, y, 0.))?;
    }
    square.close()?;
    let section: Shape = square.wire()?.into_shape();
    let tri: Handle<dyn GeomFillTrihedronLaw> = Handle::new(GeomFillCorrectedFrenet::new()).cast();
    let model: Handle<dyn GeomFillLocationLaw> =
        Handle::new(GeomFillCurveAndTrihedron::new(&tri)).cast();
    let law: Handle<dyn BRepFillLocationLaw> =
        Handle::new(BRepFillEdge3DLaw::new(&path, &model)?).cast();
    let place = BRepFillSectionPlacement::new(&law, &section, false, false)?;
    assert!(place.abscissa_on_path()?.abs() < 1e-9);
    Ok(())
}
