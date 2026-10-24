//! Martensite's build driver: PR validation and releases, run identically by
//! the single CI workflow and by contributors (`cargo xtask --help`).

mod affected;
mod archive;
mod cargo;
mod changelog;
mod ci_tests;
mod cli;
mod compiler_inventory;
mod context;
mod error;
#[cfg(test)]
mod fake_runner;
mod git;
mod lint;
mod package_manifest;
mod plan;
mod pr;
mod process;
mod publish;
mod release;
mod release_impact;
mod release_manifest;
mod release_scope;
mod report;
mod targets;
#[cfg(test)]
mod unsafe_inventory;
mod version;
mod workspace;

use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    let command = cli::Cli::parse().command;
    match cli::execute(command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
