//! The linked lists of a library: the global list of (module, protocol) pairs, and the
//! list of global nodes a library selected.
use super::LibCtlKind;
use martensite_tkernel::standard::{Handle, SharedState, StandardTransient};
use std::sync::Arc;

/// The fields of a global node.
struct GlobalNodeState<K: LibCtlKind> {
    themod: Handle<K::Module>,
    theprot: Handle<K::Protocol>,
    thenext: Option<Arc<LibCtlGlobalNode<K>>>,
}

/// `LibCtlGlobalNode` (`InterfaceGlobalNodeOfGeneralLib`...): one (module, protocol)
/// pair of the global list.
pub struct LibCtlGlobalNode<K: LibCtlKind> {
    transient: StandardTransient,
    state: SharedState<GlobalNodeState<K>>,
}

impl<K: LibCtlKind> Default for LibCtlGlobalNode<K> {
    fn default() -> Self {
        Self {
            transient: StandardTransient::default(),
            state: SharedState::new(GlobalNodeState {
                themod: Handle::default(),
                theprot: Handle::default(),
                thenext: None,
            }),
        }
    }
}

impl<K: LibCtlKind> LibCtlGlobalNode<K> {
    /// `LibCtlGlobalNode()`: empty.
    pub fn new() -> Self {
        Self::default()
    }
    /// `Add(amodule, aprotocol)`: a module already recorded is ignored; the same protocol
    /// gets the new module; else the pair goes to the end of the list.
    pub fn add(&self, amodule: &Handle<K::Module>, aprotocol: &Handle<K::Protocol>) {
        let next = {
            let mut state = self.state.write();
            if state.themod == *amodule {
                return;
            }
            if state.theprot == *aprotocol {
                state.themod = amodule.clone();
                return;
            }
            if state.thenext.is_none() && state.themod.is_null() {
                state.themod = amodule.clone();
                state.theprot = aprotocol.clone();
                return;
            }
            state
                .thenext
                .get_or_insert_with(|| Arc::new(Self::new()))
                .clone()
        };
        next.add(amodule, aprotocol);
    }
    /// `Module`.
    pub fn module(&self) -> Handle<K::Module> {
        self.state.read().themod.clone()
    }
    /// `Protocol`.
    pub fn protocol(&self) -> Handle<K::Protocol> {
        self.state.read().theprot.clone()
    }
    /// `Next`.
    pub fn next(&self) -> Option<Arc<LibCtlGlobalNode<K>>> {
        self.state.read().thenext.clone()
    }
}

/// The fields of a library node.
struct NodeState<K: LibCtlKind> {
    thenode: Option<Arc<LibCtlGlobalNode<K>>>,
    thenext: Option<Arc<LibCtlNode<K>>>,
}

/// `LibCtlNode` (`InterfaceNodeOfGeneralLib`...): one global node selected by a library.
pub struct LibCtlNode<K: LibCtlKind> {
    transient: StandardTransient,
    state: SharedState<NodeState<K>>,
}

impl<K: LibCtlKind> Default for LibCtlNode<K> {
    fn default() -> Self {
        Self {
            transient: StandardTransient::default(),
            state: SharedState::new(NodeState {
                thenode: None,
                thenext: None,
            }),
        }
    }
}

impl<K: LibCtlKind> LibCtlNode<K> {
    /// `LibCtlNode()`: empty.
    pub fn new() -> Self {
        Self::default()
    }
    /// `AddNode(anode)`: appends a global node not yet listed.
    pub fn add_node(&self, anode: &Arc<LibCtlGlobalNode<K>>) {
        let next = {
            let mut state = self.state.write();
            if state
                .thenode
                .as_ref()
                .is_some_and(|n| Arc::ptr_eq(n, anode))
            {
                return;
            }
            if state.thenext.is_none() && state.thenode.is_none() {
                state.thenode = Some(anode.clone());
                return;
            }
            state
                .thenext
                .get_or_insert_with(|| Arc::new(Self::new()))
                .clone()
        };
        next.add_node(anode);
    }
    /// `Module`: the module of the global node (null for an empty node, which the native
    /// code dereferences).
    pub fn module(&self) -> Handle<K::Module> {
        self.global().map(|n| n.module()).unwrap_or_default()
    }
    /// `Protocol`: the protocol of the global node (null for an empty node).
    pub fn protocol(&self) -> Handle<K::Protocol> {
        self.global().map(|n| n.protocol()).unwrap_or_default()
    }
    /// `Next`.
    pub fn next(&self) -> Option<Arc<LibCtlNode<K>>> {
        self.state.read().thenext.clone()
    }
    fn global(&self) -> Option<Arc<LibCtlGlobalNode<K>>> {
        self.state.read().thenode.clone()
    }
}

martensite_tkernel::impl_transient!(
    crate::interface::InterfaceGlobalNodeOfGeneralLib,
    "InterfaceGlobalNodeOfGeneralLib",
    StandardTransient,
    transient
);

martensite_tkernel::impl_transient!(
    crate::interface::InterfaceNodeOfGeneralLib,
    "InterfaceNodeOfGeneralLib",
    StandardTransient,
    transient
);

martensite_tkernel::impl_transient!(
    crate::interface::InterfaceGlobalNodeOfReaderLib,
    "InterfaceGlobalNodeOfReaderLib",
    StandardTransient,
    transient
);

martensite_tkernel::impl_transient!(
    crate::interface::InterfaceNodeOfReaderLib,
    "InterfaceNodeOfReaderLib",
    StandardTransient,
    transient
);
