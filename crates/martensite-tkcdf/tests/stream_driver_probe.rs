//! Frozen detector inputs and independent state/output-driver assertions.
#[path = "stream_driver/support.rs"]
pub(super) mod support;
use martensite_tkcdf::pcdm::{Pcdm, PcdmTypeOfFileDriver as Kind};
use martensite_tkernel::{
    storage::{HeaderData, OpenMode, state},
    tcollection::AsciiString,
};
use support::{DriverProbeDevice, marked_driver, retained};
const XML: &[u8] = include_bytes!("stream_driver/fixtures/xml.input");

#[test]
fn xml_detection_skips_only_through_the_next_declaration_end() {
    let cases: [(&[u8], Kind, usize); 7] = [
        (XML, Kind::XmlFile, 21),
        (
            include_bytes!("stream_driver/fixtures/short-declaration.input"),
            Kind::XmlFile,
            36,
        ),
        (
            include_bytes!("stream_driver/fixtures/offset-one.input"),
            Kind::XmlFile,
            12,
        ),
        (
            include_bytes!("stream_driver/fixtures/offset-two.input"),
            Kind::XmlFile,
            12,
        ),
        (
            include_bytes!("stream_driver/fixtures/offset-three.input"),
            Kind::Unknown,
            7,
        ),
        (
            include_bytes!("stream_driver/fixtures/uppercase.input"),
            Kind::Unknown,
            7,
        ),
        (
            include_bytes!("stream_driver/fixtures/unknown.input"),
            Kind::Unknown,
            7,
        ),
    ];
    for (bytes, expected, consumed) in cases {
        let mut device = DriverProbeDevice::new(bytes);
        let mut driver = marked_driver();
        assert_eq!(
            Pcdm::file_driver_type_stream(&mut device, &mut driver),
            expected
        );
        assert_eq!((device.consumed, device.calls), (consumed, consumed));
        assert_eq!(retained(&driver), expected == Kind::XmlFile);
    }
}

#[test]
fn recognized_streams_install_the_actual_compatible_text_and_binary_drivers() {
    for (magic, kind, fixture) in [
        (b"CMPFILE", Kind::CmpFile, "cmp-0"),
        (b"FSDFILE", Kind::File, "text-0"),
        (b"BINFILE", Kind::File, "binary-0"),
    ] {
        let mut bytes = magic.as_slice();
        let mut driver = marked_driver();
        assert_eq!(Pcdm::file_driver_type_stream(&mut bytes, &mut driver), kind);
        assert!(!retained(&driver));
        let driver = driver.as_mut().unwrap();
        let path = format!(
            "{}/tests/header_counters/fixtures/{fixture}.fsd",
            env!("CARGO_MANIFEST_DIR")
        );
        assert_eq!(
            driver
                .open(&AsciiString::new(&path), OpenMode::Read)
                .unwrap() as i32,
            0
        );
        let header = HeaderData::default();
        header.read(&mut **driver).unwrap();
        assert!(
            header
                .user_info()
                .iter()
                .any(|line| line.as_bytes() == b"REFERENCE_COUNTER: 17")
        );
        driver.close().unwrap();
    }
}

#[test]
fn generated_xml_prefixes_preserve_eof_masks_and_output_assignment_boundary() {
    for length in 0..=XML.len() {
        for mask in [
            0,
            state::BAD,
            state::EOF,
            state::FAIL,
            state::BAD | state::FAIL,
        ] {
            let mut device = DriverProbeDevice::new(&XML[..length]);
            let mut driver = marked_driver();
            let mut input = device.input(mask);
            let answer = Pcdm::file_driver_type_input(&mut input, &mut driver);
            let short = length < 21;
            assert_eq!(
                input.state(),
                if short { state::EOF | state::FAIL } else { 0 }
            );
            assert_eq!(
                answer.is_err(),
                short && mask & (state::EOF | state::FAIL) != 0
            );
            if let Ok(kind) = answer {
                assert_eq!(
                    kind,
                    if length >= 5 {
                        Kind::XmlFile
                    } else {
                        Kind::Unknown
                    }
                );
            }
            assert_eq!(retained(&driver), length >= 5 || (short && mask & 6 != 0));
            drop(input);
            assert_eq!(device.consumed, length.min(21));
        }
    }
}

#[test]
fn xml_device_failure_masks_preserve_original_cause_and_driver() {
    for kernel in [false, true] {
        for failure_at in 0..=22 {
            for mask in [
                0,
                state::BAD,
                state::FAIL,
                state::EOF,
                state::BAD | state::FAIL,
            ] {
                assert_device_failure(kernel, failure_at, mask);
            }
        }
    }
}

fn assert_device_failure(kernel: bool, failure_at: usize, mask: u8) {
    let mut device = DriverProbeDevice::new(XML);
    device.failure_at = Some(failure_at);
    device.kernel = kernel;
    let mut driver = marked_driver();
    let mut input = device.input(mask);
    let result = Pcdm::file_driver_type_input(&mut input, &mut driver);
    let failed = failure_at < 21;
    let throws = failed && mask & (state::BAD | state::FAIL) != 0;
    assert_eq!(result.is_err(), throws);
    assert_eq!(
        input.state(),
        if !failed {
            0
        } else if mask & state::BAD != 0 {
            state::BAD
        } else {
            state::BAD | state::FAIL
        }
    );
    assert_eq!(retained(&driver), !failed || failure_at >= 5 || throws);
    assert_eq!(input.last_error().is_some(), failed);
    if let Err(error) = result {
        assert_eq!(error.consumed(), failure_at);
        assert_eq!(error.operation(), "PCDM::FileDriverType(stream)");
        if failure_at > 7 {
            assert!(
                error
                    .to_string()
                    .contains("input bytes after the 7-byte probe")
            );
        }
        assert_eq!(
            error.kernel_failure().is_some(),
            kernel && mask & state::BAD != 0
        );
    }
    drop(input);
    assert_eq!(device.consumed, failure_at.min(21));
}

#[test]
fn xml_preserves_null_output_and_failed_reprobe_clears_old_output() {
    let mut device = DriverProbeDevice::new(XML);
    let mut driver = None;
    assert_eq!(
        Pcdm::file_driver_type_stream(&mut device, &mut driver),
        Kind::XmlFile
    );
    assert!(driver.is_none());
    let mut device = DriverProbeDevice::new(b"<?xml");
    let mut driver = marked_driver();
    let mut input = device.input(0);
    assert_eq!(
        Pcdm::file_driver_type_input(&mut input, &mut driver).unwrap(),
        Kind::XmlFile
    );
    assert!(retained(&driver));
    assert_eq!(
        Pcdm::file_driver_type_input(&mut input, &mut driver).unwrap(),
        Kind::Unknown
    );
    assert!(driver.is_none());
}

#[test]
fn format_query_keeps_its_operation_context_after_shared_detector_failure() {
    use martensite_tkcdf::pcdm::PcdmReadWriterBase;
    use martensite_tkernel::standard::Handle;
    let mut device = DriverProbeDevice::new(XML);
    device.failure_at = Some(14);
    let mut input = device.input(state::BAD);
    let error =
        PcdmReadWriterBase::file_format_input(&mut input, &mut Handle::default()).unwrap_err();
    assert_eq!(error.operation(), "PCDM_ReadWriter::FileFormat(stream)");
    assert_eq!(error.consumed(), 14);
}
