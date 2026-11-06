//! What the parts of `STEPCAFControlWriter` that write GD&T share: finding the product definition
//! shape of an entity, the unit of a context, the values of dimensions, shape aspects and presentations.
use super::STEPCAFControlGDTProperty;
use super::writer::{WHERE, WriterState};
use super::writer_support::{find_entities, finder_process_of, model_of};
use crate::geom_to_step::GeomToStepMakeAxis2Placement3d;
use crate::step_ap242::{
    StepAP242DraughtingModelItemAssociation, StepAP242GeometricItemSpecificUsage,
    StepAP242ItemIdentifiedRepresentationUsageDefinition,
};
use crate::step_basic::{
    StepBasicConversionBasedUnitAndLengthUnit, StepBasicConversionBasedUnitAndPlaneAngleUnit,
    StepBasicMeasureValueMember, StepBasicMeasureWithUnit, StepBasicNamedUnit,
    StepBasicSiUnitAndLengthUnit, StepBasicSiUnitAndPlaneAngleUnit, StepBasicUnit,
};
use crate::step_data::{
    StepDataFactors, StepDataLogical, StepDataSelectMember, StepDataSelectType,
};
use crate::step_entity::{
    HArray1, step_array1_item, step_array1_len, step_array1_set, step_live, step_new_array1,
};
use crate::step_geom::{
    StepGeomCartesianPoint, StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx,
    StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext,
    StepGeomGeometricRepresentationItem, StepGeomPlane,
};
use crate::step_repr::{
    StepReprProductDefinitionShape, StepReprReprItemAndLengthMeasureWithUnit,
    StepReprReprItemAndLengthMeasureWithUnitAndQRI, StepReprReprItemAndMeasureWithUnit,
    StepReprReprItemAndPlaneAngleMeasureWithUnit,
    StepReprReprItemAndPlaneAngleMeasureWithUnitAndQRI, StepReprRepresentationContext,
    StepReprRepresentationItem, StepReprShapeAspect,
};
use crate::step_shape::{
    StepShapeAdvancedFace, StepShapeConnectedFaceSet, StepShapeEdgeCurve, StepShapeEdgeLoop,
    StepShapeFaceBound, StepShapeOrientedEdge, StepShapeQualifiedRepresentationItem,
    StepShapeShapeDefinitionRepresentation, StepShapeShapeRepresentation,
    StepShapeTopologicalRepresentationItem,
};
use crate::step_visual::{
    StepVisualAnnotationPlane, StepVisualAnnotationPlaneElement, StepVisualDraughtingCallout,
    StepVisualDraughtingCalloutElement, StepVisualNullStyleMember,
    StepVisualPresentationStyleAssignment, StepVisualPresentationStyleSelect,
    StepVisualTessellatedAnnotationOccurrence,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkernel::tcollection::{AsciiString, HAsciiString};
use martensite_tklcaf::tdata_std::{TDataStdGenericExtString, TDataStdName};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkmath::gp::{Axis2, Point3};
use martensite_tkmath::toploc::Location;
use martensite_tkxsbase::interface::{InterfaceHGraph, InterfaceModelEntities};
use martensite_tkxsbase::xs_control::XSControlWorkSession;

mod aspects;
#[allow(unused_imports, reason = "what the sibling modules use")]
pub use aspects::*;

pub(super) fn text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

pub(super) fn empty() -> Handle<HAsciiString> {
    Handle::new(HAsciiString::default())
}

/// The bytes of a string, for a key of a map.
pub(super) fn bytes_of(value: &Handle<HAsciiString>) -> Result<Vec<u8>, Failure> {
    Ok(step_live(value, WHERE)?.string().as_bytes().to_vec())
}

/// `theWS->HGraph()`: the graph of the session, up to date with the model; null on an empty model, which
/// the writers test, where `Graph()` would raise.
pub(super) fn graph_of(
    ws: &Handle<XSControlWorkSession>,
) -> Result<Handle<InterfaceHGraph>, Failure> {
    step_live(ws, WHERE)?.h_graph()
}

/// `sharings(entity)`.
pub(super) fn sharings_of<T: RuntimeTyped>(
    graph: &Handle<InterfaceHGraph>,
    entity: &Handle<T>,
) -> Result<Vec<Handle<StandardTransient>>, Failure> {
    Ok(step_live(graph, WHERE)?
        .graph()
        .sharings(&entity.cast())?
        .content())
}

/// The entities sharing `entity` that are of type `T`.
fn sharing_of_type<S: RuntimeTyped, T: RuntimeTyped>(
    graph: &Handle<InterfaceHGraph>,
    entity: &Handle<S>,
) -> Result<Vec<Handle<T>>, Failure> {
    Ok(sharings_of(graph, entity)?
        .iter()
        .map(|sharing| sharing.cast::<T>())
        .filter(Handle::is_some)
        .collect())
}

/// `FindPDSforDGT`, from an edge up to its face: the first advanced face.
fn advanced_face_of_edge(
    graph: &Handle<InterfaceHGraph>,
    edge: &Handle<StepShapeEdgeCurve>,
) -> Result<Handle<StepShapeAdvancedFace>, Failure> {
    for oriented in sharing_of_type::<_, StepShapeOrientedEdge>(graph, edge)? {
        for edge_loop in sharing_of_type::<_, StepShapeEdgeLoop>(graph, &oriented)? {
            for bound in sharing_of_type::<_, StepShapeFaceBound>(graph, &edge_loop)? {
                let faces = sharing_of_type::<_, StepShapeAdvancedFace>(graph, &bound)?;
                if let Some(face) = faces.into_iter().next() {
                    return Ok(face);
                }
            }
        }
    }
    Ok(Handle::default())
}

/// The product definition shape of an SDR: null when its property is another.
fn pds_of_sdr(
    sdr: &Handle<StepShapeShapeDefinitionRepresentation>,
) -> Result<Handle<StepReprProductDefinitionShape>, Failure> {
    Ok(step_live(sdr, WHERE)?
        .definition()
        .property_definition()
        .cast())
}

/// The product definition shape of the first SDR of a shape representation.
fn pds_of_representation(
    graph: &Handle<InterfaceHGraph>,
    representation: &Handle<StepShapeShapeRepresentation>,
) -> Result<Handle<StepReprProductDefinitionShape>, Failure> {
    for sdr in sharing_of_type::<_, StepShapeShapeDefinitionRepresentation>(graph, representation)?
    {
        let pds = pds_of_sdr(&sdr)?;
        if pds.is_some() {
            return Ok(pds);
        }
    }
    Ok(Handle::default())
}

/// `FindPDSforDGT`: the face of an edge or a face, and the product definition shape of the face. The
/// face is null, and nothing found, for another entity.
pub(super) fn find_pds_for_dgt(
    graph: &Handle<InterfaceHGraph>,
    entity: &Handle<StandardTransient>,
    pds: &mut Handle<StepReprProductDefinitionShape>,
    context: &mut Handle<StepReprRepresentationContext>,
    face: &mut Handle<StepShapeAdvancedFace>,
    edge: &mut Handle<StepShapeEdgeCurve>,
) -> Result<bool, Failure> {
    *face = entity.cast();
    if face.is_null() {
        *edge = entity.cast();
        if edge.is_null() {
            return Ok(false);
        }
        *face = advanced_face_of_edge(graph, edge)?;
    }
    if face.is_null() {
        return Ok(false);
    }
    for set in sharing_of_type::<_, StepShapeConnectedFaceSet>(graph, face)? {
        for item in sharing_of_type::<_, StepReprRepresentationItem>(graph, &set)? {
            for representation in sharing_of_type::<_, StepShapeShapeRepresentation>(graph, &item)?
            {
                *context = step_live(&representation, WHERE)?.context_of_items();
                *pds = pds_of_representation(graph, &representation)?;
                if pds.is_some() {
                    return Ok(true);
                }
            }
        }
    }
    Ok(true)
}

/// `find_pds`: the product definition shape of an entity, through its shape representation or the
/// items sharing it.
pub(super) fn find_pds(
    graph: &Handle<InterfaceHGraph>,
    entity: &Handle<StandardTransient>,
    context: &mut Handle<StepReprRepresentationContext>,
) -> Result<Handle<StepReprProductDefinitionShape>, Failure> {
    if entity.is_null() {
        return Ok(Handle::default());
    }
    // try to find shape_representation in sharings
    for representation in sharing_of_type::<_, StepShapeShapeRepresentation>(graph, entity)? {
        *context = step_live(&representation, WHERE)?.context_of_items();
        let pds = pds_of_representation(graph, &representation)?;
        if pds.is_some() {
            return Ok(pds);
        }
    }
    for sharing in sharings_of(graph, entity)? {
        if sharing
            .cast::<StepShapeTopologicalRepresentationItem>()
            .is_some()
            || sharing
                .cast::<StepGeomGeometricRepresentationItem>()
                .is_some()
        {
            let pds = find_pds(graph, &sharing, context)?;
            if pds.is_some() {
                return Ok(pds);
            }
        }
    }
    Ok(Handle::default())
}

/// The first unit of a context the test accepts, in either of the two context types.
fn unit_of_context(
    context: &Handle<StepReprRepresentationContext>,
    accepts: impl Fn(&Handle<StepBasicNamedUnit>) -> bool,
) -> Result<Handle<StepBasicNamedUnit>, Failure> {
    let first: Handle<StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext> =
        context.cast();
    let mut units = first.get().map(|live| live.units()).transpose()?;
    let found = first_accepted(units.as_ref(), &accepts)?;
    if found.is_some() {
        return Ok(found);
    }
    let second: Handle<StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx> =
        context.cast();
    units = second.get().map(|live| live.units()).transpose()?;
    first_accepted(units.as_ref(), &accepts)
}

fn first_accepted(
    units: Option<&Handle<HArray1<Handle<StepBasicNamedUnit>>>>,
    accepts: &impl Fn(&Handle<StepBasicNamedUnit>) -> bool,
) -> Result<Handle<StepBasicNamedUnit>, Failure> {
    let Some(units) = units else {
        return Ok(Handle::default());
    };
    for index in 1..=step_array1_len(units) {
        let unit = step_array1_item(units, index, WHERE)?;
        if accepts(&unit) {
            return Ok(unit);
        }
    }
    Ok(Handle::default())
}

/// `get_unit`: the plane angle or length unit of a context, a new SI unit when it has none.
pub(super) fn get_unit(
    context: &Handle<StepReprRepresentationContext>,
    is_angle: bool,
) -> Result<StepBasicUnit, Failure> {
    let current = if is_angle {
        let found = unit_of_context(context, |unit| {
            unit.cast::<StepBasicSiUnitAndPlaneAngleUnit>().is_some()
                || unit
                    .cast::<StepBasicConversionBasedUnitAndPlaneAngleUnit>()
                    .is_some()
        })?;
        if found.is_some() {
            found
        } else {
            Handle::new(StepBasicSiUnitAndPlaneAngleUnit::new()).cast()
        }
    } else {
        let found = unit_of_context(context, |unit| {
            unit.cast::<StepBasicSiUnitAndLengthUnit>().is_some()
                || unit
                    .cast::<StepBasicConversionBasedUnitAndLengthUnit>()
                    .is_some()
        })?;
        if found.is_some() {
            found
        } else {
            Handle::new(StepBasicSiUnitAndLengthUnit::new()).cast()
        }
    };
    let mut unit = StepBasicUnit::default();
    unit.set_value(&current.cast())?;
    Ok(unit)
}
