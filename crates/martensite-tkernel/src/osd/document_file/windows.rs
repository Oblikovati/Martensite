//! Windows security-descriptor preparation. No publication API is called here.
//!
//! Access-policy queries must support integrity labels, resource attributes,
//! central policy, process trust and access filters; unavailable queries fail closed.
//! EFS identity cannot be transferred here. Custom writer ACLs and read-only
//! writer outputs are refused instead of widened. Audit-only ACEs are outside
//! this access-policy boundary. Native tests require Windows; cross-checking on
//! Linux establishes type/lint correctness, not Windows security semantics.
#![allow(unsafe_code)]

use std::ffi::c_void;
use std::fs::{self, OpenOptions};
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::fs::MetadataExt;
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
use std::path::{Path, PathBuf};
use std::ptr::null_mut;
use windows_sys::Win32::Foundation::{ERROR_NO_TOKEN, LocalFree};
use windows_sys::Win32::Security::Authorization::{
    ConvertSecurityDescriptorToStringSecurityDescriptorW, ConvertSidToStringSidW,
    ConvertStringSecurityDescriptorToSecurityDescriptorW, GetNamedSecurityInfoW, SE_FILE_OBJECT,
};
use windows_sys::Win32::Security::*;
use windows_sys::Win32::Storage::FileSystem::{CreateDirectoryW, FILE_ATTRIBUTE_ENCRYPTED};
use windows_sys::Win32::System::SystemServices::{
    ACCESS_FILTER_SECURITY_INFORMATION, PROCESS_TRUST_LABEL_SECURITY_INFORMATION,
};
use windows_sys::Win32::System::Threading::{
    GetCurrentProcess, GetCurrentThread, OpenProcessToken, OpenThreadToken,
};

const ACCESS_SECTIONS: [u32; 8] = [
    OWNER_SECURITY_INFORMATION,
    GROUP_SECURITY_INFORMATION,
    DACL_SECURITY_INFORMATION,
    LABEL_SECURITY_INFORMATION,
    ATTRIBUTE_SECURITY_INFORMATION,
    SCOPE_SECURITY_INFORMATION,
    PROCESS_TRUST_LABEL_SECURITY_INFORMATION as u32,
    ACCESS_FILTER_SECURITY_INFORMATION as u32,
];

struct LocalAllocation(*mut c_void);

impl Drop for LocalAllocation {
    fn drop(&mut self) {
        // SAFETY: Each allocation is returned by a documented LocalAlloc-family API;
        // this unique owner frees it once, after every borrowed descriptor pointer.
        unsafe {
            LocalFree(self.0);
        }
    }
}

fn win32_success(success: i32) -> io::Result<()> {
    if success == 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

fn wide_path(path: &Path) -> io::Result<Vec<u16>> {
    let mut wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    if wide.contains(&0) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "path {} contains NUL; expected a Windows file path",
                path.display()
            ),
        ));
    }
    wide.push(0);
    Ok(wide)
}

fn owned_wide_string(allocation: &LocalAllocation) -> io::Result<String> {
    let pointer = allocation.0.cast::<u16>();
    let mut length = 0;
    // SAFETY: Conversion APIs return a non-null, NUL-terminated UTF-16 allocation
    // on success. The allocation remains alive throughout traversal and copying.
    unsafe {
        while *pointer.add(length) != 0 {
            length += 1;
        }
        String::from_utf16(std::slice::from_raw_parts(pointer, length))
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
    }
}

fn parse_descriptor(sddl: &str) -> io::Result<LocalAllocation> {
    let wide: Vec<u16> = sddl.encode_utf16().chain(Some(0)).collect();
    let mut pointer = null_mut();
    // SAFETY: The input is terminated and live, and the output pointer is writable.
    win32_success(unsafe {
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            wide.as_ptr(),
            1,
            &mut pointer,
            null_mut(),
        )
    })?;
    Ok(LocalAllocation(pointer))
}

fn descriptor_text(descriptor: &LocalAllocation, section: u32) -> io::Result<String> {
    let mut pointer = null_mut();
    let selection = if section <= DACL_SECURITY_INFORMATION {
        section
    } else {
        SACL_SECURITY_INFORMATION
    };
    // SAFETY: The live allocation is a Windows-returned valid descriptor; the
    // output is independently allocated by Windows and receives its own owner.
    win32_success(unsafe {
        ConvertSecurityDescriptorToStringSecurityDescriptorW(
            descriptor.0,
            1,
            selection,
            &mut pointer,
            null_mut(),
        )
    })?;
    owned_wide_string(&LocalAllocation(pointer.cast()))
}

