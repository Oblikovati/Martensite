//! The `cpulimit` each test group's `begin` script sets, applied in a dedicated child
//! process before the case restores its first input (martensite-process-policy).
#[path = "../../common/isolated_case.rs"]
mod isolated_case;

/// `bugs_begin`: `cpulimit 1000`.
pub const BUGS_SECONDS: i64 = 1000;
/// `geometry_begin`: `cpulimit 600`.
pub const GEOMETRY_SECONDS: i64 = 600;
/// `lowalgos_begin`: `cpulimit 60`.
pub const LOWALGOS_SECONDS: i64 = 60;

/// One `#[test]` per case body of `crate::cases::<grid>`, run by `runner(identity, seconds,
/// body)` with a fresh `session` in the child; the libtest name is `<grid>::<case>`.
/// `dataset_cases!(run_case, NoView, bugs_fclasses, BUGS_SECONDS; bug25558);`
macro_rules! dataset_cases {
    ($runner:path, $session:expr, $grid:ident, $seconds:expr; $($case:ident),* $(,)?) => {$(
        #[test]
        fn $case() {
            let identity = concat!(stringify!($grid), "::", stringify!($case));
            $runner(identity, $seconds, || {
                let mut session = $session;
                crate::cases::$grid::$case(&mut session);
            });
        }
    )*};
}
pub(crate) use dataset_cases;

/// Run `body` as the libtest case `identity` (its path below the target root) alone in a
/// child process under the group's elapsed budget; `run_case("geometry_project::a1", GEOMETRY_SECONDS, || {})`.
pub fn run_case(identity: &str, seconds: i64, body: impl FnOnce()) {
    isolated_case::run_isolated(identity, seconds, body);
}
