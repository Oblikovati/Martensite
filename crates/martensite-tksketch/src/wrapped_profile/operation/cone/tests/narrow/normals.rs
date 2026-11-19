//! Sampled normal expectations come from authored tangents and the cone differential.
use super::super::completed::normals as shared;
use super::*;
use martensite_tkmath::gp::Vector3;
pub(super) fn check(
    request: &WrappedConeNormalRequest,
    result: &WrappedConeNormalResult,
    detached: &DetachedWrappedBodies,
) {
    let mut counts = [0_usize; 2];
    for face in shared::faces(&detached.bodies) {
        let key = shared::key(detached, &face);
        if shared::cap(key.role) {
            check_cap(request, result, &face, key);
            counts[0] += 1;
        } else {
            check_side(request, &face, key);
            counts[1] += 1;
        }
    }
    assert_face_counts(request, result, counts);
}
fn check_cap(
    request: &WrappedConeNormalRequest,
    result: &WrappedConeNormalResult,
    face: &Shape,
    key: WrappedHistoryKey,
) {
    let region = super::super::completed::occurrences::owner(result, key);
    let (point, reference) = match region.0 {
        1 => ([87.5, 0.], 88.),
        2 => ([81., 23.], martensite_cmath::hypot(80., 22.)),
        _ => panic!("unexpected cap owner={region:?}"),
    };
    let alpha = request.support.semi_angle;
    let theta = martensite_cmath::atan2(point[1], point[0]) / martensite_cmath::sin(alpha);
    let v = martensite_cmath::hypot(point[0], point[1]) - reference;
    let actual = shared::normal(face, &shared::derivatives(face, theta, v));
    let expected = cone_normal(theta, alpha);
    let role = shared::physical_cap_role(key.role, request.support.normal);
    let sign = cap_sign(role);
    assert!(
        sign * actual.dot(expected) > 0.,
        "cap={key:?}, normal={actual:?}"
    );
}
fn cap_sign(role: HistoryRole) -> f64 {
    match role {
        HistoryRole::FirstDepthFace => -1.,
        HistoryRole::LastDepthFace => 1.,
        other => panic!("expected physical cap role, actual={other:?}"),
    }
}
fn cone_normal(theta: f64, alpha: f64) -> Vector3 {
    let c = martensite_cmath::cos(alpha);
    Vector3::new(
        c * martensite_cmath::cos(theta),
        c * martensite_cmath::sin(theta),
        -martensite_cmath::sin(alpha),
    )
}
fn check_side(request: &WrappedConeNormalRequest, face: &Shape, key: WrappedHistoryKey) {
    assert_eq!(key.role, HistoryRole::SideFace);
    let expected = side_expectation(request, key);
    let actual = shared::normal(face, &shared::derivatives(face, 0.5, 0.5));
    assert!(
        actual.dot(expected) > 0.,
        "sampled side={key:?}, normal={actual:?}, expected={expected:?}"
    );
}
fn side_expectation(request: &WrappedConeNormalRequest, key: WrappedHistoryKey) -> Vector3 {
    let WrappedEntitySource::Curve { curve, span } = key.source else {
        panic!("side={key:?}");
    };
    let source = request
        .profile
        .curves
        .iter()
        .find(|source| oracles::id(source).0 == curve.0)
        .unwrap();
    let poles = oracles::controls(&request.profile, source);
    let t = (span.index as f64 + 0.5) / 2_f64.powi(i32::from(span.level));
    let point = evaluate(poles.clone(), t);
    let tangent = tangent(&poles, t, span.reversed);
    let depth = physical_mid_depth(request);
    push_normal(point, tangent, request.support.semi_angle, depth)
}
fn physical_mid_depth(request: &WrappedConeNormalRequest) -> f64 {
    let depth = (request.depths[0] + request.depths[1]) / 2.;
    match request.support.normal {
        NormalSense::Outward => depth,
        NormalSense::Inward => -depth,
    }
}
fn evaluate(mut poles: Vec<[f64; 2]>, t: f64) -> [f64; 2] {
    while poles.len() > 1 {
        poles = poles
            .windows(2)
            .map(|pair| std::array::from_fn(|i| (1. - t) * pair[0][i] + t * pair[1][i]))
            .collect();
    }
    poles[0]
}
fn tangent(poles: &[[f64; 2]], t: f64, reversed: bool) -> [f64; 2] {
    let factor = (poles.len() - 1) as f64 * if reversed { -1. } else { 1. };
    let derivative = poles
        .windows(2)
        .map(|pair| std::array::from_fn(|i| factor * (pair[1][i] - pair[0][i])))
        .collect();
    evaluate(derivative, t)
}
fn push_normal([x, y]: [f64; 2], [tx, ty]: [f64; 2], alpha: f64, depth: f64) -> Vector3 {
    let l = martensite_cmath::hypot(x, y);
    let (s, c) = (martensite_cmath::sin(alpha), martensite_cmath::cos(alpha));
    let theta = martensite_cmath::atan2(y, x) / s;
    let (ct, st) = (martensite_cmath::cos(theta), martensite_cmath::sin(theta));
    let radial = (x * ty - y * tx) / l;
    let phase = (-x * tx - y * ty) / (l * l);
    let r = l * s + depth * c;
    Vector3::new(
        s * ct * radial - r * st * phase / s,
        s * st * radial + r * ct * phase / s,
        c * radial,
    )
}

fn assert_face_counts(
    request: &WrappedConeNormalRequest,
    result: &WrappedConeNormalResult,
    counts: [usize; 2],
) {
    assert_eq!(counts[0], request.selections.len() * 2);
    assert_eq!(
        counts[1],
        result
            .reports()
            .values()
            .map(|r| r.coverage.faces - 2)
            .sum()
    );
}
