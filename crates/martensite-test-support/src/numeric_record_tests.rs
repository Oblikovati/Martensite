use crate::numeric_tokens::Reader;
use martensite_tkmath::gp::{Point2, Point3, Xyz};

#[test]
fn reader_retains_raw_floating_bits_and_advances_one_field() {
    let values = [-0., f64::from_bits(0x7ff8_0000_0000_0123), 42.];
    let mut reader = Reader::new(&values);
    for expected in values {
        assert_eq!(reader.real().to_bits(), expected.to_bits());
    }
}

#[test]
fn integer_and_flag_keep_the_original_truncating_conversion() {
    let mut reader = Reader::new(&[1.9, -2.9, 0.9, -1.9]);
    assert_eq!(reader.integer(), 1);
    assert_eq!(reader.integer(), -2);
    assert!(!reader.flag());
    assert!(reader.flag());
}

#[test]
fn length_prefixed_fields_retain_order_and_leave_the_next_field() {
    let mut reader = Reader::new(&[3., 2., -1., 4., 19.]);
    assert_eq!(reader.reals(), [2., -1., 4.]);
    assert_eq!(reader.real(), 19.);
}

#[test]
fn negative_length_keeps_the_existing_empty_range_behavior() {
    let mut reader = Reader::new(&[-2., 19.]);
    assert!(reader.reals().is_empty());
    assert_eq!(reader.real(), 19.);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn exhausted_reader_keeps_the_existing_bounds_failure() {
    Reader::new(&[]).real();
}

#[test]
fn geometric_fields_keep_their_original_coordinate_order() {
    let mut reader = Reader::new(&[1., 2., 3., 4., 5., 6., 7., 8.]);
    assert_eq!(reader.xyz(), Xyz::new(1., 2., 3.));
    assert_eq!(reader.pnt(), Point3::new(4., 5., 6.));
    assert_eq!(reader.pnt2d(), Point2::new(7., 8.));
}

#[test]
fn geometric_directions_keep_existing_normalization() {
    let mut reader = Reader::new(&[0., 0., 2., 3., 4.]);
    let spatial = reader.dir();
    let planar = reader.dir2d();
    assert_eq!((spatial.x(), spatial.y(), spatial.z()), (0., 0., 1.));
    crate::floating::expect_double_eq(planar.x(), 0.6);
    crate::floating::expect_double_eq(planar.y(), 0.8);
}

#[test]
#[should_panic(expected = "probe direction")]
fn zero_direction_keeps_the_original_named_failure() {
    Reader::new(&[0., 0., 0.]).dir();
}
