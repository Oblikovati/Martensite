//! The `cpulimit` each test group's `begin` script sets, applied in a dedicated child
//! process before the case restores its first input (martensite-process-policy), and the
//! parse rules each group's log is judged by.
use martensite_process_policy::{
    case_runner::{CaseRunner, CaseSpec, FactorProfile},
    reported_process_limit,
};

/// `tests/bugs/begin`: `cpulimit 1000`.
pub const BUGS_SECONDS: i64 = 1000;
/// `tests/cr/begin`: `cpulimit 300`.
pub const CR_SECONDS: i64 = 300;
/// `tests/heal/begin`: `cpulimit 1000`.
pub const HEAL_SECONDS: i64 = 1000;
/// `tests/lowalgos/begin`: `cpulimit 60`.
pub const LOWALGOS_SECONDS: i64 = 60;
/// `tests/mkface/begin`: `cpulimit 600`.
pub const MKFACE_SECONDS: i64 = 600;
/// `tests/perf/begin`: `cpulimit 500`.
pub const PERF_SECONDS: i64 = 500;
/// `tests/sewing/begin`: `cpulimit 300`.
pub const SEWING_SECONDS: i64 = 300;

/// The test group of a grid module name: `group_of("bugs_modalg_2") == "bugs"`.
pub fn group_of(grid: &str) -> &str {
    grid.split_once('_').map_or(grid, |(group, _)| group)
}

/// The group whose rules the shared log ([`super::log::finish`]) applies to `group`:
/// tests/mkface/parse.rules is heal's (`FAILED /\bFaulty\b/`), and tests/sewing and
/// tests/cr have no parse.rules, so only the common ones apply, which is what geometry's
/// single rule leaves (its `OK /Relative error of mass computation/` changes no verdict:
/// the common rules only IGNORE that message); `parse_rules_of("mkface") == "heal"`.
pub fn parse_rules_of(group: &str) -> &str {
    match group {
        "mkface" => "heal",
        "sewing" | "cr" => "geometry",
        other => other,
    }
}

/// One `#[test]` per case body of `crate::cases::<grid>`, run by `runner(identity, seconds,
/// body)` with a fresh `session` in the child; the libtest name is `<grid>::<case>`. After
/// the body the case's log gets the group's parse-rule verdict ([`super::log::finish`]).
/// `dataset_cases!(run_case, NoView, heal_fix_shape, HEAL_SECONDS; a2);`
macro_rules! dataset_cases {
    ($runner:path, $session:expr, $grid:ident, $seconds:expr; $($case:ident),* $(,)?) => {$(
        #[test]
        fn $case() {
            let identity = concat!(stringify!($grid), "::", stringify!($case));
            $runner(identity, $seconds, || {
                crate::support::messages::stdout_printer_to_log();
                let mut session = $session;
                crate::cases::$grid::$case(&mut session);
                let group = crate::support::process::group_of(stringify!($grid));
                crate::support::log::finish(crate::support::process::parse_rules_of(group));
            });
        }
    )*};
}
pub(crate) use dataset_cases;

/// Run `body` as the libtest case `identity` (its path below the target root) under the
/// group's CPU and elapsed budget; `run_case("heal_fix_shape::a2", HEAL_SECONDS, || {})`.
pub fn run_case(identity: &str, seconds: i64, body: impl FnOnce()) {
    let artifacts = std::env::temp_dir().join("martensite-tkshhealing-draw-case-receipts");
    let spec = CaseSpec::new(identity, seconds, FactorProfile::RequireUnset);
    let receipt = CaseRunner::new(&artifacts, None)
        .run(spec, || {
            assert_eq!(reported_process_limit(), seconds, "group cpulimit");
            body();
            Ok(())
        })
        .unwrap_or_else(|failure| panic!("{failure}"));
    println!(
        "{{\"case\":{identity:?},\"receipt_directory\":{:?}}}",
        receipt.directory
    );
}
