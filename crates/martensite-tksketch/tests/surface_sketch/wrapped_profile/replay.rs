//! Compare the public publication, including frozen geometry, without native-handle identity.
use super::*;

fn signature() -> String {
    let result = normal_extrude_profile(&request()).unwrap();
    format!(
        "WRAPPED_REPLAY={:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
        result.encoded_brep(),
        result.history(),
        result.reports(),
        result.budget(),
        result.material_probes(),
        result.material_relations().collect::<Vec<_>>(),
        result.material_locations().collect::<Vec<_>>(),
        result.material_work(),
    )
}
#[test]
#[ignore = "subprocess worker; explicitly executed by independent_processes_match"]
fn child_signature() {
    println!("{}", signature());
}
fn child_output() -> String {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "wrapped_profile::replay::child_signature",
            "--ignored",
            "--nocapture",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}
#[test]
fn independent_processes_match() {
    let expected = signature();
    for _ in 0..2 {
        let output = child_output();
        assert_eq!(
            output
                .lines()
                .find(|line| line.starts_with("WRAPPED_REPLAY=")),
            Some(expected.as_str())
        );
    }
}
#[test]
fn concurrent_publications_match_sequential_receipts() {
    let before = signature();
    let actual = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..4).map(|_| scope.spawn(signature)).collect();
        workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect::<Vec<_>>()
    });
    assert!(actual.iter().all(|value| value == &before));
    assert_eq!(signature(), before);
}
