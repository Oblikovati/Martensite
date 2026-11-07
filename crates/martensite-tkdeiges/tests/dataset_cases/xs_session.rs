//! The state a script's exchange commands share: the work session, its pilot, the documents, a raster
//! for the shape variables and what the message printer writes, which `[data c]` and `[tpstat c]`
//! read back as `script` reads their result.
use crate::support::{raster_session::RasterSession, script_error::script_error, session::Session};
use martensite_tkbrep::topods::Shape;
use martensite_tkdeiges::iges_control::IgesControlController;
use martensite_tkernel::{
    message::{Gravity, Message, Printer, PrinterBase, PrinterHandle, PrinterType},
    standard::{Failure, Handle, RuntimeType, RuntimeTyped},
    tcollection::AsciiString,
};
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkmath::gp::Point3;
use martensite_tkxsbase::{
    if_select::{IFSelectFunctions, IFSelectSessionPilot, IFSelectWorkSession},
    xs_algo::XSAlgo,
    xs_control::{
        XSControlFuncShape, XSControlFunctions, XSControlVars, XSControlVarsOps,
        XSControlWorkSession,
    },
};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

/// The folders of the test data submodule that hold the inputs of these scripts.
const DATA_FOLDERS: [&str; 5] = [
    "dataset-7.9.0/iges",
    "dataset-7.9.0/brep",
    "dataset-7.9.0/others",
    "occt-source-data",
    "occt-source-data/iges",
];

/// Writes each message to the case's stream through the capture of the test harness, in order with
/// the case's own lines, and keeps it while a command's result is being read.
struct CasePrinter {
    base: PrinterBase,
    kept: Mutex<Option<String>>,
}

impl Printer for CasePrinter {
    fn printer_base(&self) -> &PrinterBase {
        &self.base
    }
    fn dynamic_type(&self) -> &'static RuntimeType {
        PrinterType::static_type()
    }
    fn send(&self, text: &AsciiString, _gravity: Gravity) {
        let text = String::from_utf8_lossy(text.as_bytes());
        println!("{text}");
        if let Some(kept) = self.kept.lock().expect("a printer lock").as_mut() {
            kept.push_str(&text);
            kept.push('\n');
        }
    }
}

/// `[locate_data_file name]`; `None` when the data set does not have it.
pub fn locate_data_file(name: &str) -> Option<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../external/martensite-test-data");
    DATA_FOLDERS
        .iter()
        .map(|folder| root.join(folder).join(name))
        .find(|path| path.is_file())
}

/// A running script's exchange state; `XsSession::for_source("bug", "tests/bugs/iges/bug283")`.
pub struct XsSession {
    pub raster: RasterSession,
    pub documents: BTreeMap<String, Handle<TDocStdDocument>>,
    pub work_session: Handle<XSControlWorkSession>,
    /// The curve and surface variables of a script.
    pub curves: BTreeMap<String, Handle<dyn GeomCurve>>,
    pub surfaces: BTreeMap<String, Handle<dyn GeomSurface>>,
    pub points: BTreeMap<String, Point3>,
    /// The script's path, which names the folder its case writes in.
    pub source: String,
    pilot: Handle<IFSelectSessionPilot>,
    printer: Arc<CasePrinter>,
}

impl XsSession {
    /// The session the exchange plugins start with, the only message printer being the case's.
    /// Example: `let mut session = XsSession::for_source("bug283", "tests/bugs/iges/bug283");`.
    pub fn for_source(case_name: &str, source: &str) -> Self {
        Self::start(case_name, source).unwrap_or_else(|failure| panic!("session: {failure:?}"))
    }

    fn start(case_name: &str, source: &str) -> Result<Self, Failure> {
        let printer = Arc::new(CasePrinter {
            base: PrinterBase::default(),
            kept: Mutex::new(None),
        });
        let messenger = Message::default_messenger();
        messenger.printers_mut().clear();
        messenger.add_printer(printer.clone() as PrinterHandle);
        IgesControlController::init()?;
        XSAlgo::init()?;
        IFSelectFunctions::init()?;
        XSControlFunctions::init()?;
        XSControlFuncShape::init()?;
        let pilot = IFSelectSessionPilot::new("XSTEP>")?;
        let work_session = XSControlWorkSession::new();
        work_session
            .get()
            .expect("a work session")
            .set_vars(&Handle::new(XSControlVars::new()).cast::<dyn XSControlVarsOps>());
        pilot.set_session(&work_session.clone().cast::<IFSelectWorkSession>());
        Ok(Self {
            raster: RasterSession::new(case_name),
            documents: BTreeMap::new(),
            work_session,
            curves: BTreeMap::new(),
            surfaces: BTreeMap::new(),
            points: BTreeMap::new(),
            source: source.to_owned(),
            pilot,
            printer,
        })
    }

    /// Selects the IGES norm. Example: `session.select_iges_norm()?`.
    pub fn select_iges_norm(&self) -> Result<(), Failure> {
        let selected = self
            .work_session
            .get()
            .expect("a work session")
            .select_norm("IGES")?;
        assert!(selected, "set_norm: IGES is not registered");
        Ok(())
    }

    /// A pilot command's text: written to the stream, returned as the command's result.
    /// Example: `session.pilot_command("data c")`.
    pub fn pilot_command(&self, line: &str) -> String {
        println!("{line} ");
        *self.printer.kept.lock().expect("a printer lock") = Some(String::new());
        let outcome = self
            .pilot
            .execute(&AsciiString::from(format!("{line} ").as_str()));
        if let Err(failure) = outcome {
            panic!("{line}: {failure:?}");
        }
        let kept = self.printer.kept.lock().expect("a printer lock").take();
        kept.expect("kept while the command ran")
    }

    /// The folder a case writes in, as the case's `$imagedir`.
    /// Example: `session.image_file("a.igs")`.
    pub fn image_file(&self, name: &str) -> PathBuf {
        let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
            .join("iges-cases")
            .join(self.source.replace('/', "_"));
        std::fs::create_dir_all(&folder).unwrap();
        folder.join(name)
    }

    /// A path word of the script; a data file that is not there is the script error of the lookup.
    /// Example: `session.path_word("@data/a.igs")`.
    pub fn path_word(&self, word: &str) -> String {
        if let Some(name) = word.strip_prefix("@image/") {
            return self.image_file(name).to_string_lossy().into_owned();
        }
        let Some(name) = word.strip_prefix("@data/") else {
            return word.to_owned();
        };
        locate_data_file(name)
            .unwrap_or_else(|| script_error(&format!("File {name} could not be found")))
            .to_string_lossy()
            .into_owned()
    }

    /// The shape of a variable; a name that holds none stops the script.
    /// Example: `session.bound_shape("a")`.
    pub fn bound_shape(&self, name: &str) -> Shape {
        self.raster
            .shape(name)
            .cloned()
            .unwrap_or_else(|| script_error(&format!("{name} is not a shape")))
    }
}
