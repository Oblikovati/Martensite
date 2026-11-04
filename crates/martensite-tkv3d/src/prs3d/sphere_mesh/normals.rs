use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkmath::{
    gp::{Direction3, Xyz},
    poly::{Connect, Triangulation},
};
/// The normal of each node: the mean of the unit normals of its triangles.
pub(super) fn set_sphere_normals(tri: &Handle<Triangulation>) -> Result<(), Failure> {
    let value = tri
        .get()
        .ok_or_else(|| Failure::with_message("Standard_NullObject", "Poly_Triangulation"))?;
    let mut connect = Connect::new(tri)?;
    let tol = precision::confusion();
    for i in 1..=value.nb_nodes() {
        let eq_plan = incident_sum(value, &mut connect, tol, i)?;
        let normal = if eq_plan.modulus() > tol {
            Direction3::from_xyz(eq_plan)?
        } else {
            Direction3::new(0.0, 0.0, 1.0)?
        };
        value.set_normal(i, normal)?;
    }
    Ok(())
}
fn incident_sum(
    value: &Triangulation,
    connect: &mut Connect,
    tol: f64,
    index: i32,
) -> Result<Xyz, Failure> {
    let mut eq_plan = Xyz::new(0.0, 0.0, 0.0);
    connect.initialize(index)?;
    while connect.more() {
        let vv = triangle_cross(value, connect.value())?;
        let modulus = vv.modulus();
        if modulus >= tol {
            eq_plan += vv / modulus;
        }
        connect.next()?;
    }
    Ok(eq_plan)
}
fn triangle_cross(value: &Triangulation, index: i32) -> Result<Xyz, Failure> {
    let (i0, i1, i2) = value.triangle(index)?.get();
    let (p0, p1, p2) = (value.node(i0)?, value.node(i1)?, value.node(i2)?);
    let v1 = Xyz::new(p1.x() - p0.x(), p1.y() - p0.y(), p1.z() - p0.z());
    let v2 = Xyz::new(p2.x() - p1.x(), p2.y() - p1.y(), p2.z() - p1.z());
    Ok(v1 ^ v2)
}
