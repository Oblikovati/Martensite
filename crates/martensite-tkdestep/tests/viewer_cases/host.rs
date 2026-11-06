//! What a script's commands run on: the exchange session of the dataset cases and the real view.
use crate::scene::ViewScene;
use crate::xs_session::XsSession;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::tcollection::AsciiString;

/// A running script; `ViewerHost::new("cases_step_bug25169", "tests/bugs/step/bug25169")`.
pub struct ViewerHost {
    pub session: XsSession,
    pub scene: Option<ViewScene>,
    pub source: &'static str,
}

impl ViewerHost {
    pub fn new(case_name: &str, source: &'static str) -> Self {
        Self {
            session: XsSession::for_source(case_name, source),
            scene: None,
            source,
        }
    }

    /// The open viewer; a command that needs one stops a script without it.
    pub fn live_scene(&self) -> &ViewScene {
        self.scene
            .as_ref()
            .unwrap_or_else(|| panic!("{}: no active viewer", self.source))
    }

    pub fn shape(&self, name: &str) -> Shape {
        use crate::support::session::Session as _;
        self.session
            .raster
            .shape(name)
            .unwrap_or_else(|| panic!("{}: {name} is no shape", self.source))
            .clone()
    }

    pub fn bind(&mut self, name: &str, shape: &Shape) {
        use crate::support::session::Session as _;
        self.session.raster.bind(name, shape);
    }

    /// The kept file's stem: the case's path and the name its `vdump` gives, without the folder.
    /// Example: `host.image_stem("bug25169")` is `bugs_step_bug25169__bug25169`.
    pub fn image_stem(&self, name: &str) -> String {
        let script = self.source.trim_start_matches("tests/").replace('/', "_");
        format!("{script}__{name}")
    }

    /// Ends the script: the view must render and report no validation error.
    pub fn close(&mut self) {
        if let Some(scene) = self.scene.take() {
            scene.close();
        }
    }
}

/// An entry as the label tool takes it; `entry("0:1:1:1")`.
pub fn entry(text: &str) -> AsciiString {
    AsciiString::from(text)
}
