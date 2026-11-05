//! A case whose script declares its problem and which still shows it, as the reference does on a
//! neighbouring line: its test holds exactly that outcome, so a change either way shows. It is not
//! a pass of the script's checks.
use super::{
    hooks::CaseGroup,
    process::case_stream,
    rules::{Verdict, grade},
};

/// One `#[test]` that passes only while the case ends as the known problem on this line.
/// `pinned_problem_case!(groups::LOWALGOS, lowalgos_intss, hooks::no_grid_end, bug567_1, "Error: x");`
// `crate` is meant: the case bodies are the calling crate's.
#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! pinned_problem_case {
    ($group:path, $grid:ident, $grid_end:path, $case:ident, $line:expr) => {
        #[test]
        fn $case() {
            let identity = concat!(stringify!($grid), "::", stringify!($case));
            let group = $group;
            $crate::support::pinned_problem::run_pinned_problem_case(
                identity,
                &group,
                $line,
                || {
                    let image = concat!(stringify!($grid), "_", stringify!($case));
                    let mut session = $crate::support::raster_session::RasterSession::new(image);
                    (group.begin)(&mut session);
                    crate::cases::$grid::$case(&mut session);
                    $grid_end(&mut session);
                    (group.end)(&mut session);
                },
            );
        }
    };
}
pub use crate::pinned_problem_case;

/// Why `verdict` and `stream` are not the known problem on `line`, if they are not.
/// Example: `refusal(&Verdict::Ok, "", "Error: x").is_some()`.
pub fn refusal(verdict: &Verdict, stream: &str, line: &str) -> Option<String> {
    if matches!(verdict, Verdict::KnownProblem) && stream.contains(line) {
        return None;
    }
    Some(format!(
        "{verdict:?}; expected the known problem on {line:?}"
    ))
}

/// Runs `body` as the libtest case `identity`, and panics unless it ends as the known problem on
/// `line`. Example: `run_pinned_problem_case("g::c", &hooks::BOOLEAN, "Error: x", || println!("x"))`.
pub fn run_pinned_problem_case(identity: &str, group: &CaseGroup, line: &str, body: impl FnOnce()) {
    let stream = case_stream(identity, group.cpu_seconds, body)
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    let Some(stream) = stream else {
        return;
    };
    if let Some(reason) = refusal(&grade(&stream, group.rules), &stream, line) {
        panic!("case {identity}: {reason}\n--- stream ---\n{stream}");
    }
}

#[test]
fn only_the_problem_on_the_line_is_accepted() {
    assert_eq!(
        refusal(&Verdict::KnownProblem, "Error: x on line 3", "Error: x"),
        None
    );
    assert!(refusal(&Verdict::KnownProblem, "Error: x on line 3", "Error: y").is_some());
    assert!(refusal(&Verdict::Ok, "Error: x on line 3", "Error: x").is_some());
    assert!(
        refusal(
            &Verdict::Failed("Error: x".to_owned()),
            "Error: x",
            "Error: x"
        )
        .is_some()
    );
}
