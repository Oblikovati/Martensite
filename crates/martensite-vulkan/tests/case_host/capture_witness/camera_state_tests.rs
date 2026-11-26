use super::*;

#[test]
fn camera_snapshot_encoder_preserves_raw_ieee_words_and_matrix_order() {
    let value = CameraStateSnapshot {
        eye_f64: [1.0_f64.to_bits(), (-0.0_f64).to_bits(), 3.0_f64.to_bits()],
        direction_f64: [4.0_f64.to_bits(), 5.0_f64.to_bits(), 6.0_f64.to_bits()],
        up_f64: [7.0_f64.to_bits(), 8.0_f64.to_bits(), 9.0_f64.to_bits()],
        axial_scale_f64: [10.0_f64.to_bits(), 11.0_f64.to_bits(), 12.0_f64.to_bits()],
        scale_f64: 13.0_f64.to_bits(),
        near_f64: 14.0_f64.to_bits(),
        far_f64: 15.0_f64.to_bits(),
        orientation_f32: [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ],
        projection_f32: [[16; 4]; 4],
    };
    let encoded = String::from_utf8(encode(&value)).unwrap();
    assert!(encoded.contains("float64_words=ieee754-hex"));
    assert!(encoded.contains("eye=[3ff0000000000000, 8000000000000000, 4008000000000000]"));
    assert!(encoded.contains("orientation_f32=[[00000001, 00000002, 00000003, 00000004]"));
    assert!(encoded.contains("projection_f32=[[00000010, 00000010, 00000010, 00000010]"));
}
