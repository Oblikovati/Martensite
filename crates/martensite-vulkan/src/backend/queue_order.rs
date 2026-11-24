//! Queue operations reach the Vulkan queue in the order the host issued them, also when the
//! presentation thread submits a window frame for the host (#594). Vulkan requires external
//! synchronization of a queue, and a ticket also keeps a later host submission behind a frame
//! that the thread has not submitted yet, so the GPU runs the work in program order.
use std::sync::{Arc, Condvar, Mutex, PoisonError};

/// The ticket counter of one device's queues; owned by the runtime.
#[derive(Default)]
pub(super) struct QueueOrder {
    tickets: Mutex<QueueTickets>,
    turn: Condvar,
}

#[derive(Default)]
struct QueueTickets {
    issued: u64,
    served: u64,
}

/// The right to use the queues until it drops; `let _turn = runtime.queue_order.next_turn();`.
pub(super) struct QueueTurn {
    order: Arc<QueueOrder>,
}

/// A place in the queue order that another thread takes later. Dropping it unused still takes
/// and ends its turn, so a later ticket never waits forever.
pub(super) struct ReservedTurn {
    order: Arc<QueueOrder>,
    number: u64,
    entered: bool,
}

impl QueueOrder {
    /// Wait for every earlier ticket and take the next turn; `order.next_turn()`.
    pub fn next_turn(self: &Arc<Self>) -> QueueTurn {
        let number = self.issue();
        self.wait_for(number)
    }
    /// Reserve the next place for another thread; `let reserved = order.reserve();`.
    pub fn reserve(self: &Arc<Self>) -> ReservedTurn {
        ReservedTurn {
            order: Arc::clone(self),
            number: self.issue(),
            entered: false,
        }
    }
    fn issue(&self) -> u64 {
        let mut tickets = self.tickets.lock().unwrap_or_else(PoisonError::into_inner);
        tickets.issued += 1;
        tickets.issued - 1
    }
    fn wait_for(self: &Arc<Self>, number: u64) -> QueueTurn {
        let mut tickets = self.tickets.lock().unwrap_or_else(PoisonError::into_inner);
        while tickets.served != number {
            tickets = self
                .turn
                .wait(tickets)
                .unwrap_or_else(PoisonError::into_inner);
        }
        QueueTurn {
            order: Arc::clone(self),
        }
    }
}

impl Drop for QueueTurn {
    fn drop(&mut self) {
        let order = &self.order;
        order
            .tickets
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .served += 1;
        order.turn.notify_all();
    }
}

impl ReservedTurn {
    /// Wait for every earlier ticket and take this turn; `let turn = reserved.enter();`.
    pub fn enter(mut self) -> QueueTurn {
        self.entered = true;
        self.order.wait_for(self.number)
    }
}

impl Drop for ReservedTurn {
    fn drop(&mut self) {
        if !self.entered {
            drop(self.order.wait_for(self.number));
        }
    }
}

#[cfg(test)]
#[path = "queue_order_tests.rs"]
mod tests;