fn read_section(path: &Path, section: u32) -> io::Result<LocalAllocation> {
    let wide = wide_path(path)?;
    let mut pointer = null_mut();
    // SAFETY: The terminated path and output slot remain live. Other outputs are
    // documented optional; the returned descriptor includes the requested section.
    let status = unsafe {
        GetNamedSecurityInfoW(
            wide.as_ptr(),
            SE_FILE_OBJECT,
            section,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            &mut pointer,
        )
    };
    if status != 0 {
        return Err(io::Error::from_raw_os_error(status as i32));
    }
    Ok(LocalAllocation(pointer))
}

fn normalized_section(text: String) -> String {
    let split = text.find('(').unwrap_or(text.len());
    let (flags, entries) = text.split_at(split);
    // Auto-inheritance bookkeeping can change in SetFileSecurity; protection,
    // every ACE (including inherited flags), owner and group must remain exact.
    format!("{}{}", flags.replace("AI", "").replace("AR", ""), entries)
}

fn section_text(path: &Path, section: u32) -> io::Result<String> {
    let descriptor = read_section(path, section)?;
    Ok(normalized_section(descriptor_text(&descriptor, section)?))
}

fn apply_section(path: &Path, section: u32, descriptor: &LocalAllocation) -> io::Result<()> {
    let wide = wide_path(path)?;
    // SAFETY: Path and descriptor are live and Windows-validated. This legacy
    // setter deliberately avoids the parent-inheritance recomputation performed
    // by SetNamedSecurityInfo; mandatory readback verifies the actual access ACL.
    win32_success(unsafe { SetFileSecurityW(wide.as_ptr(), section, descriptor.0) })
}

fn effective_token() -> io::Result<OwnedHandle> {
    let mut token = null_mut();
    // SAFETY: Pseudo-handle is valid for this thread and token is writable.
    let opened = unsafe { OpenThreadToken(GetCurrentThread(), TOKEN_QUERY, 1, &mut token) };
    if opened == 0 {
        let error = io::Error::last_os_error();
        if error.raw_os_error() != Some(ERROR_NO_TOKEN as i32) {
            return Err(error);
        }
        // SAFETY: Process pseudo-handle and output slot are valid.
        win32_success(unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) })?;
    }
    // SAFETY: A successful open transfers a unique, non-null owned token handle.
    Ok(unsafe { OwnedHandle::from_raw_handle(token) })
}

fn effective_user_sid() -> io::Result<String> {
    let token = effective_token()?;
    let mut needed = 0;
    // SAFETY: Null output with zero capacity queries the required buffer size.
    unsafe {
        GetTokenInformation(token.as_raw_handle(), TokenUser, null_mut(), 0, &mut needed);
    }
    if needed == 0 {
        return Err(io::Error::last_os_error());
    }
    let mut buffer = vec![0usize; (needed as usize).div_ceil(size_of::<usize>())];
    // SAFETY: usize storage is sufficiently aligned for TOKEN_USER and provides
    // at least needed writable bytes. The token remains open during the call.
    win32_success(unsafe {
        GetTokenInformation(
            token.as_raw_handle(),
            TokenUser,
            buffer.as_mut_ptr().cast(),
            needed,
            &mut needed,
        )
    })?;
    let mut pointer = null_mut();
    // SAFETY: Successful TokenUser query initializes TOKEN_USER and its SID points
    // into the still-live aligned buffer. Conversion returns a LocalFree allocation.
    win32_success(unsafe {
        ConvertSidToStringSidW(
            (*buffer.as_ptr().cast::<TOKEN_USER>()).User.Sid,
            &mut pointer,
        )
    })?;
    owned_wide_string(&LocalAllocation(pointer.cast()))
}

pub(super) fn create_private_directory(directory: &Path) -> io::Result<()> {
    let sid = effective_user_sid()?;
    let descriptor = parse_descriptor(&format!("O:{sid}D:P(A;OICI;FA;;;{sid})"))?;
    let attributes = SECURITY_ATTRIBUTES {
        nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
        lpSecurityDescriptor: descriptor.0,
        bInheritHandle: 0,
    };
    let wide = wide_path(directory)?;
    // SAFETY: The terminated path and initialized attributes/descriptor remain
    // alive. Protected explicit DACL prevents inheriting a broad parent's grants.
    win32_success(unsafe { CreateDirectoryW(wide.as_ptr(), &attributes) })?;
    for section in [OWNER_SECURITY_INFORMATION, DACL_SECURITY_INFORMATION] {
        let expected = normalized_section(descriptor_text(&descriptor, section)?);
        if section_text(directory, section)? != expected {
            return Err(policy_mismatch(directory));
        }
    }
    Ok(())
}

