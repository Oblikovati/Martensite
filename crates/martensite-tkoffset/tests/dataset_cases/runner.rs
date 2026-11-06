//! Runs one case the way the reference test command does: the group's `begin`, the commands, and the
//! end script unless a command stopped the script. The contract's child process and whole-stream
//! grading do the rest.
use crate::conditions::{Cond, value_of};
use crate::groups::run_end;
use crate::script::{COMMAND_RETURNED_ONE, OffsetScript};
use crate::steps::{Check, OffsetCase, Step};
use crate::support::{
    checkprops::{CheckTolerances, Expected, checkprops_equal, checkprops_within},
    checkshape::{checkshape, checkshape_named},
    checkview::{CheckView, checkview, screenshot},
    nbshapes::{
        Counting, NbCount, checknbshapes, checknbshapes_like, checknbshapes_named, nbshapes,
    },
    process::verdict_of,
    raster_session::{KEEP_IMAGES_ENV, RasterSession},
    rules::{EXCEPTION_PREFIX, Verdict},
    section_check::checksection,
    tolerance_check::checkmaxtol,
};
use martensite_boolean_cases::support::{
    bop_session::BopOptions, named_checks::bopargcheck_faulties,
};
use martensite_tkernel::standard::Failure;

/// A case whose reference run never ends under its own CPU limit: this one must not either.
pub const STOPS_AT_CPU_LIMIT: &str = "stops at its CPU limit";

/// Run a case in its child process and fail the test unless its stream earns the verdict the
/// reference run has. Example: `run_listed_case("cases::draft_1::draft_angle_a1", &case)`.
pub fn run_listed_case(identity: &str, case: &OffsetCase) {
    let rules = case.group.rules();
    let body = || run_in_child(identity, case);
    let outcome = verdict_of(identity, case.cpu_seconds, rules, body);
    if case.reference_improvement == Some(STOPS_AT_CPU_LIMIT) {
        let failure = outcome.expect_err("the reference run does not end before its CPU limit");
        assert!(
            failure.to_string().contains("code=Some(2)"),
            "case {identity}: {failure}; expected the CPU limit's exit"
        );
        return;
    }
    let verdict = outcome.unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    let Some(verdict) = verdict else {
        return;
    };
    keep_verdict_for_inspection(identity, &verdict);
    let earned = match case.reference_improvement {
        Some(todo) => matches!(&verdict, Verdict::Improvement(text) if text.contains(todo)),
        None if case.reference_clean => matches!(verdict, Verdict::Ok),
        None => verdict.passes(),
    };
    assert!(
        earned,
        "case {identity}: {verdict:?}, reference improvement {:?}",
        case.reference_improvement
    );
}

/// Writes the verdict beside the kept stream, so a run can be compared with the reference verdicts.
fn keep_verdict_for_inspection(identity: &str, verdict: &Verdict) {
    if let Some(folder) = std::env::var_os(KEEP_IMAGES_ENV) {
        let file = identity.replace("::", "_") + ".verdict.txt";
        std::fs::write(
            std::path::PathBuf::from(folder).join(file),
            format!("{verdict:?}"),
        )
        .unwrap();
    }
}

fn run_in_child(identity: &str, case: &OffsetCase) {
    let mut session = RasterSession::new(&identity.replace("::", "_"));
    let budget = martensite_process_policy::reported_process_limit();
    assert_eq!(budget, case.cpu_seconds, "begin: CPU budget");
    println!("Executing {}...", case.source);
    let mut script = OffsetScript::new(&mut session);
    crate::support::script_error::run_script(|| {
        if run_steps(&mut script, case.steps) {
            run_end(&mut script, &case.end);
        }
    });
}

/// The commands in order; `false` when one stopped the script, and the end scripts do not run.
fn run_steps(script: &mut OffsetScript<'_>, steps: &[Step]) -> bool {
    for step in steps {
        if let Err(failure) = apply_step(script, step) {
            println!("{}", stop_text(&failure));
            return false;
        }
    }
    true
}

