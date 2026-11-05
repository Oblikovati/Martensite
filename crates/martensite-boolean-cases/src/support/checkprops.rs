//! `checkprops shape -s|-l|-v value|empty [-eps e] [-deps d]`: each problem is an `Error` line
//! of the case's stream, which the parse rules grade afterwards.
use super::{
    properties::{Prop, props},
    session::Session,
};
use martensite_tkernel::storage::format_general;

/// The epsilon passed to the property command unless `-eps` is given.
pub const CHECKPROPS_EPSILON: f64 = 1.0e-4;
/// The relative deviation allowed unless `-deps` is given.
pub const CHECKPROPS_DEVIATION: f64 = 1.0e-2;

/// The expected property of a `checkprops`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Expected {
    /// `empty`: a printed mass of exactly 0.
    Empty,
    /// A value: not zero, and within the allowed deviation of it.
    Mass(f64),
}

impl std::fmt::Display for Expected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expected::Empty => formatter.write_str("empty"),
            Expected::Mass(value) => write!(formatter, "{value}"),
        }
    }
}

/// The two tolerances of a `checkprops`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CheckTolerances {
    /// `-eps`: the integration epsilon.
    pub epsilon: f64,
    /// `-deps`: the allowed relative deviation.
    pub deviation: f64,
}

impl Default for CheckTolerances {
    fn default() -> Self {
        Self {
            epsilon: CHECKPROPS_EPSILON,
            deviation: CHECKPROPS_DEVIATION,
        }
    }
}

/// The `Error` lines a printed mass earns; `mass_problems(Prop::Area, 0., Expected::Empty, 0.01)`.
pub fn mass_problems(kind: Prop, mass: f64, expected: Expected, deviation: f64) -> Vec<String> {
    match expected {
        Expected::Empty => empty_problems(kind, mass),
        Expected::Mass(value) => value_problems(kind, mass, value, deviation),
    }
}

fn empty_problems(kind: Prop, mass: f64) -> Vec<String> {
    if mass == 0. {
        return Vec::new();
    }
    let (word, printed) = (kind.word(), format_general(mass, 6));
    vec![format!(
        "Error : The command is not valid. The {word} is {printed}"
    )]
}

fn value_problems(kind: Prop, mass: f64, value: f64, deviation: f64) -> Vec<String> {
    let (word, printed) = (kind.word(), format_general(mass, 6));
    let mut problems = Vec::new();
    if mass == 0. {
        problems.push(format!(
            "Error : The command is not valid. The {word} is 0."
        ));
    }
    let off = if value != 0. {
        ((value - mass) / value).abs() > deviation
    } else {
        mass != 0.
    };
    if off {
        problems.push(format!(
            "Error : The {word} of result shape is {printed}, expected {value}"
        ));
    }
    problems
}

/// `checkprops name -s|-l|-v expected`; `checkprops(s, "result", Prop::Area, Expected::Mass(4.))`.
pub fn checkprops(session: &dyn Session, name: &str, kind: Prop, expected: Expected) {
    checkprops_within(session, name, kind, expected, CheckTolerances::default());
}

/// `checkprops` with explicit `-eps` and `-deps`.
/// Example: `checkprops_within(s, "result", Prop::Area, Expected::Empty, CheckTolerances::default())`.
pub fn checkprops_within(
    session: &dyn Session,
    name: &str,
    kind: Prop,
    expected: Expected,
    tolerances: CheckTolerances,
) {
    println!("checkprops {name} {} {expected}", kind.flag());
    let Some(shape) = session.shape(name).filter(|shape| !shape.is_null()) else {
        println!("Error: The command cannot be built");
        return;
    };
    let mass = props(kind, shape, Some(tolerances.epsilon));
    for problem in mass_problems(kind, mass, expected, tolerances.deviation) {
        println!("{problem}");
    }
}

/// `checkprops name -equal other`: without a property option, the command compares lengths.
/// Example: `checkprops_equal(s, "r1_0", "r_0")`.
pub fn checkprops_equal(session: &dyn Session, name: &str, other: &str) {
    println!("checkprops {name} -equal {other}");
    let epsilon = Some(CHECKPROPS_EPSILON);
    let first = bound_mass(session, name, epsilon);
    let second = bound_mass(session, other, epsilon);
    if first != second {
        println!("Error: Shape {other} is not equal to shape {name}");
    }
}

fn bound_mass(session: &dyn Session, name: &str, epsilon: Option<f64>) -> f64 {
    let shape = session
        .shape(name)
        .unwrap_or_else(|| panic!("checkprops {name}: expected a bound shape"));
    props(Prop::Length, shape, epsilon)
}

#[test]
fn empty_refuses_any_nonzero_printed_mass() {
    assert!(mass_problems(Prop::Area, 0., Expected::Empty, 0.01).is_empty());
    assert_eq!(
        mass_problems(Prop::Area, 1.0e-20, Expected::Empty, 0.01),
        ["Error : The command is not valid. The area is 1e-20"]
    );
}

#[test]
fn a_value_refuses_zero_and_more_than_the_deviation() {
    assert!(mass_problems(Prop::Area, 4.03, Expected::Mass(4.), 0.01).is_empty());
    assert_eq!(
        mass_problems(Prop::Volume, 4.1, Expected::Mass(4.), 0.01),
        ["Error : The volume of result shape is 4.1, expected 4"]
    );
    assert_eq!(
        mass_problems(Prop::Length, 0., Expected::Mass(4.), 0.01),
        [
            "Error : The command is not valid. The length is 0.",
            "Error : The length of result shape is 0, expected 4"
        ]
    );
}

#[test]
fn an_expected_zero_refuses_any_mass() {
    assert!(mass_problems(Prop::Area, 0., Expected::Mass(0.), 0.01).len() == 1);
    assert_eq!(
        mass_problems(Prop::Area, 2., Expected::Mass(0.), 0.01),
        ["Error : The area of result shape is 2, expected 0"]
    );
}
