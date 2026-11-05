//! Runs each case in its own child process, under the group's budget, and grades everything
//! that child wrote to its stdout and stderr.
use super::{
    hooks::CaseGroup,
    raster_session::KEEP_IMAGES_ENV,
    rules::{Verdict, grade},
};
use martensite_process_policy::case_runner::{
    CaseReceipt, CaseRunError, CaseRunner, CaseSpec, FactorProfile,
};
use std::{cell::Cell, path::PathBuf};

/// What the child runs for one case: begin, body, grid end and group end on a fresh memory
/// raster session, stopping where a script error stops the script.
macro_rules! case_script {
    ($group:expr, $grid:ident, $grid_end:path, $case:ident) => {
        || {
            let image = concat!(stringify!($grid), "_", stringify!($case));
            let mut session = $crate::support::raster_session::RasterSession::new(image);
            let mut script = $crate::support::feat_script::FeatScript::new(&mut session);
            $crate::support::script_error::run_script(|| {
                ($group.begin)(&mut script);
                $crate::cases::$grid::$case(&mut script);
                $grid_end(&mut script);
                ($group.end)(&mut script);
            });
        }
    };
}
pub(crate) use case_script;

/// One `#[test]` per body of `crate::cases::<grid>`, each with a fresh memory raster session;
/// the libtest name must be `<grid>::<case>`, so call it from the crate-root module `<grid>`.
/// `dataset_cases!(hooks::FEAT, feat_featprism, hooks::feat_end; A1, A2);`
macro_rules! dataset_cases {
    ($group:path, $grid:ident, $grid_end:path; $($case:ident),* $(,)?) => {$(
        #[test]
        fn $case() {
            let identity = concat!(stringify!($grid), "::", stringify!($case));
            let group = $group;
            $crate::support::process::run_case(
                identity,
                &group,
                $crate::support::process::case_script!(group, $grid, $grid_end, $case),
            );
        }
    )*};
}
pub(crate) use dataset_cases;

/// [`dataset_cases`] for cases listed as unstable: their verdict changes from run to run on the
/// reference build too, so the default run leaves them out. `--ignored` cannot run them, because
/// the child process would ignore the test as well; to run them, build with
/// `RUSTFLAGS='--cfg martensite_unstable_cases'`.
/// `unstable_dataset_cases!(hooks::BOOLEAN, grid, hooks::no_grid_end, "why"; A1);`
#[allow(unused_macros)] // Used by a family with such a case.
macro_rules! unstable_dataset_cases {
    ($group:path, $grid:ident, $grid_end:path, $reason:literal; $($case:ident),* $(,)?) => {$(
        #[test]
        #[cfg_attr(not(martensite_unstable_cases), ignore = $reason)]
        fn $case() {
            let identity = concat!(stringify!($grid), "::", stringify!($case));
            let group = $group;
            $crate::support::process::run_case(
                identity,
                &group,
                $crate::support::process::case_script!(group, $grid, $grid_end, $case),
            );
        }
    )*};
}
#[allow(unused_imports)]
pub(crate) use unstable_dataset_cases;

/// [`dataset_cases`] for cases that exceed their script's CPU budget without optimization: there
/// libtest reports them ignored, with the reason, and every optimized build runs them. The
/// setting is `martensite_unoptimized_build`, which `build.rs` derives from the build's
/// opt-level. `optimized_dataset_cases!(groups::G, grid, hooks::no_grid_end, "why"; A1);`
#[allow(unused_macros)] // Used by a family with such a case.
macro_rules! optimized_dataset_cases {
    ($group:path, $grid:ident, $grid_end:path, $reason:literal; $($case:ident),* $(,)?) => {$(
        #[test]
        #[cfg_attr(martensite_unoptimized_build, ignore = $reason)]
        fn $case() {
            let identity = concat!(stringify!($grid), "::", stringify!($case));
            let group = $group;
            $crate::support::process::run_case(
                identity,
                &group,
                $crate::support::process::case_script!(group, $grid, $grid_end, $case),
            );
        }
    )*};
}
#[allow(unused_imports)]
pub(crate) use optimized_dataset_cases;

/// Runs `body` as the libtest case `identity` and panics unless its stream earns a passing
/// verdict. Example: `run_case("boptuc_simple::A1", &hooks::BOOLEAN, || println!("TEST COMPLETED"))`.
pub fn run_case(identity: &str, group: &CaseGroup, body: impl FnOnce()) {
    let stream = case_stream(identity, group.cpu_seconds, body)
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    let Some(stream) = stream else {
        return;
    };
    let verdict = grade(&stream, group.rules);
    assert!(
        verdict.passes(),
        "case {identity}: {verdict:?}; expected Ok or KnownProblem\n--- stream ---\n{stream}"
    );
}

