//! A check entity's destructor observes whether the iterator still owns its model.
use super::{
    InterfaceCheck, InterfaceCheckIterator, InterfaceInterfaceModel, test_norm::fake_model,
};
use martensite_tkernel::{
    impl_transient,
    standard::{Handle, SharedState, StandardTransient, WeakHandle},
};
use std::sync::Arc;

struct CheckEntityLifetime {
    transient: StandardTransient,
    model: WeakHandle<dyn InterfaceInterfaceModel>,
    observations: Arc<SharedState<Vec<bool>>>,
}
impl_transient!(
    CheckEntityLifetime,
    "CheckEntityLifetime",
    StandardTransient,
    transient
);

impl Drop for CheckEntityLifetime {
    fn drop(&mut self) {
        self.observations
            .write()
            .push(!self.model.upgrade().is_null());
    }
}

fn recorded_check_iterator(
    observations: &Arc<SharedState<Vec<bool>>>,
) -> (InterfaceCheckIterator, Handle<dyn InterfaceInterfaceModel>) {
    let model = fake_model();
    let entity: Handle<StandardTransient> = Handle::new(CheckEntityLifetime {
        transient: StandardTransient::default(),
        model: model.downgrade(),
        observations: observations.clone(),
    })
    .cast();
    let check = Handle::new(InterfaceCheck::with_entity(&entity));
    check.add_warning_text("owned warning", "");
    let mut iterator = InterfaceCheckIterator::new();
    iterator.set_model(&model);
    iterator.add(&check, 1);
    (iterator, model)
}

#[test]
fn last_iterator_releases_model_before_its_check_entities() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let (iterator, model) = recorded_check_iterator(&observations);
    drop(model);
    drop(iterator);
    assert_eq!(*observations.read(), vec![false]);
}

#[test]
fn iterator_copy_retains_shared_checks_and_its_own_model_owner() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let (iterator, model) = recorded_check_iterator(&observations);
    let retained = iterator.clone();
    drop(model);
    drop(iterator);
    assert!(observations.read().is_empty());
    assert!(retained.check(1).has_warnings());
    drop(retained);
    assert_eq!(*observations.read(), vec![false]);
}

#[test]
fn independent_model_owner_survives_iterator_cleanup() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let (iterator, model) = recorded_check_iterator(&observations);
    drop(iterator);
    assert_eq!(*observations.read(), vec![true]);
    assert!(!model.is_null());
}
