//! Live indexed-map iteration of a light set.
use super::{
    Graphic3dCLight, Graphic3dLightSet, Graphic3dLightSetFields, Graphic3dLightSetIterationFilter,
};
use martensite_tkernel::standard::exceptions::no_such_object;
use martensite_tkernel::standard::{Failure, Handle, SharedState};
use std::sync::Arc;

/// A live cursor; appends and swap-last removals are visible at its current index.
/// Filters run only on construction and `next`, as in the native iterator.
/// Martensite retains the storage so a cursor cannot outlive borrowed native memory.
/// Example: `let cursor = Graphic3dLightSetIterator::new(&set, filter);`.
#[derive(Clone, Default)]
pub struct Graphic3dLightSetIterator {
    source: Option<Arc<SharedState<Graphic3dLightSetFields>>>,
    /// `myIter`: the 1-based index of the current light in the set's `myLights` map.
    pub index: usize,
    /// `myFilter`: the `Graphic3dLightSetIterationFilter` bits that `next` and
    /// `skip_filtered` apply.
    pub filter: i32,
}

impl Graphic3dLightSetIterator {
    /// Starts at the first accepted indexed light; `Graphic3dLightSetIterator::new(&set, filter)`.
    pub fn new(set: &Graphic3dLightSet, filter: Graphic3dLightSetIterationFilter) -> Self {
        let mut cursor = Self {
            source: Some(Arc::clone(&set.state)),
            index: 1,
            filter: filter as i32,
        };
        cursor.skip_filtered();
        cursor
    }

    /// A null handle creates an empty cursor; `Graphic3dLightSetIterator::from_handle(&set, filter)`.
    pub fn from_handle(
        set: &Handle<Graphic3dLightSet>,
        filter: Graphic3dLightSetIterationFilter,
    ) -> Self {
        match set.get() {
            Some(set) => Self::new(set, filter),
            None => Self {
                filter: filter as i32,
                ..Self::default()
            },
        }
    }

    /// The map that `myIter` walks: the iterated set's protected fields, `None` for a default
    /// or null-set cursor. Example: `cursor.map().map(|fields| fields.read().lights.len())`.
    pub fn map(&self) -> Option<&SharedState<Graphic3dLightSetFields>> {
        self.source.as_deref()
    }

    /// Tests the current map extent without rechecking the filter; `cursor.more()`.
    pub fn more(&self) -> bool {
        self.source
            .as_ref()
            .is_some_and(|source| self.index <= source.read().lights.len())
    }

    /// Reads the live key or raises `Standard_NoSuchObject`; `cursor.value()?`.
    pub fn value(&self) -> Result<Handle<Graphic3dCLight>, Failure> {
        let Some(source) = &self.source else {
            return Err(self.exhausted(0));
        };
        let state = source.read();
        self.index
            .checked_sub(1)
            .and_then(|slot| state.lights.get(slot))
            .map(|(light, _)| light.clone())
            .ok_or_else(|| self.exhausted(state.lights.len()))
    }

    /// Advances once then skips currently excluded lights; `cursor.next()`.
    pub fn next(&mut self) {
        self.index = self.index.wrapping_add(1);
        self.skip_filtered();
    }

    /// Rechecks the current filter, exposing the native protected subclass hook.
    /// Normal reads leave a changed current light untouched until `next`.
    /// Example: `cursor.skip_filtered();`.
    pub fn skip_filtered(&mut self) {
        if self.filter == 0 {
            return;
        }
        while self.value().is_ok_and(|light| self.excludes(&light)) {
            self.index = self.index.wrapping_add(1);
        }
    }

    fn excludes(&self, light: &Graphic3dCLight) -> bool {
        use Graphic3dLightSetIterationFilter as F;
        let has = |flag: F| self.filter & flag as i32 != 0;
        (has(F::ExcludeAmbient) && light.light_type() == super::Graphic3dTypeOfLightSource::Ambient)
            || (has(F::ExcludeDisabled) && !light.is_enabled())
            || (has(F::ExcludeNoShadow) && !light.to_cast_shadows())
    }

    fn exhausted(&self, extent: usize) -> Failure {
        no_such_object(&format!(
            "NCollection_IndexedDataMap::Iterator::Key: index {}; expected 1..={extent}",
            self.index
        ))
    }
}
