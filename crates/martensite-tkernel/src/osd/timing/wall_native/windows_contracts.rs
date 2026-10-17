//! Fresh processes separate the first-call cache from warmed observations (#89).
use winsafe::{GetLastError, SetLastError, co::ERROR};

const SENTINEL_CASE: &str = "MARTENSITE_WALL_CLOCK_SENTINEL_CASE";
const SENTINELS: [ERROR; 3] = [ERROR::SUCCESS, ERROR::ACCESS_DENIED, ERROR::INVALID_DATA];
const CHILD_TEST: &str = "osd::timing::wall_native::windows_contracts::isolated_native_wall_clock";
const OBSERVATIONS_COMPLETE: &str = "{\"wall_clock_observations\":3}";

#[test]
fn native_wall_clock_first_and_warm_calls_in_fresh_processes() {
    for index in 0..SENTINELS.len() {
        run_isolated_case(index);
    }
}

fn run_isolated_case(index: usize) {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", CHILD_TEST, "--ignored", "--nocapture"])
        .env(SENTINEL_CASE, index.to_string())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success() && stdout.contains(OBSERVATIONS_COMPLETE),
        "child {index} expected three completed observations: {stdout}{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
#[ignore = "invoked in a fresh child by native_wall_clock_first_and_warm_calls_in_fresh_processes"]
fn isolated_native_wall_clock() {
    let Ok(index) = std::env::var(SENTINEL_CASE) else {
        return;
    };
    let incoming = SENTINELS[index.parse::<usize>().unwrap()];
    let frequency = winsafe::QueryPerformanceFrequency().unwrap();
    assert!(frequency > 0);
    check_wall_observation(frequency, incoming, true);
    check_wall_observation(frequency, incoming, false);
    check_wall_observation(frequency, incoming, false);
    println!("{OBSERVATIONS_COMPLETE}");
}

fn check_wall_observation(frequency: i64, incoming: ERROR, first: bool) {
    let before = winsafe::QueryPerformanceCounter().unwrap() as f64 / frequency as f64;
    let (observed, actual_error) = capture_clock_error(super::wall_clock_seconds, incoming);
    let after = winsafe::QueryPerformanceCounter().unwrap() as f64 / frequency as f64;
    let (_, expected_error) = capture_clock_error(|| wrapper_seconds(frequency, first), incoming);
    assert!(
        observed.is_finite() && before <= observed && observed <= after,
        "wall seconds {observed} expected within [{before}, {after}]"
    );
    assert_eq!(actual_error, expected_error, "first call: {first}");
}

fn wrapper_seconds(frequency: i64, first: bool) -> f64 {
    if first {
        let _ = winsafe::QueryPerformanceFrequency().unwrap();
    }
    winsafe::QueryPerformanceCounter().unwrap() as f64 / frequency as f64
}

fn capture_clock_error(query: impl FnOnce() -> f64, incoming: ERROR) -> (f64, ERROR) {
    let previous = GetLastError();
    SetLastError(incoming);
    let seconds = query();
    let observed = GetLastError();
    SetLastError(previous);
    (seconds, observed)
}
