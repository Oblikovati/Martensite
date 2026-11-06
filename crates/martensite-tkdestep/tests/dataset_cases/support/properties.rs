//! The `lprops`, `sprops` and `vprops` commands: they measure, write the full report to the
//! case's stream and leave the printed mass for the checks that read it back.
use martensite_geometry_raster::property_report;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::storage::format_general;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

/// The relative error a command without an epsilon integrates with. The commands then use the
/// non-adaptive rule; the kernel selects it from any epsilon above 0.001.
const FIXED_RULE_EPSILON: f64 = 1.0;

/// Which property command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prop {
    /// `lprops`: length.
    Length,
    /// `sprops`: area.
    Area,
    /// `vprops`: volume.
    Volume,
}

impl Prop {
    /// The word the checks use for it; `Prop::area.word() == "area"`.
    pub fn word(self) -> &'static str {
        match self {
            Prop::Length => "length",
            Prop::Area => "area",
            Prop::Volume => "volume",
        }
    }

    /// Its option in the check commands; `Prop::area.flag() == "-s"`.
    pub fn flag(self) -> &'static str {
        match self {
            Prop::Length => "-l",
            Prop::Area => "-s",
            Prop::Volume => "-v",
        }
    }
}

fn measured(kind: Prop, shape: &Shape, epsilon: f64, properties: &mut GProps) -> f64 {
    let outcome = match kind {
        Prop::Length => BRepGProp::linear_properties(shape, properties, false, false).map(|()| 0.),
        Prop::Area => BRepGProp::surface_properties_eps(shape, properties, epsilon, false),
        Prop::Volume => BRepGProp::volume_properties_eps(shape, properties, epsilon, false, false),
    };
    outcome.unwrap_or_else(|failure| panic!("{} with epsilon {epsilon}: {failure:?}", kind.word()))
}

/// `sprops shape [epsilon]` (or the other two): writes the report, returns the properties.
fn reported(kind: Prop, shape: &Shape, epsilon: Option<f64>) -> GProps {
    let mut properties = GProps::default();
    let reached = measured(
        kind,
        shape,
        epsilon.unwrap_or(FIXED_RULE_EPSILON),
        &mut properties,
    );
    let printed_error = (epsilon.is_some() && kind != Prop::Length).then_some(reached);
    let principal = properties.principal_properties().unwrap();
    print!(
        "{}",
        property_report(&properties, &principal, printed_error, false).unwrap()
    );
    properties
}

/// `sprops shape [epsilon]` (or the other two): writes the report, returns the printed mass.
/// Example: `let mass = props(Prop::area, &shape, Some(1.0e-4));`.
pub fn props(kind: Prop, shape: &Shape, epsilon: Option<f64>) -> f64 {
    printed_mass(reported(kind, shape, epsilon).mass())
}

/// `checkgravitycenter name -s x y z tol`: `sprops`' report, and its printed centre against the words
/// of the script, which the procedure's message repeats as they are.
/// Example: `checkgravitycenter_of_area("a", &shape, ["-30.", "0.", "137.993"], "0.001")`.
pub fn checkgravitycenter_of_area(name: &str, shape: &Shape, expected: [&str; 3], tolerance: &str) {
    let [ex, ey, ez] = expected;
    println!("checkgravitycenter {name} -s {ex} {ey} {ez} {tolerance}");
    println!("sprops {name} ");
    let centre = reported(Prop::Area, shape, None).centre_of_mass();
    let [x, y, z] = [centre.x(), centre.y(), centre.z()].map(general_six);
    let limit: f64 = tolerance.parse().expect("a tolerance");
    let near = [(&x, ex), (&y, ey), (&z, ez)]
        .iter()
        .all(|(value, wanted)| {
            let (value, wanted): (f64, f64) = (
                value.parse().expect("a real"),
                wanted.parse().expect("a real"),
            );
            (value - wanted).abs() < limit
        });
    if near {
        println!(
            "Check of center of gravity is OK: value = ({x}, {y}, {z}), expected = ({ex}, {ey}, {ez})"
        );
    } else {
        println!(
            "Error: center of gravity ({x}, {y}, {z}) is not equal to expected ({ex}, {ey}, {ez})"
        );
    }
}

/// A value as a report writes it; `general_six(94.99941) == "94.9994"`.
pub fn general_six(value: f64) -> String {
    format_general(value, 6)
}

/// A mass as the checks read it back from the report: six significant digits.
/// Example: `printed_mass(123456.5) == 123456.0`.
pub fn printed_mass(mass: f64) -> f64 {
    let text = format_general(mass, 6);
    text.parse()
        .unwrap_or_else(|cause| panic!("mass {mass} printed as {text:?}: {cause}"))
}

#[test]
fn printed_mass_keeps_six_significant_digits() {
    assert_eq!(printed_mass(123456.5), 123456.0);
    assert_eq!(printed_mass(3.5), 3.5);
    assert_eq!(printed_mass(1.0e-20), 1.0e-20);
}
