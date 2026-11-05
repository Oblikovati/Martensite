//! Original printed-length predicates and default ShapeSet reference counts.
use crate::{canonical_property, default_shape_counts};
use martensite_tkbrep::topods::Shape;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis2, Direction3};

const LABELS: [&str; 9] = [
    "VERTEX",
    "EDGE",
    "WIRE",
    "FACE",
    "SHELL",
    "SOLID",
    "COMPSOLID",
    "COMPOUND",
    "SHAPE",
];
const INDICES: [usize; 9] = [7, 6, 5, 4, 3, 2, 1, 0, 8];

/// Replay the exact ordered length/count predicates; `case_checks(&r1, &r2)`.
pub(super) fn case_checks(first: &Shape, second: &Shape) {
    println!("checkprops r1 -l 78.3329");
    canonical_property::check_property_soft_observed(first, 78.3329, true, print_properties);
    println!("checknbshapes r1 -vertex 3 -edge 2");
    let first_counts = counts_report("r1", first);
    compare_count("r1", "VERTEX", first_counts[7], 3);
    compare_count("r1", "EDGE", first_counts[6], 2);
    equal_length(second, first);
    compare_reference_counts(second, first);
}

fn equal_length(second: &Shape, first: &Shape) {
    println!("checkprops r2 -equal r1");
    // The reference scene CheckCommands measures r2 first, then r1; equal_check selects lprops only.
    let actual = printed_length(second);
    let reference = printed_length(first);
    if actual != reference {
        println!("Error: Shape r1 is not equal to shape r2");
    }
}

fn printed_length(shape: &Shape) -> f64 {
    canonical_property::measured_property_error_observed(shape, true, |properties, error| {
        assert!(
            error.is_none(),
            "lprops: expected no surface integration error estimate"
        );
        print_properties(properties);
    })
}

fn print_properties(properties: &GProps) {
    let principal = properties.principal_properties().unwrap();
    print!(
        "{}",
        martensite_geometry_raster::property_report(properties, &principal, None, false).unwrap()
    );
    // At RasterViewer::operator<< returns immediately in RasterBatch; still construct axes.
    let _axes = Axis2::with_x_direction(
        properties.centre_of_mass(),
        Direction3::from_vector(principal.third_axis_of_inertia()).unwrap(),
        Direction3::from_vector(principal.first_axis_of_inertia()).unwrap(),
    )
    .unwrap();
}

fn counts_report(name: &str, shape: &Shape) -> [usize; 9] {
    let counts = default_shape_counts::counts(shape);
    println!("Number of shapes in {name}");
    for (label, index) in LABELS.iter().zip(INDICES) {
        println!(" {label:<9} : {}", counts[index]);
    }
    counts
}

fn compare_reference_counts(second: &Shape, first: &Shape) {
    println!("checknbshapes r2 -ref nbshapes(r1)");
    let reference = counts_report("r1", first);
    let actual = counts_report("r2", second);
    for (label, index) in LABELS.iter().zip(INDICES) {
        compare_count("r2", label, actual[index], reference[index]);
    }
}

fn compare_count(name: &str, label: &str, actual: usize, reference: usize) {
    if actual != reference {
        println!(
            "Error :  is WRONG because number of {label} entities in shape \"{name}\" is {actual}"
        );
    } else {
        println!(
            "OK :  is GOOD because number of {label} entities is equal to number of expected {label} entities"
        );
    }
}
