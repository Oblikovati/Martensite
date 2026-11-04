//! Closed forms pin finite-range doubling before any surface engine is moved.
use super::finite_bounds;
#[path = "geometry.rs"]
mod geometry;
use martensite_tkernel::precision;
#[test]
fn plane_bound_doubling_preserves_symmetric_and_one_sided_distances() {
    let finite = geometry::fixture(1);
    assert_eq!(finite.get().unwrap().bounds(), (-5., 5., -5., 5.));
    let handle = geometry::plane();
    let plane = handle.get().unwrap();
    let inf = precision::infinite();
    assert_eq!(
        finite_bounds(plane, (-5., 5., -3., 3.)).unwrap(),
        (-5., 5., -3., 3.)
    );
    assert_eq!(
        finite_bounds(plane, (-inf, inf, -inf, inf)).unwrap(),
        (-256., 256., -256., 256.)
    );
    assert_one_sided_bounds(plane, inf);
}
fn assert_one_sided_bounds(plane: &dyn martensite_tkg3d::geom::GeomSurface, inf: f64) {
    assert_eq!(
        finite_bounds(plane, (2., inf, -3., 3.)).unwrap(),
        (2., 514., -3., 3.)
    );
    assert_eq!(
        finite_bounds(plane, (-inf, 2., -3., 3.)).unwrap(),
        (-510., 2., -3., 3.)
    );
    assert_eq!(
        finite_bounds(plane, (-5., 5., 2., inf)).unwrap(),
        (-5., 5., 2., 514.)
    );
    assert_eq!(
        finite_bounds(plane, (-5., 5., -inf, 2.)).unwrap(),
        (-5., 5., -510., 2.)
    );
}
#[test]
fn finite_surface_bounds_preserve_declared_nan_without_evaluation() {
    let handle = geometry::plane();
    let plane = handle.get().unwrap();
    let bounds = finite_bounds(plane, (f64::NAN, 5., -3., 3.)).unwrap();
    assert!(bounds.0.is_nan());
    assert_eq!((bounds.1, bounds.2, bounds.3), (5., -3., 3.));
}
