use super::{baseline, fakes::FailingBufferedInput, support};
use martensite_tkernel::{
    fsd::CheckedReadError,
    standard::Handle,
    storage::{
        Data,
        checked_input::{InputContext, PositionedInput, UnformattedInput},
        state,
    },
};
use std::io::{self, BufRead, Read, Seek, SeekFrom};

struct ReportedOffsetInput {
    inner: FailingBufferedInput,
    origin: u64,
    fixed: Option<u64>,
}
impl BufRead for ReportedOffsetInput {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }
    fn consume(&mut self, amount: usize) {
        self.inner.consume(amount);
    }
}
impl Read for ReportedOffsetInput {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        self.inner.read(output)
    }
}
impl Seek for ReportedOffsetInput {
    fn seek(&mut self, offset: SeekFrom) -> io::Result<u64> {
        assert_eq!(offset, SeekFrom::Current(0));
        Ok(self.fixed.unwrap_or(self.origin + self.inner.cursor as u64))
    }
}
fn reported(origin: u64, fixed: Option<u64>) -> ReportedOffsetInput {
    ReportedOffsetInput {
        inner: support::source(&baseline::empty_container()),
        origin,
        fixed,
    }
}
fn assert_range(error: CheckedReadError, position: u64) {
    let CheckedReadError::Domain(failure) = error else {
        panic!("expected typed offset range failure")
    };
    assert_eq!(failure.exception_type(), "Standard_OutOfRange");
    assert_eq!(
        failure.message(),
        format!("binary container position {position}; expected 0..=2147483647")
    );
}
fn existing_dataset() -> Handle<Data> {
    let set = Handle::new(Data::default());
    set.get()
        .unwrap()
        .header_data()
        .get()
        .unwrap()
        .set_number_of_objects(99);
    set
}
fn assert_unrepresentable(source: &mut ReportedOffsetInput, mask: u8, offset: u64) {
    let mut set = existing_dataset();
    let identity = set.as_ptr();
    let mut positioned = PositionedInput(&mut *source);
    let mut input = InputContext::new(&mut positioned);
    input.set_exception_mask(mask).unwrap();
    let error = support::driver()
        .read_complete_info_checked(&mut input, &mut set)
        .unwrap_err();
    assert_range(error, offset);
    assert_eq!(
        (input.state(), input.exception_mask(), input.gcount()),
        (0, mask, 4)
    );
    assert!(input.last_device_error().is_none());
    assert_eq!(set.as_ptr(), identity);
    assert_eq!(set.get().unwrap().number_of_objects(), 0);
    assert_untouched_sections(source);
}
fn assert_untouched_sections(source: &ReportedOffsetInput) {
    assert_eq!(source.inner.cursor, 95);
    assert_eq!(
        &source.inner.bytes[95..],
        &baseline::empty_container()[95..]
    );
}
#[test]
fn wide_origin_cannot_alias_small_section_offsets() {
    for mask in [0, state::BAD] {
        let mut source = reported(1_u64 << 32, None);
        assert_unrepresentable(&mut source, mask, (1_u64 << 32) + 95);
    }
}
#[test]
fn largest_representable_offset_is_accepted_without_reading_sections() {
    let mut source = reported(0, Some(i32::MAX as u64));
    let mut set = Handle::<Data>::default();
    let mut positioned = PositionedInput(&mut source);
    let mut input = InputContext::new(&mut positioned);
    support::driver()
        .read_complete_info_checked(&mut input, &mut set)
        .unwrap();
    assert_eq!((input.state(), input.gcount()), (0, 4));
    assert_eq!(input.position().unwrap(), Some(i32::MAX as u64));
    assert_eq!(source.inner.cursor, 95);
}
#[test]
fn first_unrepresentable_offset_preserves_good_input_state() {
    let offset = i32::MAX as u64 + 1;
    let mut source = reported(0, Some(offset));
    assert_unrepresentable(&mut source, state::BAD, offset);
}
