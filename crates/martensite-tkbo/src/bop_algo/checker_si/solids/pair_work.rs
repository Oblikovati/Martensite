//! Scoped readers evaluate stored incidence predicates only after their native More poll.
use super::*;
use crate::bop_algo::{ProgressCancellation, run_context_tasks};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
struct SolidPairWork<'p> {
    pair: (i32, i32),
    every_subshape: bool,
    range: Option<ProgressRange<'p>>,
    interferes: bool,
}
/// Evaluate actual immutable records in input order; `execute_pairs(ds, pairs, every, scope, parallel, observed)?`.
pub(super) fn execute_pairs(
    structure: &DataStructure,
    pairs: Vec<(i32, i32)>,
    every: bool,
    scope: &ProgressScope<'_>,
    parallel: bool,
    observed: &ProgressCancellation,
) -> Result<Vec<(i32, i32)>, Failure> {
    let mut work: Vec<_> = pairs
        .into_iter()
        .map(|pair| SolidPairWork {
            pair,
            every_subshape: every,
            range: Some(scope.next(1.)),
            interferes: false,
        })
        .collect();
    let pooled = parallel && work.len() != 1;
    run_context_tasks(
        &mut work,
        &(),
        pooled,
        || (),
        |item, _| item.perform(structure, observed),
    )?;
    Ok(work
        .into_iter()
        .filter(|item| item.interferes)
        .map(|item| item.pair)
        .collect())
}
impl SolidPairWork<'_> {
    fn perform(
        &mut self,
        structure: &DataStructure,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let range = self.range.take().ok_or_else(|| {
            Failure::new(&format!(
                "solid pair={:?} progress=None; expected one unused owned range",
                self.pair
            ))
        })?;
        let scope = ProgressScope::unnamed(&range, 1., false);
        if !scope.more() {
            observed.record_cancelled();
            return Ok(());
        }
        let (first, second) = self.pair;
        self.interferes = if self.every_subshape {
            structure.has_interference_shape_subshapes_mode(first, second, false)?
                || structure.has_interference_shape_subshapes_mode(second, first, false)?
        } else {
            structure.has_interference_shape_subshapes(first, second)?
                || structure.has_interference_shape_subshapes(second, first)?
        };
        Ok(())
    }
}
