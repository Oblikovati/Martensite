//! Indexed sphere presentations without command interpreters or renderer backends.
mod appearance;
use crate::ais::AisTriangulation;
use crate::prs_mgr::PrsMgrPresentableObjectMembers as _;
use crate::prs3d::sphere_mesh::{print_info, sphere_triangulation};
use appearance::{set_green_node_colors, sphere_shading};
use martensite_tkernel::standard::{Failure, Handle};
/// Build an indexed green sphere with averaged mesh normals and a plastic material.
/// The returned object follows the existing thread-confined AIS presentation ownership.
/// Example: `colored_sphere([0.; 3], 10, 100., false)?`.
pub fn colored_sphere(
    center: [f64; 3],
    resolution: i32,
    radius: f64,
    show_edges: bool,
) -> Result<Handle<AisTriangulation>, Failure> {
    let shape = AisTriangulation::new(&sphere_triangulation(center, resolution, radius)?)?;
    set_green_node_colors(&shape)?;
    shape
        .attributes()
        .set_shading_aspect(sphere_shading(show_edges)?);
    Ok(shape)
}

/// Print mesh sizes and storage statistics for a sphere presentation.
/// Example: `print_colored_sphere_info(&colored_sphere([0.; 3], 10, 100., false)?)?`.
pub fn print_colored_sphere_info(shape: &AisTriangulation) -> Result<(), Failure> {
    let tri = shape.get_triangulation();
    let tri = tri
        .get()
        .ok_or_else(|| Failure::with_message("Standard_NullObject", "Poly_Triangulation"))?;
    print_info(tri.nb_nodes(), tri.nb_triangles());
    Ok(())
}
