//! Keep the two explicitly expected CheckSpiral defects as observable predicates.
use super::{helix_script_inputs::CASES, helix_script_wire::build_wire};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::storage::format_general;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

const D6_REFERENCE: [f64; 6] = [
    1.05657e-14,
    -1.15567e-15,
    50.,
    4.01968e6,
    3.27912e6,
    3.19438e6,
];
const D7_REFERENCE: [f64; 6] = [
    1.15868e-14,
    50.,
    1.15567e-15,
    4.01968e6,
    3.27912e6,
    3.19438e6,
];

fn formatted_linear_values(wire: &Shape) -> [f64; 6] {
    let mut properties = GProps::default();
    BRepGProp::linear_properties(wire, &mut properties, false, false).unwrap();
    let center = properties.centre_of_mass();
    let (ix, iy, iz) = properties.principal_properties().unwrap().moments();
    [center.x(), center.y(), center.z(), ix, iy, iz]
        .map(|value| format_general(value, 6).parse().unwrap())
}

pub(super) fn assert_original_spiral_predicate(name: &str, wire: &Shape) {
    let expected = match name {
        "D6" => D6_REFERENCE,
        "D7" => D7_REFERENCE,
        _ => return,
    };
    // The scripts mark the numeric inequality as a known problem. Keeping the
    // complete six-value predicate preserves that observed defect, not just one field.
    assert_ne!(formatted_linear_values(wire), expected);
}

fn check_original_spiral_defect(name: &str) {
    let input = CASES.iter().find(|case| case.name == name).unwrap();
    let wire = build_wire(input);
    assert_eq!(wire.error_status(), 0);
    assert_original_spiral_predicate(name, wire.shape());
}

#[test]
fn helix_d6_original_spiral_predicate_remains_an_expected_failure() {
    check_original_spiral_defect("D6");
}

#[test]
fn helix_d7_original_spiral_predicate_remains_an_expected_failure() {
    check_original_spiral_defect("D7");
}
