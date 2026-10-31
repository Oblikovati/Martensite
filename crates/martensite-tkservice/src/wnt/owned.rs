//! Each owned HWND has a message-pumping owner thread, including during destruction (#334).
#![allow(unsafe_code)]
use martensite_tkernel::standard::Failure;
use std::{
    sync::{Arc, Mutex, OnceLock, mpsc},
    thread::JoinHandle,
    time::Duration,
};
use winsafe::{HINSTANCE, HWND, co};

const CLASS: &str = "Martensite.WNT.Window.334";
#[cfg(test)]
mod tests;

pub(super) struct WindowOwner {
    stop: mpsc::Sender<()>,
    thread: Option<JoinHandle<()>>,
    raw_input: Arc<Mutex<super::WntRawInputBatch>>,
}

impl WindowOwner {
    pub(super) fn create(title: &str, rect: [i32; 4]) -> Result<(Self, usize), Failure> {
        let (stop, stopped) = mpsc::channel();
        let (ready, receiver) = mpsc::channel();
        let title = title.to_owned();
        let raw_input = Arc::new(Mutex::new(super::WntRawInputBatch::default()));
        let queue = Arc::clone(&raw_input);
        let thread = std::thread::Builder::new()
            .name("martensite-win32".into())
            .spawn(move || run(title, rect, ready, stopped, queue))
            .map_err(|error| failure(&format!("owner thread creation failed: {error}")))?;
        let owner = Self {
            stop,
            thread: Some(thread),
            raw_input,
        };
        let handle = receiver
            .recv()
            .map_err(|error| failure(&format!("owner startup failed: {error}")))?
            .map_err(|code| failure(&format!("native creation failed with Win32 error {code}")))?;
        Ok((owner, handle))
    }
    pub(super) fn take_raw_input(&self) -> super::WntRawInputBatch {
        std::mem::take(
            &mut *self
                .raw_input
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner),
        )
    }
}

impl Drop for WindowOwner {
    fn drop(&mut self) {
        let _ = self.stop.send(());
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn failure(cause: &str) -> Failure {
    Failure::with_type(
        "Standard_ConstructionError",
        &format!("WNT_Window: {cause}; expected a native window with an active owner thread"),
        "",
    )
}

fn run(
    title: String,
    rect: [i32; 4],
    ready: mpsc::Sender<Result<usize, u32>>,
    stop: mpsc::Receiver<()>,
    raw_input: Arc<Mutex<super::WntRawInputBatch>>,
) {
    let window = match create_window(&title, rect) {
        Ok(window) => window,
        Err(error) => {
            let _ = ready.send(Err(error.raw()));
            return;
        }
    };
    if ready.send(Ok(window.ptr() as usize)).is_ok() {
        pump(&window, stop, &raw_input);
    }
    if window.IsWindow() {
        let _ = window.DestroyWindow();
    }
}

fn pump(window: &HWND, stop: mpsc::Receiver<()>, raw_input: &Mutex<super::WntRawInputBatch>) {
    while window.IsWindow() {
        match stop.recv_timeout(Duration::from_millis(5)) {
            Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => return,
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
        dispatch_pending(raw_input);
    }
}

fn dispatch_pending(raw_input: &Mutex<super::WntRawInputBatch>) {
    let mut message = winsafe::MSG::default();
    for _ in 0..64 {
        if !winsafe::PeekMessage(&mut message, None, 0, 0, co::PM::REMOVE) {
            break;
        }
        winsafe::TranslateMessage(&message);
        if message.message == co::WM::INPUT {
            let packet = super::raw_input::read(message.lParam as usize);
            raw_input
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .push(packet);
        }
        // SAFETY: this message was retrieved from this thread's native queue;
        // its window procedure remains registered for the process lifetime.
        unsafe { winsafe::DispatchMessage(&message) };
    }
}

fn create_window(title: &str, [left, top, width, height]: [i32; 4]) -> winsafe::SysResult<HWND> {
    register_class()?;
    let instance = HINSTANCE::GetModuleHandle(None)?;
    // SAFETY: the registered procedure has static lifetime, the class and title
    // buffers live through creation, and no application pointer is passed.
    unsafe {
        HWND::CreateWindowEx(
            co::WS_EX::default(),
            winsafe::AtomStr::from_str(CLASS),
            Some(title),
            co::WS::OVERLAPPEDWINDOW | co::WS::CLIPCHILDREN | co::WS::CLIPSIBLINGS,
            winsafe::POINT { x: left, y: top },
            winsafe::SIZE {
                cx: width,
                cy: height,
            },
            None,
            winsafe::IdMenu::None,
            &instance,
            None,
        )
    }
}

fn register_class() -> winsafe::SysResult<()> {
    static REGISTERED: OnceLock<winsafe::SysResult<()>> = OnceLock::new();
    *REGISTERED.get_or_init(|| {
        let mut name = winsafe::WString::from_str(CLASS);
        let mut class = winsafe::WNDCLASSEX::default();
        class.hInstance = HINSTANCE::GetModuleHandle(None)?;
        class.lpfnWndProc = Some(window_proc);
        class.style = co::CS::OWNDC;
        class.set_lpszClassName(Some(&mut name));
        winsafe::SetLastError(co::ERROR::SUCCESS);
        // SAFETY: the callback is a static function and the native registration
        // copies the class name. The executable module outlives every window.
        unsafe { winsafe::RegisterClassEx(&class) }.map(|_| ())
    })
}

extern "system" fn window_proc(
    window: HWND,
    message: co::WM,
    wparam: usize,
    lparam: isize,
) -> isize {
    // SAFETY: Win32 supplied this HWND and message's native parameters to its
    // registered callback. No callback state or Rust allocation is dereferenced.
    unsafe {
        window.DefWindowProc(winsafe::msg::Wm {
            msg_id: message,
            wparam,
            lparam,
        })
    }
}
