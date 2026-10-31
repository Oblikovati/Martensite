use super::*;
use martensite_tkernel::standard::Handle;

#[test]
fn win32_connection_borrows_the_process_module_without_x_atoms() {
    let connection = AspectDisplayConnection::new().unwrap();
    assert_eq!(
        connection.get_display_aspect(),
        winsafe::HINSTANCE::GetModuleHandle(None).unwrap().ptr() as usize
    );
    assert!(!connection.is_own_display());
    assert!(connection.get_atom(AspectXAtom::DeleteWindow).is_err());
}

#[test]
fn window_lease_blocks_reset_until_native_window_teardown() {
    use crate::{aspect::AspectWindow, wnt::WntWindow};
    let window = WntWindow::new("Retained display", 0, 0, 100, 100).unwrap();
    let display = window.native_display_connection();
    assert!(display.init_checked(0).is_err());
    window.window_base().fields_mut().display = Handle::default();
    assert!(!window.native_display_connection().is_null());
    drop(window);
    assert!(display.init_checked(0).is_ok());
}

#[test]
fn explicit_borrowed_instance_is_retained_without_ownership() {
    let address = winsafe::HINSTANCE::GetModuleHandle(None).unwrap().ptr() as usize;
    let display = AspectDisplayConnection::with_display(address).unwrap();
    assert_eq!(display.get_display_aspect(), address);
    assert!(!display.is_own_display());
}
