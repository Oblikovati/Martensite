//! `PCDM_ReadWriter::FileFormat(stream, data)` on borrowed input: binary container
//! information and XML document tags, with the reader left after what was consumed.
#[allow(dead_code)] // The shared wire builder also serves the owned-stream cases.
#[path = "owned_stream_format/wire.rs"]
mod wire;
use martensite_tkcdf::pcdm::{PcdmInputStream, PcdmReadWriterBase};
use martensite_tkernel::{
    standard::Handle,
    storage::{Data, state},
    tcollection::{AsciiString, ExtendedString},
};
use std::io::{self, Read};

/// A borrowed device that counts what was taken and can fail at one offset.
struct CountingDevice {
    bytes: Vec<u8>,
    taken: usize,
    fail_at: Option<usize>,
}

impl CountingDevice {
    fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            taken: 0,
            fail_at: None,
        }
    }

    fn rest(&self) -> &[u8] {
        &self.bytes[self.taken..]
    }
}

impl Read for CountingDevice {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        if self.fail_at == Some(self.taken) {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "owned device fault",
            ));
        }
        let count = output.len().min(self.rest().len());
        output[..count].copy_from_slice(&self.bytes[self.taken..self.taken + count]);
        self.taken += count;
        Ok(count)
    }
}

fn probe(device: &mut CountingDevice, mask: u8, storage: &mut Handle<Data>) -> ProbeOutcome {
    let mut input = PcdmInputStream::with_exception_mask(device, mask);
    let result = PcdmReadWriterBase::file_format_input(&mut input, storage);
    ProbeOutcome {
        format: result.as_ref().ok().map(ExtendedString::to_utf8),
        failure: result.err(),
        state: input.state(),
        device_error: input.last_error().is_some(),
    }
}

struct ProbeOutcome {
    format: Option<String>,
    failure: Option<martensite_tkcdf::pcdm::PcdmStreamFailure>,
    state: u8,
    device_error: bool,
}

#[test]
fn binary_information_is_read_and_the_reader_stops_at_the_data_end() {
    let container = wire::ordinary_binary(0);
    let mut bytes = container.clone();
    bytes.extend_from_slice(b"document body");
    let mut device = CountingDevice::new(bytes);
    let mut storage = Handle::default();
    let outcome = probe(&mut device, 0, &mut storage);
    assert_eq!(outcome.format.as_deref(), Some("last"));
    assert_eq!(outcome.state, 0);
    assert_eq!(device.taken, container.len());
    assert_eq!(device.rest(), b"document body");
    assert_eq!(storage.get().unwrap().user_info().length(), 2);
}

#[test]
fn binary_information_is_appended_to_existing_storage() {
    let mut device = CountingDevice::new(wire::ordinary_binary(0));
    let storage = Handle::new(Data::default());
    storage.add_to_user_info(&AsciiString::new("FILE_FORMAT: previous"));
    let mut target = storage.clone();
    let outcome = probe(&mut device, 0, &mut target);
    assert_eq!(outcome.format.as_deref(), Some("last"));
    assert!(target == storage);
    assert_eq!(storage.user_info().length(), 3);
}

#[test]
fn binary_without_format_line_answers_empty_after_reading_its_information() {
    let container = wire::binary(&[b"plain"], 0);
    let mut device = CountingDevice::new(container.clone());
    let outcome = probe(&mut device, 0, &mut Handle::default());
    assert_eq!(outcome.format.as_deref(), Some(""));
    assert_eq!(device.taken, container.len());
}

#[test]
fn truncated_binary_information_raises_the_driver_failure_at_end_of_input() {
    let mut device = CountingDevice::new(wire::ordinary_binary(0)[..65].to_vec());
    let outcome = probe(&mut device, 0, &mut Handle::default());
    let failure = outcome.failure.expect("truncated container must fail");
    let kernel = failure.kernel_failure().expect("driver exception");
    assert_eq!(kernel.exception_type(), "Storage_StreamTypeMismatchError");
    assert!(kernel.what().is_empty());
    assert_eq!(failure.consumed(), 65);
    assert_eq!(outcome.state, state::EOF | state::FAIL);
}

