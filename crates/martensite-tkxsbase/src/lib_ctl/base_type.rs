//! Compile-time parent identity of library nodes, independent of their runtime descriptor.

use super::{LibCtlGlobalNode, LibCtlKind, LibCtlNode};
use martensite_tkernel::standard::{RuntimeTyped, Transient};

/// Name the root interface shared by both library-node families.
///
/// The associated type preserves the generated node classes' `base_type` typedef
/// without changing their storage or the released runtime-type interface.
///
/// ```
/// use martensite_tkxsbase::{interface::InterfaceNodeOfGeneralLib, lib_ctl::LibCtlNodeBaseType};
/// use martensite_tkernel::standard::{Handle, Transient};
/// let node = Handle::new(InterfaceNodeOfGeneralLib::new());
/// let root: Handle<<InterfaceNodeOfGeneralLib as LibCtlNodeBaseType>::BaseType> = node.into_upcast();
/// let _: Handle<dyn Transient> = root;
/// ```
pub trait LibCtlNodeBaseType {
    /// The node's transient interface, also used by its runtime descriptor.
    type BaseType: ?Sized + RuntimeTyped + Transient;
}

impl<K: LibCtlKind> LibCtlNodeBaseType for LibCtlGlobalNode<K> {
    type BaseType = dyn Transient;
}

impl<K: LibCtlKind> LibCtlNodeBaseType for LibCtlNode<K> {
    type BaseType = dyn Transient;
}
