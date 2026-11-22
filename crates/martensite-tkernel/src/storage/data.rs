//! The picture of the data stored in, or read from, a container.
use super::{HeaderData, InternalData, Root, RootData, SectionStatus, StorageError, TypeData};
use crate::ncollection::{HSequence, Sequence};
use crate::standard::{Handle, Persistent, StandardTransient};
use crate::tcollection::{AsciiString, ExtendedString};
use std::sync::{Mutex, MutexGuard, PoisonError};

/// Roots, types, header and bookkeeping of one container, shared by handles.
///
/// The error status starts at `Ok`.
///
/// ```
/// use martensite_tkernel::standard::{Handle, Persistent, StandardPersistent};
/// use martensite_tkernel::storage::Data;
/// use martensite_tkernel::tcollection::AsciiString;
/// let data = Data::default();
/// let object = Handle::new(StandardPersistent::default()).into_upcast::<dyn Persistent>();
/// data.add_named_root(&AsciiString::new("1"), &object);
/// assert_eq!(data.number_of_roots(), 1);
/// ```
pub struct Data {
    header: Handle<HeaderData>,
    roots: Handle<RootData>,
    types: Handle<TypeData>,
    internal: Handle<InternalData>,
    status: Mutex<SectionStatus>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            roots: Handle::new(RootData::default()),
            types: Handle::new(TypeData::default()),
            internal: Handle::new(InternalData::default()),
            header: Handle::new(HeaderData::default()),
            status: Mutex::new(SectionStatus::default()),
        }
    }
}

impl Data {
    fn header(&self) -> &HeaderData {
        self.header.get().expect("Storage_Data owns its header")
    }

    fn root_section(&self) -> &RootData {
        self.roots.get().expect("Storage_Data owns its roots")
    }

    fn type_section(&self) -> &TypeData {
        self.types.get().expect("Storage_Data owns its types")
    }

    fn status(&self) -> MutexGuard<'_, SectionStatus> {
        self.status.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The status of the last schema read or write; `data.error_status()`.
    pub fn error_status(&self) -> StorageError {
        self.status().error
    }

    /// Clears this, the header, root and type statuses.
    /// Example: `data.clear_error_status()`.
    pub(super) fn clear_error_status(&self) {
        self.status().clear();
        self.header().clear_error_status();
        self.root_section().clear_error_status();
        self.type_section().clear_error_status();
    }

    /// Example: `data.error_status_extension()`.
    pub fn error_status_extension(&self) -> AsciiString {
        self.status().extension.clone()
    }

    /// The status setters the schema uses.
    pub(crate) fn set_status(&self, error: StorageError, extension: &str) {
        let mut status = self.status();
        status.error = error;
        status.extension = AsciiString::new(extension);
    }

    /// `HeaderData`; `data.header_data()`.
    pub fn header_data(&self) -> Handle<HeaderData> {
        self.header.clone()
    }

    /// `RootData`; `data.root_data()`.
    pub(crate) fn root_data(&self) -> Handle<RootData> {
        self.roots.clone()
    }

    /// `TypeData`; `data.type_data()`.
    pub fn type_data(&self) -> Handle<TypeData> {
        self.types.clone()
    }

    /// `InternalData`; `data.internal_data()`.
    pub(crate) fn internal_data(&self) -> Handle<InternalData> {
        self.internal.clone()
    }

    /// Resets the bookkeeping and the type section; `data.clear()`.
    pub fn clear(&self) {
        if let Some(internal) = self.internal.get() {
            internal.clear();
        }
        self.type_section().clear();
    }
}

impl Data {
    /// Named by the next root number; `data.add_root(&object)`.
    #[cfg(test)]
    pub(crate) fn add_root(&self, object: &Handle<dyn Persistent>) {
        let name = AsciiString::from_integer(self.root_section().number_of_roots() + 1);
        self.add_named_root(&name, object);
    }

    /// Example: `data.add_named_root(&name, &object)`.
    pub fn add_named_root(&self, name: &AsciiString, object: &Handle<dyn Persistent>) {
        self.root_section()
            .add_root(&Handle::new(Root::new(name, object)));
    }

