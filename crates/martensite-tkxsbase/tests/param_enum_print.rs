//! The complete OCC23570 parameter diagnostic regression, without an interpreter.
#[path = "support/xsbase_case_stream.rs"]
mod source_process;
pub(crate) use source_process::{
    case_bop_fatal_rules, case_bugs_bop_rules, case_common_bop_rules, case_report_receipt,
};
#[path = "support/xsbase_parent_controls.rs"]
mod parent_controls;
#[path = "support/xsbase_parent_registration.rs"]
mod parent_registration;
use martensite_tkernel::{
    message::{Gravity, Message, PrinterHandle, PrinterOStream},
    standard::Handle,
    tcollection::AsciiString,
};
use martensite_tkxsbase::{
    if_select::{
        IFSelectActivatorBase, IFSelectFunctions, IFSelectReturnStatus, IFSelectSessionPilot,
        IFSelectWorkSession,
    },
    interface::InterfaceStatic,
};
use std::{
    io,
    io::Write,
    sync::{Arc, Mutex},
};

#[derive(Clone, Default)]
struct FakeMessageWriter(Arc<Mutex<Vec<u8>>>);

impl Write for FakeMessageWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct ParamMessageCapture {
    writer: FakeMessageWriter,
    printer: PrinterHandle,
    displaced: Vec<PrinterHandle>,
}

impl ParamMessageCapture {
    /// The command's printer replaces the others until the capture ends, so the text is written once.
    fn begin() -> Self {
        let writer = FakeMessageWriter::default();
        let printer: PrinterHandle = Arc::new(PrinterOStream::with_writer(
            Box::new(writer.clone()),
            true,
            Gravity::Info,
        ));
        let messenger = Message::default_messenger();
        let displaced = std::mem::take(&mut *messenger.printers_mut());
        assert!(messenger.add_printer(printer.clone()));
        Self {
            writer,
            printer,
            displaced,
        }
    }

    fn text(&self) -> String {
        String::from_utf8(self.writer.0.lock().unwrap().clone()).unwrap()
    }
}

impl Drop for ParamMessageCapture {
    fn drop(&mut self) {
        let messenger = Message::default_messenger();
        assert!(messenger.remove_printer(&self.printer));
        messenger.printers_mut().append(&mut self.displaced);
    }
}

fn printed_parameter() -> String {
    InterfaceStatic::standards().unwrap();
    IFSelectFunctions::init().unwrap();
    let pilot = IFSelectSessionPilot::new("").unwrap();
    pilot.set_session(&Handle::new(IFSelectWorkSession::new()));
    let capture = ParamMessageCapture::begin();
    assert_eq!(
        pilot
            .execute(&AsciiString::new("param write.surfacecurve.mode"))
            .unwrap(),
        IFSelectReturnStatus::RetVoid,
    );
    capture.text()
}

fn enum_aliases(text: &str) -> Option<[&str; 2]> {
    let prefix = "--- Type : Enum [in 0-1] 0:Off 1:On , alpha: ";
    let (_, after) = text.split_once(prefix)?;
    let (aliases, _) = after.split_once("\n---")?;
    if !aliases.ends_with(' ') {
        return None;
    }
    let parts: Vec<&str> = aliases.split(' ').filter(|s| !s.is_empty()).collect();
    let [first, second] = parts.as_slice() else {
        return None;
    };
    Some([enum_alias_name(first)?, enum_alias_name(second)?])
}

fn enum_alias_name(token: &str) -> Option<&str> {
    let (name, digit) = token.rsplit_once(':')?;
    if digit.len() != 1 || !digit.as_bytes()[0].is_ascii_digit() {
        return None;
    }
    Some(name)
}

fn readable_aliases(text: &str) -> bool {
    enum_aliases(text).is_some_and(|aliases| {
        aliases
            .iter()
            .all(|name| name.bytes().all(|byte| byte.is_ascii_alphanumeric()))
    })
}

