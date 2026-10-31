use super::*;

#[test]
fn windows_iterator_destroy_releases_and_can_restart_the_cursor() {
    use crate::osd::{CadPath, FileIterator};
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("entry.txt"), b"test").unwrap();
    let path = CadPath::new(root.path().to_string_lossy().as_bytes());
    let mut iterator = FileIterator::new(&path, b"*");
    assert!(iterator.more());
    iterator.destroy();
    iterator.destroy();
    assert!(!iterator.more());
    iterator.initialize(&path, b"*");
    assert!(iterator.more());
}

#[test]
fn native_node_protection_preserves_world_rights_without_chmod() {
    use crate::osd::{CadPath, OsdErrorAccess, Protection, SingleProtection};
    let file = tempfile::NamedTempFile::new().unwrap();
    let path = file.path().to_string_lossy();
    let mut node = super::super::FileNode::with_calls(
        CadPath::new(path.as_bytes()),
        Arc::new(NativePosixCalls),
    );
    let desired = Protection::with_values(
        SingleProtection::Rwxd,
        SingleProtection::Rwxd,
        SingleProtection::Rx,
        SingleProtection::R,
    );
    node.set_protection(&desired);
    assert!(!node.failed());
    assert_eq!(node.protection().world(), SingleProtection::R);
    assert!(!node.failed());
}

#[test]
fn blocking_descriptor_action_does_not_hold_the_registry() {
    let file = tempfile::tempfile().unwrap();
    let fd = register(file);
    let (entered, observed) = std::sync::mpsc::channel();
    let (release, released) = std::sync::mpsc::channel();
    let worker = std::thread::spawn(move || {
        with_descriptor(fd, |_| {
            entered.send(()).unwrap();
            released.recv().unwrap();
            Ok(())
        })
    });
    observed
        .recv_timeout(std::time::Duration::from_secs(5))
        .unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
    let registry_available = loop {
        if DESCRIPTORS.try_lock().is_ok() {
            break true;
        }
        if std::time::Instant::now() >= deadline {
            break false;
        }
        std::thread::yield_now();
    };
    release.send(()).unwrap();
    worker.join().unwrap().unwrap();
    NativePosixCalls.close(fd).unwrap();
    assert!(
        registry_available,
        "descriptor registry must remain usable during I/O (#335)"
    );
}

#[test]
fn local_file_time_applies_timezone_bias_across_the_epoch_boundary() {
    let mut zone = winsafe::TIME_ZONE_INFORMATION::default();
    zone.bias = 60;
    let time = windows_local_time_in(0, Some(&zone)).unwrap();
    assert_eq!((time.year, time.month, time.day), (1969, 12, 31));
    assert_eq!((time.hour, time.minute, time.second), (23, 0, 0));
    assert!(windows_local_time(i64::MAX).is_none());
    assert!(windows_local_time(i64::MIN).is_none());
}

#[test]
fn native_file_time_uses_the_machine_timezone() {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let time = windows_local_time(now).unwrap();
    assert!((1..=12).contains(&time.month));
    assert!((0..24).contains(&time.hour));
}

#[test]
fn native_lock_is_released_for_an_independent_handle() {
    let temp = tempfile::NamedTempFile::new().unwrap();
    let fd = register(temp.reopen().unwrap());
    let probe = temp.reopen().unwrap();
    NativePosixCalls.record_lock(fd, RecordLock::Write).unwrap();
    assert!(probe.try_lock().is_err());
    NativePosixCalls
        .record_lock(fd, RecordLock::Unlock)
        .unwrap();
    probe.try_lock().unwrap();
    probe.unlock().unwrap();
    NativePosixCalls.close(fd).unwrap();
}
