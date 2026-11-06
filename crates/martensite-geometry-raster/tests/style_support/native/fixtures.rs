//! Circle radii, control triangles, knot diamonds and a two-by-two control net.
use super::{
    super::style_api::{
        CurvatureDisplay, CurveLook, CurveStyle, KnotsStyle, draw_pole_net, draw_polygon,
    },
    Workload,
};
use martensite_raster_core::{MarkerShape, PaletteColor, PaletteKind, RasterDisplay};
use martensite_tkernel::{ncollection::Array1, standard::Failure, storage::IoStream};
use martensite_tkgeombase::{
    geom_lprop::{CLProps, CLProps2d},
    geom_tools,
};
use martensite_tkmath::gp::{Point2, Point3};
fn red() -> PaletteColor {
    PaletteColor::new(PaletteKind::Rouge)
}
pub fn draw(workload: Workload, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
    match workload {
        Workload::Polygon(spatial) => {
            draw_control_polygon(display, spatial);
            Ok(())
        }
        Workload::Knots(spatial, shown) => draw_knots(display, spatial, shown),
        Workload::Net(periodic) => draw_net(display, periodic),
        Workload::Curvature(true, capped) => draw_spatial_radii(display, capped),
        Workload::Curvature(false, capped) => draw_planar_radii(display, capped),
    }
}
fn draw_control_polygon(display: &mut RasterDisplay<'_>, spatial: bool) {
    if spatial {
        return draw_polygon(display, red(), &spatial_poles());
    }
    draw_polygon(display, red(), &planar_poles());
}
fn spatial_poles() -> [Point3; 3] {
    [
        Point3::new(-5., -3., 0.),
        Point3::new(0., 5., 0.),
        Point3::new(5., -3., 0.),
    ]
}
fn planar_poles() -> [Point2; 3] {
    [
        Point2::new(-5., -3.),
        Point2::new(0., 5.),
        Point2::new(5., -3.),
    ]
}
fn draw_net(display: &mut RasterDisplay<'_>, periodic: bool) -> Result<(), Failure> {
    draw_pole_net(display, red(), (2, 2), (periodic, periodic), |u, v| {
        Ok(Point3::new(f64::from(u * 4 - 6), f64::from(v * 4 - 6), 0.))
    })
}
fn draw_knots(display: &mut RasterDisplay<'_>, spatial: bool, shown: bool) -> Result<(), Failure> {
    let knots = KnotsStyle::new(shown, MarkerShape::Losange, red(), 5);
    if spatial {
        return knots.mark(
            display,
            [Ok(Point3::new(-5., 0., 0.)), Ok(Point3::new(5., 0., 0.))],
        );
    }
    knots.mark(display, [Ok(Point2::new(-5., 0.)), Ok(Point2::new(5., 0.))])
}
fn radii(capped: bool) -> CurvatureDisplay {
    let style = CurveStyle::new(CurveLook::new(red()));
    style.set_radius_max(if capped { 1. } else { 1000. });
    style.set_radius_ratio(if capped { 0.5 } else { 0.2 });
    CurvatureDisplay::of(4, &style)
}
fn bounds() -> Array1<'static, f64> {
    Array1::from_shared(&[0., std::f64::consts::TAU], 1)
}
fn draw_spatial_radii(display: &mut RasterDisplay<'_>, capped: bool) -> Result<(), Failure> {
    let curve = geom_tools::read_curve(&mut IoStream::from_bytes(
        b"2 0 0 0 0 0 1 1 0 0 0 1 0 5".to_vec(),
    ));
    let radii = radii(capped);
    let mut props = CLProps::new(curve.clone(), 2, radii.resolution())?;
    radii.draw(display, &bounds(), &mut props, |t| {
        curve.get().unwrap().d0(t)
    })
}
fn draw_planar_radii(display: &mut RasterDisplay<'_>, capped: bool) -> Result<(), Failure> {
    let curve = geom_tools::read_curve2d(&mut IoStream::from_bytes(b"2 0 0 1 0 0 1 5".to_vec()));
    let radii = radii(capped);
    let mut props = CLProps2d::new(curve.clone(), 2, radii.resolution())?;
    radii.draw(display, &bounds(), &mut props, |t| {
        curve.get().unwrap().d0(t)
    })
}
