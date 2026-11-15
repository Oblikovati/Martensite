//! Builds a package's test targets without running them, with exactly the
//! cargo arguments and environment its PR test job uses, so a warmed build
//! cache matches the PR compilations (#472).

use super::*;

impl CargoTasks<'_> {
    /// Compiles every test target the PR test job would run, then stops.
    ///
    /// ```ignore
    /// tasks.build_tests(&selection, &BuildTarget::Triple(triple))?;
    /// ```
    pub fn build_tests(
        &self,
        selection: &PackageSelection,
        target: &BuildTarget,
    ) -> BuildResult<()> {
        let outcomes: Vec<_> = script_budget_profiles::groups(selection, self.runner, self.root)?
            .into_iter()
            .map(|group| {
                let spec = self.test_spec(&group.selection, target, group.optimization);
                self.runner.run(&spec.arg("--no-run"))
            })
            .collect();
        outcomes.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake_runner::RecordingRunner;

    #[test]
    fn warm_build_uses_the_test_command_and_profile_of_the_pr_job() {
        let runner = RecordingRunner::default();
        let tasks = CargoTasks::new(&runner, Path::new("/repo"));
        let selection = PackageSelection::Packages(vec!["martensite-tkbo".into()]);
        let target = BuildTarget::Triple("x86_64-unknown-linux-gnu".into());
        tasks.build_tests(&selection, &target).unwrap();
        let specs = runner.command_specs();
        assert_eq!(
            runner.commands(),
            [
                "cargo test --locked --package martensite-tkbo --target x86_64-unknown-linux-gnu --no-fail-fast --no-run"
            ]
        );
        let script = tasks.test_spec(
            &selection,
            &target,
            script_budget_profiles::Optimization::ScriptBudget,
        );
        assert_eq!(specs[0], script.arg("--no-run"));
    }
}
