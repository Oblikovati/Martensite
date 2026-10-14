//! Publishes packaged release assets as a GitHub release through `gh`.

use std::fs;
use std::path::{Path, PathBuf};

use semver::Version;

use crate::archive::verify_checksum;
use crate::changelog::release_notes;
use crate::context::BuildContext;
use crate::error::{BuildError, BuildResult, io_error};
use crate::process::CommandSpec;

/// Verifies every asset in `dist_dir` against its checksum, then creates the
/// GitHub release for the existing `v<version>` tag with the changelog notes.
///
/// ```ignore
/// publish_release(&context, &Version::new(0, 3, 0), Path::new("dist"))?;
/// ```
pub fn publish_release(
    context: &BuildContext<'_>,
    version: &Version,
    dist_dir: &Path,
) -> BuildResult<()> {
    let assets = context
        .reporter
        .step("Verify release assets", || verified_assets(dist_dir))?;
    let notes_path = write_notes(context.root(), version)?;
    let spec = release_command(context.root(), version, &notes_path, &assets);
    context
        .reporter
        .step("Create GitHub release", || context.runner.run(&spec))
}

/// `--verify-tag` refuses to create a tag: the release must name the pushed one.
fn release_command(
    root: &Path,
    version: &Version,
    notes: &Path,
    assets: &[PathBuf],
) -> CommandSpec {
    let tag = format!("v{version}");
    let notes = notes.display().to_string();
    let prerelease = (!version.pre.is_empty()).then_some("--prerelease");
    CommandSpec::new("gh", root)
        .args([
            "release",
            "create",
            tag.as_str(),
            "--verify-tag",
            "--title",
            tag.as_str(),
        ])
        .args(["--notes-file", notes.as_str()])
        .args(prerelease)
        .args(assets.iter().map(|asset| asset.display().to_string()))
}

/// Every file in `dist_dir`, sorted; each payload must match its `.sha256`.
fn verified_assets(dist_dir: &Path) -> BuildResult<Vec<PathBuf>> {
    let assets = sorted_files(dist_dir)?;
    let payloads: Vec<&PathBuf> = assets.iter().filter(|asset| !is_checksum(asset)).collect();
    if payloads.is_empty() {
        return Err(BuildError::ReleaseRule(format!(
            "{} holds no release asset; expected archives or .crate files with .sha256 companions",
            dist_dir.display()
        )));
    }
    payloads
        .into_iter()
        .try_for_each(|payload| verify_checksum(payload))?;
    Ok(assets)
}

fn sorted_files(dir: &Path) -> BuildResult<Vec<PathBuf>> {
    let listing = fs::read_dir(dir).map_err(|cause| io_error(dir, cause))?;
    let entries = listing.map(|entry| entry.map(|entry| entry.path()));
    let mut files: Vec<PathBuf> = entries
        .collect::<Result<_, _>>()
        .map_err(|cause| io_error(dir, cause))?;
    files.sort();
    Ok(files)
}

fn is_checksum(path: &Path) -> bool {
    path.extension()
        .is_some_and(|extension| extension == "sha256")
}

fn write_notes(root: &Path, version: &Version) -> BuildResult<PathBuf> {
    let changelog_path = root.join("CHANGELOG.md");
    let changelog =
        fs::read_to_string(&changelog_path).map_err(|cause| io_error(&changelog_path, cause))?;
    let notes = release_notes(&changelog, &version.to_string())?;
    let notes_path = root
        .join("target")
        .join(format!("release-notes-v{version}.md"));
    fs::create_dir_all(root.join("target")).map_err(|cause| io_error(&notes_path, cause))?;
    fs::write(&notes_path, notes.body).map_err(|cause| io_error(&notes_path, cause))?;
    Ok(notes_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archive::write_checksum;
    use crate::context::tests::context_at;
    use crate::fake_runner::RecordingRunner;

    fn fixture() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("CHANGELOG.md"),
            "## [1.0.0-rc.1]\n\n### Added\n- api\n",
        )
        .unwrap();
        fs::create_dir(dir.path().join("dist")).unwrap();
        let asset = dir.path().join("dist/m.crate");
        fs::write(&asset, "crate").unwrap();
        write_checksum(&asset).unwrap();
        dir
    }

    fn runner(root: &Path) -> RecordingRunner {
        RecordingRunner::default()
            .with_stdout("git rev-parse --show-toplevel", &root.display().to_string())
    }

    #[test]
    fn prerelease_is_published_with_notes_and_all_assets() {
        let dir = fixture();
        let runner = runner(dir.path());
        let version = Version::parse("1.0.0-rc.1").unwrap();
        publish_release(
            &context_at(&runner, dir.path()),
            &version,
            &dir.path().join("dist"),
        )
        .unwrap();
        let gh = runner.commands().pop().unwrap();
        assert!(gh.starts_with(
            "gh release create v1.0.0-rc.1 --verify-tag --title v1.0.0-rc.1 --notes-file"
        ));
        assert!(
            gh.contains("--prerelease") && gh.ends_with("m.crate.sha256"),
            "{gh}"
        );
        let notes =
            fs::read_to_string(dir.path().join("target/release-notes-v1.0.0-rc.1.md")).unwrap();
        assert_eq!(notes, "### Added\n- api");
    }

    #[test]
    fn tampered_asset_blocks_publication() {
        let dir = fixture();
        fs::write(dir.path().join("dist/m.crate"), "changed").unwrap();
        let runner = runner(dir.path());
        let version = Version::parse("1.0.0-rc.1").unwrap();
        assert!(
            publish_release(
                &context_at(&runner, dir.path()),
                &version,
                &dir.path().join("dist")
            )
            .is_err()
        );
        assert!(
            !runner
                .commands()
                .iter()
                .any(|command| command.starts_with("gh "))
        );
    }

    #[test]
    fn empty_dist_is_refused() {
        let dir = tempfile::tempdir().unwrap();
        let error = verified_assets(dir.path()).unwrap_err();
        assert!(error.to_string().contains("holds no release asset"));
    }
}
