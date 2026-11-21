//! Swapped semantic bindings must fail while the native body and volume remain valid.
use super::*;
use martensite_tkbo::profile_extrusion::{DisplayTolerance, Resolution, ToleranceCap};

fn support() -> CylinderSupport {
    CylinderSupport {
        origin: [0.; 3],
        axis: [0., 0., 1.],
        radial: [1., 0., 0.],
        radius: 10.,
        chart_origin: -0.3,
        normal: NormalSense::Outward,
    }
}
fn rectangle() -> CylinderRectangle {
    CylinderRectangle {
        support: support(),
        region: SurfaceRegionId(1),
        s: [0., 7.],
        z: [0., 4.],
        corners: [1, 2, 3, 4].map(SurfaceVertexId),
        boundaries: [1, 2, 3, 4].map(SurfaceCurveId),
    }
}
pub(super) fn input() -> admission::Admitted {
    let display = DisplayTolerance {
        chordal: 1e-3,
        angular: 0.1,
    };
    let resolution = Resolution::new(1e-6, 1e-7, 1e-9, display, ToleranceCap::Refuse(1e-5));
    let options = NormalExtrusionOptions {
        resolution,
        volume_relative_error: 1e-9,
    };
    admission::admit(&rectangle(), [-1., 2.], options).unwrap()
}
fn swapped_images(bindings: &history::Bindings, roles: [HistoryRole; 2]) -> history::Bindings {
    let first = bindings
        .iter()
        .position(|(key, _)| key.role == roles[0])
        .unwrap();
    let second = bindings
        .iter()
        .position(|(key, _)| key.role == roles[1])
        .unwrap();
    let mut swapped = bindings.clone();
    swapped[first].1 = bindings[second].1.clone();
    swapped[second].1 = bindings[first].1.clone();
    swapped
}
#[test]
fn swapped_depth_faces_and_vertices_fail_even_with_valid_body_and_volume() {
    let admitted = input();
    let sweep = cylinder_sweep::prepare(&admitted);
    let (section, mut maker, solid) = construction::construct(&sweep).unwrap();
    let bindings = history::collect(&sweep, &section, &mut maker, &solid).unwrap();
    let semantic_check = |images: &history::Bindings| semantic::validate(&admitted, images);
    assert!(validation::check_body(&solid, &bindings, &sweep, &semantic_check).is_ok());
    assert!(volume::measure(&solid, &sweep).is_ok());
    for roles in [
        [HistoryRole::FirstDepthFace, HistoryRole::LastDepthFace],
        [HistoryRole::FirstDepthVertex, HistoryRole::LastDepthVertex],
    ] {
        let swapped = swapped_images(&bindings, roles);
        assert!(history::validate_membership(&solid, &swapped).is_ok());
        assert!(semantic::validate(&admitted, &swapped).is_err());
    }
}
