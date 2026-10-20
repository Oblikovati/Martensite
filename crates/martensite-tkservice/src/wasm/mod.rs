//! Wasm: the Emscripten canvas window as the Linux library builds it,
//! without Emscripten: the window keeps its size and
//! mapping, and the event translators answer as their non-Emscripten bodies do.
mod window;

pub use window::{
    EmscriptenFocusEvent, EmscriptenKeyboardEvent, EmscriptenMouseEvent, EmscriptenTouchEvent,
    EmscriptenUiEvent, EmscriptenWheelEvent, WasmWindow, WasmWindowFields,
};
