//! Named drivers, viewers and views for document tests, on virtual windows and the Vulkan driver.
//!
//! The tests assert what the registry holds after a document or a view closes, so it stays explicit
//! instead of a bare view. Window placement and titles have no effect on a virtual window and are
//! not kept.
use martensite_tkernel::ncollection::{DoubleMap, Vec2};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::aspect::{
    ASPECT_VKEY_FLAGS_NONE, ASPECT_VKEY_MOUSE_NONE, AspectDisplayConnection,
    AspectGradientFillMethod, AspectNeutralWindow, AspectWindow, AspectWindowMembers as _,
};
use martensite_tkservice::graphic3d::{
    Graphic3dGraphicDriver, Graphic3dGraphicDriverFactory,
    graphic3d_graphic_driver_factory_default, graphic3d_graphic_driver_factory_register,
};
use martensite_tkv3d::ais::{
    AisAnimationCamera, AisAnimationMembers as _, AisInteractiveContext, AisInteractiveObject,
    AisInteractiveObjectMembers as _, AisViewController,
};
use martensite_tkv3d::select_mgr::SelectMgrViewerSelectorResultMembers as _;
use martensite_tkv3d::v3d::{V3dView, V3dViewSource, V3dViewer};
use martensite_vulkan::{VulkanDriverFactory, VulkanOptions};
use std::cell::{RefCell, RefMut};
use std::collections::VecDeque;
use std::rc::Rc;

/// The environment variable choosing the Vulkan device by enumeration index.
const DEVICE_VARIABLE: &str = "MARTENSITE_VULKAN_DEVICE";
const WINDOW_SIZE: i32 = 409;

/// Registers the Vulkan factory, without taking priority over one an application registered.
/// Example: `register_vulkan_factory();` before the first `initialize`.
pub fn register_vulkan_factory() {
    let device_index = std::env::var(DEVICE_VARIABLE)
        .ok()
        .and_then(|value| value.parse::<usize>().ok());
    let factory = VulkanDriverFactory::new();
    factory.set_default_options(VulkanOptions {
        validation: false,
        device_index,
    });
    let factory: Handle<dyn Graphic3dGraphicDriverFactory> = Handle::new(factory).cast();
    graphic3d_graphic_driver_factory_register(&factory, false);
}

/// The input controller of the current view.
pub struct DocumentViewEvents {
    controller: RefCell<AisViewController>,
}

impl DocumentViewEvents {
    fn new() -> Result<Self, Failure> {
        let mut controller = AisViewController::new()?;
        let animation = Handle::new(AisAnimationCamera::new(
            "DocumentViewEvents_ViewAnimation",
            &Handle::default(),
        ));
        animation.get().unwrap().set_own_duration(0.5);
        controller.set_view_animation(&animation);
        Ok(Self {
            controller: RefCell::new(controller),
        })
    }

    /// The controller; `events.controller().previous_move_to()`.
    pub fn controller(&self) -> RefMut<'_, AisViewController> {
        self.controller.borrow_mut()
    }
}

/// What the registry holds; the tests read it back to prove teardown.
#[derive(Default)]
pub struct ViewerLifecycleState {
    pub map_of_ais: RefCell<DoubleMap<Handle<dyn AisInteractiveObject>, AsciiString>>,
    pub ais_context: RefCell<Handle<AisInteractiveContext>>,
    pub view: RefCell<Handle<dyn V3dViewSource>>,
    pub event_managers: RefCell<VecDeque<Rc<DocumentViewEvents>>>,
    pub views: RefCell<DoubleMap<AsciiString, Handle<dyn V3dViewSource>>>,
    pub contexts: RefCell<DoubleMap<AsciiString, Handle<AisInteractiveContext>>>,
    pub drivers: RefCell<DoubleMap<AsciiString, Handle<dyn Graphic3dGraphicDriver>>>,
    pub window: RefCell<Handle<AspectNeutralWindow>>,
}

/// One document test's registry. It is thread confined by its `Rc` state.
pub struct ViewerLifecycle {
    state: Rc<ViewerLifecycleState>,
    virtual_windows: bool,
}

struct ViewNames {
    driver: String,
    viewer: String,
    view: String,
}

