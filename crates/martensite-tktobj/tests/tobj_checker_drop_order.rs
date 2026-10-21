//! A checker releases its model while its inherited message service remains alive.
use martensite_tkernel::{
    impl_transient,
    message::Messenger,
    standard::{Failure, Handle, SharedState, StandardTransient},
};
use martensite_tktobj::tobj::{TObjCheckModel, TObjModel, TObjModelBase};
use std::sync::{Arc, Weak};

struct MessengerObservingModel {
    transient: StandardTransient,
    base: TObjModelBase,
    messenger: Weak<Messenger>,
    observations: Arc<SharedState<Vec<bool>>>,
}
impl_transient!(MessengerObservingModel, "MessengerObservingModel", dyn TObjModel, transient,
    interfaces: [dyn TObjModel]);
impl TObjModel for MessengerObservingModel {
    fn model_base(&self) -> &TObjModelBase {
        &self.base
    }
    fn new_empty(&self) -> Result<Handle<dyn TObjModel>, Failure> {
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
    TObjCheckModel,
    Handle<dyn TObjModel>,
    Arc<SharedState<Vec<bool>>>,
);

fn checker_drop_fixture() -> CheckerDropFixture {
    let messenger = Arc::new(Messenger::new());
    let observations = Arc::new(SharedState::new(Vec::new()));
    let model: Handle<dyn TObjModel> = Handle::new(MessengerObservingModel {
        transient: StandardTransient::default(),
        base: TObjModelBase::new().unwrap(),
        messenger: Arc::downgrade(&messenger),
        observations: observations.clone(),
    })
    .cast();
    let checker = TObjCheckModel::new(&model);
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
