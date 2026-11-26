// SPDX-License-Identifier: GPL-2.0-only
//! Map numerical failure owners back to authored relations and arcs.
use super::{BlockOwner, EquationBlock, Failure};
use martensite_tksolve::{
    nonlinear::SolveError,
    preview::{Incomplete, PreviewError},
    residual::EvaluationError,
};

fn failure_owner(blocks: &[EquationBlock], error: &SolveError) -> Option<BlockOwner> {
    let SolveError::Evaluation(EvaluationError::Source { block, .. }) = error else {
        return None;
    };
    blocks.get(*block).map(|block| block.owner)
}

pub(super) fn solve_failure(blocks: &[EquationBlock], cause: SolveError) -> Failure {
    match failure_owner(blocks, &cause) {
        Some(BlockOwner::IntrinsicArc(arc)) => Failure::ArcSolve(arc, cause),
        Some(BlockOwner::Relation(id)) => Failure::Solve(Some(id), cause),
        None => Failure::Solve(None, cause),
    }
}

pub(super) fn preview_failure(blocks: &[EquationBlock], error: PreviewError) -> Failure {
    match error {
        PreviewError::Incomplete(Incomplete::Cancelled) => Failure::Cancelled,
        PreviewError::Solve(ref cause) => match failure_owner(blocks, cause) {
            Some(BlockOwner::IntrinsicArc(arc)) => Failure::ArcPreview(arc, error),
            Some(BlockOwner::Relation(id)) => Failure::Preview(Some(id), error),
            None => Failure::Preview(None, error),
        },
        error => Failure::Preview(None, error),
    }
}
