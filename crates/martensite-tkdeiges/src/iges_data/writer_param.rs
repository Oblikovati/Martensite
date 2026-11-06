//! The values the writer of a file takes.
use super::{IgesDataEntityKind, IgesDataIgesWriter};
use martensite_tkernel::{
    standard::{Failure, Handle, RuntimeTyped},
    tcollection::HAsciiString,
};
use martensite_tkmath::gp::{Xy, Xyz};

/// A value [`IgesDataIgesWriter::send`] takes: an integer, a real, coordinates, a text, or a
/// handle of any entity class or kind.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataIgesWriter;
/// let mut writer = IgesDataIgesWriter::default();
/// writer.send(3).unwrap();
/// writer.send(0.5).unwrap();
/// ```
pub trait IgesDataWriterParam {
    /// Sends the value to the writer.
    ///
    /// # Errors
    /// A failure of the line buffer or of the formatting of a real.
    fn send_to(self, writer: &mut IgesDataIgesWriter) -> Result<(), Failure>;
}

impl IgesDataWriterParam for i32 {
    fn send_to(self, writer: &mut IgesDataIgesWriter) -> Result<(), Failure> {
        writer.send_integer(self)
    }
}

impl IgesDataWriterParam for f64 {
    fn send_to(self, writer: &mut IgesDataIgesWriter) -> Result<(), Failure> {
        writer.send_real(self)
    }
}

impl IgesDataWriterParam for Xy {
    fn send_to(self, writer: &mut IgesDataIgesWriter) -> Result<(), Failure> {
        writer.send_real(self.x())?;
        writer.send_real(self.y())
    }
}

impl IgesDataWriterParam for &Xy {
    fn send_to(self, writer: &mut IgesDataIgesWriter) -> Result<(), Failure> {
        (*self).send_to(writer)
    }
}

impl IgesDataWriterParam for Xyz {
    fn send_to(self, writer: &mut IgesDataIgesWriter) -> Result<(), Failure> {
        writer.send_real(self.x())?;
        writer.send_real(self.y())?;
        writer.send_real(self.z())
    }
}

impl IgesDataWriterParam for &Xyz {
    fn send_to(self, writer: &mut IgesDataIgesWriter) -> Result<(), Failure> {
        (*self).send_to(writer)
    }
}

impl IgesDataWriterParam for &Handle<HAsciiString> {
    fn send_to(self, writer: &mut IgesDataIgesWriter) -> Result<(), Failure> {
        writer.send_text(self)
    }
}

impl IgesDataWriterParam for Handle<HAsciiString> {
    fn send_to(self, writer: &mut IgesDataIgesWriter) -> Result<(), Failure> {
        writer.send_text(&self)
    }
}

impl<T: ?Sized + RuntimeTyped + IgesDataEntityKind> IgesDataWriterParam for &Handle<T> {
    fn send_to(self, writer: &mut IgesDataIgesWriter) -> Result<(), Failure> {
        writer.send_entity(self, false)
    }
}

impl<T: ?Sized + RuntimeTyped + IgesDataEntityKind> IgesDataWriterParam for Handle<T> {
    fn send_to(self, writer: &mut IgesDataIgesWriter) -> Result<(), Failure> {
        writer.send_entity(&self, false)
    }
}
