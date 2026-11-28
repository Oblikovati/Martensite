//! `Cocoa_LocalPool`: a local autorelease pool for the scope of the value.
use crate::platform::unsupported;
use martensite_tkernel::standard::Failure;
use std::marker::PhantomData;

/// A local `NSAutoreleasePool`, drained when dropped; `let _pool = CocoaLocalPool::new()?;`.
///
/// Autorelease pools belong to the thread that pushed them, so the pool is neither `Send`
/// nor `Sync`.
#[derive(Debug)]
pub struct CocoaLocalPool {
    /// `myPoolObj`: the pool token.
    pool_obj: usize,
    thread_bound: PhantomData<*const ()>,
}

impl CocoaLocalPool {
    /// `Cocoa_LocalPool()`: pushes a pool on the calling thread; raises
    /// `Standard_NotImplemented` without the Objective-C runtime. Example: `CocoaLocalPool::new()`.
    pub fn new() -> Result<Self, Failure> {
        let pool_obj =
            native::push().ok_or_else(|| unsupported("Cocoa_LocalPool", "AppKit (macOS)"))?;
        Ok(Self {
            pool_obj,
            thread_bound: PhantomData,
        })
    }
    /// The pool token (`myPoolObj`); `pool.pool_object()`.
    pub fn pool_object(&self) -> usize {
        self.pool_obj
    }
}

impl Drop for CocoaLocalPool {
    /// `~Cocoa_LocalPool`: drains the pool and every pool pushed after it.
    fn drop(&mut self) {
        native::pop(self.pool_obj);
    }
}

#[cfg(target_os = "macos")]
#[allow(unsafe_code)]
mod native {
    use std::cell::RefCell;

    thread_local! {
        /// The tokens this thread pushed and has not drained, oldest first.
        static PUSHED: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
    }

    /// Pushes a pool and records its token.
    pub(super) fn push() -> Option<usize> {
        // SAFETY: objc_autoreleasePoolPush has no preconditions; the token stays on this thread.
        let token = unsafe { objc2::ffi::objc_autoreleasePoolPush() } as usize;
        PUSHED.with_borrow_mut(|pushed| pushed.push(token));
        Some(token)
    }

    /// Drains `token` and the pools above it, once. Popping a token a newer pool's drain
    /// already removed would be undefined behaviour, so an unknown token is ignored.
    pub(super) fn pop(token: usize) {
        let Some(index) = PUSHED.with_borrow(|pushed| pushed.iter().rposition(|&t| t == token))
        else {
            return;
        };
        PUSHED.with_borrow_mut(|pushed| pushed.truncate(index));
        // SAFETY: `token` was pushed on this thread (the pool is !Send) and is still live.
        unsafe { objc2::ffi::objc_autoreleasePoolPop(token as *mut std::ffi::c_void) };
    }
}

#[cfg(not(target_os = "macos"))]
mod native {
    /// No Objective-C runtime: no pool.
    pub(super) fn push() -> Option<usize> {
        None
    }
    pub(super) fn pop(_token: usize) {}
}
