//! The first `GeomFill_BSplineCurves` fill regression of `bug28131`.
#[path = "support/occ28131.rs"]
mod fixture;
use fixture::occ28131_surface;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::GeomSurface;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

#[test]
fn geom_fill_bspline_curves_occ28131_fill_surface_from_bezier_and_interpolated_curves() {
    let surface = occ28131_surface().unwrap();
    let geometry: Handle<dyn GeomSurface> = surface.cast();
    let mut maker = BRepBuilderAPIMakeFace::from_surface(&geometry, 0.0).unwrap();
    let shape = if maker.is_done() {
        maker.shape().unwrap().clone()
    } else {
        martensite_tkbrep::topods::Shape::default()
    };
    assert!(!shape.is_null());
    assert!(
        BRepCheckAnalyzer::new(&shape, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
}
