//! The state a script's exchange commands share: the work session, its pilot, the documents, a raster
//! for the shape variables and what the message printer writes, which `[data c]` and `[tpstat c]`
//! read back as `script` reads their result.
use crate::support::raster_session::RasterSession;
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

/// A running script's exchange state; `XsSession::new("de_step_1_A7")`.
pub struct XsSession {
    pub raster: RasterSession,
    pub documents: BTreeMap<String, Handle<TDocStdDocument>>,
    pub work_session: Handle<XSControlWorkSession>,
    /// The script's path, which names the folder its case writes in.
    pub source: String,
    /// `de_before_script`: resource keys the script changes for its reads, without their scope.
    pub resource_overrides: Vec<(String, String)>,
    /// `FromSTEP.exec.op` emptied in the resource file a script reads with.
    pub read_healing_off: bool,
    pilot: Handle<IFSelectSessionPilot>,
    printer: Arc<CasePrinter>,
}

impl XsSession {
    /// The session the exchange plugins start with, the only message printer being the case's.
    /// Example: `let mut session = XsSession::for_source("de_step_1_A7", "tests/de/step_1/A7");`.
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
        XSAlgo::init();
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
            resource_overrides: Vec::new(),
            read_healing_off: false,
            pilot,
            printer,
        })
    }

    /// `the session::set_norm("STEP")`: the norm `read_step` and `WriteStep` select unless one is selected.
    /// Example: `session.select_step_norm()?`.
    pub fn select_step_norm(&self) -> Result<(), Failure> {
        let selected = self
            .work_session
            .get()
            .expect("a work session")
            .select_norm("STEP")?;
        assert!(selected, "set_norm: STEP is not registered");
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

    /// What `f` writes through the message printer, also left in the stream.
    /// Example: `session.keep_messages(|| ())`.
    pub fn keep_messages(&self, f: impl FnOnce()) -> String {
        *self.printer.kept.lock().expect("a printer lock") = Some(String::new());
        f();
        let kept = self.printer.kept.lock().expect("a printer lock").take();
        kept.expect("kept while the command ran")
    }
}
