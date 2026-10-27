//! A shared input carries real archive decoding, caller observations and owned geometry.
#[path = "common/geometry_io.rs"]
mod fakes;
#[path = "borrowed_shape_input/geometry.rs"]
mod geometry;
#[path = "borrowed_shape_input/input.rs"]
mod input;
#[path = "borrowed_shape_input/reader.rs"]
mod reader;
use martensite_tkernel::{
    osd::{IStream, SeekDir, StreamAccess},
    storage::with_borrowed_input_stream,
};
use std::sync::Arc;

const RECORD: &[u8] = include_bytes!("fixtures/rectangular-solid-10-20-30.brep");
#[derive(Clone, Copy)]
struct Case {
    name: &'static str,
    limit: Option<usize>,
    offset: i64,
    initial: u8,
    seekable: bool,
    expected: (u8, usize, i64, bool, bool),
}
const CASES: [Case; 7] = [
    Case {
        name: "complete",
        limit: None,
        offset: 0,
        initial: 0,
        seekable: true,
        expected: (0, 1, 2544, true, false),
    },
    Case {
        name: "offset",
        limit: None,
        offset: 19,
        initial: 0,
        seekable: true,
        expected: (0, 1, 2544, true, false),
    },
    Case {
        name: "short",
        limit: Some(100),
        offset: 0,
        initial: 0,
        seekable: true,
        expected: (6, 38, 100, false, false),
    },
    Case {
        name: "empty",
        limit: Some(0),
        offset: 0,
        initial: 0,
        seekable: true,
        expected: (6, 0, 0, false, false),
    },
    Case {
        name: "failed",
        limit: None,
        offset: 0,
        initial: 4,
        seekable: true,
        expected: (4, 0, 0, false, false),
    },
    Case {
        name: "eof",
        limit: None,
        offset: 0,
        initial: 2,
        seekable: true,
        expected: (0, 1, 2544, true, false),
    },
    Case {
        name: "nonseeking",
        limit: None,
        offset: 0,
        initial: 0,
        seekable: false,
        expected: (4, 0, -1, false, false),
    },
];
struct ObservedCase {
    case: Case,
    initial: u8,
    actual: (u8, usize, i64, bool, bool),
    same_buffer: bool,
    decoded: reader::Decoded,
}
#[test]
fn real_archive_reads_preserve_all_declared_caller_observations() {
    let observed = CASES.map(observe_case);
    for case in observed {
        assert_case(case);
    }
}
fn observe_case(case: Case) -> ObservedCase {
    let mut source = input::source(case_bytes(case), case.seekable);
    initialize(&mut source, case);
    let initial = source.rdstate().bits();
    let buffer = source.rdbuf();
    let decoded = with_borrowed_input_stream(&mut source, reader::decode).unwrap();
    let actual = observation(&source, &decoded);
    reader::report(case.name, initial, actual, &decoded);
    let same_buffer = Arc::ptr_eq(&buffer, &source.rdbuf());
    ObservedCase {
        case,
        initial,
        actual,
        same_buffer,
        decoded,
    }
}
fn assert_case(observed: ObservedCase) {
    assert_eq!(observed.initial, observed.case.initial);
    assert_eq!(
        observed.actual, observed.case.expected,
        "profile {}: expected literal complete observation",
        observed.case.name
    );
    assert!(observed.same_buffer);
    if !observed.decoded.shape.is_null() {
        geometry::assert_box(&observed.decoded.shape);
    }
}
fn initialize(source: &mut martensite_tkernel::osd::IStreamBuffer, case: Case) {
    if case.offset != 0 {
        source.seekg(case.offset, SeekDir::Beg);
    }
    if case.initial == 4 {
        source.seekg(-1, SeekDir::Beg);
    }
    if case.initial == 2 {
        with_borrowed_input_stream(source, |stream| stream.clear(2)).unwrap();
    }
}
#[test]
fn decoded_geometry_outlives_its_stream_and_original_byte_owner() {
    let shape = {
        let mut source = input::source(RECORD, true);
        let decoded = with_borrowed_input_stream(&mut source, reader::decode).unwrap();
        decoded.result.unwrap().unwrap();
        decoded.shape
    };
    geometry::assert_box(&shape);
}
#[test]
fn explicit_caller_clear_changes_all_retained_failure_bits() {
    let mut source = input::source(RECORD, true);
    with_borrowed_input_stream(&mut source, |stream| stream.clear(6)).unwrap();
    assert_eq!(source.rdstate().bits(), 6);
    source.clear();
    assert_eq!(source.rdstate().bits(), 0);
    let decoded = with_borrowed_input_stream(&mut source, reader::decode).unwrap();
    assert!(decoded.result.as_ref().is_some_and(Result::is_ok));
    geometry::assert_box(&decoded.shape);
}

fn case_bytes(case: Case) -> &'static [u8] {
    let length = case.limit.unwrap_or(RECORD.len());
    &RECORD[..length]
}

fn observation(
    source: &martensite_tkernel::osd::IStreamBuffer,
    decoded: &reader::Decoded,
) -> (u8, usize, i64, bool, bool) {
    (
        source.rdstate().bits(),
        source.gcount(),
        input::position(source),
        !decoded.shape.is_null(),
        decoded.result.as_ref().is_some_and(Result::is_err),
    )
}
