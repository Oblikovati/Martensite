//! Real bounded-reader failure evidence survives the carrier and filesystem serializer.
use super::{
    fake_api::{FakeExecutableApi, Fault},
    query,
    records::ExecutableGap,
};
use ash::{vk, vk::Handle};
fn partial_evidence(gap: &ExecutableGap) -> &super::raw::RawQueryEvidence {
    match gap {
        ExecutableGap::PartialQuery { evidence, .. } => evidence,
        other => panic!("actual={other:?}; expected owned PartialQuery evidence"),
    }
}
#[test]
fn failed_properties_keep_all_allocated_slots_and_unknown_payload_validity() {
    let gap = failed_query(Fault::PropertiesError, 71);
    let raw = partial_evidence(&gap);
    assert_eq!(raw.calls.len(), 1);
    require_unknown_error(&raw.calls[0], 16, 1);
    assert_eq!(
        &raw.calls[0].properties[0].name[..21],
        b"retained-before-error"
    );
    assert_eq!(raw.calls[0].properties[0].subgroup_size, 77);
    assert_eq!(raw.calls[0].properties.len(), 16);
}
#[test]
fn failed_byte_call_keeps_actual_partial_bytes_and_reserved_budget() {
    let mut fake = FakeExecutableApi::new(Fault::ByteReadError);
    let mut budget = super::CaptureByteBudget::default();
    let gap = query::capture(&mut fake, vk::Pipeline::from_raw(72), &mut budget).unwrap_err();
    let raw = partial_evidence(&gap);
    assert_eq!(budget.used, 8);
    assert_eq!(raw.reserved_bytes, 8);
    assert_eq!(raw.allocation_sizes, [8]);
    assert_eq!(
        raw.buffers[0],
        [b'E', b'R', b'R', 0xa5, 0xa5, 0xa5, 0xa5, 0xa5]
    );
    require_unknown_error(&raw.calls[2], 1, 0);
    assert!(raw.calls[2].representations[0].data_present);
}
#[test]
fn third_large_query_is_refused_before_allocation_or_byte_dispatch() {
    let mut budget = super::CaptureByteBudget::default();
    for _ in 0..2 {
        let mut fake = FakeExecutableApi::new(Fault::LargeSuccess);
        let witness = query::capture(&mut fake, vk::Pipeline::from_raw(73), &mut budget).unwrap();
        assert_eq!(fake.ir_calls, 2);
        assert_eq!(witness.representations[0].bytes.len(), 8 * 1024 * 1024);
    }
    let mut fake = FakeExecutableApi::new(Fault::LargeSuccess);
    let gap = query::capture(&mut fake, vk::Pipeline::from_raw(73), &mut budget).unwrap_err();
    assert_eq!(
        fake.ir_calls, 1,
        "only descriptor call; no byte dispatch after budget refusal"
    );
    assert_eq!(budget.used, 16 * 1024 * 1024);
    let raw = partial_evidence(&gap);
    assert_eq!(raw.requested_bytes, Some(8 * 1024 * 1024));
    assert_eq!(raw.reserved_bytes, 0);
    assert!(raw.buffers.is_empty() && raw.allocation_sizes.is_empty());
}
#[test]
fn malformed_metadata_and_text_keep_allocations_and_actual_returned_fields() {
    for fault in [
        Fault::ChangedCount,
        Fault::ChangedMetadata,
        Fault::ChangedSize,
        Fault::InvalidText,
    ] {
        let mut fake = FakeExecutableApi::new(fault);
        let mut budget = super::CaptureByteBudget::default();
        let gap = query::capture(&mut fake, vk::Pipeline::from_raw(74), &mut budget).unwrap_err();
        let raw = partial_evidence(&gap);
        assert_eq!(fake.ir_calls, 2);
        assert_eq!(raw.calls.len(), 3);
        require_retained_allocation(raw, budget.used);
    }
}

