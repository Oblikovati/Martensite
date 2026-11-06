//! `STEPSelectionsSelectInstances`: all the instances of the assemblies under the roots.
use super::entities_of;
use crate::step_construct::StepConstructAssembly;
use crate::step_entity::step_live;
use crate::step_repr::{StepReprMappedItem, StepReprShapeRepresentationRelationship};
use crate::step_shape::{
    StepShapeBrepWithVoids, StepShapeContextDependentShapeRepresentation, StepShapeFaceSurface,
    StepShapeFacetedBrep, StepShapeFacetedBrepAndBrepWithVoids, StepShapeGeometricSet,
    StepShapeManifoldSolidBrep, StepShapeShapeDefinitionRepresentation,
    StepShapeShapeRepresentation, StepShapeShellBasedSurfaceModel,
};
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkxsbase::if_select::{
    IFSelectSelectDeduct, IFSelectSelectDeductBase, IFSelectSelectExplore,
    IFSelectSelectExploreBase, IFSelectSelection, IFSelectSelectionIterator, deduct_fill_iterator,
    explore_label,
};
use martensite_tkxsbase::interface::{
    InterfaceEntityIterator, InterfaceGraph, InterfaceInterfaceModel,
};
use std::cell::RefCell;
use std::collections::HashSet;

/// The result kept between calls: two thread-locals of the file, valid for one model.
#[derive(Default)]
struct InstancesCache {
    model: Handle<dyn InterfaceInterfaceModel>,
    entities: InterfaceEntityIterator,
}

thread_local! {
    static INSTANCES: RefCell<InstancesCache> = RefCell::default();
}

/// `add_all_sharings`: every entity shared below `start`.
fn add_all_sharings(
    start: &Handle<StandardTransient>,
    graph: &InterfaceGraph,
    explored: &mut InterfaceEntityIterator,
) -> Result<(), Failure> {
    if start.is_null() {
        return Ok(());
    }
    for shared in entities_of(graph.shareds(start)?)? {
        explored.add_item(&shared);
        add_all_sharings(&shared, graph, explored)?;
    }
    Ok(())
}

/// The kinds whose whole structure goes in.
fn is_shape_item(start: &Handle<StandardTransient>) -> bool {
    start.cast::<StepShapeFacetedBrep>().is_some()
        || start.cast::<StepShapeBrepWithVoids>().is_some()
        || start.cast::<StepShapeManifoldSolidBrep>().is_some()
        || start.cast::<StepShapeShellBasedSurfaceModel>().is_some()
        || start
            .cast::<StepShapeFacetedBrepAndBrepWithVoids>()
            .is_some()
        || start.cast::<StepShapeGeometricSet>().is_some()
        || start.cast::<StepShapeFaceSurface>().is_some()
        || start.cast::<StepReprMappedItem>().is_some()
}

/// `add_instances` on a context_dependent_shape_representation: the definitions of its part.
fn add_cdsr_instances(
    cdsr: &Handle<StepShapeContextDependentShapeRepresentation>,
    graph: &InterfaceGraph,
    explored: &mut InterfaceEntityIterator,
) -> Result<(), Failure> {
    let relation = step_live(cdsr, "STEPSelectionsSelectInstances")?.representation_relation();
    let Some(live) = relation.get() else {
        return Ok(());
    };
    let reversed = StepConstructAssembly::check_srr_reverses_nauo(graph, cdsr)?;
    let rep = if reversed { live.rep2() } else { live.rep1() };
    for sharing in entities_of(graph.sharings(&rep.cast())?)? {
        if sharing
            .cast::<StepShapeShapeDefinitionRepresentation>()
            .is_some()
        {
            add_instances(&sharing, graph, explored)?;
        }
    }
    Ok(())
}

