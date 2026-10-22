//! Property-check formatting and comparisons used by scenario cases.
//!
//! The commands print `Mass : ` with `std::setw(15)` and the stream's default precision
//! (6 significant digits, `%g`); `checkprops` reads that printed value back with
//! `regexp {Mass +: +([-0-9.+eE]+)}`, so it compares the rounded mass.
use super::log;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::storage::format_general;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_gprop::BRepGProp;

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
    fn name(self) -> &'static str {
        match self {
            Prop::Length => "length",
            Prop::Area => "area",
            Prop::Volume => "volume",
        }
    }
}

/// The result of one property command.
#[derive(Clone, Debug)]
pub struct PropsResult {
    pub mass: f64,
    pub centre: Point3,
    /// The relative error the adaptive integration reached (with an epsilon).
    pub error: Option<f64>,
}

impl PropsResult {
    /// The printed mass, `std::cout << mass` (`%g`, 6 digits).
    pub fn printed_mass(&self) -> String {
        format_general(self.mass, 6)
    }

    /// The printed mass as parsed by the scenario result reader.
    pub fn read_mass(&self) -> f64 {
        self.printed_mass().parse().unwrap()
    }
}

/// `lprops|sprops|vprops shape [eps]` (no `c`, no point, no `-skip`/`-full`/`-tri`).
pub fn props(kind: Prop, shape: &Shape, eps: Option<f64>) -> PropsResult {
    let mut g = GProps::default();
    let error = match (kind, eps) {
        (Prop::Length, _) => {
            BRepGProp::linear_properties(shape, &mut g, false, false).unwrap();
            None
        }
        (Prop::Area, Some(eps)) => {
            Some(BRepGProp::surface_properties_eps(shape, &mut g, eps, false).unwrap())
        }
        (Prop::Area, None) => {
            BRepGProp::surface_properties(shape, &mut g, false, false).unwrap();
            None
        }
        (Prop::Volume, Some(eps)) => {
            Some(BRepGProp::volume_properties_eps(shape, &mut g, eps, false, false).unwrap())
        }
        (Prop::Volume, None) => {
            BRepGProp::volume_properties(shape, &mut g, false, false, false).unwrap();
            None
        }
    };
    PropsResult {
        mass: g.mass(),
        centre: g.centre_of_mass(),
        error,
    }
}

/// The `checkprops` epsilon passed to the property commands unless `-eps` is given.
pub const CHECKPROPS_EPSILON: f64 = 1.0e-4;
/// The `checkprops` relative deviation allowed unless `-deps` is given.
pub const CHECKPROPS_DEPSILON: f64 = 1.0e-2;

/// `checkprops shape -l|-s|-v expected [-eps eps]`: logs `checkprops ...` and its `Error`
/// lines: a zero printed mass, or one more than `depsilon` (1%) away from `expected` (any
/// nonzero mass when `expected` is 0).
pub fn checkprops_eps(shape: &Shape, kind: Prop, expected: f64, eps: f64) {
    let flag = match kind {
        Prop::Length => "-l",
        Prop::Area => "-s",
        Prop::Volume => "-v",
    };
    log::puts(&format!("checkprops {flag} {expected} -eps {eps}"));
    let m = props_logged(kind, shape, Some(eps)).read_mass();
    let prop = kind.name();
    if m == 0.0 {
        log::puts(&format!(
            "Error : The command is not valid. The {prop} is 0."
        ));
    }
    let off = if expected != 0.0 {
        ((expected - m) / expected).abs() > CHECKPROPS_DEPSILON
    } else {
        m != 0.0
    };
    if off {
        log::puts(&format!(
            "Error : The {prop} of result shape is {m}, expected {expected}"
        ));
    }
}

/// `checkprops shape -l|-s|-v expected`.
pub fn checkprops(shape: &Shape, kind: Prop, expected: f64) {
    checkprops_eps(shape, kind, expected, CHECKPROPS_EPSILON);
}

/// `checkprops shape -equal other` without `-l`/`-s`/`-v`: the proc's `equal_check`
/// adds `lprops` and then resets, so only the printed lengths are compared.
pub fn checkprops_equal(shape: &Shape, other: &Shape) {
    log::puts("checkprops -equal");
    let m = props_logged(Prop::Length, shape, Some(CHECKPROPS_EPSILON)).read_mass();
    let compared = props_logged(Prop::Length, other, Some(CHECKPROPS_EPSILON)).read_mass();
    if compared != m {
        log::puts("Error: Shape is not equal to shape");
    }
}

/// The command and its logged output (`Mass : ...` and, with an epsilon and not
/// `lprops`, `Relative error of mass computation : ...`).
pub fn props_logged(kind: Prop, shape: &Shape, eps: Option<f64>) -> PropsResult {
    let result = props(kind, shape, eps);
    let mut text = format!("\n\nMass : {:>15}\n\n", result.printed_mass());
    if let Some(error) = result.error {
        text.push_str(&format!(
            "Relative error of mass computation : {:>15}\n\n",
            format_general(error, 6)
        ));
    }
    let c = result.centre;
    text.push_str(&format!(
        "Center of gravity : \nX = {:>15}\nY = {:>15}\nZ = {:>15}\n",
        format_general(c.x(), 6),
        format_general(c.y(), 6),
        format_general(c.z(), 6)
    ));
    log::result(&text);
    result
}
