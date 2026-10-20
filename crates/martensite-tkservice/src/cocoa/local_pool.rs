//! `Cocoa_LocalPool`: a local autorelease pool for the scope of the value.
use crate::platform::unsupported;
use martensite_tkernel::standard::Failure;

/// A local `NSAutoreleasePool`, drained when dropped; `CocoaLocalPool::new()?`.
#[derive(Debug)]
pub struct CocoaLocalPool {
    /// `myPoolObj`: the pool address.
    pool_obj: usize,
}

impl CocoaLocalPool {
    /// `Cocoa_LocalPool()`: allocates a pool. The AppKit backend is not implemented, so this raises
    /// `Standard_NotImplemented`; `CocoaLocalPool::new()`.
    pub fn new() -> Result<Self, Failure> {
        // TODO(platform): macOS — `[[NSAutoreleasePool alloc] init]`.
        Err(unsupported("Cocoa_LocalPool", "AppKit (macOS)"))
    }
    /// The pool address (`myPoolObj`); `pool.pool_object()`.
    pub fn pool_object(&self) -> usize {
        self.pool_obj
    }
}

impl Drop for CocoaLocalPool {
    /// `~Cocoa_LocalPool`: drains the pool.
    fn drop(&mut self) {
        // TODO(platform): macOS — `[myPoolObj release]`; no pool exists before the macOS backend.
        let _ = self.pool_obj;
    }
}