    /// Nothing happens for an unknown name; `data.remove_root(&name)`.
    #[cfg(test)]
    pub(crate) fn remove_root(&self, name: &AsciiString) {
        self.root_section().remove_root(name);
    }

    /// Example: `data.roots()`.
    pub(super) fn roots(&self) -> Handle<HSequence<Handle<Root>>> {
        self.root_section().roots()
    }

    /// The named root or a null handle; `data.find(&name)`.
    #[cfg(test)]
    pub(crate) fn find(&self, name: &AsciiString) -> Handle<Root> {
        self.root_section().find(name)
    }

    /// Example: `data.number_of_roots()`.
    pub fn number_of_roots(&self) -> i32 {
        self.root_section().number_of_roots()
    }

    /// Example: `data.is_root(&name)`.
    #[cfg(test)]
    pub(crate) fn is_root(&self, name: &AsciiString) -> bool {
        self.root_section().is_root(name)
    }

    /// Example: `data.number_of_types()`.
    pub fn number_of_types(&self) -> i32 {
        self.type_section().number_of_types()
    }

    /// Example: `data.is_type(&name)`.
    #[cfg(test)]
    pub(crate) fn is_type(&self, name: &AsciiString) -> bool {
        self.type_section().is_type(name)
    }

    /// Example: `data.types()`.
    pub(super) fn types(&self) -> Handle<HSequence<AsciiString>> {
        self.type_section().types()
    }
}

impl Data {
    /// Example: `data.creation_date()`.
    pub(super) fn creation_date(&self) -> AsciiString {
        self.header().creation_date()
    }

    /// Example: `data.storage_version()`.
    pub(super) fn storage_version(&self) -> AsciiString {
        self.header().storage_version()
    }

    /// Example: `data.schema_version()`.
    pub(super) fn schema_version(&self) -> AsciiString {
        self.header().schema_version()
    }

    /// Example: `data.schema_name()`.
    pub(super) fn schema_name(&self) -> AsciiString {
        self.header().schema_name()
    }

    /// Example: `data.set_application_version(&version)`.
    #[cfg(test)]
    pub(crate) fn set_application_version(&self, version: &AsciiString) {
        self.header().set_application_version(version);
    }

    /// Example: `data.application_version()`.
    pub(super) fn application_version(&self) -> AsciiString {
        self.header().application_version()
    }

    /// Example: `data.set_application_name(&name)`.
    #[cfg(test)]
    pub(crate) fn set_application_name(&self, name: &ExtendedString) {
        self.header().set_application_name(name);
    }

    /// Example: `data.application_name()`.
    pub(super) fn application_name(&self) -> ExtendedString {
        self.header().application_name()
    }

    /// Example: `data.set_data_type(&data_type)`.
    #[cfg(test)]
    pub(crate) fn set_data_type(&self, data_type: &ExtendedString) {
        self.header().set_data_type(data_type);
    }

    /// Example: `data.data_type()`.
    pub(super) fn data_type(&self) -> ExtendedString {
        self.header().data_type()
    }

    /// Example: `data.add_to_user_info(&line)`.
    pub fn add_to_user_info(&self, info: &AsciiString) {
        self.header().add_to_user_info(info);
    }

    /// Example: `data.user_info()`.
    pub fn user_info(&self) -> Sequence<AsciiString> {
        self.header().user_info()
    }

    /// Example: `data.add_to_comments(&comment)`.
    #[cfg(test)]
    pub(crate) fn add_to_comments(&self, comment: &ExtendedString) {
        self.header().add_to_comments(comment);
    }

    /// Example: `data.comments()`.
    pub(super) fn comments(&self) -> Sequence<ExtendedString> {
        self.header().comments()
    }

    /// Example: `data.number_of_objects()`.
    pub fn number_of_objects(&self) -> i32 {
        self.header().number_of_objects()
    }
}

crate::impl_transient!(Data, "Storage_Data", StandardTransient);
