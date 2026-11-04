//! Conversion commands execute in source order, including unused ray origins.
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkernel::standard::Failure;
use martensite_tkv3d::v3d::V3dView;
type Point = (f64, f64, f64);
pub(super) struct Base {
    pub scalar_view: f64,
    pub scalar_pixel: i32,
    pub origin_pixel: (i32, i32),
    pub center_view: (f64, f64),
    pub ray: [f64; 6],
    pub center_world: Point,
}
pub(super) struct Observed {
    pub base: Base,
    pub grid_world: Option<Point>,
    pub point_pixel: (i32, i32),
    pub point_grid: Option<Point>,
}
fn base(session: &ViewerSession) -> Result<Base, Failure> {
    let view = session.view().view_base();
    Ok(Base {
        scalar_view: view.convert_pixel_to_view(1)?,
        scalar_pixel: view.convert_view_to_pixel(1.)?,
        origin_pixel: view.convert_view_to_pixels(0., 0.)?,
        center_view: view.convert_to_view(200, 200)?,
        ray: view.convert_with_proj(0, 0)?,
        center_world: view.convert_to_world(200, 200)?,
    })
}
pub(super) fn observe(
    session: &ViewerSession,
    point: [f64; 3],
    grid: bool,
) -> Result<Observed, Failure> {
    let base = base(session)?;
    let view = session.view().view_base();
    let grid_world = grid_point(view, base.center_world, grid)?;
    let point_pixel = view.convert_world_to_pixels(point[0], point[1], point[2])?;
    let point_grid = grid_point(view, (point[0], point[1], point[2]), grid)?;
    Ok(Observed {
        base,
        grid_world,
        point_pixel,
        point_grid,
    })
}
fn grid_point(view: &V3dView, (x, y, z): Point, on: bool) -> Result<Option<Point>, Failure> {
    if !on {
        return Ok(None);
    }
    Ok(Some(view.convert_point_to_grid(x, y, z)?))
}
