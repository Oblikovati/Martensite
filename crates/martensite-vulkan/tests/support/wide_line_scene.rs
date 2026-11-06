//! Shared actual wide-line geometry for the existing regression and policy capture.
use martensite_tkernel::{quantity::ColorRgba, standard::Handle};
use martensite_tkservice::graphic3d::*;
pub const SEGMENTS: [(&str, [f32; 3], [f32; 3]); 3] = [
    ("h", [-20.3, 0.3, 0.], [20.3, 0.3, 0.]),
    ("v", [0.3, -20.3, 0.], [0.3, 20.3, 0.]),
    ("d", [-20.3, -10.2, 0.], [20.3, 10.1, 0.]),
];

/// A red unlit segment of `width`; native Graphic3dAspectLine3d colours it with the interior.
/// Example: `show_segment(&manager, 3., [[0.; 3], [1., 0., 0.]])`.
pub fn show_segment(
    manager: &Handle<Graphic3dStructureManager>,
    width: f32,
    [from, to]: [[f32; 3]; 2],
) -> Handle<Graphic3dStructure> {
    let structure = Graphic3dStructure::new(manager, &Handle::default()).unwrap();
    let group = structure.new_group().unwrap();
    group.set_group_primitives_aspect(&width_aspect(width));
    group.add_primitive_array(&width_endpoints([from, to]), true);
    structure.display();
    structure
}

fn width_aspect(width: f32) -> Handle<dyn Graphic3dAspectsSource> {
    let style = Handle::new(Graphic3dAspects::default());
    style.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    style.set_interior_color_rgba(ColorRgba::new(1., 0., 0., 1.).unwrap());
    style.set_line_width(width).unwrap();
    style.cast()
}
fn width_endpoints(points: [[f32; 3]; 2]) -> Handle<Graphic3dArrayOfPrimitives> {
    let array = Handle::new(
        Graphic3dArrayOfPrimitives::new(Graphic3dTypeOfPrimitiveArray::Segments, 2, 0, 0, 0)
            .unwrap(),
    );
    for [x, y, z] in points {
        array.add_vertex(x, y, z).unwrap();
    }
    array
}
