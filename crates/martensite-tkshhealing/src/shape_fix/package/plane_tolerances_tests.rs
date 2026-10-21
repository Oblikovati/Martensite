use super::plane_tolerances::update_edge;
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, ShapeIterator},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{
    adaptor3d::Adaptor3dSurface,
    geom::{GeomPlane, GeomSurface},
    geom_adaptor::GeomAdaptorSurface,
};
use martensite_tkmath::gp::{Axis3, Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace};

#[test]
fn planar_phase_grows_edge_and_both_endpoint_tolerances() {
    let surface: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(Axis3::default())).cast();
    let adaptor: Handle<dyn Adaptor3dSurface> =
        Handle::new(GeomAdaptorSurface::with_surface(&surface).unwrap()).cast();
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0., 1., 0., 1.))
        .unwrap()
        .face()
        .unwrap();
    let edge =
        BRepBuilderAPIMakeEdge::from_points(Point3::new(0., 0., 1.), Point3::new(1., 0., 1.))
            .unwrap()
            .edge()
            .unwrap();
    update_edge(&edge, &face, &adaptor).unwrap();
    assert_eq!(BRepTool::edge_tolerance(&edge).unwrap(), 1.00005);
    assert_endpoint_tolerances(&edge);
}
fn assert_endpoint_tolerances(edge: &martensite_tkbrep::topods::Edge) {
    let mut vertices = ShapeIterator::new(edge, true, true);
    while vertices.more() {
        let vertex = topods::vertex(vertices.value().unwrap()).unwrap();
        assert_eq!(BRepTool::vertex_tolerance(&vertex).unwrap(), 1.00005);
        vertices.next();
    }
}
