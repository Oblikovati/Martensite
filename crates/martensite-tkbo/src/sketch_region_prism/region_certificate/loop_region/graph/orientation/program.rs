// SPDX-License-Identifier: GPL-2.0-only
//! The existing reverse-orbit shortcut retains a derived source-bound V5 witness.
use super::*;
pub(super) struct OrientationState<'a> {
    known: BTreeMap<Vec<Dart>, i8>,
    parents: Option<BTreeMap<Vec<Dart>, usize>>,
    trace: Option<&'a mut ProgramTrace>,
}
impl<'a> OrientationState<'a> {
    pub(super) fn new(trace: Option<&'a mut ProgramTrace>) -> Self {
        Self {
            known: BTreeMap::new(),
            parents: trace.as_ref().map(|_| BTreeMap::new()),
            trace,
        }
    }
    pub(super) fn sign(
        &mut self,
        request: &GraphRegionRequest<'_, '_>,
        binding: &SegmentBinding<'_>,
        index: usize,
        orbit: &[Dart],
        cancelled: &mut impl FnMut() -> bool,
    ) -> LoopRegionCheck<i8> {
        let reverse = canonical(orbit.iter().map(|dart| dart.reversed()));
        let position = self.trace.as_ref().map(|trace| trace.position());
        let sign = match self.known.get(&reverse).copied() {
            Some(sign) => {
                self.reversed(binding, index, orbit, &reverse)?;
                -sign
            }
            None => direct(
                request,
                binding,
                index,
                orbit,
                cancelled,
                self.trace.as_deref_mut(),
            )?,
        };
        self.retain(orbit, sign, position);
        Ok(sign)
    }
    fn reversed(
        &mut self,
        binding: &SegmentBinding<'_>,
        index: usize,
        orbit: &[Dart],
        key: &[Dart],
    ) -> LoopRegionCheck<()> {
        let Some(trace) = self.trace.as_deref_mut() else {
            return Ok(());
        };
        let parent = parent_position(self.parents.as_ref(), key, index)?;
        trace.reverse(
            parent,
            orbit.iter().map(|dart| dart.authored(binding)).collect(),
        );
        Ok(())
    }
    fn retain(&mut self, orbit: &[Dart], sign: i8, position: Option<usize>) {
        if let (Some(parents), Some(position)) = (&mut self.parents, position) {
            parents.insert(canonical(orbit.iter().copied()), position);
        }
        self.known.insert(canonical(orbit.iter().copied()), sign);
    }
}
fn direct(
    request: &GraphRegionRequest<'_, '_>,
    binding: &SegmentBinding<'_>,
    index: usize,
    orbit: &[Dart],
    cancelled: &mut impl FnMut() -> bool,
    trace: Option<&mut ProgramTrace>,
) -> LoopRegionCheck<i8> {
    let Some(trace) = trace else {
        return certified_sign(request, binding, index, orbit, cancelled);
    };
    let uses: Vec<_> = orbit.iter().map(|dart| dart.authored(binding)).collect();
    let occurrences = curves(binding, &uses);
    let proof = trace
        .boundary(index, uses, &occurrences, cancelled)
        .map_err(|cause| LoopRegionFailure {
            stage: LoopRegionStage::Faces,
            cause: LoopRegionCause::IntegralProgram(Box::new(cause)),
        })?;
    strict_sign(proof.enclosure(), index)
}

fn parent_position(
    parents: Option<&BTreeMap<Vec<Dart>, usize>>,
    key: &[Dart],
    index: usize,
) -> LoopRegionCheck<usize> {
    parents
        .and_then(|parents| parents.get(key))
        .copied()
        .ok_or(LoopRegionFailure {
            stage: LoopRegionStage::Faces,
            cause: LoopRegionCause::Count {
                quantity: "executed reverse integral parent",
                actual: index,
                expected: index.saturating_sub(1),
            },
        })
}
