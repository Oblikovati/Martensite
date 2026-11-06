//! Runs one case the way the test command does: a child process under the group's limit, the
//! whole stream graded by the group's `parse.rules`, the verdict held against the reference run's.
use crate::support::{
    process::verdict_of,
    raster_session::KEEP_IMAGES_ENV,
    rules::{Rule, RuleMatch, RuleStatus, Verdict},
    script_error::run_script,
};
use crate::table::{RecordedVerdict, ScriptCase};
use crate::xs_session::XsSession;

fn is_faulty(word: &str) -> bool {
    word == "Faulty"
}

fn is_locale_skip(line: &str) -> bool {
    line.contains("Error: unsupported locale specification")
}

/// `tests/bugs/parse.rules`, `tests/de_wrapper/parse.rules`.
pub const BUGS_RULES: &[Rule] = &[
    Rule {
        status: RuleStatus::Failed,
        matching: RuleMatch::Word(is_faulty),
    },
    Rule {
        status: RuleStatus::Failed,
        matching: RuleMatch::Contains("failure reading attribute"),
    },
    Rule {
        status: RuleStatus::Skipped,
        matching: RuleMatch::Line(is_locale_skip),
    },
    Rule {
        status: RuleStatus::Ok,
        matching: RuleMatch::Contains("Relative error of mass computation"),
    },
];

/// The `bugs` groups' own `cpulimit`.
pub const BUGS_CPU_SECONDS: i64 = 1000;
/// The scripts of `de_wrapper` set no limit of their own; this one only ends a hung run.
const NO_LIMIT_SECONDS: i64 = 3600;

/// Runs a script of the tables against the reference's verdict.
/// Example: `run_script_case("cases::bugs_iges::bug131_1", &case)`.
pub fn run_script_case(identity: &str, case: &ScriptCase) {
    let body = || {
        println!("Executing {}...", case.source);
        let mut session = XsSession::for_source(&identity.replace("::", "_"), case.source);
        run_script(|| {
            if let Some(seconds) = case.cpu_seconds {
                println!("CPU and elapsed time limit set to {seconds} seconds");
            }
            crate::steps::run_steps(&mut session, case.steps);
            println!("TEST COMPLETED");
        });
    };
    let cpu = case.cpu_seconds.unwrap_or(NO_LIMIT_SECONDS);
    let verdict = verdict_of(identity, cpu, BUGS_RULES, body)
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    if let Some(verdict) = verdict {
        hold_against_reference(identity, &verdict, case.recorded);
    }
}

/// Runs a hand-written script against the reference's verdict.
/// Example: `run_one_off("one_off::bug23018", "tests/bugs/iges/bug23018", RecordedVerdict::Ok, bug23018)`.
pub fn run_one_off(
    identity: &str,
    source: &str,
    recorded: RecordedVerdict,
    script: fn(&mut XsSession),
) {
    let body = || {
        println!("Executing {source}...");
        let mut session = XsSession::for_source(&identity.replace("::", "_"), source);
        run_script(|| {
            if !source.starts_with("tests/de_wrapper") {
                println!("CPU and elapsed time limit set to {BUGS_CPU_SECONDS} seconds");
            }
            for line in crate::cases::headings::heading(source) {
                println!("{line}");
            }
            script(&mut session);
            println!("TEST COMPLETED");
        });
    };
    let verdict = verdict_of(identity, BUGS_CPU_SECONDS, BUGS_RULES, body)
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    if let Some(verdict) = verdict {
        hold_against_reference(identity, &verdict, recorded);
    }
}

/// The verdict against the reference's, kept beside the stream.
pub fn hold_against_reference(identity: &str, verdict: &Verdict, recorded: RecordedVerdict) {
    keep_verdict_for_inspection(identity, verdict);
    let earned = match recorded {
        RecordedVerdict::Ok => matches!(verdict, Verdict::Ok),
        RecordedVerdict::Known => matches!(verdict, Verdict::KnownProblem),
        RecordedVerdict::Failed => matches!(verdict, Verdict::Failed(_)),
    };
    assert!(
        earned,
        "case {identity}: {verdict:?}, reference {recorded:?}"
    );
}

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
