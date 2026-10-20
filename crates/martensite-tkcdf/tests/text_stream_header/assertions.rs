use super::{input_support::DriverProbeDevice, support::storage_header};
use martensite_tkcdf::pcdm::{PcdmInputStream, PcdmReadWriterBase, PcdmStreamFailure};
use martensite_tkernel::{
    standard::Handle,
    storage::{Data, HeaderData, state},
    tcollection::ExtendedString,
};

struct FrozenHeaderState<'a> {
    storage: Handle<Data>,
    identity: Handle<Data>,
    header: Handle<HeaderData>,
    original_bytes: &'a [u8],
}
impl<'a> FrozenHeaderState<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        let storage = storage_header(bytes);
        let identity = storage.clone();
        let header = storage.get().unwrap().header_data();
        Self {
            storage,
            identity,
            header,
            original_bytes: bytes,
        }
    }
    fn assert_unchanged(&self) {
        assert!(self.storage == self.identity);
        assert!(self.header == self.storage.get().unwrap().header_data());
        let actual: Vec<Vec<u8>> = self
            .storage
            .get()
            .unwrap()
            .user_info()
            .iter()
            .map(|line| line.as_bytes().to_vec())
            .collect();
        let expected: Vec<Vec<u8>> = std::str::from_utf8(self.original_bytes)
            .unwrap()
            .split_terminator('\n')
            .map(|line| line.as_bytes().to_vec())
            .collect();
        assert_eq!(actual, expected);
    }
}

pub(super) fn verify_header_matrix(magic: &[u8], bytes: &[u8], expected: &str) {
    for mask in [
        0,
        state::BAD,
        state::EOF,
        state::FAIL,
        state::BAD | state::FAIL,
    ] {
        for failure_at in -1..=7 {
            verify_header_case(magic, bytes, expected, mask, failure_at);
        }
    }
}

fn verify_header_case(magic: &[u8], bytes: &[u8], expected: &str, mask: u8, failure_at: i32) {
    let mut frozen = FrozenHeaderState::new(bytes);
    let mut device = DriverProbeDevice::new(magic);
    device.failure_at = usize::try_from(failure_at).ok();
    let mut input = device.input(mask);
    let actual = PcdmReadWriterBase::file_format_input(&mut input, &mut frozen.storage);
    let failed = (0..7).contains(&failure_at);
    assert_header_result(actual, expected, failed, mask);
    assert_input_state(&input, failed, mask);
    drop(input);
    assert_eq!(
        device.consumed,
        if failed { failure_at as usize } else { 7 }
    );
    frozen.assert_unchanged();
}

fn assert_input_state(input: &PcdmInputStream<'_>, failed: bool, mask: u8) {
    let expected_state = if !failed {
        0
    } else if mask & state::BAD != 0 {
        state::BAD
    } else {
        state::BAD | state::FAIL
    };
    assert_eq!(input.state(), expected_state);
    assert_eq!(input.last_error().is_some(), failed);
}

fn assert_header_result(
    result: Result<ExtendedString, PcdmStreamFailure>,
    expected: &str,
    failed: bool,
    mask: u8,
) {
    let should_throw = failed && mask & (state::BAD | state::FAIL) != 0;
    assert_eq!(result.is_err(), should_throw);
    if let Ok(format) = result {
        let expected: Vec<u16> = if failed {
            Vec::new()
        } else {
            expected.encode_utf16().collect()
        };
        assert_eq!(format.as_utf16(), expected);
    }
}
