//! Frame statistics call the statistics hooks of user render elements in native order:
//! GlFrameStats::updateStructures runs UpdateMemStats and UpdateDrawStats on every group
//! node, so an element appended with GlGroup::AddElement adds its own counters.
//!
//! Expected lines come from the recorded element_stats capture, whose
//! output is identical on llvmpipe and AMD radeonsi and is kept in
//! fixtures/element_stats/native.txt run with DISPLAY=:1). Counting
//! happens while the frame closes, so a recording backend suffices.
#[allow(dead_code)]
#[path = "support/recording_device.rs"]
mod recording_device;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::{
    Graphic3dFrameStatsCounter as Counter, Graphic3dRenderingParamsPerfCounters as Perf, *,
};
use martensite_vulkan::*;
use std::sync::{Arc, Mutex};

const NATIVE: &str = include_str!("fixtures/element_stats/native.txt");

type CallLog = Arc<Mutex<Vec<String>>>;

/// Named fake element overriding both hooks, as the native `CountedElement`.
struct CountedElement {
    calls: CallLog,
}
impl RenderElement for CountedElement {
    fn render(&self, _context: &mut RenderElementContext<'_>) -> Result<(), RenderError> {
        Ok(())
    }
    fn estimated_data_size(&self) -> usize {
        96
    }
    fn update_mem_stats(&self, stats: &mut Graphic3dFrameStatsDataTmp) {
        let geom = stats[Counter::EstimatedBytesGeom];
        self.calls
            .lock()
            .unwrap()
            .push(format!("mem counted geom_before={geom}"));
        stats[Counter::EstimatedBytesGeom] += self.estimated_data_size();
    }
    fn update_draw_stats(&self, stats: &mut Graphic3dFrameStatsDataTmp, detailed: bool) {
        let (elems, tris) = (
            stats[Counter::NbElemsNotCulled],
            stats[Counter::NbTrianglesNotCulled],
        );
        self.calls.lock().unwrap().push(format!(
            "draw counted detailed={} elems_before={elems} tris_before={tris}",
            u8::from(detailed)
        ));
        stats[Counter::NbElemsNotCulled] += 1;
        if detailed {
            stats[Counter::NbTrianglesNotCulled] += 7;
        }
    }
}

/// Named fake element keeping the default hooks, as the native `PlainElement`.
struct PlainElement;
impl RenderElement for PlainElement {
    fn render(&self, _context: &mut RenderElementContext<'_>) -> Result<(), RenderError> {
        Ok(())
    }
    fn estimated_data_size(&self) -> usize {
        40
    }
}

const COUNTERS: [(&str, Counter); 6] = [
    ("structs_not_culled", Counter::NbStructsNotCulled),
    ("groups_not_culled", Counter::NbGroupsNotCulled),
    ("elems_not_culled", Counter::NbElemsNotCulled),
    ("elems_fill_not_culled", Counter::NbElemsFillNotCulled),
    ("triangles_not_culled", Counter::NbTrianglesNotCulled),
    ("bytes_geom", Counter::EstimatedBytesGeom),
];

struct StatsScene {
    viewer_scene: scene::ViewerScene,
    structure: Handle<Graphic3dStructure>,
    calls: CallLog,
}
impl StatsScene {
    /// One group: an opaque triangle array, then the counted and the plain element.
    fn new() -> Self {
        let device = recording_device::RecordingDevice::default();
        let driver = Graphic3dRenderDriver::with_backend(device, 128).unwrap();
        let viewer_scene = scene::ViewerScene::new(&driver);
        viewer_scene.structure.erase();
        let manager = viewer_scene.viewer.structure_manager();
        let structure = Graphic3dStructure::new(&manager, &Handle::default()).unwrap();
        let group = structure.new_group().unwrap();
        group.set_group_primitives_aspect(&scene::unlit_style([1., 0., 0., 1.]));
        group.add_primitive_array(&native_triangle(), true);
        let calls = CallLog::default();
        let draw_group = group.cast::<Graphic3dDrawGroup>();
        let draw_group = draw_group.get().unwrap();
        draw_group.add_element(Arc::new(CountedElement {
            calls: calls.clone(),
        }));
        draw_group.add_element(Arc::new(PlainElement));
        Self {
            viewer_scene,
            structure,
            calls,
        }
    }
    /// The native `show`: collect `flags`, display with `visible`, redraw, print counters.
    fn show(&self, name: &str, flags: Perf, visible: bool) -> Vec<String> {
        self.viewer_scene
            .view
            .change_rendering_params(&mut |params| {
                params.stats.collected_stats = flags;
                params.stats.stats_update_interval = -1.0;
            });
        self.structure.display();
        self.structure.set_visible(visible);
        let view = self.viewer_scene.render_view();
        view.try_redraw().unwrap();
        let mut lines = vec![format!("== {name}")];
        lines.extend(self.calls.lock().unwrap().drain(..));
        let stats = view.frame_stats();
        for (label, counter) in COUNTERS {
            lines.push(format!("counter {label}={}", stats.counter_value(counter)));
        }
        self.structure.set_visible(true);
        self.structure.erase();
        lines
    }
}

/// The native non-indexed triangle with positions only (36 bytes once built).
fn native_triangle() -> Handle<Graphic3dArrayOfPrimitives> {
    let triangle = Handle::new(
        Graphic3dArrayOfPrimitives::new(Graphic3dTypeOfPrimitiveArray::Triangles, 3, 0, 0, 0)
            .unwrap(),
    );
    for [x, y] in [[-1., -1.], [1., -1.], [0., 1.]] {
        triangle.add_vertex(x, y, 0.).unwrap();
    }
    triangle
}

fn native_scene(name: &str) -> Vec<String> {
    let header = format!("== {name}");
    let mut lines = NATIVE.lines().skip_while(|line| *line != header);
    let mut scene = vec![lines.next().expect("native scene").to_string()];
    scene.extend(
        lines
            .take_while(|line| !line.starts_with("== "))
            .map(str::to_string),
    );
    scene
}

#[test]
fn user_element_hooks_add_their_counters_in_native_order() {
    let scene = StatsScene::new();
    assert_eq!(scene.show("all", Perf::ALL, true), native_scene("all"));
}

#[test]
fn group_arrays_call_only_the_draw_hook_without_detail() {
    let scene = StatsScene::new();
    let lines = scene.show("group_arrays", Perf::GROUP_ARRAYS, true);
    assert_eq!(lines, native_scene("group_arrays"));
}

#[test]
fn memory_counting_calls_both_hooks_without_detail() {
    let scene = StatsScene::new();
    let lines = scene.show("memory", Perf::ESTIM_MEM, true);
    assert_eq!(lines, native_scene("memory"));
}

#[test]
fn hidden_structures_call_only_the_memory_hook() {
    let scene = StatsScene::new();
    // As in the native sequence, the array was drawn before, so its buffers are resident.
    scene.show("all", Perf::ALL, true);
    assert_eq!(
        scene.show("hidden", Perf::ALL, false),
        native_scene("hidden")
    );
}
