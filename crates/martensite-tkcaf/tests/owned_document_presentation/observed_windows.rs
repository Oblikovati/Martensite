//! Thread-confined named proxy preserves actual memory window storage and observes command boundary order.
use martensite_raster_core::{
    MemoryWindowSystem, PixelSegment, PointerEvent as DrawWindowEvent, RasterWindow as DrawWindow,
    RasterWindowSystem as DrawWindowSystem,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

type SharedWindow = Rc<RefCell<Box<dyn DrawWindow>>>;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum WindowAction {
    Clear(usize),
    Redraw(usize),
    Title(usize, String),
    Flush,
    Save(usize),
}

/// Actual windows and ordered events stay on the creating test thread.
pub(super) struct ObservedWindows {
    inner: MemoryWindowSystem,
    windows: RefCell<Vec<SharedWindow>>,
    pub(super) actions: Rc<RefCell<Vec<WindowAction>>>,
    observe_saves: Rc<Cell<bool>>,
}

impl ObservedWindows {
    /// Connect the actual window boundary with observation only; `ObservedWindows::connected()`.
    pub(super) fn connected() -> Rc<Self> {
        let _: &(dyn Send + Sync) = &WindowAction::Flush;
        let inner = MemoryWindowSystem::new();
        Rc::new(Self {
            inner,
            windows: RefCell::default(),
            actions: Rc::default(),
            observe_saves: Rc::default(),
        })
    }

    /// Observe direct saves only when a capture-order control opts in; `system.observe_saves()`.
    #[allow(dead_code)] // Only the dedicated owned-document capture-order host opts in.
    pub(super) fn observe_saves(&self) {
        // Existing scene controls keep their original event stream until explicitly selected.
        self.observe_saves.set(true);
    }

    /// Read the window buffer directly, without any viewer repaint; `system.pixels(1)`.
    pub(super) fn pixels(&self, index: usize) -> Vec<u8> {
        let path = std::env::temp_dir().join(format!(
            "caf-window-{}-{self:p}-{index}.ppm",
            std::process::id()
        ));
        assert!(
            self.windows.borrow()[index]
                .borrow()
                .save(path.to_str().unwrap())
        );
        let bytes = std::fs::read(&path).unwrap();
        std::fs::remove_file(path).unwrap();
        bytes
    }
}

impl DrawWindowSystem for ObservedWindows {
    fn create_window(
        &self,
        title: &str,
        position: (i32, i32),
        size: (i32, i32),
        existing: u64,
    ) -> Box<dyn DrawWindow> {
        let actual = self.inner.create_window(title, position, size, existing);
        let window = Rc::new(RefCell::new(actual));
        let index = self.windows.borrow().len();
        self.windows.borrow_mut().push(window.clone());
        Box::new(ObservedWindow {
            window,
            index,
            actions: self.actions.clone(),
            observe_saves: self.observe_saves.clone(),
        })
    }

    fn define_color(&self, index: usize, name: &str) -> bool {
        let system = &self.inner;
        system.define_color(index, name)
    }

    fn flush(&self) {
        self.actions.borrow_mut().push(WindowAction::Flush);
        self.inner.flush();
    }

    fn next_event(&self) -> DrawWindowEvent {
        let system = &self.inner;
        system.next_event()
    }
}

struct ObservedWindow {
    window: SharedWindow,
    index: usize,
    actions: Rc<RefCell<Vec<WindowAction>>>,
    observe_saves: Rc<Cell<bool>>,
}

macro_rules! forward_read {
    ($(fn $name:ident($($argument:ident:$kind:ty),*) -> $result:ty;)*) => {$ (
        fn $name(&self, $($argument:$kind),*) -> $result {
            let window = self.window.borrow();
            window.$name($($argument),*)
        }
    )*};
}

macro_rules! forward_write {
    ($(fn $name:ident($($argument:ident:$kind:ty),*) -> $result:ty;)*) => {$ (
        fn $name(&mut self, $($argument:$kind),*) -> $result {
            let mut window = self.window.borrow_mut();
            window.$name($($argument),*)
        }
    )*};
}

impl DrawWindow for ObservedWindow {
    forward_read! {
        fn position() -> (i32,i32);
        fn width() -> i32;
        fn height() -> i32;
        fn title() -> String;
        fn is_mapped() -> bool;
        fn is_window(window:u64) -> bool;
    }
    forward_write! {
        fn set_position(x:i32,y:i32) -> ();
        fn set_dimension(width:i32,height:i32) -> ();
        fn display() -> ();
        fn hide() -> ();
        fn set_color(color:i32) -> ();
        fn set_raster_op(mode:i32) -> ();
        fn draw_string(x:i32,y:i32,text:&str) -> ();
        fn draw_segments(segments:&[PixelSegment]) -> ();
        fn wait(to_wait:bool) -> ();
    }

    fn save(&self, file_name: &str) -> bool {
        if self.observe_saves.get() {
            self.actions
                .borrow_mut()
                .push(WindowAction::Save(self.index));
        }
        self.window.borrow().save(file_name)
    }

    fn clear(&mut self) {
        self.actions
            .borrow_mut()
            .push(WindowAction::Clear(self.index));
        self.window.borrow_mut().clear();
    }

    fn redraw(&mut self) {
        self.actions
            .borrow_mut()
            .push(WindowAction::Redraw(self.index));
        self.window.borrow_mut().redraw();
    }

    fn set_title(&mut self, title: &str) {
        self.actions
            .borrow_mut()
            .push(WindowAction::Title(self.index, title.into()));
        self.window.borrow_mut().set_title(title);
    }
}
