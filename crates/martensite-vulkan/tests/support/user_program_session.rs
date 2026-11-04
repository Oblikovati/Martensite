//! The viewer sessions of tests/user_program_draws_native.rs: views of SIZE in one
//! viewer, display mode, deflection, lights, texture units, pointer moves, shape restore with
//! cleaned triangulation, and the dumps of the active view with the run's half-step camera shift.
#[path = "object_fill_program.rs"]
mod object_fill_program;
use super::raytrace_scene::SIZE;
use super::user_program_scene::DumpRun;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Direction3;
use martensite_tkservice::aspect::{
    AspectNeutralWindow, AspectTypeOfDeflection, AspectWindowMembers as _,
};
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::ais::*;
use martensite_tkv3d::prs_mgr::*;
use martensite_tkv3d::v3d::*;
use martensite_vulkan::Graphic3dRenderDriver;

/// One viewer with its interactive context and views; the last created or activated view is
/// the active one, as in command-host.
pub struct UserProgramSession {
    pub context: Handle<AisInteractiveContext>,
    viewer: Handle<V3dViewer>,
    views: Vec<(Handle<dyn V3dViewSource>, Handle<AspectNeutralWindow>)>,
    active: usize,
}

impl UserProgramSession {
    /// A 256x256 view on a black background with the default lights.
    pub fn new(driver: &Handle<Graphic3dRenderDriver>) -> Self {
        let viewer = V3dViewer::new(&driver.cast()).unwrap();
        viewer.set_default_background_color(Color::named(NamedColor::BLACK));
        viewer.set_default_lights().unwrap();
        viewer.set_light_on().unwrap();
        let context = AisInteractiveContext::new(&viewer).unwrap();
        let mut session = Self {
            context,
            viewer,
            views: Vec::new(),
            active: 0,
        };
        session.add_view();
        session
    }
    /// A further 256x256 view in the same viewer; the new view becomes active.
    pub fn add_view(&mut self) {
        let window = Handle::new(AspectNeutralWindow::default());
        window.set_size(SIZE.0 as i32, SIZE.1 as i32);
        window.set_virtual(true);
        let view = self.viewer.create_view().unwrap();
        view.set_window(&window.cast(), 0).unwrap();
        self.views.push((view, window));
        self.active = self.views.len() - 1;
    }
    /// Make view `index` (zero-based) active.
    pub fn activate(&mut self, index: usize) {
        self.active = index;
    }
    /// The active view's handle.
    pub fn view_handle(&self) -> &Handle<dyn V3dViewSource> {
        &self.views[self.active].0
    }
    /// The active view.
    pub fn view(&self) -> &V3dView {
        self.view_handle().get().unwrap().view_base()
    }
    /// Shaded display for objects without their own mode.
    pub fn shaded_by_default(&self) {
        self.context.set_default_display_mode(1, false).unwrap();
    }
    /// Absolute deflection `value`.
    pub fn absolute_deflection(&self, value: f64) {
        let drawer = self.context.default_drawer();
        drawer.set_type_of_deflection(AspectTypeOfDeflection::Absolute);
        drawer.set_maximal_chordial_deviation(value);
    }
    /// Relative deflection coefficient `value`.
    pub fn relative_deflection(&self, value: f64) {
        let drawer = self.context.default_drawer();
        drawer.set_type_of_deflection(AspectTypeOfDeflection::Relative);
        drawer.set_deviation_coefficient(value);
    }
    /// Axonometric projection.
    pub fn axo(&self) {
        let orientation = V3dTypeOfOrientation::XposYnegZpos;
        self.view()
            .set_proj_orientation(orientation, false)
            .unwrap();
    }
    /// Display `shape` in the context's display mode.
    pub fn display(&self, shape: &Shape) -> Handle<dyn AisInteractiveObject> {
        let object: Handle<dyn AisInteractiveObject> = AisShape::new(shape).cast();
        self.context.display(&object, false).unwrap();
        object
    }
    /// Fit the view.
    pub fn fit(&self) {
        self.view().fit_all(0.01, false).unwrap();
    }
    /// Rotate the view by `ax` about X.
    pub fn rotate(&self, ax: f64) {
        self.view().rotate(ax, 0., 0., true).unwrap();
    }
    /// Move the pointer to (x, y) in the active view.
    pub fn move_to(&self, x: i32, y: i32) {
        let view = self.view_handle();
        self.context.move_to(x, y, view, true).unwrap();
    }
    /// Erase every displayed object.
    pub fn clear(&self) {
        self.context.remove_all(false).unwrap();
    }
    /// The active view with `run`'s camera shift, restored afterwards.
    pub fn dump(&self, run: &DumpRun) -> Vec<u8> {
        let shift = run.shift;
        super::shift_control::shift_camera(self.view(), shift, SIZE.0);
        let rgb = run.dump_view(self.view_handle());
        super::shift_control::shift_camera(self.view(), [-shift[0], -shift[1]], SIZE.0);
        rgb
    }
    /// Remove the lights, then add a white ambient light and two directional headlights:
    /// green along +X and red along -X (phong_fuse2).
    pub fn green_red_headlights(&self) {
        let view = self.view();
        for light in view.active_lights() {
            self.viewer.del_light(&light).unwrap();
        }
        let ambient = V3dAmbientLight::new(Color::named(NamedColor::WHITE));
        self.add_light(Handle::new(ambient).cast());
        for (direction, color) in [(1., NamedColor::GREEN), (-1., NamedColor::RED1)] {
            let light = V3dDirectionalLight::new(
                V3dTypeOfOrientation::XposYposZpos,
                Color::named(color),
                false,
            )
            .unwrap();
            let light: Handle<Graphic3dCLight> = Handle::new(light).cast();
            let value = light.get().unwrap();
            value
                .set_direction(&Direction3::new(direction, 0., 0.).unwrap())
                .unwrap();
            value.set_headlight(true).unwrap();
            self.add_light(light);
        }
    }
    fn add_light(&self, light: Handle<Graphic3dCLight>) {
        self.viewer.add_light(&light);
        self.viewer.set_light_on_one(&light).unwrap();
    }
}

