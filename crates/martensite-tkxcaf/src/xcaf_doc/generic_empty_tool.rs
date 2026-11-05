//! The attributes of XCAFDoc derived from `TDataStdGenericEmpty` (the tools, `XCAFDocDimension`,
//! `XCAFDocGeomTolerance`, `XCAFDocView`): the RTTI, the empty `Restore` and `Paste` and the
//! derived-attribute registration their native `IMPLEMENT_DERIVED_ATTRIBUTE` expands to.

/// Declares the boilerplate of a `TDataStdGenericEmpty`-derived XCAFDoc attribute: a struct
/// with the transient, the attribute base and the given state, its RTTI and `GetID`, and a
/// `TdfAttribute` implementation with the empty `Restore`/`Paste`, `NewEmpty`, the given
/// `DumpJson` body (the whole native body, class name included) and the given `Dump`. The constructor registers the derived attributes.
macro_rules! generic_empty_attribute {
    (
        $(#[$doc:meta])* $name:ident, $class_name:literal, $id:literal, state: $state:ty,
        dump: $dump:expr $(, after_retrieval: $after:expr)?
    ) => {
        generic_empty_attribute!($(#[$doc])* $name, $class_name, $id, state: $state, dump: $dump,
            text: |attribute: &$name, stream: &mut dyn std::io::Write| {
                martensite_tklcaf::tdf::dump_attribute(attribute, stream)
            } $(, after_retrieval: $after)?);
    };
    (
        $(#[$doc:meta])* $name:ident, $class_name:literal, $id:literal, state: $state:ty,
        dump: $dump:expr, text: $text:expr $(, after_retrieval: $after:expr)?
    ) => {
        $(#[$doc])*
        pub struct $name {
            transient: martensite_tkernel::standard::StandardTransient,
            base: martensite_tklcaf::tdf::TdfAttributeBase,
            // The tools without a cache (state `()`) never read it.
            #[allow(dead_code)]
            pub(super) state: martensite_tkernel::standard::SharedState<$state>,
        }
        martensite_tkernel::impl_transient!($name, $class_name, dyn martensite_tklcaf::tdata_std::TDataStdGenericEmpty,
            transient, interfaces: [dyn martensite_tklcaf::tdf::TdfAttribute,
            dyn martensite_tklcaf::tdata_std::TDataStdGenericEmpty]);
        impl martensite_tklcaf::tdata_std::TDataStdGenericEmpty for $name {}

        impl Default for $name {
            fn default() -> Self {
                super::registration::register_derived_attributes();
                Self {
                    transient: martensite_tkernel::standard::StandardTransient::default(),
                    base: martensite_tklcaf::tdf::TdfAttributeBase::default(),
                    state: martensite_tkernel::standard::SharedState::default(),
                }
            }
        }

        impl $name {
            #[doc = concat!("`", $class_name, "()`.")]
            pub fn new() -> Self {
                Self::default()
            }
            /// `GetID`.
            pub fn get_id() -> martensite_tkernel::standard::Guid {
                const ID: martensite_tkernel::standard::Guid = martensite_tklcaf::guid_from_text($id);
                ID
            }
        }

        impl martensite_tklcaf::tdf::TdfAttribute for $name {
            fn attribute_base(&self) -> &martensite_tklcaf::tdf::TdfAttributeBase {
                &self.base
            }
            fn id(&self) -> martensite_tkernel::standard::Guid {
                Self::get_id()
            }
            /// `Restore` of `TDataStdGenericEmpty`: nothing to restore.
            fn restore(
                &self,
                _with: &martensite_tkernel::standard::Handle<dyn martensite_tklcaf::tdf::TdfAttribute>,
            ) -> Result<(), martensite_tkernel::standard::Failure> {
                Ok(())
            }
            /// `NewEmpty` (from `IMPLEMENT_DERIVED_ATTRIBUTE`).
            fn new_empty(&self) -> martensite_tkernel::standard::Handle<dyn martensite_tklcaf::tdf::TdfAttribute> {
                martensite_tkernel::standard::Handle::new(Self::new()).cast()
            }
            /// `Paste` of `TDataStdGenericEmpty`: nothing to paste.
            fn paste(
                &self,
                _into: &martensite_tkernel::standard::Handle<dyn martensite_tklcaf::tdf::TdfAttribute>,
                _table: &martensite_tkernel::standard::Handle<martensite_tklcaf::tdf::TdfRelocationTable>,
            ) -> Result<(), martensite_tkernel::standard::Failure> {
                Ok(())
            }
            fn dump(&self, stream: &mut dyn std::io::Write) -> std::io::Result<()> {
                ($text)(self, stream)
            }
            $(
                fn after_retrieval(&self, force_it: bool) -> Result<bool, martensite_tkernel::standard::Failure> {
                    ($after)(self, force_it)
                }
            )?
            fn dump_json(
                &self,
                stream: &mut martensite_tkernel::standard::dump::DumpStream,
                depth: i32,
            ) -> std::io::Result<()> {
                ($dump)(self, stream, depth)
            }
        }
    };
}
pub(crate) use generic_empty_attribute;

/// `DUMP_BASE_CLASS(theOStream, theDepth, TDataStdGenericEmpty)`.
pub(crate) fn dump_generic_empty_base<
    A: martensite_tklcaf::tdata_std::TDataStdGenericEmpty + ?Sized,
>(
    attribute: &A,
    stream: &mut martensite_tkernel::standard::dump::DumpStream,
    depth: i32,
) -> std::io::Result<()> {
    martensite_tkservice::json_dump::base_class(stream, depth, &|s, d| {
        martensite_tklcaf::tdata_std::dump_generic_empty(attribute, s, d)
    })
}
