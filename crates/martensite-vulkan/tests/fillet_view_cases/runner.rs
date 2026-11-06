//! Runs one case the way the native test command does: the group's `begin`, the commands, and
//! the group's `end` unless a command stopped the script. The contract's child process and
//! whole-stream grading do the rest.
use crate::bug_steps::BugCommands;
use crate::fillet_steps::FilletCommands;
use crate::script::{COMMAND_RETURNED_ONE, FilletScript};
use crate::steps::{BlendCase, Step};
use crate::support::{
    checkview::{checkview, screenshot},
    hooks::CaseGroup,
    process::verdict_of,
    raster_session::{KEEP_IMAGES_ENV, RasterSession},
    rules::Verdict,
};
use martensite_tkernel::standard::Failure;

/// Starts the line of a script stopped by an error, in the contract's words.
const EXCEPTION_PREFIX: &str = "ScriptStep Exception: ";

/// Run a case in its child process and fail the test unless its stream earns the verdict native
/// has. Example: `run_listed_case("cases::simple_1::simple_a1", &case)`.
pub fn run_listed_case(identity: &str, case: &BlendCase) {
    let group = case.group.case_group();
    let body = || run_in_child(identity, case, &group);
    let verdict = verdict_of(identity, &group, body)
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    let Some(verdict) = verdict else {
        return;
    };
    keep_verdict_for_inspection(identity, &verdict);
    let earned = match case.native_improvement {
        Some(todo) => matches!(&verdict, Verdict::Improvement(text) if text.contains(todo)),
        None => verdict.passes(),
    };
    assert!(
        earned,
        "case {identity}: {verdict:?}, native improvement {:?}",
        case.native_improvement
    );
}

/// Writes the verdict beside the kept stream, so a run can be compared with the native verdicts.
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

fn run_in_child(identity: &str, case: &BlendCase, group: &CaseGroup) {
    let mut session = RasterSession::new(&identity.replace("::", "_"));
    (group.begin)(&mut session);
    println!("Executing {}...", case.source);
    let finished = run_steps(&mut session, case.steps);
    if finished {
        (group.end)(&mut session);
    }
}

/// The commands in order; `false` when one stopped the script, and the end scripts do not run.
fn run_steps(session: &mut RasterSession, steps: &[Step]) -> bool {
    let mut script = FilletScript::new(session);
    let mut commands = FilletCommands::default();
    for step in steps {
        if let Err(failure) = apply_step(&mut script, &mut commands, step) {
            println!("{}", stop_text(&failure));
            return false;
        }
    }
    crate::v3d_steps::finish();
    true
}

fn apply_step(
    script: &mut FilletScript<'_>,
    commands: &mut FilletCommands,
    step: &Step,
) -> Result<(), Failure> {
    if script.apply_input_step(step)? || BugCommands.apply(script, step)? {
        return Ok(());
    }
    match step {
        Step::Caught(inner, failed, done) => apply_caught(script, commands, inner, [failed, done]),
        Step::IfDone(inner, failed, then) => apply_if_done(script, commands, inner, failed, then),
        Step::V3d(view) => crate::v3d_steps::apply(script, view),
        Step::CheckView(_) | Step::Screenshot(_) => {
            capture(script, step);
            Ok(())
        }
        _ => commands.apply(script, step),
    }
}

fn capture(script: &mut FilletScript<'_>, step: &Step) {
    match step {
        Step::CheckView(view) => checkview(script.session(), view),
        Step::Screenshot(image) => screenshot(script.session(), image),
        _ => unreachable!("apply_step sends only the capture steps"),
    }
}

/// `if {[catch {inner}]} {puts failed} else {puts done}`.
fn apply_caught(
    script: &mut FilletScript<'_>,
    commands: &mut FilletCommands,
    inner: &Step,
    [failed, done]: [&str; 2],
) -> Result<(), Failure> {
    let outcome = apply_step(script, commands, inner);
    print_unless_empty(if outcome.is_ok() { done } else { failed });
    Ok(())
}

/// `if {[catch {inner}]} {puts failed} else {then}`.
fn apply_if_done(
    script: &mut FilletScript<'_>,
    commands: &mut FilletCommands,
    inner: &Step,
    failed: &str,
    then: &[Step],
) -> Result<(), Failure> {
    if apply_step(script, commands, inner).is_err() {
        print_unless_empty(failed);
        return Ok(());
    }
    then.iter()
        .try_for_each(|next| apply_step(script, commands, next))
}

fn print_unless_empty(text: &str) {
    if !text.is_empty() {
        println!("{text}");
    }
}

/// The text a stop leaves: a command returning 1 gives its result, an exception names itself.
fn stop_text(failure: &Failure) -> String {
    let detail = String::from_utf8_lossy(failure.what()).into_owned();
    if failure.is_kind(COMMAND_RETURNED_ONE) {
        return format!("{EXCEPTION_PREFIX}{detail}");
    }
    format!("** Exception ** {detail}\n{EXCEPTION_PREFIX}** Exception ** {detail}")
}
