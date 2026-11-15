//! Splits one package's tests across PR jobs so that the parts are disjoint and
//! together run every test exactly once (#470).
//!
//! Each part builds the same test targets, lists every libtest target, assigns
//! each test by a stable hash of its target and name, and runs its share with
//! `--exact` filters. A target whose tests cannot be listed (a custom harness)
//! runs whole in one part. Doctests run in part 0. Every part recomputes the
//! same assignment from the same build, so no job depends on another.

use super::*;

/// One part of a package whose PR tests are split across several jobs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TestPart {
    index: usize,
    count: usize,
}

impl TestPart {
    /// The planned part `index` of `package`, or `None` for an unsplit package.
    /// An index outside the planned parts fails, so a stale job cannot skip tests.
    ///
    /// ```ignore
    /// let part = TestPart::planned("martensite-vulkan", 1)?;
    /// ```
    pub fn planned(package: &str, index: usize) -> BuildResult<Option<Self>> {
        let count = crate::suite_weights::pr_parts(package);
        if index >= count {
            return Err(BuildError::Usage(format!(
                "test part {index} of {package}; expected an index below {count}"
            )));
        }
        Ok((count > 1).then_some(Self { index, count }))
    }

    fn owns(&self, key: &str) -> bool {
        stable_hash(key) % self.count as u64 == self.index as u64
    }
}

// FNV-1a: a fixed, platform-independent hash, so every job agrees on ownership.
fn stable_hash(key: &str) -> u64 {
    key.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

/// A compiled test target and the cargo flags that select it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TestTarget {
    key: String,
    selector: Vec<String>,
    library: bool,
}

#[derive(Deserialize)]
struct TestArtifact {
    reason: String,
    #[serde(default)]
    executable: Option<PathBuf>,
    #[serde(default)]
    profile: Option<ArtifactProfile>,
    #[serde(default)]
    target: Option<ArtifactTarget>,
}

#[derive(Deserialize)]
struct ArtifactProfile {
    test: bool,
}

#[derive(Deserialize)]
struct ArtifactTarget {
    kind: Vec<String>,
    name: String,
}

const LIBRARY_KINDS: [&str; 6] = ["lib", "rlib", "dylib", "cdylib", "staticlib", "proc-macro"];
// Stays below the Windows command-line limit with room for the cargo arguments.
const FILTER_BYTES: usize = 24_000;

impl CargoTasks<'_> {
    /// Runs `part` of the selected package's unit, integration and doc tests.
    /// A failed target does not stop the remaining targets of the part.
    ///
    /// ```ignore
    /// tasks.test_part(&selection, &target, part)?;
    /// ```
    pub fn test_part(
        &self,
        selection: &PackageSelection,
        target: &BuildTarget,
        part: TestPart,
    ) -> BuildResult<()> {
        let outcomes: Vec<_> = script_budget_profiles::groups(selection, self.runner, self.root)?
            .into_iter()
            .map(|group| {
                let base = self.test_spec(&group.selection, target, group.optimization);
                self.run_part(&base, part)
            })
            .collect();
        outcomes.into_iter().collect()
    }

    fn run_part(&self, base: &CommandSpec, part: TestPart) -> BuildResult<()> {
        let targets = self.test_targets(base)?;
        let mut outcomes: Vec<_> = targets
            .iter()
            .map(|target| self.run_target_part(base, target, part))
            .collect();
        if part.index == 0 && targets.iter().any(|target| target.library) {
            outcomes.push(self.runner.run(&base.clone().arg("--doc")));
        }
        outcomes.into_iter().collect()
    }

    fn test_targets(&self, base: &CommandSpec) -> BuildResult<Vec<TestTarget>> {
        let build = base
            .clone()
            .args(["--no-run", "--message-format", "json-render-diagnostics"]);
        parse_test_targets(&self.runner.capture(&build)?)
    }

    fn run_target_part(
        &self,
        base: &CommandSpec,
        target: &TestTarget,
        part: TestPart,
    ) -> BuildResult<()> {
        let selected = base.clone().args(target.selector.iter().cloned());
        let Some(names) = self.listed_tests(&selected) else {
            if part.owns(&target.key) {
                return self.runner.run(&selected);
            }
            return Ok(());
        };
        let owned = names
            .into_iter()
            .filter(|name| part.owns(&format!("{}::{name}", target.key)));
        let outcomes: Vec<_> = filter_chunks(owned.collect())
            .into_iter()
            .map(|chunk| self.run_exact(&selected, chunk))
            .collect();
        outcomes.into_iter().collect()
    }

    fn listed_tests(&self, selected: &CommandSpec) -> Option<Vec<String>> {
        let list = selected.clone().args(["--", "--list", "--format", "terse"]);
        self.runner
            .capture(&list)
            .ok()
            .map(|out| parse_listed_tests(&out))
    }

    fn run_exact(&self, selected: &CommandSpec, names: Vec<String>) -> BuildResult<()> {
        let spec = selected.clone().args(["--", "--exact"]).args(names);
        self.runner.run(&spec)
    }
}

fn parse_test_targets(stdout: &str) -> BuildResult<Vec<TestTarget>> {
    let mut targets = Vec::new();
    for line in stdout.lines().filter(|line| line.starts_with('{')) {
        let artifact: TestArtifact =
            serde_json::from_str(line).map_err(|error| BuildError::UnexpectedOutput {
                origin: "cargo test --no-run --message-format json".into(),
                detail: format!("{error} in {line:?}; expected one JSON message per line"),
            })?;
        if let Some(target) = test_target(artifact).filter(|target| !targets.contains(target)) {
            targets.push(target);
        }
    }
    Ok(targets)
}

fn test_target(artifact: TestArtifact) -> Option<TestTarget> {
    let is_test = artifact.profile.is_some_and(|profile| profile.test);
    if artifact.reason != "compiler-artifact" || !is_test || artifact.executable.is_none() {
        return None;
    }
    let target = artifact.target?;
    let kind = target.kind.first()?.clone();
    let library = target
        .kind
        .iter()
        .any(|kind| LIBRARY_KINDS.contains(&kind.as_str()));
    let selector = if library {
        vec!["--lib".to_owned()]
    } else {
        vec![format!("--{kind}"), target.name.clone()]
    };
    let key = format!("{kind}:{}", target.name);
    Some(TestTarget {
        key,
        selector,
        library,
    })
}

/// Test names from libtest's terse listing; benchmarks run once as tests too.
fn parse_listed_tests(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .filter_map(|line| {
            line.strip_suffix(": test")
                .or_else(|| line.strip_suffix(": benchmark"))
        })
        .map(str::to_owned)
        .collect()
}

fn filter_chunks(names: Vec<String>) -> Vec<Vec<String>> {
    let mut chunks: Vec<Vec<String>> = Vec::new();
    let mut bytes = FILTER_BYTES;
    for name in names {
        if bytes + name.len() + 1 > FILTER_BYTES {
            chunks.push(Vec::new());
            bytes = 0;
        }
        bytes += name.len() + 1;
        if let Some(chunk) = chunks.last_mut() {
            chunk.push(name);
        }
    }
    chunks
}

#[cfg(test)]
#[path = "test_part_tests.rs"]
mod tests;
