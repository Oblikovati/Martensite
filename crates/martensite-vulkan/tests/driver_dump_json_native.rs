//! The driver objects write the native `DumpJson` `GlGroup`,
//! `GlStructure`, `GlPrimitiveArray`, `GlText`, `GlElement`,
//! `GlGraduatedTrihedron`, `GlMatrixState` and the aspects, stencil and flipping
//! elements of a group, field for field.
//!
//! Expected lines come from the recorded driver_dump_json capture, identical on llvmpipe
//! and AMD radeonsi once addresses are numbered by first appearance on each line, and kept in
//! fixtures/driver_dump_json/native.txt run with DISPLAY=:1). After the
//! first frame the native arrays and texts also dump their GL buffer objects (`VboAttribs`,
//! `VboIndices`, `VertexBuffer`); the Vulkan frame uploads the Graphic3d buffers into its own
//! buffers, so those objects are removed from the native lines before comparing.
#[allow(dead_code)]
#[path = "support/recording_device.rs"]
mod recording_device;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::ncollection::Mat4;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkmath::gp::{Axis2, Direction3, Point3, Transform3};
use martensite_tkservice::aspect::{AspectInteriorStyle, AspectTypeOfLine};
use martensite_tkservice::graphic3d::{Graphic3dTypeOfPrimitiveArray as Kind, *};
use martensite_tkv3d::v3d::V3dViewer;
use martensite_vulkan::*;
use std::sync::Arc;

const NATIVE: &str = include_str!("fixtures/driver_dump_json/native.txt");

/// Number the addresses of one dump line by first appearance (`"P1"`, `"P2"`, ...).
fn number_addresses(line: &str) -> String {
    let mut seen: Vec<String> = vec![];
    let mut out = String::new();
    let mut rest = line;
    while let Some(start) = rest.find("\"0x") {
        let end = rest[start + 1..]
            .find('"')
            .map_or(rest.len(), |e| start + 2 + e);
        let address = rest[start..end].to_owned();
        let index = seen.iter().position(|s| *s == address).unwrap_or_else(|| {
            seen.push(address.clone());
            seen.len() - 1
        });
        out.push_str(&rest[..start]);
        out.push_str(&format!("\"P{}\"", index + 1));
        rest = &rest[end..];
    }
    out + rest
}

/// Drop the native GL buffer objects (flat objects) from a dump line.
fn without_gl_buffers(line: &str) -> String {
    let mut out = line.to_owned();
    for key in ["VboAttribs", "VboIndices", "VertexBuffer"] {
        let pattern = format!(", \"{key}\": {{");
        while let Some(start) = out.find(&pattern) {
            let end = start + out[start..].find('}').expect("closed GL buffer object") + 1;
            out.replace_range(start..end, "");
        }
    }
    out
}

fn native_lines() -> Vec<String> {
    NATIVE.lines().map(without_gl_buffers).collect()
}

/// A dump line `"<label> <depth> <body>"`.
fn line(label: &str, depth: i32, dump: impl Fn(&mut DumpStream, i32)) -> String {
    let mut stream = DumpStream::default();
    dump(&mut stream, depth);
    number_addresses(&format!("{label} {depth} {}", stream.text()))
}

fn triangle() -> Handle<Graphic3dArrayOfPrimitives> {
    let array = Handle::new(Graphic3dArrayOfPrimitives::new(Kind::Triangles, 3, 0, 0, 0).unwrap());
    for [x, y] in [[-1., -1.], [1., -1.], [0., 1.]] {
        array.add_vertex(x, y, 0.).unwrap();
    }
    array
}

fn segment() -> Handle<Graphic3dArrayOfPrimitives> {
    let array = Handle::new(Graphic3dArrayOfPrimitives::new(Kind::Segments, 2, 0, 0, 0).unwrap());
    array.add_vertex(-1., 0., 0.).unwrap();
    array.add_vertex(1., 0., 0.).unwrap();
    array
}

fn label() -> Handle<Graphic3dText> {
    let text = Handle::new(Graphic3dText::new(12.));
    text.set_text_str("dump");
    text.set_position(&Point3::new(0.5, 0.25, 0.));
    text
}

/// A user element with the default `GlElement` dump; it dumps the workspace state it
/// sees on its first render.
#[derive(Default)]
struct DumpingElement {
    workspace: std::sync::Mutex<Vec<String>>,
}
impl RenderElement for DumpingElement {
    fn render(&self, context: &mut RenderElementContext<'_>) -> Result<(), RenderError> {
        let mut lines = self.workspace.lock().unwrap();
        if lines.is_empty() {
            for depth in [-1, 1] {
                lines.push(line("workspace", depth, |s, d| {
                    context.dump_json(s, d).unwrap()
                }));
            }
        }
        Ok(())
    }
}

struct DumpScene {
    driver: Handle<Graphic3dRenderDriver>,
    viewer: Handle<V3dViewer>,
    view: Handle<dyn martensite_tkv3d::v3d::V3dViewSource>,
}

impl DumpScene {
    fn new() -> Self {
        let device = recording_device::RecordingDevice::default();
        let driver = Graphic3dRenderDriver::with_backend(device, 128).unwrap();
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        let view = viewer.create_view().unwrap();
        view_setup::configure_view(&view);
        Self {
            driver,
            viewer,
            view,
        }
    }
    fn redraw(&self) {
        let cview = self.view.get().unwrap().view();
        cview.cast::<Graphic3dRenderView>().try_redraw().unwrap();
    }
}

