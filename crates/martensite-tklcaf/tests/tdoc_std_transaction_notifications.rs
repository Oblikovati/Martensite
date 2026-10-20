//! Transaction notifications follow the state changes in TDocStd_Document.cxx:
//! successful commits notify, empty commits do not, and abort notifies even when idle.
use martensite_tkcdf::{cdf::CdfApplication, cdm::CdmApplication};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, SharedState, StandardTransient},
    tcollection::ExtendedString,
};
use martensite_tklcaf::{
    tdata_std::TDataStdInteger,
    tdoc_std::{TDocStdApplication, TDocStdApplicationBase, TDocStdDocument},
    tdoc_std_application_cdf_members, tdoc_std_application_cdm_members,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NotificationKind {
    Open,
    Commit,
    Abort,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NotificationSnapshot {
    kind: NotificationKind,
    document_address: usize,
    command_open: bool,
    integer: Option<i32>,
}

#[derive(Default)]
struct FakeTransactionApplication {
    transient: StandardTransient,
    base: TDocStdApplicationBase,
    notifications: SharedState<Vec<NotificationSnapshot>>,
}
impl_transient!(FakeTransactionApplication, "FakeTransactionApplication", dyn CdfApplication, transient,
    interfaces: [dyn CdmApplication, dyn CdfApplication, dyn TDocStdApplication]);
impl CdmApplication for FakeTransactionApplication {
    tdoc_std_application_cdm_members!();
}
impl CdfApplication for FakeTransactionApplication {
    tdoc_std_application_cdf_members!();
}
impl TDocStdApplication for FakeTransactionApplication {
    fn tdoc_std_application_base(&self) -> &TDocStdApplicationBase {
        &self.base
    }
    fn on_open_transaction(&self, document: &Handle<TDocStdDocument>) -> Result<(), Failure> {
        self.record(NotificationKind::Open, document)
    }
    fn on_commit_transaction(&self, document: &Handle<TDocStdDocument>) -> Result<(), Failure> {
        self.record(NotificationKind::Commit, document)
    }
    fn on_abort_transaction(&self, document: &Handle<TDocStdDocument>) -> Result<(), Failure> {
        self.record(NotificationKind::Abort, document)
    }
}
impl FakeTransactionApplication {
    fn record(
        &self,
        kind: NotificationKind,
        document: &Handle<TDocStdDocument>,
    ) -> Result<(), Failure> {
        let mut integer = Handle::<TDataStdInteger>::default();
        let found = document
            .main()?
            .find_attribute_as(&TDataStdInteger::get_id(), &mut integer)?;
        self.notifications.write().push(NotificationSnapshot {
            kind,
            document_address: document.object_address().addr(),
            command_open: document.has_open_command(),
            integer: found.then(|| TDataStdInteger::get(&integer)),
        });
        Ok(())
    }
}

fn application_document() -> (Handle<FakeTransactionApplication>, Handle<TDocStdDocument>) {
    let application = Handle::new(FakeTransactionApplication::default());
    application
        .application_base()
        .bind_self(&application.cast());
    let mut document = Handle::default();
    application
        .new_tdoc_std_document(&ExtendedString::new("BinOcaf"), &mut document)
        .unwrap();
    document.set_undo_limit(10).unwrap();
    (application, document)
}

fn assert_notifications(
    application: &Handle<FakeTransactionApplication>,
    document: &Handle<TDocStdDocument>,
    expected: &[(NotificationKind, bool, Option<i32>)],
) {
    let snapshots = application.notifications.read();
    assert_eq!(snapshots.len(), expected.len());
    for (snapshot, &(kind, command_open, integer)) in snapshots.iter().zip(expected) {
        assert_eq!(snapshot.document_address, document.object_address().addr());
        assert_eq!(
            (snapshot.kind, snapshot.command_open, snapshot.integer),
            (kind, command_open, integer)
        );
    }
}

const COMMIT_ROLLBACK: &[(NotificationKind, bool, Option<i32>)] = &[
    (NotificationKind::Open, true, None),
    (NotificationKind::Commit, false, Some(1)),
    (NotificationKind::Open, true, Some(1)),
    (NotificationKind::Abort, false, Some(1)),
];
const EMPTY_AND_IDLE: &[(NotificationKind, bool, Option<i32>)] = &[
    (NotificationKind::Open, true, None),
    (NotificationKind::Abort, false, None),
    (NotificationKind::Abort, false, None),
];
const CLOSED: &[(NotificationKind, bool, Option<i32>)] = &[(NotificationKind::Abort, false, None)];
const ZERO_UNDO: &[(NotificationKind, bool, Option<i32>)] = &[
    (NotificationKind::Open, false, None),
    (NotificationKind::Abort, false, None),
];

#[test]
fn transaction_callbacks_observe_published_commit_and_rollback_state() {
    let (application, document) = application_document();
    document.open_command().unwrap();
    TDataStdInteger::set(&document.main().unwrap(), 1).unwrap();
    assert!(document.commit_command().unwrap());
    document.open_command().unwrap();
    TDataStdInteger::set(&document.main().unwrap(), 2).unwrap();
    document.abort_command().unwrap();
    assert_notifications(&application, &document, COMMIT_ROLLBACK);
}

#[test]
fn empty_commit_is_silent_but_each_idle_abort_notifies() {
    let (application, document) = application_document();
    document.open_command().unwrap();
    assert!(!document.commit_command().unwrap());
    document.abort_command().unwrap();
    document.abort_command().unwrap();
    assert_notifications(&application, &document, EMPTY_AND_IDLE);
}

#[test]
fn closed_document_transactions_do_not_notify_the_former_application() {
    let (application, document) = application_document();
    application.close_tdoc_std_document(&document).unwrap();
    assert_notifications(&application, &document, CLOSED);
    document.open_command().unwrap();
    TDataStdInteger::set(&document.main().unwrap(), 3).unwrap();
    assert!(document.commit_command().unwrap());
    document.abort_command().unwrap();
    assert_notifications(&application, &document, CLOSED);
}

#[test]
fn zero_undo_limit_still_reports_open_and_abort_without_open_command() {
    let (application, document) = application_document();
    document.set_undo_limit(0).unwrap();
    document.open_command().unwrap();
    assert!(!document.commit_command().unwrap());
    document.abort_command().unwrap();
    assert_notifications(&application, &document, ZERO_UNDO);
}
