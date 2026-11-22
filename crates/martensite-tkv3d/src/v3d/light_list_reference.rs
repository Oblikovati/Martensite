//! Viewer and view light lists borrow genuine handle nodes through the same native cursor mechanism.
use super::handle_chain_reference::{
    HandleChain, HandleChainStorage, V3dHandleChainCursor, V3dHandleChainNodeRef, V3dHandleChainRef,
};
use super::{V3dView, V3dViewer};
use martensite_tkservice::graphic3d::Graphic3dCLightSource;
/// Borrow actual light-list storage; `viewer.active_lights_ref().cursor()`.
pub type V3dLightListRef<'a> = V3dHandleChainRef<'a, dyn Graphic3dCLightSource>;
/// Native detached light iterator; `viewer.active_light_iterator()`.
pub type V3dLightCursor<'a> = V3dHandleChainCursor<'a, dyn Graphic3dCLightSource>;
/// Actual constHandle light node; `viewer.active_light_ref().observe()`.
pub type V3dLightNodeRef<'a> = V3dHandleChainNodeRef<'a, dyn Graphic3dCLightSource>;
impl HandleChainStorage<dyn Graphic3dCLightSource> for V3dViewer {
    fn inspect_chain(
        &self,
        key: usize,
        inspect: &mut dyn FnMut(HandleChain<'_, dyn Graphic3dCLightSource>),
    ) {
        let fields = self.state.read();
        if key == 0 {
            inspect(HandleChain::Direct(&fields.active_lights));
            return;
        }
        inspect(HandleChain::Direct(&fields.defined_lights));
    }
}
impl HandleChainStorage<dyn Graphic3dCLightSource> for V3dView {
    fn inspect_chain(
        &self,
        _key: usize,
        inspect: &mut dyn FnMut(HandleChain<'_, dyn Graphic3dCLightSource>),
    ) {
        inspect(HandleChain::Direct(&self.state.read().active_lights));
    }
}
impl V3dViewer {
    /// Borrow actual active-light list; `viewer.active_lights_ref().len()`.
    pub fn active_lights_ref(&self) -> V3dLightListRef<'_> {
        V3dLightListRef::new(self, 0)
    }
    /// Borrow actual defined-light list; `viewer.defined_lights_ref().len()`.
    pub fn defined_lights_ref(&self) -> V3dLightListRef<'_> {
        V3dLightListRef::new(self, 1)
    }
    /// Borrow deprecated active-light current node; `viewer.active_light_ref().observe()`.
    pub fn active_light_ref(&self) -> V3dLightNodeRef<'_> {
        self.active_lights_ref()
            .node_reference(self.state.read().light_cursors[0])
    }
    /// Borrow deprecated defined-light current node; `viewer.defined_light_ref().observe()`.
    pub fn defined_light_ref(&self) -> V3dLightNodeRef<'_> {
        self.defined_lights_ref()
            .node_reference(self.state.read().light_cursors[1])
    }
}
impl V3dView {
    /// Borrow the view's actual active-light field; `view.active_lights_ref().cursor()`.
    pub fn active_lights_ref(&self) -> V3dLightListRef<'_> {
        V3dLightListRef::new(self, 0)
    }
    /// Borrow deprecated actual light node; `view.active_light_ref().observe()`.
    pub fn active_light_ref(&self) -> V3dLightNodeRef<'_> {
        self.active_lights_ref()
            .node_reference(self.state.read().active_lights_cursor)
    }
}
