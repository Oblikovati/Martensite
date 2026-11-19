//! The independent normal Jacobian gives rational volume bounds without boundary quadrature.
use super::*;
const DEPTH_NUMERATOR: i128 = 3_602_879_701_896_397;
pub(super) fn bounds(region: SurfaceRegionId, sense: NormalSense) -> [f64; 2] {
    let (area, denominator, minimum, maximum) = match region.0 {
        1 => (366, 5, 80, 90),
        2 => (4, 1, 82, 86),
        _ => panic!("region={region:?}; expected O. material"),
    };
    let width = ratio(3 * DEPTH_NUMERATOR * area, (1_i128 << 55) * denominator);
    let low = correction(area, denominator, 159, 20, maximum)[0];
    let high = correction(area, denominator, 199, 25, minimum)[1];
    match sense {
        NormalSense::Outward => [(width[0] + low).next_down(), (width[1] + high).next_up()],
        NormalSense::Inward => [(width[0] - high).next_down(), (width[1] - low).next_up()],
    }
}
fn correction(
    area: i128,
    denominator: i128,
    cot_num: i128,
    cot_den: i128,
    slant: i128,
) -> [f64; 2] {
    // sin/cos alternating series at1/8 prove159/20<cot(alpha)<199/25.
    // Actual depths are -a/2^55 and a/2^54, so half their square difference is3a²/2^111.
    let numerator = 3 * DEPTH_NUMERATOR.pow(2) * cot_num * area;
    ratio(numerator, (1_i128 << 111) * cot_den * denominator * slant)
}
fn ratio(numerator: i128, denominator: i128) -> [f64; 2] {
    assert!(numerator > 0 && denominator > 0);
    let n = numerator as f64;
    let d = denominator as f64;
    [
        (n.next_down() / d.next_up()).next_down(),
        (n.next_up() / d.next_down()).next_up(),
    ]
}
#[test]
fn dyadic_depth_coefficients_and_symmetric_volume_oracle_are_independent() {
    let a = DEPTH_NUMERATOR as f64;
    assert_eq!(-a / 2_f64.powi(55), -0.1);
    assert_eq!(a / 2_f64.powi(54), 0.2);
    // Symmetric depths[-1/8,1/8] cancel the inverse-slant term exactly.
    assert_eq!(366 * 480 / (5 * 4), 183 * 48);
    for region in [SurfaceRegionId(1), SurfaceRegionId(2)] {
        let outward = bounds(region, NormalSense::Outward);
        let inward = bounds(region, NormalSense::Inward);
        assert!(inward[0] > 0. && inward[1] < outward[0] && outward[0] < outward[1]);
    }
}
