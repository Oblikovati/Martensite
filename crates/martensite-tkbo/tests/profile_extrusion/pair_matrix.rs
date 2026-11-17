// SPDX-License-Identifier: GPL-2.0-only
//! Issue #449: a complete pair matrix must remain practical without dropping proof domains.
use super::fixtures::*;
use martensite_tkbo::profile_extrusion::*;

#[test]
fn two_subdivided_rectangles_retain_all_curves_regions_and_closed_form_areas() {
    let profile = subdivided_rectangles();
    let set = build_planar_regions(&profile, resolution()).unwrap();
    assert_eq!(profile.curves.len(), 56);
    assert_eq!(set.regions().len(), 2);
    assert_eq!(set.curve_edges().len(), 56);
    for region in set.regions() {
        assert_eq!(region.outer().uses.len(), 28);
        assert!(region.holes().is_empty());
        assert!(region.area().encloses(49.));
        assert_eq!(region.validation().tolerance, StageStatus::Passed);
    }
    assert!(
        set.curve_edges()
            .iter()
            .all(|entry| entry.regions.len() == 1)
    );
}
fn subdivided_rectangles() -> SketchProfile {
    let mut fixture = SketchFixture::new();
    for rectangle in 0..2_u64 {
        let points = rectangle_boundary(rectangle as f64 * 10.);
        for (index, point) in points.into_iter().enumerate() {
            let id = rectangle * 28 + index as u64;
            fixture.vertex(id, point);
            fixture.line(id, id, rectangle * 28 + (index as u64 + 1) % 28);
        }
    }
    fixture.build()
}
fn rectangle_boundary(offset: f64) -> Vec<[f64; 2]> {
    (0..4)
        .flat_map(|side| (0..7).map(move |index| rectangle_point(side, index as f64, offset)))
        .collect()
}
fn rectangle_point(side: usize, index: f64, offset: f64) -> [f64; 2] {
    match side {
        0 => [offset + index, 0.],
        1 => [offset + 7., index],
        2 => [offset + 7. - index, 7.],
        3 => [offset, 7. - index],
        _ => unreachable!(),
    }
}
