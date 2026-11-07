//! Geometry bytes, semantic bindings and metadata are independent of process and scheduling.
pub(crate) mod geometry_bytes;
use super::refusals::{options, rectangle};
use martensite_tksketch::surface_sketch::normal_extrude;

fn signature() -> String {
    let result = normal_extrude(&rectangle(), [-1., 2.], options()).unwrap();
    format!(
        "{:?}|{:?}|{:?}|{:?}|{}",
        result.history(),
        result.volume(),
        result.volume_error_estimates(),
        result.validation(),
        geometry_bytes::signature(&result.export_brep().unwrap())
    )
}
#[test]
fn parallel_operations_match_sequential_value_metadata() {
    let expected = signature();
    let results = std::thread::scope(|scope| {
        let tasks: Vec<_> = (0..8).map(|_| scope.spawn(repeated_signatures)).collect();
        tasks
            .into_iter()
            .flat_map(|task| task.join().unwrap())
            .collect::<Vec<_>>()
    });
    assert!(results.iter().all(|actual| actual == &expected));
    crate::cone_acceptance::replay::assert_parallel_matches_sequential();
}
#[test]
fn child_signature() {
    if std::env::var_os("MARTENSITE_SURFACE_REPLAY_CHILD").is_some() {
        println!("SURFACE_REPLAY={}", signature());
    }
}
#[test]
fn independent_processes_match_value_metadata() {
    let expected = format!("SURFACE_REPLAY={}", signature());
    for _ in 0..2 {
        let stdout = child_output();
        assert_eq!(
            stdout
                .lines()
                .find(|line| line.starts_with("SURFACE_REPLAY=")),
            Some(expected.as_str())
        );
    }
}

fn child_output() -> String {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "replay::child_signature", "--nocapture"])
        .env("MARTENSITE_SURFACE_REPLAY_CHILD", "1")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn repeated_signatures() -> Vec<String> {
    (0..4).map(|_| signature()).collect()
}
