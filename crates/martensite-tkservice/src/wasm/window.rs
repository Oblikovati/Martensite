//! `Wasm_Window`: browser canvas window supporting `OffscreenCanvas`,
//! `HtmlCanvasElement`, and virtual-size canvas operation (#632).
use super::events::{
    WasmKeyEvent, WasmPointerEvent, WasmResizeEvent, WasmTouchEvent, WasmWheelEvent,
    dispatch_key_event, dispatch_pointer_event, dispatch_touch_event, dispatch_wheel_event,
    virtual_key_from_dom_code,
};
#[cfg(target_arch = "wasm32")]
use super::realm::RealmHandle;
use crate::aspect::{
    ASPECT_VKEY_MOUSE_LEFT_BUTTON, ASPECT_VKEY_MOUSE_MIDDLE_BUTTON, ASPECT_VKEY_MOUSE_NONE,
    ASPECT_VKEY_MOUSE_RIGHT_BUTTON, AspectDisplayConnection, AspectDrawable, AspectFbConfig,
    AspectTypeOfResize, AspectVKey, AspectVKeyBasic, AspectVKeyMouse, AspectWindow,
    AspectWindowBase, AspectWindowInputListener, AspectWindowInputListenerMembers as _,
    AspectWindowMembers as _,
};
use crate::cast::int_from_f64;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::{Failure, Handle, SharedState};
use martensite_tkernel::tcollection::AsciiString;

/// Browser canvas handle exposed to WebGPU without coupling `martensite-tkservice` to `wgpu` (#632).
#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug)]
pub enum WasmCanvasTarget {
    /// Dedicated-worker `OffscreenCanvas` transferred from the main thread.
    Offscreen(web_sys::OffscreenCanvas),
    /// Main-thread `HTMLCanvasElement`.
    Html(web_sys::HtmlCanvasElement),
}