#[test]
fn case_enum_parameter_diagnostic_contains_readable_aliases() {
    let identity = "case_enum_parameter_diagnostic_contains_readable_aliases";
    let body = case_parameter_diagnostic_body;
    source_process::run_case(identity, body);
}

fn case_parameter_commands_without_end() -> Result<(), Box<dyn std::error::Error>> {
    source_process::emit_source("============\nOCC23570\n============\n\n")?;
    let diagnostic = printed_parameter();
    source_process::emit_source(&format!("{diagnostic}\n"))?;
    // The original checks its enum-line shape, then rejects non-alphanumeric alias bytes.
    assert!(
        enum_aliases(&diagnostic).is_some(),
        "bad message: {diagnostic:?}"
    );
    assert!(
        readable_aliases(&diagnostic),
        "non-readable symbols: {diagnostic:?}"
    );
    print_case_aliases(&diagnostic)?;
    Ok(())
}

fn print_case_aliases(diagnostic: &str) -> std::io::Result<()> {
    let [first, second] = enum_aliases(diagnostic).unwrap();
    let aliases = format!("alpha_0={first}\nalpha_1={second}\n");
    source_process::emit_source(&aliases)
}

#[test]
fn enum_diagnostic_assertions_reject_bad_shape_and_unreadable_aliases() {
    let prefix = "--- Type : Enum [in 0-1] 0:Off 1:On , alpha: ";
    for aliases in ["Off:0 On:1 ", "On:1 Off:0 "] {
        assert!(readable_aliases(&format!(
            "{prefix}{aliases}\n--- Value : On"
        )));
    }
    for aliases in ["Off:0 Oñ:1 ", "Off:0 On:10 ", "Off:0 On:1", "Off:0 "] {
        assert!(!readable_aliases(&format!(
            "{prefix}{aliases}\n--- Value : On"
        )));
    }
    assert!(!readable_aliases("not an enum diagnostic"));
}

fn case_parameter_diagnostic_body() -> Result<(), Box<dyn std::error::Error>> {
    case_parameter_commands_without_end()?;
    // bugs/end has no area check when the inherited relative tolerance remains zero.
    source_process::emit_source("TEST COMPLETED\n")?;
    Ok(())
}

/// A debug build of the toolkit reports these; a release build, which this one follows, does not.
#[test]
fn debug_only_diagnostics_stay_off_the_stream() {
    source_process::run_case("debug_only_diagnostics_stay_off_the_stream", || {
        assert_eq!(InterfaceStatic::i_val("no.such.parameter"), 0);
        assert_eq!(InterfaceStatic::r_val("no.such.parameter"), 0.0);
        assert!(InterfaceStatic::c_val("no.such.parameter").is_empty());
        IFSelectFunctions::init().unwrap();
        let pilot = IFSelectSessionPilot::new("").unwrap();
        pilot
            .set_command_line(&AsciiString::new("two words"))
            .unwrap();
        register_twice("debug_only_duplicate_command");
        source_process::emit_source("TEST COMPLETED\n")?;
        Ok(())
    });
}

fn register_twice(command: &str) {
    let actor = IFSelectActivatorBase::select("xstep").unwrap().1;
    IFSelectActivatorBase::adding(&actor, 1, command, 0);
    IFSelectActivatorBase::adding(&actor, 1, command, 0);
}

parent_registration::register_xsbase_parent_controls!(
    super::case_parameter_diagnostic_body,
    super::case_parameter_commands_without_end
);

#[test]
fn xsbase_policy_preserves_completion_presence_and_group_priority() {
    let grade = source_process::accepted_case_stream;
    assert!(grade("TEST COMPLETED\nTEST COMPLETED\nWarning: benign\n"));
    assert!(!grade("Warning: missing completion\n"));
    assert!(!grade(
        "TEST COMPLETED\nFaulty Relative error of mass computation\n"
    ));
    assert!(grade(
        "TEST COMPLETED\nRelative error of mass computation : 0.1\n"
    ));
}
