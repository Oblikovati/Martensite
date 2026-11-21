//! `V3dView` subviews: the views drawn in a corner of a parent view's window.
use super::V3dView;
use crate::v3d::V3dViewSource;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::aspect::AspectWindowMembers as _;
use martensite_tkservice::graphic3d::{Graphic3dCViewMembers as _, Graphic3dCViewSubviews as _};

use std::sync::Arc;

/// Subviews.
impl V3dView {
    /// `IsSubview`; `view.is_subview()`.
    pub fn is_subview(&self) -> bool {
        !self.state.read().parent_view.upgrade().is_null()
    }
    /// `ParentView`: null for a top view; `view.parent_view()`.
    pub fn parent_view(&self) -> Handle<dyn V3dViewSource> {
        self.state.read().parent_view.upgrade()
    }
    /// Copy subview owners explicitly; native `Subviews` uses `view.subviews_ref()`.
    /// Example: `let copied = view.subviews()`.
    pub fn subviews(&self) -> Vec<Handle<dyn V3dViewSource>> {
        self.state
            .read()
            .subviews
            .iter()
            .map(|slot| Handle::clone(slot))
            .collect()
    }
    /// `AddSubview`; `view.add_subview(&subview)`.
    pub fn add_subview(&self, view: &Handle<dyn V3dViewSource>) {
        self.state.write().subviews.append(Arc::new(view.clone()));
    }
    /// `RemoveSubview`: true when the view was a subview; `view.remove_subview(&subview)`.
    pub fn remove_subview(&self, view: &V3dView) -> bool {
        let mut state = self.state.write();
        let address = view.this.read().object_address();
        let Some((index, removed)) = state
            .subviews
            .iter()
            .enumerate()
            .find(|(_, slot)| std::ptr::eq(slot.object_address(), address))
            .map(|(index, slot)| (index, Arc::clone(slot)))
        else {
            return false;
        };
        martensite_tkernel::standard::raise_native_failure(state.subviews.remove(index + 1));
        // The last view owner can reenter its parent from a destructor.
        drop(state);
        drop(removed);
        true
    }
    /// `PickSubview`: the topmost active subview under a pixel, this view inside the window
    /// without subview, null outside the window. Example: `view.pick_subview(Vec2::new(10, 10))`.
    pub fn pick_subview(&self, point: Vec2<i32>) -> Handle<dyn V3dViewSource> {
        let (window, subviews) = {
            let state = self.state.read();
            (
                state.window.clone(),
                state
                    .subviews
                    .iter()
                    .map(|slot| Handle::clone(slot))
                    .collect::<Vec<_>>(),
            )
        };
        let Some(dims) = window.get().map(|w| w.dimensions()) else {
            return Handle::default();
        };
        if point.x < 0 || point.x >= dims.x || point.y < 0 || point.y >= dims.y {
            return Handle::default();
        }
        for subview in subviews.iter().rev() {
            let Some(value) = subview.get() else {
                continue;
            };
            let (top_left, size) = (
                value.driver_view_observer().subview_top_left(),
                value
                    .window()
                    .get()
                    .map(|w| w.dimensions())
                    .unwrap_or(Vec2::new(0, 0)),
            );
            if value.driver_view_observer().is_active()
                && point.x >= top_left.x
                && point.x < top_left.x + size.x
                && point.y >= top_left.y
                && point.y < top_left.y + size.y
            {
                return subview.clone();
            }
        }
        self.this()
    }
}
