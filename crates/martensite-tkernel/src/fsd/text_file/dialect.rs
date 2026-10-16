//! The overridable members that the compatible dialect replaces.
use super::TextFile;
use crate::standard::Failure;
use crate::tcollection::{AsciiString, ExtendedString};
mod cmp;

/// The members that differ between the text dialect ([`FsdDialect`]) and the compatible
/// dialect ([`CmpDialect`]); every other member of [`TextFile`] is shared.
pub trait TextDialect: Sized {
    /// The magic number.
    const MAGIC_NUMBER: &'static str;

    /// Selects the four compatible-dialect text bodies for hooks this dialect does not override.
    ///
    /// ```
    /// use martensite_tkernel::fsd::TextDialect;
    /// struct DerivedCmp;
    /// impl TextDialect for DerivedCmp {
    ///     const MAGIC_NUMBER: &'static str = "CUSTOM";
    ///     const USE_CMP_TEXT_DEFAULTS: bool = true;
    ///     const USE_CMP_OBJECT_DEFAULTS: bool = true;
    /// }
    /// ```
    const USE_CMP_TEXT_DEFAULTS: bool = false;

    /// Selects compatible-dialect object-framing bodies for hooks this dialect does not override.
    const USE_CMP_OBJECT_DEFAULTS: bool = false;

    /// Overridable line reader; `MyDialect::read_line(file)`.
    fn read_line(file: &mut TextFile<Self>) -> AsciiString {
        if Self::USE_CMP_TEXT_DEFAULTS {
            cmp::read_line(file)
        } else {
            file.read_fsd_line_base()
        }
    }

    /// Overridable string reader; `MyDialect::read_string(file)`.
    fn read_string(file: &mut TextFile<Self>) -> AsciiString {
        if Self::USE_CMP_TEXT_DEFAULTS {
            cmp::read_string(file)
        } else {
            file.read_fsd_string_base()
        }
    }

    /// Overridable character reader; `MyDialect::read_char(file, 4)` reads up to four bytes.
    fn read_char(file: &mut TextFile<Self>, size: usize) -> AsciiString {
        file.read_fsd_char_base(size)
    }

    /// Overridable word reader; `MyDialect::read_word(file)` reads the next word.
    fn read_word(file: &mut TextFile<Self>) -> AsciiString {
        file.read_fsd_word_base()
    }

    /// Overridable extended-line reader.
    fn read_extended_line(
        file: &mut TextFile<Self>,
        text: &mut ExtendedString,
    ) -> Result<(), Failure> {
        if Self::USE_CMP_TEXT_DEFAULTS {
            cmp::read_extended_line(file, text)
        } else {
            file.read_fsd_extended_line_base(text)
        }
    }

    /// Overridable extended-line writer.
    fn write_extended_line(
        file: &mut TextFile<Self>,
        text: &ExtendedString,
    ) -> Result<(), Failure> {
        if Self::USE_CMP_TEXT_DEFAULTS {
            cmp::write_extended_line(file, text)
        } else {
            file.write_fsd_extended_line_base(text);
            Ok(())
        }
    }

    /// Overridable end-of-line flush; `MyDialect::flush_end_of_line(file)`.
    fn flush_end_of_line(file: &mut TextFile<Self>) {
        file.flush_fsd_end_of_line_base();
    }

    /// Overridable tag search; `MyDialect::find_tag(file, "BEGIN_INFO_SECTION")`.
    fn find_tag(file: &mut TextFile<Self>, tag: &str) -> crate::storage::StorageError {
        file.find_fsd_tag_base(tag)
    }

    /// Writes a persistent object header.
    fn write_object_header(
        file: &mut TextFile<Self>,
        reference: i32,
        type_number: i32,
    ) -> Result<(), Failure> {
        if Self::USE_CMP_OBJECT_DEFAULTS {
            cmp::write_object_header(file, reference, type_number)
        } else {
            file.write_fsd_object_header_base(reference, type_number)
        }
    }

    /// Begins writing persistent object data or object data (the same text).
    fn begin_write_object(file: &mut TextFile<Self>, member: &str) -> Result<(), Failure> {
        if Self::USE_CMP_OBJECT_DEFAULTS {
            file.begin_cmp_write_object_base(member)
        } else {
            file.begin_fsd_write_object_base(member)
        }
    }

    /// Ends writing object data (`last` false) or persistent object data (`last` true).
    fn end_write_object(
        file: &mut TextFile<Self>,
        last: bool,
        member: &str,
    ) -> Result<(), Failure> {
        if Self::USE_CMP_OBJECT_DEFAULTS {
            file.end_cmp_write_object_base(last, member)
        } else {
            file.end_fsd_write_object_base(last, member)
        }
    }

    /// Reads a persistent object header.
    fn read_object_header(
        file: &mut TextFile<Self>,
        reference: &mut i32,
        type_number: &mut i32,
    ) -> Result<(), Failure> {
        if Self::USE_CMP_OBJECT_DEFAULTS {
            file.read_cmp_object_header_base(reference, type_number)
        } else {
            file.read_fsd_object_header_base(reference, type_number)
        }
    }

    /// Begins reading persistent object data or object data.
    fn begin_read_object(file: &mut TextFile<Self>) -> Result<(), Failure> {
        if Self::USE_CMP_OBJECT_DEFAULTS {
            file.begin_cmp_read_object_base()
        } else {
            file.begin_fsd_read_object_base()
        }
    }

    /// Ends reading object data (`last` false) or persistent object data (`last` true).
    fn end_read_object(file: &mut TextFile<Self>, last: bool) -> Result<(), Failure> {
        if Self::USE_CMP_OBJECT_DEFAULTS {
            file.end_cmp_read_object_base(last)
        } else {
            file.end_fsd_read_object_base(last)
        }
    }
}

/// The text dialect's own members.
#[derive(Clone, Copy, Debug, Default)]
pub struct FsdDialect;

impl TextDialect for FsdDialect {
    const MAGIC_NUMBER: &'static str = "FSDFILE";
}

/// The compatible dialect: carriage returns are tolerated at line ends, extended lines
/// are written as a count and per-unit ext-character values, and objects are framed by their
/// header alone.
#[derive(Clone, Copy, Debug, Default)]
pub struct CmpDialect;

impl TextDialect for CmpDialect {
    const MAGIC_NUMBER: &'static str = "CMPFILE";
    const USE_CMP_TEXT_DEFAULTS: bool = true;
    const USE_CMP_OBJECT_DEFAULTS: bool = true;
}
