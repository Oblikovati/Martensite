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
    assert_eq!(draws[0].indices, [0, 1]);
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
    for vertex in &mut draw.vertices {
        vertex.position[0] = 1000.;
    }
    let mut draws = vec![draw];
    clip_far_lines(&mut draws, [64, 64]);
    assert!(draws[0].indices.is_empty());
    let mut behind = vertical_line(10.);
    behind.vertices[1].weight = -1.;
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
    let replaced = draws[0].with_replacement_geometry(draws[0].vertices.clone(), vec![0, 1]);
    assert_eq!(replaced.guard_band_source, None);
}
