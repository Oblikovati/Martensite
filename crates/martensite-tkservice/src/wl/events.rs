//! Wayland window events and translation into [`AspectWindowInputListener`].
use crate::aspect::{
    ASPECT_VKEY_FLAGS_ALT, ASPECT_VKEY_FLAGS_CTRL, ASPECT_VKEY_FLAGS_NONE, ASPECT_VKEY_FLAGS_SHIFT,
    ASPECT_VKEY_MOUSE_LEFT_BUTTON, ASPECT_VKEY_MOUSE_MIDDLE_BUTTON, ASPECT_VKEY_MOUSE_NONE,
    ASPECT_VKEY_MOUSE_RIGHT_BUTTON, AspectScrollDelta, AspectVKey, AspectVKeyBasic as K,
    AspectVKeyFlags, AspectVKeyMouse, AspectWindowInputListener,
    AspectWindowInputListenerMembers as _,
};
use crate::xw::XwWindow;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::Failure;

/// Linux evdev left mouse button (`BTN_LEFT`).
pub(crate) const BTN_LEFT: u32 = 0x110;
/// Linux evdev right mouse button (`BTN_RIGHT`).
pub(crate) const BTN_RIGHT: u32 = 0x111;
/// Linux evdev middle mouse button (`BTN_MIDDLE`).
pub(crate) const BTN_MIDDLE: u32 = 0x112;

/// A decoded native Wayland event queued for delivery to an [`AspectWindowInputListener`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WlEvent {
    /// Compositor requested window close (`xdg_toplevel::close`).
    Close,
    /// Compositor configured surface dimensions (`xdg_toplevel::configure`).
    Configure {
        /// Configured logical width in pixels.
        width: i32,
        /// Configured logical height in pixels.
        height: i32,
    },
    /// Keyboard focus gained (`true`) or lost (`false`).
    Focus(bool),
    /// Key press or release with XKB keysym.
    Key {
        /// XKB keysym value.
        keysym: u32,
        /// `true` on press, `false` on release.
        pressed: bool,
    },
    /// Pointer movement in surface-local coordinates.
    PointerMotion {
        /// Pointer X in surface coordinates.
        x: i32,
        /// Pointer Y in surface coordinates.
        y: i32,
        /// Active mouse button bitmask.
        buttons: AspectVKeyMouse,
    },
    /// Pointer button press or release.
    PointerButton {
        /// Pointer X in surface coordinates.
        x: i32,
        /// Pointer Y in surface coordinates.
        y: i32,
        /// Linux evdev button code.
        button: u32,
        /// `true` on press, `false` on release.
        pressed: bool,
    },
    /// Vertical pointer scroll step.
    PointerScroll {
        /// Pointer X in surface coordinates.
        x: i32,
        /// Pointer Y in surface coordinates.
        y: i32,
        /// Scroll delta (positive scrolls up, negative scrolls down).
        delta: f64,
    },
    /// Surface expose / redraw request.
    Expose,
}

/// Maps a Linux evdev mouse button code to an [`AspectVKeyMouse`] bit.
pub(crate) fn evdev_button_to_mouse(button: u32) -> AspectVKeyMouse {
    match button {
        BTN_LEFT => ASPECT_VKEY_MOUSE_LEFT_BUTTON,
        BTN_MIDDLE => ASPECT_VKEY_MOUSE_MIDDLE_BUTTON,
        BTN_RIGHT => ASPECT_VKEY_MOUSE_RIGHT_BUTTON,
        _ => ASPECT_VKEY_MOUSE_NONE,
    }
}

/// Maps an XKB keysym to an [`AspectVKey`] using the shared XKB keysym table.
pub(crate) fn virtual_key_from_keysym(keysym: u32) -> AspectVKey {
    XwWindow::virtual_key_from_native(u64::from(keysym))
}

