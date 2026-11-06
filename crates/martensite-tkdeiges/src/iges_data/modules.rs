//! The services a norm's package gives on its entities, by case number.
//!
//! A package writes three modules, as traits over the ones of the general interface:
//! [`IgesDataReadWriteModule`] reads and writes the own parameters, [`IgesDataGeneralModule`]
//! gives their sharing, checks and copies, and [`IgesDataSpecificModule`] dumps and corrects.
//! The orphan rule forbids blanket impls across the crates, so `impl_iges_read_write_module!`
//! and `impl_iges_general_module!` connect a module to the general interface, and
//! the shared bodies are the `iges_*` functions here.
use super::{
    IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity, IgesDataIgesReaderData,
    IgesDataIgesWriter, IgesDataParamReader, IgesDataProtocolOps,
};
use martensite_tkernel::{
    impl_transient_interface,
    standard::{Failure, Handle, StandardTransient, Transient},
};
use martensite_tkxsbase::{
    interface::{
        InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceFileReaderData,
        InterfaceGeneralModule, InterfaceReaderModule, InterfaceShareTool,
    },
    lib_ctl::{LibCtlGlobalNode, LibCtlKind, LibCtlLibrary, LibCtlNode, LibCtlRegistry},
};
use std::io;
use std::sync::OnceLock;

/// What can go wrong in a dump: the stream, or an entity not fully set up.
#[derive(Debug)]
pub enum IgesDataDumpError {
    /// The stream failed.
    Io(io::Error),
    /// A member of an entity failed.
    Failure(Failure),
}

impl From<io::Error> for IgesDataDumpError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<Failure> for IgesDataDumpError {
    fn from(error: Failure) -> Self {
        Self::Failure(error)
    }
}

/// The result of a dump.
pub type IgesDataDumpResult = Result<(), IgesDataDumpError>;

