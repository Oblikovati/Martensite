//! Preserve native raster arithmetic, then expose both attachment kinds as top-left rows.
use crate::{RenderError, error::rejected};

/// Normalize complete attachment rows; `top_left_attachment_rows(bytes, width)?` preserves pixel bits.
pub(super) fn top_left_attachment_rows(
    mut bytes: Vec<u8>,
    width: u32,
) -> Result<Vec<u8>, RenderError> {
    let (stride, rows) = validated_attachment_rows(&bytes, width)?;
    for row in 0..rows / 2 {
        let (earlier, later) = bytes.split_at_mut((rows - row - 1) * stride);
        earlier[row * stride..(row + 1) * stride].swap_with_slice(&mut later[..stride]);
    }
    Ok(bytes)
}

/// Validate either row convention without changing bits; `validated_attachment_rows(&bytes, width)?`.
pub(super) fn validated_attachment_rows(
    bytes: &[u8],
    width: u32,
) -> Result<(usize, usize), RenderError> {
    let stride = width as usize * 4;
    if stride == 0 || bytes.is_empty() || !bytes.len().is_multiple_of(stride) {
        return Err(rejected(
            (width, bytes.len()),
            "complete positive rows of four-byte pixels",
        ));
    }
    Ok((stride, bytes.len() / stride))
}

#[cfg(test)]
mod tests {
    use super::top_left_attachment_rows;
    #[test]
    fn asymmetric_rgba_rows_keep_pixel_order_and_center_row() {
        let bottom_up = vec![
            9, 0, 0, 255, 10, 0, 0, 255, 5, 0, 0, 255, 6, 0, 0, 255, 1, 0, 0, 255, 2, 0, 0, 255,
        ];
        let top_down = vec![
            1, 0, 0, 255, 2, 0, 0, 255, 5, 0, 0, 255, 6, 0, 0, 255, 9, 0, 0, 255, 10, 0, 0, 255,
        ];
        assert_eq!(top_left_attachment_rows(bottom_up, 2).unwrap(), top_down);
    }
    #[test]
    fn depth_rows_preserve_exact_float_bit_patterns() {
        let values = [0.875_f32, 0.5, 0.125];
        let bytes = values.into_iter().flat_map(f32::to_ne_bytes).collect();
        let expected: Vec<u8> = [0.125_f32, 0.5, 0.875]
            .into_iter()
            .flat_map(f32::to_ne_bytes)
            .collect();
        assert_eq!(top_left_attachment_rows(bytes, 1).unwrap(), expected);
    }
    #[test]
    fn incomplete_attachment_rows_report_the_offending_shape() {
        for (width, bytes) in [(0, vec![0; 4]), (2, vec![0; 4]), (1, Vec::new())] {
            let expected = format!("({width}, {})", bytes.len());
            let error = top_left_attachment_rows(bytes, width)
                .unwrap_err()
                .to_string();
            assert!(error.contains(&expected));
            assert!(error.contains("complete positive rows"));
        }
    }
}
