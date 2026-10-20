use super::fake_input::FakeProbeInput;
use martensite_tkcdf::{
    SharedResourceManager,
    cdf::{CdfApplication, CdfApplicationBase, CdfApplicationServices},
    cdf_application_cdm_members,
    cdm::CdmApplication,
    pcdm::{PcdmInputStream, PcdmReaderStatus},
};
use martensite_tkernel::{
    impl_transient,
    message::ProgressRange,
    resource::{ResourceHost, ResourceManager},
    standard::{Failure, Handle, SharedState, StandardTransient},
    storage::state,
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
#[path = "application_bytes.rs"]
mod byte_failures;

#[derive(Default)]
struct FakeProbeApplication {
    transient: StandardTransient,
    base: CdfApplicationBase,
    callbacks: AtomicUsize,
}
impl_transient!(FakeProbeApplication, "FakeProbeApplication", dyn CdfApplication, transient, interfaces: [dyn CdmApplication, dyn CdfApplication]);
impl CdmApplication for FakeProbeApplication {
    cdf_application_cdm_members!();
    fn resources(&self) -> Result<SharedResourceManager, Failure> {
        self.callbacks.fetch_add(1, Ordering::SeqCst);
        Ok(Arc::new(SharedState::new(ResourceManager::empty(
            ResourceHost::native(),
        ))))
    }
}
impl CdfApplication for FakeProbeApplication {
    fn new_document(
        &self,
        _format: &martensite_tkernel::tcollection::ExtendedString,
        _document: &mut Handle<dyn martensite_tkcdf::cdm::CdmDocument>,
    ) -> Result<(), Failure> {
        self.callbacks.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
    fn cdf_application_base(&self) -> &CdfApplicationBase {
        &self.base
    }
}

#[test]
fn borrowed_application_read_consumes_unknown_prefix_and_keeps_document() {
    let app = FakeProbeApplication::default();
    let mut device = FakeProbeInput::unknown(12);
    let (mut document, original) = super::support::new_document();
    app.read_stream(
        &mut device,
        &mut document,
        &Handle::default(),
        &ProgressRange::default(),
    )
    .unwrap();
    assert_eq!(device.consumed, 7);
    assert_eq!(app.get_retrieve_status(), PcdmReaderStatus::FormatFailure);
    assert!(document.cast::<super::support::FakeDocument>() == original);
}

#[test]
fn explicit_application_stream_default_mask_handles_device_errors_as_format_failure() {
    for kernel in [false, true] {
        let app = FakeProbeApplication::default();
        let mut device = FakeProbeInput::unknown(12);
        device.fail_at = Some(3);
        device.kernel_failure = kernel;
        let mut stream = PcdmInputStream::new(&mut device);
        app.read_input_stream(
            &mut stream,
            &mut Handle::default(),
            &Handle::default(),
            &ProgressRange::default(),
        )
        .unwrap();
        assert_eq!(app.get_retrieve_status(), PcdmReaderStatus::FormatFailure);
        assert_eq!(stream.state(), state::BAD | state::FAIL);
        assert!(stream.last_error().is_some());
    }
}

#[test]
fn explicit_application_stream_catches_only_kernel_failures() {
    for kernel in [false, true] {
        let app = FakeProbeApplication::default();
        app.base
            .set_retrievable_status(PcdmReaderStatus::PermissionDenied);
        let mut device = FakeProbeInput::unknown(12);
        device.fail_at = Some(3);
        device.kernel_failure = kernel;
        let mut stream = PcdmInputStream::with_exception_mask(&mut device, state::BAD);
        let (mut document, original) = super::support::new_document();
        let failure = app
            .read_input_stream(
                &mut stream,
                &mut document,
                &Handle::default(),
                &ProgressRange::default(),
            )
            .unwrap_err();
        assert!(document.cast::<super::support::FakeDocument>() == original);
        assert_eq!(failure.operation(), "CDF_Application::Read(stream)");
        assert_eq!(
            app.get_retrieve_status(),
            if kernel {
                PcdmReaderStatus::FormatFailure
            } else {
                PcdmReaderStatus::PermissionDenied
            }
        );
        if let Some(wrapped) = failure.kernel_failure() {
            assert_eq!(wrapped.exception_type(), "Standard_Failure");
            assert_eq!(wrapped.message(), "Standard_DomainError: probe sentinel\n");
        }
    }
}
