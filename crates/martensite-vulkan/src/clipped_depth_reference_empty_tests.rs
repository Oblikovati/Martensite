use super::*;
#[test]
fn exact_zero_area_is_raster_empty_even_when_its_line_contains_lattice_points() {
    let mut budget = 0;
    assert_eq!(
        reference_empty([[0, 0], [256, 256], [512, 512]], &mut budget),
        Ok(true)
    );
    assert_eq!(budget, 0);
}
#[test]
fn closed_boundary_hits_cannot_be_pruned_by_a_top_left_assumption() {
    let mut budget = 65536;
    assert_eq!(
        reference_empty([[0, 0], [256, 1], [0, 127]], &mut budget),
        Ok(false)
    );
    assert_eq!(
        reference_empty([[0, 1], [256, 1], [0, 127]], &mut budget),
        Ok(true)
    );
}
#[test]
fn negative_coordinates_and_reversed_winding_use_exact_signed_floor_and_ceiling() {
    let triangle = [[-257, -255], [-1, -255], [-257, -129]];
    let mut budget = 65536;
    assert_eq!(reference_empty(triangle, &mut budget), Ok(true));
    assert_eq!(
        reference_empty([triangle[2], triangle[1], triangle[0]], &mut budget),
        Ok(true)
    );
    assert_eq!(lattice_bounds(triangle), [[-1, 0], [-1, -1]]);
}
#[test]
fn exhausted_work_is_incomplete_and_does_not_spend_an_unperformed_scan() {
    let triangle = [[0, 0], [65536, 0], [0, 65536]];
    let mut budget = 65536;
    assert_eq!(
        reference_empty(triangle, &mut budget),
        Err(ReferenceEmptyFailure::Budget {
            requested: 66049,
            remaining: 65536
        })
    );
    assert_eq!(budget, 65536);
    assert!(reference_empty([[16777216, 0], [1, 1], [0, 1]], &mut budget).is_err());
}
#[test]
fn actual_offsets_must_be_exactly_on_the_declared_lattice() {
    for offset in [[0., 0.], [0.0625, 0.4375], [0.6875, 0.9375], [1., 1.]] {
        assert!(sample_offset(offset).is_ok());
    }
    for offset in [
        [0.1, 0.5],
        [-0.0625, 0.],
        [f32::NAN, 0.],
        [f32::INFINITY, 0.],
    ] {
        assert!(sample_offset(offset).is_err());
    }
}
#[test]
fn retained_source_predicted_folded_fan_has_no_closed_lattice_hit() {
    let mut budget = 65536;
    let triangle = [[2109312, 1051033], [2151016, 1102093], [2109312, 1051034]];
    assert_eq!(reference_empty(triangle, &mut budget), Ok(true));
    assert_eq!(budget, 65536 - 32600);
}

#[test]
fn one_certified_reference_empty_fan_leaves_the_unique_existing_plane() {
    let polygon = [[0, 0], [256, 0], [256, 256], [256, 256]];
    assert_eq!(certified_empty_mask(&polygon), Ok(2));
    assert_eq!(
        crate::clipped_depth_partition::selected_certified_plane(&polygon, [999, 999]),
        Ok(Some(0))
    );
    let empty = [[0, 0], [256, 256], [512, 512]];
    assert_eq!(certified_empty_mask(&empty), Ok(1));
    assert_eq!(
        crate::clipped_depth_partition::selected_certified_plane(&empty, [256, 256]),
        Ok(None)
    );
}
#[test]
fn mixed_reference_domains_need_a_certificate_not_a_first_matching_owner() {
    let polygon = [
        [2109312, 1051033],
        [2149930, 1101896],
        [2151016, 1102093],
        [2109312, 1051034],
    ];
    assert_eq!(certified_empty_mask(&polygon), Ok(2));
    assert_eq!(
        crate::clipped_depth_partition::selected_certified_plane(&polygon, [0, 0]),
        Ok(Some(0))
    );
    assert!(certified_empty_mask(&[[0, 0], [4096, 0], [4096, 4096], [4096, 2048]]).is_err());
    assert!(certified_empty_mask(&[[0, 0]; 5]).is_err());
}
#[test]
fn physical_q8_covered_sample_does_not_make_a_q12_reference_plane_exist() {
    let physical = [[32768, 32704], [33024, 32960], [32769, 32704]];
    let reference = [[524288, 523264], [528384, 527360], [524296, 523272]];
    assert_eq!(
        crate::clipped_depth_partition::selected(&physical, [32912, 32848]),
        Ok(Some(0))
    );
    let mut budget = 65536;
    assert_eq!(reference_empty(reference, &mut budget), Ok(true));
    assert_eq!(
        crate::clipped_depth_partition::selected_certified_plane(&reference, [526592, 525568]),
        Ok(None)
    );
}
