//! What a user element reads from its render context while it renders: the window size
//! (`GlWorkspace::Width`, `Height`), the layer's local origin (`GlView::LocalOrigin`,
//! `GlShaderManager::LocalOrigin`), the view shading model and the per-primitive choice
//! (`ChooseFace/Line/MarkerShadingModel`), the frame camera (`GlContext::Camera`), the
//! colour encoding (`ToRenderSRGB`, `IsWindowSRGB`, `IsFrameBufferSRGB`, `Vec4*FromQuantityColor`),
//! PBR support (`HasPBR`) and the transparency test (`CheckIsTransparent`).
//!
//! Expected lines come from the recorded context_queries capture (two white
//! unlit triangles with a querying element, one in a layer with origin (1000, -2000, 3000), in a
//! 64x48 virtual window; Phong, Gouraud and twice Pbr view models), kept in
//! fixtures/context_queries/native.txt (llvmpipe and radeonsi print the same lines). The window
//! visual queries (`WindowBufferBits`, `IsWindowDeepColor`) describe the GL X visual and are not
//! compared.
#[path = "../examples/support/device_options.rs"]
mod device_options;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Xyz;
use martensite_tkservice::aspect::{AspectInteriorStyle, AspectNeutralWindow, AspectWindowMembers};
use martensite_tkservice::graphic3d::{Graphic3dTypeOfPrimitiveArray as Kind, *};
use martensite_tkv3d::v3d::{V3dViewSource, V3dViewer};
use martensite_vulkan::*;
use std::sync::{Arc, Mutex};

const NATIVE: &str = include_str!("fixtures/context_queries/native.txt");
/// Keys whose values are not compared (see the module comment).
const NOT_COMPARED: [&str; 1] = ["window_buffer_bits"];
const MODELS: [Graphic3dTypeOfShadingModel; 7] = [
    Graphic3dTypeOfShadingModel::Default,
    Graphic3dTypeOfShadingModel::Unlit,
    Graphic3dTypeOfShadingModel::PhongFacet,
    Graphic3dTypeOfShadingModel::Gouraud,
    Graphic3dTypeOfShadingModel::Phong,
    Graphic3dTypeOfShadingModel::Pbr,
    Graphic3dTypeOfShadingModel::PbrFacet,
];

/// The frame label of the next redraw and the lines elements wrote.
#[derive(Default)]
struct QueryLog {
    frame: Mutex<&'static str>,
    lines: Mutex<Vec<String>>,
}

struct QueryElement {
    layer: &'static str,
    log: Arc<QueryLog>,
    done: Mutex<Vec<&'static str>>,
}
impl RenderElement for QueryElement {
    fn render(&self, context: &mut RenderElementContext<'_>) -> Result<(), RenderError> {
        let frame = *self.log.frame.lock().unwrap();
        // Rust redraws twice per step (V3d redraw, then the checked driver redraw).
        if self.done.lock().unwrap().contains(&frame) {
            return Ok(());
        }
        self.done.lock().unwrap().push(frame);
        let mut lines = self.log.lines.lock().unwrap();
        for line in element_lines(context) {
            lines.push(format!("{frame} {} {line}", self.layer));
        }
        Ok(())
    }
}

fn xyz(values: [f64; 3]) -> String {
    format!("{} {} {}", values[0], values[1], values[2])
}
fn vec4(values: [f32; 4]) -> String {
    format!("{} {} {} {}", values[0], values[1], values[2], values[3])
}

fn element_lines(context: &RenderElementContext<'_>) -> Vec<String> {
    let state = context.context_state();
    let mut lines = vec![
        format!("workspace_size {} {}", context.width(), context.height()),
        format!("view_local_origin {}", xyz(state.local_origin())),
        format!("manager_local_origin {}", xyz(state.local_origin())),
        format!("shading_model {}", state.shading_model() as i32),
    ];
    lines.extend(choice_lines(state));
    lines.extend(camera_lines(state));
    let color = [0.25, 0.5, 0.75, 0.5];
    lines.extend([
        format!(
            "vec4_from_quantity_color {}",
            vec4(state.vec4_from_quantity_color(color))
        ),
        format!(
            "vec4_linear_from_quantity_color {}",
            vec4(RenderContextState::vec4_linear_from_quantity_color(color))
        ),
        format!(
            "vec4_srgb_from_quantity_color {}",
            vec4(RenderContextState::vec4_srgb_from_quantity_color(color))
        ),
        format!(
            "srgb {} {} {}",
            state.to_render_srgb() as i32,
            state.is_window_srgb() as i32,
            state.is_frame_buffer_srgb() as i32
        ),
        format!("has_pbr {}", state.has_pbr() as i32),
    ]);
    lines.extend(transparency_lines());
    lines
}

