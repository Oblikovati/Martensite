use super::*;

#[test]
fn compact_indices_keep_65535_and_add_padding_after_the_last_index() {
    let bytes = IndexFormat::U16.encode(&[1, 65_535, 0]).unwrap();
    assert_eq!(
        bytes,
        [1u16, 65_535, 0, 0].map(u16::to_ne_bytes).as_flattened()
    );
    assert!(IndexFormat::U16.encode(&[65_536]).is_none());
}

#[test]
fn compact_index_patches_include_word_neighbors_and_the_terminal_pad() {
    let indices = [0, 4, 1, 2, 7];
    let (offset, bytes) = IndexFormat::U16.patch(&indices, 1..3, 12).unwrap();
    assert_eq!(offset, 0);
    assert_eq!(bytes, [0u16, 4, 1, 2].map(u16::to_ne_bytes).as_flattened());
    let (offset, bytes) = IndexFormat::U16.patch(&indices, 4..5, 12).unwrap();
    assert_eq!(offset, 8);
    assert_eq!(bytes, [7u16, 0].map(u16::to_ne_bytes).as_flattened());
    assert!(IndexFormat::U16.patch(&indices, 4..6, 12).is_none());
    assert!(IndexFormat::U16.patch(&indices, 4..5, 20).is_none());
}

#[test]
fn full_width_index_patches_keep_the_exact_dirty_range() {
    let indices = [11, 72_131, 19];
    let (offset, bytes) = IndexFormat::U32.patch(&indices, 1..2, 12).unwrap();
    assert_eq!(offset, 4);
    assert_eq!(bytes, 72_131u32.to_ne_bytes());
    assert_eq!(
        IndexFormat::U32.patch(&indices, 3..3, 12),
        Some((0, vec![]))
    );
}

#[test]
#[cfg(not(any(
    feature = "command-state-diagnostics",
    feature = "vertex-output-diagnostics",
    feature = "lit-only-command-diagnostics"
)))]
fn stock_triangle_selection_keeps_the_full_u16_index_domain() {
    let vertices = vec![crate::RenderVertex::new([0.; 3], [1.; 4]); 65_537];
    let compact = PrimitiveDraw::new(
        vertices[..65_536].to_vec(),
        vec![65_535; 3],
        crate::identity_clip(),
    )
    .unwrap();
    let wide = PrimitiveDraw::new(vertices, vec![65_536; 3], crate::identity_clip()).unwrap();
    assert_eq!(IndexFormat::of_draw(&compact), IndexFormat::U16);
    assert_eq!(IndexFormat::of_draw(&wide), IndexFormat::U32);
}

#[test]
fn index_formats_report_their_device_types_and_byte_widths() {
    assert_eq!(
        (IndexFormat::U16.vk_type(), IndexFormat::U16.byte_width()),
        (vk::IndexType::UINT16, 2)
    );
    assert_eq!(
        (IndexFormat::U32.vk_type(), IndexFormat::U32.byte_width()),
        (vk::IndexType::UINT32, 4)
    );
}
