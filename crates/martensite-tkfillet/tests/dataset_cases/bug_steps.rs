//! The commands only the `bugs`, `perf` and `fillet2d` scripts use.
use crate::script::{FilletScript, command_stopped};
use crate::steps::{Chamfer, ChamferEdge, Check, HistoryQuery, Step};
use crate::support::properties::Prop;
use crate::support::{
    bop_session::BopOptions,
    checkprops::{CheckTolerances, Expected, checkprops_within},
    checkshape::checkshape,
    named_checks::{bopargcheck_faulties, bopcheck, checksection},
    nbshapes::{Counting, NbCount, checknbshapes, checknbshapes_named},
    tolerance_check::{checkmaxtol_ref, tolerance_report},
};
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    brep_tools::History,
    topods::{self, Compound, Shape},
};
use martensite_tkernel::{
    message::ProgressRange,
    precision,
    standard::{Failure, Handle},
    storage::format_general,
};
use martensite_tkfillet::{
    brep_fillet_api::{BRepFilletAPILocalOperation, BRepFilletAPIMakeChamfer},
    chfi_ds::ChamfMode,
};
use martensite_tkg3d::geom::GeomBSplineSurface;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::f64::consts::PI;

/// The stateless command set of the `bugs`-style scripts.
pub struct BugCommands;

impl BugCommands {
    /// Run a step of this set; `false` when it belongs elsewhere.
    pub fn apply(&self, script: &mut FilletScript<'_>, step: &Step) -> Result<bool, Failure> {
        match step {
            Step::Puts(text) => println!("{text}"),
            Step::Check(check) => run_check(script, check)?,
            Step::Chamfer(chamfer) => run_chamfer(script, chamfer)?,
            Step::Planar(planar) => crate::planar_steps::run_planar(script, planar)?,
            Step::History(query) => run_history(script, query)?,
            _ => return Ok(false),
        }
        Ok(true)
    }
}

fn run_check(script: &mut FilletScript<'_>, check: &Check) -> Result<(), Failure> {
    match check {
        Check::Shape(name) => check_shape(script, name),
        Check::Counts(..) | Check::TypedCounts(..) | Check::TypedCountsNamed(..) => {
            run_count_check(script, check)
        }
        Check::Props(kind, name, value, deviation) => {
            check_props(script, (*kind, name), *value, *deviation);
            Ok(())
        }
        Check::MaxTolerance(..)
        | Check::MaxToleranceRef(..)
        | Check::MaxToleranceMin(..)
        | Check::ToleranceBranch(..) => run_tolerance_check(script, check),
        _ => run_else_check(script, check),
    }
}

fn run_count_check(script: &mut FilletScript<'_>, check: &Check) -> Result<(), Failure> {
    match check {
        Check::Counts(name, wanted) => check_counts(script, name, Counting::Located, wanted),
        Check::TypedCounts(name, wanted) => check_counts(script, name, Counting::Typed, wanted),
        Check::TypedCountsNamed(name, wanted, message) => {
            script.required(name)?;
            let expected: Vec<NbCount> = wanted.iter().map(|(word, n)| counted(word, *n)).collect();
            checknbshapes_named(script.session(), name, Counting::Typed, &expected, message);
            Ok(())
        }
        _ => unreachable!("run_check dispatches the other checks"),
    }
}

