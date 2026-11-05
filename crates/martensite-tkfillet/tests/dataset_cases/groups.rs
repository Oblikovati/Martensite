//! The groups these cases inherit from: their budgets, parse rules and end scripts.
use crate::support::{
    checkshape::checkshape,
    checkview::{CheckView, checkview},
    groups::{BUGS, PERF},
    hooks::{BOOLEAN, CaseGroup},
    rules::{Rule, RuleMatch, RuleStatus},
    session::Session,
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

/// `blend/parse.rules`: a `blend` that returns 1 is only an incomplete test.
pub const BLEND_RULES: &[Rule] = &[
    BAD_SHAPE,
    Rule {
        status: RuleStatus::Ignore,
        matching: RuleMatch::Contains("ScriptStep Exception: tolerance ang : "),
    },
    MASS_MESSAGE,
];
/// `fillet2d/parse.rules`.
const FILLET2D_RULES: &[Rule] = &[
    BAD_SHAPE,
    Rule {
        status: RuleStatus::Skipped,
        matching: RuleMatch::Contains("Error: unsupported locale specification"),
    },
    MASS_MESSAGE,
];
/// `demo/parse.rules`.
const DEMO_RULES: &[Rule] = &[
    Rule {
        status: RuleStatus::Ok,
        matching: RuleMatch::Contains("Wanted exception"),
    },
    Rule {
        status: RuleStatus::Failed,
        matching: RuleMatch::Contains("Failure message"),
    },
];

/// The groups of the toolkit's cases, by the tests folder they come from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FilletGroup {
    Blend,
    Bugs,
    /// `bugs` with a script's own `cpulimit 200`, which the child installs before it starts.
    BugsCpu200,
    Perf,
    Fillet2d,
    Boolean,
    /// `tests/demo`, whose scripts write the final message themselves.
    Demo,
}

/// What the group `begin` writes. The budget is in force already: a script's lower `cpulimit` is
/// installed by the child process before it starts, so the check allows for that.
fn budget_line(seconds: i64) {
    let budget = martensite_process_policy::reported_process_limit();
    assert!(
        budget > 0 && budget <= seconds,
        "group begin: CPU budget {budget}"
    );
    println!("CPU and elapsed time limit set to {seconds} seconds");
}

fn blend_begin(_session: &mut dyn Session) {
    budget_line(600);
}

fn bugs_begin(_session: &mut dyn Session) {
    budget_line(1000);
}

/// `blend/end`: `puts [checkshape result]` and `checkview`, or an error, then the final message.
fn blend_end(session: &mut dyn Session) {
    if let Some(result) = session.shape("result").cloned() {
        // `puts [checkshape result]`: the command's own output, then the same text again.
        println!("{}", checkshape(&result).text);
        checkview(session, &CheckView::of("result"));
    } else {
        println!("Error : The blend cannot be built.");
    }
    println!("TEST COMPLETED");
}

/// `bugs/end`, with the triangulation check off since `rel_tol` is 0.
fn completed_end(_session: &mut dyn Session) {
    println!("TEST COMPLETED");
}

fn no_end(_session: &mut dyn Session) {}

/// `fillet2d/end`.
fn view_end(session: &mut dyn Session) {
    checkview(session, &CheckView::of("result"));
    println!("TEST COMPLETED");
}

type Script = fn(&mut dyn Session);
/// A group's budget, rules, `begin` and `end`.
type GroupParts = (i64, &'static [Rule], Script, Script);

impl FilletGroup {
    /// What the contract needs to run and grade a case of this group.
    /// Example: `FilletGroup::Blend.case_group().cpu_seconds == 600`.
    pub fn case_group(self) -> CaseGroup {
        let (cpu_seconds, rules, begin, end): GroupParts = match self {
            Self::Blend => (600, BLEND_RULES, blend_begin, blend_end),
            Self::Bugs => return BUGS,
            Self::BugsCpu200 => (200, BUGS.rules, bugs_begin, completed_end),
            Self::Perf => return PERF,
            Self::Fillet2d => (1000, FILLET2D_RULES, bugs_begin, view_end),
            Self::Boolean => return BOOLEAN,
            Self::Demo => (1000, DEMO_RULES, no_end, no_end),
        };
        CaseGroup {
            cpu_seconds,
            rules,
            begin,
            end,
        }
    }
}
