//! The presentation thread submits and presents window frames for the host, as the submission
//! thread of an OpenGL driver does, so a redraw does not wait in the kernel for the queue
//! (#594). The maintainer's decision of 2026-11-24 (PLAN.md, phase 2) lets a submit or
//! present error surface at the next redraw, explicit wait or capture.
use super::queue_order::ReservedTurn;
use super::window_gui_contract::WindowGuiContract;
use super::{submission::CommandSubmission, window_chain::WindowChain};
use crate::RenderError;
use ash::vk;
use std::sync::{Arc, Condvar, Mutex, PoisonError, mpsc};

/// A recorded window frame and the chain whose image it was recorded for.
pub(super) struct PresentJob {
    pub commands: CommandSubmission,
    pub chain: WindowChain,
    pub queue: vk::Queue,
    pub index: u32,
    pub contract: WindowGuiContract,
    /// Reserved by the host when it handed the frame over, so the frame keeps its queue order.
    pub turn: ReservedTurn,
}

/// What the thread hands back: the commands go to the frame flight, the chain to its presenter.
#[derive(Default)]
struct PresentedFrame {
    finished: bool,
    commands: Option<CommandSubmission>,
    chain: Option<WindowChain>,
    outcome: Option<Result<(), RenderError>>,
    failed: bool,
}

/// The result of one job, shared by the renderer and the presenter that handed it over.
#[derive(Default)]
pub(super) struct PresentReceipt {
    frame: Mutex<PresentedFrame>,
    done: Condvar,
}

impl PresentReceipt {
    fn wait(&self) -> std::sync::MutexGuard<'_, PresentedFrame> {
        let mut frame = self.frame.lock().unwrap_or_else(PoisonError::into_inner);
        while !frame.finished {
            frame = self
                .done
                .wait(frame)
                .unwrap_or_else(PoisonError::into_inner);
        }
        frame
    }
    /// Wait for the job and take its commands and its error, once;
    /// `let (commands, outcome) = receipt.take_commands();`.
    pub fn take_commands(&self) -> (Option<CommandSubmission>, Result<(), RenderError>) {
        let mut frame = self.wait();
        (
            frame.commands.take(),
            frame.outcome.take().unwrap_or(Ok(())),
        )
    }
    /// Wait for the job and take back its chain, and whether the job failed;
    /// `let (chain, failed) = receipt.take_chain();`.
    pub fn take_chain(&self) -> (Option<WindowChain>, bool) {
        let mut frame = self.wait();
        (frame.chain.take(), frame.failed)
    }
    fn finish(&self, presented: PresentedFrame) {
        *self.frame.lock().unwrap_or_else(PoisonError::into_inner) = presented;
        self.done.notify_all();
    }
}

/// Submit, then present, inside the reserved queue turn; the same calls as a host redraw.
fn run(job: PresentJob) -> PresentedFrame {
    let PresentJob {
        commands,
        mut chain,
        queue,
        index,
        contract,
        turn,
    } = job;
    // Declared last, so it ends before the chain or the commands could drop.
    let turn = turn.enter();
    let outcome = super::window_submission::submit_within(&commands, &mut chain, &turn)
        .and_then(|()| chain.present_within(queue, (index, contract), &turn));
    drop(turn);
    PresentedFrame {
        finished: true,
        failed: outcome.is_err(),
        commands: Some(commands),
        chain: Some(chain),
        outcome: Some(outcome),
    }
}

/// Finishes its receipt even if the job unwinds, so no waiter blocks forever.
struct ReceiptWriter(Option<Arc<PresentReceipt>>);

impl ReceiptWriter {
    fn complete(mut self, presented: PresentedFrame) {
        if let Some(receipt) = self.0.take() {
            receipt.finish(presented);
        }
    }
}

impl Drop for ReceiptWriter {
    fn drop(&mut self) {
        let Some(receipt) = self.0.take() else {
            return;
        };
        receipt.finish(PresentedFrame {
            finished: true,
            failed: true,
            outcome: Some(Err(stopped_thread())),
            ..Default::default()
        });
    }
}

fn stopped_thread() -> RenderError {
    RenderError::rejected("stopped", "a running presentation thread")
}

type JobSender = mpsc::Sender<(PresentJob, Arc<PresentReceipt>)>;

/// One presentation thread per renderer, started by the first frame it presents. The mutexes
/// only keep the renderer `Sync`: every access holds `&mut` and takes no lock.
#[derive(Default)]
pub(super) struct PresentThread {
    jobs: Mutex<Option<JobSender>>,
    thread: Mutex<Option<std::thread::JoinHandle<()>>>,
}

impl PresentThread {
    /// Queue `job`, whose result `receipt` receives; `let receipt = thread.send(job, receipt)?;`.
    /// A thread that cannot start or has stopped returns the job with the error.
    pub fn send(
        &mut self,
        job: PresentJob,
        receipt: Arc<PresentReceipt>,
    ) -> Result<Arc<PresentReceipt>, Box<(PresentJob, RenderError)>> {
        let sender = match self.sender() {
            Ok(sender) => sender,
            Err(error) => return Err(Box::new((job, error))),
        };
        match sender.send((job, Arc::clone(&receipt))) {
            Ok(()) => Ok(receipt),
            Err(mpsc::SendError((job, _))) => Err(Box::new((job, stopped_thread()))),
        }
    }
    fn sender(&mut self) -> Result<&JobSender, RenderError> {
        let jobs = self.jobs.get_mut().unwrap_or_else(PoisonError::into_inner);
        if jobs.is_none() {
            let (sender, receiver) = mpsc::channel();
            let thread = std::thread::Builder::new()
                .name("martensite-present".into())
                .spawn(move || serve(receiver))
                .map_err(|error| RenderError::rejected(error, "a started presentation thread"))?;
            *self
                .thread
                .get_mut()
                .unwrap_or_else(PoisonError::into_inner) = Some(thread);
            *jobs = Some(sender);
        }
        jobs.as_ref()
            .ok_or_else(|| RenderError::rejected("none", "a presentation job channel"))
    }
}

fn serve(jobs: mpsc::Receiver<(PresentJob, Arc<PresentReceipt>)>) {
    for (job, receipt) in jobs {
        let writer = ReceiptWriter(Some(receipt));
        writer.complete(run(job));
    }
}

impl Drop for PresentThread {
    fn drop(&mut self) {
        // Closing the channel ends the thread after the jobs already queued.
        self.jobs
            .get_mut()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        let thread = self
            .thread
            .get_mut()
            .unwrap_or_else(PoisonError::into_inner)
            .take();
        if let Some(thread) = thread {
            let _ = thread.join();
        }
    }
}

#[cfg(test)]
#[path = "window_present_thread_tests.rs"]
mod tests;
