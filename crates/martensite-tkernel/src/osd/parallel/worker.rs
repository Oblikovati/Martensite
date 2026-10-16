//! Reserved, lazily started persistent workers with scoped borrowed dispatch.
use super::job::{self, JobBody, standard_program_error};
use crate::standard::Failure;
use std::{
    sync::{
        Arc, Mutex, MutexGuard,
        atomic::{AtomicI32, Ordering},
    },
    thread::JoinHandle,
};

#[cfg(test)]
mod tests;

pub(super) struct Worker {
    usage: AtomicI32,
    index: AtomicI32,
    executor: Mutex<Option<Arc<ScopedWorker>>>,
    failure: Mutex<Option<Failure>>,
    is_self_thread: bool,
}

struct ScopedWorker {
    pool: Option<rayon::ThreadPool>,
    threads: Vec<JoinHandle<()>>,
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|poison| poison.into_inner())
}

impl ScopedWorker {
    fn new() -> Result<Self, rayon::ThreadPoolBuildError> {
        let mut worker = Self {
            pool: None,
            threads: Vec::new(),
        };
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .spawn_handler(|thread| {
                worker
                    .threads
                    .push(std::thread::Builder::new().spawn(|| thread.run())?);
                Ok(())
            })
            .build()?;
        worker.pool = Some(pool);
        Ok(worker)
    }
}

impl Drop for ScopedWorker {
    fn drop(&mut self) {
        drop(self.pool.take());
        for thread in self.threads.drain(..) {
            let _ = thread.join();
        }
    }
}

impl Worker {
    pub(super) fn new(index: i32, is_self_thread: bool) -> Arc<Self> {
        Arc::new(Self {
            usage: AtomicI32::new(0),
            index: AtomicI32::new(index),
            executor: Mutex::new(None),
            failure: Mutex::new(None),
            is_self_thread,
        })
    }
    pub(super) fn lock(&self) -> bool {
        self.usage.swap(1, Ordering::SeqCst) == 0
    }
    pub(super) fn free(&self) {
        self.usage.store(0, Ordering::SeqCst);
    }
    pub(super) fn set_index(&self, index: i32) {
        self.index.store(index, Ordering::SeqCst);
    }
    pub(super) fn shut_down(&self) {
        let executor = lock(&self.executor).take();
        drop(executor);
    }
    fn executor(&self) -> Result<Arc<ScopedWorker>, Failure> {
        self.executor_with(ScopedWorker::new)
    }
    fn executor_with(
        &self,
        create: impl FnOnce() -> Result<ScopedWorker, rayon::ThreadPoolBuildError>,
    ) -> Result<Arc<ScopedWorker>, Failure> {
        let mut executor = lock(&self.executor);
        if let Some(existing) = executor.as_ref() {
            return Ok(Arc::clone(existing));
        }
        let worker = create().map_err(|error| {
            standard_program_error(
                &format!("Error: unable to start a pool thread ({error})"),
                "",
            )
        })?;
        let worker = Arc::new(worker);
        *executor = Some(Arc::clone(&worker));
        Ok(worker)
    }
    fn perform(&self, body: &JobBody<'_>) -> Option<Failure> {
        let failure = job::perform(body, self.index.load(Ordering::SeqCst));
        if !self.is_self_thread {
            return failure;
        }
        // Preserve the caller's sticky failure, without sharing worker completion (#106).
        let mut previous = lock(&self.failure);
        if failure.is_some() {
            *previous = failure;
        }
        previous.clone()
    }
}

/// Each scope owns its completion, including concurrent and nested launches (#106).
pub(super) fn dispatch(workers: &[Arc<Worker>], body: &JobBody<'_>) -> Vec<Failure> {
    let Some((worker, rest)) = workers.split_first() else {
        return Vec::new();
    };
    if worker.is_self_thread {
        return worker.perform(body).into_iter().collect();
    }
    let executor = match worker.executor() {
        Ok(executor) => executor,
        Err(failure) => {
            let mut failures = vec![failure];
            failures.extend(dispatch(rest, body));
            return failures;
        }
    };
    scoped_dispatch(&executor, worker, rest, body)
}

fn scoped_dispatch(
    executor: &ScopedWorker,
    worker: &Worker,
    rest: &[Arc<Worker>],
    body: &JobBody<'_>,
) -> Vec<Failure> {
    let mut failure = None;
    let mut remaining = Vec::new();
    if let Some(pool) = &executor.pool {
        pool.in_place_scope(|scope| {
            scope.spawn(|_| failure = worker.perform(body));
            remaining = dispatch(rest, body);
        });
    }
    let mut failures: Vec<Failure> = failure.into_iter().collect();
    failures.extend(remaining);
    failures
}
