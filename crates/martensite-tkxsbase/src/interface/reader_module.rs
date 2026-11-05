//! `InterfaceReaderModule`: the reading of the entities of a norm from file records,
//! by case number; `InterfaceReaderLib`: its LibCtl library.
use super::{InterfaceCheck, InterfaceFileReaderData, InterfaceProtocol};
use crate::lib_ctl::{LibCtlGlobalNode, LibCtlKind, LibCtlLibrary, LibCtlNode, LibCtlRegistry};
use martensite_tkernel::{
    impl_transient_interface,
    standard::{Failure, Handle, StandardTransient, Transient},
};
use std::sync::OnceLock;

/// `InterfaceReaderModule`, abstract. `Read` returns `Result` because the readers of
/// the norms raise on malformed parameters.
///
/// ```
/// use martensite_tkxsbase::interface::InterfaceReaderModule;
/// fn case(module: &dyn InterfaceReaderModule, data: &martensite_tkernel::standard::Handle<dyn martensite_tkxsbase::interface::InterfaceFileReaderData>) -> i32 {
///     module.case_num(data, 1)
/// }
/// ```
pub trait InterfaceReaderModule: Transient {
    /// `CaseNum(data, num)`: the case number of record `num`, 0 when not recognised.
    fn case_num(&self, data: &Handle<dyn InterfaceFileReaderData>, num: i32) -> i32;
    /// `Read(casenum, data, num, ach, ent)`: fills `ent` from record `num`.
    fn read(
        &self,
        casenum: i32,
        data: &Handle<dyn InterfaceFileReaderData>,
        num: i32,
        ach: &mut Handle<InterfaceCheck>,
        ent: &Handle<StandardTransient>,
    ) -> Result<(), Failure>;
    /// `NewRead(casenum, data, num, ach, ent)`: creates and reads an entity; false
    /// (not done) by default.
    fn new_read(
        &self,
        _casenum: i32,
        _data: &Handle<dyn InterfaceFileReaderData>,
        _num: i32,
        _ach: &mut Handle<InterfaceCheck>,
        _ent: &mut Handle<StandardTransient>,
    ) -> Result<bool, Failure> {
        Ok(false)
    }
}
impl_transient_interface!(
    dyn InterfaceReaderModule,
    "InterfaceReaderModule",
    StandardTransient
);

/// The instance of the LibCtl template for `InterfaceReaderModule`.
pub struct InterfaceReaderLibKind;

impl LibCtlKind for InterfaceReaderLibKind {
    type Module = dyn InterfaceReaderModule;
    type Protocol = dyn InterfaceProtocol;
    fn registry() -> &'static LibCtlRegistry<Self> {
        static REGISTRY: OnceLock<LibCtlRegistry<InterfaceReaderLibKind>> = OnceLock::new();
        REGISTRY.get_or_init(LibCtlRegistry::new)
    }
}

/// `InterfaceReaderLib`.
pub type InterfaceReaderLib = LibCtlLibrary<InterfaceReaderLibKind>;
/// `InterfaceGlobalNodeOfReaderLib`.
pub type InterfaceGlobalNodeOfReaderLib = LibCtlGlobalNode<InterfaceReaderLibKind>;
/// `InterfaceNodeOfReaderLib`.
pub type InterfaceNodeOfReaderLib = LibCtlNode<InterfaceReaderLibKind>;
