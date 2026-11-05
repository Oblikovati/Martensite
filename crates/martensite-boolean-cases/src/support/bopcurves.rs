//! `bopcurves first second [-2d]`: the intersection of two faces, with the command's own lines.
//!
//! The command also binds its curves as `c_N`, its 2D curves as `c2d1_N` and `c2d2_N` and its
//! points as `p_N`: [`bopcurves_bound`] does that, for a script that reads or displays them.
use super::session::{Drawing, Session};
use martensite_tkbo::int_tools::{Curve, FaceFace};
use martensite_tkbrep::topods;
use martensite_tkernel::{precision, standard::Handle, storage::format_general};
use martensite_tkg3d::geom::GeomCurve;
use martensite_tkgeomalgo::int_surf::IntSurfPntOn2S;
use martensite_tkmath::gp::Point3;

/// The approximation tolerance the command asks for.
const APPROXIMATION_TOLERANCE: f64 = 1.0e-7;
/// The digits a stream writes a double with.
const FULL_DIGITS: usize = 17;

/// What a script reads back from the command's answer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BopCurves {
    /// `Tolerance Reached=` as printed; absent without curves.
    pub tolerance_reached: Option<String>,
    /// `<n> curve(s) found.`
    pub curves: usize,
}

/// The options of one [`bopcurves_bound`].
#[derive(Clone, Copy, Debug, Default)]
pub struct BopCurvesOptions<'a> {
    /// `-2d`.
    pub pcurves: bool,
    /// Each `-p u1 v1 u2 v2`.
    pub start_points: &'a [[f64; 4]],
}

impl BopCurvesOptions<'static> {
    /// No option.
    pub const PLAIN: Self = Self {
        pcurves: false,
        start_points: &[],
    };
    /// `-2d`.
    pub const TWO_D: Self = Self {
        pcurves: true,
        start_points: &[],
    };
}

/// Everything a script reads back from the command, its curves and points included.
#[derive(Default)]
pub struct BoundCurves {
    /// What [`bopcurves`] answers.
    pub answer: BopCurves,
    /// `c_1`, `c_2`, ...
    pub lines: Vec<Curve>,
    /// `c_1`, `c_2`, ...: the names, for a view command.
    pub names: Vec<String>,
    /// `c2d1_1`, `c2d2_1`, ...: the names of the plane curves.
    pub plane_names: Vec<String>,
    /// `p_1`, `p_2`, ...: the names of the points.
    pub point_names: Vec<String>,
}

fn start_list(start_points: &[[f64; 4]]) -> Vec<IntSurfPntOn2S> {
    let at = |parameters: &[f64; 4]| {
        let mut point = IntSurfPntOn2S::new();
        point.set_parameters(parameters[0], parameters[1], parameters[2], parameters[3]);
        point
    };
    start_points.iter().map(at).collect()
}

fn performed(
    s: &dyn Session,
    first: &str,
    second: &str,
    options: BopCurvesOptions<'_>,
) -> FaceFace {
    let pcurves = options.pcurves;
    let face_of = |name: &str| {
        let shape = s
            .shape(name)
            .unwrap_or_else(|| panic!("bopcurves {name}: expected a bound shape"));
        topods::face(shape).unwrap_or_else(|failure| panic!("bopcurves {name}: {failure:?}"))
    };
    let mut operation = FaceFace::default();
    operation.set_parameters(true, pcurves, pcurves, APPROXIMATION_TOLERANCE);
    operation.set_list(&start_list(options.start_points));
    operation.set_fuzzy_value(precision::confusion());
    let (first, second) = (face_of(first), face_of(second));
    operation
        .perform(&first, &second, false)
        .unwrap_or_else(|failure| panic!("bopcurves: {failure:?}"));
    operation
}

/// One curve's words of the command's answer; `curve_words(1, &curve)`.
fn curve_words(number: usize, curve: &Curve) -> String {
    if curve.curve().is_null() {
        return format!(" has Null 3d curve# {number}\n");
    }
    let (first, second) = (
        !curve.first_curve2d().is_null(),
        !curve.second_curve2d().is_null(),
    );
    let mut words = format!("c_{number} ");
    if first || second {
        let names: Vec<String> = [(first, 1), (second, 2)]
            .into_iter()
            .filter(|(present, _)| *present)
            .map(|(_, side)| format!("c2d{side}_{number}"))
            .collect();
        words += &format!("({}) ", names.join(", "));
    }
    words
}

fn answer(lines: &[Curve]) -> BopCurves {
    let reached = lines
        .iter()
        .fold(0.0_f64, |top, curve| top.max(curve.tolerance()));
    let printed = format_general(reached, FULL_DIGITS);
    println!("Tolerance Reached={printed}");
    println!("{} curve(s) found.", lines.len());
    let words: String = lines
        .iter()
        .enumerate()
        .map(|(index, curve)| curve_words(index + 1, curve))
        .collect();
    println!("{words}");
    BopCurves {
        tolerance_reached: Some(printed),
        curves: lines.len(),
    }
}

