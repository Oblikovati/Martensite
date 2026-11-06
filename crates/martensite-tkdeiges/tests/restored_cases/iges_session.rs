//! The state a script's exchange commands share: the work session, its pilot, the documents, a raster
//! for the shape variables and what the message printer writes, which `[data c]` and `[tpstat c]`
//! read back as `script` reads their result.
use crate::support::{raster_session::RasterSession, session::Session};
use martensite_tkbrep::topods::Shape;
use martensite_tkdeiges::iges_control::IgesControlController;
use martensite_tkdestep::stepcaf_control::STEPCAFControlController;
use martensite_tkernel::{
    message::{Gravity, Message, Printer, PrinterBase, PrinterHandle, PrinterType},
    standard::{Failure, Handle, RuntimeType, RuntimeTyped},
    tcollection::AsciiString,
};
use martensite_tklcaf::tdoc_std::TDocStdDocument;
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

/// A running script's exchange state; `IgesSession::for_source("de_iges_1_B2", "tests/de/iges_1/B2")`.
pub struct IgesSession {
    pub raster: RasterSession,
    pub documents: BTreeMap<String, Handle<TDocStdDocument>>,
    pub work_session: Handle<XSControlWorkSession>,
    /// The script's path, which names the folder its case writes in.
    pub source: String,
    pilot: Handle<IFSelectSessionPilot>,
    printer: Arc<CasePrinter>,
}

impl IgesSession {
    /// The session the exchange plugins start with, the only message printer being the case's.
    /// Example: `let mut session = IgesSession::for_source("de_iges_1_B2", "tests/de/iges_1/B2");`.
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
        STEPCAFControlController::init()?;
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
            source: source.to_owned(),
            pilot,
            printer,
        })
    }

    /// `session::set_norm(norm)`: the norm the next command works in.
    /// Example: `session.select_norm("IGES")?`.
    pub fn select_norm(&self, norm: &str) -> Result<(), Failure> {
        let selected = self
            .work_session
            .get()
            .expect("a work session")
            .select_norm(norm)?;
        assert!(selected, "set_norm: {norm} is not registered");
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
    /// The shape a script bound; a missing one is a script error.
    /// Example: `session.bound_shape("res_1")`.
    pub fn bound_shape(&self, name: &str) -> Shape {
        self.raster
            .shape(name)
            .cloned()
            .unwrap_or_else(|| panic!("{name}: expected a bound shape"))
    }
}

/// `$imagedir`: this case's own folder for what it writes.
/// Example: `image_folder("tests/de/iges_1/B2")`.
pub fn image_folder(source: &str) -> PathBuf {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("iges-cases")
        .join(source.replace('/', "_"));
    std::fs::create_dir_all(&folder).unwrap();
    folder
}

/// `[locate_data_file name]`, in the folders the data set has.
/// Example: `locate_data_file("BUC50025.igs")`.
pub fn locate_data_file(name: &str) -> Option<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../external/martensite-test-data");
    [
        "dataset-7.9.0/iges",
        "dataset-7.9.0/step",
        "dataset-7.9.0/brep",
        "dataset-7.9.0/others",
    ]
    .iter()
    .map(|folder| root.join(folder).join(name))
    .find(|path| path.is_file())
}

/// A copy of the written file beside the kept stream, to compare it with the reference program's.
/// Example: `keep_written_file("/tmp/a.igs")`.
pub fn keep_written_file(file: &str) {
    let Some(folder) = std::env::var_os(crate::support::raster_session::KEEP_IMAGES_ENV) else {
        return;
    };
    let name = Path::new(file).file_name().expect("a file name");
    let _ = std::fs::copy(file, Path::new(&folder).join(name));
}
