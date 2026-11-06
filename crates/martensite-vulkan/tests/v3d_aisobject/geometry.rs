//! Exact cylinder/disks and sectional-wire arrays of the tutorial Compute implementation.
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::{
    bnd::BoundingBox3,
    gp::{Axis3, Direction3, Point3, Transform3},
};
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::prs3d::{Prs3dToolCylinder, Prs3dToolDisk, Prs3dToolQuadric};

pub(super) fn triangles() -> Result<Handle<Graphic3dArrayOfTriangles>, Failure> {
    let cylinder = Prs3dToolCylinder::new(100., 100., 100., 25, 25);
    let disk = Prs3dToolDisk::new(0., 100., 25, 1);
    let vertices = cylinder.vertices_nb(true) + 2 * disk.vertices_nb(true);
    let indices = 3 * (cylinder.triangles_nb() + 2 * disk.triangles_nb());
    let mut array = Handle::new(Graphic3dArrayOfTriangles::new(
        vertices,
        indices,
        GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL,
    )?);
    cylinder.fill_array(&mut array, &Transform3::default())?;
    disk.fill_array(&mut array, &Transform3::default())?;
    disk.fill_array(&mut array, &top_disk_transform()?)?;
    Ok(array)
}

fn top_disk_transform() -> Result<Transform3, Failure> {
    let axes = Axis3::new(
        Point3::new(0., 0., 100.),
        Direction3::new(0., 0., -1.)?,
        Direction3::new(1., 0., 0.)?,
    )?;
    let mut top = Transform3::default();
    top.set_transformation_between(axes, Axis3::default());
    Ok(top)
}

pub(super) fn section() -> Result<Handle<Graphic3dArrayOfSegments>, Failure> {
    let array = Handle::new(Graphic3dArrayOfSegments::new(4, 8, 0)?);
    for [x, y, z] in [
        [0., -100., 0.],
        [0., -100., 100.],
        [0., 100., 100.],
        [0., 100., 0.],
    ] {
        array.add_vertex_pnt(&Point3::new(x, y, z))?;
    }
    for (first, last) in [(1, 2), (2, 3), (3, 4), (4, 1)] {
        array.add_edges2(first, last)?;
    }
    Ok(array)
}

pub(super) fn bounds() -> Result<BoundingBox3, Failure> {
    let shape = crate::primitive_shapes::cylinder(100., 100.)?;
    bounds_of(&shape)
}

pub(super) fn bounds_of(shape: &martensite_tkbrep::topods::Shape) -> Result<BoundingBox3, Failure> {
    let mut bounds = BoundingBox3::default();
    martensite_tktopalgo::brep_bnd_lib::BRepBndLib::add(shape, &mut bounds, true)?;
    Ok(bounds)
}
