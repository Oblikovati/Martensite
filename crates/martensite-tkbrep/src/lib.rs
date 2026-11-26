//! The boundary representation data structure.
//!
//! The crate builds on the foundation, math, planar and spatial geometry and geometric
//! construction crates. Its modules are `topods` (the topological data structure),
//! `topexp` (its exploration), `toptools` (maps, hashers and the shape set), `brep`
//! (the geometric shapes, their builder and their queries), `brep_tools`,
//! `brep_adaptor`, `brep_lprop`, `brep_graph` and `brep_graph_inc` (the graph form of
//! a shape) and `bintools` (the binary shape stream).
// The concrete class registers the native descriptor once. Its interface also
// needs ownership conversions for derived objects with the same native parent.
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

pub mod bintools;
pub mod brep;
pub mod brep_adaptor;
pub mod brep_graph;
pub mod brep_graph_inc;
pub mod brep_lprop;
pub mod brep_tools;
pub mod operation;
pub mod topexp;
pub mod topods;
pub mod toptools;
