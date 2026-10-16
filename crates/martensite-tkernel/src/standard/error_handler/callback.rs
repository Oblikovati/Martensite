//! Error handler callbacks, with signal conversion enabled (as on Linux).
use super::handler_stack::{self, HandlerId};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

/// The shared state of one callback: its handler link and its destroy action.
pub(super) struct CallbackSlot {
    handler: Cell<Option<HandlerId>>,
    action: RefCell<Box<dyn FnMut()>>,
}
impl CallbackSlot {
    /// Runs the action when the handler unlinks the callback.
    pub(super) fn destroy_callback(&self) {
        (self.action.borrow_mut())();
    }
}
/// An object that the innermost error handler notifies when it is destroyed while the
/// object is still registered; the destroy notification is the
/// closure given to [`ErrorHandlerCallback::new`]. Callbacks belong to one thread.
/// `let callback = ErrorHandlerCallback::new(|| release()); callback.register_callback();`.
pub struct ErrorHandlerCallback {
    slot: Rc<CallbackSlot>,
}
impl ErrorHandlerCallback {
    /// An unregistered callback that runs `destroy` when notified;
    /// `ErrorHandlerCallback::new(|| println!("released"))`.
    pub fn new(destroy: impl FnMut() + 'static) -> Self {
        Self {
            slot: Rc::new(CallbackSlot {
                handler: Cell::new(None),
                action: RefCell::new(Box::new(destroy)),
            }),
        }
    }
    /// Links this callback first in the innermost handler's list.
    /// It does nothing when already linked, including when the handler it
    /// was linked to has since been destroyed without unregistering it, and nothing when
    /// no handler is active; `callback.register_callback()`.
    pub fn register_callback(&self) {
        if self.slot.handler.get().is_some() {
            return;
        }
        let Some(top) = handler_stack::top_handler() else {
            return;
        };
        self.slot.handler.set(Some(top));
        let slot: Rc<CallbackSlot> = self.slot.clone();
        handler_stack::with_frame(top, |frame| frame.callbacks.insert(0, slot));
    }
    /// Removes this callback from its handler's list, if it is
    /// still there, and forgets the handler; `callback.unregister_callback()`.
    pub fn unregister_callback(&self) {
        let Some(handler) = self.slot.handler.take() else {
            return;
        };
        handler_stack::with_frame(handler, |frame| {
            frame
                .callbacks
                .retain(|linked| !Rc::ptr_eq(linked, &self.slot))
        });
    }
    /// True while linked to a handler; `callback.is_registered()`.
    pub fn is_registered(&self) -> bool {
        self.slot.handler.get().is_some()
    }
    /// Runs the destroy action directly; `callback.destroy_callback()`.
    pub fn destroy_callback(&self) {
        self.slot.destroy_callback();
    }
}
/// Dropping the callback unregisters it.
impl Drop for ErrorHandlerCallback {
    fn drop(&mut self) {
        self.unregister_callback();
    }
}
