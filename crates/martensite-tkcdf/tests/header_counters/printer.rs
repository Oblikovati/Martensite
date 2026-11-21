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
    let base = PrinterBase::default();
    base.set_trace_level(Gravity::Trace);
    let printer = Arc::new(HeaderWarningPrinter {
        base,
        messages: Mutex::new(Vec::new()),
    });
    let messenger = Messenger::new();
    *messenger.printers_mut() = vec![printer.clone()];
    (Arc::new(messenger), printer)
}
