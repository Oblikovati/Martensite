use std::sync::mpsc;
use winsafe::co::ERROR;

fn live_child_identity() -> (usize, usize, usize) {
    let (report, reported) = mpsc::channel();
    let (release, released) = mpsc::channel::<()>();
    let child = std::thread::spawn(move || {
        report
            .send((
                super::current_thread_id(),
                winsafe::GetCurrentThreadId() as usize,
            ))
            .unwrap();
        let _ = released.recv();
    });
    let observed = reported.recv();
    let handle_id = super::thread_id(&child);
    drop(release);
    child.join().unwrap();
    let (child_id, wrapper_id) = observed.unwrap();
    (child_id, wrapper_id, handle_id)
}

#[test]
fn current_identity_matches_live_child_handle() {
    let parent = super::current_thread_id();
    let (child_id, wrapper_id, handle_id) = live_child_identity();
    assert_eq!(child_id, handle_id);
    assert_eq!(child_id, wrapper_id);
    assert_ne!(child_id, 0);
    assert_ne!(parent, child_id);
    assert_eq!(parent, super::current_thread_id());
}

#[test]
fn processor_count_keeps_signed_cast_boundaries() {
    for (raw, expected) in [
        (0, 0),
        (1, 1),
        (0x7fff_ffff, i32::MAX),
        (0x8000_0000, i32::MIN),
        (u32::MAX, -1),
    ] {
        assert_eq!(super::windows_processor_count(raw), expected);
    }
}

fn observed_error(call: impl FnOnce(), incoming: ERROR) -> ERROR {
    let saved = winsafe::GetLastError();
    winsafe::SetLastError(incoming);
    call();
    let observed = winsafe::GetLastError();
    winsafe::SetLastError(saved);
    observed
}

#[test]
fn current_id_matches_wrapper_last_error() {
    for incoming in [ERROR::SUCCESS, ERROR::ACCESS_DENIED, ERROR::INVALID_DATA] {
        let original = observed_error(
            || {
                let _ = super::current_thread_id();
            },
            incoming,
        );
        let wrapper = observed_error(
            || {
                let _ = winsafe::GetCurrentThreadId();
            },
            incoming,
        );
        assert_eq!(original, wrapper, "incoming LastError: {incoming:?}");
    }
}

#[test]
fn processor_query_matches_wrapper_last_error() {
    for incoming in [ERROR::SUCCESS, ERROR::ACCESS_DENIED, ERROR::INVALID_DATA] {
        let original = observed_error(
            || {
                let _ = super::online_processors();
            },
            incoming,
        );
        let wrapper = observed_error(
            || {
                let _ = winsafe::GetSystemInfo();
            },
            incoming,
        );
        assert_eq!(original, wrapper, "incoming LastError: {incoming:?}");
    }
}

#[test]
fn wrapper_system_info_starts_with_zero_selected_fields() {
    let info = winsafe::SYSTEM_INFO::default();
    assert_eq!(info.dwPageSize, 0);
    assert!(info.lpMinimumApplicationAddress.is_null());
    assert!(info.lpMaximumApplicationAddress.is_null());
    assert_eq!(info.dwActiveProcessorMask, 0);
    assert_eq!(info.dwNumberOfProcessors, 0);
    assert_eq!(info.dwAllocationGranularity, 0);
    assert_eq!(info.wProcessorLevel, 0);
    assert_eq!(info.wProcessorRevision, 0);
}
