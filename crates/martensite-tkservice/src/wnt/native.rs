//! WinSafe handles the Win32 calls; only importing the public integer HWND is unsafe.
#![allow(unsafe_code)]
use super::window::WntWindowFields;
use crate::aspect::AspectHandle;
use martensite_tkernel::standard::Failure;
use winsafe::{HWND, co};

fn window(handle: AspectHandle) -> HWND {
    // SAFETY: HWND is an opaque OS identifier. We do not dereference it or assume
    // ownership. The used WinSafe operations accept invalid handles as native errors.
    unsafe { HWND::from_ptr(handle as *mut _) }
}

pub(super) fn fields(handle: AspectHandle) -> Result<WntWindowFields, Failure> {
    let window = window(handle);
    if !window.IsWindow() {
        return Err(Failure::with_type(
            "Standard_ConstructionError",
            &format!("WNT_Window handle {handle:#x}: expected a live HWND"),
            "",
        ));
    }
    let rect = window.GetWindowRect().map_err(|error| Failure::with_type(
        "Standard_ConstructionError", &format!("WNT_Window handle {handle:#x}: GetWindowRect failed with {error}; expected a live window rectangle"), ""))?;
    Ok(WntWindowFields {
        h_window: handle,
        h_parent_window: window
            .GetParent()
            .map_or(0, |parent| parent.ptr() as AspectHandle),
        corners: [rect.left, rect.top, rect.right, rect.bottom],
    })
}

pub(super) fn visible(handle: AspectHandle) -> bool {
    window(handle).IsWindowVisible()
}
pub(super) fn show(handle: AspectHandle, mapped: bool) {
    window(handle).ShowWindow(if mapped { co::SW::SHOWNA } else { co::SW::HIDE });
}
pub(super) fn title(handle: AspectHandle, title: &str) {
    let _ = window(handle).SetWindowText(title);
}
pub(super) fn invalidate(handle: AspectHandle) {
    let _ = window(handle).InvalidateRect(None, false);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aspect::AspectWindow;
    use crate::wnt::WntWindow;

    struct TestWindow(HWND);
    impl Drop for TestWindow {
        fn drop(&mut self) {
            let _ = self.0.DestroyWindow();
        }
    }

    fn test_window() -> TestWindow {
        let instance = winsafe::HINSTANCE::GetModuleHandle(None).unwrap();
        // SAFETY: the OS-provided STATIC class accepts no creation data. The HWND
        // is destroyed on this test thread and all arguments live through the call.
        TestWindow(
            unsafe {
                HWND::CreateWindowEx(
                    co::WS_EX::default(),
                    winsafe::AtomStr::from_str("STATIC"),
                    Some("test"),
                    co::WS::OVERLAPPEDWINDOW,
                    winsafe::POINT { x: 10, y: 20 },
                    winsafe::SIZE { cx: 160, cy: 120 },
                    None,
                    winsafe::IdMenu::None,
                    &instance,
                    None,
                )
            }
            .unwrap(),
        )
    }

    #[test]
    fn borrowed_window_maps_titles_resizes_and_preserves_native_ownership() {
        let native = test_window();
        let wrapped = WntWindow::from_handle(native.0.ptr() as usize).unwrap();
        assert_eq!(wrapped.size(), (160, 120));
        assert!(wrapped.do_mapping());
        assert!(!wrapped.is_mapped());
        wrapped.map();
        assert!(wrapped.is_mapped());
        wrapped.set_title(&"Window title".into());
        assert_eq!(native.0.GetWindowText().unwrap(), "Window title");
        wrapped.invalidate_content(&Default::default());
        assert_eq!(
            wrapped.do_resize(),
            crate::aspect::AspectTypeOfResize::NoBorder
        );
        wrapped.unmap();
        assert!(!wrapped.is_mapped());
        drop(wrapped);
        assert!(native.0.IsWindow());
    }

    #[test]
    fn null_and_invalid_native_windows_are_rejected() {
        assert!(super::fields(0).is_err());
        assert!(super::fields(usize::MAX).is_err());
        assert!(!super::visible(0));
    }
}
