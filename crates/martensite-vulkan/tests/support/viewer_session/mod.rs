//! One viewer, its interactive context and every view a regression opens, in command order.
//!
//! Operations are named after the toolkit behaviour they perform (display, fit, zoom, grid,
//! read a pixel), each wrapped in the same immediate-update boundary the interactive viewer
//! applies after a user command.
use super::*;
use martensite_tkernel::standard::{Failure, catch_native_failure};
use view_output::with_source_view_update;

mod appearance;
mod aspects;
mod camera;
mod color_check;
mod color_scale;
mod connected;
mod context_access;
mod deferred_updates;
mod depth_range;
mod display;
mod entity_state_response;
mod face_boundary;
mod frame_counters;
mod free_boundary;
mod gestures;
mod grid;
mod grid_shader;
mod hidden_lines;
mod highlight_style;
mod lighting_capabilities;
#[cfg(test)]
pub(crate) mod lighting_capability_test_support;
mod lights;
mod location;
mod locations;
mod manipulator;
mod markers;
mod material_rendering;
mod named_objects;
mod objects;
mod performance;
mod persistence;
mod picked;
mod picked_shapes;
mod pixels;
mod presentation_defaults;
mod projection;
mod ray_tracing;
mod relations;
mod rendering;
mod selection;
mod selection_actions;
mod selection_dump;
mod selection_filters;
mod sub_shape_aspects;
mod text_labels;
mod textures;
mod tutorial_animation;
mod view_cube;
mod view_parameters;
mod view_trihedron;
mod views;
mod visibility;
mod z_layers;
pub use aspects::SurfaceAspects;
pub use connected::ConnectedPart;
pub use display::{DisplayRequest, PersistenceAnchor};
pub use face_boundary::FaceBoundary;
pub use free_boundary::FreeBoundary;
pub use grid::{GridLayout, GridRequest};
pub use grid_shader::ShaderGridRequest;
pub use highlight_style::HighlightStyle;
pub use lights::LightPresentationSetup;
pub use locations::LocationChange;
pub use manipulator::{
    ManipulatorCommand, ManipulatorDrag, ManipulatorFrame, ManipulatorPart, ManipulatorPlacement,
};
pub use performance::{FpsObservation, PerformanceFailure};
pub use persistence::corner_persistence;
pub use relations::RelationKind;
pub use sub_shape_aspects::SubShapeStyle;
pub use text_labels::{TextLabelRequest, register_font_file, register_font_file_with};
pub use textures::TextureImage;
pub use view_parameters::ViewParameters;
pub use view_trihedron::ViewTrihedronSettings;

/// Invocation-local UI owners; they stay on the worker thread that created them.
/// Example: `let mut session = ViewerSession::new(&driver)?; session.fit()?;`.
pub struct ViewerSession {
    viewer: Handle<V3dViewer>,
    context: Handle<AisInteractiveContext>,
    views: Vec<(Handle<AspectNeutralWindow>, Handle<dyn V3dViewSource>)>,
    active: usize,
    objects: named_objects::NamedObjects,
    /// Pointer input of the interactive viewer: cursor detection and picking.
    controller: AisViewController,
    /// Temporary dump selection belongs to this invocation-local UI owner.
    stereo_dump: std::cell::Cell<V3dStereoDumpOptions>,
    /// Earlier viewers and their contexts must survive while their views remain retained.
    earlier_viewers: Vec<views::RetainedViewerOwner>,
}

impl ViewerSession {
    /// Create the viewer, context and first 409x409 view; `ViewerSession::new(&driver)?`.
    pub fn new(driver: &Handle<Graphic3dRenderDriver>) -> Result<Self, Failure> {
        Self::with_view_size(driver, 409, 409)
    }

    /// Create the viewer, context and a first view of the given size; `with_view_size(&d, 400, 400)?`.
    pub fn with_view_size(
        driver: &Handle<Graphic3dRenderDriver>,
        width: i32,
        height: i32,
    ) -> Result<Self, Failure> {
        capture_artifacts::record_identity(&driver.driver_identity());
        Self::with_graphic_driver(&driver.cast(), width, height)
    }

    /// Build a session with the existing driver interface; `with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409)?`.
    pub fn with_graphic_driver(
        driver: &Handle<dyn Graphic3dGraphicDriver>,
        width: i32,
        height: i32,
    ) -> Result<Self, Failure> {
        let viewer = source_view::source_viewer_of(driver)?;
        let mut session = Self::for_viewer(viewer)?;
        session.open_view(width, height)?;
        Ok(session)
    }

