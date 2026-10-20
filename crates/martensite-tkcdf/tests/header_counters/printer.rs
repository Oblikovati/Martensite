//! Named warning printer with owned messages and a native trace threshold.
use martensite_tkernel::{
    message::{Gravity, Messenger, Printer, PrinterBase, PrinterType},
    standard::{RuntimeType, RuntimeTyped},
    tcollection::AsciiString,
};
use std::sync::{Arc, Mutex};

pub struct HeaderWarningPrinter {
    base: PrinterBase,
    pub messages: Mutex<Vec<(Vec<u8>, Gravity)>>,
}
impl Printer for HeaderWarningPrinter {
    fn printer_base(&self) -> &PrinterBase {
        &self.base
    }
    fn dynamic_type(&self) -> &'static RuntimeType {
        PrinterType::static_type()
    }
    fn send(&self, text: &AsciiString, gravity: Gravity) {
        self.messages
            .lock()
            .unwrap()
            .push((text.as_bytes().to_vec(), gravity));
    }
}
pub fn recorder() -> (Arc<Messenger>, Arc<HeaderWarningPrinter>) {
    let printer = Arc::new(HeaderWarningPrinter {
        base: PrinterBase::new(Gravity::Trace),
        messages: Mutex::new(Vec::new()),
    });
    (Arc::new(Messenger::with_printer(printer.clone())), printer)
}
