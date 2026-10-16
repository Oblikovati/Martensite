//! The default messenger and report, driven end to end and checked through the case log.
//!
//! The case prints the messenger, sends messages, adds a report printer, prints and clears
//! the report and collects metrics. `CaseLogPrinter` collects the messenger output into the
//! case log, which is checked with the demonstration rules. It replaces process-wide default
//! printers, so it runs as its own test target.
#[path = "case_log_rules/mod.rs"]
mod case_log_rules;

use case_log_rules::{COMMON_RULES, DEMO_RULES, check_log};
use martensite_tkernel::message::{
    Gravity, Level, Message, MetricType, Printer, PrinterBase, PrinterToReport, PrinterType,
};
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::standard::{RuntimeType, RuntimeTyped};
use martensite_tkernel::tcollection::AsciiString;
use std::sync::{Arc, Mutex};

/// The case log: what the steps print and what the messenger's printer receives.
#[derive(Default)]
struct CaseLogPrinter {
    base: PrinterBase,
    lines: Mutex<Vec<String>>,
}

impl Printer for CaseLogPrinter {
    fn printer_base(&self) -> &PrinterBase {
        &self.base
    }
    fn dynamic_type(&self) -> &'static RuntimeType {
        PrinterType::static_type()
    }
    fn send(&self, text: &AsciiString, _gravity: Gravity) {
        self.print(&String::from_utf8_lossy(text.as_bytes()));
    }
}

impl CaseLogPrinter {
    fn print(&self, text: &str) {
        self.lines
            .lock()
            .unwrap()
            .extend(text.lines().map(str::to_owned));
    }
}

/// The default messenger's JSON dump.
fn print_messenger(log: &CaseLogPrinter) {
    let mut stream = DumpStream::default();
    Message::default_messenger().dump_json(&mut stream, -1);
    log.print(stream.text());
}

/// Sends each text, lower-cased, at the default (warning) gravity.
fn send_message(texts: &[&str]) {
    for text in texts {
        let mut lowered = AsciiString::new(text);
        lowered.lower_case();
        Message::default_messenger().send_ascii(&lowered, Gravity::Warning);
    }
}

/// Prints the default report's JSON dump.
fn print_report(log: &CaseLogPrinter) {
    let mut stream = DumpStream::default();
    Message::default_report(false)
        .expect("the report printer created it")
        .dump_json(&mut stream, -1);
    log.print(stream.text());
}

/// Adds a report printer on the default report, activated.
fn add_report_printer() {
    let printer = Arc::new(PrinterToReport::new());
    printer.set_report(Message::default_report(true));
    Message::default_messenger().add_printer(printer);
    Message::default_report(false)
        .expect("just created")
        .update_active_in_messenger(None);
}

/// Activates metric ids 1 and 3 (thread and process CPU user time).
fn set_report_metrics(ids: &[usize]) {
    let report = Message::default_report(true).expect("created on request");
    report.clear_metrics();
    for &id in ids {
        report.set_active_metric(MetricType::ALL[id], true);
    }
}

/// Messages reach the report printer, and metrics are collected into the report.
#[test]
fn default_messenger_routes_messages_into_report() {
    let log = Arc::new(CaseLogPrinter::default());
    *Message::default_messenger().printers_mut() = vec![log.clone()];
    let sample = [
        "processing a text message in ostream",
        "information message",
        "test faulty message",
        "warning message",
    ];
    print_messenger(&log);
    send_message(&sample);
    add_report_printer();
    print_messenger(&log);
    send_message(&[
        "processing a text message in report",
        sample[1],
        sample[2],
        sample[3],
    ]);
    print_report(&log);
    Message::default_report(false).expect("created").clear();
    set_report_metrics(&[1, 3]);
    let level = Level::new(&AsciiString::new("Level"));
    send_message(&["processing metric report"]);
    drop(level);
    print_report(&log);
    Message::default_messenger().remove_printers(PrinterToReport::static_type());
    print_messenger(&log);
    log.print("TEST COMPLETED");
    let lines = log.lines.lock().unwrap().clone();
    assert!(
        lines.iter().any(|line| line.contains("Message_Report")),
        "{lines:?}"
    );
    assert_eq!(check_log(&[DEMO_RULES, COMMON_RULES], &[], &lines), Ok(()));
}
