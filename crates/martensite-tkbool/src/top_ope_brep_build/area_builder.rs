//! The area builders: the loops of a loop set grouped into areas (an outer loop with
//! the loops inside it) by a loop classifier.
use super::loops::{
    TopOpeBRepBuildLoop, TopOpeBRepBuildLoopClassifier, TopOpeBRepBuildLoopEnum,
    TopOpeBRepBuildLoopSet,
};
use martensite_tkernel::{
    ncollection::{List, ListCursor},
    standard::{Failure, Handle},
};
mod build;
mod copy;
mod extension;
mod owners;
mod traversal;
pub use extension::{AreaExtensionState, AreaExtensionStateMut};
use martensite_tkg3d::top_abs::State;
use std::ops::{Deref, DerefMut};

/// The loops of an area.
pub type LoopList = Vec<Handle<dyn TopOpeBRepBuildLoop>>;
/// Stable nodes of one retained area; `AreaLoopList::default()`.
pub type AreaLoopList = List<Handle<dyn TopOpeBRepBuildLoop>>;
/// Stable area nodes, each owning its loop nodes; `AreaList::default()`.
pub type AreaList = List<AreaLoopList>;

/// StandardNullObject of a null loop.
fn null_loop() -> Failure {
    Failure::with_type(
        "Standard_NullObject",
        "TopOpeBRepBuildAreaBuilder: null loop; expected a loop",
        "TopOpeBRepBuildAreaBuilder",
    )
}

/// The areas of a loop set (`TopOpeBRepBuildAreaBuilder`);
/// `let ab = TopOpeBRepBuildAreaBuilder::with_loops(&mut ls, &mut lc, false)?;`.
#[derive(Default)]
pub struct TopOpeBRepBuildAreaBuilder {
    /// `myArea` (protected).
    pub(crate) my_area: AreaList,
    my_area_iterator: ListCursor,
    my_loop_iterator: ListCursor,
    loop_area_hint: ListCursor,
    area_cursor_sources: Vec<AreaList>,
    loop_cursor_sources: Vec<AreaLoopList>,
    bound_area_source: Option<usize>,
    bound_loop_source: Option<usize>,
    /// `myUNKNOWNRaise` (protected): false, so an UNKNOWN state is taken as IN.
    pub(crate) my_unknown_raise: bool,
}

/// The step of the area algorithm the 2d builders add (Area2dBuilder): the boundary
/// loops become an area when there is none.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Flavor {
    Base,
    TwoD,
}

impl TopOpeBRepBuildAreaBuilder {
    /// `TopOpeBRepBuildAreaBuilder()`; `TopOpeBRepBuildAreaBuilder::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// `TopOpeBRepBuildAreaBuilder(LS, LC, ForceClass)`: `InitAreaBuilder`;
    /// `TopOpeBRepBuildAreaBuilder::with_loops(&mut ls, &mut lc, false)?`.
    pub fn with_loops(
        ls: &mut dyn TopOpeBRepBuildLoopSet,
        lc: &mut dyn TopOpeBRepBuildLoopClassifier,
        force_class: bool,
    ) -> Result<Self, Failure> {
        let mut ab = Self::new();
        ab.init_area_builder(ls, lc, force_class)?;
        Ok(ab)
    }

    /// `CompareLoopWithListOfLoop(LC, L, LOL, le)` (protected): OUT for an empty list,
    /// else the state of `l` against the tested loops of the list (the shape loops for
    /// BOUNDARY, the block loops for BLOCK), stopping at the first OUT; UNKNOWN when
    /// none is tested.
    pub fn compare_loop_with_list_of_loop(
        &self,
        lc: &mut dyn TopOpeBRepBuildLoopClassifier,
        l: &Handle<dyn TopOpeBRepBuildLoop>,
        lol: &[Handle<dyn TopOpeBRepBuildLoop>],
        what: TopOpeBRepBuildLoopEnum,
    ) -> Result<State, Failure> {
        self.compare_items(lc, l, lol.iter(), what)
    }

