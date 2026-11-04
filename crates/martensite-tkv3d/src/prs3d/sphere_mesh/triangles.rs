use super::grid::SphereGrid;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::poly::{Triangle, Triangulation};
struct TriangleWriter<'a> {
    tri: &'a Triangulation,
    number: i32,
}
impl TriangleWriter<'_> {
    fn add(&mut self, pts: [i32; 3]) -> Result<(), Failure> {
        self.tri
            .set_triangle(self.number, Triangle::new(pts[0], pts[1], pts[2]))?;
        self.number += 1;
        Ok(())
    }
}
/// The triangles around the poles and of the bands between them.
pub(super) fn set_sphere_triangles(tri: &Triangulation, grid: &SphereGrid) -> Result<(), Failure> {
    let mut writer = TriangleWriter { tri, number: 1 };
    set_poles(&mut writer, grid)?;
    set_bands(&mut writer, grid)
}
fn set_poles(writer: &mut TriangleWriter<'_>, grid: &SphereGrid) -> Result<(), Failure> {
    let (phi_res, base, poles) = (grid.phi_res, grid.base, 3);
    for i in 0..grid.theta_res {
        writer.add([phi_res * i + poles, (phi_res * (i + 1) % base) + poles, 1])?;
    }
    let offset = phi_res - 1 + poles;
    for i in 0..grid.theta_res {
        writer.add([
            phi_res * i + offset,
            poles - 1,
            ((phi_res * (i + 1)) % base) + offset,
        ])?;
    }
    Ok(())
}
fn set_bands(writer: &mut TriangleWriter<'_>, grid: &SphereGrid) -> Result<(), Failure> {
    let (phi_res, base, poles) = (grid.phi_res, grid.base, 3);
    for i in 0..grid.theta_res {
        for j in 0..phi_res - 1 {
            let p0 = phi_res * i + j + poles;
            let p2 = ((phi_res * (i + 1) + j) % base) + poles + 1;
            writer.add([p0, p0 + 1, p2])?;
            writer.add([p0, p2, p2 - 1])?;
        }
    }
    Ok(())
}