impl ViewerLifecycle {
    /// Start an empty registry; `ViewerLifecycle::new(true)` creates virtual windows.
    pub fn new(virtual_windows: bool) -> Self {
        Self {
            state: Rc::new(ViewerLifecycleState::default()),
            virtual_windows,
        }
    }

    /// Open a view named `driver/viewer/view`, or the next free view of the current viewer for an
    /// empty name, and make it current. Returns its full name.
    /// Example: `lifecycle.initialize("Driver1/Document_D/View1")?`.
    pub fn initialize(&self, view_name: &str) -> Result<String, Failure> {
        let names = self.view_names(view_name)?;
        let (driver, new_driver) = self.driver_named(&names.driver)?;
        let (viewer, new_viewer) = self.viewer_named(&names.viewer, &driver)?;
        let window = self.new_window(&names.view);
        let view = V3dView::new(&viewer, viewer.get().unwrap().default_type_of_view())?;
        let view_ref = view.get().unwrap();
        view_ref.set_window(&window.clone().cast::<dyn AspectWindow>(), 0)?;
        self.interactive_context()
            .get()
            .unwrap()
            .redraw_immediate(&viewer)?;
        *self.state.window.borrow_mut() = window.clone();
        *self.state.view.borrow_mut() = view.clone();
        self.state
            .views
            .borrow_mut()
            .bind(AsciiString::from(names.view.as_str()), view.clone())?;
        finish_view_setup(view_ref, &viewer, new_driver || new_viewer)?;
        window.get().unwrap().map();
        self.reset_events()?;
        view_ref.redraw();
        Ok(names.view)
    }

    /// Remove a named view; its context and driver go with their last view when asked.
    /// Example: `lifecycle.remove("Driver1/Document_D/View1", true)?`.
    pub fn remove(&self, name: &str, remove_context: bool) -> Result<(), Failure> {
        let key = AsciiString::from(name);
        let view = self.state.views.borrow().seek1(&key).cloned();
        let Some(view) = view else {
            return Err(Failure::with_message(
                "Standard_NoSuchObject",
                &format!("view name {name:?}; expected a registered view"),
            ));
        };
        let context = self.context_of_view(&view);
        if view == self.current_view() {
            self.leave_current_view(&key, remove_context)?;
        }
        self.state.views.borrow_mut().unbind1(&key);
        let view_ref = view.get().unwrap();
        if let Some(window) = view_ref.window().get() {
            window.unmap();
        }
        view_ref.remove();
        self.release_unused_context(&context, remove_context)
    }

    /// Remove the current context's named objects; unregistered presentations stay.
    /// Example: `lifecycle.clear()?`.
    pub fn clear(&self) -> Result<(), Failure> {
        let context = self.interactive_context();
        let Some(context_ref) = context.get() else {
            return Ok(());
        };
        let objects: Vec<Handle<dyn AisInteractiveObject>> = {
            let map = self.state.map_of_ais.borrow();
            map.items().map(|(object, _)| object.clone()).collect()
        };
        for object in objects {
            if object.get().unwrap().get_context() == context {
                context_ref.remove(&object, false)?;
                self.state.map_of_ais.borrow_mut().unbind1(&object);
            }
        }
        context_ref.rebuild_selection_structs()?;
        context_ref.update_current_viewer();
        Ok(())
    }

    /// Queue and flush pointer motion, returning the detected point or `f64::MAX` coordinates.
    /// Example: `lifecycle.move_cursor(Vec2::new(10, 10))?`.
    pub fn move_cursor(&self, pixel: Vec2<i32>) -> Result<Point3, Failure> {
        let context = self.interactive_context();
        let events = self.state.event_managers.borrow().front().cloned();
        let events = events.ok_or_else(|| {
            Failure::with_message(
                "Standard_NullObject",
                "no view; expected an initialized one",
            )
        })?;
        let mut controller = events.controller();
        controller.reset_previous_move_to();
        controller.update_mouse_position(
            pixel,
            ASPECT_VKEY_MOUSE_NONE,
            ASPECT_VKEY_FLAGS_NONE,
            false,
        )?;
        controller.flush_view_events(&context, &self.current_view(), true)?;
        picked_point(context.get().unwrap())
    }

