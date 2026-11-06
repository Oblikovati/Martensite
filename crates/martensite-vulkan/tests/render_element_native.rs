//! User render elements (GlElement + GlGroup::AddElement) observe the native
//! GlWorkspace state during an ordinary redraw.
//!
//! Expected lines come from the recorded render_element capture, whose
//! output is identical on llvmpipe and AMD radeonsi and is kept in
//! fixtures/render_element/native.txt run with DISPLAY=:1). The scene
//! state an element sees is decided while the frame is built, so a recording backend suffices.
#[allow(dead_code)]
#[path = "support/recording_device.rs"]
mod recording_device;
#[allow(dead_code)]
#[path = "support/viewer_scene.rs"]
mod scene;
#[allow(dead_code)]
#[path = "support/view_setup.rs"]
mod view_setup;
use martensite_tkernel::{
    ncollection::Vec4,
    quantity::{Color, ColorSpace},
    standard::Handle,
};
use martensite_tkservice::{aspect::AspectInteriorStyle, graphic3d::*};
use martensite_vulkan::*;
use std::sync::{Arc, Mutex};

const NATIVE: &str = include_str!("fixtures/render_element/native.txt");

/// Named fake element: records what the workspace exposes on each call.
struct RecordingElement {
    name: &'static str,
    fill: bool,
    calls: Arc<Mutex<Vec<String>>>,
}
impl RenderElement for RecordingElement {
    fn render(&self, context: &mut RenderElementContext<'_>) -> Result<(), RenderError> {
        let color = context.interior_color();
        let rgb = color.rgb();
        let line = format!(
            "call {} filter={} highlight={} culling={} zbuffer={} depth_write={} skipped={} interior={:.3},{:.3},{:.3},{:.3}",
            self.name,
            context.render_filter().bits(),
            u8::from(context.to_highlight()),
            u8::from(context.to_allow_face_culling()),
            u8::from(context.use_z_buffer()),
            u8::from(context.use_depth_write()),
            context.nb_skipped_transparent_elements(),
            rgb.red(),
            rgb.green(),
            rgb.blue(),
            color.alpha()
        );
        self.calls.lock().unwrap().push(line);
        Ok(())
    }
    fn is_fill_draw_mode(&self) -> bool {
        self.fill
    }
}

struct ElementScene {
    viewer_scene: scene::ViewerScene,
    calls: Arc<Mutex<Vec<String>>>,
}
impl ElementScene {
    fn new() -> Self {
        let device = recording_device::RecordingDevice::default();
        let driver = Graphic3dRenderDriver::with_backend(device, 128).unwrap();
        let viewer_scene = scene::ViewerScene::new(&driver);
        viewer_scene.structure.erase();
        Self {
            viewer_scene,
            calls: Arc::default(),
        }
    }
    /// A structure whose group has the native opaque red aspect and one triangle.
    fn structure(&self) -> (Handle<Graphic3dStructure>, Handle<Graphic3dDrawGroup>) {
        let manager = self.viewer_scene.viewer.structure_manager();
        let structure = Graphic3dStructure::new(&manager, &Handle::default()).unwrap();
        let group = structure.new_group().unwrap();
        group.set_group_primitives_aspect(&fill_aspect([1., 0., 0.], 0.));
        add_triangle(&group);
        (structure, group.cast())
    }
    fn element(&self, group: &Handle<Graphic3dDrawGroup>, name: &'static str, fill: bool) {
        let calls = self.calls.clone();
        let element = RecordingElement { name, fill, calls };
        group.get().unwrap().add_element(Arc::new(element));
    }
    fn show(&self, name: &str, structure: &Handle<Graphic3dStructure>) -> Vec<String> {
        self.calls.lock().unwrap().clear();
        structure.display();
        self.viewer_scene.render_view().try_redraw().unwrap();
        structure.erase();
        let mut lines = vec![format!("== {name}")];
        lines.extend(self.calls.lock().unwrap().drain(..));
        lines
    }
}

fn fill_aspect(rgb: [f64; 3], transparency: f32) -> Handle<dyn Graphic3dAspectsSource> {
    let style = Handle::new(Graphic3dAspects::default());
    style.set_interior_style(AspectInteriorStyle::Solid);
    style.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    style.set_interior_color(Color::new(rgb[0], rgb[1], rgb[2], ColorSpace::Rgb).unwrap());
    let mut material = style.front_material();
    material.set_transparency(transparency).unwrap();
    style.set_front_material(material.clone());
    style.set_back_material(material);
    style.cast()
}

fn add_triangle(group: &Handle<dyn Graphic3dGroup>) {
    let triangle = Handle::new(
        Graphic3dArrayOfPrimitives::new(Graphic3dTypeOfPrimitiveArray::Triangles, 3, 0, 0, 0)
            .unwrap(),
    );
    for [x, y] in [[-1., -1.], [1., -1.], [0., 1.]] {
        triangle.add_vertex(x, y, 0.).unwrap();
    }
    group.add_primitive_array(&triangle, true);
}

