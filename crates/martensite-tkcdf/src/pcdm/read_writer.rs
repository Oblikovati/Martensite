//! `PCDM_ReadWriter`: reads and writes the information section (references, extensions,
//! version) of a stored document.
use super::{PcdmReference, native_header};
use crate::cdm::CdmDocument;
use crate::live;
use martensite_tkernel::{
    impl_transient_interface,
    message::Messenger,
    standard::{Failure, Handle, StandardTransient, Transient},
    storage::Data,
    tcollection::{AsciiString, ExtendedString},
};
use std::{io, sync::Arc};

/// The `FILE_FORMAT: ` user-info key.
pub(super) const FILE_FORMAT: &str = "FILE_FORMAT: ";

/// `PCDM_ReadWriter`'s pure virtual members: one header layout version.
pub trait PcdmReadWriter: Transient {
    /// `Version`: the layout version this object reads and writes.
    fn version(&self) -> AsciiString;
    /// `WriteReferenceCounter`.
    fn write_reference_counter(
        &self,
        data: &Handle<Data>,
        document: &Handle<dyn CdmDocument>,
    ) -> Result<(), Failure>;
    /// `WriteReferences`.
    fn write_references(
        &self,
        data: &Handle<Data>,
        document: &Handle<dyn CdmDocument>,
        referencer_file_name: &ExtendedString,
    ) -> Result<(), Failure>;
    /// `WriteExtensions`.
    fn write_extensions(
        &self,
        data: &Handle<Data>,
        document: &Handle<dyn CdmDocument>,
    ) -> Result<(), Failure>;
    /// `WriteVersion`.
    fn write_version(
        &self,
        data: &Handle<Data>,
        document: &Handle<dyn CdmDocument>,
    ) -> Result<(), Failure>;
    /// `ReadReferenceCounter`.
    fn read_reference_counter(
        &self,
        file_name: &ExtendedString,
        messenger: &Arc<Messenger>,
    ) -> Result<i32, Failure>;
    /// `ReadReferences`.
    fn read_references(
        &self,
        file_name: &ExtendedString,
        references: &mut Vec<PcdmReference>,
        messenger: &Arc<Messenger>,
    ) -> Result<(), Failure>;
    /// `ReadExtensions`.
    fn read_extensions(
        &self,
        file_name: &ExtendedString,
        extensions: &mut Vec<ExtendedString>,
        messenger: &Arc<Messenger>,
    ) -> Result<(), Failure>;
    /// `ReadDocumentVersion`.
    fn read_document_version(
        &self,
        file_name: &ExtendedString,
        messenger: &Arc<Messenger>,
    ) -> Result<i32, Failure>;
}
impl_transient_interface!(dyn PcdmReadWriter, "PCDM_ReadWriter", StandardTransient);