/// One 2D texture per unit from `files`, applied to the
/// object's own shading aspect, with texture coordinates computed by a redisplay.
pub fn apply_texture_units(
    context: &AisInteractiveContext,
    object: &Handle<dyn AisInteractiveObject>,
    files: &[&str],
) {
    use Graphic3dTextureUnit::{Unit0, Unit1, Unit2, Unit3};
    let textures = files
        .iter()
        .zip([Unit0, Unit1, Unit2, Unit3])
        .map(|(file, unit)| {
            let texture = Graphic3dTexture2D::from_file(&(*file).into());
            texture.get_params().set_texture_unit(unit);
            Handle::new(texture).cast()
        });
    apply_texture_set(context, object, &textures.collect::<Vec<_>>());
}

/// `textures` as the texture set of the object's own shading aspect, with
/// texture mapping on, then the redisplay; `apply_texture_set(&context, &object, &[texture])`.
pub fn apply_texture_set(
    context: &AisInteractiveContext,
    object: &Handle<dyn AisInteractiveObject>,
    textures: &[Handle<dyn Graphic3dTextureMap>],
) {
    let set = Graphic3dTextureSet::with_size(textures.len() as i32).unwrap();
    for (index, texture) in textures.iter().enumerate() {
        set.set_value(index as i32, texture.clone()).unwrap();
    }
    let value = object.get().unwrap();
    let drawer = value.attributes();
    let drawer = drawer.get().unwrap();
    drawer
        .setup_own_shading_aspect(&context.default_drawer())
        .unwrap();
    let aspect = drawer.shading_aspect().get().unwrap().aspect();
    let aspect = aspect.get().unwrap();
    aspect.set_texture_map_on(true);
    aspect.set_texture_set(Handle::new(set));
    value.set_to_update(1);
    context.display(object, false).unwrap();
    value.synchronize_aspects();
}

/// A shape read from `bytes` without its triangulation.
pub fn restored_without_mesh(bytes: &[u8]) -> Shape {
    let shape = super::dataset_shape::read_shape_bytes(bytes).unwrap();
    martensite_tkbrep::brep_tools::clean(&shape, false).unwrap();
    shape
}

/// The faces of `shape` in exploration order.
pub fn faces(shape: &Shape) -> Vec<Shape> {
    use martensite_tkbrep::topexp::Explorer;
    use martensite_tkg3d::top_abs::ShapeEnum;
    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    let mut faces = Vec::new();
    while explorer.more() {
        faces.push(explorer.current().clone());
        explorer.next().unwrap();
    }
    faces
}

/// `program` (null to clear it) on the fill-area aspects of
/// `objects`, redisplaying an object whose aspects changed.
pub fn set_fill_program(
    context: &AisInteractiveContext,
    objects: &[&Handle<dyn AisInteractiveObject>],
    program: impl Fn() -> Handle<Graphic3dShaderProgram>,
) {
    object_fill_program::assign(context, objects, program).unwrap();
}
