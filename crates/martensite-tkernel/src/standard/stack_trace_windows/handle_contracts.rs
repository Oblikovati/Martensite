use winsafe::co::ERROR;

fn observed_handle(query: impl FnOnce() -> usize, incoming: ERROR) -> (usize, ERROR) {
    let saved = winsafe::GetLastError();
    winsafe::SetLastError(incoming);
    let handle = query();
    let observed = winsafe::GetLastError();
    winsafe::SetLastError(saved);
    (handle, observed)
}

fn compare_symbol_handle() {
    for incoming in [ERROR::SUCCESS, ERROR::ACCESS_DENIED, ERROR::INVALID_DATA] {
        let original = observed_handle(|| super::symbol_process_handle() as usize, incoming);
        let wrapper = observed_handle(
            || winsafe::HPROCESS::GetCurrentProcess().ptr() as usize,
            incoming,
        );
        assert_ne!(original.0, 0);
        assert_eq!(original.1, incoming);
        assert_eq!(original, wrapper);
    }
}

#[test]
fn symbol_process_handle_matches_wrapper() {
    compare_symbol_handle();
}

#[test]
fn symbol_process_handle_matches_on_spawned_thread() {
    std::thread::spawn(compare_symbol_handle).join().unwrap();
}
