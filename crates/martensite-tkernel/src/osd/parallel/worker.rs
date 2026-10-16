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
    // Fields drop in declaration order: signal shutdown before joining threads.
    pool: rayon::ThreadPool,
    _joined_threads: JoinedThreads,
}

struct JoinedThreads(Vec<JoinHandle<()>>);

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|poison| poison.into_inner())
}

impl ScopedWorker {
    fn new() -> Result<Self, rayon::ThreadPoolBuildError> {
        let mut threads = JoinedThreads(Vec::new());
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .spawn_handler(|thread| {
                threads
                    .0
                    .push(std::thread::Builder::new().spawn(|| thread.run())?);
                Ok(())
            })
            .build()?;
        Ok(Self {
            pool,
            _joined_threads: threads,
        })
    }
}

impl Drop for JoinedThreads {
    fn drop(&mut self) {
        for thread in self.0.drain(..) {
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
    let Some((caller, workers)) = workers.split_last() else {
        return Vec::new();
    };
    let (ready, mut failures) = prepare_workers(workers);
    failures.extend(dispatch_tree(&ready, caller, body));
    failures.sort_by_key(|(index, _)| *index);
    failures.into_iter().map(|(_, failure)| failure).collect()
}

struct PreparedWorker<'a> {
    worker: &'a Worker,
    executor: Arc<ScopedWorker>,
}

type IndexedFailures = Vec<(i32, Failure)>;
type PreparedWorkers<'a> = smallvec::SmallVec<[PreparedWorker<'a>; 32]>;

fn prepare_workers(workers: &[Arc<Worker>]) -> (PreparedWorkers<'_>, IndexedFailures) {
    let mut ready = PreparedWorkers::with_capacity(workers.len());
    let mut failures = Vec::new();
    for worker in workers {
        match worker.executor() {
            Ok(executor) => ready.push(PreparedWorker { worker, executor }),
            Err(failure) => failures.push((worker.index.load(Ordering::SeqCst), failure)),
        }
    }
    (ready, failures)
}

fn dispatch_tree(
    workers: &[PreparedWorker<'_>],
    caller: &Worker,
    body: &JobBody<'_>,
) -> IndexedFailures {
    let Some(owner) = workers.get(workers.len() / 2) else {
        return participant_failure(caller, body);
    };
    let middle = workers.len() / 2;
    // Each branch keeps its final participant on the current physical thread (#109).
    let mut left = Vec::new();
    let mut right = Vec::new();
    owner.executor.pool.in_place_scope(|scope| {
        scope.spawn(|_| left = dispatch_tree(&workers[..middle], owner.worker, body));
        right = dispatch_tree(&workers[middle + 1..], caller, body);
    });
    left.extend(right);
    left
}

fn participant_failure(worker: &Worker, body: &JobBody<'_>) -> IndexedFailures {
    worker
        .perform(body)
        .map(|failure| (worker.index.load(Ordering::SeqCst), failure))
        .into_iter()
        .collect()
}
