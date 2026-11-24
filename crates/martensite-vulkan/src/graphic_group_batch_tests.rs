//! The batch input of a group: the group it pins, and its material read again only after a
//! write to the aspects.
use super::*;
use martensite_tkernel::quantity::{Color, NamedColor};

struct BatchScene {
    _driver: Handle<crate::Graphic3dRenderDriver>,
    _structure: Handle<Graphic3dStructure>,
    group: Handle<Graphic3dDrawGroup>,
    other: Handle<Graphic3dDrawGroup>,
    aspect: Handle<Graphic3dAspects>,
}

struct BatchNeverRenderBackend;
impl crate::RenderBackend for BatchNeverRenderBackend {
    fn render(&mut self, _: &crate::RenderFrame) -> Result<crate::FrameCapture, RenderError> {
        panic!("unexpected rendering; expected CPU-only batch probes")
    }
}

fn scene() -> BatchScene {
    let driver = crate::Graphic3dRenderDriver::with_backend_and_line_width_configuration(
        BatchNeverRenderBackend,
        64,
        std::sync::Arc::new(crate::FreeTypeTextSource::default()),
        Default::default(),
        Default::default(),
    )
    .unwrap();
    let manager = Handle::new(Graphic3dStructureManager::new(&driver.cast()).unwrap());
    let structure = Graphic3dStructure::new(&manager, &Handle::default()).unwrap();
    let group = structure.new_group().unwrap().cast::<Graphic3dDrawGroup>();
    let other = structure.new_group().unwrap().cast::<Graphic3dDrawGroup>();
    let aspect = Handle::new(Graphic3dAspects::default());
    aspect.set_interior_style(martensite_tkservice::aspect::AspectInteriorStyle::Solid);
    aspect.set_interior_color(Color::named(NamedColor::RED));
    group.set_group_primitives_aspect(&aspect.clone().cast());
    let kind = Graphic3dTypeOfPrimitiveArray::Triangles;
    let flags = GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL;
    let array = Handle::new(Graphic3dArrayOfPrimitives::new(kind, 3, 0, 3, flags).unwrap());
    let corners = [[-0.5, -0.5, 0.], [0.5, -0.5, 0.], [0., 0.5, 0.]]
        .map(|point| array.add_vertex_normal_f64(point, [0., 0., 1.]).unwrap());
    array
        .add_edges3(corners[0], corners[1], corners[2])
        .unwrap();
    group.add_primitive_array(&array.upcast(), true);
    BatchScene {
        _driver: driver,
        _structure: structure,
        group,
        other,
        aspect,
    }
}

#[test]
fn a_pinned_input_names_only_the_group_it_was_read_from() {
    let scene = scene();
    let mut input = scene
        .group
        .probe_graphic_input()
        .expect("an ordinary surface group");
    assert!(
        !input.is_group(&scene.group.clone().cast()),
        "nothing pinned yet"
    );
    input.pin_group(&scene.group.clone().cast());
    assert!(input.is_group(&scene.group.clone().cast()));
    assert!(!input.is_group(&scene.other.clone().cast()));
}

#[test]
fn a_written_aspect_is_read_again_and_only_a_changed_material_fails() {
    let scene = scene();
    let input = scene.group.probe_graphic_input().unwrap();
    assert!(scene.group.probe_unchanged(&input));
    // The same value again advances the revision; the material read again still matches.
    scene
        .aspect
        .set_interior_color(Color::named(NamedColor::RED));
    assert!(scene.group.probe_unchanged(&input));
    scene
        .aspect
        .set_interior_color(Color::named(NamedColor::GREEN));
    assert!(!scene.group.probe_unchanged(&input));
}
