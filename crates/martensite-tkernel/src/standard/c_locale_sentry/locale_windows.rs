//! CRT locale activation is confined to the caller's thread (#334).
#![allow(unsafe_code)]
use std::ffi::{CStr, CString};
use std::io;

unsafe extern "C" {
    fn _configthreadlocale(mode: i32) -> i32;
}

#[derive(Debug)]
pub(super) struct ThreadLocaleScope {
    previous_mode: i32,
    previous_name: CString,
}

impl ThreadLocaleScope {
    pub(super) fn enter() -> Self {
        Self::try_enter().unwrap_or_else(|error| {
            panic!("C locale activation failed: {error}; expected a per-thread C locale")
        })
    }

    fn try_enter() -> io::Result<Self> {
        // SAFETY: 1 is _ENABLE_PER_THREAD_LOCALE; only this thread's mode changes.
        let previous_mode = unsafe { _configthreadlocale(1) };
        if previous_mode == -1 {
            return Err(io::Error::other("_configthreadlocale(1) returned -1"));
        }
        let previous_name = match current_name() {
            Ok(name) => name,
            Err(error) => {
                restore_mode(previous_mode);
                return Err(error);
            }
        };
        let scope = Self {
            previous_mode,
            previous_name,
        };
        set_name(c"C")?;
        Ok(scope)
    }
}

impl Drop for ThreadLocaleScope {
    fn drop(&mut self) {
        // Restore the name while per-thread mode is still enabled; restoring the
        // old mode first could change another thread's process-global locale.
        let _ = set_name(&self.previous_name);
        restore_mode(self.previous_mode);
    }
}

fn restore_mode(mode: i32) {
    // SAFETY: mode was returned by a successful _configthreadlocale call.
    unsafe { _configthreadlocale(mode) };
}

fn current_name() -> io::Result<CString> {
    // SAFETY: a null locale queries the enabled thread-local locale; its string
    // remains live until this thread changes it and is copied immediately.
    let name = unsafe { libc::setlocale(libc::LC_ALL, std::ptr::null()) };
    if name.is_null() {
        return Err(io::Error::other("setlocale(LC_ALL, NULL) returned null"));
    }
    // SAFETY: successful setlocale returns a NUL-terminated CRT-owned string.
    Ok(unsafe { CStr::from_ptr(name) }.to_owned())
}

fn set_name(name: &CStr) -> io::Result<()> {
    // SAFETY: name is terminated and live, and per-thread mode was enabled first.
    let active = unsafe { libc::setlocale(libc::LC_ALL, name.as_ptr()) };
    if active.is_null() {
        return Err(io::Error::other(format!("setlocale rejected {name:?}")));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_scope_restores_the_previous_per_thread_mode() {
        let scope = ThreadLocaleScope::enter();
        let original_mode = scope.previous_mode;
        drop(scope);
        let next = ThreadLocaleScope::enter();
        assert_eq!(next.previous_mode, original_mode);
    }

    fn concurrent_locale(name: &CStr, barrier: &std::sync::Barrier) -> CString {
        let _scope = ThreadLocaleScope::enter();
        set_name(name).unwrap();
        barrier.wait();
        current_name().unwrap()
    }

    #[test]
    fn simultaneous_threads_keep_their_distinct_crt_locales() {
        let _scope = ThreadLocaleScope::enter();
        set_name(c".UTF8").unwrap();
        let utf8_name = current_name().unwrap();
        let barrier = std::sync::Barrier::new(2);
        std::thread::scope(|threads| {
            let first = threads.spawn(|| concurrent_locale(c"C", &barrier));
            let second = threads.spawn(|| concurrent_locale(c".UTF8", &barrier));
            assert_eq!(first.join().unwrap().as_c_str(), c"C");
            assert_eq!(second.join().unwrap(), utf8_name);
        });
        assert_eq!(current_name().unwrap(), utf8_name);
    }

    #[test]
    fn nested_windows_scopes_restore_the_previous_thread_locale() {
        // Keep all fixture mutations per-thread, including test failure cleanup.
        let guard = ThreadLocaleScope::enter();
        set_name(c".UTF8").unwrap();
        let before = current_name().unwrap();
        {
            let _outer = ThreadLocaleScope::enter();
            assert_eq!(current_name().unwrap().as_c_str(), c"C");
            {
                let _inner = ThreadLocaleScope::enter();
            }
            assert_eq!(current_name().unwrap().as_c_str(), c"C");
        }
        assert_eq!(current_name().unwrap(), before);
        drop(guard);
    }

    #[test]
    fn unwinding_restores_the_thread_locale_and_rejects_invalid_names() {
        let _guard = ThreadLocaleScope::enter();
        set_name(c".UTF8").unwrap();
        let before = current_name().unwrap();
        let _ = std::panic::catch_unwind(|| {
            let _scope = ThreadLocaleScope::enter();
            panic!("fixture unwind");
        });
        assert_eq!(current_name().unwrap(), before);
        assert!(set_name(c"MartensiteInvalidLocale334").is_err());
    }
}
