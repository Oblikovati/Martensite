//! Execute independent certificates with bounded operation-local workers and ordered results.
use super::*;
use martensite_tkernel::osd::Parallel;
use std::cmp::Reverse;
#[cfg(target_arch = "wasm32")]
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
#[cfg(not(target_arch = "wasm32"))]
use std::thread;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
const CANCEL_POLL: Duration = Duration::from_millis(4);

pub(super) fn ordered<I, O, F>(
    execution: Execution,
    stage: CylinderFeatureStage,
    inputs: &[I],
    cancelled: &mut CancelPoll<'_>,
    check: F,
) -> Checked<Vec<O>>
where
    I: Sync,
    O: Send,
    F: Fn(&I, &mut CancelPoll<'_>) -> Checked<O> + Sync,
{
    execute(execution, stage, inputs, None, cancelled, check)
}

pub(super) fn prioritized<I, O, F>(
    execution: Execution,
    stage: CylinderFeatureStage,
    inputs: &[I],
    cost: fn(&I) -> usize,
    cancelled: &mut CancelPoll<'_>,
    check: F,
) -> Checked<Vec<O>>
where
    I: Sync,
    O: Send,
    F: Fn(&I, &mut CancelPoll<'_>) -> Checked<O> + Sync,
{
    execute(execution, stage, inputs, Some(cost), cancelled, check)
}

fn execute<I, O, F>(
    execution: Execution,
    stage: CylinderFeatureStage,
    inputs: &[I],
    cost: Option<fn(&I) -> usize>,
    cancelled: &mut CancelPoll<'_>,
    check: F,
) -> Checked<Vec<O>>
where
    I: Sync,
    O: Send,
    F: Fn(&I, &mut CancelPoll<'_>) -> Checked<O> + Sync,
{
    let attempt = execution.attempt;
    let available = usize::try_from(Parallel::nb_logical_processors())
        .unwrap_or(1)
        .max(1);
    let workers = available
        .min(execution.worker_limit.get())
        .min(inputs.len().div_ceil(2));
    if workers < 2 {
        return inputs
            .iter()
            .map(|input| {
                attempt.poll(cancelled, stage)?;
                check(input, cancelled)
            })
            .collect();
    }
    let dispatch = cost.map(|cost| by_cost(inputs, cost));
    let queue = Queue {
        next: AtomicUsize::new(0),
        stop: AtomicBool::new(false),
        inputs,
        dispatch: dispatch.as_deref(),
        check,
    };
    queue.run(attempt, stage, cancelled, workers)
}

fn by_cost<I>(inputs: &[I], cost: fn(&I) -> usize) -> Vec<usize> {
    let mut indices: Vec<_> = (0..inputs.len()).collect();
    // Stable ties retain canonical source order. The indices remain source ranks.
    indices.sort_by_cached_key(|&index| Reverse(cost(&inputs[index])));
    indices
}

struct Queue<'a, I, F> {
    next: AtomicUsize,
    stop: AtomicBool,
    inputs: &'a [I],
    dispatch: Option<&'a [usize]>,
    check: F,
}

type Findings<O> = (usize, Checked<O>);

impl<I: Sync, F: Sync> Queue<'_, I, F> {
    #[cfg(not(target_arch = "wasm32"))]
    fn run<O>(
        &self,
        attempt: Attempt,
        stage: CylinderFeatureStage,
        cancelled: &mut CancelPoll<'_>,
        count: usize,
    ) -> Checked<Vec<O>>
    where
        I: Sync,
        O: Send,
        F: Fn(&I, &mut CancelPoll<'_>) -> Checked<O> + Sync,
    {
        thread::scope(|scope| {
            let (send, receive) = mpsc::channel();
            let mut workers = Vec::with_capacity(count);
            let mut start = Ok(());
            for _ in 0..count {
                let sender = send.clone();
                match thread::Builder::new().spawn_scoped(scope, || self.worker(sender)) {
                    Ok(worker) => workers.push(worker),
                    Err(cause) => {
                        start = Err(attempt.fail(stage, CylinderFeatureCause::Diagnostic(cause)));
                        break;
                    }
                }
            }
            drop(send);
            let outcome = start.and_then(|()| {
                receive_ordered(attempt, stage, self.inputs.len(), cancelled, &receive)
            });
            self.stop.store(true, Relaxed);
            let mut abnormal = false;
            for worker in workers {
                abnormal |= worker.join().is_err();
            }
            require(
                attempt,
                stage,
                !abnormal,
                "completed operation-local certificate workers",
            )?;
            outcome
        })
    }

    #[cfg(target_arch = "wasm32")]
    fn run<O>(
        &self,
        attempt: Attempt,
        stage: CylinderFeatureStage,
        cancelled: &mut CancelPoll<'_>,
        count: usize,
    ) -> Checked<Vec<O>>
    where
        I: Sync,
        O: Send,
        F: Fn(&I, &mut CancelPoll<'_>) -> Checked<O> + Sync,
    {
        attempt.poll(cancelled, stage)?;
        let batches: Mutex<Vec<Vec<Findings<O>>>> = Mutex::new(Vec::new());
        let pooled = Parallel::scope_or_pool(count, |_| {
            let mut completed = Vec::new();
            while !self.stop.load(Relaxed) {
                let slot = self.next.fetch_add(1, Relaxed);
                if slot >= self.inputs.len() {
                    break;
                }
                let index = self.dispatch.map_or(slot, |order| order[slot]);
                let result = (self.check)(&self.inputs[index], &mut || self.stop.load(Relaxed));
                completed.push((index, result));
            }
            if !completed.is_empty() {
                batches
                    .lock()
                    .unwrap_or_else(|poison| poison.into_inner())
                    .push(completed);
            }
        });
        require(
            attempt,
            stage,
            pooled.is_ok(),
            "completed operation-local certificate workers",
        )?;
        attempt.poll(cancelled, stage)?;
        let mut ordered: Vec<_> = std::iter::repeat_with(|| None)
            .take(self.inputs.len())
            .collect();
        for batch in batches
            .into_inner()
            .unwrap_or_else(|poison| poison.into_inner())
        {
            for (index, result) in batch {
                ordered[index] = Some(result);
            }
        }
        ordered
            .into_iter()
            .map(|slot| {
                slot.ok_or_else(|| missing(attempt, stage, "one result per source task"))?
            })
            .collect()
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn worker<O>(&self, send: Sender<Vec<Findings<O>>>)
    where
        F: Fn(&I, &mut CancelPoll<'_>) -> Checked<O>,
    {
        let mut completed = Vec::new();
        while !self.stop.load(Relaxed) {
            let slot = self.next.fetch_add(1, Relaxed);
            if slot >= self.inputs.len() {
                break;
            }
            let index = self.dispatch.map_or(slot, |order| order[slot]);
            let result = (self.check)(&self.inputs[index], &mut || self.stop.load(Relaxed));
            completed.push((index, result));
        }
        if !completed.is_empty() {
            // Certificates are independent. Return one batch per worker so that a
            // large boundary does not wake the cancel coordinator for every span.
            let _ = send.send(completed);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn receive_ordered<O>(
    attempt: Attempt,
    stage: CylinderFeatureStage,
    count: usize,
    cancelled: &mut CancelPoll<'_>,
    receive: &Receiver<Vec<Findings<O>>>,
) -> Checked<Vec<O>> {
    let mut ordered: Vec<_> = std::iter::repeat_with(|| None).take(count).collect();
    let mut collected = 0;
    while collected < count {
        attempt.poll(cancelled, stage)?;
        match receive.recv_timeout(CANCEL_POLL) {
            Ok(batch) => {
                collected += batch.len();
                for (index, result) in batch {
                    ordered[index] = Some(result);
                }
            }
            Err(RecvTimeoutError::Timeout) => (),
            Err(RecvTimeoutError::Disconnected) => {
                return Err(missing(
                    attempt,
                    stage,
                    "complete result from every independent certificate",
                ));
            }
        }
    }
    ordered
        .into_iter()
        .map(|slot| slot.ok_or_else(|| missing(attempt, stage, "one result per source task"))?)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const ATTEMPT: Attempt = Attempt {
        attachment: FaceAttachmentId(1),
        revision: SupportRevision(7),
        kind: CylinderFeatureKind::Boss,
    };
    const EXECUTION: Execution = Execution {
        attempt: ATTEMPT,
        worker_limit: DEFAULT_WORKER_LIMIT,
    };

    #[test]
    fn preserves_input_order_and_first_typed_failure() {
        let values: Vec<_> = (0..48).collect();
        let results = ordered(
            EXECUTION,
            CylinderFeatureStage::Tool,
            &values,
            &mut || false,
            |i, _| Ok(i * i),
        )
        .unwrap();
        assert_eq!(results, values.iter().map(|i| i * i).collect::<Vec<_>>());
        let fault = ordered(
            EXECUTION,
            CylinderFeatureStage::Tool,
            &values,
            &mut || false,
            |&i, _| match i {
                3 => Err(missing(ATTEMPT, CylinderFeatureStage::Tool, "first source")),
                17 => Err(missing(ATTEMPT, CylinderFeatureStage::Tool, "later source")),
                _ => Ok(i),
            },
        )
        .unwrap_err();
        assert!(matches!(
            *fault.cause,
            CylinderFeatureCause::DirectAttachment {
                expected: "first source"
            }
        ));
    }

    #[test]
    fn heavier_tasks_start_first_with_canonical_ties() {
        let values = [1, 30, 2, 30, 5, 9];
        assert_eq!(by_cost(&values, |&weight| weight), [1, 3, 5, 4, 2, 0]);
    }

    #[test]
    fn prioritized_results_and_failures_use_source_order() {
        let values = [1, 30, 2, 30, 5, 9];
        let results = prioritized(
            EXECUTION,
            CylinderFeatureStage::Tool,
            &values,
            |&weight| weight,
            &mut || false,
            |&value, _| Ok(value),
        )
        .unwrap();
        assert_eq!(results, values);
        let fault = prioritized(
            EXECUTION,
            CylinderFeatureStage::Tool,
            &values,
            |&weight| weight,
            &mut || false,
            |&value, _| match value {
                1 => Err(missing(ATTEMPT, CylinderFeatureStage::Tool, "first source")),
                30 => Err(missing(ATTEMPT, CylinderFeatureStage::Tool, "later source")),
                _ => Ok(value),
            },
        )
        .unwrap_err();
        assert!(matches!(
            *fault.cause,
            CylinderFeatureCause::DirectAttachment {
                expected: "first source"
            }
        ));
    }

    #[test]
    fn prioritized_serial_work_keeps_source_order_without_estimating_cost() {
        let values = [1, 30, 2, 30, 5, 9];
        let execution = Execution::new(ATTEMPT, NonZeroUsize::MIN);
        let result = prioritized(
            execution,
            CylinderFeatureStage::Tool,
            &values,
            |_| panic!("serial work must not estimate dispatch cost"),
            &mut || false,
            |&value, _| Ok(value),
        );
        assert_eq!(result.unwrap(), values);
    }

    #[test]
    fn never_publishes_partial_work_after_cancellation() {
        let values = [0; 48];
        let error = ordered(
            EXECUTION,
            CylinderFeatureStage::Validation,
            &values,
            &mut || true,
            |i, _| Ok(*i),
        )
        .unwrap_err();
        assert!(matches!(*error.cause, CylinderFeatureCause::Cancelled));
    }
}
