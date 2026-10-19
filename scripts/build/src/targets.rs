//! The platforms a release is built, tested and packaged for. This list is
//! the single source of the CI release matrix (`ci plan` emits it).

/// One release platform and the GitHub-hosted runner image that builds it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReleaseTarget {
    /// The rustc target triple.
    pub triple: &'static str,
    /// The runner label; pinned images keep releases reproducible.
    pub runner: &'static str,
}

/// Release platforms. Each builds natively, so tests run on the target itself.
pub const RELEASE_TARGETS: [ReleaseTarget; 4] = [
    ReleaseTarget {
        triple: "x86_64-unknown-linux-gnu",
        runner: "ubuntu-24.04",
    },
    ReleaseTarget {
        triple: "x86_64-apple-darwin",
        runner: "macos-15-intel",
    },
    ReleaseTarget {
        triple: "aarch64-apple-darwin",
        runner: "macos-15",
    },
    ReleaseTarget {
        triple: "x86_64-pc-windows-msvc",
        runner: "windows-2025",
    },
];

/// The platform that also runs PR validation and packages the source crates,
/// which are platform-independent and must be produced exactly once.
pub const PRIMARY_TARGET: ReleaseTarget = RELEASE_TARGETS[0];

/// Whether `triple` is the primary target.
///
/// ```ignore
/// if is_primary("x86_64-unknown-linux-gnu") { /* package sources */ }
/// ```
pub fn is_primary(triple: &str) -> bool {
    triple == PRIMARY_TARGET.triple
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_platforms_bind_exact_native_runners() {
        let actual: Vec<_> = RELEASE_TARGETS
            .iter()
            .map(|target| (target.triple, target.runner))
            .collect();
        assert_eq!(
            actual,
            [
                ("x86_64-unknown-linux-gnu", "ubuntu-24.04"),
                ("x86_64-apple-darwin", "macos-15-intel"),
                ("aarch64-apple-darwin", "macos-15"),
                ("x86_64-pc-windows-msvc", "windows-2025"),
            ]
        );
    }

    #[test]
    fn triples_are_unique_and_primary_is_listed() {
        let mut triples: Vec<_> = RELEASE_TARGETS.iter().map(|target| target.triple).collect();
        triples.sort_unstable();
        triples.dedup();
        assert_eq!(triples.len(), RELEASE_TARGETS.len());
        assert!(is_primary("x86_64-unknown-linux-gnu"));
        assert!(!is_primary("aarch64-apple-darwin"));
    }
}
