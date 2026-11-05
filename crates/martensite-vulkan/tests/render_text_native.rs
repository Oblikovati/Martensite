//! `RenderText`, the Vulkan counterpart of `GlText`, replays the native element contract.
//!
//! The fixture is the output of the recorded render_text capture on llvmpipe
//! and then AMD radeonsi (build and run as the recorded depth_prepass capture; llvmpipe
//! with GALLIUM_OVERRIDE_CPU_CAPS=sse2 LP_NATIVE_VECTOR_WIDTH=128). Contract lines (residency
//! through SetFontSize, Reset, Release and SetText, the 2D switch, FontKey and StringSize) are
//! identical on both drivers and replayed line by line; the frames of a label drawn by the group's
//! AddText, by the element, and by the element in 2D are fingerprinted and compared with the
//! device's own native driver (llvmpipe for lavapipe, radeonsi for RADV).
#[path = "../examples/support/device_options.rs"]
mod device_options;
#[path = "support/reference_cpu.rs"]
mod reference_cpu;
#[allow(dead_code)]
#[path = "support/rgb_fingerprint.rs"]
mod rgb_fingerprint;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::Handle,
};
use martensite_tkmath::gp::Point3;
use martensite_tkservice::{
    aspect::{AspectNeutralWindow, AspectTypeOfDisplayText, AspectTypeOfStyleText, *},
    font::{FontFontAspect, FontHinting},
    graphic3d::{Graphic3dFrameStatsCounter as Counter, *},
    image::ImagePixMap,
};
use martensite_tkv3d::v3d::{V3dTypeOfOrientation, V3dViewSource, V3dViewer};
use martensite_vulkan::{Graphic3dDrawGroup, Graphic3dRenderDriver, RenderElement, RenderText};
use rgb_fingerprint::{fingerprint, native_fingerprints};
use std::sync::Arc;

const NATIVE: &str = include_str!("fixtures/render_text/native.txt");
const WIDTH: i32 = 160;
const HEIGHT: i32 = 96;

/// The probe's white "DejaVu Sans" text style.
fn white_text() -> Handle<Graphic3dAspectText3d> {
    Handle::new(Graphic3dAspectText3d::new(
        Color::named(NamedColor::WHITE),
        Some("DejaVu Sans"),
        1.,
        0.,
        AspectTypeOfStyleText::Normal,
        AspectTypeOfDisplayText::Normal,
    ))
}

/// A 16-pixel label at `(x, y, 0)`.
fn label(string: &str, x: f64, y: f64) -> Handle<Graphic3dText> {
    let text = Handle::new(Graphic3dText::new(16.));
    text.set_text_str(string);
    text.set_position(&Point3::new(x, y, 0.));
    text
}

/// The native `printState` line.
fn state_line(name: &str, element: &RenderText) -> String {
    let mut stats = Graphic3dFrameStatsDataTmp::default();
    element.update_draw_stats(&mut stats, true);
    let text = element.text();
    format!(
        "state {name} is2d={} height={} text={} bytes={} elems={} text_elems={} tris={}",
        u8::from(element.is_2d()),
        text.height(),
        String::from_utf8_lossy(text.text().to_c_string()).trim_end_matches('\0'),
        element.estimated_data_size(),
        stats[Counter::NbElemsNotCulled],
        stats[Counter::NbElemsTextNotCulled],
        stats[Counter::NbTrianglesNotCulled],
    )
}

/// A float printed by C `%a` (`0x1.51cp+4`, `0x0p+0`, signed).
fn hex_float(field: &str) -> f32 {
    let (sign, rest) = field
        .strip_prefix('-')
        .map_or((1., field), |rest| (-1., rest));
    let (mantissa, exponent) = rest.trim_start_matches("0x").split_once('p').unwrap();
    let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let mut value = f64::from(u32::from_str_radix(whole, 16).unwrap());
    for (index, digit) in fraction.chars().enumerate() {
        value += f64::from(digit.to_digit(16).unwrap())
            / martensite_cmath::powi(16f64, index as i32 + 1);
    }
    (sign * value * martensite_cmath::powi(2f64, exponent.parse().unwrap())) as f32
}

/// The native StringSize lines, in order: width, ascent and descent.
fn native_string_sizes() -> Vec<[f32; 3]> {
    llvmpipe_lines()
        .filter(|line| line.starts_with("stringsize "))
        .map(|line| {
            let field = |key: &str| {
                let start = line.find(key).unwrap() + key.len();
                hex_float(line[start..].split_whitespace().next().unwrap())
            };
            [field("width="), field("ascent="), field("descent=")]
        })
        .collect()
}

/// The llvmpipe half of the fixture: its lines up to the radeonsi run's first line.
fn llvmpipe_lines() -> impl Iterator<Item = &'static str> {
    let first = NATIVE.lines().next().unwrap();
    std::iter::once(first).chain(
        NATIVE
            .lines()
            .skip(1)
            .take_while(move |line| *line != first),
    )
}

/// The native lines starting with `prefix`, from the llvmpipe half of the fixture.
fn native_lines(prefix: &str) -> Vec<&'static str> {
    llvmpipe_lines()
        .filter(|line| line.starts_with(prefix))
        .collect()
}

/// The probe's 160x96 view: fixed orthographic camera, one world unit per pixel.
struct TextView {
    driver: Handle<Graphic3dRenderDriver>,
    viewer: Handle<V3dViewer>,
    view: Handle<dyn V3dViewSource>,
    _window: Handle<AspectNeutralWindow>,
}

