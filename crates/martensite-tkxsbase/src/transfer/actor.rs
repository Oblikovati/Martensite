//! The actors of the transfer processes: `Recognize` and `Transferring` are virtual, the
//! chain of actors (`SetNext`, `SetLast`) is common; xsbase-design.md section 3.
use super::{
    TransferBinder, TransferFinderKind, TransferProcessKind, TransferSimpleBinderOfTransient,
    TransferTransientKind,
};
use martensite_tkernel::{
    impl_transient,
    message::ProgressRange,
    standard::{
        Failure, Handle, RuntimeType, RuntimeTyped, SharedState, StandardTransient, Transient,
        project_interface,
    },
};

/// The private fields of an actor: the next actor of the chain and the "last" flag.
pub struct TransferActor<K: TransferProcessKind> {
    thenext: SharedState<Handle<K::Actor>>,
    thelast: SharedState<bool>,
}

impl<K: TransferProcessKind> Default for TransferActor<K> {
    /// The native constructor: no next actor, not last.
    fn default() -> Self {
        Self {
            thenext: SharedState::new(Handle::default()),
            thelast: SharedState::new(false),
        }
    }
}

/// The virtual members of `TransferActorOfProcessForTransient` /
/// `TransferActorOfProcessForFinder`; the non-virtual ones are provided.
///
/// ```
/// use martensite_tkxsbase::transfer::{TransferActorOfProcessForTransient, TransferActorOps};
/// use martensite_tkernel::standard::Handle;
/// let first = Handle::new(TransferActorOfProcessForTransient::new());
/// let second = Handle::new(TransferActorOfProcessForTransient::new());
/// first.set_next(&second.clone().cast());
/// assert!(first.next().is_some() && !first.is_last());
/// ```
pub trait TransferActorOps<K: TransferProcessKind>: Transient {
    /// The embedded private fields.
    fn actor(&self) -> &TransferActor<K>;
    /// `Recognize(start)`: by default every object.
    fn recognize(&self, _start: &K::Start) -> bool {
        true
    }
    /// `Transferring(start, TP, theProgress)`: by default no result (a null binder).
    fn transferring(
        &self,
        _start: &K::Start,
        _tp: &Handle<K::Process>,
        _progress: &ProgressRange<'_>,
    ) -> Result<Handle<dyn TransferBinder>, Failure> {
        Ok(self.null_result())
    }
    /// `TransientResult(res)`: a simple binder of `res` (null for a null `res`).
    fn transient_result(
        &self,
        res: &Handle<StandardTransient>,
    ) -> Handle<TransferSimpleBinderOfTransient> {
        if res.is_null() {
            return Handle::default();
        }
        let binder = TransferSimpleBinderOfTransient::new();
        // a new binder is never "used": SetResult cannot fail
        let _ = binder.set_result(res);
        Handle::new(binder)
    }
    /// `NullResult`: a null binder.
    fn null_result(&self) -> Handle<dyn TransferBinder> {
        Handle::default()
    }
    /// `SetLast(mode = true)`: a last actor stays at the end of the chain.
    fn set_last(&self, mode: bool) {
        *self.actor().thelast.write() = mode;
    }
    /// `IsLast`.
    fn is_last(&self) -> bool {
        *self.actor().thelast.read()
    }
    /// `SetNext(next)`: inserts `next` in the chain, before the first "last" actor.
    fn set_next(&self, next: &Handle<K::Actor>) {
        let current = self.next();
        if current == *next {
            return;
        }
        let Some(live) = current.get() else {
            *self.actor().thenext.write() = next.clone();
            return;
        };
        if live.is_last() {
            next.set_next(&current);
            *self.actor().thenext.write() = next.clone();
            return;
        }
        live.set_next(next);
    }
    /// `Next`.
    fn next(&self) -> Handle<K::Actor> {
        self.actor().thenext.read().clone()
    }
}
// The interface and embedded class represent the same native parent, not two
// distinct descriptors. Preserve both virtual projection and static parent identity
// (TransferActorOfProcessFor*.
macro_rules! actor_interface_descriptor {
    ($kind:ty, $class:ty) => {
        impl RuntimeTyped for dyn TransferActorOps<$kind> {
            const TYPE_NAME: &'static str = <$class as RuntimeTyped>::TYPE_NAME;
            fn static_type() -> &'static RuntimeType {
                <$class as RuntimeTyped>::static_type()
            }
            fn static_type_ref() -> &'static Handle<RuntimeType> {
                <$class as RuntimeTyped>::static_type_ref()
            }
            fn project_from(object: &dyn Transient) -> Option<&Self> {
                project_interface::<Self>(object)
            }
        }
    };
}
actor_interface_descriptor!(TransferTransientKind, TransferActorOfProcessForTransient);
actor_interface_descriptor!(TransferFinderKind, TransferActorOfProcessForFinder);

/// `TransferActorOfProcessForTransient` (concrete): recognizes everything, transfers
/// nothing.
#[derive(Default)]
pub struct TransferActorOfProcessForTransient {
    transient: StandardTransient,
    core: TransferActor<TransferTransientKind>,
}
impl_transient!(TransferActorOfProcessForTransient, "TransferActorOfProcessForTransient", StandardTransient, transient, interfaces: [dyn TransferActorOps<TransferTransientKind>]);

impl TransferActorOfProcessForTransient {
    /// `TransferActorOfProcessForTransient()`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl TransferActorOps<TransferTransientKind> for TransferActorOfProcessForTransient {
    fn actor(&self) -> &TransferActor<TransferTransientKind> {
        &self.core
    }
}

/// `TransferActorOfProcessForFinder` (concrete): recognizes everything, transfers
/// nothing.
#[derive(Default)]
pub struct TransferActorOfProcessForFinder {
    transient: StandardTransient,
    core: TransferActor<TransferFinderKind>,
}
impl_transient!(TransferActorOfProcessForFinder, "TransferActorOfProcessForFinder", StandardTransient, transient, interfaces: [dyn TransferActorOps<TransferFinderKind>]);

impl TransferActorOfProcessForFinder {
    /// `TransferActorOfProcessForFinder()`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl TransferActorOps<TransferFinderKind> for TransferActorOfProcessForFinder {
    fn actor(&self) -> &TransferActor<TransferFinderKind> {
        &self.core
    }
}
