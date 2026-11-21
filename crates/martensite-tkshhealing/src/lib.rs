//! Shape healing for boundary representations.
//!
//! The toolkit analyzes and repairs shapes, converts and divides their geometry,
//! and applies operator sequences to imported shapes. Its public modules separate
//! analysis, construction, repair, geometry conversion and process orchestration.
// A concrete class and its Rust parent interface share one persisted descriptor.
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

pub mod sh_message;
pub mod shape_algo;
pub mod shape_analysis;
pub mod shape_build;
pub mod shape_construct;
pub mod shape_custom;
pub mod shape_extend;
pub mod shape_fix;
pub mod shape_process;
pub mod shape_process_api;
pub mod shape_upgrade;
