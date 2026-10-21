//! One native class descriptor for split interface/concrete representations, with separate Rust casts.
macro_rules! canonical_tobj_type {
    ($concrete:ty, $interface:ty, $field:ident, [$($ancestor:ty),+]) => {
        const _: () = {
            use martensite_tkernel::standard::{
                Handle, InterfaceCaster, InterfaceEntry, RuntimeType, RuntimeTyped, StandardTransient,
                Transient, project_concrete,
            };
            use std::{
                any::{Any, TypeId},
                sync::OnceLock,
            };
            impl RuntimeTyped for $concrete {
                const TYPE_NAME: &'static str = <$interface as RuntimeTyped>::TYPE_NAME;
                fn static_type() -> &'static RuntimeType { <$interface>::static_type() }
                fn static_type_ref() -> &'static Handle<RuntimeType> { <$interface>::static_type_ref() }
                fn project_from(object: &dyn Transient) -> Option<&Self> { project_concrete(object) }
            }
            impl Transient for $concrete {
                fn dynamic_type(&self) -> &'static RuntimeType { Self::static_type() }
                fn dynamic_type_ref(&self) -> &'static Handle<RuntimeType> { Self::static_type_ref() }
                fn rust_interface_descriptor(&self) -> &RuntimeType {
                    static CASTERS: OnceLock<Handle<RuntimeType>> = OnceLock::new();
                    CASTERS.get_or_init(|| RuntimeType::register_owned_with_interfaces::<$concrete>(
                        concat!(stringify!($concrete), " Rust projections"), std::mem::size_of::<$concrete>(),
                        Some(Self::static_type_handle()),
                        vec![$(InterfaceEntry::new(InterfaceCaster::<$ancestor>::new(|object| {
                            Some(project_concrete::<$concrete>(object)? as &$ancestor)
                        }))),+],
                    )).get().expect("TObj concrete Rust projection descriptor")
                }
                fn project(&self, requested: TypeId) -> Option<&dyn Any> {
                    if requested == TypeId::of::<Self>() { return Some(self); }
                    self.$field.project(requested)
                }
                fn transient_base(&self) -> &StandardTransient { &self.$field }
            }
        };
    };
}
pub(super) use canonical_tobj_type;
