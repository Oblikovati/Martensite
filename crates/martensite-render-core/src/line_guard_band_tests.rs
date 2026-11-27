use super::*;
use crate::LineStyle;

fn vertical_line(span: f32) -> PrimitiveDraw {
    // Window x = 10.5 px of a 64 px viewport, y from -span/2 to +span/2 px around row 32.
    let ndc = |px: f32| px / 32. - 1.;
    let vertices = [-span / 2., span / 2.]
        .map(|y| RenderVertex::new([ndc(10.5), ndc(32. + y), 0.5], [1., 0., 0., 1.]))
        .to_vec();
    let style = LineStyle::new(1., 0xffff, 1).unwrap();
    PrimitiveDraw::segments(vertices, vec![0, 1], crate::identity_clip(), style).unwrap()
}

#[test]
fn short_lines_are_submitted_unchanged() {
    let mut draws = vec![vertical_line(16000.)];
    let before = draws[0].vertices.clone();
    clip_far_lines(&mut draws, [64, 64]);
    assert_eq!(draws[0].vertices, before);
}

#[test]
fn far_lines_are_clipped_to_the_band_and_keep_their_direction() {
    let mut draws = vec![vertical_line(1.0e6)];
    clip_far_lines(&mut draws, [64, 64]);
    let ends: Vec<[f32; 4]> = draws[0]
        .vertices
        .iter()
        .map(|v| clip_position(&draws[0], v))
        .collect();
    assert_eq!(draws[0].indices(), [0, 1]);
    // The band is 16384 px tall around the viewport: NDC +-256 for a 64 px viewport.
    assert_eq!(ends.iter().map(|c| c[1].abs()).fold(0., f32::max), 256.);
    assert!(ends.iter().all(|c| c[0] == 10.5 / 32. - 1.));
    assert!(
        draws[0]
            .vertices
            .iter()
            .all(|v| v.color == [1., 0., 0., 1.])
    );
}

#[test]
fn segments_outside_the_band_or_behind_the_eye_are_dropped() {
    let mut draw = vertical_line(1.0e6);
    for vertex in draw.vertices.make_mut() {
        vertex.position[0] = 1000.;
    }
    let mut draws = vec![draw];
    clip_far_lines(&mut draws, [64, 64]);
    assert!(draws[0].indices.is_empty());
    let mut behind = vertical_line(10.);
    behind.vertices.make_mut()[1].weight = -1.;
    assert!(GuardBand::new([64, 64]).needs_clipping(&behind));
}

#[test]
fn narrowing_keeps_the_non_negative_side() {
    assert_eq!(narrow([0., 1.], [1., -1.]), Some([0., 0.5]));
    assert_eq!(narrow([0., 1.], [-1., 3.]), Some([0.25, 1.]));
    assert_eq!(narrow([0., 1.], [-1., -2.]), None);
    assert_eq!(narrow([0.6, 1.], [1., -1.]), None);
}

#[test]
fn clipping_retains_the_submitted_geometry_bit_for_bit() {
    let submitted = vertical_line(1.0e6);
    let mut draws = vec![submitted.clone()];
    clip_far_lines(&mut draws, [64, 64]);
    let source = draws[0].guard_band_source.clone().unwrap();
    assert_eq!(source.vertices, submitted.vertices);
    assert_eq!(source.indices, submitted.indices);
    assert_ne!(draws[0].vertices, submitted.vertices);
}

#[test]
fn unclipped_draws_retain_nothing_and_a_second_preparation_keeps_the_first_source() {
    let mut short = vec![vertical_line(100.)];
    clip_far_lines(&mut short, [64, 64]);
    assert_eq!(short[0].guard_band_source, None);
    let submitted = vertical_line(1.0e6);
    let mut draws = vec![submitted.clone()];
    clip_far_lines(&mut draws, [64, 64]);
    // The band of a larger viewport spans fewer NDC units, so this second pass clips again.
    let first_band = draws[0].vertices.clone();
    clip_far_lines(&mut draws, [4096, 4096]);
    assert_ne!(draws[0].vertices, first_band);
    let source = draws[0].guard_band_source.clone().unwrap();
    assert_eq!(source.vertices, submitted.vertices);
}

