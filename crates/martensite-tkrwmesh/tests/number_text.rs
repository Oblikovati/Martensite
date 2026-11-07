//! The numbers of the mesh formats read back bit-identical.
use martensite_tkrwmesh::rw_mesh::{double_precision_text, single_precision_text};

#[test]
fn single_precision_values_read_back_bit_identical() {
    for value in [
        1.0e-7_f32,
        -1.0e-7,
        1234.5678,
        16_777_215.0,
        -8_388_607.5,
        1.0 + f32::EPSILON,
        f32::MAX,
        f32::MIN_POSITIVE,
    ] {
        let text = single_precision_text(value);
        assert_eq!(
            text.parse::<f32>().unwrap().to_bits(),
            value.to_bits(),
            "{text}"
        );
    }
}

#[test]
fn double_values_read_back_bit_identical() {
    for value in [1.0e-7_f64, 1234.56789012, 1.0 + f64::EPSILON, f64::MAX] {
        let text = double_precision_text(value);
        assert_eq!(
            text.parse::<f64>().unwrap().to_bits(),
            value.to_bits(),
            "{text}"
        );
    }
}

#[test]
fn the_text_is_the_shortest_form_of_its_digits() {
    assert_eq!(single_precision_text(0.5), "0.5");
    assert_eq!(single_precision_text(0.0), "0");
    assert_eq!(single_precision_text(1234.5678), "1234.56775");
    assert_eq!(single_precision_text(1.0e-7), "1.00000001e-07");
    assert_eq!(double_precision_text(0.1), "0.10000000000000001");
}
