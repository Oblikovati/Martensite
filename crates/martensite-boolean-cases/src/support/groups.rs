//! The test groups besides `tests/boolean`: their budgets, rules and hooks. Each `*_RULES` holds
//! what that group's `parse.rules` holds, and is read before the common rules.
use super::hooks;
use super::{
    heal_commands,
    hooks::CaseGroup,
    rules::{Rule, RuleMatch, RuleStatus},
    session::Session,
};

fn is_faulty(word: &str) -> bool {
    word == "Faulty"
}

/// The rules of `tests/bugs`, read before the common ones.
const BUGS_RULES: &[Rule] = &[
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
        matching: RuleMatch::Contains("Error: unsupported locale specification"),
    },
    Rule {
        status: RuleStatus::Ok,
        matching: RuleMatch::Contains("Relative error of mass computation"),
    },
];

/// The rules of `tests/lowalgos`, `tests/heal` and `tests/mkface`.
const FAULTY_RULES: &[Rule] = &[Rule {
    status: RuleStatus::Failed,
    matching: RuleMatch::Word(is_faulty),
}];

/// The rules of `tests/perf`.
const PERF_RULES: &[Rule] = &[
    Rule {
        status: RuleStatus::Failed,
        matching: RuleMatch::Word(is_faulty),
    },
    Rule {
        status: RuleStatus::Ignore,
        matching: RuleMatch::Contains("Error: performance"),
    },
];

/// `tests/bugs`: `cpulimit 1000`; its `end` only completes the case.
pub const BUGS: CaseGroup = CaseGroup {
    cpu_seconds: 1000,
    rules: BUGS_RULES,
    begin: bugs_begin,
    end: completed_end,
};

/// A `tests/bugs` case whose script lowers the budget with `cpulimit 100`: the child's budget is
/// installed before the case starts, so the group's own line is what it writes first.
pub const BUGS_CPU100: CaseGroup = CaseGroup {
    cpu_seconds: 100,
    rules: BUGS_RULES,
    begin: bugs_cpu100_begin,
    end: completed_end,
};

/// `tests/heal`: `cpulimit 1000`. Its `end` depends on each script's variables.
pub const HEAL: CaseGroup = CaseGroup {
    cpu_seconds: 1000,
    rules: FAULTY_RULES,
    begin: heal_begin,
    end: heal_commands::heal_end,
};

/// `tests/mkface`: `cpulimit 600`. Its `end` depends on each script's tolerance variables.
pub const MKFACE: CaseGroup = CaseGroup {
    cpu_seconds: 600,
    rules: FAULTY_RULES,
    begin: mkface_begin,
    end: heal_commands::mkface_end,
};

/// `tests/perf`: `cpulimit 500`; its `end` takes a 3D screenshot only when a 3D view is open.
pub const PERF: CaseGroup = CaseGroup {
    cpu_seconds: 500,
    rules: PERF_RULES,
    begin: perf_begin,
    end: completed_end,
};

/// `tests/lowalgos`: `cpulimit 60`; its `end` only completes the case.
pub const LOWALGOS: CaseGroup = CaseGroup {
    cpu_seconds: 60,
    rules: FAULTY_RULES,
    begin: lowalgos_begin,
    end: completed_end,
};

/// `bopsection/E3`: its script raises the group's budget with `cpulimit 600`.
pub const BOPSECTION_CPU600: CaseGroup = CaseGroup {
    cpu_seconds: 600,
    rules: hooks::BOOLEAN.rules,
    begin: bopsection_cpu600_begin,
    end: hooks::boolean_end,
};

/// A case whose script lowers or raises the group's budget with `cpulimit 200`.
pub const LOWALGOS_CPU200: CaseGroup = CaseGroup {
    cpu_seconds: 200,
    rules: FAULTY_RULES,
    begin: lowalgos_cpu200_begin,
    end: completed_end,
};