struct CapturedFrameFixture;
impl CapturedFrameFixture {
    fn owned() -> super::CapturedExecutableFrame {
        super::CapturedExecutableFrame {
            capture: Self::capture(),
            witness: None,
            pipeline: 77,
            device: 78,
            physical: 79,
            modules: vec![(
                vk::ShaderStageFlags::VERTEX.as_raw(),
                80,
                b"retained-module",
            )],
            attachment: Self::attachment(),
        }
    }
    fn capture() -> crate::FrameCapture {
        crate::FrameCapture {
            width: 1,
            height: 1,
            rgba: vec![1, 2, 3, 255],
            depth: vec![0.5],
        }
    }
    fn attachment() -> super::CapturedAttachment {
        super::CapturedAttachment {
            color: 43,
            depth: 126,
            samples: 1,
            raster: Default::default(),
            coordinates: Default::default(),
        }
    }
}
#[test]
fn actual_query_failure_keeps_capture_module_inputs_and_partial_bytes_together() {
    let gap = owned_carrier_after_fake_scope();
    let ExecutableGap::CapturedFrame { frame, cause } = gap else {
        panic!("actual={gap:?}; expected captured frame carrier");
    };
    assert_eq!(frame.capture.rgba, [1, 2, 3, 255]);
    assert_eq!(frame.capture.depth[0].to_bits(), 0.5f32.to_bits());
    assert_eq!(frame.pipeline, 77);
    assert_eq!(frame.modules[0].2, b"retained-module");
    assert_eq!(&partial_evidence(&cause).buffers[0][..3], b"ERR");
}
#[test]
fn missing_module_association_keeps_all_unpaired_handles_and_creation_inputs() {
    let handles = [
        vk::ShaderModule::from_raw(83),
        vk::ShaderModule::from_raw(84),
    ];
    let gap = super::module_count_gap(&handles, &[b"unpaired-input"]);
    let ExecutableGap::ModuleInputs { evidence, .. } = gap else {
        panic!("actual={gap:?}; expected unpaired module evidence");
    };
    assert_eq!(evidence.handles, [83, 84]);
    assert_eq!(evidence.inputs, [b"unpaired-input".as_slice()]);
}

