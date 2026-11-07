//! Same-kind cone binding corruption must fail independently of valid native volume.
use super::*;
use martensite_cmath::atan2;

fn admitted() -> cone_admission::AdmittedCone {
    let cylinder = semantic_tests::input();
    let support = ConeSupport {
        apex: [0.; 3],
        axis: [0., 0., 1.],
        radial: [1., 0., 0.],
        semi_angle: atan2(3., 4.),
        chart_origin: -0.3,
        normal: NormalSense::Outward,
    };
    let sector = ConeSector {
        support,
        region: cylinder.rectangle.region,
        slant: [8., 12.],
        developed_angle: [0., 0.42],
        corners: cylinder.rectangle.corners,
        boundaries: cylinder.rectangle.boundaries,
    };
    cone_admission::admit(&sector, [-1., 2.], cylinder.options).unwrap()
}
fn depth_faces(bindings: &history::Bindings) -> [usize; 2] {
    [HistoryRole::FirstDepthFace, HistoryRole::LastDepthFace].map(|role| {
        bindings
            .iter()
            .position(|(key, _)| key.role == role)
            .unwrap()
    })
}
fn slant_faces(
    admitted: &cone_admission::AdmittedCone,
    bindings: &history::Bindings,
) -> [usize; 2] {
    [0, 2].map(|index| {
        bindings
            .iter()
            .position(|(key, _)| {
                key.role == HistoryRole::SideFace
                    && key.source == HistorySource::Curve(admitted.sector.boundaries[index])
            })
            .unwrap()
    })
}
#[test]
fn swapped_depth_and_slant_cones_fail_semantic_validation() {
    let admitted = admitted();
    let (section, mut maker, solid) = construction::construct(&admitted.sweep).unwrap();
    let bindings = history::collect(&admitted.sweep, &section, &mut maker, &solid).unwrap();
    assert!(cone_semantic::validate(&admitted, &bindings).is_ok());
    assert!(volume::measure(&solid, &admitted.sweep).is_ok());
    for [first, last] in [depth_faces(&bindings), slant_faces(&admitted, &bindings)] {
        let mut swapped = bindings.clone();
        swapped[first].1 = bindings[last].1.clone();
        swapped[last].1 = bindings[first].1.clone();
        assert!(history::validate_membership(&solid, &swapped).is_ok());
        assert!(cone_semantic::validate(&admitted, &swapped).is_err());
    }
}
