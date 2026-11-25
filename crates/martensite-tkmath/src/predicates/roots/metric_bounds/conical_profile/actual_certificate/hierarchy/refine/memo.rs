// SPDX-License-Identifier: GPL-2.0-only
//! Keep a bounded number of exact restrictions of the immutable retained sides.
use super::*;

// Each split retains two patches. Beyond this storage allowance the original
// refinement runs with the same arithmetic grant and product depth.
const MAX_CACHED_PATCHES: usize = 1536;

#[derive(Clone)]
pub(super) struct PatchUse {
    source: usize,
    path: u32,
    pub(super) patch: Arc<Patch>,
}

impl PatchUse {
    pub(super) fn root(source: usize, patch: Arc<Patch>) -> Self {
        Self {
            source,
            path: 1,
            patch,
        }
    }

    fn child(&self, along_u: bool, second: bool, patch: Arc<Patch>) -> Self {
        // Two bits per restriction preserve both the chart axis and child.
        let path = (self.path << 2) | (u32::from(along_u) << 1) | u32::from(second);
        Self {
            source: self.source,
            path,
            patch,
        }
    }
}

pub(in super::super) struct Memo<'a> {
    sides: &'a [SideCapture],
    roots: BTreeMap<usize, Charged<Arc<Patch>>>,
    splits: BTreeMap<SplitKey, Charged<[Arc<Patch>; 2]>>,
    spare: usize,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct SplitKey {
    source: usize,
    path: u32,
    along_u: bool,
}

impl<'a> Memo<'a> {
    pub(in super::super) fn new(sides: &'a [SideCapture]) -> Self {
        Self {
            sides,
            roots: BTreeMap::new(),
            splits: BTreeMap::new(),
            spare: MAX_CACHED_PATCHES,
        }
    }

    #[cfg(test)]
    pub(super) fn without_storage() -> Self {
        Self {
            spare: 0,
            ..Self::new(&[])
        }
    }

    pub(super) fn capture(
        &mut self,
        source: usize,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Outcome<PatchUse> {
        if let Some(cached) = self.roots.get(&source) {
            return Ok(PatchUse::root(source, cached.replay(arithmetic)?));
        }
        let side = &self.sides[source];
        let start = arithmetic.operations();
        let patch = Arc::new(Patch::capture(
            &side.coordinates,
            &side.source.weights,
            arithmetic,
        )?);
        if self.spare > 0 {
            self.spare -= 1;
            self.roots
                .insert(source, Charged::new(Arc::clone(&patch), start, arithmetic));
        }
        Ok(PatchUse::root(source, patch))
    }

    pub(super) fn split(
        &mut self,
        parent: &PatchUse,
        along_u: bool,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Outcome<[PatchUse; 2]> {
        let key = SplitKey {
            source: parent.source,
            path: parent.path,
            along_u,
        };
        let [left, right] = if let Some(cached) = self.splits.get(&key) {
            cached.replay(arithmetic)?
        } else {
            self.record_split(key, &parent.patch, arithmetic)?
        };
        Ok([
            parent.child(along_u, false, left),
            parent.child(along_u, true, right),
        ])
    }

    fn record_split(
        &mut self,
        key: SplitKey,
        patch: &Patch,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Outcome<[Arc<Patch>; 2]> {
        let start = arithmetic.operations();
        let children = patch.bisect(key.along_u, arithmetic)?.map(Arc::new);
        if self.spare >= 2 {
            self.spare -= 2;
            self.splits
                .insert(key, Charged::new(children.clone(), start, arithmetic));
        }
        Ok(children)
    }
}

struct Charged<T> {
    value: T,
    steps: u32,
}

impl<T: Clone> Charged<T> {
    fn new(value: T, start: u32, arithmetic: &Arithmetic<'_>) -> Self {
        Self {
            value,
            steps: arithmetic.operations() - start,
        }
    }

    fn replay(&self, arithmetic: &mut Arithmetic<'_>) -> Outcome<T> {
        // Identical operands and precision already passed every numeric check.
        // Retain the original work exhaustion and cancellation position.
        for _ in 0..self.steps {
            arithmetic.certificate_step()?;
        }
        Ok(self.value.clone())
    }
}
