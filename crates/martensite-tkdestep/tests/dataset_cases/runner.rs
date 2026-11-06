//! Runs one case the way the test command does: a child process under the group's limit, the
//! whole stream graded by the group's `parse.rules`, the verdict held against the reference run's.
use crate::support::{
    process::verdict_of,
    raster_session::KEEP_IMAGES_ENV,
    rules::{Rule, RuleMatch, RuleStatus, Verdict},
    script_error::run_script,
};
use crate::table::{DeRow, RecordedVerdict, StepCase};
use crate::xs_session::XsSession;

fn is_faulty(word: &str) -> bool {
    word == "Faulty"
}

const fn ignoring(text: &'static str) -> Rule {
    Rule {
        status: RuleStatus::Ignore,
        matching: RuleMatch::Contains(text),
    }
}

/// `tests/de/parse.rules`.
pub const DE_RULES: &[Rule] = &[
    Rule {
        status: RuleStatus::Failed,
        matching: RuleMatch::Word(is_faulty),
    },
    ignoring("Software error"),
    ignoring("Syntax error"),
    ignoring("Transfer error"),
    ignoring("Operator FixShape failed with exception"),
    ignoring("Error on writing file"),
    ignoring("StepFile Error"),
    ignoring("StepToTopoDS, GeometricSet"),
    ignoring("an access violation"),
    ignoring("Exception Raised while reading Step File"),
    Rule {
        status: RuleStatus::Ignore,
        matching: RuleMatch::Line(is_fix_shape_exception),
    },
];

/// `/Warning: ShapeFix_.* Exception/`.
fn is_fix_shape_exception(line: &str) -> bool {
    line.split_once("Warning: ShapeFix_")
        .is_some_and(|(_, rest)| rest.contains(" Exception"))
}

fn is_locale_skip(line: &str) -> bool {
    line.contains("Error: unsupported locale specification")
}

/// The `de` group's own `cpulimit`.
const DE_CPU_SECONDS: i64 = 1000;

/// Runs a `de/step_*` row; the test passes when its stream earns the reference's verdict.
/// Example: `run_de_case("cases::de_step_1::de_step_1_a7", &row)`.
pub fn run_de_case(identity: &str, row: &DeRow) {
    let body = || {
        println!("Executing {}...", row.source);
        let mut session = XsSession::for_source(&identity.replace("::", "_"), row.source);
        run_script(|| crate::de_end::run(&mut session, row));
    };
    let verdict = verdict_of(identity, DE_CPU_SECONDS, DE_RULES, body)
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    if let Some(verdict) = verdict {
        hold_against_reference(identity, &verdict, row.recorded);
    }
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

/// `tests/metadata/parse.rules`: the same lines as `tests/de/parse.rules`.
const METADATA_RULES: &[Rule] = DE_RULES;

/// The `gdt` and `persist` groups have no rules of their own.
const GDT_RULES: &[Rule] = &[];

/// Runs a `gdt` row against the reference's verdict.
/// Example: `run_gdt_case("cases::gdt_import::gdt_import_a6", &row)`.
pub fn run_gdt_case(identity: &str, row: &crate::table::GdtRow) {
    let body = || {
        println!("Executing {}...", row.source);
        let mut session = XsSession::for_source(&identity.replace("::", "_"), row.source);
        run_script(|| crate::gdt_end::run(&mut session, row));
    };
    let verdict = verdict_of(identity, DE_CPU_SECONDS, GDT_RULES, body)
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    if let Some(verdict) = verdict {
        hold_against_reference(identity, &verdict, row.recorded);
    }
}

/// Runs a `metadata/step` row against the reference's verdict.
/// Example: `run_metadata_case("cases::metadata_step::metadata_step_a1", &row)`.
pub fn run_metadata_case(identity: &str, row: &crate::table::MetadataRow) {
    let body = || {
        println!("Executing {}...", row.source);
        let mut session = XsSession::for_source(&identity.replace("::", "_"), row.source);
        run_script(|| crate::metadata_end::run(&mut session, row));
    };
    let verdict = verdict_of(identity, DE_CPU_SECONDS, METADATA_RULES, body)
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    if let Some(verdict) = verdict {
        hold_against_reference(identity, &verdict, row.recorded);
    }
}

/// Runs a script with commands of its own against the reference's verdict.
/// Example: `run_step_case("cases::bugs_step::bugs_step_bug23", &case)`.
pub fn run_step_case(identity: &str, case: &StepCase) {
    let body = || {
        println!("Executing {}...", case.source);
        let mut session = XsSession::for_source(&identity.replace("::", "_"), case.source);
        run_script(|| {
            if let Some(seconds) = case.cpu_seconds {
                println!("CPU and elapsed time limit set to {seconds} seconds");
            }
            crate::steps::run_steps(&mut session, case);
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

/// The `bugs` groups' own `cpulimit`.
const BUGS_CPU_SECONDS: i64 = 1000;

/// Runs a hand-written script against the reference's verdict.
/// Example: `run_one_off("one_off::bugs_step_bug222", "tests/bugs/step/bug222", RecordedVerdict::Ok, bug222)`.
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
            println!("CPU and elapsed time limit set to {BUGS_CPU_SECONDS} seconds");
            for line in crate::cases::headings::heading(source) {
                println!("{line}");
            }
            script(&mut session);
            // A `gdt` script's own `end` writes it, before the group's `end`.
            if !source.starts_with("tests/gdt") {
                println!("TEST COMPLETED");
            }
        });
    };
    let rules = if source.starts_with("tests/gdt") || source.starts_with("tests/persist") {
        GDT_RULES
    } else {
        BUGS_RULES
    };
    let verdict = verdict_of(identity, BUGS_CPU_SECONDS, rules, body)
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    if let Some(verdict) = verdict {
        hold_against_reference(identity, &verdict, recorded);
    }
}

/// Runs a script up to its out-of-scope step, against the reference's verdict for what that part writes.
/// Example: `run_partly("partial_cases::bugs_step_bug51", "tests/bugs/step/bug51", bug51)`.
pub fn run_partly(identity: &str, source: &str, script: fn(&mut XsSession)) {
    let body = || {
        println!("Executing {source}...");
        let mut session = XsSession::for_source(&identity.replace("::", "_"), source);
        run_script(|| {
            script(&mut session);
            println!("TEST COMPLETED");
        });
    };
    let rules = if source.starts_with("tests/persist") {
        GDT_RULES
    } else {
        BUGS_RULES
    };
    let verdict = verdict_of(identity, BUGS_CPU_SECONDS, rules, body)
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    if let Some(verdict) = verdict {
        hold_against_reference(identity, &verdict, RecordedVerdict::Ok);
    }
}

/// The scripts of `de_wrapper` set no limit of their own; this one only ends a hung run.
const NO_LIMIT_SECONDS: i64 = 3600;

fn hold_against_reference(identity: &str, verdict: &Verdict, recorded: RecordedVerdict) {
    keep_verdict_for_inspection(identity, verdict);
    let earned = match recorded {
        RecordedVerdict::Ok => matches!(verdict, Verdict::Ok),
        RecordedVerdict::Known => matches!(verdict, Verdict::KnownProblem),
        RecordedVerdict::Improvement => matches!(verdict, Verdict::Improvement(_)),
        RecordedVerdict::Failed => matches!(verdict, Verdict::Failed(_)),
    };
    assert!(
        earned,
        "case {identity}: {verdict:?}, reference {recorded:?}"
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
