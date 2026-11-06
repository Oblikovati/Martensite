//! A real initialized view may precede modelling, without placeholder geometry.
use super::*;
use martensite_tkernel::standard::Failure;

/// Invocation-local UI handles; render commands stay on the worker thread owning the view.
/// Example: `let stage = HatchSourceView::new(&driver)?; let scene = stage.with_shape(shape);`.
pub struct HatchSourceView {
    viewer: Handle<V3dViewer>,
    window: Handle<AspectNeutralWindow>,
    context: Handle<AisInteractiveContext>,
    view: Handle<dyn V3dViewSource>,
}

impl HatchSourceView {
    /// Apply the exact named-object operation through this actual context; `stage.source_default_object_shading(object)?`.
    pub(super) fn source_default_object_shading(
        &self,
        object: &Handle<AisShape>,
    ) -> Result<(), Failure> {
        default_shading::source_default_shading(&self.context, object)
    }

    /// Keep the bare command separate from RAII-updated commands; `stage.source_global_display_mode(1)?`.
    pub(super) fn source_global_display_mode(&self, mode: i32) -> Result<(), Failure> {
        let requested: i32 = mode;
        bare_global_mode::source_bare_global_mode(&self.context, requested)
    }

    /// Run one context command with its native updater; `stage.source_context_command(|context| context.set_default_display_mode(1, false))?`.
    pub(super) fn source_context_command(
        &self,
        operation: impl FnOnce(&AisInteractiveContext) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        view_output::with_source_view_update(
            &*self.view,
            || operation(&self.context),
            || self.context.update_current_viewer(),
        )
    }

    /// Observe actual context state without a command updater; `stage.source_context_query(|context| context.display_mode())`.
    pub(super) fn source_context_query<T>(
        &self,
        query: impl FnOnce(&AisInteractiveContext) -> T,
    ) -> T {
        // Queries must not introduce a source command's redraw/update boundary.
        query(&self.context)
    }

    /// Execute actual initialization before a shape exists; `HatchSourceView::new(&driver)?`.
    pub fn new(driver: &Handle<Graphic3dRenderDriver>) -> Result<Self, Failure> {
        capture_artifacts::record_identity(&driver.driver_identity());
        let viewer = source_viewer(driver)?;
        let context = AisInteractiveContext::new(&viewer)?;
        let window = source_window();
        let view = viewer.create_view()?;
        source_connect_view(&viewer, &context, &window, &view)?;
        Ok(Self {
            viewer,
            window,
            context,
            view,
        })
    }

    /// Assemble real CPU fixture owners without creating a second backend; `Self::mode_fixture_owners(v, c, w, view)`.
    #[cfg(test)]
    // global_mode_command_fixture.rs:205 uses this constructor; other compiled instances of this shared fixture do not.
    #[allow(dead_code)]
    pub(crate) fn mode_fixture_owners(
        viewer: Handle<V3dViewer>,
        context: Handle<AisInteractiveContext>,
        window: Handle<AspectNeutralWindow>,
        view: Handle<dyn V3dViewSource>,
    ) -> Self {
        Self {
            viewer,
            context,
            window,
            view,
        }
    }

    /// Execute a source command with its real context updater; `stage.source_update(|view| { view.redraw(); Ok(()) })?`.
    pub fn source_update(
        &self,
        operation: impl FnOnce(&dyn V3dViewSource) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        view_output::with_source_view_update(
            &*self.view,
            || operation(&*self.view),
            || self.context.update_current_viewer(),
        )
    }

    /// Clear only this fresh, still-unbound AIS namespace; `stage.source_clear()?` before `with_shape`.
    /// Nonempty contexts are refused rather than pretending to clear untracked named objects.
    pub fn source_clear(&self) -> Result<(), Failure> {
        let mut objects = martensite_tkernel::ncollection::List::default();
        self.context
            .objects_inside(&mut objects, AisKindOfInteractive::None, -1)?;
        if !objects.is_empty() {
            return Err(martensite_tkernel::standard::exceptions::program_error(
                &format!(
                    "registered_objects={}; expected fresh empty AIS namespace",
                    objects.extent()
                ),
            ));
        }
        // ViewerCommands::Clear rebuilds and updates even when its named-object map is empty.
        self.context.rebuild_selection_structs()?;
        self.context.update_current_viewer();
        Ok(())
    }

    /// Attach the genuine source shape without displaying it; `stage.with_shape(shape)`.
    pub fn with_shape(self, shape: Shape) -> HatchBoxScene {
        HatchBoxScene {
            _viewer: self.viewer,
            _window: self.window,
            _context: self.context,
            object: AisShape::new(&shape),
            view: self.view,
        }
    }
}

#[cfg(test)]
#[path = "hatch_source_clear_controls.rs"]
mod source_clear_controls;

/// The black-background viewer every regression view belongs to; `source_viewer(&driver)?`.
pub(super) fn source_viewer(
    driver: &Handle<Graphic3dRenderDriver>,
) -> Result<Handle<V3dViewer>, Failure> {
    source_viewer_of(&driver.cast())
}

/// A black-background viewer of any graphic driver, such as the driver of an existing viewer
/// (a second viewer and view); `source_viewer_of(&viewer.driver())?`.
pub(super) fn source_viewer_of(
    driver: &Handle<dyn Graphic3dGraphicDriver>,
) -> Result<Handle<V3dViewer>, Failure> {
    let viewer = V3dViewer::new(driver)?;
    let black = Color::named(NamedColor::BLACK);
    viewer.set_default_background_color(black);
    viewer.set_default_bg_gradient_colors(black, black, AspectGradientFillMethod::None);
    Ok(viewer)
}

fn source_window() -> Handle<AspectNeutralWindow> {
    source_window_sized(409, 409)
}

/// A virtual window of the requested size; `source_window_sized(200, 400)`.
pub(super) fn source_window_sized(width: i32, height: i32) -> Handle<AspectNeutralWindow> {
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_size(width, height);
    window.set_virtual(true);
    window
}

/// Attach `window` to `view` exactly as the first view is attached; `source_connect_view(...)?`.
pub(super) fn source_connect_view(
    viewer: &Handle<V3dViewer>,
    context: &Handle<AisInteractiveContext>,
    window: &Handle<AspectNeutralWindow>,
    view: &Handle<dyn V3dViewSource>,
) -> Result<(), Failure> {
    view.get().unwrap().view().set_subview_composer(false);
    view.set_window(&window.cast(), 0)?;
    context.redraw_immediate(viewer)?;
    view.set_computed_mode(false);
    viewer.set_default_background_color(Color::named(NamedColor::BLACK));
    viewer.set_default_lights()?;
    viewer.set_light_on()?;
    window.map();
    view.redraw();
    Ok(())
}

impl SourceViewOutput for HatchSourceView {
    fn source_view_handle(&self) -> &Handle<dyn V3dViewSource> {
        // The initialized view exists independently of any future model attachment.
        &self.view
    }

    fn source_window_size(&self) -> (i32, i32) {
        // Read the retained real window; no placeholder shape supplies dimensions.
        self.window.size()
    }
}

#[path = "hatch_source_view_commands.rs"]
mod source_commands;
