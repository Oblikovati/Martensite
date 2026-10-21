use super::{
    CaseFailure, CaseRunner,
    artifacts::FilesystemReceipts,
    fake_controls::{ChildScenario, NamedFakeChild, OwnedArtifactRoot},
    launch::{ChildLauncher, ChildObservation, ChildRequest},
    lifecycle::{ChildLifecycle, ChildWorkload},
    prepared_control_support::{PreparedOwner, spec},
    runner::RunEnvironment,
};
use std::{error::Error, fs, io};

#[derive(Clone, Copy)]
enum PreparedRecord {
    Bound,
    Missing,
    Wrong,
}

struct NamedPreparedChild(PreparedRecord);

impl ChildLauncher for NamedPreparedChild {
    fn launch(&self, request: &ChildRequest<'_>) -> Result<ChildObservation, io::Error> {
        let output = NamedFakeChild::new(ChildScenario::Success).launch(request)?;
        match self.0 {
            PreparedRecord::Missing => return Ok(output),
            PreparedRecord::Bound => fs::write(
                request.context.directory.join("prepared.txt"),
                &request.context.request,
            )?,
            PreparedRecord::Wrong => fs::write(
                request.context.directory.join("prepared.txt"),
                b"wrong lifecycle",
            )?,
        }
        Ok(output)
    }
}

fn forbidden_parent_preparation() -> Result<PreparedOwner, Box<dyn Error>> {
    panic!("parent must not prepare child state")
}

fn forbidden_parent_body(_: PreparedOwner) -> Result<(), Box<dyn Error>> {
    panic!("parent must not execute child body")
}

fn run_parent(
    root: &OwnedArtifactRoot,
    record: PreparedRecord,
) -> Result<super::CaseReceipt, super::CaseRunError> {
    let workload = ChildWorkload {
        prepare: forbidden_parent_preparation,
        body: forbidden_parent_body,
        lifecycle: ChildLifecycle::Prepared,
    };
    CaseRunner::new(root.path(), None).run_workload(
        spec(),
        workload,
        &RunEnvironment::default(),
        &NamedPreparedChild(record),
        &FilesystemReceipts,
    )
}

#[test]
fn parent_never_executes_callbacks_and_records_prepared_lifecycle() {
    let root = OwnedArtifactRoot::new();
    let receipt = run_parent(&root, PreparedRecord::Bound).unwrap();
    let metadata = fs::read_to_string(receipt.directory.join("receipt.json")).unwrap();
    assert!(
        metadata.contains("\"declared_child_lifecycle\":\"entry,prepare,policy,body,complete\"")
    );
}

#[test]
fn successful_exit_without_bound_prepared_record_is_rejected() {
    let root = OwnedArtifactRoot::new();
    let error = run_parent(&root, PreparedRecord::Missing).unwrap_err();
    assert!(matches!(*error.failure, CaseFailure::Io { .. }));
}

#[test]
fn successful_exit_with_wrong_prepared_record_is_rejected() {
    let root = OwnedArtifactRoot::new();
    let error = run_parent(&root, PreparedRecord::Wrong).unwrap_err();
    assert!(matches!(*error.failure, CaseFailure::Protocol { .. }));
}
