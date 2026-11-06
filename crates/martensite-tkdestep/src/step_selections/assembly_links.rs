//! `STEPSelectionsAssemblyComponent` and `STEPSelectionsAssemblyLink`: the tree of an assembly.
use crate::step_repr::StepReprNextAssemblyUsageOccurrence;
use crate::step_shape::StepShapeShapeDefinitionRepresentation;
use martensite_tkernel::impl_transient;
use martensite_tkernel::ncollection::HSequence;
use martensite_tkernel::standard::{Handle, SharedState, StandardTransient};

/// The list of links under a component.
pub type STEPSelectionsAssemblyLinks = HSequence<Handle<STEPSelectionsAssemblyLink>>;

#[derive(Default)]
struct ComponentState {
    my_sdr: Handle<StepShapeShapeDefinitionRepresentation>,
    my_list: Handle<STEPSelectionsAssemblyLinks>,
}

/// Assembly component.
#[derive(Default)]
pub struct STEPSelectionsAssemblyComponent {
    transient: StandardTransient,
    state: SharedState<ComponentState>,
}
impl_transient!(
    STEPSelectionsAssemblyComponent,
    "STEPSelectionsAssemblyComponent",
    StandardTransient,
    transient
);

impl STEPSelectionsAssemblyComponent {
    /// `STEPSelectionsAssemblyComponent(sdr, list)`.
    pub fn new(
        sdr: &Handle<StepShapeShapeDefinitionRepresentation>,
        list: &Handle<STEPSelectionsAssemblyLinks>,
    ) -> Self {
        let made = Self::default();
        made.set_sdr(sdr);
        made.set_list(list);
        made
    }
    /// `get_sdr`.
    pub fn get_sdr(&self) -> Handle<StepShapeShapeDefinitionRepresentation> {
        self.state.read().my_sdr.clone()
    }
    /// `get_list`.
    pub fn get_list(&self) -> Handle<STEPSelectionsAssemblyLinks> {
        self.state.read().my_list.clone()
    }
    /// `set_sdr(sdr)`.
    pub fn set_sdr(&self, sdr: &Handle<StepShapeShapeDefinitionRepresentation>) {
        self.state.write().my_sdr = sdr.clone();
    }
    /// `set_list(list)`.
    pub fn set_list(&self, list: &Handle<STEPSelectionsAssemblyLinks>) {
        self.state.write().my_list = list.clone();
    }
}

#[derive(Default)]
struct LinkState {
    my_nauo: Handle<StepReprNextAssemblyUsageOccurrence>,
    my_item: Handle<StandardTransient>,
    my_component: Handle<STEPSelectionsAssemblyComponent>,
}

/// Assembly link.
#[derive(Default)]
pub struct STEPSelectionsAssemblyLink {
    transient: StandardTransient,
    state: SharedState<LinkState>,
}
impl_transient!(
    STEPSelectionsAssemblyLink,
    "STEPSelectionsAssemblyLink",
    StandardTransient,
    transient
);

impl STEPSelectionsAssemblyLink {
    /// `STEPSelectionsAssemblyLink(nauo, item, part)`.
    pub fn new(
        nauo: &Handle<StepReprNextAssemblyUsageOccurrence>,
        item: &Handle<StandardTransient>,
        part: &Handle<STEPSelectionsAssemblyComponent>,
    ) -> Self {
        let made = Self::default();
        made.set_nauo(nauo);
        made.set_item(item);
        made.set_component(part);
        made
    }
    /// `get_nauo`.
    pub fn get_nauo(&self) -> Handle<StepReprNextAssemblyUsageOccurrence> {
        self.state.read().my_nauo.clone()
    }
    /// `get_item`.
    pub fn get_item(&self) -> Handle<StandardTransient> {
        self.state.read().my_item.clone()
    }
    /// `get_component`.
    pub fn get_component(&self) -> Handle<STEPSelectionsAssemblyComponent> {
        self.state.read().my_component.clone()
    }
    /// `set_nauo(nauo)`.
    pub fn set_nauo(&self, nauo: &Handle<StepReprNextAssemblyUsageOccurrence>) {
        self.state.write().my_nauo = nauo.clone();
    }
    /// `set_item(item)`.
    pub fn set_item(&self, item: &Handle<StandardTransient>) {
        self.state.write().my_item = item.clone();
    }
    /// `set_component(part)`.
    pub fn set_component(&self, part: &Handle<STEPSelectionsAssemblyComponent>) {
        self.state.write().my_component = part.clone();
    }
}
