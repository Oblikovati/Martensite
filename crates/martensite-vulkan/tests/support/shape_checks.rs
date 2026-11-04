//! Typed checks of a shape's validity and of its length, area and volume, printing the
//! regressions' messages: an error line for each failed check.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use martensite_tktopalgo::brep_gprop::BRepGProp;

/// Check topology and geometry of `shape`; prints "This shape seems to be valid" or a
/// faulty-shape report; `check_shape_valid(&result)?`.
pub fn check_shape_valid(shape: &Shape) -> Result<bool, Failure> {
    let valid = BRepCheckAnalyzer::new(shape, true, false, false)?.is_valid()?;
    if valid {
        println!("This shape seems to be valid");
    } else {
        println!("Faulty shapes in variables faulty_1 ; shape is not valid");
    }
    Ok(valid)
}

/// The property a mass check integrates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MassKind {
    /// Total edge length.
    Length,
    /// Total face area.
    Area,
    /// Enclosed volume.
    Volume,
}

impl MassKind {
    fn label(self) -> &'static str {
        match self {
            Self::Length => "length",
            Self::Area => "area",
            Self::Volume => "volume",
        }
    }
}

/// An expected mass: a value within the relative tolerance `relative` (1% by default), or
/// `Empty` for a property that must be 0; `MassCheck::near(MassKind::Length, 6.34983)`.
#[derive(Clone, Copy, Debug)]
pub struct MassCheck {
    /// The property integrated.
    pub kind: MassKind,
    /// The expected mass; `None` requires an empty (zero) property.
    pub expected: Option<f64>,
    /// The integration's relative error.
    pub epsilon: f64,
    /// The relative tolerance of the comparison.
    pub relative: f64,
}

impl MassCheck {
    /// The default check of `kind` against `expected`; `MassCheck::near(MassKind::Area, 2.)`.
    pub fn near(kind: MassKind, expected: f64) -> Self {
        Self {
            kind,
            expected: Some(expected),
            epsilon: 1.0e-4,
            relative: 1.0e-2,
        }
    }

    /// A property that must be 0; `MassCheck::empty(MassKind::Volume)`.
    pub fn empty(kind: MassKind) -> Self {
        Self {
            expected: None,
            ..Self::near(kind, 0.)
        }
    }
}

/// Integrate the shape's property as printed with six significant digits, then compare it
/// with the check's expectation; returns the integrated properties.
/// Example: `check_mass(&result, &MassCheck::near(MassKind::Length, 6.3))?.mass()`.
pub fn check_mass(shape: &Shape, check: &MassCheck) -> Result<GProps, Failure> {
    let props = integrate(shape, check)?;
    report_mass(six_significant_digits(props.mass()), check);
    Ok(props)
}

fn report_mass(printed: f64, check: &MassCheck) {
    let label = check.kind.label();
    let Some(expected) = check.expected else {
        if printed != 0. {
            println!("Error : The command is not valid. The {label} is {printed}");
        }
        return;
    };
    if printed == 0. {
        println!("Error : The command is not valid. The {label} is 0.");
    }
    let off = if expected != 0. {
        ((expected - printed) / expected).abs() > check.relative
    } else {
        printed != 0.
    };
    if off {
        println!("Error : The {label} of result shape is {printed}, expected {expected}");
    }
}

fn integrate(shape: &Shape, check: &MassCheck) -> Result<GProps, Failure> {
    let mut props = GProps::new();
    match check.kind {
        MassKind::Length => BRepGProp::linear_properties(shape, &mut props, false, false)?,
        MassKind::Area => {
            BRepGProp::surface_properties_eps(shape, &mut props, check.epsilon, false)?;
        }
        MassKind::Volume => {
            BRepGProp::volume_properties_eps(shape, &mut props, check.epsilon, false, false)?;
        }
    }
    Ok(props)
}

/// The value as a default C++ stream prints it (six significant digits), read back.
/// Example: `six_significant_digits(6.349831) == 6.34983`.
pub fn six_significant_digits(value: f64) -> f64 {
    if value == 0. || !value.is_finite() {
        return value;
    }
    format!("{value:.5e}")
        .parse()
        .expect("a formatted float parses back")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn six_significant_digits_round_like_a_default_stream() {
        assert_eq!(six_significant_digits(6.349831), 6.34983);
        assert_eq!(six_significant_digits(1038.6249), 1038.62);
        assert_eq!(six_significant_digits(0.), 0.);
    }

    #[test]
    fn empty_shape_has_no_length() {
        let check = MassCheck::empty(MassKind::Length);
        assert!(check_mass(&Shape::default(), &check).is_ok());
    }
}
