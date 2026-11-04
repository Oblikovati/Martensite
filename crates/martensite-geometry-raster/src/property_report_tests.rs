//! Owned text expectations pin formatting independently of display or interpreter state.
use super::*;

#[test]
fn compact_and_full_significant_digits_keep_existing_widths() {
    assert_eq!(
        PropertyReportStyle { full: false }.number(0.1),
        "            0.1"
    );
    assert_eq!(
        PropertyReportStyle { full: true }.number(0.1),
        "0.10000000000000001"
    );
    assert_eq!(
        PropertyReportStyle { full: false }.number(1234567.),
        "    1.23457e+06"
    );
}

#[test]
fn signed_zero_and_nonfinite_values_keep_existing_spelling() {
    let compact = PropertyReportStyle { full: false };
    let full = PropertyReportStyle { full: true };
    for (value, expected) in [
        (f64::INFINITY, "inf"),
        (f64::NEG_INFINITY, "-inf"),
        (f64::NAN, "nan"),
        (f64::from_bits(0xfff8_0000_0000_0000), "-nan"),
        (-0., "-0"),
    ] {
        assert_eq!(full.number(value), expected);
        assert_eq!(compact.number(value), format!("{expected:>15}"));
    }
}

#[test]
fn achieved_error_line_is_present_only_when_requested() {
    let full = PropertyReportStyle { full: true };
    assert_eq!(mass_report(full, 2., None), "\n\nMass : 2\n\n");
    assert_eq!(
        mass_report(full, 2., Some(0.125)),
        "\n\nMass : 2\n\nRelative error of mass computation : 0.125\n\n"
    );
}

#[test]
fn compact_labels_and_matrix_rows_preserve_spaces_and_newlines() {
    let style = PropertyReportStyle { full: false };
    let matrix = Matrix3::new([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
    let expected = concat!(
        "Matrix of Inertia : \n",
        "              1               2               3\n",
        "              4               5               6\n",
        "              7               8               9\n\n"
    );
    assert_eq!(matrix_report(style, matrix).unwrap(), expected);
}

#[test]
fn full_report_uses_owned_measurements_without_recomputing_them() {
    let properties = measured_properties();
    let expected = concat!(
        "\n\nMass : 2\n\n",
        "Relative error of mass computation : 0.125\n\n",
        "Center of gravity : \nX = 1\nY = -2\nZ = 0.125\n\n",
        "Matrix of Inertia :\n1    2    3\n4    5    6\n7    8    9\n\n",
        "Moments :\nIX = 1.7976931348623157e+308\nIY = 1.7976931348623157e+308\nIZ = 1.7976931348623157e+308\n\n"
    );
    let before = properties;
    assert_eq!(
        property_report(&properties, &PrincipalProps::default(), Some(0.125), true).unwrap(),
        expected
    );
    assert_eq!(properties, before);
}

fn measured_properties() -> GProps {
    GProps {
        dim: 2.,
        loc: Point3::new(1., -2., 0.125),
        inertia: Matrix3::new([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]),
        ..GProps::default()
    }
}

#[test]
fn undefined_moments_and_empty_properties_keep_compact_text() {
    let report =
        property_report(&GProps::default(), &PrincipalProps::default(), None, false).unwrap();
    assert!(report.starts_with("\n\nMass :               0\n\n"));
    assert!(report.ends_with(
        "Moments : \nIX =    1.79769e+308\nIY =    1.79769e+308\nIZ =    1.79769e+308\n\n"
    ));
    assert!(!report.contains("Relative error"));
}
