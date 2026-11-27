//! Checked thread-affinity realm confinement (`SendWrapper`) and zero-unsafe native `block_on` (#634).
//!
//! On `wasm32-unknown-unknown` with `+atomics`, `wgpu` handles wrap JS WebGPU objects which are
//! `!Send + !Sync`. Wrapping them in `WebGpuRealm<T>` (backed by `send_wrapper::SendWrapper`)
//! satisfies `RenderDeviceBackend: Send + Sync` with zero `unsafe`, zero `thread_local!`, and
//! deterministic typed `RenderError::thread_confined` on foreign-thread access.
use martensite_render_core::RenderError;
use send_wrapper::SendWrapper;

/// Thread-confined container safe to store in `Send + Sync` driver/view handles (#634).
pub struct WebGpuRealm<T>(SendWrapper<T>);

impl<T> WebGpuRealm<T> {
    /// Pin `value` to the calling thread / Web Worker realm.
    pub fn new(value: T) -> Self {
        Self(SendWrapper::new(value))
    }

    /// Return whether the caller is currently on the owning realm thread.
    pub fn is_valid(&self) -> bool {
        self.0.valid()
    }

    /// Borrow the inner value on its owning realm thread, or return `RenderError::thread_confined`.
    pub fn get(&self, resource: &str) -> Result<&T, RenderError> {
        if !self.0.valid() {
            return Err(RenderError::thread_confined(resource));
        }
        Ok(&*self.0)
    }

    /// Mutably borrow the inner value on its owning realm thread, or return `RenderError::thread_confined`.
    pub fn get_mut(&mut self, resource: &str) -> Result<&mut T, RenderError> {
        if !self.0.valid() {
            return Err(RenderError::thread_confined(resource));
        }
        Ok(&mut *self.0)
    }
}

/// Minimal zero-`unsafe` blocking future executor for native desktop `wgpu` helpers (#634).
#[cfg(not(target_arch = "wasm32"))]
pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
    use std::pin::pin;
    use std::sync::Arc;
    use std::task::{Context, Poll, Wake, Waker};
    use std::thread::{self, Thread};

    struct ThreadWaker(Thread);
    impl Wake for ThreadWaker {
        fn wake(self: Arc<Self>) {
            self.0.unpark();
        }
        fn wake_by_ref(self: &Arc<Self>) {
            self.0.unpark();
        }
    }

    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut cx = Context::from_waker(&waker);
    let mut pinned = pin!(future);
    loop {
        match pinned.as_mut().poll(&mut cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => thread::park(),
        }
    }
}