#[test]
fn end_of_input_mask_throws_before_the_driver_failure() {
    let mut device = CountingDevice::new(wire::ordinary_binary(0)[..65].to_vec());
    let outcome = probe(&mut device, state::EOF, &mut Handle::default());
    let failure = outcome.failure.expect("masked end of input must throw");
    assert!(failure.kernel_failure().is_none());
    assert_eq!(failure.cause().kind(), io::ErrorKind::UnexpectedEof);
}

#[test]
fn device_fault_inside_the_information_sets_bad_and_keeps_its_cause() {
    let mut device = CountingDevice::new(wire::ordinary_binary(0));
    device.fail_at = Some(40);
    let outcome = probe(&mut device, 0, &mut Handle::default());
    assert_eq!(outcome.state, state::BAD | state::FAIL);
    assert!(outcome.device_error);
    let kernel = outcome.failure.unwrap();
    assert_eq!(
        kernel.kernel_failure().unwrap().exception_type(),
        "Storage_StreamTypeMismatchError"
    );
}

#[test]
fn device_fault_past_the_information_is_never_reached() {
    let container = wire::ordinary_binary(0);
    let mut bytes = container.clone();
    bytes.extend_from_slice(b"tail");
    let mut device = CountingDevice::new(bytes);
    device.fail_at = Some(container.len());
    let outcome = probe(&mut device, state::BAD, &mut Handle::default());
    assert_eq!(outcome.format.as_deref(), Some("last"));
    assert_eq!((outcome.state, outcome.device_error), (0, false));
}

const XML: &[u8] =
    b"<?xml version=\"1.0\"?>\n<document format=\"XmlOcaf\"><label tag=\"0\"/></document>";

#[test]
fn xml_document_tag_gives_the_format_and_leaves_its_body_unread() {
    let mut device = CountingDevice::new(XML.to_vec());
    let storage = Handle::new(Data::default());
    let mut target = storage.clone();
    let outcome = probe(&mut device, 0, &mut target);
    assert_eq!(outcome.format.as_deref(), Some("XmlOcaf"));
    assert_eq!(device.rest(), b"<label tag=\"0\"/></document>");
    assert!(target == storage);
    assert_eq!(storage.user_info().length(), 0);
}

#[test]
fn xml_without_document_root_has_no_format() {
    let mut device = CountingDevice::new(b"<?xml version=\"1.0\"?><other format=\"X\"/>".to_vec());
    let outcome = probe(&mut device, 0, &mut Handle::default());
    assert_eq!(outcome.format.as_deref(), Some(""));
}

#[test]
fn xml_declaration_reaching_end_of_input_is_not_parsed() {
    let mut device = CountingDevice::new(b"<?xml version".to_vec());
    let outcome = probe(&mut device, 0, &mut Handle::default());
    assert_eq!(outcome.format.as_deref(), Some(""));
    assert_eq!(outcome.state, state::EOF | state::FAIL);
}

#[test]
fn xml_end_of_input_mask_throws_from_the_document_parse() {
    let mut device = CountingDevice::new(b"<?xml version=\"1.0\"?><docu".to_vec());
    let outcome = probe(&mut device, state::EOF, &mut Handle::default());
    let failure = outcome.failure.expect("masked end of input must throw");
    assert_eq!(failure.operation(), "PCDM_ReadWriter::FileFormat(stream)");
    assert_eq!(failure.consumed(), 26);
}

#[test]
fn default_mask_convenience_query_reads_binary_and_xml() {
    let mut binary = CountingDevice::new(wire::ordinary_binary(0));
    let format = PcdmReadWriterBase::file_format_stream(&mut binary, &mut Handle::default());
    assert_eq!(format.to_utf8(), "last");
    let mut xml = CountingDevice::new(XML.to_vec());
    let format = PcdmReadWriterBase::file_format_stream(&mut xml, &mut Handle::default());
    assert_eq!(format.to_utf8(), "XmlOcaf");
}
