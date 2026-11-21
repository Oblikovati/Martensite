use super::{Algorithm, AlgorithmState};
use crate::standard::{
    HandleUpcast, InterfaceCaster, InterfaceEntry, RuntimeType, RuntimeTyped, StandardTransient,
    Transient, TransientHandleTarget, project_concrete,
};
use std::ops::Deref;
use std::sync::{Arc, OnceLock};

/// The shared status and messenger interface of a concrete algorithm.
/// `Handle::new(builder).into_upcast::<dyn AlgorithmSource>()` keeps the builder's identity.
pub trait AlgorithmSource: Transient {
    /// Borrow the status and reporting state; `source.algorithm_state().get_status()`.
    fn algorithm_state(&self) -> &AlgorithmState;
}

impl Deref for dyn AlgorithmSource {
    type Target = AlgorithmState;

    fn deref(&self) -> &Self::Target {
        self.algorithm_state()
    }
}

impl RuntimeTyped for dyn AlgorithmSource {
    const TYPE_NAME: &'static str = Algorithm::TYPE_NAME;

    fn static_type() -> &'static RuntimeType {
        Algorithm::static_type()
    }
}
crate::impl_transient_interface_target!(dyn AlgorithmSource);

impl RuntimeTyped for Algorithm {
    const TYPE_NAME: &'static str = "Message_Algorithm";

    fn static_type() -> &'static RuntimeType {
        static DESCRIPTOR: OnceLock<&'static RuntimeType> = OnceLock::new();
        DESCRIPTOR.get_or_init(|| {
            RuntimeType::register_with_interfaces::<Self>(
                Self::TYPE_NAME,
                std::mem::size_of::<Self>(),
                Some(StandardTransient::static_type()),
                vec![algorithm_source_caster()],
            )
        })
    }
}

fn algorithm_source_caster() -> InterfaceEntry {
    InterfaceEntry::new(InterfaceCaster::<dyn AlgorithmSource>::new(
        |object| {
            project_concrete::<Algorithm>(object).map(|algorithm| algorithm as &dyn AlgorithmSource)
        },
        |owner| {
            Algorithm::from_transient(owner).map(|algorithm| algorithm as Arc<dyn AlgorithmSource>)
        },
    ))
}

impl HandleUpcast<dyn AlgorithmSource> for Algorithm {
    fn upcast_arc(owner: Arc<Self>) -> Arc<dyn AlgorithmSource> {
        owner
    }
}
