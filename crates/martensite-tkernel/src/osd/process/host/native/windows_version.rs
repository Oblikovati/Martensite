//! The native version query has no std or WinSafe equivalent (#334).
#![allow(unsafe_code)]
use super::SystemNames;
use windows_sys::Win32::System::SystemInformation::OSVERSIONINFOW;

#[link(name = "ntdll")]
unsafe extern "system" {
    fn RtlGetVersion(version: *mut OSVERSIONINFOW) -> i32;
    fn RtlNtStatusToDosError(status: i32) -> u32;
}

pub(super) fn system_names() -> Result<SystemNames, i32> {
    let version = query_version()?;
    Ok(SystemNames {
        sysname: b"Windows NT".to_vec(),
        release: format!(
            "{}.{}.{}",
            version.dwMajorVersion, version.dwMinorVersion, version.dwBuildNumber
        )
        .into_bytes(),
    })
}

fn query_version() -> Result<OSVERSIONINFOW, i32> {
    let mut version = OSVERSIONINFOW {
        dwOSVersionInfoSize: std::mem::size_of::<OSVERSIONINFOW>() as u32,
        ..Default::default()
    };
    // SAFETY: the initialized output matches RTL_OSVERSIONINFOW's C layout and
    // advertises its actual size. The API retains no pointer to this stack value.
    let status = unsafe { RtlGetVersion(&mut version) };
    if status != 0 {
        // SAFETY: the function accepts any NTSTATUS by value and retains no state.
        return Err(unsafe { RtlNtStatusToDosError(status) } as i32);
    }
    Ok(version)
}

#[cfg(test)]
mod tests {
    #[test]
    fn native_version_contains_major_minor_and_build() {
        let names = super::system_names().unwrap();
        assert_eq!(names.sysname, b"Windows NT");
        let release = std::str::from_utf8(&names.release).unwrap();
        let fields: Vec<u32> = release
            .split('.')
            .map(|field| field.parse().unwrap())
            .collect();
        assert_eq!(fields.len(), 3);
        assert!(fields[0] >= 6);
        assert!(fields[2] > 0);
    }
}
