//! Public view readback converts real rendered encoded bytes into owned floating rows.
use super::scene;
use martensite_tkservice::{
    graphic3d::Graphic3dBufferType,
    image::{ImageFormat, ImagePixMap},
};
use martensite_vulkan::{FrameCapture, Graphic3dRenderDriver, VulkanOptions};

const FLOAT_COLOR_LAYOUTS: [(ImageFormat, usize); 3] = [
    (ImageFormat::Rgbaf, 4),
    (ImageFormat::Rgbf, 3),
    (ImageFormat::GrayF, 1),
];
const ORDINARY_COLOR_KINDS: [Graphic3dBufferType; 3] = [
    Graphic3dBufferType::Rgba,
    Graphic3dBufferType::Rgb,
    Graphic3dBufferType::Red,
];

fn assert_rendered_float_rows(image: &ImagePixMap, capture: &FrameCapture, channels: usize) {
    let storage = image.data();
    for y in 0..capture.height as usize {
        let row = storage.data.row(y).unwrap();
        let start = y * capture.width as usize * 4;
        let source = &capture.rgba[start..start + capture.width as usize * 4];
        let expected = expected_rendered_float_row(source, channels);
        assert_eq!(&row[..expected.len()], expected);
        assert_eq!(&row[expected.len()..], &[167; 8]);
    }
}

fn expected_rendered_float_row(source: &[u8], channels: usize) -> Vec<u8> {
    source
        .as_chunks::<4>()
        .0
        .iter()
        .flat_map(|pixel| &pixel[..channels])
        .flat_map(|encoded| ((f64::from(*encoded) / 255.) as f32).to_ne_bytes())
        .collect()
}

fn assert_rendered_float_destination(
    fixture: &scene::ViewerScene,
    capture: &FrameCapture,
    top_down: bool,
    format: ImageFormat,
    channels: usize,
    kind: Graphic3dBufferType,
) {
    let image = ImagePixMap::default();
    image.set_top_down(top_down);
    assert!(image.init_zero(format, 64, 64, 64 * channels * 4 + 8, 167));
    assert!(fixture.cview().buffer_dump(&image, &kind));
    assert_rendered_float_rows(&image, capture, channels);
}

fn assert_rendered_float_layout(
    fixture: &scene::ViewerScene,
    capture: &FrameCapture,
    top_down: bool,
) {
    for (format, channels) in FLOAT_COLOR_LAYOUTS {
        for kind in ORDINARY_COLOR_KINDS {
            assert_rendered_float_destination(fixture, capture, top_down, format, channels, kind);
        }
    }
}

fn midrange_render_fixture(
    driver: &martensite_tkernel::standard::Handle<Graphic3dRenderDriver>,
) -> scene::ViewerScene {
    let fixture = scene::ViewerScene::new(driver);
    fixture
        .group
        .set_group_primitives_aspect(&scene::unlit_style([0.25, 0.5, 0.75, 1.]));
    fixture.render_view().try_redraw().unwrap();
    fixture
}

#[test]
fn real_vulkan_buffer_dump_float_preserves_color_rows_and_lifecycle() {
    let options = VulkanOptions {
        validation: true,
        device_index: None,
    };
    let driver = Graphic3dRenderDriver::new(options).unwrap();
    println!("{{\"buffer_dump_device\":{:?}}}", driver.device_name());
    let fixture = midrange_render_fixture(&driver);
    let capture = fixture.render_view().capture().unwrap();
    assert!(capture.rgba.iter().any(|value| (1..255).contains(value)));
    for top_down in [false, true] {
        assert_rendered_float_layout(&fixture, &capture, top_down);
    }
    fixture.view.remove();
    assert!(fixture.render_view().capture().is_err());
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
}
