//! Process behavior and supporting operations.
use super::fixtures::shard_count;
use std::{path::PathBuf, process::Command};
const CHILD_ENV: &str = "MARTENSITE_FEM_CORPUS_CHILD_SHARD";
const TEST: &str = "all_reference_matrix_shards_match_frozen_answers";
const BEGIN: &str = "FEM_SHARD_BEGIN:";

#[derive(Debug, PartialEq, Eq)]
pub struct ShardReceipt {
    pub shard: usize,
    pub rows: usize,
    pub actual: String,
}
impl ShardReceipt {
    pub fn protocol(&self) -> String {
        format!(
            "{BEGIN}{}:{}\n{}FEM_SHARD_END:{}\n",
            self.shard, self.rows, self.actual, self.shard
        )
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum ReplayFailure {
    Launch {
        shard: usize,
        cause: String,
    },
    Exit {
        shard: usize,
        code: Option<i32>,
        stderr: String,
    },
    Protocol {
        shard: usize,
        cause: String,
    },
}
pub struct ChildOutput {
    pub success: bool,
    pub code: Option<i32>,
    pub stdout: Vec<u8>,
    pub stderr: String,
}
pub trait ChildRunner {
    fn run(&self, shard: usize) -> Result<ChildOutput, ReplayFailure>;
}
pub struct CurrentTestProcess {
    executable: PathBuf,
}
impl CurrentTestProcess {
    pub fn new(executable: PathBuf) -> Self {
        Self { executable }
    }
}
impl ChildRunner for CurrentTestProcess {
    fn run(&self, shard: usize) -> Result<ChildOutput, ReplayFailure> {
        let output = Command::new(&self.executable)
            .args(["--exact", TEST, "--nocapture", "--test-threads=1"])
            .env(CHILD_ENV, shard.to_string())
            .output()
            .map_err(|error| ReplayFailure::Launch {
                shard,
                cause: error.to_string(),
            })?;
        Ok(ChildOutput {
            success: output.status.success(),
            code: output.status.code(),
            stdout: output.stdout,
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}
pub fn child_shard() -> Result<Option<usize>, String> {
    match std::env::var(CHILD_ENV) {
        Ok(value) => parse_shard(&value).map(Some),
        Err(std::env::VarError::NotPresent) => Ok(None),
        Err(error) => Err(format!(
            "{CHILD_ENV}: {error}; expected ASCII integer 0..64"
        )),
    }
}
pub(super) fn parse_shard(value: &str) -> Result<usize, String> {
    value
        .parse::<usize>()
        .ok()
        .filter(|shard| *shard < 64)
        .ok_or_else(|| format!("child shard {value:?}; expected integer 0..64"))
}
pub fn run_original_shards(runner: &impl ChildRunner) -> Result<Vec<ShardReceipt>, ReplayFailure> {
    (0..64).map(|shard| run_one_shard(runner, shard)).collect()
}
pub(super) fn run_one_shard(
    runner: &impl ChildRunner,
    shard: usize,
) -> Result<ShardReceipt, ReplayFailure> {
    let output = runner.run(shard)?;
    if !output.success {
        return Err(ReplayFailure::Exit {
            shard,
            code: output.code,
            stderr: output.stderr,
        });
    }
    if !output.stderr.is_empty() {
        return protocol_error(
            shard,
            format!("unexpected child stderr {:?}", output.stderr),
        );
    }
    let stdout = std::str::from_utf8(&output.stdout).map_err(|error| ReplayFailure::Protocol {
        shard,
        cause: format!("invalid UTF-8 child stdout: {error}; expected original numeric text"),
    })?;
    parse_receipt(stdout, shard)
}
fn protocol_error<T>(shard: usize, cause: String) -> Result<T, ReplayFailure> {
    Err(ReplayFailure::Protocol { shard, cause })
}
fn parse_receipt(stdout: &str, shard: usize) -> Result<ShardReceipt, ReplayFailure> {
    let pieces: Vec<&str> = stdout.split(BEGIN).collect();
    if pieces.len() != 2 {
        return protocol_error(
            shard,
            format!("{} receipt markers; expected 1", pieces.len() - 1),
        );
    }
    let rest = validate_header(pieces[1], shard)?;
    let actual = receipt_payload(rest, shard)?;
    validate_rows(shard, actual)
}
fn validate_header(text: &str, shard: usize) -> Result<&str, ReplayFailure> {
    let (header, rest) = text
        .split_once('\n')
        .ok_or_else(|| ReplayFailure::Protocol {
            shard,
            cause: "missing receipt header newline".into(),
        })?;
    let expected_header = format!("{shard}:{}", shard_count(shard));
    if header != expected_header {
        return protocol_error(
            shard,
            format!("header {header:?}; expected {expected_header:?}"),
        );
    }
    Ok(rest)
}
fn receipt_payload(rest: &str, shard: usize) -> Result<&str, ReplayFailure> {
    let end = format!("FEM_SHARD_END:{shard}\n");
    let pieces: Vec<&str> = rest.split(&end).collect();
    if pieces.len() != 2 {
        return protocol_error(
            shard,
            format!("{} end markers; expected 1 {end:?}", pieces.len() - 1),
        );
    }
    Ok(pieces[0])
}

fn validate_rows(shard: usize, actual: &str) -> Result<ShardReceipt, ReplayFailure> {
    let rows = actual.lines().count();
    if rows == 0 || rows != shard_count(shard) {
        return protocol_error(
            shard,
            format!("{rows} rows; expected positive {}", shard_count(shard)),
        );
    }
    Ok(ShardReceipt {
        shard,
        rows,
        actual: actual.into(),
    })
}
