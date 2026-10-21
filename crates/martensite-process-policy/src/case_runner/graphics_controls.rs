use super::{
    CaseRunner, CaseSpec, FactorProfile,
    artifacts::FilesystemReceipts,
    fake_controls::{ChildScenario, NamedFakeChild, OwnedArtifactRoot},
    launch::{ChildLauncher, ChildObservation, ChildRequest},
    runner::RunEnvironment,
};
use std::{io, sync::Mutex};

struct HeldMutexChild<'a> {
    graphics_mutex: &'a Mutex<()>,
    inner: NamedFakeChild,
}

impl ChildLauncher for HeldMutexChild<'_> {
    fn launch(&self, request: &ChildRequest<'_>) -> Result<ChildObservation, io::Error> {
        assert!(
            self.graphics_mutex.try_lock().is_err(),
            "caller mutex must cover launch/wait"
        );
        self.inner.launch(request)
    }
}

fn run_under_mutex(root: &OwnedArtifactRoot, mutex: &Mutex<()>, child: &HeldMutexChild<'_>) {
    let spec = CaseSpec::new("controls::graphics", 60, FactorProfile::RequireUnset);
    CaseRunner::new(root.path(), Some(mutex))
        .run_with(
            spec,
            || panic!("parent fixture"),
            &RunEnvironment::default(),
            child,
            &FilesystemReceipts,
        )
        .unwrap();
}

#[test]
fn caller_graphics_mutex_covers_child_launch_and_is_released_afterward() {
    let root = OwnedArtifactRoot::new();
    let mutex = Mutex::new(());
    let child = HeldMutexChild {
        graphics_mutex: &mutex,
        inner: NamedFakeChild::new(ChildScenario::Success),
    };
    run_under_mutex(&root, &mutex, &child);
    assert!(mutex.try_lock().is_ok());
}
