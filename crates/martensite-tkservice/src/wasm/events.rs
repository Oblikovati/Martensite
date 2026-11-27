//! Portable browser DOM event structures and `AspectWindowInputListener` translators (#632).
//!
//! Works both when constructed directly from `web_sys` DOM events on the main
//! thread and when serialized over `postMessage` into the Orchestrator Worker.

use crate::aspect::{
    ASPECT_VKEY_FLAGS_ALT, ASPECT_VKEY_FLAGS_CTRL, ASPECT_VKEY_FLAGS_META, ASPECT_VKEY_FLAGS_NONE,
    ASPECT_VKEY_FLAGS_SHIFT, ASPECT_VKEY_MOUSE_LEFT_BUTTON, ASPECT_VKEY_MOUSE_MIDDLE_BUTTON,
    ASPECT_VKEY_MOUSE_NONE, ASPECT_VKEY_MOUSE_RIGHT_BUTTON, AspectScrollDelta, AspectVKey,
    AspectVKeyBasic, AspectVKeyFlags, AspectVKeyMouse, AspectWindowInputListener,
};
use crate::cast::int_from_f64;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::Failure;

/// Modifier keys held during a browser pointer, wheel, or keyboard event.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct WasmKeyModifiers {
    /// Shift modifier active.
    pub shift: bool,
    /// Control modifier active.
    pub ctrl: bool,
    /// Alt / Option modifier active.
    pub alt: bool,
    /// Meta / Command / Windows modifier active.
    pub meta: bool,
}

impl WasmKeyModifiers {
    /// Converts DOM modifier booleans into [`AspectVKeyFlags`].
    ///
    /// ```
    /// use martensite_tkservice::wasm::WasmKeyModifiers;
    /// use martensite_tkservice::aspect::ASPECT_VKEY_FLAGS_SHIFT;
    /// let mods = WasmKeyModifiers { shift: true, ..Default::default() };
    /// assert_eq!(mods.to_vkey_flags(), ASPECT_VKEY_FLAGS_SHIFT);
    /// ```
    pub const fn to_vkey_flags(self) -> AspectVKeyFlags {
        let mut flags = ASPECT_VKEY_FLAGS_NONE;
        if self.shift {
            flags |= ASPECT_VKEY_FLAGS_SHIFT;
        }
        if self.ctrl {
            flags |= ASPECT_VKEY_FLAGS_CTRL;
        }
        if self.alt {
            flags |= ASPECT_VKEY_FLAGS_ALT;
        }
        if self.meta {
            flags |= ASPECT_VKEY_FLAGS_META;
        }
        flags
    }
}

/// Kind of browser pointer/mouse event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WasmPointerEventKind {
    /// Pointer button pressed (`pointerdown` / `mousedown`).
    Down,
    /// Pointer button released (`pointerup` / `mouseup`).
    Up,
    /// Pointer moved (`pointermove` / `mousemove`).
    Move,
    /// Pointer left the canvas (`pointerleave` / `mouseleave`).
    Leave,
    /// Pointer sequence cancelled (`pointercancel`).
    Cancel,
}

/// Browser pointer or mouse event in canvas-local CSS pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WasmPointerEvent {
    /// Event phase.
    pub kind: WasmPointerEventKind,
    /// Canvas-local X coordinate in CSS pixels.
    pub css_x: f64,
    /// Canvas-local Y coordinate in CSS pixels.
    pub css_y: f64,
    /// DOM `MouseEvent.buttons` bitmask (1=left, 2=right, 4=middle).
    pub buttons: u16,
    /// Active keyboard modifiers.
    pub modifiers: WasmKeyModifiers,
    /// Event timestamp in seconds.
    pub timestamp_s: f64,
}

/// DOM `WheelEvent.deltaMode` unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WasmWheelDeltaMode {
    /// `DOM_DELTA_PIXEL` (0).
    Pixel,
    /// `DOM_DELTA_LINE` (1).
    Line,
    /// `DOM_DELTA_PAGE` (2).
    Page,
}

/// Browser wheel scroll event in canvas-local CSS pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WasmWheelEvent {
    /// Canvas-local X coordinate in CSS pixels.
    pub css_x: f64,
    /// Canvas-local Y coordinate in CSS pixels.
    pub css_y: f64,
    /// Horizontal wheel delta.
    pub delta_x: f64,
    /// Vertical wheel delta (positive scrolls down in DOM).
    pub delta_y: f64,
    /// Unit of `delta_x` and `delta_y`.
    pub delta_mode: WasmWheelDeltaMode,
    /// Active keyboard modifiers.
    pub modifiers: WasmKeyModifiers,
    /// Event timestamp in seconds.
    pub timestamp_s: f64,
}

