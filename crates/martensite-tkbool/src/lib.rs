//! Topological operations: section and face intersection, sweeps, evolved shapes and projections.
// Class descriptors are shared by standalone records and their native interfaces.
macro_rules! parent_interface {
    ($interface:ty, $concrete:ty, [$($ancestor:ty),* $(,)?]) => {
        impl martensite_tkernel::standard::RuntimeTyped for $interface {
            const TYPE_NAME: &'static str = <$concrete as martensite_tkernel::standard::RuntimeTyped>::TYPE_NAME;
            fn static_type() -> &'static martensite_tkernel::standard::RuntimeType {
                <$concrete as martensite_tkernel::standard::RuntimeTyped>::static_type()
            }
        }
        martensite_tkernel::impl_transient_interface_target!($interface);
        $(impl martensite_tkernel::standard::HandleUpcast<$ancestor> for $interface {
            fn upcast_arc(owner: std::sync::Arc<Self>) -> std::sync::Arc<$ancestor> {
                owner
            }
        })*
    };
}

pub mod brep_algo;
pub mod brep_fill;
pub mod brep_proj;
pub mod top_ope_brep;
pub mod top_ope_brep_build;
pub mod top_ope_brep_ds;
pub mod top_ope_brep_tool;
