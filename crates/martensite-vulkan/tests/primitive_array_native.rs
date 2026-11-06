//! `RenderPrimitiveArray` follows the native GlPrimitiveArray contract: driver unique ids
//! (group arrays take one each, null attributes none), draw mode and fill type per primitive
//! type, the dropped dummy index buffer, buffer residency across draw, invalidate, release and
//! init_buffers, and its draw statistics.
//!
//! Expected lines come from the recorded primitive_array capture (with
//! GlCaps::keepArrayData on because the Vulkan driver always keeps the client buffers),
//! identical on llvmpipe and AMD radeonsi and kept in fixtures/primitive_array/native.txt
//!  run with DISPLAY=:1). Residency is recorded while frames
//! are extracted, so a recording backend suffices.
#[allow(dead_code)]
#[path = "support/recording_device.rs"]
mod recording_device;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::{
    Graphic3dFrameStatsCounter as Counter, Graphic3dTypeOfPrimitiveArray as Kind, *,
};
use martensite_tkv3d::v3d::{V3dViewSource, V3dViewer};
use martensite_vulkan::*;
use std::sync::Arc;

const NATIVE: &str = include_str!("fixtures/primitive_array/native.txt");

/// Graphic3dTOPA_UNDEFINED .. Graphic3dTOPA_TRIANGLE_STRIP_ADJACENCY in native order.
const PROBED_KINDS: [Kind; 11] = [
    Kind::Undefined,
    Kind::Points,
    Kind::Segments,
    Kind::Polylines,
    Kind::Triangles,
    Kind::Trianglestrips,
    Kind::Trianglefans,
    Kind::LinesAdjacency,
    Kind::LineStripAdjacency,
    Kind::TrianglesAdjacency,
    Kind::TriangleStripAdjacency,
];

/// The probe's GL draw-mode names.
fn mode_name(mode: Option<Kind>) -> &'static str {
    match mode {
        None => "none",
        Some(Kind::Points) => "points",
        Some(Kind::Segments) => "lines",
        Some(Kind::Polylines) => "line_strip",
        Some(Kind::Triangles) => "triangles",
        Some(Kind::Trianglestrips) => "triangle_strip",
        Some(Kind::Trianglefans) => "triangle_fan",
        Some(Kind::LinesAdjacency) => "lines_adjacency",
        Some(Kind::LineStripAdjacency) => "line_strip_adjacency",
        Some(Kind::TrianglesAdjacency) => "triangles_adjacency",
        Some(Kind::TriangleStripAdjacency) => "triangle_strip_adjacency",
        Some(_) => "unknown",
    }
}

fn triangle() -> Handle<Graphic3dArrayOfPrimitives> {
    let array = Handle::new(Graphic3dArrayOfPrimitives::new(Kind::Triangles, 3, 0, 0, 0).unwrap());
    for [x, y] in [[-1., -1.], [1., -1.], [0., 1.]] {
        array.add_vertex(x, y, 0.).unwrap();
    }
    array
}

/// Four vertices, six 16-bit indices: two triangles.
fn indexed_quad() -> Handle<Graphic3dArrayOfPrimitives> {
    let array = Handle::new(Graphic3dArrayOfPrimitives::new(Kind::Triangles, 4, 0, 6, 0).unwrap());
    for [x, y] in [[-1., -1.], [1., -1.], [1., 1.], [-1., 1.]] {
        array.add_vertex(x, y, 0.).unwrap();
    }
    for index in [1, 2, 3, 1, 3, 4] {
        array.add_edge(index).unwrap();
    }
    array
}

fn array_over(
    driver: &Graphic3dRenderDriver,
    kind: Kind,
    source: &Graphic3dArrayOfPrimitives,
) -> RenderPrimitiveArray {
    RenderPrimitiveArray::new(
        driver,
        kind,
        &source.indices(),
        &source.attributes(),
        &source.bounds(),
    )
}

fn mode_lines(driver: &Graphic3dRenderDriver) -> Vec<String> {
    let source = triangle();
    let mut lines = vec![];
    for (number, kind) in PROBED_KINDS.into_iter().enumerate() {
        for with_attributes in [true, false] {
            let attributes = if with_attributes {
                source.attributes()
            } else {
                Handle::default()
            };
            let array = RenderPrimitiveArray::new(
                driver,
                kind,
                &source.indices(),
                &attributes,
                &source.bounds(),
            );
            lines.push(format!(
                "mode type={number} attribs={} draw={} fill={}",
                u8::from(with_attributes),
                mode_name(array.draw_mode()),
                u8::from(array.is_fill_draw_mode())
            ));
        }
    }
    lines
}

