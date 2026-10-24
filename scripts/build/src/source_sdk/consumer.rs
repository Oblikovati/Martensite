//! Verify extracted libraries and an external consumer without shared Cargo caches.
use super::{commands, files, vendor};
use crate::{
    archive,
    error::{BuildError, BuildResult, io_error},
    package_manifest::parse_table,
    process::CommandRunner,
};
use std::{fs, path::Path};

const SMOKE: &str = r#"#[cfg(test)]
mod tests {
use martensite_tkernel::standard::Failure;
use martensite_tkbrep::topods::Shape;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;
#[test]
fn shared_types_measure_empty_shape() -> Result<(), Failure> {
    let mut properties = GProps::new();
    BRepGProp::surface_properties(&Shape::default(), &mut properties, false, false)?;
    assert_eq!(properties.mass(), 0.0);
    assert_eq!(properties.centre_of_mass().x(), 0.0);
    Ok(())
}
}
"#;

pub(super) fn verify(runner: &dyn CommandRunner, root: &Path, temporary: &Path) -> BuildResult<()> {
    commands::reject_ancestor_configuration(root)?;
    let home = prepare_home(temporary)?;
    let target = temporary.join("consumer-target");
    let channel = toolchain(root)?;
    runner.run(&commands::clean_process("rustup", root, &home).args([
        "run",
        &channel,
        "rustc",
        "--version",
    ]))?;
    let lock = archive::sha256_hex(&root.join("Cargo.lock"))?;
    libraries(runner, root, &home, &target, &channel)?;
    external(runner, root, temporary, &home, &target, &channel)?;
    if lock != archive::sha256_hex(&root.join("Cargo.lock"))? {
        return Err(BuildError::ReleaseRule(
            "SDK consumer changed Cargo.lock; expected committed lock bytes".into(),
        ));
    }
    Ok(())
}

fn prepare_home(temporary: &Path) -> BuildResult<std::path::PathBuf> {
    let home = temporary.join("cargo-home");
    fs::create_dir(&home).map_err(|cause| io_error(&home, cause))?;
    Ok(home)
}

fn libraries(
    runner: &dyn CommandRunner,
    root: &Path,
    home: &Path,
    target: &Path,
    channel: &str,
) -> BuildResult<()> {
    let metadata = commands::output(
        runner,
        &commands::isolated(root, home, target)
            .environment("RUSTUP_TOOLCHAIN", Some(channel))
            .args(["metadata", "--locked", "--offline", "--format-version", "1"]),
    )?;
    validate_metadata(root, &metadata)?;
    runner.run(
        &commands::isolated(root, home, target)
            .environment("RUSTUP_TOOLCHAIN", Some(channel))
            .args([
                "build",
                "--locked",
                "--offline",
                "--workspace",
                "--lib",
                "--all-features",
            ]),
    )
}

fn toolchain(root: &Path) -> BuildResult<String> {
    let path = root.join("rust-toolchain.toml");
    let text = fs::read_to_string(&path).map_err(|cause| io_error(&path, cause))?;
    let table = parse_table(&text)?;
    table
        .get("toolchain")
        .and_then(|v| v.get("channel"))
        .and_then(|v| v.as_str())
        .map(String::from)
        .ok_or_else(|| {
            BuildError::ReleaseRule(format!(
                "SDK toolchain {text:?}; expected toolchain.channel"
            ))
        })
}

fn external(
    runner: &dyn CommandRunner,
    root: &Path,
    temporary: &Path,
    home: &Path,
    target: &Path,
    channel: &str,
) -> BuildResult<()> {
    let consumer = temporary.join("external-consumer");
    files::write(
        &consumer,
        Path::new("Cargo.toml"),
        smoke_manifest(root)?.as_bytes(),
    )?;
    files::write(&consumer, Path::new("src/lib.rs"), SMOKE.as_bytes())?;
    let config = absolute_vendor(root)?;
    files::write(
        &consumer,
        Path::new(".cargo/config.toml"),
        config.as_bytes(),
    )?;
    commands::reject_ancestor_configuration(&consumer)?;
    let cargo = || {
        commands::isolated(&consumer, home, target).environment("RUSTUP_TOOLCHAIN", Some(channel))
    };
    runner.run(&cargo().args(["generate-lockfile", "--offline"]))?;
    runner.run(&cargo().args(["test", "--locked", "--offline"]))
}

fn smoke_manifest(root: &Path) -> BuildResult<String> {
    let mut manifest = String::from(
        "[package]\nname='martensite-sdk-consumer'\nversion='0.0.0'\nedition='2024'\n[dependencies]\n",
    );
    for name in ["tkernel", "tkbrep", "tkgeombase", "tktopalgo"] {
        let path = root.join(format!("crates/martensite-{name}"));
        let value = serde_json::to_string(&path.to_string_lossy())
            .map_err(|cause| BuildError::ReleaseRule(format!("SDK path {path:?}: {cause}")))?;
        manifest.push_str(&format!("martensite-{name}={{path={value}}}\n"));
    }
    Ok(manifest)
}

fn absolute_vendor(root: &Path) -> BuildResult<String> {
    let config = vendor::read_config(root)?;
    let absolute = serde_json::to_string(&root.join("vendor").to_string_lossy())
        .map_err(|cause| BuildError::ReleaseRule(format!("SDK vendor path: {cause}")))?;
    Ok(config.replace("directory = \"vendor\"", &format!("directory = {absolute}")))
}

fn validate_metadata(root: &Path, bytes: &[u8]) -> BuildResult<()> {
    let root = fs::canonicalize(root).map_err(|cause| io_error(root, cause))?;
    for package in package_array(bytes)? {
        validate_package(&root, &package)?;
    }
    Ok(())
}

fn package_array(bytes: &[u8]) -> BuildResult<Vec<serde_json::Value>> {
    let mut value: serde_json::Value = serde_json::from_slice(bytes).map_err(|cause| {
        BuildError::ReleaseRule(format!("SDK metadata: {cause}; expected Cargo JSON"))
    })?;
    let packages = value
        .get_mut("packages")
        .and_then(|v| v.as_array_mut())
        .map(std::mem::take)
        .ok_or_else(|| {
            BuildError::ReleaseRule("SDK metadata packages missing; expected array".into())
        })?;
    if packages.is_empty() {
        return Err(BuildError::ReleaseRule(
            "SDK metadata packages empty; expected production workspace".into(),
        ));
    }
    Ok(packages)
}

fn validate_package(root: &Path, package: &serde_json::Value) -> BuildResult<()> {
    let path = package["manifest_path"].as_str().ok_or_else(|| {
        BuildError::ReleaseRule(format!(
            "SDK manifest_path {:?}; expected a path string",
            package["manifest_path"]
        ))
    })?;
    let resolved = fs::canonicalize(path).map_err(|cause| io_error(Path::new(path), cause))?;
    if !resolved.starts_with(root) {
        return Err(BuildError::ReleaseRule(format!(
            "SDK dependency manifest {path:?}; expected inside {root:?}"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dependency_metadata_outside_the_extracted_tree_is_rejected() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path().join("sdk");
        fs::create_dir(&root).unwrap();
        let outside = directory.path().join("Cargo.toml");
        fs::write(&outside, "[package]").unwrap();
        let metadata = serde_json::json!({"packages":[{"manifest_path":outside}]}).to_string();
        assert!(
            validate_metadata(&root, metadata.as_bytes())
                .unwrap_err()
                .to_string()
                .contains("expected inside")
        );
    }
}
