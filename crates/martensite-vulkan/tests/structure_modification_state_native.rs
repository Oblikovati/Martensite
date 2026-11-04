//! GlStructure's ray-tracing modification counter (ModificationState, OnVisibilityChanged)
//! against native values: the recorded structure_modification_state capture printed the
//! counters into fixtures/structure_modification_state/native.txt
//! (driver-independent: no view renders). A structure holding a triangle array is ray-traceable
//! and counts its filling, each visibility change and its transformation; a structure of
//! segments only keeps 0. The Vulkan structure dumps the counter as `ModificationState`.
#[allow(dead_code)]
#[path = "support/recording_device.rs"]
mod recording_device;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkmath::gp::{Transform3, Vector3};
use martensite_tkservice::graphic3d::{Graphic3dTypeOfPrimitiveArray as Kind, *};
use martensite_tkv3d::v3d::V3dViewer;
use martensite_vulkan::*;

const NATIVE: &str = include_str!("fixtures/structure_modification_state/native.txt");
/// the recorded raytrace_counter_reset capture on llvmpipe and radeonsi (equal outputs)
/// in its three runs: `default`, `keep` and `noredraw`.
const NATIVE_RAYTRACED: &str =
    include_str!("fixtures/structure_modification_state/native_raytraced.txt");
const NATIVE_RAYTRACED_KEEP: &str =
    include_str!("fixtures/structure_modification_state/native_raytraced_keep.txt");
const NATIVE_RAYTRACED_NOREDRAW: &str =
    include_str!("fixtures/structure_modification_state/native_raytraced_noredraw.txt");

/// The `ModificationState` field of the structure's DumpJson (DUMP_FIELD_VALUE_NUMERICAL
/// drops the `my` prefix).
fn modification_state(structure: &Handle<Graphic3dStructure>) -> u64 {
    let mut stream = DumpStream::default();
    structure.cstructure().dump_json(&mut stream, 1).unwrap();
    let text = stream.text();
    let key = "\"ModificationState\": ";
    let start = text.find(key).map(|at| at + key.len());
    let start = start.unwrap_or_else(|| panic!("received {text}; expected {key}"));
    let digits: String = text[start..]
        .chars()
        .take_while(char::is_ascii_digit)
        .collect();
    digits.parse().unwrap()
}

fn array(kind: Kind, points: &[[f32; 2]]) -> Handle<Graphic3dArrayOfPrimitives> {
    let array =
        Handle::new(Graphic3dArrayOfPrimitives::new(kind, points.len() as i32, 0, 0, 0).unwrap());
    for [x, y] in points {
        array.add_vertex(*x, *y, 0.).unwrap();
    }
    array
}

#[test]
fn ray_traceable_structures_count_visibility_and_transformation_changes_like_native() {
    let device = recording_device::RecordingDevice::default();
    let driver = Graphic3dRenderDriver::with_backend(device, 128).unwrap();
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    let manager = viewer.structure_manager();
    let traced = Graphic3dStructure::new(&manager, &Handle::default()).unwrap();
    let segments = Graphic3dStructure::new(&manager, &Handle::default()).unwrap();
    let mut lines = Vec::new();
    let mut print = |label: &str| {
        let counts = (modification_state(&traced), modification_state(&segments));
        lines.push(format!("{label} {} {}", counts.0, counts.1));
    };
    print("created");
    let triangle = array(Kind::Triangles, &[[-1., -1.], [1., -1.], [0., 1.]]);
    traced
        .new_group()
        .unwrap()
        .add_primitive_array(&triangle, true);
    let segment = array(Kind::Segments, &[[-1., 0.], [1., 0.]]);
    segments
        .new_group()
        .unwrap()
        .add_primitive_array(&segment, true);
    print("filled");
    for (label, visible) in [("hidden", false), ("shown", true)] {
        traced.set_visible(visible);
        segments.set_visible(visible);
        print(label);
    }
    let mut moved = Transform3::default();
    moved.set_translation(Vector3::new(1., 2., 3.));
    for structure in [&traced, &segments] {
        let datum = martensite_tkmath::toploc::Datum3::new(moved);
        structure.set_transformation(&Handle::new(datum));
    }
    print("moved");
    assert_eq!(lines, NATIVE.lines().collect::<Vec<_>>());
}

