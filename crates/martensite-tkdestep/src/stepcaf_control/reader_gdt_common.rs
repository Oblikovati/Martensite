//! The shapes and shape aspects GD&T entities refer to, in `STEPCAFControlReader`.
use super::reader::WHERE;
use super::reader_support::{SessionGraph, first_of};
use crate::step_data::StepDataSelectType;
use crate::step_dim_tol::{
    StepDimTolDatum, StepDimTolDatumFeature, StepDimTolDatumTarget,
    StepDimTolGeometricToleranceWithDatumReference,
};
use crate::step_entity::{step_array1_item, step_live};
use crate::step_repr::{
    StepReprDerivedShapeAspect, StepReprPropertyDefinition, StepReprRepresentationItem,
    StepReprShapeAspect, StepReprShapeAspectDerivingRelationship, StepReprShapeAspectRelationship,
};
use crate::step_shape::{
    StepShapeAdvancedFace, StepShapeDimensionalLocation, StepShapeShapeDefinitionRepresentation,
    StepShapeTopologicalRepresentationItem,
};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkxcaf::xcaf_doc::{XCAFDocDimTolTool, XCAFDocShapeTool};

/// `find_shape_index_for_dgt`: the index of an entity in the map of imported shapes; for a geometry item,
/// of its topological item.
pub(super) fn find_shape_index_for_dgt(
    entity: &Handle<StandardTransient>,
    session: &SessionGraph,
) -> Result<i32, Failure> {
    let process = session.process()?;
    // try to find index of given entity
    let index = process.map_index(entity);
    if index > 0 || entity.is_null() {
        return Ok(index);
    }
    // if theEnt is a geometry item try to find its topological item
    for sharing in session.sharings(entity)? {
        if sharing
            .cast::<StepShapeTopologicalRepresentationItem>()
            .is_some()
        {
            let index = process.map_index(&sharing);
            if index > 0 {
                return Ok(index);
            }
        }
    }
    Ok(0)
}

/// `collectShapeAspect`: the simple aspects under an aspect.
pub(super) fn collect_shape_aspect(
    aspect: &Handle<StepReprShapeAspect>,
    session: &SessionGraph,
    aspects: &mut Vec<Handle<StepReprShapeAspect>>,
) -> Result<(), Failure> {
    if aspect.is_null() {
        return Ok(());
    }
    // Retrieve Shape_Aspect, connected to Representation_Item from Derived_Shape_Aspect
    if aspect.cast::<StepReprDerivedShapeAspect>().is_some() {
        let deriving: Handle<StepReprShapeAspectDerivingRelationship> =
            first_of(&session.sharings(&aspect.cast())?);
        if let Some(deriving) = deriving.get() {
            collect_shape_aspect(&deriving.related_shape_aspect(), session, aspects)?;
        }
        return Ok(());
    }
    if aspect.cast::<StepDimTolDatumFeature>().is_some()
        || aspect.cast::<StepDimTolDatumTarget>().is_some()
    {
        aspects.push(aspect.clone());
        return Ok(());
    }
    // Find all children Shape_Aspect
    let mut is_simple = true;
    for sharing in session.sharings(&aspect.cast())? {
        let relationship = sharing.cast::<StepReprShapeAspectRelationship>();
        let Some(relationship) = relationship
            .get()
            .filter(|_| sharing.cast::<StepShapeDimensionalLocation>().is_null())
        else {
            continue;
        };
        let related = relationship.related_shape_aspect();
        if relationship.relating_shape_aspect() == *aspect
            && related.is_some()
            && related.cast::<StepDimTolDatum>().is_null()
        {
            collect_shape_aspect(&related, session, aspects)?;
            is_simple = false;
        }
    }
    // If not Composite_Shape_Aspect (or subtype) append to sequence.
    if is_simple {
        aspects.push(aspect.clone());
    }
    Ok(())
}

