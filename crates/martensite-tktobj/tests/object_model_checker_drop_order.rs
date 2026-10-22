//! A checker releases its model while its inherited message service remains alive.
use martensite_tkernel::{
    impl_transient,
    message::Messenger,
    standard::{Failure, Handle, SharedState, StandardTransient},
};
use martensite_tktobj::object_model::{DocumentModel, DocumentModelBase, ModelIntegrityCheck};
use std::sync::{Arc, Weak};

struct MessengerObservingModel {
    transient: StandardTransient,
    base: DocumentModelBase,
    messenger: Weak<Messenger>,
    observations: Arc<SharedState<Vec<bool>>>,
}
impl_transient!(MessengerObservingModel, "MessengerObservingModel", dyn DocumentModel, transient,
    interfaces: [dyn DocumentModel]);
impl DocumentModel for MessengerObservingModel {
    fn model_base(&self) -> &DocumentModelBase {
        &self.base
    }
    fn new_empty(&self) -> Result<Handle<dyn DocumentModel>, Failure> {
        Ok(Handle::default())
    }
}
impl Drop for MessengerObservingModel {
    fn drop(&mut self) {
        self.observations
            .write()
            .push(self.messenger.upgrade().is_some());
    }
}

type CheckerDropFixture = (
    ModelIntegrityCheck,
    Handle<dyn DocumentModel>,
    Arc<SharedState<Vec<bool>>>,
);

fn checker_drop_fixture() -> CheckerDropFixture {
    let messenger = Arc::new(Messenger::new());
    let observations = Arc::new(SharedState::new(Vec::new()));
    let model: Handle<dyn DocumentModel> = Handle::new(MessengerObservingModel {
        transient: StandardTransient::default(),
        base: DocumentModelBase::new().unwrap(),
        messenger: Arc::downgrade(&messenger),
        observations: observations.clone(),
    })
    .cast();
    let checker = ModelIntegrityCheck::new(&model);
    checker.algorithm().set_messenger(Some(messenger.clone()));
    drop(messenger);
    (checker, model, observations)
}

#[test]
fn checker_releases_its_model_before_its_inherited_message_service() {
    let (checker, model, observations) = checker_drop_fixture();
    drop(model);
    assert!(observations.read().is_empty());
    drop(checker);
    assert_eq!(*observations.read(), vec![true]);
}

#[test]
fn externally_owned_model_outlives_checker_and_its_message_service() {
    let (checker, model, observations) = checker_drop_fixture();
    drop(checker);
    assert!(observations.read().is_empty());
    drop(model);
    assert_eq!(*observations.read(), vec![false]);
}
