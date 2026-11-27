//! Platform thread-spawning seam and pre-warmed WebAssembly Web Worker pool (#631).
//!
//! Native targets spawn OS threads via `std::thread::Builder`. Browser
//! `wasm32-unknown-unknown` (`+atomics`) cannot spawn OS threads synchronously;
//! instead, the Orchestrator Worker calls [`init_wasm_worker_pool`] asynchronously
//! before any synchronous parallel kernel operation runs. Pre-warmed Compute Web
//! Workers park on a shared-memory `Condvar` and execute submitted closures.

#[cfg(not(target_arch = "wasm32"))]
use std::thread::JoinHandle;
#[cfg(target_arch = "wasm32")]
use std::{
    collections::VecDeque,
    sync::{
        Arc, Condvar, Mutex, OnceLock,
        atomic::{AtomicUsize, Ordering},
    },
};

/// Platform-neutral join handle used by [`super::worker::Worker`] and [`super::thread::OsdThread`].
pub(super) struct PlatformJoinHandle<T> {
    #[cfg(not(target_arch = "wasm32"))]
    inner: JoinHandle<T>,
    #[cfg(target_arch = "wasm32")]
    state: Arc<WasmTaskState<T>>,
}

impl<T: Send + 'static> PlatformJoinHandle<T> {
    /// Joins the spawned task and returns its output, or `Err` on abnormal exit.
    pub(super) fn join(self) -> Result<T, ()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.inner.join().map_err(|_| ())
        }
        #[cfg(target_arch = "wasm32")]
        {
            let mut guard = self
                .state
                .result
                .lock()
                .unwrap_or_else(|poison| poison.into_inner());
            while guard.is_none() {
                guard = self
                    .state
                    .done
                    .wait(guard)
                    .unwrap_or_else(|poison| poison.into_inner());
            }
            guard.take().ok_or(())
        }
    }

    /// Returns true if the calling thread is the thread executing this handle.
    pub(super) fn is_current_thread(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.inner.thread().id() == std::thread::current().id()
        }
        #[cfg(target_arch = "wasm32")]
        {
            let tid = self.state.thread_id.load(Ordering::Acquire);
            tid != 0 && tid == super::thread::native::current_thread_id()
        }
    }

    /// Returns the platform thread identifier for `OsdThread::id`.
    pub(super) fn native_thread_id(&self) -> usize {
        #[cfg(not(target_arch = "wasm32"))]
        {
            super::thread::native::thread_id(&self.inner)
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.state.thread_id.load(Ordering::Acquire)
        }
    }

    /// Applies OS thread priority where supported (Windows only).
    pub(super) fn apply_priority(&self, priority: i32) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            super::thread::native::apply_priority(&self.inner, priority);
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = priority;
        }
    }
}

/// Spawns `body` on a platform worker thread (OS thread on native, pre-warmed
/// Web Worker slot on `wasm32`). Returns `Err(reason)` if no worker is available.
pub(super) fn spawn_platform_thread<F, T>(
    stack_size: i32,
    body: F,
) -> Result<PlatformJoinHandle<T>, String>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    #[cfg(not(target_arch = "wasm32"))]
    {
        let builder = thread_builder(stack_size);
        builder
            .spawn(body)
            .map(|inner| PlatformJoinHandle { inner })
            .map_err(|error| error.to_string())
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = stack_size;
        WasmWorkerDispatch::global().try_spawn(body)
    }
}

#[cfg(windows)]
fn thread_builder(stack_size: i32) -> std::thread::Builder {
    let builder: std::thread::Builder = std::thread::Builder::new();
    match usize::try_from(stack_size) {
        Ok(size) if size > 0 => builder.stack_size(size),
        _ => builder,
    }
}

#[cfg(all(not(windows), not(target_arch = "wasm32")))]
fn thread_builder(_stack_size: i32) -> std::thread::Builder {
    std::thread::Builder::new()
}

