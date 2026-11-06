//! `StepControlReader::nb_roots_for_transfer`: which entities of the model are transfer roots.
use super::reader::RootEntity;
use crate::step_basic::{
    StepBasicDocumentProductEquivalence, StepBasicProductDefinition,
    StepBasicProductDefinitionContext, StepBasicProductDefinitionFormation,
    StepBasicProductDefinitionWithAssociatedDocuments,
};
use crate::step_data::StepDataStepModel;
use crate::step_entity::step_live;
use crate::step_repr::{
    StepReprMappedItem, StepReprNextAssemblyUsageOccurrence, StepReprRepresentationMap,
    StepReprRepresentationRelationship,
};
use crate::step_shape::{
    StepShapeManifoldSolidBrep, StepShapeShapeDefinitionRepresentation,
    StepShapeShapeRepresentation, StepShapeShellBasedSurfaceModel,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkxsbase::interface::{
    InterfaceEntityIterator, InterfaceGraph, InterfaceModelEntities,
};
use martensite_tkxsbase::xs_control::XSControlWorkSession;

/// `InterfaceEntityIterator` as a list.
fn listed(iterator: InterfaceEntityIterator) -> Result<Vec<RootEntity>, Failure> {
    let mut entities = Vec::new();
    iterator.start();
    while iterator.more() {
        entities.push(iterator.value()?);
        iterator.next();
    }
    Ok(entities)
}

/// PTV 31.01.2003 TRJ11: exclude Product Definition With Associated Document from roots: check if
/// PDWAD-> PDF <-Document_Product_Equivalence.
fn is_document_product(ent: &RootEntity, graph: &InterfaceGraph) -> Result<bool, Failure> {
    if ent
        .cast::<StepBasicProductDefinitionWithAssociatedDocuments>()
        .is_null()
    {
        return Ok(false);
    }
    for shared in listed(graph.shareds(ent)?)? {
        if shared
            .cast::<StepBasicProductDefinitionFormation>()
            .is_null()
        {
            continue;
        }
        let sharings = listed(graph.sharings(&shared)?)?;
        if sharings
            .iter()
            .any(|sub| sub.cast::<StepBasicDocumentProductEquivalence>().is_some())
        {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Whether a product_definition is a root: not used by an assembly, and of the context asked for.
fn product_is_root(
    ent: &RootEntity,
    graph: &InterfaceGraph,
    model: &StepDataStepModel,
) -> Result<bool, Failure> {
    let pd: Handle<StepBasicProductDefinition> = ent.cast();
    // determinate roots used NextAssemblyUsageOccurrence
    for sharing in listed(graph.sharings(ent)?)? {
        let nauo: Handle<StepReprNextAssemblyUsageOccurrence> = sharing.cast();
        if nauo
            .get()
            .is_some_and(|live| pd == live.related_product_definition())
        {
            return Ok(false);
        }
    }
    // determinate roots used ProductDefinitionContext
    let (context_text, mode) = {
        let parameters = model.internal_parameters();
        let mode = parameters.read_product_context;
        (parameters.get_string(mode), mode.as_raw())
    };
    if mode <= 1 {
        return Ok(true);
    }
    let wanted = String::from_utf8_lossy(context_text.as_bytes()).to_ascii_lowercase();
    let mut is_root = true;
    for shared in listed(graph.shareds(ent)?)? {
        let context: Handle<StepBasicProductDefinitionContext> = shared.cast();
        let Some(live) = context.get() else {
            continue;
        };
        let stage = live.life_cycle_stage();
        let stage_text = step_live(&stage, "StepControlReader")?.string().clone();
        let name = live.name();
        let name_text = step_live(&name, "StepControlReader")?.string().clone();
        let matches = |text: &[u8]| String::from_utf8_lossy(text).to_ascii_lowercase() == wanted;
        if !matches(stage_text.as_bytes()) && !matches(name_text.as_bytes()) {
            is_root = false;
        }
    }
    Ok(is_root)
}

/// Whether a shape_definition_representation is a root when products are not read.
fn sdr_is_root(ent: &RootEntity) -> Result<bool, Failure> {
    let sdr: Handle<StepShapeShapeDefinitionRepresentation> = ent.cast();
    let live = step_live(&sdr, "StepControlReader")?;
    let definition = live.definition().property_definition();
    let Some(live_definition) = definition.get() else {
        return Ok(true);
    };
    let select = live_definition.definition();
    if select.product_definition().is_some() {
        return Ok(false);
    }
    let aspect = select.shape_aspect();
    if let Some(live_aspect) = aspect.get() {
        let of_shape = live_aspect.of_shape();
        if step_live(&of_shape, "StepControlReader")?
            .definition()
            .product_definition()
            .is_some()
        {
            return Ok(false);
        }
    }
    let nauo: Handle<StepReprNextAssemblyUsageOccurrence> =
        select.product_definition_relationship().cast();
    if nauo.is_some() {
        return Ok(false);
    }
    Ok(live
        .used_representation()
        .cast::<StepShapeShapeRepresentation>()
        .is_some())
}

/// A relationship of `sr` whose other representation has a shape_definition_representation.
fn related_has_sdr(
    sr: &Handle<StepShapeShapeRepresentation>,
    relationship: &Handle<StepReprRepresentationRelationship>,
    graph: &InterfaceGraph,
) -> Result<bool, Failure> {
    let live = step_live(relationship, "StepControlReader")?;
    let mut other: Handle<StepShapeShapeRepresentation> = live.rep1().cast();
    if *sr == other {
        other = live.rep2().cast();
    }
    if other.is_null() {
        return Ok(false);
    }
    let sharings = listed(graph.sharings(&other.cast())?)?;
    Ok(sharings.iter().any(|ent| {
        ent.cast::<StepShapeShapeDefinitionRepresentation>()
            .is_some()
    }))
}

/// A representation map of `sr` placed in another shape_representation.
fn mapped_into_representation(
    map: &Handle<StepReprRepresentationMap>,
    graph: &InterfaceGraph,
) -> Result<bool, Failure> {
    for sharing in listed(graph.sharings(&map.cast())?)? {
        if sharing.cast::<StepReprMappedItem>().is_null() {
            continue;
        }
        for user in listed(graph.sharings(&sharing)?)? {
            if user.cast::<StepShapeShapeRepresentation>().is_some() {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// Whether a shape_representation is a root when products are not read.
fn shape_representation_is_root(ent: &RootEntity, graph: &InterfaceGraph) -> Result<bool, Failure> {
    let sr: Handle<StepShapeShapeRepresentation> = ent.cast();
    for sharing in listed(graph.sharings(ent)?)? {
        if sharing
            .cast::<StepShapeShapeDefinitionRepresentation>()
            .is_some()
        {
            return Ok(false);
        }
        let relationship: Handle<StepReprRepresentationRelationship> = sharing.cast();
        if relationship.is_some() && related_has_sdr(&sr, &relationship, graph)? {
            return Ok(false);
        }
        let map: Handle<StepReprRepresentationMap> = sharing.cast();
        if map.is_some() && mapped_into_representation(&map, graph)? {
            return Ok(false);
        }
    }
    Ok(true)
}

/// The loop over the model.
pub(super) fn roots_for_transfer(
    ws: &Handle<XSControlWorkSession>,
) -> Result<Vec<RootEntity>, Failure> {
    let session = step_live(ws, "StepControlReader")?;
    let generic = session.model();
    let model: Handle<StepDataStepModel> = generic.cast();
    let live_model = step_live(&model, "StepControlReader")?;
    let hgraph = session.graph()?;
    let graph_holder = step_live(&hgraph, "StepControlReader")?.graph();
    let graph = &*graph_holder;
    let reader = session.transfer_reader();
    let tp = step_live(&reader, "StepControlReader")?.transient_process();
    let live_tp = step_live(&tp, "StepControlReader")?;
    let (read_all, product_mode) = {
        let parameters = live_model.internal_parameters();
        (parameters.read_all_shapes, parameters.read_product_mode)
    };
    let mut roots = Vec::new();
    let nb = live_model.nb_entities();
    for rank in 1..=nb {
        let ent = live_model.value(rank)?;
        let mut found = Vec::new();
        // Special case to read invalid shape_representation without links to shapes.
        if read_all
            && (ent.cast::<StepShapeManifoldSolidBrep>().is_some()
                || ent.cast::<StepShapeShellBasedSurfaceModel>().is_some())
            && !graph.sharings(&ent)?.more()
        {
            found.push(ent.clone());
        }
        if ent.cast::<StepBasicProductDefinition>().is_some()
            && !is_document_product(&ent, graph)?
            && product_is_root(&ent, graph, live_model)?
        {
            found.push(ent.clone());
        }
        if !product_mode {
            if ent
                .cast::<StepShapeShapeDefinitionRepresentation>()
                .is_some()
                && sdr_is_root(&ent)?
            {
                found.push(ent.clone());
            }
            if ent.cast::<StepShapeShapeRepresentation>().is_some()
                && shape_representation_is_root(&ent, graph)?
            {
                found.push(ent.clone());
            }
        }
        for root in found {
            live_tp.roots_for_transfer().write().push(root.clone());
            roots.push(root);
        }
    }
    Ok(roots)
}
