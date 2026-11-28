//! `RecordingAppKit`: a scripted AppKit for the Cocoa window tests on every platform.
use super::appkit::{CocoaAppKitCalls, CocoaCreatedWindow, CocoaRect};
use std::collections::BTreeMap;
use std::sync::Mutex;

/// The object addresses the fake hands out.
pub(super) const FAKE_WINDOW: usize = 0x1000;
pub(super) const FAKE_VIEW: usize = 0x2000;
pub(super) const FAKE_LAYER: usize = 0x3000;

/// What the fake answers and what it has been asked.
#[derive(Debug, Default)]
pub(super) struct RecordingAppKitState {
    /// `false` makes every query answer "cannot tell", like AppKit off the main thread.
    pub main_thread: bool,
    pub application: bool,
    pub screen_height: f64,
    pub bounds: Option<CocoaRect>,
    pub frame: Option<CocoaRect>,
    pub visible: bool,
    pub refuse_creation: bool,
    /// Reference counts by address; created objects start at 1.
    pub references: BTreeMap<usize, i64>,
    /// Every mutation, in call order.
    pub log: Vec<String>,
}

/// A scripted [`CocoaAppKitCalls`]; `RecordingAppKit::on_main_thread()`.
#[derive(Debug, Default)]
pub(super) struct RecordingAppKit {
    pub state: Mutex<RecordingAppKitState>,
}

impl RecordingAppKit {
    /// The main thread of an application with a 1080-point screen and a 640x480 view.
    pub(super) fn on_main_thread() -> Self {
        let state = RecordingAppKitState {
            main_thread: true,
            application: true,
            screen_height: 1080.0,
            bounds: Some(CocoaRect::new(0.0, 0.0, 640.0, 480.0)),
            ..RecordingAppKitState::default()
        };
        Self {
            state: Mutex::new(state),
        }
    }
    /// Runs `edit` on the scripted state.
    pub(super) fn with<R>(&self, edit: impl FnOnce(&mut RecordingAppKitState) -> R) -> R {
        edit(&mut self.state.lock().expect("the fake state is never poisoned"))
    }
    fn record(&self, entry: String) {
        self.with(|state| state.log.push(entry));
    }
    fn count(&self, object: usize, delta: i64) {
        self.with(|state| *state.references.entry(object).or_default() += delta);
    }
    /// The reference count of `object`.
    pub(super) fn references(&self, object: usize) -> i64 {
        self.with(|state| state.references.get(&object).copied().unwrap_or(0))
    }
    /// The mutation log.
    pub(super) fn log(&self) -> Vec<String> {
        self.with(|state| state.log.clone())
    }
    /// A query answer, withheld off the main thread.
    fn answer<T>(&self, read: impl FnOnce(&RecordingAppKitState) -> Option<T>) -> Option<T> {
        self.with(|state| if state.main_thread { read(state) } else { None })
    }
}

impl CocoaAppKitCalls for RecordingAppKit {
    fn is_main_thread(&self) -> bool {
        self.with(|state| state.main_thread)
    }
    fn has_application(&self) -> bool {
        self.with(|state| state.application)
    }
    fn main_screen_height(&self) -> Option<f64> {
        self.answer(|state| Some(state.screen_height))
    }
    fn create_window(&self, title: &str, frame: CocoaRect) -> Option<CocoaCreatedWindow> {
        self.record(format!("create {title} {frame:?}"));
        if self.with(|state| state.refuse_creation) {
            return None;
        }
        self.count(FAKE_WINDOW, 1);
        self.count(FAKE_VIEW, 1);
        Some(CocoaCreatedWindow {
            window: FAKE_WINDOW,
            view: FAKE_VIEW,
        })
    }
    fn close_window(&self, window: usize) {
        self.record(format!("close {window:#x}"));
    }
    fn retain_object(&self, object: usize) {
        self.count(object, 1);
    }
    fn release_object(&self, object: usize) {
        self.count(object, -1);
    }
    fn is_window_visible(&self, _view: usize) -> Option<bool> {
        self.answer(|state| Some(state.visible))
    }
    fn order_window(&self, view: usize, front: bool) {
        self.record(format!("order {view:#x} front={front}"));
    }
    fn backing_bounds(&self, _view: usize) -> Option<CocoaRect> {
        self.answer(|state| state.bounds)
    }
    fn window_frame(&self, _view: usize) -> Option<CocoaRect> {
        self.answer(|state| state.frame)
    }
    fn set_window_title(&self, view: usize, title: &str) {
        self.record(format!("title {view:#x} {title}"));
    }
    fn invalidate_view(&self, view: usize) {
        self.record(format!("invalidate {view:#x}"));
    }
    fn metal_layer(&self, view: usize) -> Option<usize> {
        let layer = self.answer(|_| Some(FAKE_LAYER))?;
        self.record(format!("metal layer {view:#x}"));
        self.count(layer, 1);
        Some(layer)
    }
}
