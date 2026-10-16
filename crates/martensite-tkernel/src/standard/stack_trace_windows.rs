//! The Win64 stack capture: frames from RtlCaptureStackBackTrace, names from DbgHelp.
//! DbgHelp's process-wide symbol state
//! requires serialization even when callers capture different threads.
#![allow(unsafe_code)]

use std::{
    ffi::{CStr, c_void},
    sync::{Mutex, OnceLock},
};
use windows_sys::Win32::{
    Foundation::HANDLE,
    System::{
        Diagnostics::Debug::*, LibraryLoader::GetModuleFileNameA, Threading::GetCurrentProcess,
    },
};

// DbgHelp keeps one symbol handler per process. It is started once and never
// cleaned up, as Rust's own backtrace support does. The earlier walker
// (RtlCaptureContext + StackWalk64, with SymInitialize/SymCleanup per capture)
// crashed with an access violation on Windows x64 in the 0.3.0 release
// rehearsal; this version uses neither.
static SYMBOL_SESSION: Mutex<()> = Mutex::new(());
static SYMBOLS_READY: OnceLock<bool> = OnceLock::new();

/// Starts the process symbol handler on first use; e.g. `symbols_ready(process)`.
fn symbols_ready(process: HANDLE) -> bool {
    *SYMBOLS_READY.get_or_init(|| {
        // SAFETY: `process` is the current-process pseudo-handle, the caller holds
        // SYMBOL_SESSION, and a null search path selects the default.
        unsafe { SymInitialize(process, std::ptr::null(), 1) != 0 }
    })
}

/// Captures and symbolizes up to `nb_traces` frames after skipping `top_skip`
/// frames, the first being this function; e.g. `native_frames(10, 1)`.
#[inline(never)]
pub(super) fn native_frames(nb_traces: i32, top_skip: i32) -> Option<Vec<String>> {
    let count = u32::try_from(nb_traces).ok()?;
    let skip = u32::try_from(top_skip).ok()?;
    let mut addresses: Vec<*mut c_void> = vec![std::ptr::null_mut(); count as usize];
    // SAFETY: `addresses` is writable for `count` pointers and the hash output is
    // optional. RtlCaptureStackBackTrace reads only the calling thread's stack.
    let captured = unsafe {
        RtlCaptureStackBackTrace(skip, count, addresses.as_mut_ptr(), std::ptr::null_mut())
    };
    addresses.truncate(usize::from(captured));
    let _lock = SYMBOL_SESSION
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    // SAFETY: GetCurrentProcess returns a pseudo-handle that is always valid.
    let process: HANDLE = unsafe { GetCurrentProcess() };
    if !symbols_ready(process) {
        return None;
    }
    let lines: Vec<String> = addresses
        .iter()
        .map(|address| symbolized_frame(process, *address as u64))
        .collect();
    (!lines.is_empty()).then_some(lines)
}

fn module_filename(process: HANDLE, address: u64) -> String {
    let mut buffer: [u8; 260] = [0; 260];
    // SAFETY: DbgHelp resolves an address from the captured stack. A zero module
    // must not reach GetModuleFileNameA, where it would mean the main executable.
    let module: u64 = unsafe { SymGetModuleBase64(process, address) };
    if module == 0 {
        return String::new();
    }
    // SAFETY: buffer is writable for its declared capacity.
    let count =
        unsafe { GetModuleFileNameA(module as _, buffer.as_mut_ptr(), buffer.len() as u32) };
    String::from_utf8_lossy(&buffer[..count as usize]).into_owned()
}

#[repr(C)]
struct SymbolBuffer {
    info: SYMBOL_INFO,
    // SYMBOL_INFO ends in a variable-length Name. This extends its aligned storage.
    name: [u8; MAX_SYM_NAME as usize],
}

fn symbolized_frame(process: HANDLE, address: u64) -> String {
    let module: String = module_filename(process, address);
    let mut symbol = SymbolBuffer {
        info: SYMBOL_INFO::default(),
        name: [0; MAX_SYM_NAME as usize],
    };
    symbol.info.SizeOfStruct = std::mem::size_of::<SYMBOL_INFO>() as u32;
    symbol.info.MaxNameLen = MAX_SYM_NAME;
    let mut displacement: u64 = 0;
    // SAFETY: the aligned symbol buffer has MaxNameLen bytes beyond SYMBOL_INFO.
    if unsafe { SymFromAddr(process, address, &mut displacement, &mut symbol.info) } == 0 {
        return format!("{module}(???)");
    }
    // SAFETY: successful SymFromAddr writes a terminated name within the buffer.
    let name = unsafe { CStr::from_ptr(symbol.info.Name.as_ptr().cast()) };
    format!("{module}({})", name.to_string_lossy())
}