fn policy_mismatch(path: &Path) -> io::Error {
    io::Error::new(
        io::ErrorKind::PermissionDenied,
        format!(
            "access policy for {} differs from expected security descriptor; publication refused",
            path.display()
        ),
    )
}

struct EmptyProbe(PathBuf);

impl Drop for EmptyProbe {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

fn policy_probe(parent: &Path) -> io::Result<EmptyProbe> {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(io::Error::other)?
        .as_nanos();
    for attempt in 0..32 {
        let path = parent.join(format!(
            ".martensite-policy-{}-{nonce}-{attempt}",
            std::process::id()
        ));
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => {
                drop(file);
                return Ok(EmptyProbe(path));
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        }
    }
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "32 exclusive empty policy probe names already exist",
    ))
}

fn reject_encryption(path: &Path) -> io::Result<()> {
    if fs::metadata(path)?.file_attributes() & FILE_ATTRIBUTE_ENCRYPTED != 0 {
        return Err(io::Error::new(
            io::ErrorKind::Unsupported,
            format!(
                "encrypted file {} cannot preserve its encryption identity during replacement",
                path.display()
            ),
        ));
    }
    Ok(())
}

fn require_default_writer_policy(staged: &Path) -> io::Result<()> {
    let parent = staged.parent().ok_or_else(|| policy_mismatch(staged))?;
    let probe = policy_probe(parent)?;
    if fs::metadata(staged)?.permissions().readonly() {
        return Err(policy_mismatch(staged));
    }
    for section in ACCESS_SECTIONS {
        if section_text(staged, section)? != section_text(&probe.0, section)? {
            // A custom writer policy must never be widened implicitly. There is
            // no generally correct intersection for ordered conditional ACLs.
            return Err(policy_mismatch(staged));
        }
    }
    Ok(())
}

fn apply_verified_section(source: &LocalAllocation, section: u32, staged: &Path) -> io::Result<()> {
    let expected = normalized_section(descriptor_text(source, section)?);
    if section_text(staged, section)? != expected {
        apply_section(staged, section, source)?;
    }
    if section_text(staged, section)? != expected {
        return Err(policy_mismatch(staged));
    }
    Ok(())
}

fn access_snapshot(source: &Path) -> io::Result<Vec<(u32, LocalAllocation)>> {
    ACCESS_SECTIONS
        .iter()
        .map(|&section| read_section(source, section).map(|snapshot| (section, snapshot)))
        .collect()
}

fn verify_snapshots(
    source: &Path,
    staged: &Path,
    snapshots: &[(u32, LocalAllocation)],
) -> io::Result<()> {
    for (section, descriptor) in snapshots {
        let expected = normalized_section(descriptor_text(descriptor, *section)?);
        if section_text(source, *section)? != expected
            || section_text(staged, *section)? != expected
        {
            return Err(policy_mismatch(source));
        }
    }
    Ok(())
}

fn preserve_readonly(source: &Path, staged: &Path) -> io::Result<()> {
    let permissions = fs::metadata(source)?.permissions();
    let readonly = permissions.readonly();
    fs::set_permissions(staged, permissions)?;
    if fs::metadata(staged)?.permissions().readonly() != readonly {
        return Err(policy_mismatch(staged));
    }
    Ok(())
}

fn prepare_access(source: &Path, staged: &Path) -> io::Result<()> {
    reject_encryption(source)?;
    reject_encryption(staged)?;
    require_default_writer_policy(staged)?;
    let snapshots = access_snapshot(source)?;
    for (section, descriptor) in &snapshots {
        apply_verified_section(descriptor, *section, staged)?;
    }
    verify_snapshots(source, staged, &snapshots)?;
    preserve_readonly(source, staged)
}

pub(super) fn preserve_access(destination: &Path, staged: &Path) -> io::Result<()> {
    match fs::symlink_metadata(destination) {
        Ok(metadata) if metadata.file_type().is_file() => prepare_access(destination, staged),
        Ok(_) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "destination {} is not regular; expected a regular file",
                destination.display()
            ),
        )),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            let parent = destination
                .parent()
                .filter(|path| !path.as_os_str().is_empty())
                .unwrap_or(Path::new("."));
            let probe = policy_probe(parent)?;
            prepare_access(&probe.0, staged)
        }
        Err(error) => Err(error),
    }
}

#[cfg(test)]
#[path = "windows_tests.rs"]
mod tests;