fn choice_lines(state: &RenderContextState) -> Vec<String> {
    let mut lines = Vec::new();
    for model in MODELS {
        for normals in [false, true] {
            lines.push(format!(
                "choose {} {} {} {} {}",
                model as i32,
                normals as i32,
                state.choose_face_shading_model(model, normals) as i32,
                state.choose_line_shading_model(model, normals) as i32,
                state.choose_marker_shading_model(model, normals) as i32,
            ));
        }
    }
    lines
}

fn camera_lines(state: &RenderContextState) -> Vec<String> {
    let camera = state.camera().get().expect("the frame camera");
    let point = |p: martensite_tkmath::gp::Point3| [p.x(), p.y(), p.z()];
    let up = camera.up();
    vec![
        format!("camera_eye {}", xyz(point(camera.eye()))),
        format!("camera_center {}", xyz(point(camera.center()))),
        format!("camera_up {}", xyz([up.x(), up.y(), up.z()])),
    ]
}

/// `CheckIsTransparent` of the probe aspects: opaque, half transparent, forced blend and
/// masked, each without and with a translucent red highlight colour.
fn transparency_lines() -> Vec<String> {
    let mut half_material = Graphic3dAspects::default().front_material();
    half_material.set_transparency(0.5).unwrap();
    let half = Handle::new(Graphic3dAspects::default());
    half.set_front_material(half_material.clone());
    let blend = Handle::new(Graphic3dAspects::default());
    blend.set_alpha_mode(Graphic3dAlphaMode::Blend, 0.5);
    let mask = Handle::new(Graphic3dAspects::default());
    mask.set_front_material(half_material.clone());
    mask.set_alpha_mode(Graphic3dAlphaMode::Mask, 0.5);
    let opaque = Handle::new(Graphic3dAspects::default());
    let highlight = Handle::new(Graphic3dPresentationAttributes::default());
    highlight.set_color(Color::named(NamedColor::RED));
    highlight.set_transparency(0.25);
    let highlights: [Handle<dyn Graphic3dPresentationAttributesSource>; 2] =
        [Handle::default(), highlight.cast()];
    let mut lines = Vec::new();
    for (index, aspect) in [opaque, half, blend, mask].iter().enumerate() {
        let aspect = aspect.cast::<dyn Graphic3dAspectsSource>();
        for (hl, style) in highlights.iter().enumerate() {
            let (blend, [front, back]) =
                RenderContextState::check_is_transparent_alphas(&aspect, style);
            let plain = RenderContextState::check_is_transparent(&aspect, style);
            lines.push(format!(
                "transparent {index} {hl} {} {front} {back} {}",
                blend as i32, plain as i32
            ));
        }
    }
    lines
}

fn triangle_structure(
    viewer: &V3dViewer,
    layer: &'static str,
    log: &Arc<QueryLog>,
) -> Handle<Graphic3dStructure> {
    let structure =
        Graphic3dStructure::new(&viewer.structure_manager(), &Handle::default()).unwrap();
    let group = structure.new_group().unwrap();
    let array = Handle::new(Graphic3dArrayOfPrimitives::new(Kind::Triangles, 3, 0, 0, 0).unwrap());
    for [x, y] in [[-10., -10.], [10., -10.], [0., 10.]] {
        array.add_vertex(x, y, 0.).unwrap();
    }
    let fill = Handle::new(Graphic3dAspectFillArea3d::default());
    fill.set_interior_style(AspectInteriorStyle::Solid);
    fill.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    fill.set_interior_color(Color::named(NamedColor::WHITE));
    group.set_group_primitives_aspect(&fill.cast::<dyn Graphic3dAspectsSource>());
    group.add_primitive_array(&array, true);
    let element = Arc::new(QueryElement {
        layer,
        log: log.clone(),
        done: Mutex::default(),
    });
    group
        .cast::<Graphic3dDrawGroup>()
        .get()
        .unwrap()
        .add_element(element);
    structure
}

