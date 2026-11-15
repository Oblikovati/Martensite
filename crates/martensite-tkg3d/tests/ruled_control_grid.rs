//! Bit-exact contiguous copying and legacy indexed-error order for #464.
use martensite_tkernel::ncollection::Array1;
use martensite_tkg2d::geom2d::arrays::at;
use martensite_tkg3d::geom::ruled_control_grid;
use martensite_tkmath::gp::Point3;

fn poles(lower: i32, upper: i32) -> Array1<'static, Point3> {
    let mut result = Array1::with_bounds(lower, upper).unwrap();
    for index in lower..=upper {
        result
            .set_value(index, Point3::new(f64::from(index), -0., 0.))
            .unwrap();
    }
    result
}
#[test]
fn shifted_contiguous_ranges_copy_the_requested_one_based_indices_bit_exactly() {
    let first = poles(-1, 3);
    let last = poles(0, 2);
    let (grid, weights) = ruled_control_grid([&first, &last], None, 2).unwrap();
    assert!(weights.is_none());
    for index in 1..=2 {
        assert_eq!(grid.value(index, 1).unwrap(), first.value(index).unwrap());
        assert_eq!(
            grid.value(index, 2).unwrap().y().to_bits(),
            (-0_f64).to_bits()
        );
    }
}
#[test]
fn missing_second_first_index_precedes_missing_first_second_index() {
    let first = poles(1, 1);
    let last = poles(2, 2);
    let expected = at(&last, 1).unwrap_err();
    let actual = ruled_control_grid([&first, &last], None, 2).err().unwrap();
    assert_eq!(actual.to_string(), expected.to_string());
}
#[test]
fn missing_first_index_preserves_the_original_first_column_error() {
    let first = poles(2, 3);
    let last = poles(0, 2);
    let expected = at(&first, 1).unwrap_err();
    let actual = ruled_control_grid([&first, &last], None, 2).err().unwrap();
    assert_eq!(actual.to_string(), expected.to_string());
}
