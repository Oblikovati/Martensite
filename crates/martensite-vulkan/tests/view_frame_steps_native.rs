//! The protected virtual frame steps of GlView (redraw, redrawImmediate, render,
//! renderScene, renderStructs) as overridable Vulkan view hooks.
//!
//! The expected order comes from the recorded view_frame_steps capture,
//! whose output is identical on llvmpipe and AMD radeonsi (fixtures/view_frame_steps/native.txt; run with DISPLAY=:1). A Vulkan view keeps no back buffer between frames,
//! so an immediate redraw takes the native fallback branch of `RedrawImmediate` (a full
//! `Redraw`) and records the native `redraw` sequence.
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
use martensite_tkservice::graphic3d::*;
use martensite_vulkan::*;
use std::sync::{Arc, Mutex};

const NATIVE: &str = include_str!("fixtures/view_frame_steps/native.txt");

/// Named fake override: records each step, then runs the native base body.
struct RecordingHooks {
    calls: Arc<Mutex<Vec<String>>>,
    skip_immediate_structs: bool,
}
impl RecordingHooks {
    fn note(&self, name: &str, step: &RenderFrameStep<'_>) {
        let projection = step.projection() as i32;
        let line = match name {
            "redraw" => format!("step redraw projection={projection}"),
            "redrawImmediate" => format!(
                "step redrawImmediate projection={projection} partial={}",
                u8::from(step.is_partial_update())
            ),
            _ => format!(
                "step {name} projection={projection} immediate={}",
                u8::from(step.to_draw_immediate())
            ),
        };
        self.calls.lock().unwrap().push(line);
    }
}
impl RenderViewFrameHooks for RecordingHooks {
    fn redraw(
        &self,
        view: &Graphic3dRenderView,
        step: &mut RenderFrameStep<'_>,
    ) -> Result<(), RenderError> {
        self.note("redraw", step);
        view.redraw_base(step)
    }
    fn redraw_immediate(
        &self,
        view: &Graphic3dRenderView,
        step: &mut RenderFrameStep<'_>,
    ) -> Result<(), RenderError> {
        self.note("redrawImmediate", step);
        view.redraw_immediate_base(step)
    }
    fn render(
        &self,
        view: &Graphic3dRenderView,
        step: &mut RenderFrameStep<'_>,
    ) -> Result<(), RenderError> {
        self.note("render", step);
        view.render_base(step)
    }
    fn render_scene(
        &self,
        view: &Graphic3dRenderView,
        step: &mut RenderFrameStep<'_>,
    ) -> Result<(), RenderError> {
        self.note("renderScene", step);
        view.render_scene_base(step)
    }
    fn render_structs(
        &self,
        view: &Graphic3dRenderView,
        step: &mut RenderFrameStep<'_>,
    ) -> Result<(), RenderError> {
        self.note("renderStructs", step);
        if self.skip_immediate_structs && step.to_draw_immediate() {
            return Ok(());
        }
        view.render_structs_base(step)
    }
}

fn native_scenario(name: &str) -> Vec<String> {
    let header = format!("== {name}");
    NATIVE
        .lines()
        .skip_while(|line| *line != header)
        .skip(1)
        .take_while(|line| line.starts_with("step "))
        .map(str::to_string)
        .collect()
}

struct StepScene {
    scene: scene::ViewerScene,
    device: recording_device::RecordingDevice,
    calls: Arc<Mutex<Vec<String>>>,
}
impl StepScene {
    fn new(skip_immediate_structs: bool) -> Self {
        let device = recording_device::RecordingDevice::default();
        let driver = Graphic3dRenderDriver::with_backend(device.clone(), 128).unwrap();
        let scene = scene::ViewerScene::new(&driver);
        let calls: Arc<Mutex<Vec<String>>> = Arc::default();
        let hooks = RecordingHooks {
            calls: calls.clone(),
            skip_immediate_structs,
        };
        scene.render_view().set_frame_hooks(Some(Arc::new(hooks)));
        Self {
            scene,
            device,
            calls,
        }
    }
    fn record(&self, action: impl FnOnce(&Handle<dyn Graphic3dCView>)) -> Vec<String> {
        self.calls.lock().unwrap().clear();
        action(&self.scene.cview());
        self.calls.lock().unwrap().drain(..).collect()
    }
    fn display_in_top_layer(&self) {
        let manager = self.scene.viewer.structure_manager();
        let top = Graphic3dStructure::new(&manager, &Handle::default()).unwrap();
        let group = top.new_group().unwrap();
        group.set_group_primitives_aspect(&scene::unlit_style([0., 1., 0., 1.]));
        group.add_primitive_array(&view_setup::triangle_vertices(), true);
        top.set_z_layer(GRAPHIC3D_ZLAYER_ID_TOP);
        top.display();
    }
}

#[test]
fn redraw_runs_the_native_step_sequence() {
    let steps = StepScene::new(false);
    assert_eq!(
        steps.record(|view| view.redraw()),
        native_scenario("redraw")
    );
    steps.display_in_top_layer();
    let with_top = steps.record(|view| view.redraw());
    assert_eq!(with_top, native_scenario("redraw_with_immediate_layer"));
}

#[test]
fn immediate_redraw_takes_the_native_full_redraw_fallback() {
    let steps = StepScene::new(false);
    let immediate = steps.record(|view| view.redraw_immediate());
    assert_eq!(immediate, native_scenario("redraw"));
}

#[test]
fn an_override_can_drop_the_immediate_structures() {
    let base = StepScene::new(false);
    base.display_in_top_layer();
    base.scene.render_view().try_redraw().unwrap();
    let drawn = base.device.last_frame().draws().len();
    let skipping = StepScene::new(true);
    skipping.display_in_top_layer();
    skipping.scene.render_view().try_redraw().unwrap();
    let kept = skipping.device.last_frame().draws().len();
    assert!(kept > 0 && kept < drawn, "received {kept} of {drawn} draws");
}
