//! Separate processes must publish identical physical bytes, authored history and reports.
use super::*;
const WORKER: &str =
    "surface_sketch::wrapped_profile::api::placement::replay_tests::physical_replay_worker";
const CHILD_ENV: &str = "MARTENSITE_WRAPPED_OBLIQUE_REPLAY";
const PREFIX: &str = "WRAPPED_OBLIQUE_REPLAY=";
fn signature(reverse: bool) -> String {
    let mut request = tests::request();
    if reverse {
        request.selections.reverse();
    }
    let result = normal_extrude_profile(&request).unwrap();
    format!(
        "{:?}|{:?}|{:?}|{:?}|{:02x?}",
        result.history(),
        result.budget(),
        result.placement(),
        result.reports(),
        result.encoded_brep()
    )
}
#[test]
#[ignore = "subprocess worker, invoked by the independent-process physical replay control"]
fn physical_replay_worker() {
    let order = std::env::var(CHILD_ENV).unwrap();
    assert!(matches!(order.as_str(), "forward" | "reverse"));
    println!("{PREFIX}{}", signature(order == "reverse"));
}
#[test]
fn independent_processes_replay_physical_geometry_history_and_reports() {
    let expected = signature(false);
    for order in ["forward", "reverse"] {
        assert!(
            child_signature(order) == expected,
            "physical process replay differs for {order}"
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
