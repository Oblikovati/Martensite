//! The protocols of the norm: what a package knows of its entities, and several as one.
use super::{IgesDataIgesModel, IgesDataUndefinedEntity};
use martensite_tkernel::{
    impl_transient, impl_transient_interface,
    standard::{Handle, RuntimeType, RuntimeTyped, SharedState, StandardTransient},
};
use martensite_tkxsbase::interface::{InterfaceInterfaceModel, InterfaceProtocol};

/// What a protocol of the norm redefines. A package's protocol implements this, with
/// `impl_iges_protocol!` for the general interface.
///
/// ```
/// use martensite_tkdeiges::iges_data::{IgesDataProtocol, IgesDataProtocolOps};
/// assert_eq!(IgesDataProtocol::default().iges_nb_resources(), 0);
/// ```
pub trait IgesDataProtocolOps: InterfaceProtocol {
    /// The count of protocols this one builds on.
    fn iges_nb_resources(&self) -> i32 {
        0
    }
    /// The protocol this one builds on, at a rank from 1; null when none.
    fn iges_resource(&self, _num: i32) -> Handle<dyn InterfaceProtocol> {
        Handle::default()
    }
    /// The case number of a type, 0 when not recognised; here the undefined entity.
    fn iges_type_number(&self, atype: &RuntimeType) -> i32 {
        i32::from(atype.is_subtype(IgesDataUndefinedEntity::static_type()))
    }
}
impl_transient_interface!(
    dyn IgesDataProtocolOps,
    "IgesDataProtocolOps",
    dyn InterfaceProtocol
);

/// A new model of the norm.
pub fn iges_protocol_new_model() -> Handle<dyn InterfaceInterfaceModel> {
    IgesDataIgesModel::new().cast()
}

/// Whether a model is of the norm.
pub fn iges_protocol_is_suitable_model(model: &Handle<dyn InterfaceInterfaceModel>) -> bool {
    model.is_kind(IgesDataIgesModel::static_type())
}

/// The entity kept for what is not recognised.
pub fn iges_protocol_unknown_entity() -> Handle<StandardTransient> {
    Handle::new(IgesDataUndefinedEntity::new()).cast()
}

/// Whether an entity is the one kept for what is not recognised.
pub fn iges_protocol_is_unknown_entity(ent: &Handle<StandardTransient>) -> bool {
    ent.is_kind(IgesDataUndefinedEntity::static_type())
}

/// Implements `InterfaceProtocol` for an [`IgesDataProtocolOps`].
#[macro_export]
macro_rules! impl_iges_protocol {
    ($protocol:ty) => {
        impl martensite_tkxsbase::interface::InterfaceProtocol for $protocol {
            fn nb_resources(&self) -> i32 {
                $crate::iges_data::IgesDataProtocolOps::iges_nb_resources(self)
            }
            fn resource(
                &self,
                num: i32,
            ) -> martensite_tkernel::standard::Handle<
                dyn martensite_tkxsbase::interface::InterfaceProtocol,
            > {
                $crate::iges_data::IgesDataProtocolOps::iges_resource(self, num)
            }
            fn type_number(&self, atype: &martensite_tkernel::standard::RuntimeType) -> i32 {
                $crate::iges_data::IgesDataProtocolOps::iges_type_number(self, atype)
            }
            fn new_model(
                &self,
            ) -> martensite_tkernel::standard::Handle<
                dyn martensite_tkxsbase::interface::InterfaceInterfaceModel,
            > {
                $crate::iges_data::iges_protocol_new_model()
            }
            fn is_suitable_model(
                &self,
                model: &martensite_tkernel::standard::Handle<
                    dyn martensite_tkxsbase::interface::InterfaceInterfaceModel,
                >,
            ) -> bool {
                $crate::iges_data::iges_protocol_is_suitable_model(model)
            }
            fn unknown_entity(
                &self,
            ) -> martensite_tkernel::standard::Handle<
                martensite_tkernel::standard::StandardTransient,
            > {
                $crate::iges_data::iges_protocol_unknown_entity()
            }
            fn is_unknown_entity(
                &self,
                ent: &martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
            ) -> bool {
                $crate::iges_data::iges_protocol_is_unknown_entity(ent)
            }
        }
    };
}

/// The basic protocol of the norm: it recognises only the undefined entity.
///
/// ```
/// use martensite_tkdeiges::iges_data::{IgesDataProtocol, IgesDataUndefinedEntity};
/// use martensite_tkernel::standard::RuntimeTyped;
/// use martensite_tkxsbase::interface::InterfaceProtocol;
/// let protocol = IgesDataProtocol::default();
/// assert_eq!(protocol.type_number(IgesDataUndefinedEntity::static_type()), 1);
/// ```
#[derive(Default)]
pub struct IgesDataProtocol {
    transient: StandardTransient,
}
impl_transient!(
    IgesDataProtocol,
    "IgesDataProtocol",
    dyn IgesDataProtocolOps,
    transient,
    interfaces: [dyn InterfaceProtocol, dyn IgesDataProtocolOps]
);
crate::impl_iges_protocol!(IgesDataProtocol);
impl IgesDataProtocolOps for IgesDataProtocol {}

/// A protocol whose resources are the protocols added to it, each class once.
///
/// ```
/// use martensite_tkdeiges::iges_data::{IgesDataFileProtocol, IgesDataProtocol, IgesDataProtocolOps};
/// use martensite_tkernel::standard::Handle;
/// let file = IgesDataFileProtocol::default();
/// file.add(&Handle::new(IgesDataProtocol::default()).cast());
/// file.add(&Handle::new(IgesDataProtocol::default()).cast());
/// assert_eq!(file.iges_nb_resources(), 1);
/// ```
#[derive(Default)]
pub struct IgesDataFileProtocol {
    transient: StandardTransient,
    theresources: SharedState<Vec<Handle<dyn IgesDataProtocolOps>>>,
}
impl_transient!(
    IgesDataFileProtocol,
    "IgesDataFileProtocol",
    dyn IgesDataProtocolOps,
    transient,
    interfaces: [dyn InterfaceProtocol, dyn IgesDataProtocolOps]
);
crate::impl_iges_protocol!(IgesDataFileProtocol);

impl IgesDataFileProtocol {
    /// Adds a resource, unless one of the same class is there.
    pub fn add(&self, protocol: &Handle<dyn IgesDataProtocolOps>) {
        let Some(kind) = protocol.dynamic_type() else {
            return;
        };
        let mut resources = self.theresources.write();
        if resources.iter().any(|known| known.is_instance(kind)) {
            return;
        }
        resources.push(protocol.clone());
    }
}

impl IgesDataProtocolOps for IgesDataFileProtocol {
    fn iges_nb_resources(&self) -> i32 {
        self.theresources.read().len() as i32
    }
    fn iges_resource(&self, num: i32) -> Handle<dyn InterfaceProtocol> {
        let resources = self.theresources.read();
        usize::try_from(num - 1)
            .ok()
            .and_then(|rank| resources.get(rank))
            .map_or_else(Handle::default, |found| found.cast())
    }
}
