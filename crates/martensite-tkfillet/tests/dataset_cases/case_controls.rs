//! What this host's own groups must refuse. A control, not a case.
use crate::groups::BLEND_RULES;
use crate::support::{
    groups::BUGS,
    rules::{Verdict, grade},
};

#[test]
fn a_blend_stop_is_ignored_only_in_the_blend_group() {
    let stopped = "ScriptStep Exception: tolerance ang : 0.01\nTODO All:TEST INCOMPLETE\n";
    let incomplete = "TODO All:TEST INCOMPLETE\nScriptStep Exception: tolerance ang : 0.01\n";
    assert_eq!(grade(incomplete, BLEND_RULES), Verdict::KnownProblem);
    assert!(matches!(grade(stopped, BUGS.rules), Verdict::Failed(_)));
}
