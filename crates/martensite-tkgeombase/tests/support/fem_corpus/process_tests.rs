//! Process tests behavior and supporting operations.
use super::{fixtures::shard_count, process::*};
#[derive(Clone, Copy)]
enum FakeResult {
    Valid,
    Launch,
    FailedExit,
    WrongHeader,
    Empty,
    Duplicate,
    InvalidUtf8,
    Stderr,
}
struct NamedShardRunner {
    result: FakeResult,
}
impl ChildRunner for NamedShardRunner {
    fn run(&self, shard: usize) -> Result<ChildOutput, ReplayFailure> {
        if matches!(self.result, FakeResult::Launch) {
            return Err(ReplayFailure::Launch {
                shard,
                cause: "named launch failure".into(),
            });
        }
        let rows = shard_count(shard);
        let receipt = ShardReceipt {
            shard,
            rows,
            actual: "0 \n".repeat(rows),
        }
        .protocol();
        let stdout = if matches!(self.result, FakeResult::InvalidUtf8) {
            vec![0xff]
        } else {
            fake_stdout(self.result, shard, &receipt).into_bytes()
        };
        Ok(ChildOutput {
            success: !matches!(self.result, FakeResult::FailedExit),
            code: Some(23),
            stdout,
            stderr: if matches!(self.result, FakeResult::Stderr | FakeResult::FailedExit) {
                "named stderr".into()
            } else {
                String::new()
            },
        })
    }
}
fn fake_stdout(result: FakeResult, shard: usize, receipt: &str) -> String {
    match result {
        FakeResult::WrongHeader => receipt.replacen(
            &format!("FEM_SHARD_BEGIN:{shard}:"),
            "FEM_SHARD_BEGIN:65:",
            1,
        ),
        FakeResult::Empty => format!(
            "FEM_SHARD_BEGIN:{shard}:{}\nFEM_SHARD_END:{shard}\n",
            shard_count(shard)
        ),
        FakeResult::Duplicate => receipt.repeat(2),
        _ => format!("running 1 test\n{receipt}test result: ok; finished in 9.87s\n"),
    }
}
#[test]
fn all_positive_shards_are_required_and_harness_timing_is_not_payload() {
    let receipts = run_original_shards(&NamedShardRunner {
        result: FakeResult::Valid,
    })
    .unwrap();
    assert_eq!(receipts.len(), 64);
    assert_eq!(
        receipts.iter().map(|receipt| receipt.rows).sum::<usize>(),
        1230
    );
    assert_eq!(receipts[0].actual, "0 \n".repeat(shard_count(0)));
}
#[test]
fn child_launch_and_nonzero_status_propagate_with_shard_and_cause() {
    for result in [FakeResult::Launch, FakeResult::FailedExit] {
        let error = run_one_shard(&NamedShardRunner { result }, 3).unwrap_err();
        assert!(matches!(
            error,
            ReplayFailure::Launch { shard: 3, .. }
                | ReplayFailure::Exit {
                    shard: 3,
                    code: Some(23),
                    ..
                }
        ));
    }
}
#[test]
fn malformed_duplicate_empty_or_diagnostic_child_outputs_fail() {
    for result in [
        FakeResult::WrongHeader,
        FakeResult::Empty,
        FakeResult::Duplicate,
        FakeResult::InvalidUtf8,
        FakeResult::Stderr,
    ] {
        assert!(matches!(
            run_one_shard(&NamedShardRunner { result }, 2),
            Err(ReplayFailure::Protocol { shard: 2, .. })
        ));
    }
}
#[test]
fn shard_selection_requires_an_explicit_in_range_integer() {
    assert_eq!(parse_shard("0"), Ok(0));
    assert_eq!(parse_shard("63"), Ok(63));
    for value in ["64", "-1", "", "invalid"] {
        assert!(parse_shard(value).unwrap_err().contains(value));
    }
}