/// One touch contact in canvas-local CSS pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WasmTouchPoint {
    /// Stable DOM touch identifier (`Touch.identifier`).
    pub id: usize,
    /// Canvas-local X coordinate in CSS pixels.
    pub css_x: f64,
    /// Canvas-local Y coordinate in CSS pixels.
    pub css_y: f64,
}

/// Kind of browser touch event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WasmTouchEventKind {
    /// `touchstart`.
    Start,
    /// `touchmove`.
    Move,
    /// `touchend`.
    End,
    /// `touchcancel`.
    Cancel,
}

/// Browser touch event carrying changed touch points.
#[derive(Clone, Debug, PartialEq)]
pub struct WasmTouchEvent {
    /// Event phase.
    pub kind: WasmTouchEventKind,
    /// Touch points affected by this event.
    pub changed_touches: Vec<WasmTouchPoint>,
    /// Event timestamp in seconds.
    pub timestamp_s: f64,
}

/// Kind of browser keyboard event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WasmKeyEventKind {
    /// `keydown`.
    Down,
    /// `keyup`.
    Up,
}

/// Browser keyboard event carrying physical `KeyboardEvent.code`.
#[derive(Clone, Debug, PartialEq)]
pub struct WasmKeyEvent {
    /// Key press or release.
    pub kind: WasmKeyEventKind,
    /// Physical W3C `KeyboardEvent.code` (e.g. `"KeyA"`, `"ArrowUp"`, `"Escape"`).
    pub code: String,
    /// Active keyboard modifiers.
    pub modifiers: WasmKeyModifiers,
    /// Whether this event is an auto-repeat.
    pub repeat: bool,
    /// Event timestamp in seconds.
    pub timestamp_s: f64,
}

/// Canvas resize observation from `ResizeObserver` or window `resize`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WasmResizeEvent {
    /// Canvas CSS width in logical pixels.
    pub css_width: f64,
    /// Canvas CSS height in logical pixels.
    pub css_height: f64,
    /// Device pixel ratio (`window.devicePixelRatio`).
    pub device_pixel_ratio: f64,
}

/// Translates DOM `MouseEvent.buttons` bitmask into [`AspectVKeyMouse`].
pub(super) fn buttons_from_dom(buttons: u16) -> AspectVKeyMouse {
    let mut out = ASPECT_VKEY_MOUSE_NONE;
    if buttons & 0x1 != 0 {
        out |= ASPECT_VKEY_MOUSE_LEFT_BUTTON;
    }
    if buttons & 0x2 != 0 {
        out |= ASPECT_VKEY_MOUSE_RIGHT_BUTTON;
    }
    if buttons & 0x4 != 0 {
        out |= ASPECT_VKEY_MOUSE_MIDDLE_BUTTON;
    }
    out
}

