use windows_sys::Win32::Foundation::HANDLE;
use winsafe::{GetLastError, SetLastError, co::ERROR};

#[test]
fn current_handles_match_safe_wrapper_on_the_calling_thread() {
    compare_current_handles();
}

#[test]
fn current_handles_match_safe_wrapper_on_a_spawned_thread() {
    std::thread::spawn(compare_current_handles).join().unwrap();
}

#[test]
fn current_cpu_queries_are_monotonic_on_the_calling_thread() {
    check_current_cpu_queries();
}

#[test]
fn current_cpu_queries_are_monotonic_on_a_spawned_thread() {
    std::thread::spawn(check_current_cpu_queries)
        .join()
        .unwrap();
}

fn check_current_cpu_queries() {
    for query in [super::process_cpu, super::thread_cpu] {
        let before = query();
        let after = query();
        for (earlier, later) in [(before.0, after.0), (before.1, after.1)] {
            assert!(earlier.is_finite() && earlier >= 0.0);
            assert!(later.is_finite() && later >= earlier);
        }
    }
}

fn compare_current_handles() {
    for sentinel in [ERROR::SUCCESS, ERROR::ACCESS_DENIED, ERROR::INVALID_DATA] {
        compare_handle(
            super::current_process_handle,
            || winsafe::HPROCESS::GetCurrentProcess().ptr(),
            sentinel,
        );
        compare_handle(
            super::current_thread_handle,
            || winsafe::HTHREAD::GetCurrentThread().ptr(),
            sentinel,
        );
    }
}

fn compare_handle(original: fn() -> HANDLE, candidate: fn() -> HANDLE, incoming: ERROR) {
    let expected = capture_handle(original, incoming);
    let observed = capture_handle(candidate, incoming);
    assert_ne!(expected.0, 0);
    assert_eq!(expected.1, incoming);
    assert_eq!(observed, expected);
}

fn capture_handle(query: fn() -> HANDLE, incoming: ERROR) -> (usize, ERROR) {
    let previous = GetLastError();
    SetLastError(incoming);
    let handle = query();
    let observed = GetLastError();
    SetLastError(previous);
    (handle as usize, observed)
}
