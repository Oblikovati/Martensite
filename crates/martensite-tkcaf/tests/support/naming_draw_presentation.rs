//! Complete-original presentation is injected; model fixtures retain their nonvisual purpose.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use std::{cell::RefCell, rc::Rc};

trait PresentationCommands {
    fn publish(&mut self, name: &str, shape: &Shape);
    fn isos(&mut self, count: i32);
    fn only(&mut self, name: &str);
    fn erase(&mut self, names: &[&str]);
    fn erase_all(&mut self);
    fn display(&mut self, names: &[&str]);
    fn fit(&mut self);
    fn result_view(&mut self, image: &str);
    fn end(&mut self);
}

/// Shared live drawable state is confined to its CPU-limited UI child, never Send/Sync.
#[derive(Clone, Default)]
pub(super) struct NamingPresentation(Option<Rc<RefCell<dyn PresentationCommands>>>);

impl NamingPresentation {
    /// Connect a real supported backend or return typed unsupported; `NamingPresentation::connected()?`.
    #[cfg(target_os = "linux")]
    pub(super) fn connected() -> Result<Self, Failure> {
        let destination =
            std::env::var_os("MARTENSITE_GEOMETRY_ARTIFACT_DIR").map(std::path::PathBuf::from);
        let scene = crate::document_presentation::DocumentScene::connect(destination)?;
        let state = PresentationState { scene };
        Ok(Self(Some(Rc::new(RefCell::new(state)))))
    }

    /// Keep unsupported native presentation explicit; `NamingPresentation::connected()` returns an error.
    #[cfg(not(target_os = "linux"))]
    pub(super) fn connected() -> Result<Self, Failure> {
        Err(Failure::new(&format!(
            "CAF presentation on {}: expected implemented Linux X11 backend; other backends pending",
            std::env::consts::OS
        )))
    }

    fn apply(&self, action: impl FnOnce(&mut dyn PresentationCommands)) {
        if let Some(state) = &self.0 {
            action(&mut *state.borrow_mut());
        }
    }

    /// Publish the actual named shape with live source style; `presentation.publish("S", &shape)`.
    pub(super) fn publish(&self, name: &str, shape: &Shape) {
        let command = |state: &mut dyn PresentationCommands| state.publish(name, shape);
        self.apply(command);
    }

    /// Preserve the explicit source default change; `presentation.isos(12)`.
    pub(super) fn isos(&self, count: i32) {
        let command = |state: &mut dyn PresentationCommands| state.isos(count);
        self.apply(command);
    }

    /// Keep source donly and its camera; `presentation.only("S")`.
    pub(super) fn only(&self, name: &str) {
        let command = |state: &mut dyn PresentationCommands| state.only(name);
        self.apply(command);
    }

    /// Hide actual aliases without deleting identities; `presentation.erase(&["Pr_8"])`.
    pub(super) fn erase(&self, names: &[&str]) {
        let command = |state: &mut dyn PresentationCommands| state.erase(names);
        self.apply(command);
    }

    /// Preserve variables while erasing the display; `presentation.erase_all()`.
    pub(super) fn erase_all(&self) {
        let command = |state: &mut dyn PresentationCommands| state.erase_all();
        self.apply(command);
    }

    /// Redisplay actual aliases without changing the camera; `presentation.display(&["Pr_8", "Pr_9"])`.
    pub(super) fn display(&self, names: &[&str]) {
        let command = |state: &mut dyn PresentationCommands| state.display(names);
        self.apply(command);
    }

    /// Fit existing views without inventing one; `presentation.fit()`.
    pub(super) fn fit(&self) {
        let command = |state: &mut dyn PresentationCommands| state.fit();
        self.apply(command);
    }

    /// Run the literal result-only AXON checkview; `presentation.result_view("bug21707")`.
    pub(super) fn result_view(&self, image: &str) {
        let command = |state: &mut dyn PresentationCommands| state.result_view(image);
        self.apply(command);
    }

    /// End before document cleanup; `presentation.end()`.
    pub(super) fn end(&self) {
        let command = |state: &mut dyn PresentationCommands| state.end();
        self.apply(command);
    }
}

#[cfg(target_os = "linux")]
struct PresentationState {
    scene: crate::document_presentation::DocumentScene,
}

#[cfg(target_os = "linux")]
impl PresentationCommands for PresentationState {
    fn publish(&mut self, name: &str, shape: &Shape) {
        let scene = &mut self.scene;
        scene.publish(name, shape).unwrap();
    }

    fn isos(&mut self, count: i32) {
        let scene = &mut self.scene;
        scene.set_isos(count);
    }

    fn only(&mut self, name: &str) {
        let scene = &mut self.scene;
        scene.only(name).unwrap();
    }

    fn erase(&mut self, names: &[&str]) {
        let scene = &mut self.scene;
        scene.erase(names).unwrap();
    }

    fn erase_all(&mut self) {
        let scene = &mut self.scene;
        scene.erase_all().unwrap();
    }

    fn display(&mut self, names: &[&str]) {
        let scene = &mut self.scene;
        scene.display(names).unwrap();
    }

    fn fit(&mut self) {
        let scene = &mut self.scene;
        scene.fit_existing().unwrap();
    }

    fn result_view(&mut self, image: &str) {
        let scene = &mut self.scene;
        scene.result_view(image).unwrap();
    }

    fn end(&mut self) {
        println!("TEST COMPLETED");
        self.scene.close().unwrap();
    }
}
