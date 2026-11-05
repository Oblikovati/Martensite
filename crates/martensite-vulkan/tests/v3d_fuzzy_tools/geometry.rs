use super::*;
use crate::primitive_shapes::{box_sized, cylinder_sector, rotated_about, rotated_x, translated};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

fn upper_half_sphere() -> Result<Shape, BoxError> {
    let mut maker = BRepPrimAPIMakeSphere::with_latitudes_angle(
        1.600000023842,
        0.,
        90_f64.to_radians(),
        180_f64.to_radians(),
    )?;
    Ok(maker.shape()?.clone())
}

pub(super) fn source_six_inputs() -> Result<[Shape; 6], BoxError> {
    let object = source_box_object()?;
    let sphere_left = translated(&upper_half_sphere()?, [-15., 50.1, 5.5])?;
    let cylinder_left = translated(
        &cylinder_sector(1.600000023842, 3.200000047684, 180.)?,
        [-15., 50.1, 2.35],
    )?;
    let sphere_right = translated(&rotated_x(&upper_half_sphere()?, 90.)?, [-15., -0.1, 5.5])?;
    let cylinder_right = source_right_cylinder()?;
    let cylinder_top = source_top_cylinder()?;
    Ok([
        object,
        sphere_left,
        cylinder_left,
        sphere_right,
        cylinder_right,
        cylinder_top,
    ])
}

fn source_right_cylinder() -> Result<Shape, BoxError> {
    let cylinder_right = rotated_about(
        &cylinder_sector(1.600000023842, 3.200000047684, 180.)?,
        [0.; 3],
        [0., 0., 1.],
        180.,
    )?;
    translated(&cylinder_right, [-15., -0.1, 2.35]).map_err(Into::into)
}

fn source_top_cylinder() -> Result<Shape, BoxError> {
    let cylinder_top = rotated_about(
        &cylinder_sector(1.600000023842, 50.200000762939, 180.)?,
        [0.; 3],
        [0., 0.7071067811865475, 0.7071067811865475],
        180.,
    )?;
    Ok(translated(
        &cylinder_top,
        [-15.000003814697, -0.100001335144, 5.500000953674],
    )?)
}

fn source_box_object() -> Result<Shape, BoxError> {
    Ok(translated(
        &box_sized([13.550000190735, 50.200000762939, 3.299999952316])?,
        [-19.1, -0.1, 2.35],
    )?)
}
