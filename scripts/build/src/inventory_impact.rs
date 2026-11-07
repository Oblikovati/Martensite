//! Inputs to the source ratchet and expanded compiler evidence have different costs (#411).

pub(crate) fn source_input(path: &str) -> bool {
    build_input(path)
        || shared_driver_input(path)
        || driver_module(path, "unsafe_inventory")
        || path == "docs/features/safe-toolkits/unsafe-source-baseline.json"
}

pub(crate) fn expanded_input(path: &str) -> bool {
    build_input(path)
        || shared_driver_input(path)
        || driver_module(path, "compiler_inventory")
        || (path.starts_with("docs/features/safe-toolkits/compiler-baselines/")
            && path.ends_with(".json"))
}

fn build_input(path: &str) -> bool {
    // Build scripts and include macros can consume arbitrary bytes, including deleted inputs.
    path.starts_with("crates/")
        || path.starts_with("external/")
        || matches!(
            path.rsplit('/').next(),
            Some("Cargo.toml" | "Cargo.lock" | "build.rs")
        )
        || environment_input(path)
}

fn environment_input(path: &str) -> bool {
    matches!(
        path,
        "rust-toolchain"
            | "rust-toolchain.toml"
            | "install-linux.sh"
            | "install-macos.sh"
            | "install-windows.ps1"
    ) || path.starts_with(".cargo/")
        || path.starts_with("scripts/install/")
        || path.starts_with(".github/actions/")
        || path == ".github/workflows/martensite.yml"
}

fn shared_driver_input(path: &str) -> bool {
    // These modules can change invocation, selection, diagnostics or error propagation.
    [
        "main",
        "cli",
        "pr",
        "context",
        "process",
        "cargo",
        "git",
        "error",
        "report",
        "plan",
        "targets",
        "lint",
        "inventory_impact",
    ]
    .iter()
    .any(|module| driver_module(path, module))
}

fn driver_module(path: &str, module: &str) -> bool {
    let Some(relative) = path.strip_prefix("scripts/build/src/") else {
        return false;
    };
    relative == format!("{module}.rs") || relative.starts_with(&format!("{module}/"))
}

#[cfg(test)]
const BUILD_INPUTS: [&str; 16] = [
    "crates/a/src/lib.rs",
    "crates/a/schema.bin",
    "external/fixtures/input.dat",
    "Cargo.toml",
    "Cargo.lock",
    "scripts/build/Cargo.toml",
    "tools/build.rs",
    "rust-toolchain.toml",
    "rust-toolchain",
    ".cargo/config.toml",
    ".github/workflows/martensite.yml",
    ".github/actions/setup/action.yml",
    "scripts/install/native.sh",
    "install-linux.sh",
    "install-macos.sh",
    "install-windows.ps1",
];

#[cfg(test)]
const SHARED_MODULES: [&str; 13] = [
    "main",
    "cli",
    "pr",
    "context",
    "process",
    "cargo",
    "git",
    "error",
    "report",
    "plan",
    "targets",
    "lint",
    "inventory_impact",
];

#[cfg(test)]
const SOURCE_INPUTS: [&str; 3] = [
    "docs/features/safe-toolkits/unsafe-source-baseline.json",
    "scripts/build/src/unsafe_inventory.rs",
    "scripts/build/src/unsafe_inventory/parser.rs",
];

#[cfg(test)]
const COMPILER_INPUTS: [&str; 3] = [
    "docs/features/safe-toolkits/compiler-baselines/x86_64.json",
    "scripts/build/src/compiler_inventory.rs",
    "scripts/build/src/compiler_inventory/ownership.rs",
];

#[cfg(test)]
const IRRELEVANT_INPUTS: [&str; 12] = [
    "README.md",
    "scripts/build/src/thread_sanitizer.rs",
    "scripts/build/src/publish.rs",
    "scripts/build/src/publication_record.rs",
    "scripts/build/src/pr_inventory_tests.rs",
    "docs/features/safe-toolkits/closeout.md",
    "docs/features/safe-toolkits/host-resolver-inventory.json",
    "docs/features/safe-toolkits/compiler-baselines/README.md",
    "rustfmt.toml",
    "clippy.toml",
    "notesCargo.toml",
    "scripts/build/src/process_tests.rs",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arbitrary_crate_and_build_inputs_require_both_checks() {
        for path in BUILD_INPUTS {
            assert!(source_input(path), "{path}");
            assert!(expanded_input(path), "{path}");
        }
    }

    #[test]
    fn shared_driver_dependencies_require_both_checks() {
        for module in SHARED_MODULES {
            for path in [
                format!("scripts/build/src/{module}.rs"),
                format!("scripts/build/src/{module}/detail.rs"),
            ] {
                assert!(source_input(&path), "{path}");
                assert!(expanded_input(&path), "{path}");
            }
        }
    }

    #[test]
    fn source_and_compiler_baselines_and_implementations_are_independent() {
        for path in SOURCE_INPUTS {
            assert!(source_input(path), "{path}");
            assert!(!expanded_input(path), "{path}");
        }
        for path in COMPILER_INPUTS {
            assert!(!source_input(path), "{path}");
            assert!(expanded_input(path), "{path}");
        }
    }

    #[test]
    fn unrelated_driver_prose_and_historical_evidence_skip_inventories() {
        for path in IRRELEVANT_INPUTS {
            assert!(!source_input(path), "{path}");
            assert!(!expanded_input(path), "{path}");
        }
    }
}
