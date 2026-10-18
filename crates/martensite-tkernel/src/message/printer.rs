//! An output context for text messages filtered by gravity.
use super::Gravity;
use crate::standard::{Failure, Handle, RuntimeType, StandardTransient};
use crate::tcollection::{AsciiString, ExtendedString};
use std::any::Any;
use std::sync::atomic::{AtomicI32, Ordering};

/// Shared printer handle; printers are never null.
pub type PrinterHandle = std::sync::Arc<dyn Printer>;

/// State every printer embeds: the trace level, default `Gravity::Info`.
#[derive(Debug)]
pub struct PrinterBase {
    trace_level: AtomicI32,
}
impl Default for PrinterBase {
    fn default() -> Self {
        Self::new(Gravity::Info)
    }
}
impl PrinterBase {
    /// Creates the state with a trace level; `PrinterBase::new(Gravity::Info)`.
    pub fn new(trace_level: Gravity) -> Self {
        Self {
            trace_level: AtomicI32::new(trace_level as i32),
        }
    }
    /// Messages with a lower gravity are ignored; `base.trace_level()`.
    pub fn trace_level(&self) -> Gravity {
        Gravity::ALL[self.trace_level.load(Ordering::Relaxed) as usize]
    }
    /// Changes the filter threshold; `base.set_trace_level(Gravity::Trace)`.
    pub fn set_trace_level(&self, level: Gravity) {
        self.trace_level.store(level as i32, Ordering::Relaxed);
    }
}

/// Marker type carrying the printer runtime descriptor.
pub struct PrinterType;
message_runtime_type!(PrinterType, "Message_Printer", StandardTransient);

/// A message output; implement `send` and embed a
/// [`PrinterBase`]; the public `send_*` overloads filter by trace level first.
///
/// ```
/// use martensite_tkernel::message::{Gravity, Printer, PrinterBase, PrinterType};
/// use martensite_tkernel::standard::{RuntimeType, RuntimeTyped};
/// use martensite_tkernel::tcollection::AsciiString;
/// struct Silent(PrinterBase);
/// impl Printer for Silent {
///     fn printer_base(&self) -> &PrinterBase { &self.0 }
///     fn dynamic_type(&self) -> &'static RuntimeType { PrinterType::static_type() }
///     fn send(&self, _: &AsciiString, _: Gravity) {}
/// }
/// Silent(PrinterBase::default()).send_c_str(b"ignored", Gravity::Fail);
/// ```
pub trait Printer: Any + Send + Sync {
    /// The embedded trace-level state; `printer.printer_base()`.
    fn printer_base(&self) -> &PrinterBase;
    /// The most-derived descriptor, used by `Messenger::remove_printers`; `printer.dynamic_type()`.
    fn dynamic_type(&self) -> &'static RuntimeType;
    /// Receives one message that passed the filter.
    fn send(&self, text: &AsciiString, gravity: Gravity);

    /// Delivers an already filtered message with an owned failure channel.
    /// Existing printers delegate to `send`; `printer.try_send(&text, g)?`.
    fn try_send(&self, text: &AsciiString, gravity: Gravity) -> Result<(), Failure> {
        self.send(text, gravity);
        Ok(())
    }

    /// Filters and delivers an ASCII message; `printer.try_send_ascii(&text, g)?`.
    fn try_send_ascii(&self, text: &AsciiString, gravity: Gravity) -> Result<(), Failure> {
        if gravity < self.trace_level() {
            return Ok(());
        }
        self.try_send(text, gravity)
    }

    /// Filters before UTF-8 conversion; `printer.try_send_extended(&text, g)?`.
    fn try_send_extended(&self, text: &ExtendedString, gravity: Gravity) -> Result<(), Failure> {
        if gravity < self.trace_level() {
            return Ok(());
        }
        self.try_send(&AsciiString::from(text), gravity)
    }

    /// Delivers bytes up to the first NUL; `printer.try_send_c_str(b"text", g)?`.
    fn try_send_c_str(&self, text: &[u8], gravity: Gravity) -> Result<(), Failure> {
        if gravity < self.trace_level() {
            return Ok(());
        }
        self.try_send(&AsciiString::from_bytes(text), gravity)
    }

    /// Delivers stream bytes up to the first NUL; `printer.try_send_string_stream(b"x", g)?`.
    fn try_send_string_stream(&self, stream: &[u8], gravity: Gravity) -> Result<(), Failure> {
        self.try_send_c_str(stream, gravity)
    }

    /// Delivers a non-null object's description; `printer.try_send_object(&object, g)?`.
    fn try_send_object(
        &self,
        object: &Handle<StandardTransient>,
        gravity: Gravity,
    ) -> Result<(), Failure> {
        if !object.is_some() || gravity < self.trace_level() {
            return Ok(());
        }
        self.try_send(&object_description(object), gravity)
    }

    /// Example: `printer.trace_level()`.
    fn trace_level(&self) -> Gravity {
        self.printer_base().trace_level()
    }
    /// Example: `printer.set_trace_level(Gravity::Trace)`.
    fn set_trace_level(&self, level: Gravity) {
        self.printer_base().set_trace_level(level);
    }
    /// Sends an extended string, converted to UTF-8; `printer.send_extended(&text, g)`.
    fn send_extended(&self, text: &ExtendedString, gravity: Gravity) {
        if gravity >= self.trace_level() {
            self.send(&AsciiString::from(text), gravity);
        }
    }
    /// Bytes up to the first NUL; `printer.send_c_str(b"text", g)`.
    fn send_c_str(&self, text: &[u8], gravity: Gravity) {
        if gravity >= self.trace_level() {
            self.send(&AsciiString::from_bytes(text), gravity);
        }
    }
    /// Sends an ASCII string; `printer.send_ascii(&text, g)`.
    fn send_ascii(&self, text: &AsciiString, gravity: Gravity) {
        if gravity >= self.trace_level() {
            self.send(text, gravity);
        }
    }
    /// The stream's C string (cut at a NUL); `printer.send_string_stream(b"x", g)`.
    fn send_string_stream(&self, stream: &[u8], gravity: Gravity) {
        if gravity >= self.trace_level() {
            self.send(&AsciiString::from_bytes(stream), gravity);
        }
    }
    /// `"<type name>: <pointer>"` for a non-null object; `printer.send_object(&object, g)`.
    fn send_object(&self, object: &Handle<StandardTransient>, gravity: Gravity) {
        if object.is_some() && gravity >= self.trace_level() {
            self.send(&object_description(object), gravity);
        }
    }
}

/// The object's runtime type name, `": "`, and its pointer text.
pub(crate) fn object_description(object: &Handle<StandardTransient>) -> AsciiString {
    let name = object.dynamic_type().map_or("", RuntimeType::name);
    let mut text = AsciiString::new(name);
    text.assign_cat(": ");
    text.assign_cat(super::json::pointer_info(object.as_ptr().cast()).as_str());
    text
}
