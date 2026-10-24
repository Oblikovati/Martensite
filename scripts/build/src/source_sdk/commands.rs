//! Offline child processes use private dependency and build caches (#279).
use crate::{
    error::{BuildError, BuildResult},
    process::{CommandRunner, CommandSpec},
};
use std::path::Path;

pub(super) fn output(runner: &dyn CommandRunner, spec: &CommandSpec) -> BuildResult<Vec<u8>> {
    let output = runner.capture_output(spec)?;
    if output.code != 0 {
        return Err(BuildError::ReleaseRule(format!(
            "{} exited {}: {}; expected successful SDK command",
            spec.display(),
            output.code,
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(output.stdout)
}

pub(super) fn isolated(root: &Path, home: &Path, target: &Path) -> CommandSpec {
    clean_process("cargo", root, home)
        .environment_os("CARGO_HOME", Some(home.as_os_str()))
        .environment_os("CARGO_TARGET_DIR", Some(target.as_os_str()))
        .environment("CARGO_NET_OFFLINE", Some("true"))
        .environment("CARGO_BUILD_JOBS", Some("2"))
        .environment("CARGO_INCREMENTAL", Some("0"))
        .environment("CARGO_PROFILE_DEV_DEBUG", Some("0"))
        .environment("CARGO_PROFILE_TEST_DEBUG", Some("0"))
}

pub(super) fn clean_process(program: &str, root: &Path, home: &Path) -> CommandSpec {
    let mut spec = CommandSpec::new(program, root).clear_environment();
    for name in NATIVE_ENVIRONMENT {
        spec = spec.environment_os(name, std::env::var_os(name).as_deref());
    }
    spec = private_directories(spec, home);
    spec.environment_os("RUSTUP_HOME", installed_rustup_home().as_deref())
}

// Installed toolchains and declared native SDK discovery remain prerequisites.
// Credentials, compiler wrappers, Cargo configuration and flags are never inherited (#279).
const NATIVE_ENVIRONMENT: &[&str] = &[
    "PATH",
    "SystemRoot",
    "WINDIR",
    "COMSPEC",
    "PATHEXT",
    "SYSTEMDRIVE",
    "ProgramFiles",
    "ProgramFiles(x86)",
    "ProgramW6432",
    "VCINSTALLDIR",
    "VSINSTALLDIR",
    "VCToolsInstallDir",
    "WindowsSdkDir",
    "WindowsSDKVersion",
    "UCRTVersion",
    "UniversalCRTSdkDir",
    "SDKROOT",
    "DEVELOPER_DIR",
    "MACOSX_DEPLOYMENT_TARGET",
    "PKG_CONFIG_PATH",
    "PKG_CONFIG_LIBDIR",
    "PKG_CONFIG_SYSROOT_DIR",
    "LIB",
    "LIBPATH",
    "INCLUDE",
    "LIBRARY_PATH",
    "LD_LIBRARY_PATH",
    "DYLD_LIBRARY_PATH",
    "DYLD_FALLBACK_LIBRARY_PATH",
];

fn private_directories(mut spec: CommandSpec, home: &Path) -> CommandSpec {
    for name in [
        "HOME",
        "USERPROFILE",
        "APPDATA",
        "LOCALAPPDATA",
        "XDG_CONFIG_HOME",
        "XDG_CACHE_HOME",
    ] {
        spec = spec.environment_os(name, Some(home.as_os_str()));
    }
    for name in ["TMP", "TEMP", "TMPDIR"] {
        spec = spec.environment_os(name, Some(home.as_os_str()));
    }
    spec.environment("LC_ALL", Some("C"))
        .environment("LANG", Some("C"))
}

fn installed_rustup_home() -> Option<std::ffi::OsString> {
    std::env::var_os("RUSTUP_HOME").or_else(|| {
        std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map(|home| Path::new(&home).join(".rustup").into_os_string())
    })
}

pub(super) fn reject_ancestor_configuration(root: &Path) -> BuildResult<()> {
    let physical =
        std::fs::canonicalize(root).map_err(|cause| crate::error::io_error(root, cause))?;
    for ancestor in physical.ancestors().skip(1) {
        for name in [".cargo/config", ".cargo/config.toml"] {
            let path = ancestor.join(name);
            if path.exists() {
                return Err(BuildError::ReleaseRule(format!(
                    "SDK ancestor Cargo configuration {path:?}; expected no inherited ancestor configuration"
                )));
            }
        }
    }
    Ok(())
}