fn native_scene(name: &str) -> Vec<String> {
    let header = format!("== {name}");
    let mut lines = NATIVE.lines().skip_while(|line| *line != header);
    let mut scene = vec![lines.next().expect("native scene").to_string()];
    scene.extend(
        lines
            .take_while(|line| line.starts_with("call "))
            .map(str::to_string),
    );
    scene
}

#[test]
fn plain_closed_and_clipped_groups_report_filter_and_culling() {
    let scene = ElementScene::new();
    let (plain, group) = scene.structure();
    scene.element(&group, "line", false);
    scene.element(&group, "fill", true);
    assert_eq!(scene.show("plain", &plain), native_scene("plain"));
    let (closed, group) = scene.structure();
    group.get().unwrap().set_closed(true);
    scene.element(&group, "closed", true);
    assert_eq!(scene.show("closed", &closed), native_scene("closed"));
    let (clipped, group) = scene.structure();
    group.get().unwrap().set_closed(true);
    scene.element(&group, "clipped", true);
    let planes = Handle::new(Graphic3dSequenceOfHClipPlane::default());
    // Native gpPln(origin, +X) has the equation (1, 0, 0, 0).
    let plane = Graphic3dClipPlane::from_equation(Vec4::new(1., 0., 0., 0.)).unwrap();
    planes.append(Handle::new(plane));
    clipped.set_clip_planes(&planes);
    assert_eq!(scene.show("clipped", &clipped), native_scene("clipped"));
}

#[test]
fn highlighted_structure_exposes_its_style_color() {
    let scene = ElementScene::new();
    let (structure, group) = scene.structure();
    scene.element(&group, "highlighted", true);
    let style = Handle::new(Graphic3dPresentationAttributes::default());
    style.set_color(Color::new(0., 1., 0., ColorSpace::Rgb).unwrap());
    structure.display();
    structure.highlight(&style.cast(), true);
    assert_eq!(
        scene.show("highlight", &structure),
        native_scene("highlight")
    );
}

#[test]
fn transparent_fill_elements_are_skipped_then_drawn_in_the_transparent_pass() {
    let scene = ElementScene::new();
    let (structure, group) = scene.structure();
    let inner: Handle<dyn Graphic3dGroup> = group.clone().cast();
    inner.set_primitives_aspect(&fill_aspect([0., 0., 1.], 0.5));
    scene.element(&group, "transparent_fill", true);
    scene.element(&group, "transparent_line", false);
    assert_eq!(
        scene.show("transparent", &structure),
        native_scene("transparent")
    );
}

#[test]
fn skipped_counter_includes_builtin_transparent_arrays() {
    let scene = ElementScene::new();
    let (structure, group) = scene.structure();
    let inner: Handle<dyn Graphic3dGroup> = group.clone().cast();
    inner.set_primitives_aspect(&fill_aspect([0., 0., 1.], 0.5));
    add_triangle(&inner);
    inner.set_primitives_aspect(&fill_aspect([1., 1., 0.], 0.));
    scene.element(&group, "after_builtin", true);
    assert_eq!(scene.show("builtin", &structure), native_scene("builtin"));
}

#[test]
fn layer_depth_write_setting_reaches_the_element() {
    let scene = ElementScene::new();
    let mut settings = Graphic3dZLayerSettings::default();
    settings.set_enable_depth_write(false);
    let layer = scene
        .viewer_scene
        .viewer
        .add_z_layer(&settings)
        .unwrap()
        .unwrap();
    let (structure, group) = scene.structure();
    structure.set_z_layer(layer);
    scene.element(&group, "no_depth_write", true);
    assert_eq!(
        scene.show("depth_write", &structure),
        native_scene("depth_write")
    );
}

/// Named fake element: counts aspect synchronizations and reports a fixed retained size.
#[derive(Default)]
struct SizedElement {
    synchronized: std::sync::atomic::AtomicUsize,
}
impl RenderElement for SizedElement {
    fn render(&self, _: &mut RenderElementContext<'_>) -> Result<(), RenderError> {
        Ok(())
    }
    fn estimated_data_size(&self) -> usize {
        96
    }
    fn synchronize_aspects(&self) {
        self.synchronized
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

#[test]
fn group_synchronization_reaches_elements_and_defaults_follow_native() {
    let scene = ElementScene::new();
    // Keep the structure alive: a group of a released structure is deleted and ignores edits.
    let (_structure, group) = scene.structure();
    let element = Arc::new(SizedElement::default());
    group.get().unwrap().add_element(element.clone());
    group.get().unwrap().synchronize_aspects();
    assert_eq!(
        element
            .synchronized
            .load(std::sync::atomic::Ordering::Relaxed),
        1
    );
    assert!(!element.is_fill_draw_mode());
    let mut stats = Graphic3dFrameStatsDataTmp::default();
    element.update_mem_stats(&mut stats);
    element.update_draw_stats(&mut stats, true);
    // GlElement::UpdateMemStats adds EstimatedDataSize to EstimatedBytesGeom only.
    assert_eq!(stats[Graphic3dFrameStatsCounter::EstimatedBytesGeom], 96);
    assert_eq!(stats[Graphic3dFrameStatsCounter::NbElemsNotCulled], 0);
}