/// Runs one step; exposed to the end scripts through their own commands.
pub fn apply_step(script: &mut OffsetScript<'_>, step: &Step) -> Result<(), Failure> {
    if script.apply_input_step(step)? {
        return Ok(());
    }
    match step {
        Step::Bool(operation) => crate::boolean_steps::apply(script, operation),
        Step::Loft(loft) => crate::sweep_steps::loft(script, loft),
        Step::Shell(shell) => crate::sweep_steps::shell(script, shell),
        Step::Draft(draft) => crate::sweep_steps::draft(script, draft),
        Step::Offset(offset) => crate::offset_steps::apply(script, offset),
        Step::More(more) => crate::more_steps::apply(script, more),
        Step::Fillet(fillet) => {
            let mut commands = std::mem::take(&mut script.fillets);
            let outcome = commands.fillet(script, fillet);
            script.fillets = commands;
            outcome
        }
        Step::Check(check) => {
            apply_check(script, check);
            Ok(())
        }
        Step::Puts(text) => {
            println!("{text}");
            Ok(())
        }
        Step::If(condition, then, otherwise) => {
            let branch = if holds(script, condition) {
                then
            } else {
                otherwise
            };
            branch.iter().try_for_each(|next| apply_step(script, next))
        }
        Step::Let(name, value) => {
            let kept = value_of(script, value);
            script.values.keep(name, kept);
            Ok(())
        }
        Step::Catch(inner) => {
            if let Err(failure) = apply_step(script, inner) {
                println!("{}", caught_text(&failure));
            }
            Ok(())
        }
        Step::CheckView(view) => {
            checkview(script.session(), &CheckView::of(view));
            Ok(())
        }
        Step::CheckViewOtherwise(view, others, kind) => {
            checkview(
                script.session(),
                &CheckView::of(view).otherwise(others, *kind),
            );
            Ok(())
        }
        Step::Screenshot(image) => {
            screenshot(script.session(), image);
            Ok(())
        }
        Step::Block(steps) => steps.iter().try_for_each(|next| apply_step(script, next)),
        Step::CatchKept(name, inner) => {
            let text = match apply_step(script, inner) {
                Ok(()) => String::new(),
                Err(failure) => {
                    let text = caught_text(&failure);
                    println!("{text}");
                    text
                }
            };
            script.values.keep(name, text);
            Ok(())
        }
        Step::Extra(extra) => crate::extra_steps::apply(script, extra),
        Step::Run(run) => run(script),
        Step::Stop(text) => Err(crate::script::command_stopped(text)),
        _ => unreachable!("apply_input_step takes the other steps"),
    }
}

fn holds(script: &mut OffsetScript<'_>, condition: &Cond) -> bool {
    match condition {
        Cond::Bound(name) => script.is_bound(name),
        Cond::Unbound(name) => !script.is_bound(name),
        Cond::Failed(step) => match apply_step(script, step) {
            Ok(()) => false,
            Err(failure) => {
                println!("{}", caught_text(&failure));
                true
            }
        },
        Cond::KindHolds(name, text) => {
            let kind = script
                .shape(name)
                .shape_type()
                .map(|kind| format!("{kind:?}").to_uppercase());
            kind.is_ok_and(|kind| kind.contains(text))
        }
        Cond::KindLacks(name, text) => {
            let kind = script.shape(name).shape_type();
            !kind.is_ok_and(|kind| format!("{kind:?}").to_uppercase().contains(text))
        }
        Cond::NullShape(name) => script.shape(name).is_null(),
        Cond::Below(a, b, shift) => number_kept(script, a) + shift < number_kept(script, b),
        Cond::NotBelow(a, b) => number_kept(script, a) >= number_kept(script, b),
        Cond::Above(name, limit) => number_kept(script, name) > *limit,
        Cond::Differs(name, value) => number_kept(script, name) != *value,
        Cond::ArgumentFaulties(name) => {
            bopargcheck_faulties(&script.shape(name), &BopOptions::default())
        }
        Cond::Holds(name, text) => script.values.text(name).contains(text),
        Cond::Lacks(name, text) => !script.values.text(name).contains(text),
    }
}

fn number_kept(script: &OffsetScript<'_>, name: &str) -> f64 {
    let text = script.values.text(name);
    text.parse()
        .unwrap_or_else(|_| panic!("${name} is {text:?}; expected a number"))
}

