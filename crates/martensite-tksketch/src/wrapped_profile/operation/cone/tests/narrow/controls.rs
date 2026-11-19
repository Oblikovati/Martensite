//! Fixture topology, exact areas and source-preserving quadrant changes precede cone proofs.
use super::*;
use fixture::{Quadrant, request};
#[test]
fn polynomial_punctuation_has_exact_green_area_and_explicit_counter_material() {
    let request = request(NormalSense::Outward, Quadrant::PositiveX);
    let selections = &request.selections;
    assert_eq!(selections.len(), 2);
    assert_eq!(selections[0].region, SurfaceRegionId(1));
    assert_eq!(selections[1].region, SurfaceRegionId(2));
    assert_eq!(selections[0].holes.len(), 1);
    assert!(selections[1].holes.is_empty());
    let area = |contour| oracles::area_times_480(&request.profile, contour);
    assert_eq!(area(&selections[0].outer), 1098 * 96);
    assert_eq!(area(&selections[0].holes[0]), -732 * 96);
    assert_eq!(area(&selections[1].outer), 4 * 480);
    assert_eq!(
        area(&selections[0].outer) + area(&selections[0].holes[0]),
        366 * 96
    );
}
#[test]
fn polynomial_punctuation_has_twelve_unique_source_owners_and_exact_closed_uses() {
    for quadrant in [
        Quadrant::PositiveX,
        Quadrant::PositiveY,
        Quadrant::NegativeX,
    ] {
        assert_source_owners(&request(NormalSense::Outward, quadrant));
    }
}
fn assert_source_owners(request: &WrappedConeNormalRequest) {
    let ids: std::collections::BTreeSet<_> =
        request.profile.curves.iter().map(oracles::id).collect();
    assert_eq!(ids, (1..=12).map(ProfileCurveId).collect());
    assert_eq!(request.profile.vertices.len(), 12);
    let contours = request
        .selections
        .iter()
        .flat_map(|selection| std::iter::once(&selection.outer).chain(&selection.holes));
    let mut selected = std::collections::BTreeSet::new();
    for contour in contours {
        assert_closed(&request.profile, contour);
        for source in &contour.uses {
            assert!(selected.insert(source.curve));
        }
    }
    assert_eq!(selected, ids);
    assert_endpoint_bindings(&request.profile);
}
fn assert_closed(profile: &SketchProfile, contour: &ProfileLoop) {
    let directed: Vec<_> = contour
        .uses
        .iter()
        .map(|source| directed_controls(profile, source))
        .collect();
    for (first, next) in directed.iter().zip(directed.iter().cycle().skip(1)) {
        assert_eq!(
            first.last().map(|p| p.map(f64::to_bits)),
            next.first().map(|p| p.map(f64::to_bits))
        );
        assert_ne!(first.first(), first.last());
    }
}
fn directed_controls(profile: &SketchProfile, source: &LoopUse) -> Vec<[f64; 2]> {
    let curve = profile
        .curves
        .iter()
        .find(|curve| oracles::id(curve) == source.curve)
        .unwrap();
    let mut poles = oracles::controls(profile, curve);
    if source.reversed {
        poles.reverse();
    }
    poles
}
fn assert_endpoint_bindings(profile: &SketchProfile) {
    for curve in &profile.curves {
        let (start, end) = match curve {
            ProfileCurve::Line { start, end, .. } | ProfileCurve::Spline { start, end, .. } => {
                (*start, *end)
            }
            _ => panic!("expected owned polynomial O. curve"),
        };
        let poles = oracles::controls(profile, curve);
        for (id, endpoint) in [(start, poles[0]), (end, *poles.last().unwrap())] {
            assert_vertex_bits(profile, id, endpoint, oracles::id(curve));
        }
    }
}
fn assert_vertex_bits(
    profile: &SketchProfile,
    id: ProfileVertexId,
    endpoint: [f64; 2],
    curve: ProfileCurveId,
) {
    let vertex = profile
        .vertices
        .iter()
        .find(|vertex| vertex.id == id)
        .unwrap();
    assert_eq!(
        endpoint.map(f64::to_bits),
        vertex.point.map(f64::to_bits),
        "curve={curve:?}, vertex={id:?}"
    );
}
#[test]
fn exact_quadrant_rotations_preserve_owned_ids_areas_and_depth_sense() {
    let original = request(NormalSense::Outward, Quadrant::PositiveX);
    for quadrant in [Quadrant::PositiveY, Quadrant::NegativeX] {
        let rotated = request(NormalSense::Inward, quadrant);
        assert_eq!(rotated.selections, original.selections);
        assert_eq!(rotated.depths, [-0.1, 0.2]);
        assert_eq!(rotated.support.normal, NormalSense::Inward);
        assert_eq!(rotated.support.semi_angle, 0.125);
        assert_rotated_vertices(&original.profile, &rotated.profile, quadrant);
        for selection in &rotated.selections {
            assert_eq!(
                oracles::area_times_480(&rotated.profile, &selection.outer),
                oracles::area_times_480(&original.profile, &selection.outer)
            );
        }
    }
}
fn assert_rotated_vertices(original: &SketchProfile, rotated: &SketchProfile, quadrant: Quadrant) {
    for (before, after) in original.vertices.iter().zip(&rotated.vertices) {
        assert_eq!(before.id, after.id);
        let [x, y] = before.point;
        let expected = match quadrant {
            Quadrant::PositiveY => [-y, x],
            Quadrant::NegativeX => [-x, -y],
            Quadrant::PositiveX => [x, y],
        };
        let expected = expected.map(|value| if value == 0. { 0. } else { value });
        assert_eq!(after.point.map(f64::to_bits), expected.map(f64::to_bits));
    }
}
#[test]
fn captured_narrow_sources_keep_both_regions_and_caller_relative_physical_depths() {
    for (sense, expected) in [
        (NormalSense::Outward, [-0.1, 0.2]),
        (NormalSense::Inward, [-0.2, 0.1]),
    ] {
        let mut request = request(sense, Quadrant::PositiveX);
        let captured = input::capture(&request, &mut || false).unwrap();
        assert_eq!(captured.originals.len(), 2);
        assert_eq!(captured.originals[0].loops.len(), 2);
        assert_eq!(captured.originals[1].loops.len(), 1);
        for original in &captured.originals {
            assert_eq!(original.depths, expected);
        }
        request.profile.curves.clear();
        assert_eq!(captured.originals[0].loops[0][0].poles[1], [88., 9.]);
        assert_eq!(
            captured.originals[1].loops[0][0].poles,
            [[80., 22.], [82., 22.]]
        );
    }
}
