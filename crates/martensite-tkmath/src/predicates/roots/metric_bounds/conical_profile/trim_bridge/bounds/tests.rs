// SPDX-License-Identifier: GPL-2.0-only
use super::*;

#[test]
fn family_position_bound_uses_arc_length_without_changing_l1_or_axial_reports() {
    let original = composed(ConicalChartDomainVersion::OriginalV2);
    let family = composed(ConicalChartDomainVersion::FamilyV3);
    assert_eq!(family[..2], original[..2]);
    assert_eq!(family[2], 0.46875);
    assert!(original[2] > 0.5);
    // A known radial offset, angular displacement, axial offset and storage move
    // are each bounded by the corresponding dyadic inputs below.
    let [u, radius] = [0.125_f64, 2.];
    let xy = martensite_cmath::hypot(
        radius * (martensite_cmath::cos(u) - 1.) + 0.125,
        radius * martensite_cmath::sin(u),
    );
    assert!(martensite_cmath::hypot(xy, 0.0625) + 0.03125 <= family[2]);
}

fn composed(version: ConicalChartDomainVersion) -> [f64; 3] {
    let mut cancel = || false;
    let mut arithmetic = Arithmetic::new(
        ArithmeticPolicy {
            fractional_bits: 128,
            max_operations: 100,
        },
        &mut cancel,
    );
    let [radial, axial, radius, angular, storage] =
        [0.125, 0.0625, 2., 0.125, 0.03125].map(|value| arithmetic.scalar(value).unwrap());
    compose(
        radial,
        axial,
        radius,
        angular,
        storage,
        version,
        &mut arithmetic,
    )
    .unwrap()
}
