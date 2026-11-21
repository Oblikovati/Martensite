//! Behaviour tests of `CDF_Application`.
#![allow(non_snake_case)]
use martensite_tkcdf::{
    SharedResourceManager, cdf::CdfApplication, cdf::CdfApplicationBase,
    cdf::CdfApplicationServices, cdf_application_cdm_members, cdm::CdmApplication,
    pcdm::PcdmReaderStatus,
};
use martensite_tkernel::{
    impl_transient,
    resource::{ResourceHost, ResourceManager},
    standard::{Handle, SharedState, Transient, WeakHandle},
    tcollection::ExtendedString,
};
use std::sync::{Arc, OnceLock};

/// `CDF_TestApplication`: a CDF application with its own empty resource manager.
struct CdfTestApplication {
    base: CdfApplicationBase,
    resources: SharedResourceManager,
}
impl_transient!(CdfTestApplication, "CDF_TestApplication", dyn CdfApplication,
    interfaces: [dyn CdmApplication, dyn CdfApplication]);
impl CdmApplication for CdfTestApplication {
    cdf_application_cdm_members!();
    fn resources(&self) -> Result<SharedResourceManager, martensite_tkernel::standard::Failure> {
        Ok(self.resources.clone())
    }
}
impl CdfApplication for CdfTestApplication {
    fn cdf_application_base(&self) -> &CdfApplicationBase {
        &self.base
    }
}

fn new_application() -> Handle<CdfTestApplication> {
    let manager = ResourceManager::empty(ResourceHost::native());
    let application = Handle::new(CdfTestApplication {
        base: CdfApplicationBase::default(),
        resources: Arc::new(SharedState::new(manager)),
    });
    application
        .application_base()
        .bind_self(&application.upcast());
    application
}

#[test]
fn CDF_ApplicationTest__FormatFromExtensionResource() {
    let application = new_application();
    application
        .resources()
        .unwrap()
        .write()
        .set_resource("MiXeD.FileFormat", b"Test format");
    let mut format = ExtendedString::default();
    assert!(
        application
            .format(&ExtendedString::new("document.MiXeD"), &mut format)
            .unwrap()
    );
    assert_eq!(format, ExtendedString::new("Test format"));
}

#[test]
fn CDF_ApplicationTest__ExtensionlessFileHasNoFormat() {
    let application = new_application();
    application
        .resources()
        .unwrap()
        .write()
        .set_resource(".FileFormat", b"Invalid fallback");
    let mut format = ExtendedString::default();
    assert!(
        !application
            .format(&ExtendedString::new("document"), &mut format)
            .unwrap()
    );
    assert!(format.is_empty());
}

#[test]
fn CDF_ApplicationTest__EmptyExtensionHasNoFormat() {
    let application = new_application();
    let mut format = ExtendedString::default();
    assert!(
        !application
            .format(&ExtendedString::new("document."), &mut format)
            .unwrap()
    );
    assert!(format.is_empty());
}

#[test]
fn application_views_preserve_native_class_chain_and_single_owner_count() {
    let concrete = new_application();
    let cdf = concrete.upcast::<dyn CdfApplication>();
    let cdm = cdf.upcast::<dyn CdmApplication>();
    let erased = cdm.upcast::<dyn Transient>();
    let weak = erased.downgrade();

    assert_eq!(concrete.object_address(), cdf.object_address());
    assert_eq!(cdf.object_address(), cdm.object_address());
    assert_eq!(cdm.object_address(), erased.object_address());
    assert_eq!(erased.dynamic_type().unwrap().name(), "CDF_TestApplication");
    assert!(erased.is_kind_name("CDF_Application"));
    assert!(erased.is_kind_name("CDM_Application"));
    assert!(erased.is_kind_name("Standard_Transient"));
    assert_eq!(concrete.ref_count(), 4);

    let recovered = erased.into_cast::<dyn CdfApplication>();
    assert_eq!(recovered.object_address(), cdf.object_address());
    assert_eq!(concrete.ref_count(), 4);
    drop((concrete, cdf, cdm, recovered));
    assert!(weak.upgrade().is_null());
}

struct WrongReaderPlugin {
    application: WeakHandle<CdfTestApplication>,
    release_status: Arc<OnceLock<PcdmReaderStatus>>,
}
impl_transient!(
    WrongReaderPlugin,
    "CDF_TestWrongReaderPlugin",
    dyn Transient
);

impl Drop for WrongReaderPlugin {
    fn drop(&mut self) {
        if let Some(application) = self.application.upgrade().get() {
            let _ = self
                .release_status
                .set(application.base.retrievable_status());
        }
    }
}

fn install_wrong_reader_plugin(
    application: &Handle<CdfTestApplication>,
) -> Arc<OnceLock<PcdmReaderStatus>> {
    application.resources().unwrap().write().set_resource(
        "Named.RetrievalPlugin",
        b"12345678-1234-1234-1234-123456789abc",
    );
    let status = Arc::new(OnceLock::new());
    let release_status = status.clone();
    let weak_application = application.downgrade();
    application.base.set_plugin_load(Arc::new(move |_| {
        Ok(Handle::new(WrongReaderPlugin {
            application: weak_application.clone(),
            release_status: release_status.clone(),
        })
        .into_upcast())
    }));
    status
}

#[test]
fn wrong_plugin_releases_after_reader_failure_status_is_published() {
    let application = new_application();
    let status = install_wrong_reader_plugin(&application);
    let reader = application
        .reader_from_format(&ExtendedString::new("Named"))
        .unwrap();
    assert!(reader.is_null());
    assert_eq!(status.get(), Some(&PcdmReaderStatus::WrongResource));
}
