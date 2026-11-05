//! Retained public reader state observes transfer-binder destruction after session cleanup.
use super::{XSControlTransferReader, XSControlWorkSession};
use crate::{
    interface::test_norm::fake_model,
    transfer::{
        TransferBinder, TransferBinderBase, TransferTransientMapper, TransferTransientProcess,
    },
};
use martensite_tkernel::{
    impl_transient,
    standard::{Handle, RuntimeType, RuntimeTyped, SharedState, StandardTransient},
};
use std::sync::Arc;
struct ReaderObservingBinder {
    transient: StandardTransient,
    base: TransferBinderBase,
    reader: Handle<XSControlTransferReader>,
    observations: Arc<SharedState<Vec<(bool, bool)>>>,
}
impl_transient!(ReaderObservingBinder,"ReaderObservingBinder",dyn TransferBinder,transient,interfaces:[dyn TransferBinder]);
impl TransferBinder for ReaderObservingBinder {
    fn binder_base(&self) -> &TransferBinderBase {
        &self.base
    }
    fn result_type(&self) -> &'static RuntimeType {
        StandardTransient::static_type()
    }
    fn result_type_name(&self) -> String {
        "Standard_Transient".into()
    }
}
impl Drop for ReaderObservingBinder {
    fn drop(&mut self) {
        self.observations.write().push((
            self.reader.model().is_null(),
            self.reader.transient_process().is_null(),
        ));
    }
}
fn observing_binder(
    reader: &Handle<XSControlTransferReader>,
    observations: &Arc<SharedState<Vec<(bool, bool)>>>,
) -> Handle<dyn TransferBinder> {
    Handle::new(ReaderObservingBinder {
        transient: StandardTransient::default(),
        base: TransferBinderBase::default(),
        reader: reader.clone(),
        observations: observations.clone(),
    })
    .cast()
}
fn populated_session() -> Handle<XSControlWorkSession> {
    let session = XSControlWorkSession::new();
    let reader = session.transfer_reader();
    reader.set_model(&fake_model());
    reader.set_transient_process(&TransferTransientProcess::new(1));
    session
}
#[test]
fn session_keeps_head_binders_until_reader_model_and_process_are_cleared() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let session = populated_session();
    let reader = session.transfer_reader();
    let binder = observing_binder(&reader, &observations);
    let mapper = Handle::new(TransferTransientMapper::new(&Handle::new(
        StandardTransient::default(),
    )))
    .cast();
    let process = session.transfer_writer().finder_process();
    process.bind(&mapper, &binder).unwrap();
    drop(binder);
    drop(mapper);
    drop(session);
    assert_eq!(*observations.read(), vec![(true, true)]);
    assert_eq!(process.nb_mapped(), 0);
    assert!(reader.model().is_null() && reader.transient_process().is_null());
}

#[test]
fn multiple_head_chains_are_retained_through_session_cleanup() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let session = populated_session();
    let reader = session.transfer_reader();
    for _ in 0..2 {
        let head = observing_binder(&reader, &observations);
        let tail = observing_binder(&reader, &observations);
        head.add_result(&tail);
        let mapper = Handle::new(TransferTransientMapper::new(&Handle::new(
            StandardTransient::default(),
        )))
        .cast();
        session
            .transfer_writer()
            .finder_process()
            .bind(&mapper, &head)
            .unwrap();
    }
    drop(session);
    assert_eq!(*observations.read(), vec![(true, true); 4]);
}
#[test]
fn an_externally_retained_head_is_not_destroyed_by_session_cleanup() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let session = populated_session();
    let reader = session.transfer_reader();
    let head = observing_binder(&reader, &observations);
    let mapper = Handle::new(TransferTransientMapper::new(&Handle::new(
        StandardTransient::default(),
    )))
    .cast();
    session
        .transfer_writer()
        .finder_process()
        .bind(&mapper, &head)
        .unwrap();
    drop(session);
    assert!(observations.read().is_empty());
    drop(head);
    assert_eq!(*observations.read(), vec![(true, true)]);
}
