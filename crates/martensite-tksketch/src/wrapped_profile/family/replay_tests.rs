//! Compare retained bytes and authored history across processes and overlapping operations.
use super::tests::requests;
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::CylindricalMeasureLimit;
use std::fmt::Write;
#[path = "replay_stress.rs"]
mod stress;
const WORKER: &str = "surface_sketch::wrapped_profile::family::replay_tests::family_replay_worker";
const CHILD_ENV: &str = "MARTENSITE_RATIONAL_FAMILY_REPLAY";
const PREFIX: &str = "RATIONAL_FAMILY_REPLAY=";
fn signature(reverse: bool) -> String {
    let mut inputs = requests();
    if reverse {
        inputs.reverse();
    }
    let result = construct_budgeted(&inputs, CylindricalMeasureLimit::Report)
        .unwrap_or_else(|e| panic!("{e}"));
    signature_of(&result)
}
fn signature_of(result: &BudgetedFamily) -> String {
    let mut signature = format!(
        "{:?}|{:?}|{:?}|",
        result.family.keys, result.family.history, result.budget
    );
    append_members(&mut signature, &result.family.members);
    append_bytes(&mut signature, &result.family.bytes);
    signature
}

fn append_bytes(signature: &mut String, bytes: &[u8]) {
    signature.reserve(bytes.len() * 2);
    for byte in bytes {
        write!(signature, "{byte:02x}").unwrap();
    }
}
#[test]
#[ignore = "subprocess worker, invoked by the independent-process replay control"]
fn family_replay_worker() {
    let order = std::env::var(CHILD_ENV).unwrap();
    assert!(matches!(order.as_str(), "forward" | "reverse"));
    println!("{PREFIX}{}", signature(order == "reverse"));
}
#[test]
fn independent_processes_replay_frozen_bytes_history_and_measure_evidence() {
    let expected = signature(false);
    for order in ["forward", "reverse"] {
        let actual = child_signature(order);
        assert!(
            actual == expected,
            "rational family process replay differs for {order}"
        );
    }
}
fn child_signature(order: &str) -> String {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", WORKER, "--ignored", "--nocapture"])
        .env(CHILD_ENV, order)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "child status={:?}, stderr={}, stdout={}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout
        .lines()
        .find_map(|line| line.strip_prefix(PREFIX))
        .unwrap()
        .to_owned()
}
#[test]
fn overlapping_operations_and_later_sequential_replay_have_identical_evidence() {
    let expected = signature(false);
    let barrier = std::sync::Barrier::new(2);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..2)
            .map(|worker| {
                let (expected, barrier) = (&expected, &barrier);
                scope.spawn(move || repeated_signatures(worker, expected, barrier))
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
    assert!(
        signature(true) == expected,
        "sequential replay changed after concurrent operations"
    );
}
fn repeated_signatures(worker: usize, expected: &str, barrier: &std::sync::Barrier) {
    barrier.wait();
    for iteration in 0..2 {
        let actual = signature((worker + iteration) % 2 == 1);
        assert!(
            actual == expected,
            "concurrent family replay differs: worker={worker}, iteration={iteration}"
        );
    }
}

fn append_members(
    signature: &mut String,
    members: &BTreeMap<SurfaceRegionId, pipeline::RetainedMember>,
) {
    for (id, member) in members {
        write!(
            signature,
            "{id:?}|{:?}|{:?}|{:?}|{:?}|{:?}|",
            member.coverage,
            member.report,
            member.volume_estimate.to_bits(),
            member.integration_estimate.to_bits(),
            member.measures.comparison
        )
        .unwrap();
    }
}
