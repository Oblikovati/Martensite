//! Wasm_Window (built without Emscripten) and the display-free parts of Xw_Window.
use super::common::{Reader, end, flag, int, real, text};
use super::print::Output;
use super::tried;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::aspect::{AspectWindow, AspectWindowMembers};
use martensite_tkservice::wasm::WasmWindow;
use martensite_tkservice::xw::XwWindow;

fn geometry(output: &Output, window: &WasmWindow, is_virtual: bool, size: [i32; 2]) {
    real(output, window.ratio());
    window.set_size_backing(Vec2::new(size[0], size[1]));
    real(output, window.ratio());
    let (w, h) = window.size();
    int(output, if is_virtual { w } else { 0 });
    int(output, if is_virtual { h } else { 0 });
    let (x1, y1, x2, y2) = window.position();
    int(output, x1);
    int(output, y1);
    int(output, if is_virtual { x2 } else { 0 });
    int(output, if is_virtual { y2 } else { 0 });
}

/// `sv_window`: a canvas window's geometry and conversions, then the Xw key translations
/// and constructor checks.
pub fn window_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (is_virtual, ratio) = (r.flag(), r.real());
    let size = [r.integer(), r.integer()];
    let (log_w, log_h) = (r.real(), r.real());
    let (buttons, key) = (r.integer(), r.integer());
    let window = WasmWindow::new(&AsciiString::from("#canvas"), true);
    window.set_virtual(is_virtual);
    text(output, window.canvas_id().as_bytes());
    flag(output, window.is_mapped());
    window.unmap();
    flag(output, window.is_mapped());
    window.map();
    flag(output, window.is_mapped());
    flag(output, window.do_mapping());
    int(output, window.do_resize() as i32);
    geometry(output, &window, is_virtual, size);
    window.set_device_pixel_ratio(ratio);
    real(output, window.device_pixel_ratio());
    window.set_size_logical(Vec2::new(log_w, log_h));
    real(output, window.ratio());
    int(output, window.native_handle() as i32);
    int(output, window.native_parent_handle() as i32);
    flag(output, window.native_fb_config() == 0);
    int(
        output,
        WasmWindow::mouse_buttons_from_native(buttons as u16) as i32,
    );
    int(output, WasmWindow::virtual_key_from_native(key) as i32);
    for _ in 0..r.integer() {
        int(
            output,
            XwWindow::virtual_key_from_native(r.real() as u64) as i32,
        );
    }
    tried(
        output,
        XwWindow::create(&Handle::default(), "t", 0, 0, size[0], size[1]).map(|_| ()),
    );
    tried(
        output,
        XwWindow::wrap(&Handle::default(), key as u64, 0).map(|_| ()),
    );
    end(output);
}