fn apply_check(script: &mut OffsetScript<'_>, check: &Check) {
    let session = script.session();
    match check {
        Check::Shape(name) => checkshape_named(session, name),
        Check::ShapePrinted(name) => {
            let shape = session.shape(name).cloned().unwrap_or_default();
            println!("{}", checkshape(&shape).text);
        }
        Check::Counts(name, expected) => {
            let counts: Vec<NbCount> = expected.iter().map(nb_count).collect();
            checknbshapes(session, name, Counting::Located, &counts);
        }
        Check::TypedCounts(name, expected) => {
            let counts: Vec<NbCount> = expected.iter().map(nb_count).collect();
            checknbshapes(session, name, Counting::Typed, &counts);
        }
        Check::TypedCountsNamed(name, expected, message) => {
            let counts: Vec<NbCount> = expected.iter().map(nb_count).collect();
            checknbshapes_named(session, name, Counting::Typed, &counts, message);
        }
        Check::CountsLike(name, other) => checknbshapes_like(session, name, other),
        Check::Props(kind, name, value, deviation, epsilon) => {
            let tolerances = CheckTolerances {
                epsilon: epsilon.unwrap_or(crate::support::checkprops::CHECKPROPS_EPSILON),
                deviation: *deviation,
            };
            checkprops_within(session, name, *kind, Expected::Mass(*value), tolerances);
        }
        Check::PropsAs(kind, name, value, word, deviation, epsilon) => {
            let tolerances = CheckTolerances {
                epsilon: epsilon.unwrap_or(crate::support::checkprops::CHECKPROPS_EPSILON),
                deviation: *deviation,
            };
            checkprops_within(
                session,
                name,
                *kind,
                Expected::Word(*value, word),
                tolerances,
            );
        }
        Check::NbShapes(name) => {
            nbshapes(session, name, Counting::Located);
        }
        Check::CountsNamed(name, expected, message) => {
            let counts: Vec<NbCount> = expected.iter().map(nb_count).collect();
            checknbshapes_named(session, name, Counting::Located, &counts, message);
        }
        Check::PropsEmpty(kind, name) => {
            checkprops_within(
                session,
                name,
                *kind,
                Expected::Empty,
                CheckTolerances::default(),
            );
        }
        Check::PropsEqual(name, other) => checkprops_equal(session, name, other),
        Check::MaxTolerance(name, reference, minimum) => {
            checkmaxtol(session, name, *reference, *minimum);
        }
        Check::Section(name) => checksection(session, name),
        Check::ClosedElse(name, message) => closed_else(script, name, message),
    }
}

fn nb_count((kind, value): &(&str, usize)) -> NbCount {
    match *kind {
        "VERTEX" => NbCount::Vertices(*value),
        "EDGE" => NbCount::Edges(*value),
        "WIRE" => NbCount::Wires(*value),
        "FACE" => NbCount::Faces(*value),
        "SHELL" => NbCount::Shells(*value),
        "SOLID" => NbCount::Solids(*value),
        "COMPSOLID" => NbCount::CompSolids(*value),
        "COMPOUND" => NbCount::Compounds(*value),
        "SHAPE" => NbCount::Shapes(*value),
        other => panic!("checknbshapes: {other}; expected a shape kind"),
    }
}

/// `set index [lsearch [whatis name] Closed]; if {$index == -1} {puts message}`.
fn closed_else(script: &mut OffsetScript<'_>, name: &str, message: &str) {
    let shape = script.shape(name);
    let closed = shape.tshape().get().is_some_and(|tshape| tshape.closed());
    if !closed {
        println!("{message}");
    }
}

/// What a failure under `catch` still writes: the interpreter's report, with no script stop.
fn caught_text(failure: &Failure) -> String {
    let detail = String::from_utf8_lossy(failure.what()).into_owned();
    if failure.is_kind(COMMAND_RETURNED_ONE) {
        return detail;
    }
    format!(
        "An exception was caught {}: {detail}",
        failure.exception_type()
    )
}

/// The text a stop leaves: a command returning 1 gives its result, an exception is written as the
/// interpreter writes one, with its type.
fn stop_text(failure: &Failure) -> String {
    let detail = String::from_utf8_lossy(failure.what()).into_owned();
    if failure.is_kind(COMMAND_RETURNED_ONE) {
        return format!("{EXCEPTION_PREFIX}{detail}");
    }
    let caught = format!(
        "An exception was caught {}: {detail}",
        failure.exception_type()
    );
    format!("{caught}\n{EXCEPTION_PREFIX}{caught}")
}
