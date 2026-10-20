//! Injected fallible queries wrap the original infallible fake without changing its defaults.
use super::callbacks::{FakeHeaderApplication, FakeRetrievalMetaDataDriver, legacy_driver};
use martensite_tkcdf::{
    cdf::CdfMetaDataDriver,
    cdm::{CdmDocument, CdmMetaData},
};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, StandardTransient},
    tcollection::ExtendedString,
};
use std::sync::Arc;

pub type QueryHook = Arc<
    dyn Fn(&str, &ExtendedString, &ExtendedString, &ExtendedString) -> Result<(), Failure>
        + Send
        + Sync,
>;
pub struct FallibleRetrievalDriver {
    transient: StandardTransient,
    legacy: FakeRetrievalMetaDataDriver,
    hook: Option<QueryHook>,
}
impl_transient!(FallibleRetrievalDriver, "FallibleRetrievalDriver", StandardTransient,
    transient, interfaces: [dyn CdfMetaDataDriver]);

/// Customize query callbacks; `configured_driver(&app, meta, true, true, false, None)`.
pub fn configured_driver(
    app: &FakeHeaderApplication,
    metadata: Handle<CdmMetaData>,
    found: bool,
    readable: bool,
    fail_metadata: bool,
    hook: Option<QueryHook>,
) -> Handle<FallibleRetrievalDriver> {
    Handle::new(FallibleRetrievalDriver {
        transient: StandardTransient::default(),
        legacy: legacy_driver(app, metadata, found, readable, fail_metadata),
        hook,
    })
}
impl FallibleRetrievalDriver {
    fn query_callback(
        &self,
        phase: &str,
        folder: &ExtendedString,
        name: &ExtendedString,
        version: &ExtendedString,
    ) -> Result<(), Failure> {
        match &self.hook {
            Some(hook) => hook(phase, folder, name, version),
            None => Ok(()),
        }
    }
}
impl CdfMetaDataDriver for FallibleRetrievalDriver {
    fn find(
        &self,
        folder: &ExtendedString,
        name: &ExtendedString,
        version: &ExtendedString,
    ) -> bool {
        self.legacy.find(folder, name, version)
    }
    fn try_find(
        &self,
        folder: &ExtendedString,
        name: &ExtendedString,
        version: &ExtendedString,
    ) -> Result<bool, Failure> {
        let found = self.find(folder, name, version);
        self.query_callback("find", folder, name, version)?;
        Ok(found)
    }
    fn has_read_permission(
        &self,
        folder: &ExtendedString,
        name: &ExtendedString,
        version: &ExtendedString,
    ) -> bool {
        self.legacy.has_read_permission(folder, name, version)
    }
    fn try_has_read_permission(
        &self,
        folder: &ExtendedString,
        name: &ExtendedString,
        version: &ExtendedString,
    ) -> Result<bool, Failure> {
        let readable = self.has_read_permission(folder, name, version);
        self.query_callback("permission", folder, name, version)?;
        Ok(readable)
    }
    fn meta_data(
        &self,
        folder: &ExtendedString,
        name: &ExtendedString,
        version: &ExtendedString,
    ) -> Result<Handle<CdmMetaData>, Failure> {
        let result = self.legacy.meta_data(folder, name, version);
        self.query_callback("metadata", folder, name, version)?;
        result
    }
    fn build_file_name(
        &self,
        document: &Handle<dyn CdmDocument>,
    ) -> Result<ExtendedString, Failure> {
        self.legacy.build_file_name(document)
    }
    fn create_meta_data(
        &self,
        document: &Handle<dyn CdmDocument>,
        file: &ExtendedString,
    ) -> Result<Handle<CdmMetaData>, Failure> {
        self.legacy.create_meta_data(document, file)
    }
    fn find_folder(&self, folder: &ExtendedString) -> bool {
        self.legacy.find_folder(folder)
    }
    fn default_folder(&self) -> ExtendedString {
        self.legacy.default_folder()
    }
}
