//! Compile-time parent identity of library nodes, independent of their runtime descriptor.

use super::{LibCtlGlobalNode, LibCtlKind, LibCtlNode};
use martensite_tkernel::standard::{RuntimeTyped, StandardTransient, Transient};

/// Name the actual embedded parent shared by both library-node families.
///
/// The associated type preserves the generated node classes' `base_type` typedef
/// without changing their storage or the released runtime-type interface.
///
/// ```
/// use martensite_tkxsbase::{interface::InterfaceNodeOfGeneralLib, lib_ctl::LibCtlNodeBaseType};
/// let parent: <InterfaceNodeOfGeneralLib as LibCtlNodeBaseType>::BaseType = Default::default();
/// let _: martensite_tkernel::standard::StandardTransient = parent;
/// ```
pub trait LibCtlNodeBaseType {
    /// The node's embedded transient parent, also used by its runtime descriptor.
    type BaseType: RuntimeTyped + Transient;
}

impl<K: LibCtlKind> LibCtlNodeBaseType for LibCtlGlobalNode<K> {
    type BaseType = StandardTransient;
}

impl<K: LibCtlKind> LibCtlNodeBaseType for LibCtlNode<K> {
    type BaseType = StandardTransient;
}