    fn for_viewer(viewer: Handle<V3dViewer>) -> Result<Self, Failure> {
        let context = AisInteractiveContext::new(&viewer)?;
        Ok(Self {
            viewer,
            context,
            views: Vec::new(),
            active: 0,
            objects: named_objects::NamedObjects::default(),
            controller: pointer_controller()?,
            stereo_dump: std::cell::Cell::new(V3dStereoDumpOptions::Mono),
            earlier_viewers: Vec::new(),
        })
    }

    /// Open another view of the same viewer and make it active; `session.open_view(200, 400)?`.
    pub fn open_view(&mut self, width: i32, height: i32) -> Result<(), Failure> {
        let window = source_view::source_window_sized(width, height);
        let view = self.viewer.create_view()?;
        source_view::source_connect_view(&self.viewer, &self.context, &window, &view)?;
        self.views.push((window, view));
        self.active = self.views.len() - 1;
        Ok(())
    }

    /// Remove every named object from the context and forget its name; `session.clear()?`.
    pub fn clear(&mut self) -> Result<(), Failure> {
        for (_, object) in self.objects.drain() {
            self.context.remove(&object, false)?;
        }
        // An empty named-object map still rebuilds selection and updates the viewer.
        self.context.rebuild_selection_structs()?;
        self.context.update_current_viewer();
        Ok(())
    }

    /// Fit all displayed objects with a 1% margin; `session.fit()?`.
    pub fn fit(&self) -> Result<(), Failure> {
        let view = self.view();
        with_source_view_update(&**view, || view.fit_all(0.01, false), || view.redraw())
    }

    /// Axonometric Z-up projection, which pans and updates on its own; `session.set_axonometric()?`.
    pub fn set_axonometric(&self) -> Result<(), Failure> {
        catch_native_failure(|| {
            self.view()
                .set_proj_orientation(V3dTypeOfOrientation::XposYnegZpos, false)
        })
    }

    /// Scale the current zoom by `factor`; `session.zoom(0.2)?`.
    pub fn zoom(&self, factor: f64) -> Result<(), Failure> {
        catch_native_failure(|| self.view().view_base().set_zoom(factor, true))
    }

    /// Change the context's default display mode for displayed objects; `session.set_display_mode(1)?`.
    pub fn set_display_mode(&self, mode: i32) -> Result<(), Failure> {
        // The global mode keeps the same bare context call and updater as single-view regressions.
        bare_global_mode::source_bare_global_mode(&self.context, mode)
    }

    /// The active view; `session.view().redraw()`.
    pub fn view(&self) -> &Handle<dyn V3dViewSource> {
        &self.views[self.active].1
    }

    fn with_update(
        &self,
        operation: impl FnOnce(&AisInteractiveContext) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        let view = self.view();
        with_source_view_update(
            &**view,
            || operation(&self.context),
            || self.context.update_current_viewer(),
        )
    }
}

/// The pointer controller with the half-second camera animation of the interactive viewer.
fn pointer_controller() -> Result<AisViewController, Failure> {
    let mut controller = AisViewController::new()?;
    let animation = AisAnimationCamera::new("ViewerSession_ViewAnimation", &Handle::default());
    animation.set_own_duration(0.5);
    controller.set_view_animation(&Handle::new(animation));
    Ok(controller)
}

impl SourceViewOutput for ViewerSession {
    fn source_view_handle(&self) -> &Handle<dyn V3dViewSource> {
        // Image dumps read the view that is active at the time of the dump.
        self.view()
    }

    fn source_window_size(&self) -> (i32, i32) {
        self.views[self.active].0.size()
    }

    fn source_dump(
        &self,
        image: &martensite_tkservice::image::ImagePixMap,
    ) -> Result<bool, Failure> {
        view_output::dump_view(
            self.view(),
            self.source_window_size(),
            self.stereo_dump.get(),
            image,
        )
    }

    fn close(&self) {
        // Retire every view the regression opened, after its END.
        for (_, view) in &self.views {
            view.remove();
        }
    }
}

#[cfg(test)]
#[path = "viewer_ownership_contract.rs"]
mod viewer_ownership_contract;

mod subview_binding;
