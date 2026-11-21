//! Native subview traversal retains actual Sequence nodes through valid callback edits.
use super::{V3dView, V3dViewSource};
use martensite_tkernel::{
    ncollection::SequenceCursor,
    standard::{Handle, HandleObserver, HandleReference, raise_native_failure},
};
/// Live native subview iterator; `let mut cursor=parent.subview_iterator(); cursor.next()`.
pub struct V3dSubviewCursor<'a> {
    parent: &'a V3dView,
    position: SequenceCursor,
}
/// Actual current subview handle slot; `cursor.value_ref().observe()`.
pub struct V3dSubviewNodeRef<'a> {
    parent: &'a V3dView,
    position: SequenceCursor,
}
impl<'a> V3dSubviewCursor<'a> {
    pub(super) fn new(parent: &'a V3dView) -> Self {
        Self {
            parent,
            position: parent.state.read().subviews.cursor(),
        }
    }
    /// Inspect the current native node; `while cursor.more(){cursor.next();}`.
    pub fn more(&self) -> bool {
        self.position.more()
    }
    /// Borrow the actual current node slot; `let child=cursor.value_ref()`.
    pub fn value_ref(&self) -> V3dSubviewNodeRef<'a> {
        V3dSubviewNodeRef {
            parent: self.parent,
            position: self.position,
        }
    }
    /// Follow live links after callbacks; `cursor.next()`.
    pub fn next(&mut self) {
        let state = self.parent.state.read();
        self.position.next(&state.subviews);
    }
}
impl<'a> V3dSubviewNodeRef<'a> {
    pub(super) fn new(parent: &'a V3dView, position: SequenceCursor) -> Self {
        Self { parent, position }
    }
    fn with_handle<R>(&self, inspect: impl FnOnce(&Handle<dyn V3dViewSource>) -> R) -> R {
        let state = self.parent.state.read();
        let value = self
            .position
            .value(&state.subviews)
            .unwrap_or_else(|| invalid_subview_node(self.position));
        inspect(value)
    }
}
impl HandleReference<dyn V3dViewSource> for V3dSubviewNodeRef<'_> {
    fn observe(&self) -> HandleObserver<'_, dyn V3dViewSource> {
        HandleObserver::Snapshot(self.with_handle(Handle::clone))
    }
    fn reference_address(&self) -> *const () {
        self.with_handle(|handle| std::ptr::from_ref(handle).cast())
    }
    fn object_address(&self) -> *const () {
        self.with_handle(Handle::object_address)
    }
    fn is_null(&self) -> bool {
        self.with_handle(Handle::is_null)
    }
}

fn invalid_subview_node(position: SequenceCursor) -> ! {
    match raise_native_failure::<std::convert::Infallible>(Err(
        martensite_tkernel::standard::exceptions::program_error(&format!(
            "V3d subview node={:?}; expected original live sequence node",
            position.current_node()
        )),
    )) {}
}