/// Translates a W3C `KeyboardEvent.code` string into [`AspectVKey`].
///
/// ```
/// use martensite_tkservice::wasm::virtual_key_from_dom_code;
/// use martensite_tkservice::aspect::AspectVKeyBasic;
/// assert_eq!(virtual_key_from_dom_code("KeyA"), AspectVKeyBasic::A as u32);
/// assert_eq!(virtual_key_from_dom_code("Escape"), AspectVKeyBasic::Escape as u32);
/// ```
pub fn virtual_key_from_dom_code(code: &str) -> AspectVKey {
    use AspectVKeyBasic as K;
    let key = match code {
        "KeyA" => K::A,
        "KeyB" => K::B,
        "KeyC" => K::C,
        "KeyD" => K::D,
        "KeyE" => K::E,
        "KeyF" => K::F,
        "KeyG" => K::G,
        "KeyH" => K::H,
        "KeyI" => K::I,
        "KeyJ" => K::J,
        "KeyK" => K::K,
        "KeyL" => K::L,
        "KeyM" => K::M,
        "KeyN" => K::N,
        "KeyO" => K::O,
        "KeyP" => K::P,
        "KeyQ" => K::Q,
        "KeyR" => K::R,
        "KeyS" => K::S,
        "KeyT" => K::T,
        "KeyU" => K::U,
        "KeyV" => K::V,
        "KeyW" => K::W,
        "KeyX" => K::X,
        "KeyY" => K::Y,
        "KeyZ" => K::Z,
        "Digit0" => K::Digit0,
        "Digit1" => K::Digit1,
        "Digit2" => K::Digit2,
        "Digit3" => K::Digit3,
        "Digit4" => K::Digit4,
        "Digit5" => K::Digit5,
        "Digit6" => K::Digit6,
        "Digit7" => K::Digit7,
        "Digit8" => K::Digit8,
        "Digit9" => K::Digit9,
        "F1" => K::F1,
        "F2" => K::F2,
        "F3" => K::F3,
        "F4" => K::F4,
        "F5" => K::F5,
        "F6" => K::F6,
        "F7" => K::F7,
        "F8" => K::F8,
        "F9" => K::F9,
        "F10" => K::F10,
        "F11" => K::F11,
        "F12" => K::F12,
        "ArrowUp" => K::Up,
        "ArrowDown" => K::Down,
        "ArrowLeft" => K::Left,
        "ArrowRight" => K::Right,
        "PageUp" => K::PageUp,
        "PageDown" => K::PageDown,
        "Home" => K::Home,
        "End" => K::End,
        "Escape" => K::Escape,
        "Enter" | "NumpadEnter" => K::Enter,
        "Backspace" => K::Backspace,
        "Space" => K::Space,
        "Delete" => K::Delete,
        "Backquote" => K::Tilde,
        "Tab" => K::Tab,
        "Minus" => K::Minus,
        "Equal" => K::Equal,
        "Comma" => K::Comma,
        "Period" => K::Period,
        "Semicolon" => K::Semicolon,
        "Slash" => K::Slash,
        "BracketLeft" => K::BracketLeft,
        "Backslash" => K::Backslash,
        "BracketRight" => K::BracketRight,
        "Quote" => K::Apostrophe,
        "NumLock" => K::Numlock,
        "ScrollLock" => K::Scroll,
        "Numpad0" => K::Numpad0,
        "Numpad1" => K::Numpad1,
        "Numpad2" => K::Numpad2,
        "Numpad3" => K::Numpad3,
        "Numpad4" => K::Numpad4,
        "Numpad5" => K::Numpad5,
        "Numpad6" => K::Numpad6,
        "Numpad7" => K::Numpad7,
        "Numpad8" => K::Numpad8,
        "Numpad9" => K::Numpad9,
        "NumpadMultiply" => K::NumpadMultiply,
        "NumpadAdd" => K::NumpadAdd,
        "NumpadSubtract" => K::NumpadSubtract,
        "NumpadDivide" => K::NumpadDivide,
        "ShiftLeft" | "ShiftRight" => K::Shift,
        "ControlLeft" | "ControlRight" => K::Control,
        "AltLeft" | "AltRight" => K::Alt,
        "ContextMenu" => K::Menu,
        "MetaLeft" | "MetaRight" => K::Meta,
        _ => K::Unknown,
    };
    key as AspectVKey
}

pub(super) fn scale_point(css_x: f64, css_y: f64, scale: f64) -> Vec2<i32> {
    Vec2::new(int_from_f64(css_x * scale), int_from_f64(css_y * scale))
}

pub(super) fn dispatch_pointer_event(
    listener: &mut dyn AspectWindowInputListener,
    event: &WasmPointerEvent,
    scale: f64,
) -> Result<bool, Failure> {
    let point = scale_point(event.css_x, event.css_y, scale);
    let buttons = buttons_from_dom(event.buttons);
    let modifiers = event.modifiers.to_vkey_flags();
    match event.kind {
        WasmPointerEventKind::Down | WasmPointerEventKind::Up => {
            listener.update_mouse_buttons(point, buttons, modifiers, false)
        }
        WasmPointerEventKind::Move => {
            listener.update_mouse_position(point, buttons, modifiers, false)
        }
        WasmPointerEventKind::Leave | WasmPointerEventKind::Cancel => {
            listener.update_mouse_buttons(point, ASPECT_VKEY_MOUSE_NONE, modifiers, false)
        }
    }
}