/// `getShapeLabel`: the label of the shape of an item, null when not found.
pub(super) fn shape_label_of(
    item: &Handle<StepReprRepresentationItem>,
    session: &SessionGraph,
    tool: &Handle<XCAFDocShapeTool>,
) -> Result<TdfLabel, Failure> {
    let index = find_shape_index_for_dgt(&item.cast(), session)?;
    let shape = session.shape_at(index)?;
    let mut label = TdfLabel::default();
    if !shape.is_null() {
        step_live(tool, WHERE)?.search(&shape, &mut label, [true, true, true])?;
    }
    Ok(label)
}

/// The datums of a tolerance's datum system.
fn datums_of_system(
    tolerance: &StepDimTolGeometricToleranceWithDatumReference,
) -> Result<Option<Vec<Handle<StepDimTolDatum>>>, Failure> {
    let systems = tolerance.datum_system_ap242();
    let Some(live) = systems.get() else {
        return Ok(None);
    };
    let mut datums = Vec::new();
    for rank in 1..=live.array1().length() {
        let select = step_array1_item(&systems, rank, WHERE)?;
        if select.is_null() {
            continue;
        }
        if let Some(system) = select.datum_system().get() {
            let constituents = system.constituents();
            for index in 1..=step_live(&constituents, WHERE)?.array1().length() {
                let compartment = step_array1_item(&constituents, index, WHERE)?;
                datums.push(step_live(&compartment, WHERE)?.base().datum());
            }
        } else if let Some(reference) = select.datum_reference().get() {
            datums.push(reference.referenced_datum());
        }
    }
    Ok(Some(datums))
}

/// `read_datums`: the datums of a tolerance in the first version of the structure; false when there is
/// none, or a datum feature has no advanced face.
pub(super) fn read_datums(
    shape_tool: &Handle<XCAFDocShapeTool>,
    dim_tol_tool: &XCAFDocDimTolTool,
    session: &SessionGraph,
    tolerance_label: &TdfLabel,
    tolerance: &Handle<StepDimTolGeometricToleranceWithDatumReference>,
) -> Result<bool, Failure> {
    let Some(live) = tolerance.get() else {
        return Ok(false);
    };
    let Some(datums) = datums_of_system(live)? else {
        return Ok(false);
    };
    let process = session.process()?;
    for datum in datums {
        let Some(live_datum) = datum.get() else {
            continue;
        };
        for sharing in session.sharings(&datum.cast())? {
            let relationship = sharing.cast::<StepReprShapeAspectRelationship>();
            let Some(relationship) = relationship.get() else {
                continue;
            };
            let feature = relationship
                .relating_shape_aspect()
                .cast::<StepDimTolDatumFeature>();
            if feature.is_null() {
                continue;
            }
            let definition: Handle<StepReprPropertyDefinition> =
                first_of(&session.sharings(&feature.cast())?);
            let Some(live_definition) = definition.get() else {
                continue;
            };
            let mut face = Handle::<StepShapeAdvancedFace>::default();
            for second in session.sharings(&definition.cast())? {
                if let Some(sdr) = second
                    .cast::<StepShapeShapeDefinitionRepresentation>()
                    .get()
                {
                    let representation = sdr.used_representation();
                    if let Some(live_rep) = representation.get().filter(|rep| rep.nb_items() > 0) {
                        face = live_rep.items_value(1)?.cast();
                    }
                }
            }
            if face.is_null() {
                return Ok(false);
            }
            let shape = session.shape_at(process.map_index(&face.cast()))?;
            if shape.is_null() {
                continue;
            }
            let mut label = TdfLabel::default();
            if !step_live(shape_tool, WHERE)?.search(&shape, &mut label, [true, true, true])? {
                continue;
            }
            dim_tol_tool.set_datum(
                &label,
                tolerance_label,
                &live_definition.name(),
                &live_definition.description(),
                &live_datum.identification(),
            )?;
        }
    }
    Ok(true)
}
