//! The `cpulimit` each test group's `begin` script sets, applied in a dedicated child
//! process before the case restores its first input (martensite-process-policy).
#[path = "../../common/isolated_case.rs"]
mod isolated_case;

/// `bugs_begin`: `cpulimit 1000`.
pub const BUGS_SECONDS: i64 = 1000;
/// `geometry_begin`: `cpulimit 600`.
pub const GEOMETRY_SECONDS: i64 = 600;
/// `heal_begin`: `cpulimit 1000`.
pub const HEAL_SECONDS: i64 = 1000;
/// `lowalgos_begin`: `cpulimit 60`.
pub const LOWALGOS_SECONDS: i64 = 60;
/// `perf_begin`: `cpulimit 500`.
pub const PERF_SECONDS: i64 = 500;

/// The test group of a grid module name: `group_of("bugs_modalg_2") == "bugs"`.
pub fn group_of(grid: &str) -> &str {
    grid.split_once('_').map_or(grid, |(group, _)| group)
}

/// One `#[test]` per case body of `crate::cases::<grid>`, run by `runner(identity, seconds,
/// body)` with a fresh `session` in the child; the libtest name is `<grid>::<case>`. After
/// the body the case's log gets the group's parse-rule verdict ([`super::log::finish`]).
/// `dataset_cases!(run_case, NoView, heal_checkshape, HEAL_SECONDS; bug27814_1);`
#[allow(unused_macros)] // The timing target writes its one ignored case out.
macro_rules! dataset_cases {
    ($runner:path, $session:expr, $grid:ident, $seconds:expr; $($case:ident),* $(,)?) => {$(
        #[test]
        fn $case() {
            let identity = concat!(stringify!($grid), "::", stringify!($case));
            $runner(identity, $seconds, || {
                let mut session = $session;
                crate::cases::$grid::$case(&mut session);
                crate::support::log::finish(crate::support::process::group_of(stringify!($grid)));
            });
        }
    )*};
}
#[allow(unused_imports)]
pub(crate) use dataset_cases;

/// Run `body` as the libtest case `identity` (its path below the target root) alone in a
/// child process under the group's elapsed budget; `run_case("heal_checkshape::bug27814_1", HEAL_SECONDS, || {})`.
pub fn run_case(identity: &str, seconds: i64, body: impl FnOnce()) {
    isolated_case::run_isolated(identity, seconds, body);
}
