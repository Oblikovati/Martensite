//! Deliberately unstarted geometry proves the worker's cancellation-before-computation boundary.
use super::EdgeEdgeTask;
use crate::bop_ds::PaveBlock;
use martensite_tkbrep::topods::Edge;
use std::sync::{Arc, Mutex};

/// Supplies unreachable geometry for a cancellation-before-computation control.
/// Example: `let task = unstarted_ee_task();` must be cancelled before execution.
pub(super) fn unstarted_ee_task() -> EdgeEdgeTask {
    let blocks = [
        Arc::new(Mutex::new(PaveBlock::default())),
        Arc::new(Mutex::new(PaveBlock::default())),
    ];
    EdgeEdgeTask::new(blocks, [Edge::default(), Edge::default()], [(0.0, 1.0); 2])
}
