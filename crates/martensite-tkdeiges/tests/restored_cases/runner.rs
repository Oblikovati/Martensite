//! Runs one case the way the test command does: a child process under the group's limit, the
//! whole stream graded by the group's `parse.rules`, the verdict held against the reference run's.
use crate::iges_session::IgesSession;
use crate::support::{
    process::verdict_of,
    raster_session::KEEP_IMAGES_ENV,
    rules::{Rule, RuleMatch, RuleStatus, Verdict},
    script_error::run_script,
};
use crate::table::{DeRow, RecordedVerdict, XcafRow};

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

/// `tests/bugs/parse.rules`.
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

/// `tests/xcaf/parse.rules`.
const XCAF_RULES: &[Rule] = &[ignoring(
    "Error: the STEP document reader: NAUO is not mapped to shape",
)];

/// The groups' own `cpulimit`.
const GROUP_CPU_SECONDS: i64 = 1000;

/// Runs a `de/iges_*` row; the test passes when its stream earns the reference's verdict.
/// Example: `run_de_case("cases::de_iges::de_iges_1_b2", &row)`.
pub fn run_de_case(identity: &str, row: &DeRow) {
    let body = || {
        println!("Executing {}...", row.source);
        let mut session = IgesSession::for_source(&identity.replace("::", "_"), row.source);
        run_script(|| crate::de_end::run(&mut session, row));
    };
    let verdict = verdict_of(identity, GROUP_CPU_SECONDS, DE_RULES, body)
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    if let Some(verdict) = verdict {
        hold_against_reference(identity, &verdict, row.recorded);
    }
}

/// Runs an `xcaf/*` row against the reference's verdict.
/// Example: `run_xcaf_case("cases::xcaf_igs::xcaf_brep_to_igs_add_cl_a1", &row)`.
pub fn run_xcaf_case(identity: &str, row: &XcafRow) {
    let body = || {
        println!("Executing {}...", row.source);
        let mut session = IgesSession::for_source(&identity.replace("::", "_"), row.source);
        run_script(|| crate::xcaf_end::run(&mut session, row));
    };
    let verdict = verdict_of(identity, GROUP_CPU_SECONDS, XCAF_RULES, body)
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    if let Some(verdict) = verdict {
        hold_against_reference(identity, &verdict, row.recorded);
    }
}

/// Runs a hand-written `bugs` script against the reference's verdict.
/// Example: `run_one_off("one_off::bugs_xde_bug21802", "tests/bugs/xde/bug21802", RecordedVerdict::Ok, bug21802)`.
pub fn run_one_off(
    identity: &str,
    source: &str,
    recorded: RecordedVerdict,
    script: fn(&mut IgesSession),
) {
    let body = || {
        println!("Executing {source}...");
        let mut session = IgesSession::for_source(&identity.replace("::", "_"), source);
        run_script(|| {
            println!("CPU and elapsed time limit set to {GROUP_CPU_SECONDS} seconds");
            script(&mut session);
            println!("TEST COMPLETED");
        });
    };
    let verdict = verdict_of(identity, GROUP_CPU_SECONDS, BUGS_RULES, body)
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    if let Some(verdict) = verdict {
        hold_against_reference(identity, &verdict, recorded);
    }
}

fn hold_against_reference(identity: &str, verdict: &Verdict, recorded: RecordedVerdict) {
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