#[cfg(target_arch = "wasm32")]
struct WasmTaskState<T> {
    result: Mutex<Option<T>>,
    done: Condvar,
    thread_id: AtomicUsize,
}

#[cfg(target_arch = "wasm32")]
struct DispatchQueue {
    pending: VecDeque<Box<dyn FnOnce() + Send + 'static>>,
    ready_workers: usize,
    busy_workers: usize,
    shutdown: bool,
}

#[cfg(target_arch = "wasm32")]
struct WasmWorkerDispatch {
    queue: Mutex<DispatchQueue>,
    available: Condvar,
}

#[cfg(target_arch = "wasm32")]
impl WasmWorkerDispatch {
    fn global() -> &'static Self {
        static DISPATCH: OnceLock<WasmWorkerDispatch> = OnceLock::new();
        DISPATCH.get_or_init(|| Self {
            queue: Mutex::new(DispatchQueue {
                pending: VecDeque::new(),
                ready_workers: 0,
                busy_workers: 0,
                shutdown: false,
            }),
            available: Condvar::new(),
        })
    }

    fn ready_worker_count(&self) -> usize {
        self.queue
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .ready_workers
    }

    fn register_worker(&self) {
        let mut guard = self
            .queue
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        guard.shutdown = false;
        guard.ready_workers += 1;
    }

    fn try_spawn<F, T>(&self, body: F) -> Result<PlatformJoinHandle<T>, String>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let mut guard = self
            .queue
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        let occupied = guard.busy_workers + guard.pending.len();
        if guard.ready_workers == 0 || occupied >= guard.ready_workers {
            return Err(format!(
                "no idle pre-warmed wasm worker available (ready={}, busy={}, queued={})",
                guard.ready_workers,
                guard.busy_workers,
                guard.pending.len()
            ));
        }
        let state = Arc::new(WasmTaskState {
            result: Mutex::new(None),
            done: Condvar::new(),
            thread_id: AtomicUsize::new(0),
        });
        let task_state = Arc::clone(&state);
        guard.pending.push_back(Box::new(move || {
            task_state.thread_id.store(
                super::thread::native::current_thread_id(),
                Ordering::Release,
            );
            let output = body();
            let mut slot = task_state
                .result
                .lock()
                .unwrap_or_else(|poison| poison.into_inner());
            *slot = Some(output);
            task_state.done.notify_all();
        }));
        self.available.notify_one();
        Ok(PlatformJoinHandle { state })
    }

    fn run_worker_loop(&self) {
        loop {
            let task = {
                let mut guard = self
                    .queue
                    .lock()
                    .unwrap_or_else(|poison| poison.into_inner());
                while guard.pending.is_empty() && !guard.shutdown {
                    guard = self
                        .available
                        .wait(guard)
                        .unwrap_or_else(|poison| poison.into_inner());
                }
                if guard.shutdown && guard.pending.is_empty() {
                    guard.ready_workers = guard.ready_workers.saturating_sub(1);
                    return;
                }
                let next = guard.pending.pop_front();
                if next.is_some() {
                    guard.busy_workers += 1;
                }
                next
            };
            if let Some(task) = task {
                task();
                let mut guard = self
                    .queue
                    .lock()
                    .unwrap_or_else(|poison| poison.into_inner());
                guard.busy_workers = guard.busy_workers.saturating_sub(1);
            }
        }
    }

    fn signal_shutdown(&self) {
        let mut guard = self
            .queue
            .lock()
            .unwrap_or_else(|poison| poison.into_inner());
        guard.shutdown = true;
        self.available.notify_all();
    }
}

