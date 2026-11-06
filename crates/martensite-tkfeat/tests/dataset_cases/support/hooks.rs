//! The scripts every feature case inherits: the group `begin` and `end`, their budgets and rules.
use super::{
    checkshape::checkshape,
    checkview::{CheckView, checkview},
    rules::{Rule, RuleMatch, RuleStatus},
    session::Session,
};

/// What every case of one group inherits.
pub struct CaseGroup {
    /// The group's CPU and elapsed budget, in seconds.
    pub cpu_seconds: i64,
    /// The group's parse rules, before the common ones.
    pub rules: &'static [Rule],
    /// The group's `begin` script.
    pub begin: fn(&mut dyn Session),
    /// The group's `end` script.
    pub end: fn(&mut dyn Session),
}

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

/// `feat/parse.rules`.
const FEAT_RULES: &[Rule] = &[
    BAD_SHAPE,
    Rule {
        status: RuleStatus::Ignore,
        matching: RuleMatch::Contains("No error"),
    },
    MASS_MESSAGE,
];
/// `bugs/parse.rules`.
const BUGS_RULES: &[Rule] = &[
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
/// `perf/parse.rules`.
const PERF_RULES: &[Rule] = &[
    BAD_SHAPE,
    Rule {
        status: RuleStatus::Ignore,
        matching: RuleMatch::Contains("Error: performance"),
    },
];

/// `tests/feat`: `cpulimit 600`, with `checkshape` and a view of `result` at the end.
pub const FEAT: CaseGroup = CaseGroup {
    cpu_seconds: 600,
    rules: FEAT_RULES,
    begin: |_| group_budget(600, 600),
    end: feat_end,
};

/// `feat/featprism/M9`: its script raises the group's budget with `cpulimit 700`.
pub const FEAT_CPU700: CaseGroup = CaseGroup {
    cpu_seconds: 700,
    rules: FEAT_RULES,
    begin: |_| group_budget(700, 600),
    end: feat_end,
};

/// `tests/bugs`: `cpulimit 1000`; its `end` only completes the case.
pub const BUGS: CaseGroup = CaseGroup {
    cpu_seconds: 1000,
    rules: BUGS_RULES,
    begin: |_| group_budget(1000, 1000),
    end: completed_end,
};

/// `perf/modalg/bug26929`: its script lowers the group's `cpulimit 500` to 100.
pub const PERF_CPU100: CaseGroup = CaseGroup {
    cpu_seconds: 100,
    rules: PERF_RULES,
    begin: |_| group_budget(100, 500),
    end: completed_end,
};

/// What the budget command writes: the child already runs under `installed`, set before the case
/// starts, so a script's own `cpulimit` is checked here and written by its own line.
fn group_budget(installed: i64, group: i64) {
    let budget = martensite_process_policy::reported_process_limit();
    assert_eq!(budget, installed, "group begin: CPU budget");
    println!("CPU and elapsed time limit set to {group} seconds");
}

/// `tests/feat/end`: `puts [checkshape result]` and its view, or the error that names no result.
/// Example: `feat_end(session)`.
pub fn feat_end(session: &mut dyn Session) {
    match session.shape("result") {
        Some(result) => {
            println!("{}", checkshape(result).text);
            checkview(session, &CheckView::of("result"));
        }
        None => println!("Error : The feat grid cannot be built."),
    }
    println!("TEST COMPLETED");
}

fn completed_end(_session: &mut dyn Session) {
    println!("TEST COMPLETED");
}

/// The grid `end` of the grids that have none; `no_grid_end(session)`.
pub fn no_grid_end(_session: &mut dyn Session) {}
