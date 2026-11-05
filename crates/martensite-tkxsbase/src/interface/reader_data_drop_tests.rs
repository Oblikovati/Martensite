//! Entity destructors can observe whether their reader's parameter list was cleared.
use super::{
    InterfaceFileReaderData, InterfaceFileReaderDataBase, InterfaceFileReaderDataParams,
    InterfaceParamList, InterfaceParamType,
};
use martensite_tkernel::{
    impl_transient,
    standard::{Handle, SharedState, StandardTransient},
};
use std::sync::Arc;

struct LifetimeReader {
    transient: StandardTransient,
    base: InterfaceFileReaderDataBase,
}
impl_transient!(LifetimeReader, "LifetimeReader", dyn InterfaceFileReaderData, transient, interfaces: [dyn InterfaceFileReaderData]);
impl InterfaceFileReaderData for LifetimeReader {
    fn reader_data_base(&self) -> &InterfaceFileReaderDataBase {
        &self.base
    }
    fn find_next_record(&self, num: i32) -> i32 {
        if num == 0 { 1 } else { 0 }
    }
}

struct ParameterLengthEntity {
    transient: StandardTransient,
    list: Handle<InterfaceParamList>,
    observations: Arc<SharedState<Vec<i32>>>,
}
impl_transient!(
    ParameterLengthEntity,
    "ParameterLengthEntity",
    StandardTransient,
    transient
);
impl Drop for ParameterLengthEntity {
    fn drop(&mut self) {
        self.observations.write().push(self.list.length());
    }
}

fn populated_reader() -> Handle<dyn InterfaceFileReaderData> {
    let reader = Handle::new(LifetimeReader {
        transient: StandardTransient::default(),
        base: InterfaceFileReaderDataBase::new(1, 2),
    });
    reader
        .add_param(1, b"12", InterfaceParamType::ParamInteger, 0)
        .unwrap();
    reader.init_params(1).unwrap();
    reader.cast()
}

fn length_entity(
    list: &Handle<InterfaceParamList>,
    observations: &Arc<SharedState<Vec<i32>>>,
) -> Handle<StandardTransient> {
    Handle::new(ParameterLengthEntity {
        transient: StandardTransient::default(),
        list: list.clone(),
        observations: observations.clone(),
    })
    .cast()
}

#[test]
fn reader_destroys_bound_entities_before_clearing_parameters() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let reader = populated_reader();
    let list = reader.params(0).unwrap();
    reader
        .bind_entity(1, &length_entity(&list, &observations))
        .unwrap();
    drop(reader);
    assert_eq!(*observations.read(), vec![1]);
    assert_eq!(list.length(), 0);
}

#[test]
fn retained_reader_owner_delays_bound_entity_and_parameter_cleanup() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let reader = populated_reader();
    let list = reader.params(0).unwrap();
    reader
        .bind_entity(1, &length_entity(&list, &observations))
        .unwrap();
    let retained = reader.clone();
    drop(reader);
    assert!(observations.read().is_empty());
    assert_eq!(list.length(), 1);
    drop(retained);
    assert_eq!(*observations.read(), vec![1]);
    assert_eq!(list.length(), 0);
}

#[test]
fn independent_bound_entity_survives_reader_and_observes_cleared_list() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let reader = populated_reader();
    let list = reader.params(0).unwrap();
    let entity = length_entity(&list, &observations);
    reader.bind_entity(1, &entity).unwrap();
    drop(reader);
    assert!(observations.read().is_empty());
    assert_eq!(list.length(), 0);
    drop(entity);
    assert_eq!(*observations.read(), vec![0]);
}
