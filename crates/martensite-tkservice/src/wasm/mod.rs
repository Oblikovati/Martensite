//! Wasm: browser canvas windows (`OffscreenCanvas` / `HtmlCanvasElement`),
//! realm-confined handles, and portable DOM input translation (#632).
mod events;
mod realm;
mod window;

pub use events::{
    WasmKeyEvent, WasmKeyEventKind, WasmKeyModifiers, WasmPointerEvent, WasmPointerEventKind,
    WasmResizeEvent, WasmTouchEvent, WasmTouchEventKind, WasmTouchPoint, WasmWheelDeltaMode,
    WasmWheelEvent, virtual_key_from_dom_code,
};
pub use realm::RealmHandle;
#[cfg(target_arch = "wasm32")]
pub use window::WasmCanvasTarget;
pub use window::{
    EmscriptenFocusEvent, EmscriptenKeyboardEvent, EmscriptenMouseEvent, EmscriptenTouchEvent,
    EmscriptenUiEvent, EmscriptenWheelEvent, WasmWindow, WasmWindowFields,
};
