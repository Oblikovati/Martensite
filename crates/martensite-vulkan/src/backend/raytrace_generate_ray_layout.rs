//! Independent row/transport controls preserve the observed raw bits without fitting rows.
use super::inputs::RayProbeInput;
pub(super) fn require_ray_column(bytes: &[u8], input: &RayProbeInput, leading: u32) {
    assert_eq!(bytes.len(), super::RAY_BYTES);
    for (index, row) in bytes.as_chunks::<{ 48 * 4 }>().0.iter().enumerate() {
        let words: [u32; 48] =
            std::array::from_fn(|i| u32::from_ne_bytes(row[i * 4..i * 4 + 4].try_into().unwrap()));
        require_row_header(&words, index as u32, leading);
        assert_eq!(words[32..42], input.constants);
        assert_eq!(words[28..31], input_vector_bits(input, 192));
        assert_eq!(words[44..47], input_vector_bits(input, 256));
        for start in [4, 8, 12, 16] {
            assert!(
                words[start..start + 3]
                    .iter()
                    .all(|bits| f32::from_bits(*bits).is_finite())
            );
        }
    }
}
fn require_row_header(words: &[u32; 48], row: u32, leading: u32) {
    assert_eq!(words[20], 1);
    assert_eq!(words[2], 0.5f32.to_bits());
    assert_eq!(words[23], row + leading);
    assert_eq!(words[21], 204.5f32.to_bits());
    assert_eq!(words[22], (row as f32 + leading as f32 + 0.5).to_bits());
    assert_eq!(words[24..28], [409, 409, 1, leading]);
}
fn input_vector_bits(input: &RayProbeInput, offset: usize) -> [u32; 3] {
    std::array::from_fn(|i| {
        u32::from_le_bytes(
            input.uniforms[offset + i * 4..offset + i * 4 + 4]
                .try_into()
                .unwrap(),
        )
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn row_header_checks_actual_leading_row_and_single_visit() {
        let mut words = [0; 48];
        words[2] = 0.5f32.to_bits();
        words[20] = 1;
        words[21] = 204.5f32.to_bits();
        words[22] = 1.5f32.to_bits();
        words[23] = 1;
        words[24..28].copy_from_slice(&[409, 409, 1, 1]);
        require_row_header(&words, 0, 1);
        words[23] = 0;
        assert!(std::panic::catch_unwind(|| require_row_header(&words, 0, 1)).is_err());
        words[23] = 1;
        words[20] = 0;
        assert!(std::panic::catch_unwind(|| require_row_header(&words, 0, 1)).is_err());
    }
}
