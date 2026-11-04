//! A user render element draws through the same Vulkan path as the group's own arrays: an
//! element emitting the scene triangle produces the byte-identical frame on the real device.
#[path = "../examples/support/device_options.rs"]
mod device_options;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use martensite_tkservice::image::ImagePixMap;
use martensite_vulkan::*;
use std::sync::Arc;

/// Named element: emits one retained primitive array with the current aspects.
struct ArrayElement(Handle<Graphic3dArrayOfPrimitives>);
impl RenderElement for ArrayElement {
    fn render(&self, context: &mut RenderElementContext<'_>) -> Result<(), RenderError> {
        let array = self.0.get().expect("a retained array");
        context.add_primitive_array(
            array.primitive_type(),
            array.indices_ref(),
            array.attributes_ref(),
            array.bounds_ref(),
        );
        Ok(())
    }
    fn is_fill_draw_mode(&self) -> bool {
        true
    }
}

fn dump(scene: &scene::ViewerScene) -> Vec<u8> {
    let image = ImagePixMap::default();
    image.set_top_down(true);
    let view = scene.view.get().unwrap();
    assert!(
        view.to_pix_map_sized(&image, 64, 64, Graphic3dBufferType::Rgba, true)
            .unwrap()
    );
    image.data().data.data().to_vec()
}

#[test]
fn element_geometry_renders_like_the_group_array() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let scene = scene::ViewerScene::new(&driver);
    let builtin = dump(&scene);
    scene.group.clear(true);
    scene
        .group
        .set_group_primitives_aspect(&scene::unlit_style([1., 0., 0., 1.]));
    let group: Handle<Graphic3dDrawGroup> = scene.group.clone().cast();
    let element = ArrayElement(scene.vertices.clone());
    group.get().unwrap().add_element(Arc::new(element));
    let emitted = dump(&scene);
    assert!(builtin.as_chunks::<4>().0.contains(&[255, 0, 0, 255]));
    assert!(
        builtin == emitted,
        "element frame differs from the group array frame"
    );
    assert!(driver.validation_errors().unwrap().is_empty());
}

#[test]
fn primitive_array_element_renders_like_the_group_array() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let scene = scene::ViewerScene::new(&driver);
    let builtin = dump(&scene);
    scene.group.clear(true);
    scene
        .group
        .set_group_primitives_aspect(&scene::unlit_style([1., 0., 0., 1.]));
    let vertices = scene.vertices.get().unwrap();
    let array = Arc::new(RenderPrimitiveArray::new(
        driver.get().unwrap(),
        vertices.primitive_type(),
        &vertices.indices(),
        &vertices.attributes(),
        &vertices.bounds(),
    ));
    let group: Handle<Graphic3dDrawGroup> = scene.group.clone().cast();
    group.get().unwrap().add_element(array.clone());
    let emitted = dump(&scene);
    assert!(builtin.as_chunks::<4>().0.contains(&[255, 0, 0, 255]));
    assert!(
        builtin == emitted,
        "array element frame differs from the group array frame"
    );
    assert!(array.is_initialized());
    // Released buffers are built again by the next frame, which stays identical.
    array.release();
    assert!(
        builtin == dump(&scene),
        "rebuilt array element frame differs"
    );
    assert!(driver.validation_errors().unwrap().is_empty());
}