    /// Read the registry; `lifecycle.state().views.borrow().extent()`.
    pub fn state(&self) -> Rc<ViewerLifecycleState> {
        self.state.clone()
    }

    /// Read the current view; `lifecycle.current_view().is_null()`.
    pub fn current_view(&self) -> Handle<dyn V3dViewSource> {
        self.state.view.borrow().clone()
    }

    /// Read the current interactive context; `lifecycle.interactive_context().is_null()`.
    pub fn interactive_context(&self) -> Handle<AisInteractiveContext> {
        self.state.ais_context.borrow().clone()
    }

    fn view_names(&self, input: &str) -> Result<ViewNames, Failure> {
        if input.is_empty() {
            return self.next_view_names();
        }
        let Some((viewer, _)) = input.rsplit_once('/') else {
            return Err(unexpected_view_name(input));
        };
        let Some((driver, _)) = viewer.rsplit_once('/') else {
            return Err(unexpected_view_name(input));
        };
        Ok(ViewNames {
            driver: driver.into(),
            viewer: viewer.into(),
            view: input.into(),
        })
    }

    fn next_view_names(&self) -> Result<ViewNames, Failure> {
        let state = &self.state;
        let context = self.interactive_context();
        let (driver, viewer) = match context.get() {
            None => ("Driver1".to_string(), "Driver1/Viewer1".to_string()),
            Some(context_ref) => (
                text(state.drivers.borrow().find2(&driver_of(context_ref))?),
                text(state.contexts.borrow().find2(&context)?),
            ),
        };
        let views = state.views.borrow();
        let free = (1..)
            .map(|number| format!("{viewer}/View{number}"))
            .find(|name| !views.is_bound1(&AsciiString::from(name.as_str())));
        Ok(ViewNames {
            driver,
            viewer,
            view: free.unwrap(),
        })
    }

    fn driver_named(
        &self,
        name: &str,
    ) -> Result<(Handle<dyn Graphic3dGraphicDriver>, bool), Failure> {
        let key = AsciiString::from(name);
        if let Some(driver) = self.state.drivers.borrow().seek1(&key) {
            return Ok((driver.clone(), false));
        }
        let factory = graphic3d_graphic_driver_factory_default();
        let factory = factory.get().ok_or_else(|| {
            Failure::with_message("Standard_ProgramError", "no graphic driver factory found")
        })?;
        let driver = factory.create_driver(&Handle::<AspectDisplayConnection>::default())?;
        if self.virtual_windows {
            // A window that is not on a screen has no reason to wait for vertical sync.
            driver.get().unwrap().set_vertical_sync(false);
        }
        self.state.drivers.borrow_mut().bind(key, driver.clone())?;
        Ok((driver, true))
    }

    fn viewer_named(
        &self,
        name: &str,
        driver: &Handle<dyn Graphic3dGraphicDriver>,
    ) -> Result<(Handle<V3dViewer>, bool), Failure> {
        let key = AsciiString::from(name);
        let existing = self.state.contexts.borrow().seek1(&key).cloned();
        if let Some(context) = existing {
            let viewer = viewer_of(context.get().unwrap());
            *self.state.ais_context.borrow_mut() = context;
            return Ok((viewer, false));
        }
        let viewer = V3dViewer::new(driver)?;
        // A new view takes these defaults, and the saved captures compare black backgrounds.
        let black = Color::named(NamedColor::BLACK);
        let viewer_ref = viewer.get().unwrap();
        viewer_ref.set_default_background_color(black);
        viewer_ref.set_default_bg_gradient_colors(black, black, AspectGradientFillMethod::None);
        let context = AisInteractiveContext::new(&viewer)?;
        *self.state.ais_context.borrow_mut() = context.clone();
        self.state.contexts.borrow_mut().bind(key, context)?;
        Ok((viewer, true))
    }

    fn new_window(&self, view_name: &str) -> Handle<AspectNeutralWindow> {
        let window = AspectNeutralWindow::default();
        window.set_position_and_size(20, 40, 20 + WINDOW_SIZE, 40 + WINDOW_SIZE);
        window.set_title(&AsciiString::from(
            format!("3D View - {view_name}").as_str(),
        ));
        let window = Handle::new(window);
        window.get().unwrap().set_virtual(self.virtual_windows);
        window
    }

