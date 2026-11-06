//! Primitive solids used as regression inputs.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// An axis-aligned box with its corner at `corner`; `box_at([-1., -1., -1.], [3., 3., 3.])?`.
pub(crate) fn box_at(corner: [f64; 3], size: [f64; 3]) -> Result<Shape, Failure> {
    let [x, y, z] = corner;
    let mut maker = BRepPrimAPIMakeBox::at_point(Point3::new(x, y, z), size[0], size[1], size[2])?;
    Ok(maker.shape()?.clone())
}

/// A box with its corner at the origin; `box_sized([25., 25., 25.])?`.
pub(crate) fn box_sized(size: [f64; 3]) -> Result<Shape, Failure> {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(size[0], size[1], size[2])?;
    Ok(maker.shape()?.clone())
}

/// A cylinder; `cylinder(10., 20.)?`.
pub(crate) fn cylinder(radius: f64, height: f64) -> Result<Shape, Failure> {
    let mut maker = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder::new(radius, height);
    Ok(maker.shape()?.clone())
}

/// A cone; `cone(10., 0., 20.)?`.
pub(crate) fn cone(bottom: f64, top: f64, height: f64) -> Result<Shape, Failure> {
    let mut maker =
        martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCone::new(bottom, top, height)?;
    Ok(maker.shape()?.clone())
}

/// A sphere of radius `radius`; `sphere(1.)?`.
pub(crate) fn sphere(radius: f64) -> Result<Shape, Failure> {
    let mut maker = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere::new(radius)?;
    Ok(maker.shape()?.clone())
}

/// A translation without copy (BRepBuilderAPITransform); `translated(&s, [25., 25., 0.])?`.
pub(crate) fn translated(shape: &Shape, offset: [f64; 3]) -> Result<Shape, Failure> {
    transformed(
        shape,
        martensite_tkmath::gp::Transform3::translation(offset),
    )
}

/// A rotation by `degrees` about the X axis through the
/// origin, without copy; `rotated_x(&s, 45.)?`.
pub(crate) fn rotated_x(shape: &Shape, degrees: f64) -> Result<Shape, Failure> {
    use martensite_tkmath::gp::{Axis1, StandardDirection3, Transform3};
    let rotation = Transform3::rotation(
        Axis1::from_standard(StandardDirection3::X),
        degrees * std::f64::consts::PI / 180.,
    )?;
    transformed(shape, rotation)
}

fn transformed(
    shape: &Shape,
    transform: martensite_tkmath::gp::Transform3,
) -> Result<Shape, Failure> {
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPITransform;
    let mut maker = BRepBuilderAPITransform::new(transform);
    maker.perform(shape, false, false)?;
    Ok(maker.shape()?.clone())
}

/// A rotation by `degrees` about the axis through
/// `point` along `direction`, without copy; `rotated_about(&s, [100., 0., 25.], [0., 1., 0.], 90.)?`.
pub(crate) fn rotated_about(
    shape: &Shape,
    [px, py, pz]: [f64; 3],
    [dx, dy, dz]: [f64; 3],
    degrees: f64,
) -> Result<Shape, Failure> {
    use martensite_tkmath::gp::{Axis1, Direction3, Transform3};
    let axis = Axis1::new(Point3::new(px, py, pz), Direction3::new(dx, dy, dz)?);
    let mut rotation = Transform3::default();
    rotation.set_rotation(axis, degrees * (std::f64::consts::PI / 180.0))?;
    transformed(shape, rotation)
}

/// The shapes in one compound; `compound_of(&[a, b])?`.
pub(crate) fn compound_of(shapes: &[Shape]) -> Result<Shape, Failure> {
    use martensite_tkbrep::topods::{Builder, Compound};
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    for shape in shapes {
        Builder.add(&mut compound, shape)?;
    }
    Ok(compound.into_shape())
}

/// A cylinder sector of `degrees`; `cylinder_sector(2., 10., 90.)?`.
pub(crate) fn cylinder_sector(radius: f64, height: f64, degrees: f64) -> Result<Shape, Failure> {
    let angle = degrees.to_radians();
    let mut maker = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder::with_angle(
        radius, height, angle,
    )?;
    Ok(maker.shape()?.clone())
}

/// A sphere sector of `degrees`; `sphere_sector(10., 180.)?`.
pub(crate) fn sphere_sector(radius: f64, degrees: f64) -> Result<Shape, Failure> {
    let angle = degrees.to_radians();
    let mut maker =
        martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere::with_angle(radius, angle)?;
    Ok(maker.shape()?.clone())
}

/// The mirror image in the plane through `point` with
/// normal `normal`, without copy; `mirrored(&s, [0.; 3], [0., 1., 0.])?`.
pub(crate) fn mirrored(
    shape: &Shape,
    [px, py, pz]: [f64; 3],
    [nx, ny, nz]: [f64; 3],
) -> Result<Shape, Failure> {
    use martensite_tkmath::gp::{Axis2, Direction3, Transform3};
    let mut mirror = Transform3::default();
    mirror.set_mirror_plane(Axis2::new(
        Point3::new(px, py, pz),
        Direction3::new(nx, ny, nz)?,
    )?);
    transformed(shape, mirror)
}

/// A preview box includes zero-height rectangles; `preview_box_at([0.; 3], [2., 3., 0.])?`.
pub(crate) fn preview_box_at(corner: [f64; 3], size: [f64; 3]) -> Result<Shape, Failure> {
    let mut maker = martensite_tkprim::brep_preview_api::BRepPreviewAPIMakeBox::new();
    maker.init_at_point(
        Point3::new(corner[0], corner[1], corner[2]),
        size[0],
        size[1],
        size[2],
    )?;
    Ok(maker.shape()?.clone())
}
