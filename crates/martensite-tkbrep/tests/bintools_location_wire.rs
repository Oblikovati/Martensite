//! Native location-table wire layout, datum ownership and exact failure payloads.
#[path = "bintools_location_wire/expected.rs"]
mod expected;
use martensite_tkbrep::bintools::BinLocationSet;
use martensite_tkernel::{standard::Failure, storage::IoStream};
use martensite_tkmath::{
    gp::{Point3, Transform3},
    toploc::Location,
};

fn translated(offset: [f64; 3]) -> Location {
    let transform = Transform3::translation(offset);
    Location::new(transform)
}

fn written(set: &BinLocationSet) -> Vec<u8> {
    let mut stream = IoStream::from_bytes(Vec::new());
    set.write(&mut stream).unwrap();
    let count = stream.tellp() as usize;
    let mut bytes = vec![0; count];
    stream.seekg(0).read(&mut bytes);
    assert!(!stream.fail());
    bytes
}

fn check_failure(error: Failure, expected: &[u8]) {
    assert_eq!(error.exception_type(), "Standard_Failure");
    assert_eq!(error.message_bytes(), expected);
}

#[test]
fn constructor_identity_unknown_index_duplicate_datum_and_clear() {
    let mut set = BinLocationSet::new();
    let a = translated([1., 2., 3.]);
    assert_eq!(set.nb_locations(), 0);
    assert_eq!(set.add(&Location::default()), 0);
    assert!(set.location(0).unwrap().is_identity());
    assert_eq!(set.index(&a), 0);
    assert_eq!(set.add(&a), 1);
    assert_eq!(set.add(&a.clone()), 1);
    assert_eq!(set.add(&translated([1., 2., 3.])), 2);
    assert_eq!(set.index(&a), 1);
    set.clear();
    assert_eq!(set.nb_locations(), 0);
    assert_eq!(set.index(&a), 0);
    assert_eq!(written(&set), b"Locations 0\n");
}

#[test]
fn elementary_write_is_twelve_native_order_doubles_in_row_major_order() {
    let mut set = BinLocationSet::new();
    set.add(&translated([1., 2., 3.]));
    assert_eq!(
        written(&set),
        expected::table(&[expected::translation_record([1., 2., 3.])])
    );
}

#[test]
fn negative_power_write_defines_its_datum_before_the_power_reference() {
    let a = translated([1., 2., 3.]);
    let mut set = BinLocationSet::new();
    assert_eq!(set.add(&a.powered(-2).unwrap()), 2);
    assert_eq!(set.index(&a), 1);
    let records = [
        expected::translation_record([1., 2., 3.]),
        expected::chain_record(&[(1, -2)]),
    ];
    assert_eq!(written(&set), expected::table(&records));
}

fn compound_wire() -> Vec<u8> {
    expected::table(&[
        expected::translation_record([-4., 5., 6.]),
        expected::translation_record([1., 2., 3.]),
        expected::chain_record(&[(1, 1), (2, 2)]),
    ])
}

#[test]
fn compound_write_and_read_preserve_datum_order_power_and_shared_identity() {
    let a = translated([1., 2., 3.]);
    let b = translated([-4., 5., 6.]);
    let compound = a.powered(2).unwrap().multiplied(&b).unwrap();
    let mut set = BinLocationSet::new();
    assert_eq!(set.add(&compound), 3);
    assert_eq!(written(&set), compound_wire());
    let mut read = BinLocationSet::new();
    read.read(&mut IoStream::from_bytes(compound_wire()))
        .unwrap();
    let chain = read.location(3).unwrap();
    assert_eq!(chain.transformation().translation_part().x(), -2.);
    assert_eq!(chain.first_power().unwrap(), 1);
    assert_eq!(
        chain.first_datum().unwrap().as_ptr(),
        read.location(1).unwrap().first_datum().unwrap().as_ptr()
    );
    assert_eq!(chain.next_location().first_power().unwrap(), 2);
    assert_eq!(written(&read), compound_wire());
}

#[test]
fn bad_tag_clears_the_previous_table_and_preserves_native_unwrapped_message() {
    let mut set = BinLocationSet::new();
    set.add(&translated([8., 0., 0.]));
    let mut input = IoStream::from_bytes(b"Bogus 0\n".to_vec());
    let error = set.read(&mut input).unwrap_err();
    assert_eq!(
        error.message_bytes(),
        b"BinTools_LocationSet::Read: Not a location table\n"
    );
    assert_eq!(set.nb_locations(), 0);
    assert!(!input.fail());
}

#[test]
fn every_unknown_record_kind_preserves_raw_bytes_including_nul_termination() {
    for kind in (0_u8..=255).filter(|kind| ![1, 2].contains(kind)) {
        let mut set = BinLocationSet::new();
        let mut input = IoStream::from_bytes(expected::table(&[vec![kind]]));
        check_failure(
            set.read(&mut input).unwrap_err(),
            &expected::unknown_kind(kind),
        );
        assert_eq!(set.nb_locations(), 0);
        assert!(!input.fail());
    }
}

#[test]
fn eof_casts_to_ff_and_retains_the_input_failure_and_eof_flags() {
    let mut set = BinLocationSet::new();
    let mut input = IoStream::from_bytes(b"Locations 1\n".to_vec());
    check_failure(
        set.read(&mut input).unwrap_err(),
        &expected::unknown_kind(255),
    );
    assert!(input.fail() && input.eof());
}

