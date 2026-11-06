//! The shape every package of IGES entities repeats: a protocol that numbers its classes, three
//! modules that dispatch by that number to the classes' tools, and an `init` that registers them.
//!
//! `iges_package_tables!` writes all of it from one table per package, so the dispatch exists once.

/// Dispatches a case number to the tool of its class, on an entity cast to that class.
///
/// The arm casts `$ent`, returns `$null` for a null cast, then evaluates `$call` with `$tool` and
/// `$typed` bound. A case outside the table evaluates to `$other`.
#[macro_export]
macro_rules! iges_case_dispatch {
    ($cn:expr, $ent:expr, $null:expr, $other:expr, [$($case:literal => $entity:ty, $class_tool:ty;)*], |$tool:ident, $typed:ident| $call:expr) => {
        match $cn {
            $($case => {
                let $typed = $ent.cast::<$entity>();
                if $typed.is_null() {
                    return $null;
                }
                let $tool = <$class_tool>::new();
                $call
            })*
            _ => $other,
        }
    };
}

/// Declares the protocol, the modules and the package class of a package of entities.
///
/// `cases` lists every class in the order of its case number, `corrected` those whose tool
/// corrects, `new_void` the expression that makes an empty entity of a case. `case_iges` and
/// `category` are functions of the package, the category taking the case and the entity, on which
/// some classes depend; `general_extra` adds members to the general module.
#[macro_export]
macro_rules! iges_package_tables {
    (
        package: $package:ident,
        protocol: $protocol:ident $protocol_name:literal,
        read_write: $read_write:ident $read_write_name:literal,
        general: $general:ident $general_name:literal,
        specific: $specific:ident $specific_name:literal,
        resources: [$($resource_rank:literal => $resource:expr),* $(,)?],
        prior_init: [$($prior:expr),* $(,)?],
        case_iges: $case_iges:path,
        category: $category:path,
        cases: [$($case:literal => $entity:ty, $class_tool:ty;)*],
        corrected: [$($fixed_case:literal => $fixed_entity:ty, $fixed_tool:ty;)*],
        new_void: [$($void_case:literal => $void:expr;)*],
        general_extra: { $($general_extra:tt)* }
    ) => {
        #[doc = concat!("What `", stringify!($package), "` knows of its entities: their case numbers and the protocols it builds on.")]
        #[derive(Default)]
        pub struct $protocol {
            base: martensite_tkernel::standard::StandardTransient,
        }
        martensite_tkernel::impl_transient!(
            $protocol,
            $protocol_name,
            dyn $crate::iges_data::IgesDataProtocolOps,
            base,
            interfaces: [dyn martensite_tkxsbase::interface::InterfaceProtocol, dyn $crate::iges_data::IgesDataProtocolOps]
        );
        $crate::impl_iges_protocol!($protocol);
        impl $crate::iges_data::IgesDataProtocolOps for $protocol {
            fn iges_nb_resources(&self) -> i32 {
                [$($resource_rank),*].len() as i32
            }
            fn iges_resource(
                &self,
                num: i32,
            ) -> martensite_tkernel::standard::Handle<dyn martensite_tkxsbase::interface::InterfaceProtocol> {
                match num {
                    $($resource_rank => $resource.cast(),)*
                    _ => martensite_tkernel::standard::Handle::default(),
                }
            }
            fn iges_type_number(&self, atype: &martensite_tkernel::standard::RuntimeType) -> i32 {
                $(if std::ptr::eq(atype, <$entity as martensite_tkernel::standard::RuntimeTyped>::static_type()) {
                    return $case;
                })*
                0
            }
        }

        #[doc = concat!("The reading and writing of the own parameters of the entities of `", stringify!($package), "`.")]
        #[derive(Default)]
        pub struct $read_write {
            base: martensite_tkernel::standard::StandardTransient,
        }
        martensite_tkernel::impl_transient!(
            $read_write,
            $read_write_name,
            dyn $crate::iges_data::IgesDataReadWriteModule,
            base,
            interfaces: [dyn martensite_tkxsbase::interface::InterfaceReaderModule, dyn $crate::iges_data::IgesDataReadWriteModule]
        );
        $crate::impl_iges_read_write_module!($read_write);
        impl $crate::iges_data::IgesDataReadWriteModule for $read_write {
            fn case_iges(&self, typenum: i32, formnum: i32) -> i32 {
                $case_iges(typenum, formnum)
            }
            fn read_own_params(
                &self,
                cn: i32,
                ent: &martensite_tkernel::standard::Handle<$crate::iges_data::IgesDataIgesEntity>,
                ir: &martensite_tkernel::standard::Handle<$crate::iges_data::IgesDataIgesReaderData>,
                pr: &mut $crate::iges_data::IgesDataParamReader,
            ) -> Result<(), martensite_tkernel::standard::Failure> {
                $crate::iges_case_dispatch!(cn, ent, Ok(()), Ok(()), [$($case => $entity, $class_tool;)*], |tool, typed| tool.read_own_params(&typed, ir, pr))
            }
            fn write_own_params(
                &self,
                cn: i32,
                ent: &martensite_tkernel::standard::Handle<$crate::iges_data::IgesDataIgesEntity>,
                iw: &mut $crate::iges_data::IgesDataIgesWriter,
            ) -> Result<(), martensite_tkernel::standard::Failure> {
                $crate::iges_case_dispatch!(cn, ent, Ok(()), Ok(()), [$($case => $entity, $class_tool;)*], |tool, typed| tool.write_own_params(&typed, iw))
            }
        }

        #[doc = concat!("The sharing, checks and copies of the entities of `", stringify!($package), "`.")]
        #[derive(Default)]
        pub struct $general {
            base: martensite_tkernel::standard::StandardTransient,
        }
        martensite_tkernel::impl_transient!(
            $general,
            $general_name,
            dyn $crate::iges_data::IgesDataGeneralModule,
            base,
            interfaces: [dyn martensite_tkxsbase::interface::InterfaceGeneralModule, dyn $crate::iges_data::IgesDataGeneralModule]
        );
        $crate::impl_iges_general_module!($general);
        impl $crate::iges_data::IgesDataGeneralModule for $general {
            fn own_shared_case(
                &self,
                cn: i32,
                ent: &martensite_tkernel::standard::Handle<$crate::iges_data::IgesDataIgesEntity>,
                iter: &mut martensite_tkxsbase::interface::InterfaceEntityIterator,
            ) -> Result<(), martensite_tkernel::standard::Failure> {
                $crate::iges_case_dispatch!(cn, ent, Ok(()), Ok(()), [$($case => $entity, $class_tool;)*], |tool, typed| tool.own_shared(&typed, iter))
            }
            fn dir_checker(
                &self,
                cn: i32,
                ent: &martensite_tkernel::standard::Handle<$crate::iges_data::IgesDataIgesEntity>,
            ) -> $crate::iges_data::IgesDataDirChecker {
                $crate::iges_case_dispatch!(
                    cn, ent, $crate::iges_data::IgesDataDirChecker::new(), $crate::iges_data::IgesDataDirChecker::new(),
                    [$($case => $entity, $class_tool;)*], |tool, typed| tool.dir_checker(&typed))
            }
            fn own_check_case(
                &self,
                cn: i32,
                ent: &martensite_tkernel::standard::Handle<$crate::iges_data::IgesDataIgesEntity>,
                shares: &martensite_tkxsbase::interface::InterfaceShareTool,
                ach: &mut martensite_tkernel::standard::Handle<martensite_tkxsbase::interface::InterfaceCheck>,
            ) -> Result<(), martensite_tkernel::standard::Failure> {
                $crate::iges_case_dispatch!(cn, ent, Ok(()), Ok(()), [$($case => $entity, $class_tool;)*], |tool, typed| tool.own_check(&typed, shares, ach))
            }
            fn iges_new_void(
                &self,
                cn: i32,
                entto: &mut martensite_tkernel::standard::Handle<martensite_tkernel::standard::StandardTransient>,
            ) -> bool {
                *entto = match cn {
                    $($void_case => $void,)*
                    _ => return false,
                };
                true
            }
            fn own_copy_case(
                &self,
                cn: i32,
                entfrom: &martensite_tkernel::standard::Handle<$crate::iges_data::IgesDataIgesEntity>,
                entto: &martensite_tkernel::standard::Handle<$crate::iges_data::IgesDataIgesEntity>,
                tc: &mut martensite_tkxsbase::interface::InterfaceCopyTool,
            ) -> Result<(), martensite_tkernel::standard::Failure> {
                $crate::iges_case_dispatch!(cn, entfrom, Ok(()), Ok(()), [$($case => $entity, $class_tool;)*], |tool, typed| tool.own_copy(&typed, &entto.cast(), tc))
            }
            fn iges_category_number(
                &self,
                cn: i32,
                ent: &martensite_tkernel::standard::Handle<martensite_tkernel::standard::StandardTransient>,
                _shares: &martensite_tkxsbase::interface::InterfaceShareTool,
            ) -> i32 {
                $category(cn, ent)
            }
            $($general_extra)*
        }

        #[doc = concat!("The dump and the corrections of the entities of `", stringify!($package), "`.")]
        #[derive(Default)]
        pub struct $specific {
            base: martensite_tkernel::standard::StandardTransient,
        }
        martensite_tkernel::impl_transient!(
            $specific,
            $specific_name,
            dyn $crate::iges_data::IgesDataSpecificModule,
            base,
            interfaces: [dyn $crate::iges_data::IgesDataSpecificModule]
        );
        impl $crate::iges_data::IgesDataSpecificModule for $specific {
            fn own_dump(
                &self,
                cn: i32,
                ent: &martensite_tkernel::standard::Handle<$crate::iges_data::IgesDataIgesEntity>,
                dumper: &$crate::iges_data::IgesDataIgesDumper,
                s: &mut dyn std::io::Write,
                own: i32,
            ) -> $crate::iges_data::IgesDataDumpResult {
                $crate::iges_case_dispatch!(cn, ent, Ok(()), Ok(()), [$($case => $entity, $class_tool;)*], |tool, typed| tool.own_dump(&typed, dumper, s, own))
            }
            #[allow(unused_variables, reason = "a package may correct nothing")]
            fn own_correct(
                &self,
                cn: i32,
                ent: &martensite_tkernel::standard::Handle<$crate::iges_data::IgesDataIgesEntity>,
            ) -> Result<bool, martensite_tkernel::standard::Failure> {
                $crate::iges_case_dispatch!(cn, ent, Ok(false), Ok(false), [$($fixed_case => $fixed_entity, $fixed_tool;)*], |tool, typed| tool.own_correct(&typed))
            }
        }

        impl $protocol {
            /// A protocol of the package.
            pub fn new() -> Self {
                Self::default()
            }
        }
        impl $read_write {
            /// A read and write module of the package.
            pub fn new() -> Self {
                Self::default()
            }
        }
        impl $general {
            /// A general module of the package.
            pub fn new() -> Self {
                Self::default()
            }
        }
        impl $specific {
            /// A specific module of the package.
            pub fn new() -> Self {
                Self::default()
            }
        }

        #[doc = concat!("The dynamic data of `", stringify!($package), "`: its protocol, and its modules once in the libraries.")]
        pub struct $package;
        impl $package {
            /// Prepares the protocol and the modules of the package and of what it builds on, once.
            ///
            /// # Errors
            /// A failure of the set-up of the data package.
            pub fn try_init() -> Result<(), martensite_tkernel::standard::Failure> {
                $($prior?;)*
                let _registered = Self::protocol();
                Ok(())
            }
            /// Prepares the package, as [`Self::try_init`] does; a failure there shows when reading.
            pub fn init() {
                let _shown_when_reading = Self::try_init();
            }
            /// The protocol of the package; its modules are registered on the first call.
            pub fn protocol() -> martensite_tkernel::standard::Handle<$protocol> {
                static PROTOCOL: std::sync::OnceLock<martensite_tkernel::standard::Handle<$protocol>> = std::sync::OnceLock::new();
                PROTOCOL.get_or_init(|| {
                    let protocol = martensite_tkernel::standard::Handle::new($protocol::default());
                    let any = protocol.cast::<dyn martensite_tkxsbase::interface::InterfaceProtocol>();
                    let norm = protocol.cast::<dyn $crate::iges_data::IgesDataProtocolOps>();
                    let read_write = martensite_tkernel::standard::Handle::new($read_write::default());
                    let general = martensite_tkernel::standard::Handle::new($general::default());
                    let specific = martensite_tkernel::standard::Handle::new($specific::default());
                    martensite_tkxsbase::interface::InterfaceGeneralLib::set_global(&general.cast(), &any);
                    martensite_tkxsbase::interface::InterfaceReaderLib::set_global(&read_write.cast(), &any);
                    $crate::iges_data::IgesDataWriterLib::set_global(&read_write.cast(), &norm);
                    $crate::iges_data::IgesDataSpecificLib::set_global(&specific.cast(), &norm);
                    protocol
                }).clone()
            }
        }
    };
}
