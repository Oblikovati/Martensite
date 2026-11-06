//! Source mutations from a pixmap destructor route through the virtual setters.
//!
//! Native `~ImagePixMap` of the retiring `myTexturePixMap` target may call the virtual
//! `SetTextureFileName`/`SetTexturePixMap` of its owner: the call reaches the most derived
//! override, whose base call nullifies or assigns the field still pointing at the retiring
//! object. Here the destructor opens a retirement scope; while it runs, the base setter bodies
//! of that owner perform the slot operation through the destructor borrow, so an application
//! override sees the same order as natively. Unscoped access to a pixmap whose drop is in
//! progress is refused with a typed failure (accepted safe-Rust deviation, see
//! the design notes).
#![allow(unsafe_code)] // One lifetime-erased destructor borrow, used only inside its own scope.
use super::textured_shape_source::AisTexturedShapeSource;
use martensite_tkernel::{
    standard::{
        Failure, HandleReference, IntrusiveRetirementBorrow, SharedState, Transient,
        catch_native_failure,
    },
    tcollection::AsciiString,
};
use martensite_tkservice::image::ImagePixMap;
use std::marker::PhantomData;
use std::sync::Arc;
use std::thread::{self, ThreadId};

/// Destructor borrows lent to the base setter bodies while retirement scopes run.
/// Example: `base.retirement.is_active()` is true only inside `ais_textured_shape_with_retirement`.
#[derive(Default)]
pub struct AisTexturedRetirementStash {
    // Innermost last; nested pixmap destructors open nested scopes on the same thread.
    active: SharedState<Vec<ActiveTexturedRetirement>>,
}
type PixMapRetirement<'a> = IntrusiveRetirementBorrow<'a, ImagePixMap>;
struct ActiveTexturedRetirement {
    thread: ThreadId,
    // Lifetime-erased: valid only while the pushing scope frame runs (see `lend`).
    borrow: Arc<PixMapRetirement<'static>>,
}
impl AisTexturedRetirementStash {
    /// Whether this thread runs a retirement scope of the owner; `stash.is_active()`.
    pub fn is_active(&self) -> bool {
        self.innermost().is_some()
    }
    fn innermost(&self) -> Option<Arc<PixMapRetirement<'static>>> {
        let current = thread::current().id();
        let active = self.active.read();
        let found = active.iter().rev().find(|entry| entry.thread == current);
        found.map(|entry| entry.borrow.clone())
    }
    /// Run `body` with the destructor `borrow` lent to this owner's base setters on this thread.
    fn lend<R>(&self, borrow: PixMapRetirement<'_>, body: impl FnOnce() -> R) -> R {
        // SAFETY: only the lifetime is erased (same type and layout). The erased borrow never
        // leaves this module: it is stored until the guard below drops, which happens before
        // the original borrow's lifetime ends (also on unwind), and `with_innermost` lends it
        // only to the thread that pushed it, whose call stack is nested inside `body` while
        // the entry exists.
        let erased: PixMapRetirement<'static> = unsafe { std::mem::transmute(borrow) };
        let entry = ActiveTexturedRetirement {
            thread: thread::current().id(),
            borrow: Arc::new(erased),
        };
        self.active.write().push(entry);
        let _guard = PopRetirement(self);
        body()
    }
    /// Lend the innermost scope borrow of this thread to `inspect`, which cannot keep it.
    fn with_innermost<R>(&self, inspect: impl FnOnce(Option<&PixMapRetirement<'_>>) -> R) -> R {
        // The field guard ends here: a nested pixmap destructor may open its own scope.
        let innermost = self.innermost();
        inspect(innermost.as_deref())
    }
}
struct PopRetirement<'a>(&'a AisTexturedRetirementStash);
impl Drop for PopRetirement<'_> {
    fn drop(&mut self) {
        let current = thread::current().id();
        let mut active = self.0.active.write();
        if let Some(index) = active.iter().rposition(|entry| entry.thread == current) {
            active.remove(index);
        }
    }
}

/// Assign the `myTexturePixMap` field, through the destructor borrow inside a retirement
/// scope of this thread; `assign_textured_pix_map_slot(&object, &next)?`.
pub(super) fn assign_textured_pix_map_slot<S: AisTexturedShapeSource + ?Sized>(
    object: &S,
    source: &dyn HandleReference<ImagePixMap>,
) -> Result<(), Failure> {
    let base = object.textured_shape_base();
    let slot = &base.texture_pix_map;
    base.retirement.with_innermost(|borrow| match borrow {
        Some(borrow) => borrow.assign(source),
        None => slot.assign(source),
    })
}

/// Virtual source operations lent to a pixmap destructor; `scope.set_texture_pix_map(&next)?`.
/// The scope has no object identity of its own and cannot escape its closure.
pub struct AisTexturedShapeRetirement<'a, S: AisTexturedShapeSource + ?Sized> {
    source: &'a S,
    lifetime: PhantomData<&'a dyn Transient>,
}
/// Open a retirement scope for `retiring`, the current `myTexturePixMap` target whose drop is
/// running; `ais_textured_shape_with_retirement(&*owner, self, |scope| scope.set_texture_file_name(&file))?`.
/// Inside it the virtual setters (application overrides included) reach the field as natively.
pub fn ais_textured_shape_with_retirement<S: AisTexturedShapeSource + ?Sized, R>(
    source: &S,
    retiring: &dyn Transient,
    inspect: impl FnOnce(AisTexturedShapeRetirement<'_, S>) -> R,
) -> Result<R, Failure> {
    let base = source.textured_shape_base();
    let scope = AisTexturedShapeRetirement {
        source,
        lifetime: PhantomData,
    };
    // Only the current retiring target may open a scope; anything else fails typed.
    base.texture_pix_map
        .with_retiring_reference(retiring, |borrow| {
            base.retirement.lend(borrow, || inspect(scope))
        })
}
impl<S: AisTexturedShapeSource + ?Sized> AisTexturedShapeRetirement<'_, S> {
    /// The virtual `SetTextureFileName` of the owner; `scope.set_texture_file_name(&file)?`.
    pub fn set_texture_file_name(&self, file: &AsciiString) -> Result<(), Failure> {
        self.source.set_texture_file_name(file)
    }
    /// The virtual `SetTexturePixMap` of the owner, its failure returned; `scope.set_texture_pix_map(&next)?`.
    pub fn set_texture_pix_map(
        &self,
        pixels: &dyn HandleReference<ImagePixMap>,
    ) -> Result<(), Failure> {
        catch_native_failure(|| {
            self.source.set_texture_pix_map(pixels);
            Ok(())
        })
    }
}

#[cfg(test)]
#[path = "textured_shape_retirement_tests.rs"]
mod tests;
