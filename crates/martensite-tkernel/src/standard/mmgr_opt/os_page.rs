//! Narrow platform FFI boundary for the page size the manager's initialization reads.
#![allow(unsafe_code)]

#[cfg(all(test, unix, not(target_vendor = "apple")))]
mod page_size_contracts;

/// The system page size (`getpagesize()` / `GetSystemInfo`); `os_page_size() % 4096 == 0`.
#[cfg(all(unix, not(target_vendor = "apple")))]
pub(super) fn os_page_size() -> usize {
    let size = crate::native_config::native_sysconf(nix::unistd::SysconfVar::PAGE_SIZE);
    usize::try_from(size).unwrap_or(0)
}

/// The system page size (`getpagesize()` / `GetSystemInfo`); `os_page_size() % 4096 == 0`.
#[cfg(target_vendor = "apple")]
pub(super) fn os_page_size() -> usize {
    // SAFETY: vm_page_size is an immutable kernel-initialized global of the Mach runtime.
    unsafe { mach2::vm_page_size::vm_page_size }
}

/// The system page size (`getpagesize()` / `GetSystemInfo`); `os_page_size() % 4096 == 0`.
#[cfg(windows)]
pub(super) fn os_page_size() -> usize {
    use windows_sys::Win32::System::SystemInformation::{GetSystemInfo, SYSTEM_INFO};
    // The dependency's safe default preserves the all-zero initial state (#89).
    let mut info = SYSTEM_INFO::default();
    // SAFETY: info is initialized, writable and alive throughout the call.
    unsafe { GetSystemInfo(&mut info) };
    info.dwPageSize as usize
}

#[cfg(all(test, windows))]
mod windows_contracts;

#[cfg(all(test, windows))]
mod system_info_tests {
    #[test]
    fn native_page_size_is_nonzero_and_a_power_of_two() {
        assert!(super::os_page_size().is_power_of_two());
    }
}
