//! The default messenger's output in the test log: scenario keeps the default
//! Message_PrinterOStream (trace level Info), and the test harness logs every command
//! with `dlog on`, which redirects standard output into the log while the command runs,
//! so what the algorithms send (ShapeProcess statistics, healing warnings) reaches the
//! log the parse rules read. Here the standard-output printer is replaced by one that
//! writes each message as a log line.
use super::log;
use martensite_tkernel::message::{
    Gravity, Message, Printer, PrinterBase, PrinterOStream, PrinterType,
};
use martensite_tkernel::standard::{RuntimeType, RuntimeTyped};
use martensite_tkernel::tcollection::AsciiString;
use std::sync::Arc;

/// The standard-output printer of the case: each message becomes a log line.
struct LogPrinter(PrinterBase);

impl Printer for LogPrinter {
    fn printer_base(&self) -> &PrinterBase {
        &self.0
    }
    fn dynamic_type(&self) -> &'static RuntimeType {
        PrinterType::static_type()
    }
    fn send(&self, text: &AsciiString, _gravity: Gravity) {
        log::puts(&String::from_utf8_lossy(text.as_bytes()));
    }
}

/// The case process messenger: the standard-output printers
/// removed and the log printer added (trace level Info, the Message_Printer default).
pub fn stdout_printer_to_log() {
    let messenger = Message::default_messenger();
    messenger.remove_printers(PrinterOStream::static_type());
    messenger.add_printer(Arc::new(LogPrinter(PrinterBase::default())));
}
