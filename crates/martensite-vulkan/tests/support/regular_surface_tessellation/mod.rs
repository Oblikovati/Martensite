//! Exact regular surface grid and edge polygons of MeshTest::tessellate.
mod boundaries;
#[cfg(test)]
mod controls;
mod grid;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    brep_tools,
    topods::Face,
};
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle, exceptions::program_error},
};
use martensite_tkg3d::geom::GeomSurface;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
type Bounds = (f64, f64, f64, f64);

/// Rebuild the source face's UV rectangle and mesh its actual located surface; `from_face(&face, 50, 50)?`.
pub fn from_face(face: &Face, nbu: i32, nbv: i32) -> Result<Face, Failure> {
    construct(
        &BRepTool::surface(face)?,
        brep_tools::uv_bounds(face)?,
        nbu,
        nbv,
    )
}

/// Mesh the natural rectangle of a finite surface; `from_surface(&surface, 2, 3)?`.
pub fn from_surface(
    surface: &Handle<dyn GeomSurface>,
    nbu: i32,
    nbv: i32,
) -> Result<Face, Failure> {
    let value = surface
        .get()
        .ok_or_else(|| program_error("tessellate surface=null; expected an actual surface"))?;
    construct(surface, value.bounds(), nbu, nbv)
}

fn construct(
    surface: &Handle<dyn GeomSurface>,
    bounds: Bounds,
    nbu: i32,
    nbv: i32,
) -> Result<Face, Failure> {
    let sizes = grid::sizes(nbu, nbv)?;
    require_finite(bounds)?;
    let maker =
        BRepBuilderAPIMakeFace::from_surface_bounds(surface, bounds, precision::confusion())?;
    let face = maker.face()?.clone();
    let mesh = grid::triangulate(surface, bounds, (nbu, nbv), sizes)?;
    BRepBuilder::default().update_face_triangulation(&face, &mesh, true)?;
    boundaries::attach(&face, &mesh, bounds, nbu, nbv)?;
    Ok(face)
}

fn require_finite((umin, umax, vmin, vmax): Bounds) -> Result<(), Failure> {
    if [umin, umax, vmin, vmax]
        .into_iter()
        .any(precision::is_infinite)
    {
        return Err(program_error(&format!(
            "tessellate bounds={:?}; expected finite parametric range",
            (umin, umax, vmin, vmax)
        )));
    }
    Ok(())
}
