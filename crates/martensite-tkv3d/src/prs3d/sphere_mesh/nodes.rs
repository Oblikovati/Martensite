use super::grid::SphereGrid;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::Point3;
use martensite_tkmath::poly::Triangulation;
/// The poles and the intermediate nodes of the sphere.
pub(super) fn set_sphere_nodes(
    tri: &Triangulation,
    grid: &SphereGrid,
    center: [f64; 3],
    radius: f64,
) -> Result<(), Failure> {
    tri.set_node(1, Point3::new(center[0], center[1], center[2] + radius))?;
    tri.set_node(2, Point3::new(center[0], center[1], center[2] - radius))?;
    let mut number = 3;
    for i in 0..grid.theta_res {
        let theta = grid.start_theta + f64::from(i) * grid.delta_theta;
        set_meridian(tri, grid, center, radius, theta, &mut number)?;
    }
    Ok(())
}

fn set_meridian(
    tri: &Triangulation,
    grid: &SphereGrid,
    center: [f64; 3],
    radius: f64,
    theta: f64,
    number: &mut i32,
) -> Result<(), Failure> {
    for j in grid.j_start..grid.j_end {
        let phi = f64::from(j) * grid.delta_phi;
        let ring = radius * martensite_cmath::sin(phi);
        let n = [
            ring * martensite_cmath::cos(theta),
            ring * martensite_cmath::sin(theta),
            radius * martensite_cmath::cos(phi),
        ];
        tri.set_node(
            *number,
            Point3::new(n[0] + center[0], n[1] + center[1], n[2] + center[2]),
        )?;
        *number += 1;
    }
    Ok(())
}
