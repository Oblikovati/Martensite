//! Test-local fake capability/device I/O proves admission ordering without Vulkan.
use super::*;
use std::cell::Cell;
struct FakeRasterDevice {
    capabilities: RasterPolicyCapabilities,
    query_failure: Option<RenderError>,
    creation_failure: Option<RenderError>,
    queries: Cell<usize>,
    identities: Cell<usize>,
    creations: Cell<usize>,
}
impl RasterPolicyCapabilitySource for FakeRasterDevice {
    fn capabilities(&self) -> Result<RasterPolicyCapabilities, RenderError> {
        self.queries.set(self.queries.get() + 1);
        match &self.query_failure {
            Some(failure) => Err(failure.clone()),
            None => Ok(self.capabilities),
        }
    }
    fn identity(&self) -> RenderDriverIdentity {
        self.identities.set(self.identities.get() + 1);
        fake_driver_identity()
    }
}
impl FakeRasterDevice {
    fn create(&self, admitted: RenderRasterPolicy) -> Result<RenderRasterPolicy, RenderError> {
        self.creations.set(self.creations.get() + 1);
        match &self.creation_failure {
            Some(failure) => Err(failure.clone()),
            None => Ok(admitted),
        }
    }
}
fn fake_device(extension_present: bool, last_vertex_feature: bool) -> FakeRasterDevice {
    FakeRasterDevice {
        capabilities: RasterPolicyCapabilities {
            extension_present,
            last_vertex_feature,
        },
        query_failure: None,
        creation_failure: None,
        queries: Cell::new(0),
        identities: Cell::new(0),
        creations: Cell::new(0),
    }
}
fn fake_driver_identity() -> RenderDriverIdentity {
    RenderDriverIdentity {
        vendor_id: 17,
        device_id: 23,
        driver_id: 31,
        driver_version: 41,
        driver_name: "fake raster driver".to_owned(),
        driver_info: "owned fixture".to_owned(),
        device_name: "fake selected device".to_owned(),
    }
}
fn admit_fake(
    policy: RenderRasterPolicy,
    device: &FakeRasterDevice,
) -> Result<RenderRasterPolicy, RenderError> {
    let create = |admitted| device.create(admitted);
    create_after_raster_admission(policy, device, create)
}

#[test]
fn legacy_policy_skips_new_capability_queries_and_preserves_replay_version() {
    let device = fake_device(false, false);
    let _: &(dyn Send + Sync) = &device.capabilities;
    let policy = admit_fake(RenderRasterPolicy::default(), &device).unwrap();
    let _: &(dyn Send + Sync) = &policy;
    assert_eq!(
        (policy, policy.behavior_version()),
        (RenderRasterPolicy::LegacyFirstV1, 1)
    );
    assert_eq!(
        (
            device.queries.get(),
            device.identities.get(),
            device.creations.get()
        ),
        (0, 0, 1)
    );
}

#[test]
fn missing_extension_refuses_before_device_creation_with_owned_identity() {
    let device = fake_device(false, true);
    let error = admit_fake(RenderRasterPolicy::LastProvokingVertexV2, &device).unwrap_err();
    assert_capability_refusal(
        &device,
        &error,
        RenderRasterCapabilityFailure::MissingExtension,
    );
    assert!(std::error::Error::source(&error).is_some());
}

#[test]
fn missing_last_feature_is_distinct_and_never_downgrades_the_request() {
    let device = fake_device(true, false);
    let error = admit_fake(RenderRasterPolicy::LastProvokingVertexV2, &device).unwrap_err();
    assert_capability_refusal(
        &device,
        &error,
        RenderRasterCapabilityFailure::MissingLastVertexFeature,
    );
    assert!(error.to_string().contains("provokingVertexLast=true"));
}

fn assert_capability_refusal(
    device: &FakeRasterDevice,
    error: &RenderError,
    reason: RenderRasterCapabilityFailure,
) {
    let failure = error
        .raster_policy_failure()
        .unwrap_or_else(|| panic!("error {error:?}, expected typed raster-policy refusal"));
    assert_owned_refusal(failure, reason);
    assert_eq!(device.creations.get(), 0);
    assert_eq!((device.queries.get(), device.identities.get()), (1, 1));
}
fn assert_owned_refusal(
    failure: &RenderRasterPolicyFailure,
    reason: RenderRasterCapabilityFailure,
) {
    let _: &(dyn Send + Sync) = failure;
    let _: &(dyn Send + Sync) = &failure.reason;
    assert_eq!(
        failure.requested_policy,
        RenderRasterPolicy::LastProvokingVertexV2
    );
    assert_eq!(failure.device, fake_driver_identity());
    assert_eq!(failure.reason, reason);
}

#[test]
fn supported_last_policy_creates_once_with_its_distinct_behavior_version() {
    let device = fake_device(true, true);
    let policy = admit_fake(RenderRasterPolicy::LastProvokingVertexV2, &device).unwrap();
    assert_eq!(
        (policy, policy.behavior_version()),
        (RenderRasterPolicy::LastProvokingVertexV2, 2)
    );
    assert_eq!(
        (
            device.queries.get(),
            device.identities.get(),
            device.creations.get()
        ),
        (1, 0, 1)
    );
}

#[test]
fn capability_query_failure_preserves_its_cause_and_never_creates_a_device() {
    let mut device = fake_device(true, true);
    let original = RenderError::rejected("fake query failure", "a successful capability query");
    device.query_failure = Some(original.clone());
    let error = admit_fake(RenderRasterPolicy::LastProvokingVertexV2, &device).unwrap_err();
    assert_eq!(error, original);
    assert!(error.raster_policy_failure().is_none());
    assert_eq!(
        (
            device.queries.get(),
            device.identities.get(),
            device.creations.get()
        ),
        (1, 0, 0)
    );
}

#[test]
fn actual_device_failure_after_admission_is_not_a_capability_refusal() {
    let mut device = fake_device(true, true);
    let original = RenderError::rejected("fake device failure", "successful device creation");
    device.creation_failure = Some(original.clone());
    let error = admit_fake(RenderRasterPolicy::LastProvokingVertexV2, &device).unwrap_err();
    assert_eq!(error, original);
    assert!(error.raster_policy_failure().is_none());
    assert_eq!(device.creations.get(), 1);
}

#[test]
fn owned_pipeline_configuration_composes_without_changing_legacy_defaults() {
    let legacy = crate::RenderPipelinePolicies::default();
    let _: &(dyn Send + Sync) = &legacy;
    let selected = crate::RenderPipelinePolicies {
        raster: RenderRasterPolicy::LastProvokingVertexV2,
        lighting: crate::RenderLightingPipeline::FixedFunction,
        ..legacy
    };
    assert_eq!(legacy.raster, RenderRasterPolicy::LegacyFirstV1);
    assert_eq!(selected.raster, RenderRasterPolicy::LastProvokingVertexV2);
    assert_eq!(
        selected.lighting,
        crate::RenderLightingPipeline::FixedFunction
    );
    assert_eq!(selected.gui_surface, legacy.gui_surface);
}
