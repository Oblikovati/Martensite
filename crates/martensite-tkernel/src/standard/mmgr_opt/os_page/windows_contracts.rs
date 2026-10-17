use winsafe::co::ERROR;

fn observed_page_size(query: impl FnOnce() -> usize, incoming: ERROR) -> (usize, ERROR) {
    let saved = winsafe::GetLastError();
    winsafe::SetLastError(incoming);
    let size = query();
    let observed = winsafe::GetLastError();
    winsafe::SetLastError(saved);
    (size, observed)
}

#[test]
fn page_size_matches_wrapper_and_last_error() {
    for incoming in [ERROR::SUCCESS, ERROR::ACCESS_DENIED, ERROR::INVALID_DATA] {
        let original = observed_page_size(super::os_page_size, incoming);
        let wrapper = observed_page_size(|| winsafe::GetSystemInfo().dwPageSize as usize, incoming);
        assert!(original.0.is_power_of_two());
        assert_eq!(original, wrapper);
    }
}

#[test]
fn wrapper_page_size_starts_at_zero() {
    assert_eq!(winsafe::SYSTEM_INFO::default().dwPageSize, 0);
}
