// SPDX-License-Identifier: GPL-2.0-only
//! Qualify ordinary rounded circular endpoints and a rotated ellipse carrier.
use super::*;
use martensite_tkbo::body_check::{CheckMode, ValidationStage};
use martensite_tkbo::profile_extrusion::{ArcSense, PlanarProofVersion};
use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4, PI};

const CENTER: [f64; 2] = [4., 0.];

#[test]
fn rational_reference_sewn_conic_accepts_unit_circle_unsnapped_45_degree_vertex() {
    let mut request = cylinder();
    request.profile = sector(None);
    request.selections[0].outer.uses.pop();
    let result = normal_extrude_rational_cylinder(&request, 3, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    validated(&result, FRAC_PI_4 / 2.);
    let authored_end = request.profile.vertices[1].point;
    assert_eq!(result.authored_profile().vertices[1].point, authored_end);
    let first = mapped(&result, 0);
    assert_eq!(first.count, 1);
    assert_eq!(first.parameter, WrappedConicParameterization::Symmetric);
    let [start, sweep] = first.sweep();
    assert_eq!(start, 0.);
    assert!((sweep - FRAC_PI_4).abs() <= 8. * f64::EPSILON);
}

#[test]
fn rational_reference_sewn_conic_checks_rotated_elliptic_arc_on_cone() {
    let mut request = cone();
    request.profile = sector(Some([0.25, 1.25]));
    request.selections[0].outer.uses.pop();
    request.proof.planar_proof = PlanarProofVersion::GeometricCompositeV6 {
        max_components: 64,
        max_total_subintervals: 100_000,
    };
    let result =
        normal_extrude_rational_cone_with_selection(&request, 2, strict_cone::V8_FAMILY_V3, || {
            false
        })
        .unwrap_or_else(|error| panic!("{error}"));
    validated(&result, QUARTER_RADIUS * QUARTER_RADIUS / 4.);
    let [start, sweep] = mapped(&result, 0).sweep();
    assert!((start - 0.25).abs() < 1e-13 && (sweep - 1.).abs() < 1e-13);
    assert_eq!(
        result
            .validation()
            .stage(ValidationStage::SelfIntersection)
            .mode,
        CheckMode::Exact
    );
}

#[test]
fn rational_reference_sewn_conic_keeps_the_original_long_sweep_anchors() {
    let mut request = cylinder();
    let r = QUARTER_RADIUS / 2.;
    request.profile = sector(None);
    request.profile.vertices[0].point = [CENTER[0] + r, 0.];
    request.profile.vertices[1].point = [CENTER[0], -r];
    request.profile.vertices.truncate(2);
    request.profile.curves[1] = ProfileCurve::Line {
        id: ProfileCurveId(2),
        start: ProfileVertexId(2),
        end: ProfileVertexId(1),
    };
    request.profile.curves.truncate(2);
    request.selections[0].outer.uses.truncate(2);
    let result = normal_extrude_rational_cylinder(&request, 2, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    let expected = r * r * (3. * FRAC_PI_2 + 1.) / 2.;
    validated(&result, expected);
    for index in 0..3 {
        let map = mapped(&result, index);
        assert_eq!(map.count, 3);
        assert_eq!(map.sweep(), [0., 3. * FRAC_PI_2]);
        assert_eq!(map.parameter, WrappedConicParameterization::FirstEndpoint);
        assert_eq!(
            result.history().iter().any(|record| record.result.source
                == WrappedEntitySource::AdmittedArcAnchor {
                    curve: SurfaceCurveId(1),
                    after: map
                }),
            index < 2
        );
    }
}

#[test]
fn rational_reference_sewn_conic_retains_the_closed_rotated_ellipse_seam() {
    let mut request = cylinder();
    let r = QUARTER_RADIUS / 2.;
    request.profile = sector(Some([0.25, 1.25]));
    request.profile.vertices.clear();
    request.profile.curves = vec![ProfileCurve::Ellipse {
        id: ProfileCurveId(1),
        center: CENTER,
        major_axis: [0.6 * r, 0.8 * r],
        minor_radius: r / 2.,
    }];
    request.selections[0].outer.uses.truncate(1);
    let result = normal_extrude_rational_cylinder(&request, 2, || false)
        .unwrap_or_else(|error| panic!("{error}"));
    validated(&result, PI * r * r / 2.);
    for quarter in 0..4 {
        assert!(result.history().iter().any(|record| record.result.source
            == WrappedEntitySource::ConicAnchor {
                curve: SurfaceCurveId(1),
                quarter
            }));
    }
}

fn sector(elliptic: Option<[f64; 2]>) -> SketchProfile {
    let mut profile = quarter(QUARTER_RADIUS);
    profile.vertices[2].point = CENTER;
    let (id, start, end) = (ProfileCurveId(1), ProfileVertexId(1), ProfileVertexId(2));
    profile.curves[0] = if let Some(angles) = elliptic {
        let [major, minor] = [QUARTER_RADIUS, QUARTER_RADIUS / 2.];
        let major_axis = [0.6 * major, 0.8 * major];
        let ends = angles.map(|angle| ellipse_point(major, minor, angle));
        set_ends(&mut profile, ends);
        ProfileCurve::EllipticArc {
            id,
            start,
            end,
            center: CENTER,
            major_axis,
            minor_radius: minor,
            sense: ArcSense::CounterClockwise,
        }
    } else {
        set_ends(
            &mut profile,
            [[5., 0.], [4. + FRAC_1_SQRT_2, FRAC_1_SQRT_2]],
        );
        ProfileCurve::Arc {
            id,
            start,
            end,
            center: CENTER,
            sense: ArcSense::CounterClockwise,
        }
    };
    profile
}

fn set_ends(profile: &mut SketchProfile, ends: [[f64; 2]; 2]) {
    for (vertex, point) in profile.vertices.iter_mut().zip(ends) {
        vertex.point = point;
    }
}

fn ellipse_point(major: f64, minor: f64, angle: f64) -> [f64; 2] {
    let [along, across] = [major * angle.cos(), minor * angle.sin()];
    [
        CENTER[0] + 0.6 * along - 0.8 * across,
        0.8 * along + 0.6 * across,
    ]
}

fn mapped(result: &RationalWrappedResult, expected: u8) -> WrappedAdmittedArcMap {
    result
        .history()
        .iter()
        .find_map(|record| match record.result.source {
            WrappedEntitySource::AdmittedArcSplitVertex {
                curve: SurfaceCurveId(1),
                map,
                ..
            } if map.index == expected => Some(map),
            _ => None,
        })
        .unwrap()
}

fn validated(result: &RationalWrappedResult, expected_area: f64) {
    let area = result.authored_area();
    assert!(
        area.lower <= expected_area && expected_area <= area.upper,
        "{area:?}"
    );
    let transport = result.area_transport_error().upper;
    assert!(transport > 0. && transport < 1e-12, "{transport}");
    assert!(result.validation().is_valid());
    assert_eq!(
        result.export_native().unwrap().entities.len(),
        result.history().len()
    );
}