/// `add_instances`.
fn add_instances(
    start: &Handle<StandardTransient>,
    graph: &InterfaceGraph,
    explored: &mut InterfaceEntityIterator,
) -> Result<(), Failure> {
    if start.is_null() {
        return Ok(());
    }
    explored.add_item(start);
    let sdr: Handle<StepShapeShapeDefinitionRepresentation> = start.cast();
    if let Some(live) = sdr.get() {
        add_instances(&live.used_representation().cast(), graph, explored)?;
        for shared in entities_of(graph.shareds(start)?)? {
            if shared
                .cast::<StepShapeContextDependentShapeRepresentation>()
                .is_some()
            {
                add_instances(&shared, graph, explored)?;
            }
        }
        return Ok(());
    }
    let rep: Handle<StepShapeShapeRepresentation> = start.cast();
    if let Some(live) = rep.get() {
        for rank in 1..=live.nb_items() {
            add_instances(&live.items_value(rank)?.cast(), graph, explored)?;
        }
        return Ok(());
    }
    if is_shape_item(start) {
        return add_all_sharings(start, graph, explored);
    }
    let cdsr: Handle<StepShapeContextDependentShapeRepresentation> = start.cast();
    if cdsr.is_some() {
        return add_cdsr_instances(&cdsr, graph, explored);
    }
    let und: Handle<StepReprShapeRepresentationRelationship> = start.cast();
    if let Some(live) = und.get() {
        add_instances(&live.rep1().cast(), graph, explored)?;
        add_instances(&live.rep2().cast(), graph, explored)?;
    }
    Ok(())
}

/// Select instances.
pub struct STEPSelectionsSelectInstances {
    transient: StandardTransient,
    base: IFSelectSelectExploreBase,
}
impl_transient!(STEPSelectionsSelectInstances, "STEPSelectionsSelectInstances", dyn IFSelectSelectExplore, transient, interfaces: [dyn IFSelectSelection, dyn IFSelectSelectDeduct, dyn IFSelectSelectExplore]);

impl Default for STEPSelectionsSelectInstances {
    fn default() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: IFSelectSelectExploreBase::new(-1),
        }
    }
}

impl IFSelectSelectDeduct for STEPSelectionsSelectInstances {
    fn deduct_base(&self) -> &IFSelectSelectDeductBase {
        &self.base.deduct
    }
}

impl IFSelectSelectExplore for STEPSelectionsSelectInstances {
    fn explore_base(&self) -> &IFSelectSelectExploreBase {
        &self.base
    }
    /// `explore`: never used, `root_result` does the work.
    fn explore(
        &self,
        _level: i32,
        _start: &Handle<StandardTransient>,
        _g: &InterfaceGraph,
        _explored: &mut InterfaceEntityIterator,
    ) -> Result<bool, Failure> {
        Ok(false)
    }
    fn explore_label(&self) -> AsciiString {
        AsciiString::new("Instances")
    }
}

impl IFSelectSelection for STEPSelectionsSelectInstances {
    /// `root_result(G)`: the instances, computed once for the model, filtered by the input.
    fn root_result(&self, g: &InterfaceGraph) -> Result<InterfaceEntityIterator, Failure> {
        let entities = INSTANCES.with(|cell| -> Result<InterfaceEntityIterator, Failure> {
            let mut cache = cell.borrow_mut();
            if cache.model.is_null() || cache.model != *g.model() {
                let mut found = InterfaceEntityIterator::new();
                for root in entities_of(g.root_entities()?)? {
                    add_instances(&root, g, &mut found)?;
                }
                cache.model = g.model().clone();
                cache.entities = found;
            }
            Ok(cache.entities.clone())
        })?;
        if !self.has_input() && !self.has_alternate() {
            return Ok(entities);
        }
        let selected = entities_of(self.input_result(g)?)?;
        let filter: HashSet<usize> = selected
            .iter()
            .map(|ent| ent.object_address() as usize)
            .collect();
        let mut result = InterfaceEntityIterator::new();
        for entity in entities_of(entities)? {
            if filter.contains(&(entity.object_address() as usize)) {
                result.add_item(&entity);
            }
        }
        Ok(result)
    }
    fn fill_iterator(&self, iter: &mut IFSelectSelectionIterator) {
        deduct_fill_iterator(self, iter)
    }
    fn label(&self) -> AsciiString {
        explore_label(self)
    }
    fn has_unique_result(&self) -> bool {
        true
    }
}
