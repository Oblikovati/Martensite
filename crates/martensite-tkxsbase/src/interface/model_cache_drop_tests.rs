//! Entity destruction observes the already-cleared general-tool module cache.
use super::{
    InterfaceGTool, InterfaceGeneralLib, InterfaceGeneralModule, InterfaceInterfaceModel,
    InterfaceModelEntities, InterfaceProtocol,
    general_lib::InterfaceGeneralLibKind,
    test_norm::{FakeModule, FakeProtocol, fake_entity, fake_model},
};
use crate::lib_ctl::LibCtlRegistry;
use martensite_tkernel::{
    impl_transient,
    standard::{Handle, SharedState, StandardTransient},
};
use std::sync::Arc;

type ModuleOwnerObservations = Arc<SharedState<Vec<i32>>>;
struct ModuleObservingModelEntity {
    transient: StandardTransient,
    module: Handle<dyn InterfaceGeneralModule>,
    observations: ModuleOwnerObservations,
}
impl_transient!(
    ModuleObservingModelEntity,
    "ModuleObservingModelEntity",
    StandardTransient,
    transient
);
impl Drop for ModuleObservingModelEntity {
    fn drop(&mut self) {
        self.observations
            .write()
            .push(self.module.transient_base().ref_count());
    }
}

fn privately_cached_module() -> (Handle<InterfaceGTool>, Handle<dyn InterfaceGeneralModule>) {
    let registry = LibCtlRegistry::<InterfaceGeneralLibKind>::new();
    let module: Handle<dyn InterfaceGeneralModule> = Handle::new(FakeModule::default()).cast();
    let protocol: Handle<dyn InterfaceProtocol> = Handle::new(FakeProtocol::default()).cast();
    registry.set_global(&module, &protocol);
    let gtool = Handle::new(InterfaceGTool::new());
    gtool.change_lib(|lib| *lib = InterfaceGeneralLib::new_in(&registry, &protocol));
    assert_eq!(
        gtool
            .select(&fake_entity("cache key", &[]), false)
            .unwrap()
            .1,
        1
    );
    (gtool, module)
}

fn model_with_observing_entity(
    observations: &ModuleOwnerObservations,
) -> (
    Handle<dyn InterfaceInterfaceModel>,
    Handle<dyn InterfaceGeneralModule>,
) {
    let (gtool, module) = privately_cached_module();
    let model = fake_model();
    model.set_gtool(&gtool);
    let entity = Handle::new(ModuleObservingModelEntity {
        transient: StandardTransient::default(),
        module: module.clone(),
        observations: observations.clone(),
    })
    .cast();
    model.add_entity(&entity).unwrap();
    // Four owners: this fixture, library node, type cache, and observed entity.
    assert_eq!(module.transient_base().ref_count(), 4);
    (model, module)
}

#[test]
fn clear_entities_releases_cached_module_before_observed_entity() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let (model, module) = model_with_observing_entity(&observations);
    model.clear_entities();
    assert_eq!(observations.snapshot(), vec![3]);
    assert_eq!(model.nb_entities(), 0);
    assert_eq!(module.transient_base().ref_count(), 2);
}

#[test]
fn model_destruction_releases_cached_module_before_observed_entity() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let (model, module) = model_with_observing_entity(&observations);
    drop(model);
    assert_eq!(observations.snapshot(), vec![3]);
    assert_eq!(module.transient_base().ref_count(), 1);
}