#[test]
fn later_bad_record_retains_completed_locations_after_clearing_old_state() {
    let mut set = BinLocationSet::new();
    let seed = translated([8., 0., 0.]);
    set.add(&seed);
    let first = expected::translation_record([1., 2., 3.]);
    let bytes = expected::table(&[first, vec![128]]);
    let mut input = IoStream::from_bytes(bytes);
    check_failure(
        set.read(&mut input).unwrap_err(),
        &expected::unknown_kind(128),
    );
    assert_eq!(set.nb_locations(), 1);
    assert_eq!(set.index(&seed), 0);
    let kept = set.location(1).unwrap();
    assert_eq!(kept.transformation().translation_part().x(), 1.);
    assert!(!input.fail());
}

#[test]
fn a_short_matrix_reports_storage_mismatch_and_retains_stream_failure() {
    let mut set = BinLocationSet::new();
    let mut input = IoStream::from_bytes(expected::table(&[vec![1, 0, 0, 0, 0, 0]]));
    let message = expected::wrapped("Storage_StreamTypeMismatchError", b"");
    check_failure(set.read(&mut input).unwrap_err(), &message);
    assert!(input.fail() && input.eof());
    assert_eq!(set.nb_locations(), 0);
}

#[test]
fn a_singular_matrix_has_the_enabled_check_native_construction_error_payload() {
    let mut record = vec![1];
    record.extend([0_u8; 96]);
    let mut set = BinLocationSet::new();
    let mut input = IoStream::from_bytes(expected::table(&[record]));
    let message = expected::wrapped(
        "Standard_ConstructionError",
        b"gp_Trsf::SetValues, null determinant",
    );
    check_failure(set.read(&mut input).unwrap_err(), &message);
    assert_eq!(set.nb_locations(), 0);
    assert!(!input.fail());
}

#[test]
fn public_and_chain_range_errors_use_native_indexed_map_diagnostics() {
    for (index, suffix) in [(-1, ": negative index"), (1, "")] {
        let mut set = BinLocationSet::new();
        let expected = format!("NCollection_IndexedMap::FindKey{suffix}");
        let error = set.location(index).unwrap_err();
        assert_eq!(error.exception_type(), "Standard_OutOfRange");
        assert_eq!(error.message(), expected);
        let mut input =
            IoStream::from_bytes(expected::table(&[expected::chain_record(&[(index, 1)])]));
        check_failure(
            set.read(&mut input).unwrap_err(),
            &expected::wrapped("Standard_OutOfRange", expected.as_bytes()),
        );
    }
}

#[test]
fn empty_chain_records_are_skipped_and_negative_counts_clear_without_reading() {
    let bytes = expected::table(&[
        expected::chain_record(&[]),
        expected::translation_record([1., 2., 3.]),
    ]);
    let mut set = BinLocationSet::new();
    set.read(&mut IoStream::from_bytes(bytes)).unwrap();
    assert_eq!(set.nb_locations(), 1);
    assert_eq!(
        written(&set),
        expected::table(&[expected::translation_record([1., 2., 3.])])
    );
    set.read(&mut IoStream::from_bytes(b"Locations -1\n".to_vec()))
        .unwrap();
    assert_eq!(set.nb_locations(), 0);
}

#[test]
fn a_failed_output_stream_sets_flags_without_inventing_an_exception() {
    let mut set = BinLocationSet::new();
    set.add(&translated([1., 2., 3.]));
    let mut stream = IoStream::from_bytes(Vec::new());
    stream.set_state(martensite_tkernel::storage::state::FAIL);
    assert!(set.write(&mut stream).is_ok());
    assert!(stream.fail());
    assert_eq!(set.nb_locations(), 1);
}

#[test]
fn matrix_identity_is_a_stored_datum_but_empty_chain_is_not() {
    let mut set = BinLocationSet::new();
    set.read(&mut IoStream::from_bytes(expected::table(&[
        expected::translation_record([0., 0., 0.]),
        expected::chain_record(&[]),
    ])))
    .unwrap();
    assert_eq!(set.nb_locations(), 1);
    assert!(!set.location(1).unwrap().is_identity());
    assert_eq!(set.index(&Location::default()), 0);
}

const QUARTER_TURN_ROWS: [[f64; 4]; 3] = [[0., -1., 0., 4.], [1., 0., 0., 5.], [0., 0., 1., 6.]];

fn quarter_turn() -> Location {
    let mut transform = Transform3::default();
    transform.set_values(QUARTER_TURN_ROWS).unwrap();
    Location::new(transform)
}

#[test]
fn rotated_matrix_writes_exact_offdiagonal_values_and_restores_them() {
    let mut set = BinLocationSet::new();
    set.add(&quarter_turn());
    let wire = expected::table(&[expected::matrix_record(QUARTER_TURN_ROWS)]);
    assert_eq!(written(&set), wire);
    set.read(&mut IoStream::from_bytes(wire)).unwrap();
    let point = Point3::new(1., 0., 0.);
    let transformed = point.transformed(set.location(1).unwrap().transformation());
    assert_eq!(transformed, Point3::new(4., 6., 6.));
}

#[test]
fn compound_read_preserves_noncommuting_rotation_translation_order() {
    let translated = translated([1., 2., 3.]).powered(2).unwrap();
    let compound = translated.multiplied(&quarter_turn()).unwrap();
    let mut set = BinLocationSet::new();
    set.add(&compound);
    let wire = expected::table(&[
        expected::matrix_record(QUARTER_TURN_ROWS),
        expected::translation_record([1., 2., 3.]),
        expected::chain_record(&[(1, 1), (2, 2)]),
    ]);
    assert_eq!(written(&set), wire);
    set.read(&mut IoStream::from_bytes(wire)).unwrap();
    let point = Point3::new(1., 0., 0.);
    let transformed = point.transformed(set.location(3).unwrap().transformation());
    assert_eq!(transformed, Point3::new(6., 10., 12.));
}
