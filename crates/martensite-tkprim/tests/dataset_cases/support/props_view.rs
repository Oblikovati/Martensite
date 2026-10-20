//! `lprops`/`sprops`/`vprops` as the case harness runs them: the whole
//! printed report (mass, relative error, centre of gravity, matrix of inertia and
//! moments, `-full` or not) and the orange principal axes of the principal axes each call
//! puts in the viewer; the `checkprops` check procedure with several of `-l`,
//! `-s`, `-v` and the `bugs_begin` `checkarea` proc on top.
use super::log;
use super::logged_checks::checkreal;
use super::props::Prop;
use super::session::{Session, ViewStep};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::storage::format_general;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tktopalgo::brep_gprop::BRepGProp;

/// What one property command computed and printed.
#[derive(Clone, Debug)]
pub struct PropsOutput {
    pub mass: f64,
    pub centre: Point3,
    /// `MatrixOfInertia()`, rows then columns.
    pub inertia: [[f64; 3]; 3],
    /// The relative error the adaptive integration reached (with an epsilon, not `lprops`).
    pub error: Option<f64>,
    /// The command's script result.
    pub text: String,
}

impl PropsOutput {
    /// `regexp {Mass +: +([-0-9.+eE]+)}` of the printed report, as script reads it.
    pub fn read_mass(&self) -> f64 {
        super::text::numbers_after(&self.text, "Mass : ", 1, "").expect("Mass : in the report")[0]
    }
}

fn g6(value: f64) -> String {
    format!("{:>15}", format_general(value, 6))
}

fn g17(value: f64) -> String {
    format_general(value, 17)
}

