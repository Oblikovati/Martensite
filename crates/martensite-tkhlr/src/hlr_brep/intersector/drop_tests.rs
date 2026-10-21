use super::{Intersector, ThePolyhedronOfInterCSurf};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{
    geom::{GeomPlane, GeomSurface},
    geom_adaptor::GeomAdaptorSurface,
};
use martensite_tkmath::gp::{Axis3, Point3};

fn populated_intersector() -> Intersector {
    let surface: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(Axis3::default())).cast();
    let adaptor = GeomAdaptorSurface::with_bounds(&surface, (-1., 1., -1., 1.)).unwrap();
    let grid = ThePolyhedronOfInterCSurf::on_adaptor(&adaptor, (3, 3), [-1., -1., 1., 1.]).unwrap();
    let mut intersector = Intersector::new();
    intersector.polyhedron = Some(grid);
    intersector
}
#[test]
fn explicit_intersector_drop_uses_owned_grid_and_idempotent_destroy_path() {
    #[allow(
        drop_bounds,
        reason = "requires an explicit destructor body, not only owned fields"
    )]
    fn explicit_drop<T: Drop>() {}
    explicit_drop::<Intersector>();
    explicit_drop::<ThePolyhedronOfInterCSurf>();
    let mut intersector = populated_intersector();
    let snapshot = intersector.polyhedron.as_ref().unwrap().clone();
    intersector.destroy();
    intersector.destroy();
    assert!(intersector.polyhedron.is_none());
    drop(intersector);
    assert_eq!(snapshot.point(1).unwrap(), Point3::new(-1., -1., 0.));
}
#[test]
fn populated_intersector_drop_and_load_preserve_independent_grid_snapshot() {
    let mut intersector = populated_intersector();
    let snapshot = intersector.polyhedron.as_ref().unwrap().clone();
    intersector.load();
    assert!(intersector.polyhedron.is_none());
    drop(intersector);
    let intersector = populated_intersector();
    drop(intersector);
    assert_eq!(snapshot.point(1).unwrap(), Point3::new(-1., -1., 0.));
}