impl TextView {
    fn new() -> Self {
        let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        let view = viewer.create_view().unwrap();
        let v3d = view.get().unwrap();
        v3d.set_immediate_update(false);
        let window = Handle::new(AspectNeutralWindow::default());
        window.set_virtual(true);
        window.set_size(WIDTH, HEIGHT);
        v3d.set_window(&window.cast(), 0).unwrap();
        v3d.set_background_color(Color::named(NamedColor::BLACK));
        v3d.set_auto_z_fit_mode(false, 1.).unwrap();
        v3d.set_proj_orientation(V3dTypeOfOrientation::Zpos, false)
            .unwrap();
        let camera = v3d.camera();
        camera.set_center(&Point3::new(0., 0., 0.)).unwrap();
        camera.set_eye(&Point3::new(0., 0., 100.)).unwrap();
        camera.set_scale(96.);
        // The 2D element replaces the world-view: its glyphs sit at view depth 0.
        camera.set_z_range(-200., 200.).unwrap();
        Self {
            driver,
            viewer,
            view,
            _window: window,
        }
    }
    /// An infinite structure with one white-text group, displayed.
    fn text_group(&self) -> (Handle<Graphic3dStructure>, Handle<Graphic3dDrawGroup>) {
        let structure =
            Graphic3dStructure::new(&self.viewer.structure_manager(), &Handle::default()).unwrap();
        let group = structure.new_group().unwrap();
        group.set_group_primitives_aspect(&white_text().cast());
        structure.set_infinite_state(true);
        structure.display();
        (structure, group.cast())
    }
    /// Redraw, dump and compare with the device's native fingerprint of `name`.
    fn mismatch(&self, name: &str) -> Option<String> {
        let v3d = self.view.get().unwrap();
        v3d.redraw();
        let image = ImagePixMap::default();
        image.set_top_down(true);
        let dumped = v3d.to_pix_map_sized(&image, WIDTH, HEIGHT, Graphic3dBufferType::Rgba, true);
        assert!(dumped.unwrap());
        let rgba = image.data().data.data().to_vec();
        let rgb: Vec<u8> = rgba.chunks(4).flat_map(|p| p[..3].to_vec()).collect();
        if let Ok(dir) = std::env::var("XW_DUMP") {
            let mut ppm = format!("P6\n{WIDTH} {HEIGHT}\n255\n").into_bytes();
            ppm.extend(&rgb);
            std::fs::write(format!("{dir}/{name}.ppm"), ppm).unwrap();
        }
        assert!(self.driver.validation_errors().unwrap().is_empty());
        let index = match self.driver.driver_identity().unwrap().driver_name.as_str() {
            "llvmpipe" => 0,
            "radv" => 1,
            other => panic!("no native fixture for driver {other:?}"),
        };
        let actual = fingerprint(&rgb);
        (actual != native_fingerprints(NATIVE, name)[index])
            .then(|| format!("{name}: fnv {} {:?}", actual.hash, actual.counts))
    }
}

#[test]
fn font_keys_and_string_sizes_match_the_native_helpers() {
    let default = Graphic3dAspectText3d::default();
    let bold = white_text();
    bold.set_text_font_aspect(FontFontAspect::Bold);
    let keys = [
        format!(
            "fontkey default={}",
            RenderText::font_key(&default, 16, 72, FontHinting::OFF)
        ),
        format!(
            "fontkey bold={}",
            RenderText::font_key(&*bold, 20, 96, FontHinting::NORMAL)
        ),
    ];
    assert_eq!(keys.to_vec(), native_lines("fontkey "));
    let sizes: Vec<[f32; 3]> = ["Ag", "AV\nab", "a\tb", ""]
        .iter()
        .map(|string| RenderText::string_size(string, &*white_text(), 16., 72, FontHinting::OFF))
        .collect();
    assert_eq!(sizes, native_string_sizes());
}

#[test]
fn text_element_follows_the_native_contract_and_frames() {
    let scene = TextView::new();
    let mut lines = vec![state_line("default", &RenderText::default())];
    let mut failures = Vec::new();
    let (built_in, group) = scene.text_group();
    group.get().unwrap().add_text(&label("Ag", -30., 0.), false);
    failures.extend(scene.mismatch("group"));
    built_in.erase();
    let (_structure, group) = scene.text_group();
    let element = Arc::new(RenderText::new(&label("Ag", -30., 0.)));
    lines.push(state_line("constructed", &element));
    group.get().unwrap().add_element(element.clone());
    failures.extend(scene.mismatch("element"));
    lines.push(state_line("rendered", &element));
    element.set_font_size(16);
    lines.push(state_line("same_size", &element));
    element.set_font_size(20);
    lines.push(state_line("resized", &element));
    failures.extend(scene.mismatch("element-resized"));
    lines.push(state_line("resized_rendered", &element));
    element.reset();
    lines.push(state_line("reset", &element));
    scene.view.get().unwrap().redraw();
    lines.push(state_line("reset_rendered", &element));
    element.release();
    lines.push(state_line("released", &element));
    element.set_text(&label("Wide", -40., -20.));
    lines.push(state_line("replaced", &element));
    failures.extend(scene.mismatch("element-replaced"));
    lines.push(state_line("replaced_rendered", &element));
    element.set_2d(true);
    lines.push(state_line("two_d", &element));
    failures.extend(scene.mismatch("element-2d"));
    lines.push(state_line("two_d_rendered", &element));
    assert_eq!(lines, native_lines("state "));
    reference_cpu::require_exact(failures.is_empty(), || format!("{failures:#?}"));
}
