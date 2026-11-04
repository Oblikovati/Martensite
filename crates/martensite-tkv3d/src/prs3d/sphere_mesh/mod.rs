//! Deterministic indexed sphere meshes, independent of any rendering backend.
mod grid;
mod nodes;
mod normals;
mod statistics;
mod triangles;
use grid::sphere_grid;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::poly::Triangulation;
use nodes::set_sphere_nodes;
use normals::set_sphere_normals;
pub(crate) use statistics::print_info;
use triangles::set_sphere_triangles;
/// `CalculationOfSphere`: the triangulation of a sphere with its node normals.
pub(crate) fn sphere_triangulation(
    center: [f64; 3],
    res: i32,
    radius: f64,
) -> Result<Handle<Triangulation>, Failure> {
    let grid = sphere_grid(res);
    let nb_points = 2 + grid.theta_res * (grid.j_end - grid.j_start);
    let nb_triangles = 2 * grid.theta_res + 2 * grid.theta_res * (grid.phi_res - 1);
    let tri = Triangulation::with_sizes(nb_points, nb_triangles, false, true)?;
    set_sphere_nodes(&tri, &grid, center, radius)?;
    set_sphere_triangles(&tri, &grid)?;
    let tri = Handle::new(tri);
    set_sphere_normals(&tri)?;
    Ok(tri)
}
