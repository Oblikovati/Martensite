//! Actual retained-view images pin pole order, control changes and live geometry.
use super::{
    bezier_api::{self, Planar, Spatial},
    planar, spatial,
};
use martensite_raster_core::{PaletteColor, PaletteKind};
use std::rc::Rc;
#[test]
fn bezier_native_matrix_keeps_modes_flags_curvature_and_live_edits() {
    for mode in [0, 1] {
        for shown in [false, true] {
            capture_pair(mode, shown);
        }
    }
    mutate_pair();
}
fn capture_pair(mode: i32, shown: bool) {
    let colors = colors();
    let space = Rc::new(Spatial::with_style(
        spatial(),
        colors,
        shown,
        (30, 0.01, mode),
    ));
    let plane = Rc::new(Planar::with_style(planar(), colors, shown, 30));
    bezier_api::set_plane_mode(&plane, mode);
    let label = format!("mode{mode}-poles{shown}");
    capture(space, plane, &label);
}
fn mutate_pair() {
    let space = Rc::new(Spatial::with_style(
        spatial(),
        colors(),
        true,
        (30, 0.01, 0),
    ));
    let plane = Rc::new(Planar::with_style(planar(), colors(), true, 30));
    capture(space.clone(), plane.clone(), "before");
    configure(space.curve_part().style(), space.poles());
    configure(plane.curve_part().style(), plane.poles());
    capture(space.clone(), plane.clone(), "controls");
    mutate_geometry(&space, &plane);
    capture(space, plane, "geometry");
}
fn mutate_geometry(space: &Spatial, plane: &Planar) {
    use martensite_tkg2d::geom2d::Geom2dBezierCurve;
    use martensite_tkg3d::geom::GeomBezierCurve;
    use martensite_tkmath::gp::{Point2, Point3};
    let handle = space.curve_part().curve().cast::<GeomBezierCurve>();
    handle
        .get()
        .unwrap()
        .set_pole(2, Point3::new(0., 8., 0.))
        .unwrap();
    let handle = plane.curve_part().curve().cast::<Geom2dBezierCurve>();
    handle
        .get()
        .unwrap()
        .set_pole(2, Point2::new(0., 8.))
        .unwrap();
}
fn configure(
    style: &martensite_geometry_raster::CurveStyle,
    poles: &martensite_geometry_raster::PolesStyle,
) {
    style.set_display_origin(false);
    style.show_curvature();
    style.set_radius_ratio(0.2);
    style.set_radius_max(1.);
    poles.set_color(PaletteColor::new(PaletteKind::Bleu));
}
fn colors() -> (PaletteColor, PaletteColor) {
    (
        PaletteColor::new(PaletteKind::Jaune),
        PaletteColor::new(PaletteKind::Rouge),
    )
}
fn capture(space: Rc<Spatial>, plane: Rc<Planar>, label: &str) {
    capture_one(
        bezier_api::space_handle(space),
        true,
        &format!("bezier-space-{label}"),
    );
    capture_one(
        bezier_api::plane_handle(plane),
        false,
        &format!("bezier-plane-{label}"),
    );
}
fn capture_one(handle: bezier_api::DrawableHandle, spatial: bool, label: &str) {
    let mut viewer = bezier_api::viewer(spatial);
    bezier_api::reset_visibility(&handle);
    viewer.display(&handle).unwrap();
    assert!(Rc::ptr_eq(&viewer.drawables()[0], &handle));
    viewer.repaint_view(1).unwrap();
    let bytes = save_capture(&mut viewer, label);
    assert_capture(&bytes, label);
    viewer.delete_view(1);
}
fn save_capture(viewer: &mut bezier_api::Viewer, label: &str) -> Vec<u8> {
    let root = std::env::var("MARTENSITE_GEOMETRY_ARTIFACT_DIR")
        .unwrap_or_else(|_| format!("/tmp/bezier-curves-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let path = format!("{root}/{label}.ppm");
    assert!(viewer.save_view(1, &path).unwrap());
    std::fs::read(path).unwrap()
}
fn assert_capture(bytes: &[u8], label: &str) {
    assert!(bytes.starts_with(b"P6\n400 400\n255\n"));
    assert_eq!(bytes.len(), 480015);
    assert!(bytes[15..].windows(3).any(|pixel| pixel == [255, 255, 0]));
    if label.contains("polestrue") || label.ends_with("before") {
        assert_eq!(&bytes[15 + (150 * 400 + 200) * 3..][..3], &[255, 0, 0]);
    }
    if label.contains("polesfalse") {
        assert_ne!(&bytes[15 + (150 * 400 + 200) * 3..][..3], &[255, 0, 0]);
    }
}
#[test]
fn bezier_pick_and_null_failures_keep_boundary_contracts() {
    let space = Spatial::new(spatial());
    let plane = Planar::new(planar());
    let mut viewer = bezier_api::viewer(true);
    {
        let display = viewer.make_display(1);
        assert_eq!(space.find_pole((-50., 0.), &display, 0.1, 0).unwrap(), 1);
        assert_eq!(plane.find_pole((0., 50.), &display, 0.1, 0).unwrap(), 2);
        assert_eq!(space.find_pole((-50., 0.), &display, 0.1, 1).unwrap(), 0);
        assert_eq!(plane.find_pole((999., 999.), &display, 0.1, 0).unwrap(), 0);
        assert_null_pick(&display);
    }
    viewer.delete_view(1);
}

fn assert_null_pick(display: &martensite_raster_core::RasterDisplay<'_>) {
    let error = Spatial::new(Default::default())
        .find_pole((0., 0.), display, 1., 0)
        .unwrap_err();
    assert_eq!(error.exception_type(), "Standard_NullObject");
    assert_eq!(error.message(), "null geometry in a drawable");
    let error = Planar::new(Default::default())
        .find_pole((0., 0.), display, 1., 0)
        .unwrap_err();
    assert_eq!(error.exception_type(), "Standard_NullObject");
}

#[test]
fn null_bezier_display_preserves_failures_with_poles_shown_and_hidden() {
    for shown in [false, true] {
        let space = Rc::new(Spatial::with_style(
            Default::default(),
            colors(),
            shown,
            (30, 0.01, 0),
        ));
        let plane = Rc::new(Planar::with_style(Default::default(), colors(), shown, 30));
        null_display(bezier_api::space_handle(space), true);
        null_display(bezier_api::plane_handle(plane), false);
    }
}
fn null_display(handle: bezier_api::DrawableHandle, spatial: bool) {
    let mut viewer = bezier_api::viewer(spatial);
    let error = viewer.display(&handle).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_NullObject");
    assert_eq!(error.message(), "null geometry in a drawable");
    assert!(Rc::ptr_eq(&viewer.drawables()[0], &handle));
    viewer.delete_view(1);
}
