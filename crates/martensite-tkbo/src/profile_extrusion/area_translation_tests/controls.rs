use super::super::*;
use super::{fixtures::*, issue_resolution};
use std::f64::consts::PI;
fn require_area(profile: &SketchProfile, expected: f64) {
    let regions = build_planar_regions(profile, issue_resolution()).unwrap();
    assert_eq!(regions.regions().len(), 1);
    assert!(
        regions.regions()[0].area().encloses(expected),
        "{:?} vs {expected}",
        regions.regions()[0].area()
    );
    require_lineage(profile, &regions);
    assert!(
        regions
            .curve_edges()
            .iter()
            .all(|binding| binding.regions.len() == 1)
    );
}
fn require_lineage(profile: &SketchProfile, regions: &PlanarRegionSet) {
    let authored: Vec<_> = profile.curves.iter().map(ProfileCurve::id).collect();
    let mapped: Vec<_> = regions
        .curve_edges()
        .iter()
        .map(|binding| binding.curve)
        .collect();
    assert_eq!(mapped, authored);
    assert!(
        regions
            .curve_edges()
            .iter()
            .all(|binding| !binding.regions.is_empty())
    );
}
fn require_ring(origin: f64, outer: bool, hole: bool) {
    let profile = ring(origin, outer, hole);
    let regions = build_planar_regions(&profile, issue_resolution()).unwrap();
    assert_eq!(regions.regions().len(), 2);
    let ring = regions
        .regions()
        .iter()
        .find(|region| region.holes().len() == 1)
        .unwrap();
    let disc = regions
        .regions()
        .iter()
        .find(|region| region.holes().is_empty())
        .unwrap();
    assert!(ring.area().encloses(15. / 16.));
    assert!(disc.area().encloses(1. / 16.));
    require_lineage(&profile, &regions);
}
#[test]
fn origin_and_translated_dyadic_rectangles_preserve_area_and_lineage() {
    let small = 671_089.0 / 67_108_864.0;
    for origin in [0., 1e8, -1e8] {
        for width in [1., small, 1. / 1_048_576.] {
            require_rectangle_windings(origin, width);
        }
    }
}
#[test]
fn translated_holes_keep_the_same_area_for_both_loop_orientations() {
    for origin in [0., 1e8, -1e8] {
        require_ring_windings(origin);
    }
}
#[test]
fn translated_circle_and_ellipse_carriers_keep_analytic_area() {
    for origin in [0., 16., -16.] {
        require_area(&whole_conic(origin, false), PI);
        require_area(&whole_conic(origin, true), 2. * PI);
    }
}
#[test]
fn translated_conic_chords_keep_area_when_traversal_reverses() {
    for origin in [0., 16., -16.] {
        for reverse in [false, true] {
            require_area(&half_conic(origin, false, reverse), 0.5 * PI);
            require_area(&half_conic(origin, true, reverse), PI);
        }
    }
}
#[test]
fn empty_profile_admission_keeps_its_original_decline() {
    assert_eq!(
        build_planar_regions(&empty_profile(), issue_resolution())
            .unwrap_err()
            .stage,
        OperationStage::Admission
    );
}
#[test]
fn exactly_degenerate_rectangle_still_declines() {
    for origin in [0., 1e8] {
        assert!(
            build_planar_regions(&rectangle(origin, [1., 0.], false), issue_resolution()).is_err()
        );
    }
}
#[test]
fn origin_centered_profiles_keep_accepted_behavior() {
    for reverse in [false, true] {
        for width in [1., 671_089.0 / 67_108_864.0, 1. / 1_048_576.] {
            require_area(&rectangle(0., [width; 2], reverse), width * width);
        }
        for hole in [false, true] {
            require_ring(0., reverse, hole);
        }
        require_area(&half_conic(0., false, reverse), 0.5 * PI);
        require_area(&half_conic(0., true, reverse), PI);
    }
    require_area(&whole_conic(0., false), PI);
    require_area(&whole_conic(0., true), 2. * PI);
}

#[test]
fn empty_closed_loop_retains_zero_closed_form() {
    let profile = rectangle(0., [1.; 2], false);
    let admitted = super::super::admission::admit_profile(&profile, issue_resolution()).unwrap();
    let empty = ProfileLoop { uses: Vec::new() };
    assert_eq!(
        super::super::loop_geometry::signed_area(&admitted, &empty),
        Some(0.)
    );
}

fn require_rectangle_windings(origin: f64, width: f64) {
    for reverse in [false, true] {
        require_area(&rectangle(origin, [width; 2], reverse), width * width);
    }
}

fn require_ring_windings(origin: f64) {
    for outer in [false, true] {
        for hole in [false, true] {
            require_ring(origin, outer, hole);
        }
    }
}
