//! Execute independent certificates with bounded operation-local workers and ordered results.
use super::*;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed},
    mpsc::{self, Receiver, RecvTimeoutError, Sender},
};
use std::thread;
use std::time::Duration;

const MAX_WORKERS: usize = 12;
const CANCEL_POLL: Duration = Duration::from_millis(4);

pub(super) fn ordered<I, O, F>(
    attempt: Attempt,
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
    let available = thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    let workers = available.min(MAX_WORKERS).min(inputs.len().div_ceil(2));
    if workers < 2 {
        return inputs
            .iter()
            .map(|input| {
                attempt.poll(cancelled, stage)?;
                check(input, cancelled)
            })
            .collect();
    }
    let queue = Queue {
        next: AtomicUsize::new(0),
        stop: AtomicBool::new(false),
        inputs,
        check,
    };
    queue.run(attempt, stage, cancelled, workers)
}

struct Queue<'a, I, F> {
    next: AtomicUsize,
    stop: AtomicBool,
    inputs: &'a [I],
    check: F,
}

type Findings<O> = (usize, Checked<O>);

impl<I: Sync, F: Sync> Queue<'_, I, F> {
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

    fn worker<O>(&self, send: Sender<Vec<Findings<O>>>)
    where
        F: Fn(&I, &mut CancelPoll<'_>) -> Checked<O>,
    {
        let mut completed = Vec::new();
        while !self.stop.load(Relaxed) {
            let index = self.next.fetch_add(1, Relaxed);
            let Some(input) = self.inputs.get(index) else {
                break;
            };
            let result = (self.check)(input, &mut || self.stop.load(Relaxed));
            completed.push((index, result));
        }
        if !completed.is_empty() {
            // Certificates are independent. Return one batch per worker so that a
            // large boundary does not wake the cancel coordinator for every span.
            let _ = send.send(completed);
        }
    }
}

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

    #[test]
    fn preserves_input_order_and_first_typed_failure() {
        let values: Vec<_> = (0..48).collect();
        let results = ordered(
            ATTEMPT,
            CylinderFeatureStage::Tool,
            &values,
            &mut || false,
            |i, _| Ok(i * i),
        )
        .unwrap();
        assert_eq!(results, values.iter().map(|i| i * i).collect::<Vec<_>>());
        let fault = ordered(
            ATTEMPT,
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
    fn never_publishes_partial_work_after_cancellation() {
        let values = [0; 48];
        let error = ordered(
            ATTEMPT,
            CylinderFeatureStage::Validation,
            &values,
            &mut || true,
            |i, _| Ok(*i),
        )
        .unwrap_err();
        assert!(matches!(*error.cause, CylinderFeatureCause::Cancelled));
    }
}
