//! Independent pthread identity oracle through the standard-library join handle.
#![cfg(unix)]

#[test]
fn current_identity_matches_live_join_handle() {
    use martensite_tkernel::osd::OsdThread;
    use std::os::unix::thread::JoinHandleExt;
    let (identity_tx, identity_rx) = std::sync::mpsc::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let worker = std::thread::spawn(move || {
        let identity = OsdThread::current();
        identity_tx.send((identity, OsdThread::current())).unwrap();
        release_rx.recv().unwrap();
    });
    #[allow(clippy::unnecessary_cast)]
    let expected = worker.as_pthread_t() as usize;
    let observed = identity_rx.recv().unwrap();
    release_tx.send(()).unwrap();
    worker.join().unwrap();
    assert_eq!(observed, (expected, expected));
}