/// The native `printState`: residency, accessors and detailed draw statistics.
fn state_line(label: &str, array: &RenderPrimitiveArray) -> String {
    let mut stats = Graphic3dFrameStatsDataTmp::default();
    array.update_draw_stats(&mut stats, true);
    format!(
        "state {label} init={} bytes={} draw={} fill={} indices={} attribs={} bounds={} \
         elems={} fill_elems={} line_elems={} tris={} lines={}",
        u8::from(array.is_initialized()),
        array.estimated_data_size(),
        mode_name(array.draw_mode()),
        u8::from(array.is_fill_draw_mode()),
        u8::from(!array.indices().is_null()),
        u8::from(!array.attributes().is_null()),
        u8::from(!array.bounds().is_null()),
        stats[Counter::NbElemsNotCulled],
        stats[Counter::NbElemsFillNotCulled],
        stats[Counter::NbElemsLineNotCulled],
        stats[Counter::NbTrianglesNotCulled],
        stats[Counter::NbLinesNotCulled],
    )
}

/// A viewer with one 64x64 view over a recording device.
struct ArrayScene {
    driver: Handle<Graphic3dRenderDriver>,
    viewer: Handle<V3dViewer>,
    view: Handle<dyn V3dViewSource>,
}
impl ArrayScene {
    fn new(lines: &mut Vec<String>) -> Self {
        let device = recording_device::RecordingDevice::default();
        let driver = Graphic3dRenderDriver::with_backend(device, 128).unwrap();
        lines.push(format!("uid first={}", driver.next_primitive_array_uid()));
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        let view = viewer.create_view().unwrap();
        view_setup::configure_view(&view);
        lines.push(format!(
            "uid after_view={}",
            driver.next_primitive_array_uid()
        ));
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

/// Replays the recorded run line by line; `lines` collects what it printed so far.
fn replay() -> Vec<String> {
    let mut lines = vec![];
    let scene = ArrayScene::new(&mut lines);
    let driver = scene.driver.get().unwrap();
    let manager = scene.viewer.structure_manager();
    let structure = Graphic3dStructure::new(&manager, &Handle::default()).unwrap();
    let group = structure.new_group().unwrap();
    group.set_group_primitives_aspect(&view_setup::unlit_aspects::unlit_style([1., 0., 0., 1.]));
    group.add_primitive_array(&triangle(), true);
    group.add_primitive_array(&triangle(), true);
    let none = (Handle::default(), Handle::default(), Handle::default());
    group.add_primitive_array_buffers(Kind::Triangles, &none.0, &none.1, &none.2, true);
    lines.push(format!(
        "uid after_group={}",
        driver.next_primitive_array_uid()
    ));
    let empty = RenderPrimitiveArray::empty(driver);
    lines.push(format!("uid empty={}", empty.uid()));
    lines.push(state_line("empty", &empty));
    lines.extend(mode_lines(driver));
    lines.push(format!(
        "uid after_modes={}",
        driver.next_primitive_array_uid()
    ));
    lines.push(state_line("dummy_indices", &dummy_index_array(driver)));
    lifecycle(&scene, &group, &structure, &mut lines);
    lines.push(format!("uid last={}", driver.next_primitive_array_uid()));
    lines
}

/// An indexed quad whose index buffer has no elements: the constructor drops it.
fn dummy_index_array(driver: &Graphic3dRenderDriver) -> RenderPrimitiveArray {
    let quad = indexed_quad();
    let dummy =
        Handle::new(Graphic3dIndexBuffer::new(Graphic3dBufferBase::default_allocator()).unwrap());
    dummy.init_indices::<u16>(0).unwrap();
    RenderPrimitiveArray::new(
        driver,
        Kind::Triangles,
        &dummy.cast(),
        &quad.attributes(),
        &quad.bounds(),
    )
}

/// Draw, invalidate, release and re-initialize one array appended as a group element.
fn lifecycle(
    scene: &ArrayScene,
    group: &Handle<dyn Graphic3dGroup>,
    structure: &Handle<Graphic3dStructure>,
    lines: &mut Vec<String>,
) {
    let (tri, quad) = (triangle(), indexed_quad());
    let array = Arc::new(array_over(
        scene.driver.get().unwrap(),
        Kind::Triangles,
        &tri,
    ));
    lines.push(format!("uid element={}", array.uid()));
    let draw_group = group.cast::<Graphic3dDrawGroup>();
    draw_group.get().unwrap().add_element(array.clone());
    structure.display();
    lines.push(state_line("constructed", &array));
    let mut step = |label: &str, action: &dyn Fn(), redraw: bool| {
        action();
        if redraw {
            scene.redraw();
        }
        lines.push(state_line(label, &array));
    };
    let reinit =
        |kind: Kind| array.init_buffers(kind, &quad.indices(), &quad.attributes(), &quad.bounds());
    step("rendered", &|| {}, true);
    step("invalidated", &|| array.invalidate(), false);
    step("rerendered", &|| {}, true);
    step("released", &|| array.release(), false);
    step("released_rendered", &|| {}, true);
    step("reinitialized", &|| reinit(Kind::Triangles), false);
    step("reinitialized_rendered", &|| {}, true);
    step("polyline_rendered", &|| reinit(Kind::Polylines), true);
}

#[test]
fn primitive_arrays_follow_the_native_contract_line_by_line() {
    let native: Vec<&str> = NATIVE.lines().collect();
    assert_eq!(replay(), native);
}