fn points_of(operation: &FaceFace) -> Vec<Point3> {
    let located = operation.points().iter();
    located.map(|point| *point.p1().pnt()).collect()
}

fn write_points(points: &[Point3]) -> Vec<String> {
    if points.is_empty() {
        return Vec::new();
    }
    println!("{} point(s) found.", points.len());
    let names: Vec<String> = (1..=points.len()).map(|i| format!("p_{i}")).collect();
    println!("{}", names.join(" "));
    names
}

/// `bopcurves first second`, with `-2d` when `pcurves`; `bopcurves(s, "f1", "f2", false)`.
pub fn bopcurves(s: &dyn Session, first: &str, second: &str, pcurves: bool) -> BopCurves {
    let options = BopCurvesOptions {
        pcurves,
        start_points: &[],
    };
    command_lines(s, first, second, options).0
}

/// The command's own lines; its answer, its curves and its points.
fn command_lines(
    s: &dyn Session,
    first: &str,
    second: &str,
    options: BopCurvesOptions<'_>,
) -> (BopCurves, Vec<Curve>, Vec<Point3>) {
    let mut operation = performed(s, first, second, options);
    if !operation.is_done() {
        println!("Error: Intersection failed");
        return Default::default();
    }
    operation.prepare_lines_3d(false).unwrap();
    let lines: Vec<Curve> = operation.lines().unwrap().to_vec();
    let points = points_of(&operation);
    if lines.is_empty() && points.is_empty() {
        println!(" has no 3d curves");
        println!(" has no 3d points");
        return Default::default();
    }
    let reply = if lines.is_empty() {
        BopCurves::default()
    } else {
        answer(&lines)
    };
    write_points(&points);
    (reply, lines, points)
}

fn bind_curve(s: &mut dyn Session, number: usize, curve: &Curve, bound: &mut BoundCurves) {
    if curve.curve().is_null() {
        return;
    }
    let name = format!("c_{number}");
    s.bind_drawing(&name, Drawing::Curve(curve.curve()));
    bound.names.push(name);
    for (side, plane) in [(1, curve.first_curve2d()), (2, curve.second_curve2d())] {
        if !plane.is_null() {
            let name = format!("c2d{side}_{number}");
            s.bind_drawing(&name, Drawing::PlaneCurve(plane));
            bound.plane_names.push(name);
        }
    }
}

/// `bopcurves first second ...`, the curves and points bound as the command binds them.
/// Example: `bopcurves_bound(s, "f1", "f2", BopCurvesOptions::default())`.
pub fn bopcurves_bound(
    s: &mut dyn Session,
    first: &str,
    second: &str,
    options: BopCurvesOptions<'_>,
) -> BoundCurves {
    let (answer, lines, points) = command_lines(s, first, second, options);
    let mut bound = BoundCurves {
        answer,
        ..BoundCurves::default()
    };
    for (index, curve) in lines.iter().enumerate() {
        bind_curve(s, index + 1, curve, &mut bound);
    }
    for (index, at) in points.iter().enumerate() {
        let name = format!("p_{}", index + 1);
        s.bind_drawing(&name, Drawing::Point(*at));
        bound.point_names.push(name);
    }
    bound.lines = lines;
    bound
}

impl BoundCurves {
    /// The script's `regexp {Tolerance Reached=+(..)\n+(..)}`: the tolerance as printed and the
    /// number of curves. Where nothing matches the script stops at the variable it then reads.
    /// Example: `let (toler, nb_curv) = log.reached_and_count();`.
    pub fn reached_and_count(&self) -> (&str, usize) {
        let printed = self.answer.tolerance_reached.as_deref().filter(|text| {
            text.chars()
                .all(|c| c.is_ascii_digit() || matches!(c, '-' | '.' | '+' | 'e' | 'E'))
        });
        let Some(printed) = printed else {
            super::script_error::script_error("can't read \"Toler\": no such variable");
        };
        (printed, self.answer.curves)
    }

    /// `c_N`, with `index` counted from 0; `log.curve_at(0)`.
    pub fn curve_at(&self, index: usize) -> Handle<dyn GeomCurve> {
        self.lines[index].curve().clone()
    }

    /// The tolerance reached as a number; `log.reached()`.
    pub fn reached(&self) -> f64 {
        let printed = self.reached_and_count().0;
        printed
            .parse()
            .unwrap_or_else(|cause| panic!("Tolerance Reached {printed:?}: {cause}"))
    }
}
