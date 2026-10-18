//! Explicit first-failure delivery, leaving infallible entry points unchanged.
use super::Messenger;
use crate::message::Gravity;
use crate::standard::{Failure, Handle, StandardTransient};
use crate::tcollection::{AsciiString, ExtendedString};

impl Messenger {
    /// Sends a warning and returns the first delivery failure; `messenger.try_send("text")?`.
    pub fn try_send(&self, text: &str) -> Result<(), Failure> {
        self.try_send_c_str(text.as_bytes(), Gravity::Warning)
    }
    /// Delivers bytes in snapshot order, stopping at the first failure.
    /// Example: `messenger.try_send_c_str(b"text", Gravity::Info)?`.
    pub fn try_send_c_str(&self, text: &[u8], gravity: Gravity) -> Result<(), Failure> {
        for printer in self.snapshot() {
            printer.try_send_c_str(text, gravity)?;
        }
        Ok(())
    }
    /// Delivers stream bytes in snapshot order; `messenger.try_send_string_stream(b"x", g)?`.
    pub fn try_send_string_stream(&self, stream: &[u8], gravity: Gravity) -> Result<(), Failure> {
        for printer in self.snapshot() {
            printer.try_send_string_stream(stream, gravity)?;
        }
        Ok(())
    }
    /// Delivers ASCII text in snapshot order; `messenger.try_send_ascii(&text, g)?`.
    pub fn try_send_ascii(&self, text: &AsciiString, gravity: Gravity) -> Result<(), Failure> {
        for printer in self.snapshot() {
            printer.try_send_ascii(text, gravity)?;
        }
        Ok(())
    }
    /// Delivers extended text in snapshot order; `messenger.try_send_extended(&text, g)?`.
    pub fn try_send_extended(
        &self,
        text: &ExtendedString,
        gravity: Gravity,
    ) -> Result<(), Failure> {
        for printer in self.snapshot() {
            printer.try_send_extended(text, gravity)?;
        }
        Ok(())
    }
    /// Delivers a non-null object in snapshot order; `messenger.try_send_object(&object, g)?`.
    pub fn try_send_object(
        &self,
        object: &Handle<StandardTransient>,
        gravity: Gravity,
    ) -> Result<(), Failure> {
        for printer in self.snapshot() {
            printer.try_send_object(object, gravity)?;
        }
        Ok(())
    }
}
