//! An owned model destructor observes the graph's sharing-owner release order.
use super::{
    InterfaceGraph, InterfaceInterfaceModel, InterfaceModelBase, InterfaceSharingTableHandle,
};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, SharedState, StandardTransient, Transient},
    tcollection::HAsciiString,
};
use std::{
    io::{self, Write},
    sync::Arc,
};

#[derive(Default)]
struct DeletionCountModel {
    transient: StandardTransient,
    base: InterfaceModelBase,
    held_table: SharedState<Option<InterfaceSharingTableHandle>>,
    observations: Arc<SharedState<Vec<i32>>>,
}
impl_transient!(DeletionCountModel, "DeletionCountModel", dyn InterfaceInterfaceModel, transient,
    interfaces: [dyn InterfaceInterfaceModel]);

impl Drop for DeletionCountModel {
    fn drop(&mut self) {
        if let Some(table) = self.held_table.read().as_ref() {
            self.observations
                .write()
                .push(table.get().unwrap().transient_base().ref_count());
        }
    }
}

impl InterfaceInterfaceModel for DeletionCountModel {
    fn model_base(&self) -> &InterfaceModelBase {
        &self.base
    }
    fn clear_labels(&self) {}
    fn clear_header(&self) {}
    fn get_from_another(&self, _: &Handle<dyn InterfaceInterfaceModel>) -> Result<(), Failure> {
        Ok(())
    }
    fn new_empty_model(&self) -> Handle<dyn InterfaceInterfaceModel> {
        Handle::default()
    }
    fn dump_header(&self, _: &mut dyn Write, _: i32) -> io::Result<()> {
        Ok(())
    }
    fn print_label(&self, _: &Handle<StandardTransient>, _: &mut dyn Write) -> io::Result<()> {
        Ok(())
    }
    fn string_label(&self, _: &Handle<StandardTransient>) -> Handle<HAsciiString> {
        Handle::default()
    }
}

fn recorded_model(observations: Arc<SharedState<Vec<i32>>>) -> Handle<DeletionCountModel> {
    Handle::new(DeletionCountModel {
        transient: StandardTransient::default(),
        base: InterfaceModelBase::default(),
        held_table: SharedState::new(None),
        observations,
    })
}

#[test]
fn graph_releases_its_table_owner_before_destroying_its_model() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let model = recorded_model(observations.clone());
    let graph = InterfaceGraph::new(&model.cast(), false).unwrap();
    *model.held_table.write() = graph.sharing_table_handle().cloned();
    drop(model);
    drop(graph);
    assert_eq!(observations.snapshot(), vec![1]);
}

#[test]
fn assignment_releases_the_old_model_before_replacing_its_old_table() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let model = recorded_model(observations.clone());
    let mut target = InterfaceGraph::new(&model.cast(), false).unwrap();
    *model.held_table.write() = target.sharing_table_handle().cloned();
    drop(model);
    let source_model = Handle::new(DeletionCountModel::default()).cast();
    let source = InterfaceGraph::new(&source_model, false).unwrap();
    target.assign_from(&source).unwrap();
    assert_eq!(observations.snapshot(), vec![2]);
}