    fn compare_items<'a>(
        &self,
        classifier: &mut dyn TopOpeBRepBuildLoopClassifier,
        candidate: &Handle<dyn TopOpeBRepBuildLoop>,
        items: impl Iterator<Item = &'a Handle<dyn TopOpeBRepBuildLoop>>,
        kind: TopOpeBRepBuildLoopEnum,
    ) -> Result<State, Failure> {
        let mut items = items.peekable();
        if items.peek().is_none() {
            return Ok(State::Out);
        }
        let mut state = State::Unknown;
        for current in items {
            if selected_loop(current, kind)? {
                state = classifier.compare(candidate, current)?;
                if state == State::Out {
                    break;
                }
            }
        }
        Ok(state)
    }

    /// `Atomize(state, newstate)` (protected): `newstate`, or StandardDomainError for
    /// an UNKNOWN state when UNKNOWN raises.
    pub fn atomize(&self, state: &mut State, newstate: State) -> Result<(), Failure> {
        if self.my_unknown_raise {
            if *state == State::Unknown {
                return Err(Failure::with_type(
                    "Standard_DomainError",
                    "AreaBuilder : Position Unknown",
                    "TopOpeBRepBuildAreaBuilder",
                ));
            }
            return Ok(());
        }
        *state = newstate;
        Ok(())
    }

    /// The state of a comparison, UNKNOWN atomized to IN.
    fn known(&self, mut state: State) -> Result<State, Failure> {
        if state == State::Unknown {
            self.atomize(&mut state, State::In)?;
        }
        Ok(state)
    }

    /// `InitAreaBuilder(LS, LC, ForceClass)` (virtual natively; ForceClass defaulted to
    /// false): each shape loop (unless ForceClass) added to the first area whose block
    /// loops contain it, else kept as a boundary loop; each block loop inside an area
    /// moves out the loops of the area outside it, else starts an area with the
    /// boundary loops around it; `ab.init_area_builder(&mut ls, &mut lc, false)?`.
    pub fn init_area_builder(
        &mut self,
        ls: &mut dyn TopOpeBRepBuildLoopSet,
        lc: &mut dyn TopOpeBRepBuildLoopClassifier,
        force_class: bool,
    ) -> Result<(), Failure> {
        self.build_areas(ls, lc, force_class, Flavor::Base)
    }

    /// `ADD_Loop_TO_LISTOFLoop(L, LOL, s)`: appended; `ab.add_loop_to_list_of_loop(&l, &mut lol)`.
    pub fn add_loop_to_list_of_loop(
        &self,
        l: &Handle<dyn TopOpeBRepBuildLoop>,
        lol: &mut LoopList,
    ) {
        lol.push(l.clone());
    }

    /// `REM_Loop_FROM_LISTOFLoop(ITLOL, LOL, s)`: the loop at the position removed;
    /// `ab.rem_loop_from_list_of_loop(0, &mut lol)`.
    pub fn rem_loop_from_list_of_loop(&self, itlol: usize, lol: &mut LoopList) {
        if itlol < lol.len() {
            lol.remove(itlol);
        }
    }

    /// `ADD_LISTOFLoop_TO_LISTOFLoop(LOL1, LOL2, s, s1, s2)`: the loops of `lol1` moved
    /// to the end of `lol2`; `ab.add_list_of_loop_to_list_of_loop(&mut a, &mut b)`.
    pub fn add_list_of_loop_to_list_of_loop(&self, lol1: &mut LoopList, lol2: &mut LoopList) {
        lol2.append(lol1);
    }
}

/// The 2d area builder (`TopOpeBRepBuildArea2dBuilder`): the boundary loops become an
/// area when no area is built; `TopOpeBRepBuildArea2dBuilder::with_loops(&mut ls, &mut lc, false)?`.
#[derive(Clone, Default)]
pub struct TopOpeBRepBuildArea2dBuilder {
    base: TopOpeBRepBuildAreaBuilder,
}

impl Deref for TopOpeBRepBuildArea2dBuilder {
    type Target = TopOpeBRepBuildAreaBuilder;
    fn deref(&self) -> &TopOpeBRepBuildAreaBuilder {
        &self.base
    }
}