    fn reset_events(&self) -> Result<(), Failure> {
        let mut managers = self.state.event_managers.borrow_mut();
        managers.clear();
        managers.push_front(Rc::new(DocumentViewEvents::new()?));
        Ok(())
    }

    /// Make another view current, or empty the current state when this was the last one.
    fn leave_current_view(&self, key: &AsciiString, remove_context: bool) -> Result<(), Failure> {
        let other = {
            let views = self.state.views.borrow();
            let other = views.items().find(|(name, _)| *name != key);
            other.map(|(_, view)| view.clone())
        };
        let Some(other) = other else {
            *self.state.window.borrow_mut() = Handle::default();
            *self.state.view.borrow_mut() = Handle::default();
            if remove_context {
                *self.state.ais_context.borrow_mut() = Handle::default();
            }
            return self.reset_events();
        };
        let context = self.context_of_view(&other);
        *self.state.view.borrow_mut() = other.clone();
        *self.state.ais_context.borrow_mut() = context;
        *self.state.window.borrow_mut() =
            other.get().unwrap().window().cast::<AspectNeutralWindow>();
        self.reset_events()?;
        other.get().unwrap().redraw();
        Ok(())
    }

    fn context_of_view(&self, view: &Handle<dyn V3dViewSource>) -> Handle<AisInteractiveContext> {
        let viewer = view.get().unwrap().viewer();
        let contexts = self.state.contexts.borrow();
        contexts
            .items()
            .find(|(_, context)| viewer_of(context.get().unwrap()) == viewer)
            .map(|(_, context)| context.clone())
            .unwrap_or_default()
    }

    /// A context with no view left goes when asked, and the driver goes with its last context.
    fn release_unused_context(
        &self,
        context: &Handle<AisInteractiveContext>,
        remove_context: bool,
    ) -> Result<(), Failure> {
        let Some(context_ref) = context.get() else {
            return Ok(());
        };
        let no_views = viewer_of(context_ref)
            .get()
            .unwrap()
            .defined_views()
            .is_empty();
        let several = self.state.contexts.borrow().extent() != 1;
        if !no_views || !(remove_context || several) {
            return Ok(());
        }
        let driver = driver_of(context_ref);
        let driver_shared = self
            .state
            .contexts
            .borrow()
            .items()
            .any(|(_, other)| other != context && driver_of(other.get().unwrap()) == driver);
        context_ref.remove_all(false)?;
        if !driver_shared {
            self.state.drivers.borrow_mut().unbind2(&driver);
        }
        self.state.contexts.borrow_mut().unbind2(context);
        Ok(())
    }
}

fn finish_view_setup(
    view: &dyn V3dViewSource,
    viewer: &Handle<V3dViewer>,
    new_viewer: bool,
) -> Result<(), Failure> {
    view.set_computed_mode(false);
    let viewer = viewer.get().unwrap();
    viewer.set_default_background_color(Color::named(NamedColor::BLACK));
    if new_viewer {
        viewer.set_default_lights()?;
        viewer.set_light_on()?;
    }
    Ok(())
}

fn picked_point(context: &AisInteractiveContext) -> Result<Point3, Failure> {
    let owner = (*context.detected_owner()).clone();
    let selector = context.main_selector();
    for rank in 1..=selector.nb_picked() {
        if selector.picked(rank)? == owner {
            return selector.picked_point(rank);
        }
    }
    Ok(Point3::new(f64::MAX, f64::MAX, f64::MAX))
}

fn viewer_of(context: &AisInteractiveContext) -> Handle<V3dViewer> {
    (*context.current_viewer()).clone()
}

fn driver_of(context: &AisInteractiveContext) -> Handle<dyn Graphic3dGraphicDriver> {
    viewer_of(context).get().unwrap().driver()
}

fn text(value: &AsciiString) -> String {
    String::from_utf8_lossy(value.as_bytes()).into_owned()
}

fn unexpected_view_name(input: &str) -> Failure {
    Failure::with_message(
        "Standard_DomainError",
        &format!("view name {input:?}; expected empty or driver/viewer/view"),
    )
}
