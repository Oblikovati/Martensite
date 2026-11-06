//! `InterfaceGeneralLib`: the LibCtl library of general modules
//! (xsbase-design.md section 2).
use super::{InterfaceGeneralModule, InterfaceProtocol};
use crate::lib_ctl::{LibCtlGlobalNode, LibCtlKind, LibCtlLibrary, LibCtlNode, LibCtlRegistry};
use std::sync::OnceLock;

/// The instance of the LibCtl template for `InterfaceGeneralModule`.
pub struct InterfaceGeneralLibKind;

impl LibCtlKind for InterfaceGeneralLibKind {
    type Module = dyn InterfaceGeneralModule;
    type Protocol = dyn InterfaceProtocol;
    fn registry() -> &'static LibCtlRegistry<Self> {
        static REGISTRY: OnceLock<LibCtlRegistry<InterfaceGeneralLibKind>> = OnceLock::new();
        REGISTRY.get_or_init(LibCtlRegistry::new)
    }
}

/// `InterfaceGeneralLib`;
/// `InterfaceGeneralLib::new(&protocol).select(&entity)` gives the module and case number.
pub type InterfaceGeneralLib = LibCtlLibrary<InterfaceGeneralLibKind>;
/// `InterfaceGlobalNodeOfGeneralLib`.
pub type InterfaceGlobalNodeOfGeneralLib = LibCtlGlobalNode<InterfaceGeneralLibKind>;
/// `InterfaceNodeOfGeneralLib`.
pub type InterfaceNodeOfGeneralLib = LibCtlNode<InterfaceGeneralLibKind>;