/// `lprops|sprops|vprops shape [eps] [-full]` (no `c`, no point, no `-skip`/`-tri`): the
/// properties, the printed report (logged) and the principal axes shown in the viewer.
pub fn props_command(
    session: &mut dyn Session,
    kind: Prop,
    shape: &Shape,
    eps: Option<f64>,
    full: bool,
) -> PropsOutput {
    let mut g = GProps::default();
    let error = match (kind, eps) {
        // With or without an epsilon, `LinearProperties(S, G, SkipShared)`.
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
    let p = g.centre_of_mass();
    let matrix = g.matrix_of_inertia();
    let inertia: [[f64; 3]; 3] = std::array::from_fn(|row| {
        std::array::from_fn(|col| matrix.value(row as i32 + 1, col as i32 + 1).unwrap())
    });
    let principal = g
        .principal_properties()
        .unwrap_or_else(|failure| panic!("GProp_PrincipalProps: {failure:?}"));
    let (ix, iy, iz) = principal.moments();
    let mass = g.mass();
    let mut text = String::new();
    if !full {
        text.push_str(&format!("\n\nMass : {}\n\n", g6(mass)));
        if let Some(error) = error {
            text.push_str(&format!(
                "Relative error of mass computation : {}\n\n",
                g6(error)
            ));
        }
        text.push_str(&format!(
            "Center of gravity : \nX = {}\nY = {}\nZ = {}\n\n",
            g6(p.x()),
            g6(p.y()),
            g6(p.z())
        ));
        text.push_str("Matrix of Inertia : \n");
        for row in inertia {
            text.push_str(&format!("{} {} {}\n", g6(row[0]), g6(row[1]), g6(row[2])));
        }
        text.push('\n');
        // `std::ends` closes each of the two string streams.
        text.push('\0');
        text.push_str(&format!(
            "Moments : \nIX = {}\nIY = {}\nIZ = {}\n\n\0",
            g6(ix),
            g6(iy),
            g6(iz)
        ));
    } else {
        text.push_str(&format!("\n\nMass : {}\n\n", g17(mass)));
        if let Some(error) = error {
            text.push_str(&format!(
                "Relative error of mass computation : {}\n\n",
                g17(error)
            ));
        }
        text.push_str(&format!(
            "Center of gravity : \nX = {}\nY = {}\nZ = {}\n\n",
            g17(p.x()),
            g17(p.y()),
            g17(p.z())
        ));
        text.push_str("Matrix of Inertia :\n");
        for row in inertia {
            text.push_str(&format!(
                "{}    {}    {}\n",
                g17(row[0]),
                g17(row[1]),
                g17(row[2])
            ));
        }
        text.push('\n');
        text.push_str(&format!(
            "Moments :\nIX = {}\nIY = {}\nIZ = {}\n\n",
            g17(ix),
            g17(iy),
            g17(iz)
        ));
    }
    log::result(&text.replace('\0', ""));
    let third = Direction3::from_vector(principal.third_axis_of_inertia());
    let first = Direction3::from_vector(principal.first_axis_of_inertia());
    let axes = third
        .and_then(|third| Axis2::with_x_direction(p, third, first?))
        .unwrap_or_else(|failure| panic!("props: gp_Ax2 of the principal axes: {failure:?}"));
    session.step(ViewStep::InertiaAxes(&axes));
    PropsOutput {
        mass,
        centre: p,
        inertia,
        error,
        text,
    }
}

/// The `checkprops` options the cases pass: `-l`, `-s`, `-v` expected values, `-eps`
/// (1e-4 by default) and `-deps` (1e-2 by default).
#[derive(Clone, Copy, Debug, Default)]
pub struct Expected {
    pub length: Option<f64>,
    pub area: Option<f64>,
    pub volume: Option<f64>,
    pub eps: Option<f64>,
    pub deps: Option<f64>,
}

impl Expected {
    pub fn l(length: f64) -> Self {
        Self {
            length: Some(length),
            ..Self::default()
        }
    }
    pub fn s(area: f64) -> Self {
        Self {
            area: Some(area),
            ..Self::default()
        }
    }
    pub fn v(volume: f64) -> Self {
        Self {
            volume: Some(volume),
            ..Self::default()
        }
    }
}

/// `checkprops shape [-l L] [-s S] [-v V] [-eps e] [-deps d]`: `lprops`, `sprops`, then
/// `vprops` with the epsilon, each printed mass read back and compared as the proc does.
pub fn checkprops(session: &mut dyn Session, name: &str, shape: &Shape, expected: Expected) {
    let mut args = Vec::new();
    for (flag, value) in [
        ("-l", expected.length),
        ("-s", expected.area),
        ("-v", expected.volume),
        ("-eps", expected.eps),
        ("-deps", expected.deps),
    ] {
        if let Some(value) = value {
            args.push(format!("{flag} {value}"));
        }
    }
    log::puts(&format!("checkprops {name} {}", args.join(" ")));
    let epsilon = expected.eps.unwrap_or(super::props::CHECKPROPS_EPSILON);
    let depsilon = expected.deps.unwrap_or(super::props::CHECKPROPS_DEPSILON);
    for (kind, mass, prop) in [
        (Prop::Length, expected.length, "length"),
        (Prop::Area, expected.area, "area"),
        (Prop::Volume, expected.volume, "volume"),
    ] {
        let Some(mass) = mass else { continue };
        let output = props_command(session, kind, shape, Some(epsilon), false);
        let m = output.read_mass();
        if m == 0.0 {
            log::puts(&format!(
                "Error : The command is not valid. The {prop} is 0."
            ));
        }
        if (mass != 0.0 && ((mass - m) / mass).abs() > depsilon) || (mass == 0.0 && m != 0.0) {
            log::puts(&format!(
                "Error : The {prop} of result shape is {}, expected {}",
                tcl_number(m),
                tcl_number(mass)
            ));
        }
    }
}

/// How script prints a number it read back from text (the shortest round-trip form).
pub fn tcl_number(value: f64) -> String {
    if value == value.trunc() && value.abs() < 1e15 {
        format!("{value:.0}")
    } else {
        format!("{value}")
    }
}

/// `bugs_begin` `checkarea shape area_expected tol_abs tol_rel`: `sprops` with the
/// relative precision `0.5 * |tol_rel| + 0.001`, then `checkreal "area of shape"`.
pub fn checkarea(
    session: &mut dyn Session,
    name: &str,
    shape: &Shape,
    expected: f64,
    tol_abs: f64,
    tol_rel: f64,
) {
    let output = props_command(
        session,
        Prop::Area,
        shape,
        Some(0.5 * tol_rel.abs() + 0.001),
        false,
    );
    let area = output.read_mass();
    checkreal(&format!("area of {name}"), area, expected, tol_abs, tol_rel);
}