/// Builds the group of the probe: aspects, arrays, stencil and flipping brackets, a text and
/// a user element, in the native call order.
fn fill_group(group: &Handle<dyn Graphic3dGroup>) -> Arc<DumpingElement> {
    let fill = Handle::new(Graphic3dAspectFillArea3d::default());
    fill.set_interior_style(AspectInteriorStyle::Solid);
    fill.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    group.set_group_primitives_aspect(&fill.cast::<dyn Graphic3dAspectsSource>());
    group.add_primitive_array(&triangle(), true);
    group.set_stencil_test_options(true);
    group.set_flipping_options(true, &Axis2::default());
    let line =
        Graphic3dAspectLine3d::new(Color::named(NamedColor::GREEN), AspectTypeOfLine::Dash, 2.)
            .unwrap();
    group.set_primitives_aspect(&Handle::new(line).cast::<dyn Graphic3dAspectsSource>());
    group.add_primitive_array(&segment(), true);
    group.add_text(&label(), false);
    group.set_flipping_options(false, &Axis2::default());
    group.set_stencil_test_options(false);
    let element = Arc::new(DumpingElement::default());
    group
        .cast::<Graphic3dDrawGroup>()
        .get()
        .unwrap()
        .add_element(element.clone());
    element
}

fn replay() -> Vec<String> {
    let scene = DumpScene::new();
    let manager = scene.viewer.structure_manager();
    let structure = Graphic3dStructure::new(&manager, &Handle::default()).unwrap();
    let group = structure.new_group().unwrap();
    let mut lines = vec![line("group_empty", -1, |s, d| {
        group.dump_json(s, d).unwrap()
    })];
    let element = fill_group(&group);
    lines.push(line("element", -1, |s, d| element.dump_json(s, d).unwrap()));
    for depth in [-1, 1, 0] {
        lines.push(line("group", depth, |s, d| group.dump_json(s, d).unwrap()));
    }
    let cstructure = structure.cstructure();
    for depth in [-1, 1] {
        lines.push(line("structure", depth, |s, d| {
            cstructure.dump_json(s, d).unwrap()
        }));
    }
    let mut mirror = Transform3::default();
    mirror.set_mirror_plane(
        Axis2::new(Point3::default(), Direction3::new(1., 0., 0.).unwrap()).unwrap(),
    );
    structure.set_transformation(&Handle::new(martensite_tkmath::toploc::Datum3::new(mirror)));
    lines.push(line("structure_mirrored", 1, |s, d| {
        cstructure.dump_json(s, d).unwrap()
    }));
    let shadow = Graphic3dStructure::new(&manager, &structure.clone().cast()).unwrap();
    lines.push(line("structure_instance", 1, |s, d| {
        shadow.cstructure().dump_json(s, d).unwrap()
    }));
    structure.display();
    scene.redraw();
    lines.extend(element.workspace.lock().unwrap().iter().cloned());
    lines.push(line("group_rendered", -1, |s, d| {
        group.dump_json(s, d).unwrap()
    }));
    lines.push(line("structure_rendered", 1, |s, d| {
        cstructure.dump_json(s, d).unwrap()
    }));
    lines.extend(standalone_elements(&scene.driver));
    lines
}

fn standalone_elements(driver: &Graphic3dRenderDriver) -> Vec<String> {
    let mut lines = vec![];
    let tri = triangle();
    let array = RenderPrimitiveArray::new(
        driver,
        Kind::Triangles,
        &tri.indices(),
        &tri.attributes(),
        &tri.bounds(),
    );
    for depth in [-1, 1] {
        lines.push(line("primitive_array", depth, |s, d| {
            array.dump_json(s, d).unwrap()
        }));
    }
    let empty = RenderPrimitiveArray::empty(driver);
    lines.push(line("primitive_array_empty", -1, |s, d| {
        empty.dump_json(s, d).unwrap()
    }));
    let text = RenderText::new(&label());
    lines.push(line("text", -1, |s, d| text.dump_json(s, d).unwrap()));
    let default_text = RenderText::default();
    lines.push(line("text_default", -1, |s, d| {
        default_text.dump_json(s, d).unwrap()
    }));
    let trihedron = RenderGraduatedTrihedron::new();
    lines.push(line("graduated_trihedron", -1, |s, d| {
        trihedron.dump_json(s, d).unwrap()
    }));
    let mut state = RenderMatrixState::<f32>::default();
    lines.push(line("matrix_state", -1, |s, d| {
        state.dump_json(s, d).unwrap()
    }));
    state.push();
    let mut matrix = Mat4::<f32>::identity();
    matrix.set_value(0, 3, 2.5);
    matrix.set_value(1, 1, -0.125);
    matrix.set_value(2, 0, 1.0e-7);
    state.set_current(&matrix);
    lines.push(line("matrix_state", -1, |s, d| {
        state.dump_json(s, d).unwrap()
    }));
    let mut double = RenderMatrixState::<f64>::default();
    let mut matrix = Mat4::<f64>::identity();
    matrix.set_value(3, 2, 0.1);
    double.set_current(&matrix);
    lines.push(line("matrix_state_double", -1, |s, d| {
        double.dump_json(s, d).unwrap()
    }));
    lines
}

#[test]
fn driver_objects_dump_the_native_json() {
    let expected = native_lines();
    let actual = replay();
    for (index, (actual, expected)) in actual.iter().zip(&expected).enumerate() {
        assert_eq!(actual, expected, "dump line {index}");
    }
    assert_eq!(actual.len(), expected.len());
}

#[test]
fn matrix_state_keeps_the_popped_slots() {
    let mut state = RenderMatrixState::<f32>::default();
    state.push();
    state.push();
    state.pop().unwrap();
    let text = line("m", -1, |s, d| state.dump_json(s, d).unwrap());
    // Native NCollectionDynamicArray keeps both appended slots; the head is the one left.
    assert!(
        text.ends_with("\"Stack.Length\": 2, \"StackHead\": 0"),
        "{text}"
    );
}