/// Declares the uninhabited Emscripten event types (none exists without Emscripten).
macro_rules! emscripten_events {
    ($($(#[$doc:meta])* $name:ident;)+) => {
        $(
            $(#[$doc])*
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub enum $name {}
        )+
    };
}

emscripten_events! {
    /// `EmscriptenMouseEvent`.
    EmscriptenMouseEvent;
    /// `EmscriptenWheelEvent`.
    EmscriptenWheelEvent;
    /// `EmscriptenTouchEvent`.
    EmscriptenTouchEvent;
    /// `EmscriptenKeyboardEvent`.
    EmscriptenKeyboardEvent;
    /// `EmscriptenUiEvent`.
    EmscriptenUiEvent;
    /// `EmscriptenFocusEvent`.
    EmscriptenFocusEvent;
}

/// The protected fields of a canvas window.
#[derive(Clone, Debug)]
pub struct WasmWindowFields {
    /// `myCanvasId`.
    pub canvas_id: AsciiString,
    /// `mySize`: the backing size.
    pub size: Vec2<i32>,
    /// `myDevicePixelRatio`.
    pub device_pixel_ratio: f64,
    /// `myToScaleBacking`.
    pub to_scale_backing: bool,
    /// `myIsMapped`.
    pub is_mapped: bool,
}

/// A canvas window; `WasmWindow::new(&"#canvas".into(), true)`.
pub struct WasmWindow {
    base: AspectWindowBase,
    /// The fields, shared by every handle.
    pub fields: SharedState<WasmWindowFields>,
    #[cfg(target_arch = "wasm32")]
    canvas: SharedState<Option<RealmHandle<WasmCanvasTarget>>>,
}
martensite_tkernel::impl_transient!(WasmWindow, "Wasm_Window", dyn AspectWindow, interfaces: [dyn AspectWindow]);

impl WasmWindow {
    /// A mapped window of a canvas, of size 0 and pixel ratio 1; `WasmWindow::new(&id, true)`.
    pub fn new(canvas_id: &AsciiString, to_scale_backing: bool) -> Self {
        let fields = WasmWindowFields {
            canvas_id: canvas_id.clone(),
            size: Vec2::new(0, 0),
            device_pixel_ratio: 1.0,
            to_scale_backing,
            is_mapped: true,
        };
        Self {
            base: AspectWindowBase::default(),
            fields: SharedState::new(fields),
            #[cfg(target_arch = "wasm32")]
            canvas: SharedState::new(None),
        }
    }

    /// Binds a worker-owned [`web_sys::OffscreenCanvas`] for WebGPU presentation (#632).
    #[cfg(target_arch = "wasm32")]
    pub fn from_offscreen_canvas(
        canvas_id: &AsciiString,
        canvas: web_sys::OffscreenCanvas,
        device_pixel_ratio: f64,
    ) -> Self {
        let ratio = if device_pixel_ratio.is_finite() && device_pixel_ratio > 0.0 {
            device_pixel_ratio
        } else {
            1.0
        };
        let size = Vec2::new(canvas.width() as i32, canvas.height() as i32);
        let fields = WasmWindowFields {
            canvas_id: canvas_id.clone(),
            size,
            device_pixel_ratio: ratio,
            to_scale_backing: true,
            is_mapped: true,
        };
        Self {
            base: AspectWindowBase::default(),
            fields: SharedState::new(fields),
            canvas: SharedState::new(Some(RealmHandle::new(WasmCanvasTarget::Offscreen(canvas)))),
        }
    }

    /// Binds a main-thread [`web_sys::HtmlCanvasElement`] for WebGPU presentation (#632).
    #[cfg(target_arch = "wasm32")]
    pub fn from_html_canvas(
        canvas_id: &AsciiString,
        canvas: web_sys::HtmlCanvasElement,
        device_pixel_ratio: f64,
    ) -> Self {
        let ratio = if device_pixel_ratio.is_finite() && device_pixel_ratio > 0.0 {
            device_pixel_ratio
        } else {
            1.0
        };
        let size = Vec2::new(canvas.width() as i32, canvas.height() as i32);
        let fields = WasmWindowFields {
            canvas_id: canvas_id.clone(),
            size,
            device_pixel_ratio: ratio,
            to_scale_backing: true,
            is_mapped: true,
        };
        Self {
            base: AspectWindowBase::default(),
            fields: SharedState::new(fields),
            canvas: SharedState::new(Some(RealmHandle::new(WasmCanvasTarget::Html(canvas)))),
        }
    }

    /// Clones the bound browser canvas handle on its owning realm thread (#632).
    /// Returns a typed `Standard_ProgramError` if accessed from another thread.
    #[cfg(target_arch = "wasm32")]
    pub fn canvas_target(&self) -> Result<Option<WasmCanvasTarget>, Failure> {
        let guard = self.canvas.read();
        let Some(handle) = guard.as_ref() else {
            return Ok(None);
        };
        handle.get_checked("WasmWindow").cloned().map(Some)
    }

    /// Releases the bound browser canvas handle on its owning realm thread (#632).
    #[cfg(target_arch = "wasm32")]
    pub fn close(&self) -> Result<(), Failure> {
        let mut guard = self.canvas.write();
        let Some(handle) = guard.take() else {
            return Ok(());
        };
        match handle.try_take() {
            Ok(target) => {
                drop(target);
                Ok(())
            }
            Err(unreleased) => {
                *guard = Some(unreleased);
                Err(Failure::with_type(
                    "Standard_ProgramError",
                    "WasmWindow::close: called from a non-owning thread; expected the creating realm thread",
                    "",
                ))
            }
        }
    }

    /// Translates a W3C `KeyboardEvent.code` string into [`AspectVKey`] (#632).
    pub fn virtual_key_from_code(code: &str) -> AspectVKey {
        virtual_key_from_dom_code(code)
    }

    /// Translates a [`WasmPointerEvent`] into `AspectWindowInputListener` mouse callbacks (#632).
    pub fn process_wasm_pointer_event(
        &self,
        listener: &mut dyn AspectWindowInputListener,
        event: &WasmPointerEvent,
    ) -> Result<bool, Failure> {
        dispatch_pointer_event(listener, event, self.input_scale())
    }

    /// Translates a [`WasmWheelEvent`] into `AspectWindowInputListener::update_mouse_scroll` (#632).
    pub fn process_wasm_wheel_event(
        &self,
        listener: &mut dyn AspectWindowInputListener,
        event: &WasmWheelEvent,
    ) -> bool {
        dispatch_wheel_event(listener, event, self.input_scale())
    }

    /// Translates a [`WasmTouchEvent`] into `AspectWindowInputListener` touch callbacks (#632).
    pub fn process_wasm_touch_event(
        &self,
        listener: &mut dyn AspectWindowInputListener,
        event: &WasmTouchEvent,
    ) -> Result<bool, Failure> {
        dispatch_touch_event(listener, event, self.input_scale())
    }

    /// Translates a [`WasmKeyEvent`] into `AspectWindowInputListener::key_down`/`key_up` (#632).
    pub fn process_wasm_key_event(
        &self,
        listener: &mut dyn AspectWindowInputListener,
        event: &WasmKeyEvent,
    ) -> Result<bool, Failure> {
        dispatch_key_event(listener, event)
    }

    /// Applies a [`WasmResizeEvent`], updates backing dimensions and DPR, and notifies `listener` (#632).
    pub fn process_wasm_resize_event(
        &self,
        listener: &mut dyn AspectWindowInputListener,
        event: &WasmResizeEvent,
    ) -> AspectTypeOfResize {
        let ratio = if event.device_pixel_ratio.is_finite() && event.device_pixel_ratio > 0.0 {
            event.device_pixel_ratio
        } else {
            1.0
        };
        let (prev, next) = {
            let mut fields = self.fields.write();
            let prev = fields.size;
            fields.device_pixel_ratio = ratio;
            let scale = if fields.to_scale_backing { ratio } else { 1.0 };
            let next = Vec2::new(
                int_from_f64(event.css_width * scale).max(0),
                int_from_f64(event.css_height * scale).max(0),
            );
            fields.size = next;
            (prev, next)
        };
        #[cfg(target_arch = "wasm32")]
        if let Ok(Some(target)) = self.canvas_target() {
            let (w, h) = (next.x.max(0) as u32, next.y.max(0) as u32);
            match target {
                WasmCanvasTarget::Offscreen(c) => {
                    c.set_width(w);
                    c.set_height(h);
                }
                WasmCanvasTarget::Html(c) => {
                    c.set_width(w);
                    c.set_height(h);
                }
            }
        }
        let changed = prev != next;
        listener.process_configure(changed);
        if changed {
            AspectTypeOfResize::TopAndRightBorder
        } else {
            AspectTypeOfResize::Unknown
        }
    }

    fn input_scale(&self) -> f64 {
        let fields = self.fields.read();
        if fields.to_scale_backing {
            fields.device_pixel_ratio
        } else {
            1.0
        }
    }

    fn has_live_canvas(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            self.canvas.read().is_some()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            false
        }
    }
    /// `MouseButtonsFromNative`: bits 0, 1 and 2 are the left, right and middle buttons;
    /// `WasmWindow::mouse_buttons_from_native(0x3)`.
    pub fn mouse_buttons_from_native(buttons: u16) -> AspectVKeyMouse {
        let pairs = [
            (0x1, ASPECT_VKEY_MOUSE_LEFT_BUTTON),
            (0x2, ASPECT_VKEY_MOUSE_RIGHT_BUTTON),
            (0x4, ASPECT_VKEY_MOUSE_MIDDLE_BUTTON),
        ];
        pairs
            .iter()
            .filter(|(bit, _)| buttons & bit != 0)
            .fold(ASPECT_VKEY_MOUSE_NONE, |acc, (_, key)| acc | key)
    }
    /// `VirtualKeyFromNative`: unknown without Emscripten; `WasmWindow::virtual_key_from_native(13)`.
    pub fn virtual_key_from_native(key: i32) -> AspectVKey {
        let _ = key;
        AspectVKeyBasic::Unknown as AspectVKey
    }
    /// `SetSizeLogical`: the logical size scaled by the pixel ratio (truncated);
    /// `window.set_size_logical(Vec2::new(320.5, 200.0))`.
    pub fn set_size_logical(&self, size: Vec2<f64>) {
        let mut fields = self.fields.write();
        let ratio = fields.device_pixel_ratio;
        fields.size = Vec2::new(int_from_f64(size.x * ratio), int_from_f64(size.y * ratio));
    }
    /// `SetSizeBacking`; `window.set_size_backing(Vec2::new(640, 400))`.
    pub fn set_size_backing(&self, size: Vec2<i32>) {
        self.fields.write().size = size;
    }
    /// `CanvasId`; `window.canvas_id()`.
    pub fn canvas_id(&self) -> AsciiString {
        self.fields.read().canvas_id.clone()
    }
    /// `SetDevicePixelRatio`; `window.set_device_pixel_ratio(2.0)`.
    pub fn set_device_pixel_ratio(&self, ratio: f64) {
        self.fields.write().device_pixel_ratio = ratio;
    }
    /// `ProcessMessage`: false without Emscripten; `window.process_message(&mut listener, 0)`.
    pub fn process_message(
        &self,
        listener: &mut dyn AspectWindowInputListener,
        event_type: i32,
    ) -> bool {
        let _ = (listener, event_type);
        false
    }
    /// `ProcessMouseEvent`: false without Emscripten.
    pub fn process_mouse_event(
        &self,
        listener: &mut dyn AspectWindowInputListener,
        event_type: i32,
        event: Option<&EmscriptenMouseEvent>,
    ) -> bool {
        let _ = (listener, event_type, event);
        false
    }
    /// `ProcessWheelEvent`: false without Emscripten.
    pub fn process_wheel_event(
        &self,
        listener: &mut dyn AspectWindowInputListener,
        event_type: i32,
        event: Option<&EmscriptenWheelEvent>,
    ) -> bool {
        let _ = (listener, event_type, event);
        false
    }
    /// `ProcessTouchEvent`: whether the listener has touch points; `window.process_touch_event(&mut l, 0, None)`.
    pub fn process_touch_event(
        &self,
        listener: &mut dyn AspectWindowInputListener,
        event_type: i32,
        event: Option<&EmscriptenTouchEvent>,
    ) -> bool {
        let _ = (event_type, event);
        listener.has_touch_points()
    }
    /// `ProcessKeyEvent`: false without Emscripten.
    pub fn process_key_event(
        &self,
        listener: &mut dyn AspectWindowInputListener,
        event_type: i32,
        event: Option<&EmscriptenKeyboardEvent>,
    ) -> bool {
        let _ = (listener, event_type, event);
        false
    }
    /// `ProcessUiEvent`: a resize configure of the listener; `window.process_ui_event(&mut l, 0, None)`.
    pub fn process_ui_event(
        &self,
        listener: &mut dyn AspectWindowInputListener,
        event_type: i32,
        event: Option<&EmscriptenUiEvent>,
    ) -> bool {
        let _ = (event_type, event);
        listener.process_configure(true);
        true
    }
    /// `ProcessFocusEvent`: a focus loss of the listener; `window.process_focus_event(&mut l, 0, None)`.
    pub fn process_focus_event(
        &self,
        listener: &mut dyn AspectWindowInputListener,
        event_type: i32,
        event: Option<&EmscriptenFocusEvent>,
    ) -> bool {
        let _ = (event_type, event);
        listener.process_focus(false);
        true
    }
}

impl AspectWindow for WasmWindow {
    fn window_base(&self) -> &AspectWindowBase {
        &self.base
    }
    fn is_mapped(&self) -> bool {
        self.fields.read().is_mapped
    }
    fn map(&self) {
        self.fields.write().is_mapped = true;
    }
    fn unmap(&self) {
        self.fields.write().is_mapped = false;
    }
    /// Unknown without Emscripten.
    fn do_resize(&self) -> AspectTypeOfResize {
        AspectTypeOfResize::Unknown
    }
    fn do_mapping(&self) -> bool {
        true
    }
    /// The width over the height of the size, 1 for an empty size.
    fn ratio(&self) -> f64 {
        let size = self.fields.read().size;
        if size.x != 0 && size.y != 0 {
            f64::from(size.x) / f64::from(size.y)
        } else {
            1.0
        }
    }
    /// `(0, 0, width, height)` for a virtual or canvas-bound window; the native leaves
    /// the right and bottom unset otherwise (0 here).
    fn position(&self) -> (i32, i32, i32, i32) {
        let size = self.fields.read().size;
        if self.is_virtual() || self.has_live_canvas() {
            (0, 0, size.x, size.y)
        } else {
            (0, 0, 0, 0)
        }
    }
    /// The size of a virtual or canvas-bound window; the native leaves the outputs unset otherwise (0 here).
    fn size(&self) -> (i32, i32) {
        let size = self.fields.read().size;
        if self.is_virtual() || self.has_live_canvas() {
            (size.x, size.y)
        } else {
            (0, 0)
        }
    }
    fn native_handle(&self) -> AspectDrawable {
        if self.has_live_canvas() { 1 } else { 0 }
    }
    fn native_parent_handle(&self) -> AspectDrawable {
        0
    }
    fn native_fb_config(&self) -> AspectFbConfig {
        AspectFbConfig::default()
    }
    fn device_pixel_ratio(&self) -> f64 {
        self.fields.read().device_pixel_ratio
    }
    /// Nothing without Emscripten.
    fn invalidate_content(&self, display: &Handle<AspectDisplayConnection>) {
        let _ = display;
    }
}

#[cfg(test)]
mod wasm_event_tests {
    use super::*;
    use crate::aspect::{
        ASPECT_VKEY_FLAGS_SHIFT, AspectScrollDelta, AspectVKey, AspectVKeyFlags, AspectVKeyMouse,
        AspectWindowInputListenerBase,
    };
    use crate::wasm::{
        WasmKeyEvent, WasmKeyEventKind, WasmKeyModifiers, WasmPointerEvent, WasmPointerEventKind,
        WasmResizeEvent, WasmTouchEvent, WasmTouchEventKind, WasmTouchPoint, WasmWheelDeltaMode,
        WasmWheelEvent,
    };
    use crate::wnt::WntHidSpaceMouse;

    #[derive(Default)]
    struct RecordingListener {
        base: AspectWindowInputListenerBase,
        configured: Vec<bool>,
        scrolls: Vec<(Vec2<i32>, f64, AspectVKeyFlags)>,
        last_point: Vec2<i32>,
        last_buttons: AspectVKeyMouse,
        last_mods: AspectVKeyFlags,
        keys_down: Vec<AspectVKey>,
        keys_up: Vec<AspectVKey>,
    }

    impl AspectWindowInputListener for RecordingListener {
        fn input_base(&self) -> &AspectWindowInputListenerBase {
            &self.base
        }
        fn input_base_mut(&mut self) -> &mut AspectWindowInputListenerBase {
            &mut self.base
        }
        fn process_expose(&mut self) {}
        fn process_configure(&mut self, is_resized: bool) {
            self.configured.push(is_resized);
        }
        fn process_input(&mut self) {}
        fn process_focus(&mut self, _is_activated: bool) {}
        fn process_close(&mut self) {}
        fn key_down(&mut self, key: AspectVKey, time: f64, pressure: f64) -> Result<(), Failure> {
            self.keys_down.push(key);
            self.base.key_down(key, time, pressure)
        }
        fn key_up(&mut self, key: AspectVKey, time: f64) -> Result<(), Failure> {
            self.keys_up.push(key);
            self.base.key_up(key, time)
        }
        fn key_from_axis(
            &mut self,
            neg: AspectVKey,
            pos: AspectVKey,
            time: f64,
            pressure: f64,
        ) -> Result<(), Failure> {
            self.base.key_from_axis(neg, pos, time, pressure)
        }
        fn update_mouse_scroll(&mut self, delta: &AspectScrollDelta) -> bool {
            self.scrolls.push((delta.point, delta.delta, delta.flags));
            true
        }
        fn update_mouse_buttons(
            &mut self,
            point: Vec2<i32>,
            buttons: AspectVKeyMouse,
            modifiers: AspectVKeyFlags,
            _is_emulated: bool,
        ) -> Result<bool, Failure> {
            self.last_point = point;
            self.last_buttons = buttons;
            self.last_mods = modifiers;
            Ok(true)
        }
        fn update_mouse_position(
            &mut self,
            point: Vec2<i32>,
            buttons: AspectVKeyMouse,
            modifiers: AspectVKeyFlags,
            _is_emulated: bool,
        ) -> Result<bool, Failure> {
            self.last_point = point;
            self.last_buttons = buttons;
            self.last_mods = modifiers;
            Ok(true)
        }
        fn update_3d_mouse(&mut self, _event: &WntHidSpaceMouse<'_>) -> Result<bool, Failure> {
            Ok(false)
        }
    }

    #[test]
    fn wasm_events_scale_by_dpr_and_dispatch_to_input_listener() {
        let win = WasmWindow::new(&AsciiString::from("#cad-canvas"), true);
        win.set_virtual(true);
        let mut listener = RecordingListener::default();

        let resize = WasmResizeEvent {
            css_width: 400.0,
            css_height: 300.0,
            device_pixel_ratio: 2.0,
        };
        assert_ne!(
            win.process_wasm_resize_event(&mut listener, &resize),
            AspectTypeOfResize::Unknown
        );
        assert_eq!(win.size(), (800, 600));
        assert_eq!(win.device_pixel_ratio(), 2.0);
        assert_eq!(listener.configured, vec![true]);

        let pointer = WasmPointerEvent {
            kind: WasmPointerEventKind::Down,
            css_x: 100.0,
            css_y: 50.0,
            buttons: 0x1,
            modifiers: WasmKeyModifiers {
                shift: true,
                ..Default::default()
            },
            timestamp_s: 1.0,
        };
        assert!(
            win.process_wasm_pointer_event(&mut listener, &pointer)
                .unwrap()
        );
        assert_eq!(listener.last_point, Vec2::new(200, 100));
        assert_eq!(listener.last_buttons, ASPECT_VKEY_MOUSE_LEFT_BUTTON);
        assert_eq!(listener.last_mods, ASPECT_VKEY_FLAGS_SHIFT);

        let wheel = WasmWheelEvent {
            css_x: 10.0,
            css_y: 20.0,
            delta_x: 0.0,
            delta_y: -100.0,
            delta_mode: WasmWheelDeltaMode::Pixel,
            modifiers: WasmKeyModifiers::default(),
            timestamp_s: 1.1,
        };
        assert!(win.process_wasm_wheel_event(&mut listener, &wheel));
        assert_eq!(listener.scrolls.len(), 1);
        assert_eq!(listener.scrolls[0].0, Vec2::new(20, 40));
        assert!(listener.scrolls[0].1 > 0.0);

        let touch = WasmTouchEvent {
            kind: WasmTouchEventKind::Start,
            changed_touches: vec![WasmTouchPoint {
                id: 7,
                css_x: 15.0,
                css_y: 25.0,
            }],
            timestamp_s: 1.2,
        };
        assert!(win.process_wasm_touch_event(&mut listener, &touch).unwrap());
        assert!(listener.has_touch_points());

        let key = WasmKeyEvent {
            kind: WasmKeyEventKind::Down,
            code: "KeyF".to_string(),
            modifiers: WasmKeyModifiers::default(),
            repeat: false,
            timestamp_s: 1.3,
        };
        assert!(win.process_wasm_key_event(&mut listener, &key).unwrap());
        assert_eq!(listener.keys_down, vec![AspectVKeyBasic::F as AspectVKey]);
    }
}
