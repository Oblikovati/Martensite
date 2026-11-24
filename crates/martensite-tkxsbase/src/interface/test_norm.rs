//! A small fake norm for the unit tests: entities that share other entities, their
//! protocol, general module and model.
use super::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceGeneralLib,
    InterfaceGeneralModule, InterfaceInterfaceModel, InterfaceModelBase, InterfaceModelEntities,
    InterfaceProtocol, InterfaceShareTool,
};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, RuntimeType, RuntimeTyped, SharedState, Transient},
    tcollection::HAsciiString,
};
use std::io::{self, Write};
use std::sync::OnceLock;

/// An entity with a name and the entities it shares.
#[derive(Default)]
pub(crate) struct FakeEntity {
    pub(crate) name: SharedState<String>,
    pub(crate) shared: SharedState<Vec<Handle<dyn Transient>>>,
}
impl_transient!(FakeEntity, "Fake_Entity", dyn Transient);

/// A new entity sharing `shared`.
pub(crate) fn fake_entity(name: &str, shared: &[Handle<dyn Transient>]) -> Handle<dyn Transient> {
    let entity = FakeEntity::default();
    *entity.name.write() = name.to_owned();
    *entity.shared.write() = shared.to_vec();
    Handle::new(entity).into_upcast()
}

/// The protocol of the fake norm: `FakeEntity` is case 1.
#[derive(Default)]
pub(crate) struct FakeProtocol {}
impl_transient!(FakeProtocol, "Fake_Protocol", dyn InterfaceProtocol, interfaces: [dyn InterfaceProtocol]);

impl InterfaceProtocol for FakeProtocol {
    fn nb_resources(&self) -> i32 {
        0
    }
    fn resource(&self, _num: i32) -> Handle<dyn InterfaceProtocol> {
        Handle::default()
    }
    fn type_number(&self, atype: &RuntimeType) -> i32 {
        i32::from(std::ptr::eq(atype, FakeEntity::static_type()))
    }
    fn new_model(&self) -> Handle<dyn InterfaceInterfaceModel> {
        fake_model()
    }
    fn is_suitable_model(&self, model: &Handle<dyn InterfaceInterfaceModel>) -> bool {
        model.cast::<FakeModel>().is_some()
    }
    fn unknown_entity(&self) -> Handle<dyn Transient> {
        fake_entity("unknown", &[])
    }
    fn is_unknown_entity(&self, ent: &Handle<dyn Transient>) -> bool {
        ent.cast::<FakeEntity>()
            .get()
            .is_some_and(|e| *e.name.read() == "unknown")
    }
}

/// The general module of the fake norm.
#[derive(Default)]
pub(crate) struct FakeModule {}
impl_transient!(FakeModule, "Fake_GeneralModule", dyn InterfaceGeneralModule, interfaces: [dyn InterfaceGeneralModule]);

impl InterfaceGeneralModule for FakeModule {
    fn fill_shared_case(
        &self,
        _cn: i32,
        ent: &Handle<dyn Transient>,
        iter: &mut InterfaceEntityIterator,
    ) {
        let shared = ent
            .cast::<FakeEntity>()
            .get()
            .map(|e| e.shared.read().clone())
            .unwrap_or_default();
        shared.iter().for_each(|s| self.share(iter, s));
    }
    fn check_case(
        &self,
        _cn: i32,
        ent: &Handle<dyn Transient>,
        _shares: &InterfaceShareTool,
        ach: &mut Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let fake = ent.cast::<FakeEntity>();
        let Some(live) = fake.get() else {
            return Ok(());
        };
        if *live.name.read() == "raise" {
            return Err(Failure::new("check raised"));
        }
        if live.shared.read().is_empty() {
            ach.add_warning_text("no shared entity", "");
        }
        Ok(())
    }
    fn can_copy(&self, _cn: i32, _ent: &Handle<dyn Transient>) -> bool {
        true
    }
    fn new_void(&self, _cn: i32, entto: &mut Handle<dyn Transient>) -> bool {
        *entto = fake_entity("", &[]);
        true
    }
    fn copy_case(
        &self,
        _cn: i32,
        entfrom: &Handle<dyn Transient>,
        entto: &Handle<dyn Transient>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let (from, to) = (entfrom.cast::<FakeEntity>(), entto.cast::<FakeEntity>());
        let (Some(from), Some(to)) = (from.get(), to.get()) else {
            return Ok(());
        };
        *to.name.write() = format!("copy of {}", from.name.read());
        let shared = from.shared.read().clone();
        let mut copies = Vec::new();
        for entity in &shared {
            copies.push(tc.transferred(entity)?);
        }
        *to.shared.write() = copies;
        Ok(())
    }
    fn name(
        &self,
        _cn: i32,
        ent: &Handle<dyn Transient>,
        _shares: &InterfaceShareTool,
    ) -> Handle<HAsciiString> {
        let name = ent
            .cast::<FakeEntity>()
            .get()
            .map(|e| e.name.read().clone())
            .unwrap_or_default();
        Handle::new(HAsciiString::new(name.as_str()))
    }
}

