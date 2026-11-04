//! The active clipping planes of a view and the object being drawn (`GlClipping` and
//! `GlClippingIterator`: enabled clipping, capping and chain counts, planes
//! temporarily disabled by index, and the capping algorithm's chain filter.
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::{Graphic3dClipPlane, Graphic3dSequenceOfHClipPlane};

/// Global (view) planes followed by local (object) planes, indexed from 1 as native
/// `PlaneIndex`; `RenderClippingState::default().nb_clipping_or_capping_on() == 0`.
#[derive(Clone, Debug, Default)]
pub struct RenderClippingState {
    global: Handle<Graphic3dSequenceOfHClipPlane>,
    local: Handle<Graphic3dSequenceOfHClipPlane>,
    disabled: Vec<bool>,
    capped_chain: Handle<Graphic3dClipPlane>,
    capped_sub_plane: i32,
    nb_clipping: i32,
    nb_capping: i32,
    nb_chains: i32,
    nb_disabled: i32,
}

fn planes(sequence: &Handle<Graphic3dSequenceOfHClipPlane>) -> Vec<Handle<Graphic3dClipPlane>> {
    sequence.get().map(|s| s.items()).unwrap_or_default()
}

fn sub_planes(plane: &Handle<Graphic3dClipPlane>) -> i32 {
    plane.get().map_or(0, |p| p.nb_chain_next_planes())
}

fn is_on(plane: &Handle<Graphic3dClipPlane>) -> bool {
    plane.get().is_some_and(|p| p.is_on())
}

fn is_capping(plane: &Handle<Graphic3dClipPlane>) -> bool {
    plane.get().is_some_and(|p| p.is_capping())
}

impl RenderClippingState {
    /// Forget every plane, count and filter; `state.init()`.
    pub fn init(&mut self) {
        let disabled = std::mem::take(&mut self.disabled);
        *self = Self {
            disabled,
            ..Self::default()
        };
    }

    /// Replace the global planes, drop the local ones, re-enable every plane and reset the
    /// capping filter; `state.reset(&view_planes)`.
    pub fn reset(&mut self, planes: &Handle<Graphic3dSequenceOfHClipPlane>) {
        let start = self.local_start();
        self.remove(&self.local.clone(), start);
        self.remove(&self.global.clone(), 1);
        self.global = planes.clone();
        self.local = Handle::default();
        self.add(planes, 1);
        self.nb_disabled = 0;
        self.reset_capping_filter();
    }

    /// Replace the local (object) planes after the global ones; `state.set_local_planes(&p)`.
    pub fn set_local_planes(&mut self, planes: &Handle<Graphic3dSequenceOfHClipPlane>) {
        let start = self.local_start();
        self.remove(&self.local.clone(), start);
        self.local = planes.clone();
        self.add(planes, start);
    }

    fn local_start(&self) -> usize {
        self.global.get().map_or(1, |s| s.size() as usize + 1)
    }

    fn add(&mut self, sequence: &Handle<Graphic3dSequenceOfHClipPlane>, start: usize) {
        for (index, plane) in (start..).zip(planes(sequence)) {
            if self.disabled.len() <= index {
                self.disabled.resize(index + 1, false);
            }
            self.disabled[index] = false;
            if is_on(&plane) {
                self.count(&plane, 1);
            }
        }
    }

    fn remove(&mut self, sequence: &Handle<Graphic3dSequenceOfHClipPlane>, start: usize) {
        for (index, plane) in (start..).zip(planes(sequence)) {
            if is_on(&plane) && !self.disabled.get(index).copied().unwrap_or(false) {
                self.count(&plane, -1);
            }
        }
    }

    /// Add (`sign` 1) or remove (-1) one enabled chain and its sub-planes.
    fn count(&mut self, plane: &Handle<Graphic3dClipPlane>, sign: i32) {
        let nb = sign * sub_planes(plane);
        if is_capping(plane) {
            self.nb_capping += nb;
        } else {
            self.nb_clipping += nb;
        }
        self.nb_chains += sign;
    }

    /// True when any enabled plane is capping; `state.is_capping_on()`.
    pub fn is_capping_on(&self) -> bool {
        self.nb_capping > 0
    }

    /// True when any plane clips or caps; `state.is_clipping_or_capping_on()`.
    pub fn is_clipping_or_capping_on(&self) -> bool {
        self.nb_clipping_or_capping_on() > 0
    }

    /// Enabled clipping and capping sub-planes: 1 under `disable_all_except`, one fewer under
    /// `enable_all_except`; `state.nb_clipping_or_capping_on()`.
    pub fn nb_clipping_or_capping_on(&self) -> i32 {
        if self.is_capping_disable_all_except() {
            return 1;
        }
        self.nb_clipping + self.nb_capping - i32::from(self.is_capping_enable_all_except())
    }

    /// True when an enabled chain has more than one sub-plane, the capping filter considered;
    /// `state.has_clipping_chains()`.
    pub fn has_clipping_chains(&self) -> bool {
        if self.is_capping_disable_all_except()
            || self.nb_chains == self.nb_clipping + self.nb_capping
        {
            return false;
        }
        !self.is_capping_enable_all_except()
            || sub_planes(&self.capped_chain) == 1
            || self.nb_chains > 1
    }

    /// True when a defined plane is temporarily disabled; `state.has_disabled()`.
    pub fn has_disabled(&self) -> bool {
        self.nb_disabled > 0
    }

