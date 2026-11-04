use super::*;
fn point_bytes(indices: &[u32]) -> Vec<u8> {
    indices
        .iter()
        .flat_map(|index| [0x80000000u32, 0x3f800000, 0x3f000000, 0x3f800000, *index])
        .flat_map(u32::to_ne_bytes)
        .collect()
}
#[test]
fn point_output_layout_has_no_fabricated_normal_and_counts_each_point() {
    let request = VertexOutputRequest::points(vec![7, 2, 7], 60).unwrap();
    assert_eq!(request.format(), VertexOutputFormat::PointsClipId20);
    assert_eq!(request.format().stride(), 20);
    assert!(request.counts(3, 3).is_ok());
    assert!(request.counts(1, 1).is_err());
    let records = request.decode_points(&point_bytes(&[7, 2, 7])).unwrap();
    assert_eq!(
        records[0].clip_bits,
        [0x80000000, 0x3f800000, 0x3f000000, 0x3f800000]
    );
    assert_eq!(records[0].vertex_index, 7);
    assert!(request.decode(&point_bytes(&[7, 2, 7])).is_err());
}
#[test]
fn point_capacity_and_complete_direct_identity_are_required() {
    assert!(VertexOutputRequest::points(vec![], 20).is_err());
    assert!(VertexOutputRequest::points(vec![1, 2], 39).is_err());
    let request = VertexOutputRequest::points(vec![7, 2, 7], 60).unwrap();
    for indices in [vec![7, 2], vec![7, 2, 8], vec![7, 2, 2]] {
        assert!(request.decode_points(&point_bytes(&indices)).is_err());
    }
    let mut conflict = point_bytes(&[7, 2, 7]);
    conflict[40..44].copy_from_slice(&1u32.to_ne_bytes());
    assert!(request.decode_points(&conflict).is_err());
}
#[test]
fn legacy_triangle_constructor_and_record_layout_remain_default() {
    let request = VertexOutputRequest::new(vec![0, 1, 2], 96).unwrap();
    assert_eq!(
        request.format(),
        VertexOutputFormat::TrianglesClipNormalId32
    );
    assert_eq!(request.format().stride(), 32);
    assert!(request.counts(1, 1).is_ok());
    assert!(request.decode_points(&point_bytes(&[0, 1, 2])).is_err());
    assert!(VertexOutputRequest::new(vec![0], 32).is_err());
}
#[test]
fn text_output_decodes_clip_uv_and_direct_id_without_tolerance() {
    let request = VertexOutputRequest::text(vec![4, 7, 9], 84).unwrap();
    assert_eq!(request.format(), VertexOutputFormat::TextClipUvId28);
    let words: [[u32; 7]; 3] = [
        [
            0x80000000, 0x3f800000, 0xbf800000, 0x3f800000, 0x3e000000, 0x3f000000, 4,
        ],
        [1, 2, 3, 4, 5, 6, 7],
        [8, 9, 10, 11, 12, 13, 9],
    ];
    let bytes = words
        .into_iter()
        .flatten()
        .flat_map(|word| word.to_ne_bytes())
        .collect::<Vec<_>>();
    let records = request.decode_text(&bytes).unwrap();
    assert_eq!(
        records[0].clip_bits,
        [0x80000000, 0x3f800000, 0xbf800000, 0x3f800000]
    );
    assert_eq!(records[0].uv_bits, [0x3e000000, 0x3f000000]);
    assert_eq!(
        records.iter().map(|r| r.vertex_index).collect::<Vec<_>>(),
        [4, 7, 9]
    );
    assert!(request.decode(&bytes).is_err());
    assert!(request.decode_text(&bytes[..bytes.len() - 1]).is_err());
}
