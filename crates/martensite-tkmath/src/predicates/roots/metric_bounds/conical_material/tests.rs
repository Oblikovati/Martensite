// SPDX-License-Identifier: GPL-2.0-only
use super::*;
fn query<'a>(spans: &'a [ConicalMaterialSpan<'a>]) -> ConicalMaterialQuery<'a> {
    let (sine, cosine) = (martensite_cmath::sin(0.7), martensite_cmath::cos(0.7));
    ConicalMaterialQuery {
        spans,
        carriers: [-0.1, 0.2].map(|depth| ConicalTrimCarrier {
            radius: 4. * sine + depth * cosine,
            axial_origin: 4. * cosine - depth * sine,
            semi_angle: 0.7,
        }),
        reference_slant: 4.,
        chart_origin: 0.,
        depths: [-0.1, 0.2],
        spatial_limit: 0.1,
        options: ConeClearanceOptions::default(),
    }
}
fn fixture() -> ([[f64; 2]; 2], [[f64; 2]; 2]) {
    let phi = 0.05 * martensite_cmath::sin(0.7);
    let x = 4. * martensite_cmath::cos(phi);
    let y = 4. * martensite_cmath::sin(phi);
    ([[x, -y], [x, y]], [[-0.05, 0.], [0.05, 0.]])
}
fn span<'a>(original: &'a [[f64; 2]], uv: &'a [[f64; 2]]) -> ConicalMaterialSpan<'a> {
    ConicalMaterialSpan {
        original,
        uv,
        interval: CylindricalProfileInterval::new(0, 0).unwrap(),
    }
}
#[test]
fn direct_development_bounds_independent_midpoint_chord_error() {
    let (original, uv) = fixture();
    let spans = [span(&original, &uv)];
    let evidence = certify_conical_material(query(&spans), || false).unwrap();
    let midpoint_error = 4. - original[0][0];
    assert!(evidence.spatial_error() >= midpoint_error);
    assert!(evidence.spatial_error() < 0.1);
    assert!(evidence.operations() > 0);
    assert!(evidence.matches(&query(&spans)));
}
#[test]
fn exact_original_restriction_is_used_instead_of_rounded_child() {
    let (original, uv) = fixture();
    let spans = [span(&original, &uv)];
    let evidence = certify_conical_material(query(&spans), || false).unwrap();
    let mut changed = spans;
    changed[0].interval = CylindricalProfileInterval::new(1, 1).unwrap();
    assert!(!evidence.matches(&query(&changed)));
    let mut changed_query = query(&changed);
    changed_query.spatial_limit = 1.;
    let proof = certify_conical_material(changed_query, || false).unwrap();
    assert!(proof.spatial_error() > evidence.spatial_error());
}
#[test]
fn compact_domain_near_apex_is_refused_over_full_depth_interval() {
    let (original, mut uv) = fixture();
    uv[0][1] = -3.99;
    uv[1][1] = -3.99;
    let spans = [span(&original, &uv)];
    assert!(certify_conical_material(query(&spans), || false).is_err());
}
#[test]
fn actual_cap_datum_error_is_included_for_every_filled_point() {
    let (original, uv) = fixture();
    let spans = [span(&original, &uv)];
    let mut input = query(&spans);
    input.carriers[0].axial_origin += 0.025;
    let evidence = certify_conical_material(input, || false).unwrap();
    assert!(evidence.spatial_error() >= 0.025);
    assert!(!evidence.matches(&query(&spans)));
}
#[test]
fn positive_x_admission_checks_original_and_actual_developed_domains() {
    let (mut original, uv) = fixture();
    original[0][0] = -1.;
    let spans = [span(&original, &uv)];
    assert!(certify_conical_material(query(&spans), || false).is_err());
    let (original, mut uv) = fixture();
    uv[1][0] = 4.;
    let spans = [span(&original, &uv)];
    assert!(certify_conical_material(query(&spans), || false).is_err());
}
#[test]
fn cancellation_and_budget_exhaustion_cannot_create_evidence() {
    let (original, uv) = fixture();
    let spans = [span(&original, &uv)];
    assert!(certify_conical_material(query(&spans), || true).is_err());
    let mut input = query(&spans);
    input.options.max_operations = 1;
    let refusal = certify_conical_material(input, || false).unwrap_err();
    assert!(refusal.operations <= 1);
}

#[test]
fn original_angular_margin_refuses_a_near_period_normal_map() {
    let original = near_period_original();
    let uv = [[-3., 0.], [3., 0.]];
    let spans = [span(&original, &uv)];
    let mut input = query(&spans);
    input.spatial_limit = 10.;
    input.carriers = shallow_carriers();
    let refusal = certify_conical_material(input, || false).unwrap_err();
    assert!(refusal.operations > 0);
    assert!(matches!(
        refusal.cause,
        CylindricalProfileFailure::Value {
            role: "original filled-material angular width plus certified margin below 2pi",
            ..
        }
    ));
}

fn shallow_carriers() -> [ConicalTrimCarrier; 2] {
    [-0.1, 0.2].map(|depth| ConicalTrimCarrier {
        radius: 4. * martensite_cmath::sin(0.1) + depth * martensite_cmath::cos(0.1),
        axial_origin: 4. * martensite_cmath::cos(0.1) - depth * martensite_cmath::sin(0.1),
        semi_angle: 0.1,
    })
}

fn near_period_original() -> [[f64; 2]; 2] {
    [-0.35, 0.35].map(|phase| {
        [
            4. * martensite_cmath::cos(phase),
            4. * martensite_cmath::sin(phase),
        ]
    })
}
