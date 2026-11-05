//! Public reader-tool ownership releases the model before its reader data.
use super::{
    InterfaceCheck, InterfaceFileReaderData, InterfaceFileReaderDataBase, InterfaceFileReaderTool,
    InterfaceFileReaderToolBase, InterfaceFileReaderToolOps, InterfaceInterfaceModel,
    test_norm::fake_model,
};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, SharedState, StandardTransient, WeakHandle},
};
use std::sync::Arc;

struct ModelObservingReader {
    transient: StandardTransient,
    base: InterfaceFileReaderDataBase,
    model: WeakHandle<dyn InterfaceInterfaceModel>,
    observations: Arc<SharedState<Vec<bool>>>,
}
impl_transient!(ModelObservingReader, "ModelObservingReader", dyn InterfaceFileReaderData, transient, interfaces: [dyn InterfaceFileReaderData]);
impl InterfaceFileReaderData for ModelObservingReader {
    fn reader_data_base(&self) -> &InterfaceFileReaderDataBase {
        &self.base
    }
    fn find_next_record(&self, _: i32) -> i32 {
        0
    }
}
impl Drop for ModelObservingReader {
    fn drop(&mut self) {
        self.observations
            .write()
            .push(!self.model.upgrade().is_null());
    }
}

#[derive(Default)]
struct LifetimeReaderTool {
    base: InterfaceFileReaderToolBase,
}
impl InterfaceFileReaderTool for LifetimeReaderTool {
    fn reader_tool_base(&self) -> &InterfaceFileReaderToolBase {
        &self.base
    }
    fn recognize(
        &self,
        _: i32,
        _: &mut Handle<InterfaceCheck>,
        _: &mut Handle<StandardTransient>,
    ) -> Result<bool, Failure> {
        Ok(false)
    }
    fn begin_read(&self, _: &Handle<dyn InterfaceInterfaceModel>) -> Result<(), Failure> {
        Ok(())
    }
    fn analyse_record(
        &self,
        _: i32,
        _: &Handle<StandardTransient>,
        _: &mut Handle<InterfaceCheck>,
    ) -> Result<bool, Failure> {
        Ok(false)
    }
}

fn observed_reader(
    model: &Handle<dyn InterfaceInterfaceModel>,
    observations: &Arc<SharedState<Vec<bool>>>,
) -> Handle<dyn InterfaceFileReaderData> {
    Handle::new(ModelObservingReader {
        transient: StandardTransient::default(),
        base: InterfaceFileReaderDataBase::new(0, 1),
        model: model.downgrade(),
        observations: observations.clone(),
    })
    .cast()
}

#[test]
fn public_tool_drop_releases_model_before_reader() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let model = fake_model();
    let tool: Box<dyn InterfaceFileReaderTool> = Box::<LifetimeReaderTool>::default();
    tool.set_data(&observed_reader(&model, &observations), &Handle::default());
    tool.set_model(&model);
    drop(model);
    drop(tool);
    assert_eq!(*observations.read(), vec![false]);
}

#[test]
fn externally_retained_reader_delays_its_destructor_beyond_tool_model() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let model = fake_model();
    let reader = observed_reader(&model, &observations);
    let tool: Box<dyn InterfaceFileReaderTool> = Box::<LifetimeReaderTool>::default();
    tool.set_data(&reader, &Handle::default());
    tool.set_model(&model);
    drop(model);
    drop(tool);
    assert!(observations.read().is_empty());
    drop(reader);
    assert_eq!(*observations.read(), vec![false]);
}

#[test]
fn external_model_owner_survives_tool_destruction() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let model = fake_model();
    let tool: Box<dyn InterfaceFileReaderTool> = Box::<LifetimeReaderTool>::default();
    tool.set_data(&observed_reader(&model, &observations), &Handle::default());
    tool.set_model(&model);
    drop(tool);
    assert_eq!(*observations.read(), vec![true]);
    assert!(!model.is_null());
}