fn redraw(view: &Handle<dyn V3dViewSource>, log: &QueryLog, frame: &'static str) {
    *log.frame.lock().unwrap() = frame;
    view.redraw();
    let cview = view.get().unwrap().view();
    cview.cast::<Graphic3dRenderView>().try_redraw().unwrap();
}

/// The probe scene on the selected device; returns the element lines of every frame.
fn replay(driver: &Handle<Graphic3dRenderDriver>) -> Vec<String> {
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let view = viewer.create_view().unwrap();
    view.set_immediate_update(false);
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_virtual(true);
    window.set_size(64, 48);
    view.set_window(&window.cast(), 0).unwrap();
    let mut settings = Graphic3dZLayerSettings::default();
    settings.set_origin(Xyz::new(1000., -2000., 3000.));
    let layer = viewer
        .add_z_layer(&settings)
        .unwrap()
        .expect("a free layer id");
    let log = Arc::new(QueryLog::default());
    triangle_structure(&viewer, "default", &log).display();
    let shifted = triangle_structure(&viewer, "origin", &log);
    shifted.set_z_layer(layer);
    shifted.display();
    redraw(&view, &log, "phong");
    view.set_shading_model(Graphic3dTypeOfShadingModel::Gouraud)
        .unwrap();
    redraw(&view, &log, "gouraud");
    view.set_shading_model(Graphic3dTypeOfShadingModel::Pbr)
        .unwrap();
    redraw(&view, &log, "pbr");
    redraw(&view, &log, "pbr_again");
    log.lines.lock().unwrap().clone()
}

/// Native lines without the uncompared keys and the deep-colour flag of `srgb`.
fn native_lines() -> Vec<String> {
    NATIVE
        .lines()
        .filter(|line| {
            !NOT_COMPARED
                .iter()
                .any(|key| line.split(' ').nth(2) == Some(key))
        })
        .map(|line| match line.split(' ').nth(2) {
            Some("srgb") => line.rsplit_once(' ').unwrap().0.to_owned(),
            Some("camera_same_as_view") => String::new(),
            _ => line.to_owned(),
        })
        .filter(|line| !line.is_empty())
        .collect()
}

/// Tokens agree exactly or as numbers: `vec4_*` colours are single precision (native prints
/// `%.9g`, Rust the shortest round trip) and must be the same `f32`; the double camera geometry
/// agrees within `1e-12` relative (V3d evaluates it in a different order).
fn same_line(actual: &str, expected: &str) -> bool {
    let (a, e): (Vec<_>, Vec<_>) = (actual.split(' ').collect(), expected.split(' ').collect());
    let single = e.get(2).is_some_and(|key| key.starts_with("vec4_"));
    a.len() == e.len()
        && a.iter()
            .zip(&e)
            .all(|(a, e)| a == e || same_number(a, e, single))
}
fn same_number(actual: &str, expected: &str, single: bool) -> bool {
    if single {
        return matches!((actual.parse::<f32>(), expected.parse::<f32>()), (Ok(a), Ok(e)) if a == e);
    }
    match (actual.parse::<f64>(), expected.parse::<f64>()) {
        (Ok(a), Ok(e)) => (a - e).abs() <= 1e-12 * e.abs().max(1.),
        _ => false,
    }
}

#[test]
fn element_context_queries_equal_the_native_context() {
    let driver = Graphic3dRenderDriver::new(device_options::validation_options()).unwrap();
    let actual = replay(&driver);
    let expected = native_lines();
    assert_eq!(actual.len(), expected.len(), "{actual:#?}");
    for (index, (actual, expected)) in actual.iter().zip(&expected).enumerate() {
        assert!(
            same_line(actual, expected),
            "line {index}: {actual} != {expected}"
        );
    }
}

#[test]
fn pbr_fallback_keeps_other_models_and_drops_pbr_without_permission() {
    use Graphic3dTypeOfShadingModel as M;
    let fallback = RenderContextState::pbr_shading_model_fallback;
    assert_eq!(fallback(M::Pbr, false), M::Phong);
    assert_eq!(fallback(M::PbrFacet, false), M::PhongFacet);
    assert_eq!(fallback(M::Gouraud, false), M::Gouraud);
    assert_eq!(fallback(M::PbrFacet, true), M::PbrFacet);
}