/// The verdict of a case's stream in the parent, `none` in the child that ran the body.
/// Example: `verdict_of("controls::fatal", &hooks::BOOLEAN, || println!("Error"))`.
pub fn verdict_of(
    identity: &str,
    group: &CaseGroup,
    body: impl FnOnce(),
) -> Result<Option<Verdict>, CaseRunError> {
    let stream = case_stream(identity, group.cpu_seconds, body)?;
    Ok(stream.map(|text| grade(&text, group.rules)))
}

/// The child's whole stream in the parent, `none` in the child that ran the body: the policy
/// holds before the body starts, and its receipt folder goes once the stream is read.
/// Example: `case_stream("controls::line", 300, || println!("line"))`.
pub fn case_stream(
    identity: &str,
    cpu_seconds: i64,
    body: impl FnOnce(),
) -> Result<Option<String>, CaseRunError> {
    let spec = CaseSpec::new(identity, cpu_seconds, FactorProfile::RequireUnset);
    let executed = Cell::new(false);
    let root = receipts_root();
    let receipt = CaseRunner::new(&root, None).run(spec, || {
        executed.set(true);
        body();
        Ok(())
    })?;
    if executed.get() {
        return Ok(None);
    }
    let stream = finalized_stream(&receipt, identity);
    keep_stream_for_inspection(identity, &stream);
    std::fs::remove_dir_all(&receipt.directory).unwrap();
    Ok(Some(stream))
}

fn receipts_root() -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("dataset-case-receipts")
}

fn finalized_stream(receipt: &CaseReceipt, identity: &str) -> String {
    let read = |file: &str| {
        let path = receipt.directory.join(file);
        std::fs::read_to_string(&path).unwrap_or_else(|cause| panic!("{path:?}: {cause}"))
    };
    let stdout = read("stdout.log");
    let body = captured_body(&stdout, identity).unwrap_or_else(|| {
        panic!("case {identity}: expected one complete libtest frame, got {stdout:?}")
    });
    format!("{body}\n{}", read("stderr.log"))
}

/// Writes the stream beside the kept images, so a run can be compared with a reference log.
fn keep_stream_for_inspection(identity: &str, stream: &str) {
    let Some(folder) = std::env::var_os(KEEP_IMAGES_ENV) else {
        return;
    };
    std::fs::create_dir_all(&folder).unwrap();
    let file = identity.replace("::", "_") + ".stream.txt";
    std::fs::write(PathBuf::from(folder).join(file), stream).unwrap();
}

/// What the child's test wrote: everything libtest's frame holds, plus any bytes written past
/// libtest's capture before the frame. `none` unless it holds exactly this one passing test;
/// a test that wrote nothing has no frame, and its stream is empty.
fn captured_body(stdout: &str, identity: &str) -> Option<String> {
    let header = format!("\n---- {identity} stdout ----\n");
    let trailer = format!("\n\nsuccesses:\n    {identity}\n\ntest result:");
    if stdout.matches(&header).count() > 1 || stdout.matches(&trailer).count() != 1 {
        return None;
    }
    let (head, summary) = stdout.split_once(&trailer)?;
    if summary.trim().lines().count() != 1 {
        return None;
    }
    let (prefix, body) = match head.split_once(&header) {
        Some((prefix, body)) => (prefix.to_owned(), body),
        None => (format!("{head}\n"), ""),
    };
    let progress = format!("running 1 test\ntest {identity} ... ");
    let direct = prefix.trim_start_matches('\n').strip_prefix(&progress)?;
    let direct = direct.strip_suffix("ok\n\nsuccesses:\n")?;
    Some(format!("{direct}{body}"))
}

#[test]
fn only_one_complete_passing_frame_is_read() {
    let frame = "\nrunning 1 test\ntest g::c ... ok\n\nsuccesses:\n\n---- g::c stdout ----\nline\n\nsuccesses:\n    g::c\n\ntest result: ok. 1 passed\n";
    assert_eq!(captured_body(frame, "g::c").as_deref(), Some("line"));
    assert_eq!(captured_body(frame, "g::other"), None);
    assert_eq!(captured_body("no frame", "g::c"), None);
    let silent = "\nrunning 1 test\ntest g::c ... ok\n\nsuccesses:\n\nsuccesses:\n    g::c\n\ntest result: ok. 1 passed\n";
    assert_eq!(captured_body(silent, "g::c").as_deref(), Some(""));
    let direct = "\nrunning 1 test\ntest g::c ... raw\nok\n\nsuccesses:\n\nsuccesses:\n    g::c\n\ntest result: ok. 1 passed\n";
    assert_eq!(captured_body(direct, "g::c").as_deref(), Some("raw\n"));
}