    /// Disable or re-enable the plane at `plane_index` (from `RenderClippingIterator::plane_index`);
    /// false when it already had that state or the index names no plane;
    /// `state.set_enabled(1, false)`.
    pub fn set_enabled(&mut self, plane_index: usize, enabled: bool) -> bool {
        let Some(plane) = self.plane(plane_index) else {
            return false;
        };
        if self.disabled[plane_index] == !enabled {
            return false;
        }
        self.disabled[plane_index] = !enabled;
        let sign = if enabled { 1 } else { -1 };
        self.count(&plane, sign);
        self.nb_disabled -= sign * sub_planes(&plane);
        true
    }

    fn plane(&self, plane_index: usize) -> Option<Handle<Graphic3dClipPlane>> {
        let mut iter = self.iter();
        while iter.more() {
            if iter.plane_index() == plane_index {
                return Some(iter.value());
            }
            iter.next();
        }
        None
    }

    /// Disable the global planes (they come first); `state.disable_global()`.
    pub fn disable_global(&mut self) {
        let globals = planes(&self.global).len();
        for index in 1..=globals {
            self.set_enabled(index, false);
        }
    }

    /// Re-enable every temporarily disabled plane; `state.restore_disabled()`.
    pub fn restore_disabled(&mut self) {
        if self.nb_disabled == 0 {
            return;
        }
        self.nb_disabled = 0;
        let all = planes(&self.global).into_iter().chain(planes(&self.local));
        for (index, plane) in (1..).zip(all) {
            if self.disabled[index] {
                self.disabled[index] = false;
                self.count(&plane, 1);
            }
        }
    }

    /// The chain of the capping filter (null without one); `state.capped_chain()`.
    pub fn capped_chain(&self) -> &Handle<Graphic3dClipPlane> {
        &self.capped_chain
    }

    /// The filtered sub-plane: positive under `disable_all_except`, negative under
    /// `enable_all_except`, 0 without a filter; `state.capped_sub_plane()`.
    pub fn capped_sub_plane(&self) -> i32 {
        self.capped_sub_plane
    }

    /// True when a capping filter chain is set; `state.is_capping_filter_on()`.
    pub fn is_capping_filter_on(&self) -> bool {
        !self.capped_chain.is_null()
    }

    /// True when only the filtered sub-plane is enabled; `state.is_capping_disable_all_except()`.
    pub fn is_capping_disable_all_except(&self) -> bool {
        self.capped_sub_plane > 0
    }

    /// True when every plane but the filtered sub-plane is enabled;
    /// `state.is_capping_enable_all_except()`.
    pub fn is_capping_enable_all_except(&self) -> bool {
        self.capped_sub_plane < 0
    }

    /// Enable only sub-plane `sub_plane` (from 1) of `chain`; `state.disable_all_except(&c, 1)`.
    pub fn disable_all_except(&mut self, chain: &Handle<Graphic3dClipPlane>, sub_plane: i32) {
        self.capped_chain = chain.clone();
        self.capped_sub_plane = sub_plane;
    }

    /// Enable every plane but sub-plane `sub_plane` (from 1) of `chain`;
    /// `state.enable_all_except(&c, 1)`.
    pub fn enable_all_except(&mut self, chain: &Handle<Graphic3dClipPlane>, sub_plane: i32) {
        self.capped_chain = chain.clone();
        self.capped_sub_plane = -sub_plane;
    }

    /// Clear the capping filter; `state.reset_capping_filter()`.
    pub fn reset_capping_filter(&mut self) {
        self.capped_chain = Handle::default();
        self.capped_sub_plane = 0;
    }

    /// Iterate the global then the local planes; `let mut it = state.iter(); while it.more() {..}`.
    pub fn iter(&self) -> RenderClippingIterator<'_> {
        RenderClippingIterator {
            state: self,
            global: planes(&self.global),
            local: planes(&self.local),
            index: 1,
        }
    }
}

/// A cursor over the planes of a `RenderClippingState` (`GlClippingIterator`); indices
/// start at 1. `state.iter.more`.
pub struct RenderClippingIterator<'a> {
    state: &'a RenderClippingState,
    global: Vec<Handle<Graphic3dClipPlane>>,
    local: Vec<Handle<Graphic3dClipPlane>>,
    index: usize,
}

impl RenderClippingIterator<'_> {
    /// True while the cursor names a plane; `it.more()`.
    pub fn more(&self) -> bool {
        self.index <= self.global.len() + self.local.len()
    }

    /// Move to the next plane; `it.next()`.
    #[allow(clippy::should_implement_trait)] // Cursor step of the native iterator, not `Iterator`.
    pub fn next(&mut self) {
        self.index += 1;
    }

    /// True when the plane is off or temporarily disabled (the capping filter is not
    /// considered); `it.is_disabled()`.
    pub fn is_disabled(&self) -> bool {
        self.state
            .disabled
            .get(self.index)
            .copied()
            .unwrap_or(false)
            || !is_on(&self.value())
    }

    /// The plane at the cursor (null past the end); `it.value()`.
    pub fn value(&self) -> Handle<Graphic3dClipPlane> {
        let local = self.index.wrapping_sub(self.global.len() + 1);
        self.global
            .get(self.index.wrapping_sub(1))
            .or_else(|| self.local.get(local))
            .cloned()
            .unwrap_or_default()
    }

    /// True for a global (view) plane; `it.is_global()`.
    pub fn is_global(&self) -> bool {
        self.index <= self.global.len()
    }

    /// The plane index, from 1; `it.plane_index()`.
    pub fn plane_index(&self) -> usize {
        self.index
    }
}

#[cfg(test)]
#[path = "clipping_state_tests.rs"]
mod tests;
