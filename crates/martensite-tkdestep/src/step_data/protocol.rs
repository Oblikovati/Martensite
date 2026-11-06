//! `StepDataProtocolOps`: the basic STEP protocol, which recognises only undefined entities,
//! with its late-binding descriptions; `StepDataFileProtocol`: several protocols as one.
//!
//! The class is both concrete and a base class: [`StepDataProtocolOps`] holds its virtual
//! and non-virtual members, [`StepDataProtocol`] is the concrete class. A subclass
//! inherits the `InterfaceProtocol` overrides; [`impl_step_protocol`] writes them, because
//! the orphan rule forbids a blanket impl across the crates.
use super::{
    StepDataDescribed, StepDataECDescr, StepDataEDescr, StepDataESDescr, StepDataPDescr,
    StepDataStepModel, StepDataUndefinedEntity,
};
use martensite_tkernel::{
    impl_transient, impl_transient_interface,
    ncollection::DataMap,
    standard::{Handle, RuntimeType, RuntimeTyped, SharedState, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceGraph, InterfaceInterfaceModel, InterfaceProtocol,
};

/// The private fields of `StepDataProtocolOps`: the description tables.
#[derive(Default)]
pub struct StepDataProtocolTables {
    state: SharedState<ProtocolState>,
}

#[derive(Default)]
struct ProtocolState {
    thedscnum: DataMap<Handle<StandardTransient>, i32>,
    thedscnam: DataMap<AsciiString, Handle<dyn StepDataEDescr>>,
    thepdescr: DataMap<AsciiString, Handle<StepDataPDescr>>,
    thedscbas: DataMap<AsciiString, Handle<StepDataESDescr>>,
}

/// The resources of a protocol that are STEP protocols.
fn step_resources<P: StepDataProtocolOps + ?Sized>(
    protocol: &P,
) -> Vec<Handle<dyn StepDataProtocolOps>> {
    (1..=protocol.step_nb_resources())
        .map(|num| {
            protocol
                .step_resource(num)
                .cast::<dyn StepDataProtocolOps>()
        })
        .filter(|resource| resource.is_some())
        .collect()
}

/// `StepDataProtocolOps`, the members a subclass inherits. The `step_*` members are the
/// virtual ones of `InterfaceProtocol` this class overrides; a subclass overrides them
/// here, and [`impl_step_protocol`] connects them.
///
/// ```
/// use martensite_tkdestep::step_data::{StepDataProtocol, StepDataProtocolOps};
/// assert_eq!(StepDataProtocol::default().step_nb_resources(), 0);
/// ```
pub trait StepDataProtocolOps: InterfaceProtocol {
    /// The embedded description tables.
    fn protocol_tables(&self) -> &StepDataProtocolTables;
    /// `nb_resources`.
    fn step_nb_resources(&self) -> i32 {
        0
    }
    /// `resource(num)`.
    fn step_resource(&self, _num: i32) -> Handle<dyn InterfaceProtocol> {
        Handle::default()
    }
    /// `type_number(atype)`: here only the undefined entity.
    fn step_type_number(&self, atype: &RuntimeType) -> i32 {
        i32::from(std::ptr::eq(atype, StepDataUndefinedEntity::static_type()))
    }
    /// `global_check(G, ach)`.
    fn step_global_check(
        &self,
        _graph: &InterfaceGraph,
        _ach: &mut Handle<InterfaceCheck>,
    ) -> bool {
        true
    }
    /// `schema_name(model)`: `(DEFAULT)` here.
    fn schema_name(&self, _model: &Handle<dyn InterfaceInterfaceModel>) -> String {
        "(DEFAULT)".to_owned()
    }
    /// `descr_number(adescr)`: 0 when not recorded.
    fn descr_number(&self, adescr: &Handle<dyn StepDataEDescr>) -> i32 {
        let state = self.protocol_tables().state.read();
        state.thedscnum.find(&adescr.cast()).copied().unwrap_or(0)
    }
    /// `add_descr(adescr, CN)`: a simple description is also recorded by its name; every one
    /// by its case number in text.
    fn add_descr(&self, adescr: &Handle<dyn StepDataEDescr>, cn: i32) {
        let mut state = self.protocol_tables().state.write();
        state.thedscnum.bind(adescr.cast(), cn);
        if let Some(simple) = adescr.cast::<StepDataESDescr>().get() {
            state
                .thedscnam
                .bind(AsciiString::new(simple.type_name()), adescr.clone());
        }
        state
            .thedscnam
            .bind(AsciiString::new(&cn.to_string()), adescr.clone());
    }
    /// `has_descr`.
    fn has_descr(&self) -> bool {
        !self.protocol_tables().state.read().thedscnam.is_empty()
    }
    /// `descr(num)`.
    fn descr_of_case(&self, num: i32) -> Handle<dyn StepDataEDescr> {
        let state = self.protocol_tables().state.read();
        let key = AsciiString::new(&num.to_string());
        state.thedscnam.find(&key).cloned().unwrap_or_default()
    }
    /// `descr(name, anylevel = true)`.
    fn descr(&self, name: &str, anylevel: bool) -> Handle<dyn StepDataEDescr> {
        {
            let state = self.protocol_tables().state.read();
            if let Ok(found) = state.thedscnam.find(&AsciiString::new(name)) {
                return found.clone();
            }
        }
        if !anylevel {
            return Handle::default();
        }
        for resource in step_resources(self) {
            let found = resource.descr(name, anylevel);
            if found.is_some() {
                return found;
            }
        }
        Handle::default()
    }
    /// `ESDescr(name, anylevel = true)`.
    fn es_descr(&self, name: &str, anylevel: bool) -> Handle<StepDataESDescr> {
        self.descr(name, anylevel).cast()
    }
    /// `ECDescr(names, anylevel = true)`: the complex description with exactly these
    /// member names.
    fn ec_descr(&self, names: &[AsciiString], anylevel: bool) -> Handle<StepDataECDescr> {
        {
            let state = self.protocol_tables().state.read();
            for key in state.thedscnum.items().map(|(key, _)| key) {
                let complex = key.cast::<StepDataECDescr>();
                if complex.get().is_some_and(|c| c.type_list() == names) {
                    return complex;
                }
            }
        }
        if !anylevel {
            return Handle::default();
        }
        for resource in step_resources(self) {
            let found = resource.ec_descr(names, anylevel);
            if found.is_some() {
                return found;
            }
        }
        Handle::default()
    }
    /// `add_p_descr(pdescr)`.
    fn add_p_descr(&self, pdescr: &Handle<StepDataPDescr>) {
        let name = pdescr.get().map(StepDataPDescr::name).unwrap_or_default();
        let mut state = self.protocol_tables().state.write();
        state
            .thepdescr
            .bind(AsciiString::new(&name), pdescr.clone());
    }
    /// `PDescr(name, anylevel = true)`.
    fn p_descr(&self, name: &str, anylevel: bool) -> Handle<StepDataPDescr> {
        {
            let state = self.protocol_tables().state.read();
            if let Ok(found) = state.thepdescr.find(&AsciiString::new(name)) {
                return found.clone();
            }
        }
        if !anylevel {
            return Handle::default();
        }
        for resource in step_resources(self) {
            let found = resource.p_descr(name, anylevel);
            if found.is_some() {
                return found;
            }
        }
        Handle::default()
    }
    /// `add_basic_descr(esdescr)`.
    fn add_basic_descr(&self, esdescr: &Handle<StepDataESDescr>) {
        let name = esdescr
            .get()
            .map_or_else(String::new, |d| d.type_name().to_owned());
        let mut state = self.protocol_tables().state.write();
        state
            .thedscbas
            .bind(AsciiString::new(&name), esdescr.clone());
    }
    /// `basic_descr(name, anylevel = true)`.
    fn basic_descr(&self, name: &str, anylevel: bool) -> Handle<dyn StepDataEDescr> {
        {
            let state = self.protocol_tables().state.read();
            if let Ok(found) = state.thedscbas.find(&AsciiString::new(name)) {
                return found.cast();
            }
        }
        if !anylevel {
            return Handle::default();
        }
        for resource in step_resources(self) {
            let found = resource.basic_descr(name, anylevel);
            if found.is_some() {
                return found;
            }
        }
        Handle::default()
    }
}
impl_transient_interface!(
    dyn StepDataProtocolOps,
    "StepDataProtocolOps",
    dyn InterfaceProtocol
);

/// The `case_number(obj)` of a STEP protocol: `type_number`, then for a described
/// entity `descr_number`.
pub fn step_protocol_case_number<P: StepDataProtocolOps + ?Sized>(
    protocol: &P,
    obj: &Handle<StandardTransient>,
) -> i32 {
    let Some(kind) = obj.dynamic_type() else {
        return 0;
    };
    let num = protocol.step_type_number(kind);
    if num > 0 {
        return num;
    }
    let described = obj.cast::<dyn StepDataDescribed>();
    described
        .get()
        .map_or(0, |live| protocol.descr_number(&live.description()))
}

/// The `new_model` of a STEP protocol.
pub fn step_protocol_new_model() -> Handle<dyn InterfaceInterfaceModel> {
    StepDataStepModel::new().cast()
}

/// The `is_suitable_model(model)` of a STEP protocol.
pub fn step_protocol_is_suitable_model(model: &Handle<dyn InterfaceInterfaceModel>) -> bool {
    model.is_kind(StepDataStepModel::static_type())
}

/// The `unknown_entity` of a STEP protocol.
pub fn step_protocol_unknown_entity() -> Handle<StandardTransient> {
    Handle::new(StepDataUndefinedEntity::new(false)).cast()
}

/// The `is_unknown_entity(ent)` of a STEP protocol.
pub fn step_protocol_is_unknown_entity(ent: &Handle<StandardTransient>) -> bool {
    ent.is_kind(StepDataUndefinedEntity::static_type())
}

/// Implements `InterfaceProtocol` for a [`StepDataProtocolOps`], by the `step_*` members;
/// `impl_step_protocol!(HeaderSectionProtocol);`.
#[macro_export]
macro_rules! impl_step_protocol {
    ($protocol:ty) => {
        impl martensite_tkxsbase::interface::InterfaceProtocol for $protocol {
            fn nb_resources(&self) -> i32 {
                $crate::step_data::StepDataProtocolOps::step_nb_resources(self)
            }
            fn resource(
                &self,
                num: i32,
            ) -> martensite_tkernel::standard::Handle<
                dyn martensite_tkxsbase::interface::InterfaceProtocol,
            > {
                $crate::step_data::StepDataProtocolOps::step_resource(self, num)
            }
            fn case_number(
                &self,
                obj: &martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
            ) -> i32 {
                $crate::step_data::step_protocol_case_number(self, obj)
            }
            fn type_number(&self, atype: &martensite_tkernel::standard::RuntimeType) -> i32 {
                $crate::step_data::StepDataProtocolOps::step_type_number(self, atype)
            }
            fn global_check(
                &self,
                graph: &martensite_tkxsbase::interface::InterfaceGraph,
                ach: &mut martensite_tkernel::standard::Handle<
                    martensite_tkxsbase::interface::InterfaceCheck,
                >,
            ) -> bool {
                $crate::step_data::StepDataProtocolOps::step_global_check(self, graph, ach)
            }
            fn new_model(
                &self,
            ) -> martensite_tkernel::standard::Handle<
                dyn martensite_tkxsbase::interface::InterfaceInterfaceModel,
            > {
                $crate::step_data::step_protocol_new_model()
            }
            fn is_suitable_model(
                &self,
                model: &martensite_tkernel::standard::Handle<
                    dyn martensite_tkxsbase::interface::InterfaceInterfaceModel,
                >,
            ) -> bool {
                $crate::step_data::step_protocol_is_suitable_model(model)
            }
            fn unknown_entity(
                &self,
            ) -> martensite_tkernel::standard::Handle<
                martensite_tkernel::standard::StandardTransient,
            > {
                $crate::step_data::step_protocol_unknown_entity()
            }
            fn is_unknown_entity(
                &self,
                ent: &martensite_tkernel::standard::Handle<
                    martensite_tkernel::standard::StandardTransient,
                >,
            ) -> bool {
                $crate::step_data::step_protocol_is_unknown_entity(ent)
            }
        }
    };
}

/// ```
/// use martensite_tkdestep::step_data::{StepDataProtocol, StepDataProtocolOps};
/// use martensite_tkernel::standard::Handle;
/// assert_eq!(StepDataProtocol::default().schema_name(&Handle::default()), "(DEFAULT)");
/// ```
#[derive(Default)]
pub struct StepDataProtocol {
    transient: StandardTransient,
    tables: StepDataProtocolTables,
}
impl_transient!(
    StepDataProtocol,
    "StepDataProtocolOps",
    dyn StepDataProtocolOps,
    transient,
    interfaces: [dyn InterfaceProtocol, dyn StepDataProtocolOps]
);
crate::impl_step_protocol!(StepDataProtocol);

impl StepDataProtocolOps for StepDataProtocol {
    fn protocol_tables(&self) -> &StepDataProtocolTables {
        &self.tables
    }
}

/// `StepDataFileProtocol`: its resources are the protocols added, each class once.
///
/// ```
/// use martensite_tkdestep::step_data::{StepDataFileProtocol, StepDataProtocol, StepDataProtocolOps};
/// use martensite_tkernel::standard::Handle;
/// let file = StepDataFileProtocol::default();
/// file.add(&Handle::new(StepDataProtocol::default()).cast());
/// file.add(&Handle::new(StepDataProtocol::default()).cast());
/// assert_eq!(file.step_nb_resources(), 1);
/// ```
#[derive(Default)]
pub struct StepDataFileProtocol {
    transient: StandardTransient,
    tables: StepDataProtocolTables,
    thecomps: SharedState<Vec<Handle<dyn StepDataProtocolOps>>>,
}
impl_transient!(
    StepDataFileProtocol,
    "StepDataFileProtocol",
    dyn StepDataProtocolOps,
    transient,
    interfaces: [dyn InterfaceProtocol, dyn StepDataProtocolOps]
);
crate::impl_step_protocol!(StepDataFileProtocol);

impl StepDataFileProtocol {
    /// `add(protocol)`: ignored when null or of a class already added.
    pub fn add(&self, protocol: &Handle<dyn StepDataProtocolOps>) {
        let Some(kind) = protocol.dynamic_type() else {
            return;
        };
        let mut comps = self.thecomps.write();
        if comps.iter().any(|known| known.is_instance(kind)) {
            return;
        }
        comps.push(protocol.clone());
    }
}

impl StepDataProtocolOps for StepDataFileProtocol {
    fn protocol_tables(&self) -> &StepDataProtocolTables {
        &self.tables
    }
    fn step_nb_resources(&self) -> i32 {
        self.thecomps.read().len() as i32
    }
    fn step_resource(&self, num: i32) -> Handle<dyn InterfaceProtocol> {
        let comps = self.thecomps.read();
        usize::try_from(num - 1)
            .ok()
            .and_then(|rank| comps.get(rank))
            .map_or_else(Handle::default, |found| found.cast())
    }
    fn step_type_number(&self, _atype: &RuntimeType) -> i32 {
        0
    }
    fn step_global_check(&self, graph: &InterfaceGraph, ach: &mut Handle<InterfaceCheck>) -> bool {
        let comps = self.thecomps.read().clone();
        let mut result = false;
        for comp in comps {
            result |= comp.global_check(graph, ach);
        }
        result
    }
    fn schema_name(&self, _model: &Handle<dyn InterfaceInterfaceModel>) -> String {
        String::new()
    }
}