/// The static members of `PCDM_ReadWriter`.
pub struct PcdmReadWriterBase;
impl PcdmReadWriterBase {
    /// `WriteFileFormat`: adds `FILE_FORMAT: <storage format>` (non-Latin-1 units as `?`)
    /// to the user information of `data`.
    pub fn write_file_format(
        data: &Handle<Data>,
        document: &Handle<dyn CdmDocument>,
    ) -> Result<(), Failure> {
        let format = live(document, "PCDM_ReadWriter::WriteFileFormat")?.storage_format();
        let mut line = AsciiString::new(FILE_FORMAT);
        line.append(&AsciiString::from_extended(&format, Some(b'?')));
        live(data, "PCDM_ReadWriter::WriteFileFormat")?.add_to_user_info(&line);
        Ok(())
    }
    /// `ParseReference`: splits `"<identifier> <version> <file name>"`; false, outputs
    /// unchanged, unless both numbers are integers and the file name is not empty.
    ///
    /// ```
    /// use martensite_tkcdf::pcdm::PcdmReadWriterBase;
    /// use martensite_tkernel::tcollection::ExtendedString;
    /// let (mut id, mut version, mut file) = (0, 0, ExtendedString::default());
    /// let text = ExtendedString::new("12 3 a b.ext");
    /// assert!(PcdmReadWriterBase::parse_reference(&text, &mut id, &mut version, &mut file));
    /// assert_eq!((id, version, file), (12, 3, ExtendedString::new("a b.ext")));
    /// ```
    pub fn parse_reference(
        reference: &ExtendedString,
        identifier: &mut i32,
        document_version: &mut i32,
        file_name: &mut ExtendedString,
    ) -> bool {
        let Some((parsed_identifier, parsed_version, parsed_file)) = split_reference(reference)
        else {
            return false;
        };
        *identifier = parsed_identifier;
        *document_version = parsed_version;
        *file_name = parsed_file;
        true
    }
    /// `FileFormat(theIStream, theData)` on explicit-state input: the container's
    /// `FILE_FORMAT` user info (a binary container's information is read into `storage`)
    /// or an XML document's `format` attribute; empty for unknown input. Stream exceptions
    /// follow the exception mask; driver exceptions propagate as kernel failures.
    /// The reader's position is its byte count since it was handed over: a binary
    /// container's section offsets are measured from there.
    /// Example: `PcdmReadWriterBase::file_format_input(&mut input, &mut storage)?;`.
    pub fn file_format_input(
        stream: &mut super::PcdmInputStream<'_>,
        storage: &mut Handle<Data>,
    ) -> Result<ExtendedString, super::PcdmStreamFailure> {
        super::stream_format::probe_input_format(stream, storage).map(|probed| probed.format)
    }

    /// `FileFormat(theIStream, theData)` on an owned storage stream, whose position is
    /// exact: only the container information, or the XML prolog and document tag, is read.
    /// Example: `let format = PcdmReadWriterBase::file_format_storage_stream(&mut stream, &mut storage)?;`.
    pub fn file_format_storage_stream(
        stream: &mut martensite_tkernel::storage::IoStream,
        storage: &mut Handle<Data>,
    ) -> Result<ExtendedString, Failure> {
        super::stream_format::file_format_storage(stream, storage)
    }

    /// Convenience filename format query, retaining the original infallible Rust interface.
    /// Use `file_format_checked` when native driver exceptions must propagate.
    pub fn file_format(file_name: &ExtendedString) -> ExtendedString {
        native_header::file_format(file_name)
    }
    /// Read the filename header with explicit native exception propagation.
    /// Example: `let format = PcdmReadWriterBase::file_format_checked(&file_name)?;`.
    pub fn file_format_checked(file_name: &ExtendedString) -> Result<ExtendedString, Failure> {
        super::file_header::file_format(file_name)
    }
    /// `FileFormat(theIStream, theData)` with the default mask; a driver exception gives
    /// an empty format. A binary container's information is staged from the reader, so
    /// it may read past the information section; use `file_format_input` with
    /// `CDF_Application` reads, which continue from the staged bytes.
    /// Example: `PcdmReadWriterBase::file_format_stream(&mut input, &mut storage)`.
    pub fn file_format_stream(
        stream: &mut dyn io::Read,
        data: &mut Handle<Data>,
    ) -> ExtendedString {
        Self::file_format_input(&mut super::PcdmInputStream::new(stream), data).unwrap_or_default()
    }
}

/// The three fields of a reference line, or `None` for a malformed one.
fn split_reference(reference: &ExtendedString) -> Option<(i32, i32, ExtendedString)> {
    let space = [u16::from(b' ')];
    let first = reference.search(space);
    if first <= 0 || first >= reference.length() {
        return None;
    }
    let mut identifier = reference.clone();
    let mut rest = identifier.split(first).ok()?;
    let second = rest.search(space);
    if second <= 0 || second >= rest.length() {
        return None;
    }
    let file = rest.split(second).ok()?;
    if file.is_empty() || !identifier.is_integer_value() || !rest.is_integer_value() {
        return None;
    }
    Some((
        identifier.integer_value().ok()?,
        rest.integer_value().ok()?,
        file,
    ))
}