/// One triangle around (shift, 0) in its own structure, as the reference builds it.
fn triangle_structure(
    manager: &Handle<Graphic3dStructureManager>,
    shift: f32,
) -> Handle<Graphic3dStructure> {
    let structure = Graphic3dStructure::new(manager, &Handle::default()).unwrap();
    let points = [[shift - 1., -1.], [shift + 1., -1.], [shift, 1.]];
    let triangle = array(Kind::Triangles, &points);
    structure
        .new_group()
        .unwrap()
        .add_primitive_array(&triangle, true);
    structure
}

/// How the reference run prepares the ray-traced view.
#[derive(Clone, Copy, PartialEq)]
enum CounterRun {
    /// `V3dView::SetWindow` redraws the view (rasterized) first.
    Default,
    /// `keepArrayData` is on from the start.
    Keep,
    /// `SetImmediateUpdate(false)` before `SetWindow`: no redraw before the ray-traced ones.
    NoRedraw,
}

/// The reference's lines for one run: two structures displayed, a ray-traced frame, both cleared,
/// three more frames.
fn counter_reset_lines(run: CounterRun) -> Vec<String> {
    use martensite_tkservice::aspect::{AspectNeutralWindow, AspectWindowMembers as _};
    let options = VulkanOptions {
        validation: true,
        device_index: None,
    };
    let driver = Graphic3dRenderDriver::new(options).unwrap();
    driver.change_options(|options| options.keep_array_data = run == CounterRun::Keep);
    let viewer = V3dViewer::new(&driver.cast()).unwrap();
    viewer.set_default_lights().unwrap();
    viewer.set_light_on().unwrap();
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_size(64, 64);
    window.set_virtual(true);
    let view = viewer.create_view().unwrap();
    let base = view.get().unwrap().view_base();
    if run == CounterRun::NoRedraw {
        base.set_immediate_update(false);
    }
    view.set_window(&window.cast(), 0).unwrap();
    base.change_rendering_params(&mut |params| {
        params.method = Graphic3dRenderingMode::Raytracing;
    });
    let manager = viewer.structure_manager();
    let first = triangle_structure(&manager, -1.);
    let second = triangle_structure(&manager, 1.);
    first.display();
    second.display();
    let mut lines = Vec::new();
    let mut print = |label: &str| {
        let counts = (modification_state(&first), modification_state(&second));
        lines.push(format!("{label} {} {}", counts.0, counts.1));
    };
    print("displayed");
    view.redraw();
    print("frame1");
    first.clear(true);
    second.clear(true);
    print("cleared");
    for label in ["frame2", "frame3", "frame4"] {
        view.redraw();
        print(label);
    }
    assert_eq!(driver.validation_errors().unwrap(), Vec::<String>::new());
    lines
}

/// GlView::updateRaytraceGeometry's check (toUpdateStructure) resets, in each ray-traced
/// frame, the counter of the first structure that lost its ray-traced geometry: two such
/// structures reach 0 one frame after the other.
///
/// By default the first ray-traced Redraw finds `keepArrayData` off: GlView::Redraw switches
/// it on and loses the device of the view, which SetWindow had redrawn, so V3dView::Redraw
/// clears and recomputes both structures (they lose their groups) and redraws in the same frame.
/// The first counter is then reset at frame1 and the second at frame2. With `keepArrayData` on,
/// or without an earlier redraw, the structures keep their groups until `Clear` and reset at
/// frame2 and frame3.
#[test]
fn ray_traced_frames_reset_the_counter_of_cleared_structures_one_per_frame_like_native() {
    for (run, native) in [
        (CounterRun::Default, NATIVE_RAYTRACED),
        (CounterRun::Keep, NATIVE_RAYTRACED_KEEP),
        (CounterRun::NoRedraw, NATIVE_RAYTRACED_NOREDRAW),
    ] {
        assert_eq!(counter_reset_lines(run), native.lines().collect::<Vec<_>>());
    }
}