struct OwnedQueryEvidenceDirectory(std::path::PathBuf);
impl OwnedQueryEvidenceDirectory {
    fn new() -> Self {
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "selected-query-evidence-{}-{stamp}",
            std::process::id()
        ));
        std::fs::create_dir(&path).unwrap_or_else(|error| {
            panic!("path={path:?} cause={error}; expected new owned directory")
        });
        Self(path)
    }
    fn read(&self, relative: &str) -> Vec<u8> {
        let path = self.0.join(relative);
        std::fs::read(&path)
            .unwrap_or_else(|error| panic!("path={path:?} cause={error}; expected retained file"))
    }
}
impl Drop for OwnedQueryEvidenceDirectory {
    fn drop(&mut self) {
        if let Err(error) = std::fs::remove_dir_all(&self.0) {
            // Cleanup must fail an otherwise passing test without replacing an earlier panic.
            assert!(
                std::thread::panicking(),
                "path={:?} cause={error}; expected owned-directory retirement",
                self.0
            );
        }
    }
}
#[test]
fn actual_gap_serializer_preserves_capture_module_raw_fields_and_partial_bytes() {
    let mut fake = FakeExecutableApi::new(Fault::ByteReadError);
    let mut budget = super::CaptureByteBudget::default();
    let outcome = query::capture(&mut fake, vk::Pipeline::from_raw(77), &mut budget).map(Some);
    let gap = super::settle_query(CapturedFrameFixture::owned(), outcome).unwrap_err();
    let directory = OwnedQueryEvidenceDirectory::new();
    super::retain_gap(&gap, &directory.0).unwrap();
    require_serialized_capture(&directory);
    require_serialized_partial(&directory);
    assert_eq!(
        budget.used, 8,
        "retention must not charge reserved bytes again"
    );
}
fn require_serialized_capture(directory: &OwnedQueryEvidenceDirectory) {
    assert_eq!(directory.read("captured-frame/frame.rgba"), [1, 2, 3, 255]);
    assert_eq!(
        directory.read("captured-frame/frame.depth"),
        0.5f32.to_bits().to_le_bytes()
    );
    assert_eq!(
        directory.read("captured-frame/module-0-1.spv"),
        b"retained-module"
    );
    let context = directory.read("gap.json");
    assert!(context.starts_with(b"{\"event\":\"selected-executable-gap\""));
}
fn require_serialized_partial(directory: &OwnedQueryEvidenceDirectory) {
    let bytes = directory.read("cause/partial-query/allocated-buffer-0.bin");
    assert_eq!(bytes, [b'E', b'R', b'R', 0xa5, 0xa5, 0xa5, 0xa5, 0xa5]);
    let expected = format!(
        "{{\"operation\":\"IR bytes\",\"status\":{},\"allocated_slots\":1,\"reported_count\":0,\"validity\":\"UNKNOWN_OR_INCOMPLETE_RETURNED_FIELDS\"}}\n",
        vk::Result::ERROR_DEVICE_LOST.as_raw()
    );
    assert_eq!(
        directory.read("cause/partial-query/call-2/call.json"),
        expected.as_bytes()
    );
    require_serialized_descriptor(directory);
    require_serialized_cause(directory);
}
fn require_serialized_descriptor(directory: &OwnedQueryEvidenceDirectory) {
    let expected = format!(
        "{{\"slot\":0,\"structure_type\":{},\"next_present\":false,\"is_text\":1,\"reported_bytes\":8,\"data_present\":true}}\n",
        vk::StructureType::PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR.as_raw()
    );
    assert_eq!(
        directory.read("cause/partial-query/call-2/representation-0.json"),
        expected.as_bytes()
    );
    let mut name = vec![0; 256];
    name[..8].copy_from_slice(b"Assembly");
    assert_eq!(
        directory.read("cause/partial-query/call-2/representation-0-name.bin"),
        name
    );
}

fn failed_query(fault: Fault, pipeline: u64) -> ExecutableGap {
    let mut fake = FakeExecutableApi::new(fault);
    query::capture(
        &mut fake,
        vk::Pipeline::from_raw(pipeline),
        &mut Default::default(),
    )
    .unwrap_err()
}
fn require_unknown_error(call: &super::raw::RawCall, allocated: usize, reported: u32) {
    assert_eq!(
        (call.allocated_slots, call.reported_count),
        (allocated, reported)
    );
    assert_eq!(call.status, vk::Result::ERROR_DEVICE_LOST.as_raw());
    assert_eq!(call.validity, "UNKNOWN_OR_INCOMPLETE_RETURNED_FIELDS");
}
fn require_retained_allocation(raw: &super::raw::RawQueryEvidence, used: usize) {
    assert_eq!(raw.buffers.len(), raw.allocation_sizes.len());
    assert_eq!(
        raw.buffers.iter().map(Vec::len).sum::<usize>(),
        raw.reserved_bytes
    );
    assert_eq!(used, raw.reserved_bytes);
    assert_eq!(
        raw.calls[2].representations.len(),
        raw.allocation_sizes.len()
    );
}

fn require_serialized_cause(directory: &OwnedQueryEvidenceDirectory) {
    let expected = b"{\"event\":\"selected-executable-gap\",\"context_hex\":\"417069207b206f7065726174696f6e3a20224952206279746573222c20706970656c696e653a2037372c20726573756c743a204552524f525f4445564943455f4c4f5354207d\"}\n";
    assert_eq!(directory.read("cause/cause/gap.json"), expected);
}

fn owned_carrier_after_fake_scope() -> ExecutableGap {
    let mut fake = FakeExecutableApi::new(Fault::ByteReadError);
    let outcome = query::capture(
        &mut fake,
        vk::Pipeline::from_raw(77),
        &mut Default::default(),
    )
    .map(Some);
    super::settle_query(CapturedFrameFixture::owned(), outcome).unwrap_err()
}