fn check_counts(
    script: &mut FilletScript<'_>,
    name: &str,
    counting: Counting,
    wanted: &[(&'static str, usize)],
) -> Result<(), Failure> {
    script.required(name)?;
    let expected: Vec<NbCount> = wanted.iter().map(|(word, n)| counted(word, *n)).collect();
    checknbshapes(script.session(), name, counting, &expected);
    Ok(())
}

/// The count of one `-vertex n` style argument, by the word the tables keep.
fn counted(word: &str, count: usize) -> NbCount {
    match word {
        "VERTEX" => NbCount::Vertices(count),
        "EDGE" => NbCount::Edges(count),
        "WIRE" => NbCount::Wires(count),
        "FACE" => NbCount::Faces(count),
        "SHELL" => NbCount::Shells(count),
        "SOLID" => NbCount::Solids(count),
        "COMPSOLID" => NbCount::CompSolids(count),
        "COMPOUND" => NbCount::Compounds(count),
        "SHAPE" => NbCount::Shapes(count),
        _ => panic!("checknbshapes: unknown entity {word}"),
    }
}

fn check_props(
    script: &mut FilletScript<'_>,
    (kind, name): (Prop, &str),
    value: f64,
    deviation: f64,
) {
    let tolerances = CheckTolerances {
        deviation,
        ..CheckTolerances::default()
    };
    checkprops_within(
        script.session(),
        name,
        kind,
        Expected::Mass(value),
        tolerances,
    );
}

/// The checks of the shape's largest tolerance, as the scripts write them.
fn run_tolerance_check(script: &mut FilletScript<'_>, check: &Check) -> Result<(), Failure> {
    match check {
        Check::MaxTolerance(name, limit, message) => {
            print_when(largest_tolerance(script, name)? > *limit, message);
        }
        Check::MaxToleranceRef(name, reference) => {
            script.required(name)?;
            checkmaxtol_ref(script.session(), name, reference);
        }
        Check::MaxToleranceMin(name, limit, written) => {
            checkmaxtol_min(script, name, *limit, written)?;
        }
        Check::ToleranceBranch(name, limit, over, under) => {
            tolerance_branch(script, name, *limit, [over, under])?;
        }
        _ => unreachable!("run_check dispatches the other checks"),
    }
    Ok(())
}

/// `regexp {Tolerance +MAX=...} [tolerance name]`, its value written, then the script's branch.
fn tolerance_branch(
    script: &mut FilletScript<'_>,
    name: &str,
    limit: f64,
    [over, under]: [&str; 2],
) -> Result<(), Failure> {
    let value = largest_tolerance(script, name)?;
    println!("MaxTolerance = {}", format_general(value, 17));
    println!("{}", if value > limit { over } else { under });
    Ok(())
}

/// What `checkmaxtol name` returns: the first number `tolerance name` writes.
fn largest_tolerance(script: &mut FilletScript<'_>, name: &str) -> Result<f64, Failure> {
    script.required(name)?;
    let text = tolerance_report(script.session(), name);
    Ok(text.parse().expect("tolerance writes a number"))
}

/// `checkmaxtol name -min_tol limit`, whose message prints the limit as the script writes it.
fn checkmaxtol_min(
    script: &mut FilletScript<'_>,
    name: &str,
    limit: f64,
    written: &str,
) -> Result<(), Failure> {
    script.required(name)?;
    let maximal = tolerance_report(script.session(), name);
    if maximal.parse::<f64>().expect("tolerance writes a number") > limit {
        println!(
            "Error: tolerance of \"{name}\" ({maximal}) is greater than checking tolerance ({written})"
        );
    }
    Ok(())
}

fn print_when(condition: bool, message: &str) {
    if condition {
        println!("{message}");
    }
}

/// The checks a script writes as `if ... { puts ... }`.
fn run_else_check(script: &mut FilletScript<'_>, check: &Check) -> Result<(), Failure> {
    match check {
        Check::ValidElse(name, message) => {
            print_when(!checkshape(&script.required(name)?).is_valid(), message);
        }
        Check::BopcheckElse(name, message) => {
            let response = bopcheck(&script.required(name)?, &BopOptions::default());
            print_when(!response.contains("This shape seems to be OK"), message);
        }
        Check::ArgcheckElse(name, message) => {
            let shape = script.required(name)?;
            print_when(
                bopargcheck_faulties(&shape, &BopOptions::default()),
                message,
            );
        }
        _ => run_bound_check(script, check)?,
    }
    Ok(())
}

/// The checks that read what a variable holds rather than a command's answer.
fn run_bound_check(script: &mut FilletScript<'_>, check: &Check) -> Result<(), Failure> {
    match check {
        Check::Section(name) => {
            checksection(script.session(), name, None);
        }
        Check::BoundElse(name, message) => print_when(script.shape(name).is_null(), message),
        Check::BoundBranch(name, missing, present) => {
            let found = !script.shape(name).is_null();
            println!("{}", if found { present } else { missing });
        }
        Check::VKnotMultiplicities(edge, wanted, message) => {
            let edge = script.required(edge)?;
            print_when(v_multiplicities_match(&edge, wanted)?, message);
        }
        _ => unreachable!("run_check dispatches the other checks"),
    }
    Ok(())
}

/// The `dump` of an edge lists its first B-spline surface first: its `VKnots` lines are what
/// the script reads, as `index : knot multiplicity`.
fn v_multiplicities_match(edge: &Shape, wanted: &[(i32, i32)]) -> Result<bool, Failure> {
    let edge = topods::edge(edge)?;
    for rank in 1.. {
        let on_surface = BRepTool::curve_on_surface_by_index(&edge, rank)?;
        if on_surface.pcurve.is_null() {
            return Ok(false);
        }
        let surface = on_surface.surface.clone().cast::<GeomBSplineSurface>();
        if let Some(bspline) = surface.get() {
            for (index, multiplicity) in wanted {
                if bspline.v_multiplicity(*index)? != *multiplicity {
                    return Ok(false);
                }
            }
            return Ok(true);
        }
    }
    Ok(false)
}

/// A null shape makes `checkshape` return 1, which stops the native script.
fn check_shape(script: &FilletScript<'_>, name: &str) -> Result<(), Failure> {
    let shape = script.shape(name);
    if shape.is_null() {
        // The command's own output comes first; the stop then writes the same words.
        let message = format!("{name} is not a topological shape!!!");
        println!("{message}");
        return Err(command_stopped(&message));
    }
    checkshape(&shape);
    Ok(())
}

fn run_chamfer(script: &mut FilletScript<'_>, chamfer: &Chamfer) -> Result<(), Failure> {
    let (result, shape) = match chamfer {
        Chamfer::Mixed(r, s, _) | Chamfer::Throat(r, s, _) | Chamfer::Penetration(r, s, _) => {
            (*r, *s)
        }
    };
    let mut maker = BRepFilletAPIMakeChamfer::new(&script.required(shape)?)?;
    add_chamfer_edges(script, &mut maker, chamfer)?;
    if maker.nb_contours() == 0 {
        return Err(command_stopped("No suitable edges to chamfer"));
    }
    maker.build(&ProgressRange::default())?;
    if !maker.is_done() {
        return Err(command_stopped("compute of chamfer failed"));
    }
    script.set_shape(result, maker.shape()?.clone());
    Ok(())
}

fn add_chamfer_edges(
    script: &FilletScript<'_>,
    maker: &mut BRepFilletAPIMakeChamfer,
    chamfer: &Chamfer,
) -> Result<(), Failure> {
    match chamfer {
        Chamfer::Mixed(_, _, edges) => add_mixed(script, maker, edges),
        Chamfer::Throat(_, _, edges) => add_throats(script, maker, edges),
        Chamfer::Penetration(_, _, edges) => add_penetrations(script, maker, edges),
    }
}

fn add_mixed(
    script: &FilletScript<'_>,
    maker: &mut BRepFilletAPIMakeChamfer,
    edges: &[ChamferEdge],
) -> Result<(), Failure> {
    let confusion = precision::confusion();
    for ChamferEdge::DistanceAngle(edge, face, d, degrees) in edges {
        let (edge, face) = (script.required(edge)?, script.required(face)?);
        let edge = topods::edge(&edge)?;
        let angle = degrees * PI / 180.0;
        let valid = *d > confusion && angle > confusion && PI / 2.0 - angle > confusion;
        if maker.contour(&edge) == 0 && valid {
            maker.add_dist_angle(*d, angle, &edge, &topods::face(&face)?)?;
        }
    }
    Ok(())
}

fn add_throats(
    script: &FilletScript<'_>,
    maker: &mut BRepFilletAPIMakeChamfer,
    edges: &[(&str, f64)],
) -> Result<(), Failure> {
    maker.set_mode(ChamfMode::ConstThroatChamfer);
    for (edge, throat) in edges {
        let edge = topods::edge(&script.required(edge)?)?;
        if maker.contour(&edge) == 0 && *throat > precision::confusion() {
            maker.add_with_dist(*throat, &edge)?;
        }
    }
    Ok(())
}

fn add_penetrations(
    script: &FilletScript<'_>,
    maker: &mut BRepFilletAPIMakeChamfer,
    edges: &[(&str, &str, f64, f64)],
) -> Result<(), Failure> {
    maker.set_mode(ChamfMode::ConstThroatWithPenetrationChamfer);
    for (edge, face, offset, throat) in edges {
        let edge = topods::edge(&script.required(edge)?)?;
        let face = topods::face(&script.required(face)?)?;
        if maker.contour(&edge) == 0 && *offset > precision::confusion() && throat > offset {
            maker.add_with_dists(*offset, *throat, &edge, &face)?;
        }
    }
    Ok(())
}

fn run_history(script: &mut FilletScript<'_>, query: &HistoryQuery) -> Result<(), Failure> {
    match query {
        HistoryQuery::Save => {
            script.saved_history = script.last_history.clone();
            Ok(())
        }
        HistoryQuery::Generated(target, source) => history_query(script, target, source, true),
        HistoryQuery::Modified(target, source) => history_query(script, target, source, false),
    }
}

fn history_query(
    script: &mut FilletScript<'_>,
    target: &'static str,
    source: &str,
    generated: bool,
) -> Result<(), Failure> {
    let saved: Option<Handle<History>> = script.saved_history.clone();
    let history = saved.ok_or_else(|| command_stopped("no saved history"))?;
    let shape = script.required(source)?;
    let live = history
        .get()
        .ok_or_else(|| command_stopped("null history"))?;
    let shapes = if generated {
        live.generated(&shape)?
    } else {
        live.modified(&shape)?
    };
    publish_history_result(script, target, &shapes)
}

/// One shape as itself, several in a compound; nothing leaves the variable alone.
fn publish_history_result(
    script: &mut FilletScript<'_>,
    target: &'static str,
    shapes: &[Shape],
) -> Result<(), Failure> {
    match shapes {
        [] => {}
        [only] => script.set_shape(target, only.clone()),
        many => {
            let builder = BRepBuilder::default();
            let mut compound = Compound::default();
            builder.make_compound(&mut compound);
            for shape in many {
                builder.add(&mut compound, shape)?;
            }
            script.set_shape(target, compound.into_shape());
        }
    }
    Ok(())
}