/// The reading and writing of the own parameters of a package's entities, by case number.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataReadWriteModule;
/// fn known(module: &dyn IgesDataReadWriteModule) -> bool { module.case_iges(110, 0) > 0 }
/// ```
pub trait IgesDataReadWriteModule: InterfaceReaderModule {
    /// The case of a type and form, 0 when not recognised.
    fn case_iges(&self, typenum: i32, formnum: i32) -> i32;
    /// Reads the own parameters of `ent` by `pr`, then sets `ent` up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    fn read_own_params(
        &self,
        cn: i32,
        ent: &Handle<IgesDataIgesEntity>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure>;
    /// Writes the own parameters of `ent`.
    ///
    /// # Errors
    /// A failure of the writer or an entity member.
    fn write_own_params(
        &self,
        cn: i32,
        ent: &Handle<IgesDataIgesEntity>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure>;
}
impl_transient_interface!(
    dyn IgesDataReadWriteModule,
    "IgesDataReadWriteModule",
    dyn InterfaceReaderModule
);

/// The case number of a record, from the type and form of its directory entry.
pub fn iges_module_case_num<M: IgesDataReadWriteModule + ?Sized>(
    module: &M,
    data: &Handle<dyn InterfaceFileReaderData>,
    num: i32,
) -> i32 {
    data.cast::<IgesDataIgesReaderData>()
        .get()
        .map_or(0, |live| {
            let dir_type = live.dir_type(num);
            module.case_iges(dir_type.type_number(), dir_type.form())
        })
}

/// Implements `InterfaceReaderModule` for an [`IgesDataReadWriteModule`]; the reader tool does the
/// reading, so its `read` does nothing.
#[macro_export]
macro_rules! impl_iges_read_write_module {
    ($module:ty) => {
        impl martensite_tkxsbase::interface::InterfaceReaderModule for $module {
            fn case_num(
                &self,
                data: &martensite_tkernel::standard::Handle<
                    dyn martensite_tkxsbase::interface::InterfaceFileReaderData,
                >,
                num: i32,
            ) -> i32 {
                $crate::iges_data::iges_module_case_num(self, data, num)
            }
            fn read(
                &self,
                _casenum: i32,
                _data: &martensite_tkernel::standard::Handle<
                    dyn martensite_tkxsbase::interface::InterfaceFileReaderData,
                >,
                _num: i32,
                _ach: &mut martensite_tkernel::standard::Handle<
                    martensite_tkxsbase::interface::InterfaceCheck,
                >,
                _ent: &martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
            ) -> Result<(), martensite_tkernel::standard::Failure> {
                Ok(())
            }
        }
    };
}

/// The services of a package on its entities, by case number; the `iges_*` shared bodies apply
/// them with what every entity has in common.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataGeneralModule;
/// fn empty_case(module: &dyn IgesDataGeneralModule) -> bool {
///     let mut made = Default::default();
///     module.iges_new_void(0, &mut made)
/// }
/// ```
pub trait IgesDataGeneralModule: InterfaceGeneralModule {
    /// Adds the entities the own parameters of `ent` refer to.
    ///
    /// # Errors
    /// A failure of an entity member.
    fn own_shared_case(
        &self,
        cn: i32,
        ent: &Handle<IgesDataIgesEntity>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure>;
    /// Adds the entities implied by the own parameters; none by default.
    ///
    /// # Errors
    /// A failure of an entity member.
    fn own_implied_case(
        &self,
        _cn: i32,
        _ent: &Handle<IgesDataIgesEntity>,
        _iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// The criteria on the directory entry of the case.
    fn dir_checker(&self, cn: i32, ent: &Handle<IgesDataIgesEntity>) -> IgesDataDirChecker;
    /// Checks the own parameters.
    ///
    /// # Errors
    /// A failure of an entity member.
    fn own_check_case(
        &self,
        cn: i32,
        ent: &Handle<IgesDataIgesEntity>,
        shares: &InterfaceShareTool,
        ach: &mut Handle<InterfaceCheck>,
    ) -> Result<(), Failure>;
    /// An empty entity of the case; false when the case is not known.
    fn iges_new_void(&self, cn: i32, entto: &mut Handle<StandardTransient>) -> bool;
    /// Copies the own parameters.
    ///
    /// # Errors
    /// A failure of an entity member or of the copy tool.
    fn own_copy_case(
        &self,
        cn: i32,
        entfrom: &Handle<IgesDataIgesEntity>,
        entto: &Handle<IgesDataIgesEntity>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure>;
    /// Renews the references that were not copied; nothing by default.
    ///
    /// # Errors
    /// A failure of an entity member or of the copy tool.
    fn own_renew_case(
        &self,
        _cn: i32,
        _entfrom: &Handle<IgesDataIgesEntity>,
        _entto: &Handle<IgesDataIgesEntity>,
        _tc: &InterfaceCopyTool,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// What deleting the entity implies; nothing by default.
    fn own_delete_case(&self, _cn: i32, _ent: &Handle<IgesDataIgesEntity>) {}
    /// The category of an entity; not specified by default.
    fn iges_category_number(
        &self,
        _cn: i32,
        _ent: &Handle<StandardTransient>,
        _shares: &InterfaceShareTool,
    ) -> i32 {
        0
    }
}
impl_transient_interface!(
    dyn IgesDataGeneralModule,
    "IgesDataGeneralModule",
    dyn InterfaceGeneralModule
);

/// Implements `InterfaceGeneralModule` for an [`IgesDataGeneralModule`].
///
/// `fill_shared_case`, `list_implied_case` and `when_delete_case` cannot report: a failure of an
/// own member there adds nothing more to the list.
#[macro_export]
macro_rules! impl_iges_general_module {
    ($module:ty) => {
        impl martensite_tkxsbase::interface::InterfaceGeneralModule for $module {
            fn fill_shared_case(
                &self,
                cn: i32,
                ent: &martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
                iter: &mut martensite_tkxsbase::interface::InterfaceEntityIterator,
            ) {
                let _kept_partial = $crate::iges_data::iges_fill_shared_case(self, cn, ent, iter);
            }
            fn list_implied_case(
                &self,
                cn: i32,
                ent: &martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
                iter: &mut martensite_tkxsbase::interface::InterfaceEntityIterator,
            ) {
                let _kept_partial = $crate::iges_data::iges_list_implied_case(self, cn, ent, iter);
            }
            fn check_case(
                &self,
                cn: i32,
                ent: &martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
                shares: &martensite_tkxsbase::interface::InterfaceShareTool,
                ach: &mut martensite_tkernel::standard::Handle<
                    martensite_tkxsbase::interface::InterfaceCheck,
                >,
            ) -> Result<(), martensite_tkernel::standard::Failure> {
                $crate::iges_data::iges_check_case(self, cn, ent, shares, ach)
            }
            fn can_copy(
                &self,
                _cn: i32,
                _ent: &martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
            ) -> bool {
                true
            }
            fn new_void(
                &self,
                cn: i32,
                entto: &mut martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
            ) -> bool {
                $crate::iges_data::IgesDataGeneralModule::iges_new_void(self, cn, entto)
            }
            fn copy_case(
                &self,
                cn: i32,
                entfrom: &martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
                entto: &martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
                tc: &mut martensite_tkxsbase::interface::InterfaceCopyTool,
            ) -> Result<(), martensite_tkernel::standard::Failure> {
                $crate::iges_data::iges_copy_case(self, cn, entfrom, entto, tc)
            }
            fn renew_implied_case(
                &self,
                cn: i32,
                entfrom: &martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
                entto: &martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
                tc: &martensite_tkxsbase::interface::InterfaceCopyTool,
            ) -> Result<(), martensite_tkernel::standard::Failure> {
                $crate::iges_data::iges_renew_implied_case(self, cn, entfrom, entto, tc)
            }
            fn when_delete_case(
                &self,
                cn: i32,
                ent: &martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
                _dispatched: bool,
            ) {
                $crate::iges_data::iges_when_delete_case(self, cn, ent);
            }
            fn category_number(
                &self,
                cn: i32,
                ent: &martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
                shares: &martensite_tkxsbase::interface::InterfaceShareTool,
            ) -> i32 {
                $crate::iges_data::IgesDataGeneralModule::iges_category_number(
                    self, cn, ent, shares,
                )
            }
            fn name(
                &self,
                _cn: i32,
                ent: &martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
                _shares: &martensite_tkxsbase::interface::InterfaceShareTool,
            ) -> martensite_tkernel::standard::Handle<martensite_tkernel::tcollection::HAsciiString>
            {
                $crate::iges_data::iges_name_of(ent)
            }
        }
    };
}

/// The dump and the corrections of a package's entities, by case number.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataSpecificModule;
/// fn corrects(module: &dyn IgesDataSpecificModule) -> bool {
///     module.own_correct(0, &Default::default()).unwrap_or(false)
/// }
/// ```
pub trait IgesDataSpecificModule: Transient {
    /// Writes the own parameters of `ent`, at the level `own`.
    ///
    /// # Errors
    /// A failure of the stream or of an entity member.
    fn own_dump(
        &self,
        cn: i32,
        ent: &Handle<IgesDataIgesEntity>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn io::Write,
        own: i32,
    ) -> IgesDataDumpResult;
    /// Corrects the own parameters of `ent`; true when something changed. Nothing by default.
    ///
    /// # Errors
    /// A failure of an entity member.
    fn own_correct(&self, _cn: i32, _ent: &Handle<IgesDataIgesEntity>) -> Result<bool, Failure> {
        Ok(false)
    }
}
impl_transient_interface!(
    dyn IgesDataSpecificModule,
    "IgesDataSpecificModule",
    StandardTransient
);

/// The library of the read-write modules of the norm.
pub struct IgesDataWriterLibKind;

impl LibCtlKind for IgesDataWriterLibKind {
    type Module = dyn IgesDataReadWriteModule;
    type Protocol = dyn IgesDataProtocolOps;
    fn registry() -> &'static LibCtlRegistry<Self> {
        static REGISTRY: OnceLock<LibCtlRegistry<IgesDataWriterLibKind>> = OnceLock::new();
        REGISTRY.get_or_init(LibCtlRegistry::new)
    }
}

/// Writer library.
pub type IgesDataWriterLib = LibCtlLibrary<IgesDataWriterLibKind>;
/// Global node of the writer library.
pub type IgesDataGlobalNodeOfWriterLib = LibCtlGlobalNode<IgesDataWriterLibKind>;
/// Node of the writer library.
pub type IgesDataNodeOfWriterLib = LibCtlNode<IgesDataWriterLibKind>;

/// The library of the specific modules of the norm.
pub struct IgesDataSpecificLibKind;

impl LibCtlKind for IgesDataSpecificLibKind {
    type Module = dyn IgesDataSpecificModule;
    type Protocol = dyn IgesDataProtocolOps;
    fn registry() -> &'static LibCtlRegistry<Self> {
        static REGISTRY: OnceLock<LibCtlRegistry<IgesDataSpecificLibKind>> = OnceLock::new();
        REGISTRY.get_or_init(LibCtlRegistry::new)
    }
}

/// Specific library.
pub type IgesDataSpecificLib = LibCtlLibrary<IgesDataSpecificLibKind>;
/// Global node of the specific library.
pub type IgesDataGlobalNodeOfSpecificLib = LibCtlGlobalNode<IgesDataSpecificLibKind>;
/// Node of the specific library.
pub type IgesDataNodeOfSpecificLib = LibCtlNode<IgesDataSpecificLibKind>;