/// Dispatches one decoded [`WlEvent`] to `listener`.
pub(crate) fn dispatch_event(
    listener: &mut dyn AspectWindowInputListener,
    event: WlEvent,
) -> Result<bool, Failure> {
    match event {
        WlEvent::Close => {
            listener.process_close();
            Ok(true)
        }
        WlEvent::Configure { .. } => {
            listener.process_configure(true);
            Ok(true)
        }
        WlEvent::Focus(gained) => {
            listener.process_focus(gained);
            Ok(true)
        }
        WlEvent::Expose => {
            listener.process_expose();
            Ok(true)
        }
        WlEvent::Key { keysym, pressed } => dispatch_key(listener, keysym, pressed),
        WlEvent::PointerMotion { x, y, buttons } => dispatch_motion(listener, x, y, buttons),
        WlEvent::PointerButton {
            x,
            y,
            button,
            pressed,
        } => dispatch_button(listener, x, y, button, pressed),
        WlEvent::PointerScroll { x, y, delta } => dispatch_scroll(listener, x, y, delta),
    }
}

fn modifier_flags(listener: &dyn AspectWindowInputListener) -> Result<AspectVKeyFlags, Failure> {
    let mut flags = ASPECT_VKEY_FLAGS_NONE;
    if listener.keys().is_key_down(K::Control as AspectVKey)? {
        flags |= ASPECT_VKEY_FLAGS_CTRL;
    }
    if listener.keys().is_key_down(K::Shift as AspectVKey)? {
        flags |= ASPECT_VKEY_FLAGS_SHIFT;
    }
    if listener.keys().is_key_down(K::Alt as AspectVKey)? {
        flags |= ASPECT_VKEY_FLAGS_ALT;
    }
    Ok(flags)
}

fn dispatch_key(
    listener: &mut dyn AspectWindowInputListener,
    keysym: u32,
    pressed: bool,
) -> Result<bool, Failure> {
    let key = virtual_key_from_keysym(keysym);
    if key == K::Unknown as AspectVKey {
        return Ok(true);
    }
    let time = listener.event_time();
    if pressed {
        listener.key_down(key, time, 1.0)?;
    } else {
        listener.key_up(key, time)?;
    }
    listener.process_input();
    Ok(true)
}

fn dispatch_motion(
    listener: &mut dyn AspectWindowInputListener,
    x: i32,
    y: i32,
    buttons: AspectVKeyMouse,
) -> Result<bool, Failure> {
    let flags = modifier_flags(listener)?;
    listener.update_mouse_position(Vec2::new(x, y), buttons, flags, false)?;
    listener.process_input();
    Ok(true)
}

fn dispatch_button(
    listener: &mut dyn AspectWindowInputListener,
    x: i32,
    y: i32,
    button: u32,
    pressed: bool,
) -> Result<bool, Failure> {
    let flags = modifier_flags(listener)?;
    let point = Vec2::new(x, y);
    let mapped = evdev_button_to_mouse(button);
    if pressed {
        listener.press_mouse_button(point, mapped, flags, false)?;
    } else {
        listener.release_mouse_button(point, mapped, flags, false)?;
    }
    listener.process_input();
    Ok(true)
}

fn dispatch_scroll(
    listener: &mut dyn AspectWindowInputListener,
    x: i32,
    y: i32,
    delta: f64,
) -> Result<bool, Failure> {
    let flags = modifier_flags(listener)?;
    listener.update_mouse_scroll(&AspectScrollDelta::new(Vec2::new(x, y), delta, flags));
    listener.process_input();
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evdev_buttons_map_to_aspect_mouse_flags() {
        assert_eq!(
            evdev_button_to_mouse(BTN_LEFT),
            ASPECT_VKEY_MOUSE_LEFT_BUTTON
        );
        assert_eq!(
            evdev_button_to_mouse(BTN_MIDDLE),
            ASPECT_VKEY_MOUSE_MIDDLE_BUTTON
        );
        assert_eq!(
            evdev_button_to_mouse(BTN_RIGHT),
            ASPECT_VKEY_MOUSE_RIGHT_BUTTON
        );
        assert_eq!(evdev_button_to_mouse(0), ASPECT_VKEY_MOUSE_NONE);
    }

    #[test]
    fn xkb_keysyms_map_to_aspect_virtual_keys() {
        assert_eq!(virtual_key_from_keysym(0x61), K::A as AspectVKey);
        assert_eq!(virtual_key_from_keysym(0xFF1B), K::Escape as AspectVKey);
        assert_eq!(virtual_key_from_keysym(0xFF0D), K::Enter as AspectVKey);
    }
}