/// Returns the number of pre-warmed browser Compute Workers registered in the
/// shared-memory pool (0 before [`init_wasm_worker_pool`] completes or on native).
///
/// ```
/// assert_eq!(martensite_tkernel::osd::wasm_worker_pool_size(), 0);
/// ```
pub fn wasm_worker_pool_size() -> usize {
    #[cfg(target_arch = "wasm32")]
    {
        WasmWorkerDispatch::global().ready_worker_count()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm_browser_pool {
    use super::WasmWorkerDispatch;
    use crate::osd::parallel::ThreadPool;
    use crate::standard::Failure;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = "
export function martensite_wasm_glue_url() {
    return import.meta.url;
}
")]
    extern "C" {
        fn martensite_wasm_glue_url() -> String;
    }

    /// Registers a Compute Web Worker in the shared-memory pool before entering its loop.
    #[wasm_bindgen]
    pub fn martensite_wasm_worker_register() {
        WasmWorkerDispatch::global().register_worker();
    }

    /// Enters the blocking Compute Web Worker dispatch loop on shared `WebAssembly.Memory`.
    #[wasm_bindgen]
    pub fn martensite_wasm_worker_entry() {
        WasmWorkerDispatch::global().run_worker_loop();
    }

    /// Owns the pre-warmed Compute Web Workers created by [`init_wasm_worker_pool`].
    ///
    /// Dropping or calling [`WasmWorkerPool::terminate`] shuts down and terminates
    /// all browser workers on the owning Orchestrator Worker realm.
    pub struct WasmWorkerPool {
        workers: Vec<web_sys::Worker>,
        blob_url: Option<String>,
    }

    impl WasmWorkerPool {
        /// Returns the number of pre-warmed Compute Web Workers managed by this pool.
        pub fn worker_count(&self) -> usize {
            self.workers.len()
        }

        /// Shuts down and terminates every Compute Web Worker in this pool.
        pub fn terminate(mut self) {
            self.terminate_inner();
        }

        fn terminate_inner(&mut self) {
            WasmWorkerDispatch::global().signal_shutdown();
            for worker in self.workers.drain(..) {
                worker.terminate();
            }
            if let Some(url) = self.blob_url.take() {
                let _ = web_sys::Url::revoke_object_url(&url);
            }
        }
    }

    impl Drop for WasmWorkerPool {
        fn drop(&mut self) {
            self.terminate_inner();
        }
    }

    /// Spawns and handshakes `worker_count` pre-warmed Compute Web Workers sharing
    /// the current `WebAssembly.Module` and `WebAssembly.Memory` (`+atomics`).
    ///
    /// Must be awaited asynchronously from the Orchestrator Worker before calling
    /// synchronous `OSD_Parallel`, `ThreadPool`, BVH, or `BRepCheck` operations.
    pub async fn init_wasm_worker_pool(worker_count: usize) -> Result<WasmWorkerPool, Failure> {
        let glue_url = martensite_wasm_glue_url();
        init_wasm_worker_pool_from_url(worker_count, &glue_url).await
    }

    /// Spawns and handshakes `worker_count` pre-warmed Compute Web Workers using an
    /// explicit `wasm-bindgen` ES module glue URL.
    pub async fn init_wasm_worker_pool_from_url(
        worker_count: usize,
        glue_url: &str,
    ) -> Result<WasmWorkerPool, Failure> {
        if worker_count == 0 {
            return Ok(WasmWorkerPool {
                workers: Vec::new(),
                blob_url: None,
            });
        }
        let blob_url = create_bootstrap_blob_url()?;
        let mut workers = Vec::with_capacity(worker_count);
        let mut ready_promises = Vec::with_capacity(worker_count);
        for index in 0..worker_count {
            let (worker, ready) = spawn_one_worker(&blob_url, glue_url, index)?;
            workers.push(worker);
            ready_promises.push(ready);
        }
        for promise in ready_promises {
            wasm_bindgen_futures::JsFuture::from(promise)
                .await
                .map_err(|error| {
                    pool_failure(&format!("wasm compute worker failed to start: {error:?}"))
                })?;
        }
        let total_participants = i32::try_from(worker_count.saturating_add(1)).unwrap_or(i32::MAX);
        let _ = ThreadPool::default_pool(total_participants).init(total_participants);
        Ok(WasmWorkerPool {
            workers,
            blob_url: Some(blob_url),
        })
    }

    fn create_bootstrap_blob_url() -> Result<String, Failure> {
        const BOOTSTRAP: &str = r#"
self.onmessage = async (event) => {
    try {
        const { glueUrl, module, memory } = event.data;
        const wasm = await import(glueUrl);
        await wasm.default({ module_or_path: module, memory });
        wasm.martensite_wasm_worker_register();
        self.postMessage({ type: "ready" });
        wasm.martensite_wasm_worker_entry();
    } catch (err) {
        self.postMessage({ type: "error", message: String(err && err.stack || err) });
    }
};
"#;
        let parts = js_sys::Array::of1(&JsValue::from_str(BOOTSTRAP));
        let bag = web_sys::BlobPropertyBag::new();
        bag.set_type("text/javascript");
        let blob = web_sys::Blob::new_with_str_sequence_and_options(&parts, &bag)
            .map_err(|error| pool_failure(&format!("failed to create worker Blob: {error:?}")))?;
        web_sys::Url::create_object_url_with_blob(&blob)
            .map_err(|error| pool_failure(&format!("failed to create worker Blob URL: {error:?}")))
    }

    fn spawn_one_worker(
        blob_url: &str,
        glue_url: &str,
        index: usize,
    ) -> Result<(web_sys::Worker, js_sys::Promise), Failure> {
        let options = web_sys::WorkerOptions::new();
        options.set_type(web_sys::WorkerType::Module);
        options.set_name(&format!("martensite-compute-{index}"));
        let worker = web_sys::Worker::new_with_options(blob_url, &options)
            .map_err(|error| pool_failure(&format!("failed to construct Web Worker: {error:?}")))?;
        let ready = ready_promise(&worker);
        let payload = js_sys::Object::new();
        let _ = js_sys::Reflect::set(
            &payload,
            &JsValue::from_str("glueUrl"),
            &JsValue::from_str(glue_url),
        );
        let _ = js_sys::Reflect::set(
            &payload,
            &JsValue::from_str("module"),
            &wasm_bindgen::module(),
        );
        let _ = js_sys::Reflect::set(
            &payload,
            &JsValue::from_str("memory"),
            &wasm_bindgen::memory(),
        );
        worker
            .post_message(&payload)
            .map_err(|error| pool_failure(&format!("failed to post init to worker: {error:?}")))?;
        Ok((worker, ready))
    }

    fn ready_promise(worker: &web_sys::Worker) -> js_sys::Promise {
        js_sys::Promise::new(&mut |resolve, reject| {
            let resolve_msg = resolve.clone();
            let reject_msg = reject.clone();
            let on_message = Closure::once(move |event: web_sys::MessageEvent| {
                let data = event.data();
                let kind = js_sys::Reflect::get(&data, &JsValue::from_str("type"))
                    .ok()
                    .and_then(|v| v.as_string());
                if kind.as_deref() == Some("ready") {
                    let _ = resolve_msg.call0(&JsValue::UNDEFINED);
                } else {
                    let msg =
                        js_sys::Reflect::get(&data, &JsValue::from_str("message")).unwrap_or(data);
                    let _ = reject_msg.call1(&JsValue::UNDEFINED, &msg);
                }
            });
            let on_error = Closure::once(move |event: web_sys::ErrorEvent| {
                let _ = reject.call1(&JsValue::UNDEFINED, &JsValue::from_str(&event.message()));
            });
            worker.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
            worker.set_onerror(Some(on_error.as_ref().unchecked_ref()));
            on_message.forget();
            on_error.forget();
        })
    }

    fn pool_failure(message: &str) -> Failure {
        Failure::with_type("Standard_ProgramError", message, "")
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_browser_pool::{
    WasmWorkerPool, init_wasm_worker_pool, init_wasm_worker_pool_from_url,
    martensite_wasm_worker_entry, martensite_wasm_worker_register,
};