#[test]
fn replacement_geometry_drops_a_stale_source() {
    let mut draws = vec![vertical_line(1.0e6)];
    clip_far_lines(&mut draws, [64, 64]);
    let replaced = draws[0].with_replacement_geometry(draws[0].vertices.clone(), vec![0, 1].into());
    assert_eq!(replaced.guard_band_source, None);
}

#[test]
fn retained_short_geometry_skips_the_scan_for_a_shared_wide_line() {
    let draw = retained_line([-1., 1.], 8.);
    let band = GuardBand::new([4096, 4096]);
    let snapshot = draw.clone();
    assert!(band.short_by_bounds(&draw));
    assert!(band.short_by_bounds(&snapshot));
    assert!(!band.scan_segments(&draw));
    assert!(!band.needs_clipping(&snapshot));
    assert_eq!(draw, snapshot);
}

#[test]
fn only_spans_strictly_below_the_f32_threshold_are_certified() {
    let band = GuardBand::new([64, 64]);
    let below = retained_line([-256., 256_f32.next_down().next_down()], 1.);
    assert!(band.short_by_bounds(&below));
    assert!(!band.scan_segments(&below));
    for maximum in [256_f32.next_down(), 256., 256_f32.next_up(), 1.0e6] {
        let draw = retained_line([-256., maximum], 1.);
        assert!(!band.short_by_bounds(&draw));
        assert!(band.scan_segments(&draw));
        assert!(band.needs_clipping(&draw));
    }
}

#[test]
fn perspective_certification_requires_a_finite_positive_weight_interval() {
    let band = GuardBand::new([3840, 2160]);
    let mut draw = retained_line([-2., 2.], 1.);
    draw.clip_transform[7] = -0.125;
    assert!(band.short_by_bounds(&draw));
    assert!(!band.scan_segments(&draw));
    draw.clip_transform[15] = 0.25;
    assert!(!band.short_by_bounds(&draw));
    assert!(band.needs_clipping(&draw));
    draw.clip_transform[15] = -1.;
    assert!(!band.short_by_bounds(&draw));
    assert!(band.needs_clipping(&draw));
    draw.clip_transform[15] = 1.;
    draw.clip_transform[6] = f32::MAX;
    assert!(!band.short_by_bounds(&draw));
    assert!(band.needs_clipping(&draw));
}

#[test]
fn cloned_geometry_mutations_cannot_reuse_the_retained_bound() {
    let band = GuardBand::new([64, 64]);
    let original = retained_line([-1., 1.], 1.);
    assert!(band.short_by_bounds(&original));
    let mut edited_vertex = original.clone();
    edited_vertex.vertices.make_mut()[1].position[1] = 1.0e6;
    assert!(!band.short_by_bounds(&edited_vertex));
    assert!(band.needs_clipping(&edited_vertex));
    let mut edited_index = original.clone();
    edited_index.indices.make_mut()[1] = 2;
    assert!(!band.short_by_bounds(&edited_index));
    assert!(band.needs_clipping(&edited_index));
    assert!(band.short_by_bounds(&original));
}

fn retained_line(ends: [f32; 2], width: f32) -> PrimitiveDraw {
    // The outlier is unreferenced until the index stream is explicitly edited.
    let vertices = [ends[0], ends[1], 1.0e6]
        .map(|y| RenderVertex::new([0., y, 0.], [1.; 4]))
        .to_vec();
    PrimitiveDraw::segments(
        vertices,
        [0, 1].repeat(RETAINED_INDEX_MINIMUM / 2),
        crate::identity_clip(),
        LineStyle::new(width, 0xffff, 1).unwrap(),
    )
    .unwrap()
}
