use super::ThePolyhedronOfHInter;
use martensite_tkernel::standard::{Handle, WeakHandle};
use martensite_tkg3d::{
    geom::{GeomPlane, GeomSurface},
    geom_adaptor::GeomAdaptorSurface,
};
use martensite_tkmath::gp::{Axis3, Point3};

fn detached_grid() -> (ThePolyhedronOfHInter, WeakHandle<dyn GeomSurface>) {
    let mut frame = Axis3::default();
    frame.set_location(Point3::new(0., 0., 3.));
    let geometry: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(frame)).cast();
    let weak = geometry.downgrade();
    let adaptor = GeomAdaptorSurface::with_bounds(&geometry, (-1., 1., -1., 1.)).unwrap();
    let grid = ThePolyhedronOfHInter::on_adaptor(&adaptor, (3, 3), [-1., -1., 1., 1.]).unwrap();
    (grid, weak)
}

#[test]
fn sampled_grid_owns_coordinates_after_source_geometry_is_released() {
    let (grid, source) = detached_grid();
    assert!(source.upgrade().is_null());
    assert_eq!(grid.point(1).unwrap(), Point3::new(-1., -1., 3.));
    #[allow(
        drop_bounds,
        reason = "requires an explicit destructor body, not only owned fields"
    )]
    fn explicit_drop<T: Drop>() {}
    explicit_drop::<ThePolyhedronOfHInter>();
    drop(grid);
    assert!(source.upgrade().is_null());
}

#[test]
fn destroying_one_grid_releases_all_four_buffers_without_touching_its_clone() {
    let (mut grid, _) = detached_grid();
    let copy = grid.clone();
    assert_ne!(grid.grid.pnts.as_ptr(), copy.grid.pnts.as_ptr());
    grid.destroy();
    grid.destroy();
    assert_eq!(grid.grid.pnts.capacity(), 0);
    assert_eq!(grid.grid.u.capacity(), 0);
    assert_eq!(grid.grid.v.capacity(), 0);
    assert_eq!(grid.grid.is_on_bounds.capacity(), 0);
    assert_eq!(copy.point(1).unwrap(), Point3::new(-1., -1., 3.));
    drop(grid);
    assert_eq!(copy.point(1).unwrap(), Point3::new(-1., -1., 3.));
}

#[test]
fn grid_drop_during_unwind_preserves_independent_clone_coordinates() {
    let (grid, _) = detached_grid();
    let copy = grid.clone();
    let unwound = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let _owned_until_unwind = grid;
        panic!("owned grid unwind exercise");
    }));
    assert!(unwound.is_err());
    assert_eq!(copy.point(1).unwrap(), Point3::new(-1., -1., 3.));
}