impl DerefMut for TopOpeBRepBuildArea2dBuilder {
    fn deref_mut(&mut self) -> &mut TopOpeBRepBuildAreaBuilder {
        &mut self.base
    }
}

impl TopOpeBRepBuildArea2dBuilder {
    /// `TopOpeBRepBuildArea2dBuilder()`; `TopOpeBRepBuildArea2dBuilder::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// `TopOpeBRepBuildArea2dBuilder(LS, LC, ForceClass)`;
    /// `TopOpeBRepBuildArea2dBuilder::with_loops(&mut ls, &mut lc, false)?`.
    pub fn with_loops(
        ls: &mut dyn TopOpeBRepBuildLoopSet,
        lc: &mut dyn TopOpeBRepBuildLoopClassifier,
        force_class: bool,
    ) -> Result<Self, Failure> {
        let mut ab = Self::new();
        ab.init_area_builder(ls, lc, force_class)?;
        Ok(ab)
    }

    /// `InitAreaBuilder(LS, LC, ForceClass)`: the areas of AreaBuilder, then the
    /// boundary loops as an area when there is none;
    /// `ab.init_area_builder(&mut ls, &mut lc, false)?`.
    pub fn init_area_builder(
        &mut self,
        ls: &mut dyn TopOpeBRepBuildLoopSet,
        lc: &mut dyn TopOpeBRepBuildLoopClassifier,
        force_class: bool,
    ) -> Result<(), Failure> {
        self.base.build_areas(ls, lc, force_class, Flavor::TwoD)
    }
}

/// The area builder of faces (`TopOpeBRepBuildFaceAreaBuilder`);
/// `TopOpeBRepBuildFaceAreaBuilder::with_loops(&mut ls, &mut lc, true)?`.
#[derive(Clone, Default)]
pub struct TopOpeBRepBuildFaceAreaBuilder {
    base: TopOpeBRepBuildArea2dBuilder,
}

impl Deref for TopOpeBRepBuildFaceAreaBuilder {
    type Target = TopOpeBRepBuildArea2dBuilder;
    fn deref(&self) -> &TopOpeBRepBuildArea2dBuilder {
        &self.base
    }
}

impl DerefMut for TopOpeBRepBuildFaceAreaBuilder {
    fn deref_mut(&mut self) -> &mut TopOpeBRepBuildArea2dBuilder {
        &mut self.base
    }
}

impl TopOpeBRepBuildFaceAreaBuilder {
    /// `TopOpeBRepBuildFaceAreaBuilder()`; `TopOpeBRepBuildFaceAreaBuilder::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// `TopOpeBRepBuildFaceAreaBuilder(LS, LC, ForceClass)`;
    /// `TopOpeBRepBuildFaceAreaBuilder::with_loops(&mut ls, &mut lc, true)?`.
    pub fn with_loops(
        ls: &mut dyn TopOpeBRepBuildLoopSet,
        lc: &mut dyn TopOpeBRepBuildLoopClassifier,
        force_class: bool,
    ) -> Result<Self, Failure> {
        let mut ab = Self::new();
        ab.init_face_area_builder(ls, lc, force_class)?;
        Ok(ab)
    }

    /// `InitFaceAreaBuilder(LS, LC, ForceClass)`: the 2d areas;
    /// `ab.init_face_area_builder(&mut ls, &mut lc, true)?`.
    pub fn init_face_area_builder(
        &mut self,
        ls: &mut dyn TopOpeBRepBuildLoopSet,
        lc: &mut dyn TopOpeBRepBuildLoopClassifier,
        force_class: bool,
    ) -> Result<(), Failure> {
        self.base.init_area_builder(ls, lc, force_class)
    }
}

#[cfg(test)]
mod cursor_tests;

fn selected_loop(
    current: &Handle<dyn TopOpeBRepBuildLoop>,
    kind: TopOpeBRepBuildLoopEnum,
) -> Result<bool, Failure> {
    let shape = current.get().ok_or_else(null_loop)?.is_shape();
    Ok(match kind {
        TopOpeBRepBuildLoopEnum::AnyLoop => true,
        TopOpeBRepBuildLoopEnum::Boundary => shape,
        TopOpeBRepBuildLoopEnum::Block => !shape,
    })
}
