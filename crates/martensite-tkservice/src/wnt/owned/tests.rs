use crate::{aspect::AspectWindow, wnt::WntWindow};

#[test]
fn owned_window_is_destroyed_when_dropped_on_another_thread() {
    let window = WntWindow::new("Owned window", 10, 20, 240, 160).unwrap();
    let handle = window.h_window();
    assert_eq!(window.size(), (240, 160));
    window.map();
    assert!(window.is_mapped());
    window.set_title(&"Owned title".into());
    window.unmap();
    std::thread::spawn(move || drop(window)).join().unwrap();
    assert!(WntWindow::from_handle(handle).is_err());
}

#[test]
fn owned_creation_rejects_invalid_dimensions_and_embedded_nul() {
    for (title, width, height) in [("bad", 0, 10), ("bad", 10, -1), ("a\0b", 10, 10)] {
        assert!(WntWindow::new(title, 0, 0, width, height).is_err());
    }
}

#[test]
fn owned_windows_can_be_created_concurrently() {
    let workers: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                let window = WntWindow::new("Concurrent", 0, 0, 100, 100).unwrap();
                assert!(window.do_mapping());
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
}

#[test]
fn owned_window_and_owner_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<WntWindow>();
    assert_send_sync::<super::WindowOwner>();
    assert_send_sync::<crate::wnt::WntRawHidReport>();
    assert_send_sync::<crate::wnt::WntRawInputBatch>();
}
