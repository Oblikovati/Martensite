//! The scheduled job that refreshes PR build caches on `main` (#472).
//!
//! GitHub lets a pull request restore caches saved on its own ref or on the
//! default branch, never another pull request's. PR test jobs therefore only
//! restore; this job compiles each long suite's tests on `main`, with the
//! exact PR test command, and the workflow saves the resulting store there.

use crate::cargo::{BuildTarget, CargoTasks, PackageSelection};
use crate::context::BuildContext;
use crate::error::{BuildError, BuildResult};

/// The `warm-matrix` output listing every package whose cache is refreshed.
///
/// ```ignore
/// write_outputs(&warm_plan_outputs(), github_output)?;
/// ```
pub fn warm_plan_outputs() -> Vec<(&'static str, String)> {
    let packages = crate::suite_weights::warm_packages();
    let matrix = serde_json::json!({"package": packages});
    vec![("warm-matrix", matrix.to_string())]
}

/// Compiles `package`'s tests as its PR job would, refusing other packages so
/// a stale matrix cannot spend the cache budget on an unplanned store.
///
/// ```ignore
/// warm_package(&context, "martensite-vulkan", &target)?;
/// ```
pub fn warm_package(
    context: &BuildContext<'_>,
    package: &str,
    target: &BuildTarget,
) -> BuildResult<()> {
    let planned = crate::suite_weights::warm_packages();
    if !planned.contains(&package) {
        return Err(BuildError::Usage(format!(
            "warm package {package:?}; expected one of {planned:?}"
        )));
    }
    let selection = PackageSelection::Packages(vec![package.to_owned()]);
    CargoTasks::new(context.runner, context.root()).build_tests(&selection, target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::tests::context_at;
    use crate::fake_runner::RecordingRunner;
    use std::path::Path;

    #[test]
    fn plan_lists_the_warm_packages_as_a_matrix() {
        let outputs = warm_plan_outputs();
        let matrix: serde_json::Value = serde_json::from_str(&outputs[0].1).unwrap();
        assert_eq!(outputs[0].0, "warm-matrix");
        assert_eq!(matrix["package"][0], "martensite-vulkan");
    }

    #[test]
    fn warming_builds_planned_packages_and_refuses_others() {
        let runner = RecordingRunner::default();
        let context = context_at(&runner, Path::new("/repo"));
        let target = BuildTarget::Triple("x86_64-unknown-linux-gnu".into());
        warm_package(&context, "martensite-vulkan", &target).unwrap();
        assert!(
            runner
                .commands()
                .last()
                .unwrap()
                .ends_with("--no-fail-fast --no-run")
        );
        let error = warm_package(&context, "martensite-tkmath", &target).unwrap_err();
        assert!(
            error.to_string().contains("\"martensite-tkmath\""),
            "{error}"
        );
    }
}
