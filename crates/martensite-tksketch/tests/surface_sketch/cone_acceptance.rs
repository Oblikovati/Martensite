//! Independent rational controls for conical normal extrusion (#417).
mod bindings;
mod geometry;
mod ownership;
mod refusals;
pub(crate) mod replay;

use geometry::{Expected, assert_cone};
use martensite_cmath::atan2;
use martensite_tksketch::surface_sketch::*;

pub(super) fn sector() -> ConeSector {
    ConeSector {
        support: ConeSupport {
            apex: [0.; 3],
            axis: [0., 0., 1.],
            radial: [1., 0., 0.],
            semi_angle: atan2(3., 4.),
            chart_origin: 0.2,
            normal: NormalSense::Outward,
        },
        region: SurfaceRegionId(71),
        slant: [8., 12.],
        developed_angle: [0., 0.42],
        corners: [19, 3, 81, 7].map(SurfaceVertexId),
        boundaries: [62, 4, 91, 8].map(SurfaceCurveId),
    }
}

fn construct(request: &ConeSector, depths: [f64; 2]) -> NormalExtrusionResult {
    normal_extrude_cone(request, depths, super::refusals::options()).unwrap()
}

#[test]
fn rational_cone_areas_corners_and_volume_cover_both_normal_senses() {
    for (normal, expected) in [
        (NormalSense::Outward, Expected::outward()),
        (NormalSense::Inward, Expected::inward()),
    ] {
        let mut request = sector();
        request.support.normal = normal;
        assert_cone(
            &construct(&request, [-1., 2.]),
            &request,
            [-1., 2.],
            expected,
        );
    }
}

#[test]
fn seam_period_and_oblique_frame_preserve_authored_bindings() {
    let mut request = sector();
    let keys = construct(&request, [-1., 2.]).history().to_vec();
    for chart in [-0.3, -0.3 + std::f64::consts::TAU] {
        request.support.chart_origin = chart;
        let result = construct(&request, [-1., 2.]);
        assert_eq!(result.history(), keys);
        assert_cone(&result, &request, [-1., 2.], Expected::outward());
    }
    request.support.apex = [17., -23., 31.];
    request.support.axis = [1. / 14_f64.sqrt(), 2. / 14_f64.sqrt(), 3. / 14_f64.sqrt()];
    request.support.radial = [2. / 5_f64.sqrt(), -1. / 5_f64.sqrt(), 0.];
    let result = construct(&request, [-1., 2.]);
    assert_eq!(result.history(), keys);
    assert_cone(&result, &request, [-1., 2.], Expected::outward());
}

#[test]
fn narrow_major_and_near_full_sectors_preserve_all_six_faces() {
    let mut request = sector();
    let keys = construct(&request, [-1., 2.]).history().to_vec();
    for angle in [
        0.01,
        std::f64::consts::PI,
        1.5 * std::f64::consts::PI,
        std::f64::consts::TAU - 1e-4,
    ] {
        request.developed_angle[1] = angle * 0.6;
        let result = construct(&request, [-1., 2.]);
        assert_eq!(result.history(), keys);
        assert_cone(
            &result,
            &request,
            [-1., 2.],
            Expected::outward().angular_scale(angle / 0.7),
        );
    }
}

#[test]
fn small_cone_uses_relative_measure_checks() {
    let mut request = sector();
    request.slant = [0.008, 0.012];
    let result = construct(&request, [-0.001, 0.002]);
    assert_cone(
        &result,
        &request,
        [-0.001, 0.002],
        Expected::outward().linear_scale(1e-3),
    );
}

#[test]
fn far_outward_offset_crosses_original_apex_plane_without_axis_contact() {
    let request = sector();
    // Independent section bounds are r=[12.8,18.4], z=[-2,3.6].
    let expected = Expected {
        volume: 174.72,
        areas: [39.2, 48.16, 40.32, 16., 47.04, 16.],
    };
    assert_cone(
        &construct(&request, [10., 14.]),
        &request,
        [10., 14.],
        expected,
    );
}

#[test]
fn normal_reversal_preserves_body_and_swaps_requested_depth_roles() {
    let mut request = sector();
    let out = construct(&request, [-1., 2.]).export_brep().unwrap();
    request.support.normal = NormalSense::Inward;
    let result = construct(&request, [-2., 1.]);
    let mut expected = Expected::outward();
    expected.areas.swap(0, 1);
    assert_cone(&result, &request, [-2., 1.], expected);
    let inward = result.export_brep().unwrap();
    for id in request.corners {
        geometry::assert_swapped_depth_vertices(&out, &inward, id);
    }
}
