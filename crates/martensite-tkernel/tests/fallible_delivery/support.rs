use martensite_tkernel::message::{Gravity, Messenger, Printer, PrinterBase, PrinterType};
use martensite_tkernel::standard::{Failure, RuntimeType, RuntimeTyped};
use martensite_tkernel::tcollection::AsciiString;
use std::sync::{Arc, Mutex, Weak};

pub type DeliveryRecord = (Vec<u8>, Gravity);
#[derive(Default)]
pub struct LegacyRecorder {
    base: PrinterBase,
    pub records: Mutex<Vec<DeliveryRecord>>,
}
impl Printer for LegacyRecorder {
    fn printer_base(&self) -> &PrinterBase {
        &self.base
    }
    fn dynamic_type(&self) -> &'static RuntimeType {
        PrinterType::static_type()
    }
    fn send(&self, text: &AsciiString, gravity: Gravity) {
        self.records
            .lock()
            .unwrap()
            .push((text.as_bytes().to_vec(), gravity));
    }
}

pub struct RefusingPrinter {
    base: PrinterBase,
    pub ordinary: LegacyRecorder,
    pub attempts: Mutex<Vec<DeliveryRecord>>,
    pub failure: Failure,
}
impl RefusingPrinter {
    pub fn new() -> Self {
        Self {
            base: PrinterBase::default(),
            ordinary: LegacyRecorder::default(),
            attempts: Mutex::default(),
            failure: Failure::with_type(
                "DeliveryRefused",
                "sink quota 7, expected available capacity",
                "owned stack",
            ),
        }
    }
}
impl Printer for RefusingPrinter {
    fn printer_base(&self) -> &PrinterBase {
        &self.base
    }
    fn dynamic_type(&self) -> &'static RuntimeType {
        PrinterType::static_type()
    }
    fn send(&self, text: &AsciiString, gravity: Gravity) {
        self.ordinary.send(text, gravity);
    }
    fn try_send(&self, text: &AsciiString, gravity: Gravity) -> Result<(), Failure> {
        self.attempts
            .lock()
            .unwrap()
            .push((text.as_bytes().to_vec(), gravity));
        Err(self.failure.clone())
    }
}

pub struct ReentrantEditor {
    base: PrinterBase,
    pub target: Mutex<Weak<Messenger>>,
    pub replacement: Arc<LegacyRecorder>,
    entered: std::sync::atomic::AtomicBool,
}
impl ReentrantEditor {
    pub fn new(replacement: Arc<LegacyRecorder>) -> Self {
        Self {
            base: PrinterBase::default(),
            target: Mutex::default(),
            replacement,
            entered: std::sync::atomic::AtomicBool::new(false),
        }
    }
}
impl Printer for ReentrantEditor {
    fn printer_base(&self) -> &PrinterBase {
        &self.base
    }
    fn dynamic_type(&self) -> &'static RuntimeType {
        PrinterType::static_type()
    }
    fn send(&self, _: &AsciiString, _: Gravity) {}
    fn try_send(&self, _: &AsciiString, _: Gravity) -> Result<(), Failure> {
        if self.entered.swap(true, std::sync::atomic::Ordering::SeqCst) {
            return Ok(());
        }
        let target = self.target.lock().unwrap().upgrade().unwrap();
        *target.printers_mut() = vec![self.replacement.clone()];
        target.try_send("inner")
    }
}

pub fn send_concurrent_workload(messenger: &Arc<Messenger>) {
    std::thread::scope(|scope| {
        for worker in 0..4 {
            let messenger = messenger.clone();
            scope.spawn(move || {
                for message in 0..1000 {
                    messenger.try_send(&format!("{worker}:{message}")).unwrap();
                }
            });
        }
    });
}
