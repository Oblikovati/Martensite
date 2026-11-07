//! What a script's commands run on: the exchange session of the dataset cases and the real view.
use crate::scene::ViewScene;
use crate::xs_session::XsSession;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::tcollection::AsciiString;

/// A running script; `ViewerHost::new("cases_iges_bug25518", "tests/bugs/iges/bug25518")`.
pub struct ViewerHost {
    pub session: XsSession,
    pub scene: Option<ViewScene>,
    pub source: &'static str,
    /// The display mode `XDisplay` asks for: its `-dispMode`, or the default one.
    pub xdisplay_mode: Option<i32>,
}

impl ViewerHost {
    pub fn new(case_name: &str, source: &'static str) -> Self {
        Self {
            session: XsSession::for_source(case_name, source),
            scene: None,
            source,
            xdisplay_mode: None,
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
    /// Example: `host.image_stem("bug25518")` is `bugs_iges_bug25518__bug25518`.
    pub fn image_stem(&self, name: &str) -> String {
        let script = self.source.trim_start_matches("tests/").replace('/', "_");
        format!("{script}__{name}")
    }

    /// `dall`: the case's shape variables go, so a later `isdraw` of one is false.
    /// Example: `host.dall(&["b1", "b2"])`.
    pub fn dall(&mut self, names: &[&str]) {
        use crate::support::session::Session as _;
        println!("dall ");
        for name in names {
            self.session.raster.unbind(name);
        }
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
