//! One-sided physical depths preserve the signed normal-map volume correction.
use super::*;

#[test]
fn rational_reference_one_sided_cylinder_uses_caller_normal_sense() {
    let mut request = cylinder();
    request.profile = quarter(QUARTER_RADIUS);
    request.selections[0].outer.uses.pop();
    request.depths = [0., 0.1];
    let area = std::f64::consts::FRAC_PI_4 * QUARTER_RADIUS * QUARTER_RADIUS;
    for (normal, mid_radius_ratio) in [(NormalSense::Outward, 1.005), (NormalSense::Inward, 0.995)]
    {
        request.support.normal = normal;
        let result = normal_extrude_rational_cylinder(&request, 3, || false)
            .unwrap_or_else(|error| panic!("{error}"));
        check(&result, area * 0.1 * mid_radius_ratio);
    }
}

#[test]
fn rational_reference_one_sided_cone_bounds_the_authored_inverse_slant_correction() {
    let mut request = cone();
    request.proof.planar_proof =
        martensite_tkbo::profile_extrusion::PlanarProofVersion::GeometricCompositeV6 {
            max_components: 64,
            max_total_subintervals: 100_000,
        };
    request.profile = quarter(QUARTER_RADIUS);
    request.selections[0].outer.uses.pop();
    request.depths = [0., 0.1];
    // Integrate the exact sector independently by first integrating 1/slant in y,
    // then using x = 4 + r cos(theta) over the positive quarter.
    let expected = [
        (NormalSense::Outward, 0.000_311_319_118_081_034_5),
        (NormalSense::Inward, 0.000_302_273_197_073_222),
    ];
    for (normal, volume) in expected {
        request.support.normal = normal;
        let result = normal_extrude_rational_cone(&request, 3, || false)
            .unwrap_or_else(|error| panic!("{error}"));
        check(&result, volume);
        let bounds = result.authored_volume();
        assert!(bounds.upper - bounds.lower <= request.proof.authored_volume.enclosure_width);
    }
}

fn check(result: &RationalWrappedResult, expected: f64) {
    let volume = result.authored_volume();
    assert!(
        volume.lower <= expected && expected <= volume.upper,
        "{volume:?} excludes {expected}"
    );
    assert!(result.validation().is_valid());
    assert_original_quarter_history(result);
}