/// The model of the fake norm, with a header text.
#[derive(Default)]
pub(crate) struct FakeModel {
    base: InterfaceModelBase,
    pub(crate) header: SharedState<String>,
}
impl_transient!(FakeModel, "Fake_Model", dyn InterfaceInterfaceModel, interfaces: [dyn InterfaceInterfaceModel]);

impl InterfaceInterfaceModel for FakeModel {
    fn model_base(&self) -> &InterfaceModelBase {
        &self.base
    }
    fn clear_labels(&self) {}
    fn clear_header(&self) {
        self.header.write().clear();
    }
    fn get_from_another(&self, other: &Handle<dyn InterfaceInterfaceModel>) -> Result<(), Failure> {
        let header = other
            .cast::<FakeModel>()
            .get()
            .map(|m| m.header.read().clone())
            .unwrap_or_default();
        *self.header.write() = header;
        Ok(())
    }
    fn new_empty_model(&self) -> Handle<dyn InterfaceInterfaceModel> {
        fake_model()
    }
    fn dump_header(&self, s: &mut dyn Write, _level: i32) -> io::Result<()> {
        s.write_all(self.header.read().as_bytes())
    }
    fn print_label(&self, ent: &Handle<dyn Transient>, s: &mut dyn Write) -> io::Result<()> {
        write!(s, "#{}", self.number(ent))
    }
    fn string_label(&self, ent: &Handle<dyn Transient>) -> Handle<HAsciiString> {
        let name = ent
            .cast::<FakeEntity>()
            .get()
            .map(|e| e.name.read().clone())
            .unwrap_or_default();
        Handle::new(HAsciiString::new(name.as_str()))
    }
}

/// The protocol of the fake norm, registered once with its module in the general library.
pub(crate) fn fake_protocol() -> Handle<dyn InterfaceProtocol> {
    static PROTOCOL: OnceLock<Handle<dyn InterfaceProtocol>> = OnceLock::new();
    PROTOCOL
        .get_or_init(|| {
            let protocol: Handle<dyn InterfaceProtocol> =
                Handle::new(FakeProtocol::default()).into_upcast();
            let module: Handle<dyn InterfaceGeneralModule> =
                Handle::new(FakeModule::default()).into_upcast();
            InterfaceGeneralLib::set_global(&module, &protocol);
            protocol
        })
        .clone()
}

/// A new empty fake model with the fake protocol, bound to its handle.
pub(crate) fn fake_model() -> Handle<dyn InterfaceInterfaceModel> {
    let model: Handle<dyn InterfaceInterfaceModel> =
        Handle::new(FakeModel::default()).into_upcast();
    model.model_base().bind_self(&model);
    model.set_protocol(&fake_protocol());
    model
}

/// A fake model of four entities: d shares c, c shares a and b.
pub(crate) fn fake_chain() -> (
    Handle<dyn InterfaceInterfaceModel>,
    Vec<Handle<dyn Transient>>,
) {
    let a = fake_entity("a", &[]);
    let b = fake_entity("b", &[]);
    let c = fake_entity("c", &[a.clone(), b.clone()]);
    let d = fake_entity("d", std::slice::from_ref(&c));
    let model = fake_model();
    for entity in [&a, &b, &c, &d] {
        // Adding a live entity cannot fail.
        let _ = model.add_entity(entity);
    }
    (model, vec![a, b, c, d])
}