pub(super) fn dispatch_wheel_event(
    listener: &mut dyn AspectWindowInputListener,
    event: &WasmWheelEvent,
    scale: f64,
) -> bool {
    let point = scale_point(event.css_x, event.css_y, scale);
    // Normalize DOM scroll delta (positive = scroll down) into AspectScrollDelta
    // where positive delta zooms in (wheel up).
    let step = match event.delta_mode {
        WasmWheelDeltaMode::Pixel => -event.delta_y / 100.0,
        WasmWheelDeltaMode::Line => -event.delta_y / 3.0,
        WasmWheelDeltaMode::Page => -event.delta_y,
    };
    let delta = AspectScrollDelta::new(point, step, event.modifiers.to_vkey_flags());
    listener.update_mouse_scroll(&delta)
}

pub(super) fn dispatch_touch_event(
    listener: &mut dyn AspectWindowInputListener,
    event: &WasmTouchEvent,
    scale: f64,
) -> Result<bool, Failure> {
    for touch in &event.changed_touches {
        let point = Vec2::new(touch.css_x * scale, touch.css_y * scale);
        match event.kind {
            WasmTouchEventKind::Start => listener.add_touch_point(touch.id, point, false)?,
            WasmTouchEventKind::Move => listener.update_touch_point(touch.id, point)?,
            WasmTouchEventKind::End | WasmTouchEventKind::Cancel => {
                listener.remove_touch_point(touch.id, false)?;
            }
        }
    }
    Ok(true)
}

pub(super) fn dispatch_key_event(
    listener: &mut dyn AspectWindowInputListener,
    event: &WasmKeyEvent,
) -> Result<bool, Failure> {
    let vkey = virtual_key_from_dom_code(&event.code);
    if vkey == AspectVKeyBasic::Unknown as AspectVKey {
        return Ok(false);
    }
    match event.kind {
        WasmKeyEventKind::Down => listener.key_down(vkey, event.timestamp_s, 1.0)?,
        WasmKeyEventKind::Up => listener.key_up(vkey, event.timestamp_s)?,
    }
    Ok(true)
}

#[cfg(target_arch = "wasm32")]
mod web_sys_adapters {
    use super::{
        WasmKeyEvent, WasmKeyEventKind, WasmKeyModifiers, WasmPointerEvent, WasmPointerEventKind,
        WasmWheelDeltaMode, WasmWheelEvent,
    };

    impl WasmPointerEvent {
        /// Extracts a [`WasmPointerEvent`] from a DOM [`web_sys::PointerEvent`].
        pub fn from_dom(kind: WasmPointerEventKind, event: &web_sys::PointerEvent) -> Self {
            Self {
                kind,
                css_x: f64::from(event.offset_x()),
                css_y: f64::from(event.offset_y()),
                buttons: event.buttons(),
                modifiers: WasmKeyModifiers {
                    shift: event.shift_key(),
                    ctrl: event.ctrl_key(),
                    alt: event.alt_key(),
                    meta: event.meta_key(),
                },
                timestamp_s: event.time_stamp() * 1e-3,
            }
        }
    }

    impl WasmWheelEvent {
        /// Extracts a [`WasmWheelEvent`] from a DOM [`web_sys::WheelEvent`].
        pub fn from_dom(event: &web_sys::WheelEvent) -> Self {
            let delta_mode = match event.delta_mode() {
                1 => WasmWheelDeltaMode::Line,
                2 => WasmWheelDeltaMode::Page,
                _ => WasmWheelDeltaMode::Pixel,
            };
            Self {
                css_x: f64::from(event.offset_x()),
                css_y: f64::from(event.offset_y()),
                delta_x: event.delta_x(),
                delta_y: event.delta_y(),
                delta_mode,
                modifiers: WasmKeyModifiers {
                    shift: event.shift_key(),
                    ctrl: event.ctrl_key(),
                    alt: event.alt_key(),
                    meta: event.meta_key(),
                },
                timestamp_s: event.time_stamp() * 1e-3,
            }
        }
    }

    impl WasmKeyEvent {
        /// Extracts a [`WasmKeyEvent`] from a DOM [`web_sys::KeyboardEvent`].
        pub fn from_dom(kind: WasmKeyEventKind, event: &web_sys::KeyboardEvent) -> Self {
            Self {
                kind,
                code: event.code(),
                modifiers: WasmKeyModifiers {
                    shift: event.shift_key(),
                    ctrl: event.ctrl_key(),
                    alt: event.alt_key(),
                    meta: event.meta_key(),
                },
                repeat: event.repeat(),
                timestamp_s: event.time_stamp() * 1e-3,
            }
        }
    }
}
