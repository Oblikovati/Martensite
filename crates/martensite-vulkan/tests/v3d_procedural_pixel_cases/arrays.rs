//! Primitive arrays retain their actual mutable storage and source vertex-color material.
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::Point3;
use martensite_tkservice::aspect::AspectTypeOfMarker;
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_DEINTERLEAVED, GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_MUTABLE,
    GRAPHIC3D_ARRAY_FLAGS_INDEXES_MUTABLE, GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR,
    Graphic3dArrayOfPoints, Graphic3dArrayOfPrimitives, Graphic3dAspectMarker3d,
    Graphic3dTypeOfPrimitiveArray as Kind,
};
use martensite_tkv3d::ais::PrimitiveArrayPresentation;

const TRIANGLE_VERTICES: [[f32; 3]; 6] = [
    [10., 0., 0.],
    [11., 0., 0.],
    [11., 1., 0.],
    [11., 1., 0.],
    [10., 1., 0.],
    [10., 0., 0.],
];

pub(super) fn point_presentation() -> Result<Handle<PrimitiveArrayPresentation>, Failure> {
    let array = Handle::new(Graphic3dArrayOfPoints::new(4, 0)?);
    for [x, y, z] in [[0., 0., 0.], [0., 0., 10.], [0., 10., 10.], [0., 10., 0.]] {
        array.add_vertex_pnt(&Point3::new(x, y, z))?;
    }
    let object = PrimitiveArrayPresentation::from_array(array.cast());
    let marker = Graphic3dAspectMarker3d::new(
        AspectTypeOfMarker::Point,
        Color::named(NamedColor::YELLOW),
        1.,
    )?;
    object.set_marker_aspect(&Handle::new(marker));
    Ok(object)
}

pub(super) fn mutable_triangles() -> Result<Handle<PrimitiveArrayPresentation>, Failure> {
    let flags = GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_DEINTERLEAVED
        | GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_MUTABLE
        | GRAPHIC3D_ARRAY_FLAGS_INDEXES_MUTABLE
        | GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR;
    let array =
        Graphic3dArrayOfPrimitives::create_array_with_bounds(Kind::Triangles, 6, 0, 0, flags)?;
    let object = PrimitiveArrayPresentation::from_array(array);
    object.setup_vertex_color_shading()?;
    fill_triangles(&object.primitive_array(), false)?;
    Ok(object)
}

pub(super) fn patch_triangles(object: &PrimitiveArrayPresentation) -> Result<bool, Failure> {
    let array = object.primitive_array();
    if !PrimitiveArrayPresentation::patch_counts(&array, Kind::Triangles, [6, 0, 0])? {
        return Ok(false);
    }
    fill_triangles(&array, true)?;
    Ok(true)
}

fn fill_triangles(array: &Graphic3dArrayOfPrimitives, swapped: bool) -> Result<(), Failure> {
    for (index, [x, y, z]) in TRIANGLE_VERTICES.into_iter().enumerate() {
        let rank = index as i32 + 1;
        array.set_vertice(rank, x, y, z)?;
        let [red, green, blue] = vertex_color(index, swapped);
        array.set_vertex_color_rgb(rank, red, green, blue)?;
    }
    Ok(())
}

fn vertex_color(index: usize, swapped: bool) -> [f64; 3] {
    if (index < 3) != swapped {
        return [1., 0., 0.];
    }
    [0., 0., 1.]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_points_keep_case_four_coordinates_and_marker() {
        let object = point_presentation().unwrap();
        let array = object.primitive_array();
        assert_eq!(array.vertex_number(), 4);
        assert_eq!(array.vertice(4).unwrap(), Point3::new(0., 10., 0.));
        assert!(!object.marker_aspect().is_null());
    }

    #[test]
    fn mutable_patch_changes_colors_without_replacing_storage() {
        let object = mutable_triangles().unwrap();
        let array = object.primitive_array();
        assert_eq!(array.vertex_color_rgb(1).unwrap(), [1., 0., 0.]);
        assert_eq!(array.vertex_color_rgb(4).unwrap(), [0., 0., 1.]);
        assert!(patch_triangles(&object).unwrap());
        assert_eq!(
            object.primitive_array().object_address(),
            array.object_address()
        );
        assert_eq!(array.vertex_color_rgb(1).unwrap(), [0., 0., 1.]);
        assert_eq!(array.vertex_color_rgb(4).unwrap(), [1., 0., 0.]);
        assert!(array.attributes().is_mutable());
        assert!(!array.attributes().is_interleaved());
    }
}
