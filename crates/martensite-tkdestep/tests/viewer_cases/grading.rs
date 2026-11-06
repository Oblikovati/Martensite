//! How a case's stream is graded: its group's parse rules and the verdict the reference run earned.
use crate::host::ViewerHost;
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

/// One grid family: its parse rules and the limit its `begin` sets.
pub struct CaseGroup {
    pub rules: &'static [Rule],
    pub cpu_seconds: i64,
}

/// `tests/bugs`.
pub const BUGS: CaseGroup = CaseGroup {
    rules: &[
        BAD_SHAPE,
        Rule {
            status: RuleStatus::Failed,
            matching: RuleMatch::Contains("failure reading attribute"),
        },
        Rule {
            status: RuleStatus::Skipped,
            matching: RuleMatch::Contains("Error: unsupported locale specification"),
        },
        Rule {
            status: RuleStatus::Ok,
            matching: RuleMatch::Contains("Relative error of mass computation"),
        },
    ],
    cpu_seconds: 1000,
};
/// `tests/v3d`.
pub const V3D: CaseGroup = CaseGroup {
    rules: &[BAD_SHAPE],
    cpu_seconds: 1000,
};
/// `tests/de_mesh` sets no limit; the child process still needs one, so it takes the others'.
pub const DE_MESH: CaseGroup = CaseGroup {
    rules: &[],
    cpu_seconds: 1000,
};

/// Runs a script in its child process; the parent requires the verdict the reference run earned.
/// Example: `run_case("cases::step::bug25169", "tests/bugs/step/bug25169", &BUGS, Verdict::ok, script)`.
pub fn run_case(
    identity: &str,
    source: &'static str,
    group: &CaseGroup,
    reference: Verdict,
    script: fn(&mut ViewerHost),
) {
    let body = || {
        println!("Executing {source}...");
        let mut host = ViewerHost::new(&identity.replace("::", "_"), source);
        run_script(|| {
            println!(
                "CPU and elapsed time limit set to {} seconds",
                group.cpu_seconds
            );
            for line in heading_of(source) {
                println!("{line}");
            }
            script(&mut host);
            host.close();
            println!("TEST COMPLETED");
        });
    };
    let verdict = verdict_of(identity, group.cpu_seconds, group.rules, body)
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    let Some(verdict) = verdict else {
        return;
    };
    keep_verdict(identity, &verdict);
    assert_eq!(verdict, reference, "case {identity}: reference verdict");
}

/// What the script says before its first command; the scripts with a generated table say it there.
fn heading_of(source: &str) -> &'static [&'static str] {
    const SAID_BY_THE_SCRIPT: [&str; 8] = [
        "tests/de_mesh/step_read/A2",
        "tests/de_mesh/step_write/A1",
        "tests/bugs/step/bug24283_2",
        "tests/bugs/step/bug25167",
        "tests/bugs/step/bug25169",
        "tests/bugs/step/bug32350",
        "tests/bugs/xde/bug22535_1",
        "tests/bugs/xde/bug22535_2",
    ];
    if SAID_BY_THE_SCRIPT.contains(&source) {
        return &[];
    }
    crate::cases::headings::heading(source)
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
