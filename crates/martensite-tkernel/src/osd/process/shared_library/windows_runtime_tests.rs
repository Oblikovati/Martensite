use super::*;

#[test]
fn borrowed_cache_close_does_not_release_another_owners_reference() {
    let root = tempfile::tempdir().unwrap();
    let system = std::path::PathBuf::from(std::env::var_os("SystemRoot").unwrap())
        .join("System32/version.dll");
    let path = root.path().join("martensite-loader-probe.dll");
    std::fs::copy(system, &path).unwrap();
    let name = path.to_str().unwrap();
    let mut owner = SharedLibrary::named(Some(name));
    assert!(owner.dl_open(LoadMode::RtldNow));
    let mut borrowed = SharedLibrary::named(Some(name));
    assert!(!borrowed.handle.is_null());
    borrowed.dl_close();
    assert!(owner.dl_symb("GetFileVersionInfoSizeW").is_some());
    owner.dl_close();
}

#[test]
fn native_library_reopens_and_resolves_symbols_after_close() {
    let mut library = SharedLibrary::named(Some("kernel32.dll"));
    for _ in 0..2 {
        assert!(library.dl_open(LoadMode::RtldNow));
        assert!(library.dl_symb("GetCurrentProcessId").is_some());
        assert!(library.dl_symb("MartensiteMissingSymbol334").is_none());
        library.dl_close();
        assert!(library.dl_symb("GetCurrentProcessId").is_none());
    }
}

#[test]
fn missing_native_library_returns_a_diagnostic() {
    let mut library = SharedLibrary::named(Some("MartensiteMissingLibrary334.dll"));
    assert!(!library.dl_open(LoadMode::RtldNow));
    assert!(!library.dl_error().unwrap_or_default().is_empty());
}
