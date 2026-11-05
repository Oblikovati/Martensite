//! Borrowed extension capabilities expose actual operation-local storage and node cursors.
use super::*;
/// Shared area-building state; `builder.area_extension_state().areas().extent()`.
pub struct AreaExtensionState<'a> {
    builder: &'a TopOpeBRepBuildAreaBuilder,
}
/// Exclusive state before publication; `builder.area_extension_state_mut().areas_mut().clear()`.
pub struct AreaExtensionStateMut<'a> {
    builder: &'a mut TopOpeBRepBuildAreaBuilder,
}
impl TopOpeBRepBuildAreaBuilder {
    /// Borrows live retained fields; `let state = builder.area_extension_state()`.
    pub fn area_extension_state(&self) -> AreaExtensionState<'_> {
        AreaExtensionState { builder: self }
    }
    /// Borrows mutation capabilities exclusively; `let state = builder.area_extension_state_mut()`.
    pub fn area_extension_state_mut(&mut self) -> AreaExtensionStateMut<'_> {
        AreaExtensionStateMut { builder: self }
    }
}
impl<'a> AreaExtensionState<'a> {
    /// Reads actual area nodes; `state.areas().extent()`.
    pub fn areas(&self) -> &'a AreaList {
        &self.builder.my_area
    }
    /// Reads the retained area cursor; `state.area_cursor().more()`.
    pub fn area_cursor(&self) -> &'a ListCursor {
        &self.builder.my_area_iterator
    }
    /// Reads the independent loop cursor; `state.loop_cursor().more()`.
    pub fn loop_cursor(&self) -> &'a ListCursor {
        &self.builder.my_loop_iterator
    }
    /// Reads the comparison policy; `*state.unknown_raises()`.
    pub fn unknown_raises(&self) -> &'a bool {
        &self.builder.my_unknown_raise
    }
    /// Reads the current area cursor's owner, including a rebound list; `state.area_cursor_source()`.
    pub fn area_cursor_source(&self) -> Option<&'a AreaList> {
        self.builder.area_cursor_owner()
    }
    /// Reads the current loop cursor's owner independently of area position; `state.loop_cursor_source()`.
    pub fn loop_cursor_source(&self) -> Option<&'a AreaLoopList> {
        self.builder.loop_owner()
    }
}
impl AreaExtensionStateMut<'_> {
    /// Mutates actual area nodes without repositioning cursors; `state.areas_mut().prepend(area)`.
    pub fn areas_mut(&mut self) -> &mut AreaList {
        &mut self.builder.my_area
    }
    /// Mutates the retained area cursor; `*state.area_cursor_mut() = position`.
    pub fn area_cursor_mut(&mut self) -> &mut ListCursor {
        &mut self.builder.my_area_iterator
    }
    /// Mutates the independent loop cursor; `*state.loop_cursor_mut() = position`.
    pub fn loop_cursor_mut(&mut self) -> &mut ListCursor {
        &mut self.builder.my_loop_iterator
    }
    /// Changes the actual comparison policy; `*state.unknown_raises_mut() = true`.
    pub fn unknown_raises_mut(&mut self) -> &mut bool {
        &mut self.builder.my_unknown_raise
    }
    /// Transfers a list without copying nodes and rebinds only the area cursor; `state.bind_area_cursor(owner, cursor)?`.
    pub fn bind_area_cursor(&mut self, owner: AreaList, cursor: ListCursor) -> Result<(), Failure> {
        self.builder.retain_area_cursor_source(owner, cursor)
    }
    /// Transfers a list without copying nodes and rebinds only the loop cursor; `state.bind_loop_cursor(owner, cursor)?`.
    pub fn bind_loop_cursor(
        &mut self,
        owner: AreaLoopList,
        cursor: ListCursor,
    ) -> Result<(), Failure> {
        self.builder.retain_loop_cursor_source(owner, cursor)
    }
    /// Edits the retained area source without snapshots; `state.area_cursor_source_mut().unwrap().append(area)`.
    pub fn area_cursor_source_mut(&mut self) -> Option<&mut AreaList> {
        self.builder.area_cursor_owner_mut()
    }
    /// Edits the retained loop source without snapshots; `state.loop_cursor_source_mut().unwrap().append(loop_handle)`.
    pub fn loop_cursor_source_mut(&mut self) -> Option<&mut AreaLoopList> {
        self.builder.loop_owner_mut()
    }
}
