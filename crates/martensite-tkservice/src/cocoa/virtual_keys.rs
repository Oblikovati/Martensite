//! `Cocoa_Window::VirtualKeyFromNative`: the Carbon virtual key codes 0x00..0x7F.
use crate::aspect::{AspectVKey, AspectVKeyBasic as K};

/// The keys of codes 0x00..0x3F (index = code).
#[rustfmt::skip]
const LOW_KEYS: [K; 64] = [
    K::A, K::S, K::D, K::F, K::H, K::G, K::Z, K::X, // 0x00
    K::C, K::V, K::Unknown, K::B, K::Q, K::W, K::E, K::R, // 0x08
    K::Y, K::T, K::Digit1, K::Digit2, K::Digit3, K::Digit4, K::Digit6, K::Digit5, // 0x10
    K::Plus, K::Digit9, K::Digit7, K::Minus, K::Digit8, K::Digit0, K::BracketRight, K::O, // 0x18
    K::U, K::BracketLeft, K::I, K::P, K::Enter, K::L, K::J, K::Apostrophe, // 0x20
    K::K, K::Semicolon, K::Backslash, K::Comma, K::Slash, K::N, K::M, K::Period, // 0x28
    // 0x36/0x37 are the right/left Command keys, 0x3F is fn
    K::Tab, K::Space, K::Tilde, K::Backspace, K::Unknown, K::Escape, K::Unknown, K::Unknown, // 0x30
    K::Shift, K::Unknown, K::Alt, K::Control, K::Shift, K::Alt, K::Unknown, K::Unknown, // 0x38
];

/// The named keys of codes 0x40..0x7F; every other code there is unknown.
const HIGH_KEYS: [(i32, K); 22] = [
    (0x4C, K::Enter), // fn + return
    (0x60, K::F5),
    (0x61, K::F6),
    (0x62, K::F7),
    (0x63, K::F3),
    (0x64, K::F8),
    (0x65, K::F9),
    (0x67, K::F11),
    (0x6D, K::F10),
    (0x6F, K::F12),
    (0x73, K::Home),
    (0x74, K::PageUp),
    (0x75, K::Delete),
    (0x76, K::F4),
    (0x77, K::End),
    (0x78, K::F2),
    (0x79, K::PageDown),
    (0x7A, K::F1),
    (0x7B, K::Left),
    (0x7C, K::Right),
    (0x7D, K::Down),
    (0x7E, K::Up),
];

/// The Aspect key of a Carbon virtual key code, unknown outside the table;
/// `virtual_key_from_native(0x00)` is `A`.
pub(super) fn virtual_key_from_native(key: i32) -> AspectVKey {
    let low = usize::try_from(key)
        .ok()
        .and_then(|index| LOW_KEYS.get(index));
    let found = low.copied().or_else(|| {
        HIGH_KEYS
            .iter()
            .find(|(code, _)| *code == key)
            .map(|(_, vkey)| *vkey)
    });
    found.unwrap_or(K::Unknown) as AspectVKey
}
