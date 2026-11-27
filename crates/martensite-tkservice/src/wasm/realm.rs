//! Checked thread-affinity realm confinement for `!Send + !Sync` browser handles (#632).
//!
//! `Transient` requires `Send + Sync`, while `web_sys` canvas handles and WebGPU
//! objects are confined to the browser realm (window or dedicated worker) that
//! created them. Wrapping handles in `send_wrapper::SendWrapper` preserves
//! `Send + Sync` with zero custom `unsafe` and zero `thread_local!`.

use martensite_tkernel::standard::Failure;
use send_wrapper::SendWrapper;

/// Wraps a thread-confined value so it satisfies `Send + Sync` with runtime
/// owning-thread checks.
#[derive(Debug)]
pub struct RealmHandle<T> {
    inner: SendWrapper<T>,
}

impl<T> RealmHandle<T> {
    /// Binds `value` to the calling browser realm thread.
    ///
    /// ```
    /// use martensite_tkservice::wasm::RealmHandle;
    /// let handle = RealmHandle::new(42u32);
    /// assert!(handle.is_on_owning_thread());
    /// assert_eq!(handle.try_get(), Some(&42u32));
    /// ```
    pub fn new(value: T) -> Self {
        Self {
            inner: SendWrapper::new(value),
        }
    }

    /// Returns `true` when called from the thread that created this handle.
    pub fn is_on_owning_thread(&self) -> bool {
        self.inner.valid()
    }

    /// Borrows the inner value when on the owning thread, or `None` otherwise.
    pub fn try_get(&self) -> Option<&T> {
        self.inner.valid().then(|| &*self.inner)
    }

    /// Borrows the inner value on the owning thread, or returns a typed
    /// `Standard_ProgramError` naming the thread-confinement violation.
    pub fn get_checked(&self, owner_name: &str) -> Result<&T, Failure> {
        self.try_get().ok_or_else(|| {
            Failure::with_type(
                "Standard_ProgramError",
                &format!(
                    "{owner_name}: accessed realm-confined browser handle from a non-owning thread; expected the creating realm thread"
                ),
                "",
            )
        })
    }

    /// Unwraps the inner value when on the owning thread, or returns `Err(self)`
    /// without panicking.
    pub fn try_take(self) -> Result<T, Self> {
        if self.inner.valid() {
            Ok(self.inner.take())
        } else {
            Err(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RealmHandle;

    #[test]
    fn realm_handle_allows_owning_thread_and_rejects_foreign_thread() {
        let handle = RealmHandle::new(std::rc::Rc::new(7u32));
        assert!(handle.is_on_owning_thread());
        assert_eq!(**handle.get_checked("WasmWindow").unwrap(), 7);
        let foreign_ok = std::thread::scope(|scope| {
            scope
                .spawn(|| handle.get_checked("WasmWindow").is_err())
                .join()
                .unwrap()
        });
        assert!(foreign_ok);
        let recovered = handle.try_take().expect("owning thread must unwrap");
        assert_eq!(*recovered, 7);
    }
}
