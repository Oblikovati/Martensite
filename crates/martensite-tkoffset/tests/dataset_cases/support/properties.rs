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

/// `sprops shape [epsilon]` (or the other two): writes the report, returns the printed mass.
/// Example: `let mass = props(Prop::area, &shape, Some(1.0e-4));`.
pub fn props(kind: Prop, shape: &Shape, epsilon: Option<f64>) -> f64 {
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
    printed_mass(properties.mass())
}

/// The centre of gravity of `shape`'s area.
pub fn center_of_area(shape: &Shape) -> martensite_tkmath::gp::Point3 {
    let mut properties = GProps::default();
    measured(Prop::Area, shape, FIXED_RULE_EPSILON, &mut properties);
    properties.centre_of_mass()
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

#[test]
fn property_commands_return_the_printed_mass() {
    let mut maker = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::at_point(
        martensite_tkmath::gp::Point3::new(0., 0., 0.),
        1.,
        1.,
        1.,
    )
    .unwrap();
    let cube =
        martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape::shape(&mut maker).unwrap();
    assert_eq!(props(Prop::Area, cube, Some(1.0e-4)), 6.);
    assert_eq!(props(Prop::Area, cube, None), 6.);
    assert_eq!(props(Prop::Volume, cube, Some(1.0e-4)), 1.);
    assert_eq!(props(Prop::Length, cube, None), 24.);
}
