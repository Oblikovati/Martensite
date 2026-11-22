//! Replaced swapchains stay alive until their own presentation fences signal,
//! so a resize does not wait for the display to finish showing the old images (#594).
use super::{frame_flight::FRAMES_IN_FLIGHT, window_chain::WindowChain};

/// The most replaced chains kept before the oldest one is retired with a blocking wait.
const RETIRING_CHAIN_LIMIT: usize = FRAMES_IN_FLIGHT + 1;

/// Replaced chains in replacement order; `retiring.push(old)` never blocks below the limit.
#[derive(Default)]
pub(super) struct RetiringChains {
    chains: Vec<WindowChain>,
}

impl RetiringChains {
    /// Keep `chain` until its fences signal; a chain with an unpresented acquired image is
    /// dropped at once, because only its device-idle retirement proves its work completed.
    pub fn push(&mut self, chain: WindowChain) {
        if chain.acquired_index.is_some() {
            drop(chain);
            return;
        }
        self.chains.push(chain);
        if self.chains.len() > RETIRING_CHAIN_LIMIT {
            drop(self.chains.remove(0));
        }
    }

    /// Destroy every chain whose fences have all signaled; `retiring.collect()` never blocks.
    pub fn collect(&mut self) {
        self.chains
            .retain_mut(|chain| !chain.sync.try_retire().is_stopped());
    }
}
