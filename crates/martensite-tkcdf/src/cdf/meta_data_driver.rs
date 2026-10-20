//! `CDF_MetaDataDriver`: where an application finds and creates the meta-data of stored
//! documents.
use crate::cdm::{CdmDocument, CdmMetaData};
use crate::pcdm::PcdmReferenceIterator;
use martensite_tkernel::{
    impl_transient_interface,
    message::Messenger,
    standard::{Failure, Handle, StandardTransient, Transient},
    tcollection::ExtendedString,
};
use std::sync::Arc;

/// `CDF_MetaDataDriver`'s members; the protected constructor has no state.
pub trait CdfMetaDataDriver: Transient {
    /// `HasVersionCapability`: false by default.
    fn has_version_capability(&self) -> bool {
        false
    }
    /// `CreateDependsOn`: does nothing by default.
    fn create_depends_on(&self, _first: &Handle<CdmMetaData>, _second: &Handle<CdmMetaData>) {}
    /// `CreateReference`: does nothing by default.
    fn create_reference(
        &self,
        _from: &Handle<CdmMetaData>,
        _to: &Handle<CdmMetaData>,
        _reference_identifier: i32,
        _to_document_version: i32,
    ) {
    }
    /// `HasVersion(folder, name)`: true by default.
    fn has_version(&self, _folder: &ExtendedString, _name: &ExtendedString) -> bool {
        true
    }
    /// `BuildFileName` (pure virtual): the file name a document is stored in.
    fn build_file_name(
        &self,
        document: &Handle<dyn CdmDocument>,
    ) -> Result<ExtendedString, Failure>;
    /// `SetName`: the name itself by default.
    fn set_name(
        &self,
        _document: &Handle<dyn CdmDocument>,
        name: &ExtendedString,
    ) -> Result<ExtendedString, Failure> {
        Ok(name.clone())
    }
    /// `Find(folder, name, version)` (pure virtual): the document exists.
    fn find(
        &self,
        folder: &ExtendedString,
        name: &ExtendedString,
        version: &ExtendedString,
    ) -> bool;
    /// Preserve a provider's typed `Find` failure; `driver.try_find(folder, name, version)?`.
    /// Existing providers keep their infallible query through this default adapter.
    fn try_find(
        &self,
        folder: &ExtendedString,
        name: &ExtendedString,
        version: &ExtendedString,
    ) -> Result<bool, Failure> {
        Ok(self.find(folder, name, version))
    }
    /// `HasReadPermission` (pure virtual).
    fn has_read_permission(
        &self,
        folder: &ExtendedString,
        name: &ExtendedString,
        version: &ExtendedString,
    ) -> bool;
    /// Preserve a permission provider's failure; `driver.try_has_read_permission(folder, name, version)?`.
    /// Existing providers keep their infallible query through this default adapter.
    fn try_has_read_permission(
        &self,
        folder: &ExtendedString,
        name: &ExtendedString,
        version: &ExtendedString,
    ) -> Result<bool, Failure> {
        Ok(self.has_read_permission(folder, name, version))
    }
    /// `MetaData(folder, name, version)` (pure virtual).
    fn meta_data(
        &self,
        folder: &ExtendedString,
        name: &ExtendedString,
        version: &ExtendedString,
    ) -> Result<Handle<CdmMetaData>, Failure>;
    /// `LastVersion`: the meta-data itself by default.
    fn last_version(&self, meta_data: &Handle<CdmMetaData>) -> Handle<CdmMetaData> {
        meta_data.clone()
    }
    /// `CreateMetaData` (pure virtual): the meta-data of a document stored in `file_name`.
    fn create_meta_data(
        &self,
        document: &Handle<dyn CdmDocument>,
        file_name: &ExtendedString,
    ) -> Result<Handle<CdmMetaData>, Failure>;
    /// `FindFolder` (pure virtual).
    fn find_folder(&self, folder: &ExtendedString) -> bool;
    /// `DefaultFolder` (pure virtual).
    fn default_folder(&self) -> ExtendedString;
    /// `ReferenceIterator`: a new `PCDM_ReferenceIterator` by default.
    fn reference_iterator(&self, message_driver: &Arc<Messenger>) -> Handle<PcdmReferenceIterator> {
        Handle::new(PcdmReferenceIterator::new(message_driver))
    }
    /// `Find(folder, name)`: [`CdfMetaDataDriver::find`] with an empty version.
    fn find_without_version(&self, folder: &ExtendedString, name: &ExtendedString) -> bool {
        self.find(folder, name, &ExtendedString::default())
    }
    /// Query the unversioned provider without losing its failure; `driver.try_find_without_version(folder, name)?`.
    fn try_find_without_version(
        &self,
        folder: &ExtendedString,
        name: &ExtendedString,
    ) -> Result<bool, Failure> {
        self.try_find(folder, name, &ExtendedString::default())
    }
    /// `MetaData(folder, name)`: [`CdfMetaDataDriver::meta_data`] with an empty version.
    fn meta_data_without_version(
        &self,
        folder: &ExtendedString,
        name: &ExtendedString,
    ) -> Result<Handle<CdmMetaData>, Failure> {
        self.meta_data(folder, name, &ExtendedString::default())
    }
}
impl_transient_interface!(
    dyn CdfMetaDataDriver,
    "CDF_MetaDataDriver",
    StandardTransient
);
