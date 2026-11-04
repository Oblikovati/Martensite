//! Failed real image uploads retain the allocated destination and staging until actual close.
use super::{
    VulkanRenderer,
    resource_lifecycle_test_support::{FaultingCloseCalls, inject_completion_failure},
    retained_submission::RetainedOwnerSnapshot,
    sampled_surface_image::SampledSurfaceImage,
    sampled_surface_upload::{prepare_owned_upload, settle_owned_upload},
};
use crate::{
    RenderDeviceCloseOutcome,
    surface_texture::{SurfaceImageEncoding, SurfaceTextureImage},
};
use ash::vk::{self, Handle};
use std::sync::{Arc, Weak};
fn upload_fixture() -> SurfaceTextureImage {
    SurfaceTextureImage {
        lifetime: Arc::default(),
        id: Vec::new(),
        revision: 0,
        extent: [2, 2],
        depth: None,
        encoding: SurfaceImageEncoding::Linear8,
        alpha_only: false,
        has_alpha: false,
        bytes: Arc::new([255, 0, 0, 255].repeat(4)),
        top_down: true,
        mipmaps: false,
        supplied_levels: Vec::new(),
    }
}
fn inject_owned_upload(
    renderer: &VulkanRenderer,
) -> (Weak<SampledSurfaceImage>, RetainedOwnerSnapshot) {
    let source = upload_fixture();
    let image =
        SampledSurfaceImage::allocated(&renderer.runtime, &source, vk::Format::R8G8B8A8_UNORM)
            .unwrap();
    let weak = Arc::downgrade(&image);
    let (commands, staging) = prepare_owned_upload(&image, &source).unwrap();
    let expected = upload_owner_identity(&commands, &image, &staging);
    let outcome = inject_completion_failure(&commands, &[], &[]);
    assert!(
        settle_owned_upload(&image, staging, commands, &renderer.retirements, outcome).is_err()
    );
    drop(image);
    (weak, expected)
}
struct PendingUploadWitness {
    renderer: VulkanRenderer,
    monitor: Arc<super::validation::ValidationMessages>,
    image: Weak<SampledSurfaceImage>,
    expected: RetainedOwnerSnapshot,
}
impl PendingUploadWitness {
    fn new() -> Self {
        let renderer = super::resource_lifecycle_test_support::create_resource_renderer();
        let monitor = renderer.runtime.validation.clone();
        let (image, expected) = inject_owned_upload(&renderer);
        Self {
            renderer,
            monitor,
            image,
            expected,
        }
    }
    fn assert_incomplete(&mut self) {
        assert!(self.image.upgrade().is_some());
        assert_eq!(
            self.renderer.retirements.snapshot().as_slice(),
            std::slice::from_ref(&self.expected)
        );
        let mut calls = FaultingCloseCalls { calls: 0 };
        assert!(matches!(
            self.renderer.close_dispatched(&mut calls),
            RenderDeviceCloseOutcome::Incomplete { .. }
        ));
        assert_eq!(calls.calls, 1);
        assert_eq!(
            self.renderer.retirements.snapshot().as_slice(),
            std::slice::from_ref(&self.expected)
        );
        assert!(self.image.upgrade().is_some());
    }
    fn close_actual(mut self) {
        assert_eq!(
            self.renderer.close(),
            RenderDeviceCloseOutcome::Closed { failure: None }
        );
        assert!(self.renderer.retirements.snapshot().is_empty());
        assert!(self.image.upgrade().is_none());
        drop(self.renderer);
        assert!(self.monitor.is_observable());
        assert!(
            self.monitor.errors().unwrap().is_empty(),
            "{:?}",
            self.monitor.errors().unwrap()
        );
        assert_eq!(Arc::strong_count(&self.monitor), 1);
    }
}
#[test]
fn real_failed_upload_retains_destination_and_staging_through_incomplete_close() {
    let mut witness = PendingUploadWitness::new();
    witness.assert_incomplete();
    witness.close_actual();
}

fn upload_owner_identity(
    commands: &super::submission::CommandSubmission,
    image: &SampledSurfaceImage,
    staging: &super::buffer::DeviceBuffer,
) -> RetainedOwnerSnapshot {
    RetainedOwnerSnapshot::Upload {
        command: commands.command.as_raw(),
        image: image.handle.as_raw(),
        staging: staging.handle.as_raw(),
    }
}
