//! How a case's stream is graded: its group's parse rules and the verdict the reference run earned.
use crate::support::{
    process::verdict_of,
    raster_session::KEEP_IMAGES_ENV,
    rules::{Rule, RuleMatch, RuleStatus, Verdict},
    script_error::run_script,
};

fn is_faulty(word: &str) -> bool {
    word == "Faulty"
}

const BAD_SHAPE: Rule = Rule {
    status: RuleStatus::Failed,
    matching: RuleMatch::Word(is_faulty),
};
const MASS_MESSAGE: Rule = Rule {
    status: RuleStatus::Ok,
    matching: RuleMatch::Contains("Relative error of mass computation"),
};

/// `pipe`.
pub const PIPE_RULES: &[Rule] = &[BAD_SHAPE, MASS_MESSAGE];
/// `bugs`.
pub const BUGS_RULES: &[Rule] = &[
    BAD_SHAPE,
    Rule {
        status: RuleStatus::Failed,
        matching: RuleMatch::Contains("failure reading attribute"),
    },
    Rule {
        status: RuleStatus::Skipped,
        matching: RuleMatch::Contains("Error: unsupported locale specification"),
    },
    MASS_MESSAGE,
];
/// `demo`.
pub const DEMO_RULES: &[Rule] = &[
    Rule {
        status: RuleStatus::Ok,
        matching: RuleMatch::Contains("Wanted exception"),
    },
    Rule {
        status: RuleStatus::Failed,
        matching: RuleMatch::Contains("Failure message"),
    },
];

/// Everything one case needs from its group.
pub struct CaseGroup {
    pub rules: &'static [Rule],
    /// The budget the group's `begin` sets, in seconds.
    pub cpu_seconds: i64,
}

/// `bugs/begin`.
pub const BUGS: CaseGroup = CaseGroup {
    rules: BUGS_RULES,
    cpu_seconds: 1000,
};
/// `pipe/begin`.
pub const PIPE: CaseGroup = CaseGroup {
    rules: PIPE_RULES,
    cpu_seconds: 600,
};
/// `demo` sets no limit; the child process still needs one, so it takes the longest of the others.
pub const DEMO: CaseGroup = CaseGroup {
    rules: DEMO_RULES,
    cpu_seconds: 1000,
};

/// Runs a script in its child process; the parent requires the verdict the reference run earned.
/// Example: `run_alone("cases::bugs::bug22978", &BUGS, Verdict::ok, || script())`.
pub fn run_alone(identity: &str, group: &CaseGroup, reference: Verdict, script: impl FnOnce()) {
    let body = || {
        assert_eq!(
            martensite_process_policy::reported_process_limit(),
            group.cpu_seconds,
            "begin: CPU budget"
        );
        run_script(script);
    };
    let verdict = verdict_of(identity, group.cpu_seconds, group.rules, body)
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    let Some(verdict) = verdict else {
        return;
    };
    keep_verdict(identity, &verdict);
    assert_eq!(verdict, reference, "case {identity}: reference verdict");
}

/// Writes the verdict beside the kept stream, so a run can be compared with the reference verdicts.
fn keep_verdict(identity: &str, verdict: &Verdict) {
    if let Some(folder) = std::env::var_os(KEEP_IMAGES_ENV) {
        let file = identity.replace("::", "_") + ".verdict.txt";
        std::fs::write(
            std::path::PathBuf::from(folder).join(file),
            format!("{verdict:?}"),
        )
        .unwrap();
    }
}