fn budget_begin(group: &CaseGroup) {
    let budget = martensite_process_policy::reported_process_limit();
    assert_eq!(budget, group.cpu_seconds, "group begin: CPU budget");
    println!("CPU and elapsed time limit set to {budget} seconds");
}

fn bugs_begin(_session: &mut dyn Session) {
    budget_begin(&BUGS);
}

fn bugs_cpu100_begin(_session: &mut dyn Session) {
    let budget = martensite_process_policy::reported_process_limit();
    assert_eq!(
        budget, BUGS_CPU100.cpu_seconds,
        "script cpulimit: CPU budget"
    );
    println!(
        "CPU and elapsed time limit set to {} seconds",
        BUGS.cpu_seconds
    );
}

fn lowalgos_begin(_session: &mut dyn Session) {
    budget_begin(&LOWALGOS);
}

fn lowalgos_cpu200_begin(_session: &mut dyn Session) {
    let budget = martensite_process_policy::reported_process_limit();
    assert_eq!(
        budget, LOWALGOS_CPU200.cpu_seconds,
        "script cpulimit: CPU budget"
    );
    println!(
        "CPU and elapsed time limit set to {} seconds",
        LOWALGOS.cpu_seconds
    );
}

fn bopsection_cpu600_begin(_session: &mut dyn Session) {
    let budget = martensite_process_policy::reported_process_limit();
    assert_eq!(
        budget, BOPSECTION_CPU600.cpu_seconds,
        "script cpulimit: CPU budget"
    );
    println!(
        "CPU and elapsed time limit set to {} seconds",
        hooks::BOOLEAN.cpu_seconds
    );
    println!("CPU and elapsed time limit set to {budget} seconds");
}

fn heal_begin(_session: &mut dyn Session) {
    budget_begin(&HEAL);
}

fn mkface_begin(_session: &mut dyn Session) {
    budget_begin(&MKFACE);
}

fn perf_begin(_session: &mut dyn Session) {
    budget_begin(&PERF);
}

/// The `end` of a group without inherited checks; the triangulation check of `tests/bugs` is
/// off, its `rel_tol` being 0.
fn completed_end(_session: &mut dyn Session) {
    println!("TEST COMPLETED");
}

#[test]
fn the_group_rules_keep_a_warning_and_refuse_a_faulty_line() {
    use super::rules::{Verdict, grade};
    let completed = "Warning: small edge\nTEST COMPLETED\n";
    assert_eq!(grade(completed, FAULTY_RULES), Verdict::Ok);
    assert_eq!(grade(completed, BUGS_RULES), Verdict::Ok);
    let faulty = "Faulty\nRelative error of mass computation : Error\nTEST COMPLETED\n";
    assert!(matches!(grade(faulty, FAULTY_RULES), Verdict::Failed(_)));
    assert!(matches!(grade(faulty, BUGS_RULES), Verdict::Failed(_)));
}

#[test]
fn only_the_bugs_group_accepts_the_mass_error_message_without_its_colon() {
    use super::rules::{Verdict, grade};
    let line = "Relative error of mass computation is large\nTEST COMPLETED\n";
    assert_eq!(grade(line, BUGS_RULES), Verdict::Ok);
    assert!(matches!(grade(line, FAULTY_RULES), Verdict::Failed(_)));
    let usual = "Relative error of mass computation :     1e-14\nTEST COMPLETED\n";
    assert_eq!(grade(usual, FAULTY_RULES), Verdict::Ok);
}

#[test]
fn the_perf_group_ignores_only_its_own_error() {
    use super::rules::{Verdict, grade};
    let ignored = "Error: performance is below the limit\nTEST COMPLETED\n";
    assert_eq!(grade(ignored, PERF_RULES), Verdict::Ok);
    let other = "Error: something else\nTEST COMPLETED\n";
    assert!(matches!(grade(other, PERF_RULES), Verdict::Failed(_)));
}
