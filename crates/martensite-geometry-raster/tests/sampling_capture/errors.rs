//! Unloaded geometry must preserve its typed lower-level failure in either mode.
use super::{Geometry, SamplingDriver, pixels::live_viewer};
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkg3d::{adaptor3d::Adaptor3dIsoCurve, geom_adaptor::GeomAdaptorCurve};
use martensite_tkmath::geom_abs::IsoType;
pub fn unloaded_geometry_failures<S: SamplingDriver>() {
    for mode in [0, 1] {
        assert_failure::<S>(
            Geometry::Spatial(GeomAdaptorCurve::default()),
            mode,
            "GeomAdaptor_Curve: no curve is loaded; expected Load() first",
        );
        assert_failure::<S>(
            Geometry::Planar(Box::<Geom2dAdaptorCurve>::default()),
            mode,
            "Geom2dAdaptor_Curve: no curve is loaded; expected Load() first",
        );
        assert_failure::<S>(
            Geometry::Iso(Adaptor3dIsoCurve::default(), IsoType::IsoU, 0., (0., 1.)),
            mode,
            "Adaptor3d_IsoCurve::Load: no surface is loaded",
        );
    }
}
fn assert_failure<S: SamplingDriver>(geometry: Geometry, mode: i32, expected: &str) {
    let mut viewer = live_viewer(geometry.spatial());
    let mut display = viewer.make_display(1);
    let error = S::configured(mode)
        .draw_geometry(&geometry, &mut display)
        .unwrap_err();
    assert_eq!(error.exception_type(), "Standard_NullObject");
    assert_eq!(error.message(), expected);
    display.flush();
    viewer.delete_view(1);
}
