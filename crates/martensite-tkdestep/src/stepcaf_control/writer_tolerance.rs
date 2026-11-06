//! `STEPCAFControlWriter`: the datum system of a tolerance, its tolerance zone and the entity of a
//! geometric tolerance.
use super::STEPCAFControlGDTProperty;
use super::writer::{WHERE, WriterState};
use super::writer_datum::relate_aspects;
use super::writer_gdt_common::{
    GdtPresentation, empty, get_unit, graph_of, one_item, sharings_of, text,
};
use super::writer_support::model_of;
use crate::geom_to_step::GeomToStepMakeAxis2Placement3d;
use crate::step_ap242::{
    StepAP242GeometricItemSpecificUsage, StepAP242ItemIdentifiedRepresentationUsageDefinition,
};
use crate::step_basic::{
    StepBasicLengthMeasureWithUnit, StepBasicMeasureValueMember,
    StepBasicPlaneAngleMeasureWithUnit, StepBasicUnit,
};
use crate::step_data::{
    StepDataFactors, StepDataLogical, StepDataSelectMember, StepDataSelectType,
};
use crate::step_dim_tol::{
    StepDimTolDatum, StepDimTolDatumOrCommonDatum, StepDimTolDatumReferenceCompartment,
    StepDimTolDatumReferenceElement, StepDimTolDatumReferenceModifier, StepDimTolDatumSystem,
    StepDimTolDatumSystemOrReference, StepDimTolGeoTolAndGeoTolWthDatRef,
    StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol,
    StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMod, StepDimTolGeoTolAndGeoTolWthMaxTol,
    StepDimTolGeoTolAndGeoTolWthMod, StepDimTolGeometricTolerance,
    StepDimTolGeometricToleranceModifier, StepDimTolGeometricToleranceTarget,
    StepDimTolGeometricToleranceWithDatumReference, StepDimTolGeometricToleranceWithModifiers,
    StepDimTolRunoutZoneDefinition, StepDimTolRunoutZoneOrientation, StepDimTolToleranceZone,
    StepDimTolToleranceZoneForm, StepDimTolToleranceZoneTarget,
};
use crate::step_entity::{
    HArray1, step_array1_item, step_array1_len, step_array1_set, step_live, step_new_array1,
};
use crate::step_repr::{
    StepReprCompositeShapeAspect, StepReprRepresentationContext, StepReprShapeAspect,
};
use crate::step_shape::StepShapeShapeDefinitionRepresentation;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkxcaf::xcaf_dim_tol_objects::{
    XCAFDimTolObjectsDatumModifWithValue, XCAFDimTolObjectsDatumObject,
    XCAFDimTolObjectsGeomToleranceMatReqModif, XCAFDimTolObjectsGeomToleranceModif,
    XCAFDimTolObjectsGeomToleranceObject, XCAFDimTolObjectsGeomToleranceTypeValue,
    XCAFDimTolObjectsGeomToleranceZoneModif,
};
use martensite_tkxcaf::xcaf_doc::{XCAFDocDatum, XCAFDocGeomTolerance, XCAFDocShapeTool};
use martensite_tkxsbase::interface::InterfaceModelEntities;
use martensite_tkxsbase::xs_control::XSControlWorkSession;
use std::collections::HashMap;

mod tolerance_entity;
#[allow(unused_imports, reason = "what the sibling modules use")]
pub use tolerance_entity::*;

/// The written datums by their names.
pub(super) type DatumMap = HashMap<Vec<u8>, Handle<StandardTransient>>;

/// The objects of the datums of a tolerance, and the largest position among them.
fn datum_objects(
    datums: &[TdfLabel],
) -> Result<(Vec<Handle<XCAFDimTolObjectsDatumObject>>, i32), Failure> {
    let mut objects = Vec::new();
    let mut max_position = 0;
    for label in datums {
        let mut attribute: Handle<XCAFDocDatum> = Handle::default();
        if !label.find_attribute_as(&XCAFDocDatum::get_id(), &mut attribute)? {
            continue;
        }
        let object = step_live(&attribute, WHERE)?.get_object()?;
        let Some(live) = object.get() else {
            continue;
        };
        max_position = max_position.max(live.get_position());
        objects.push(object);
    }
    Ok((objects, max_position))
}

/// `get_datum_ref_modifiers` of a datum, with its modifier with value added to the model.
fn datum_ref_modifiers(
    ws: &Handle<XSControlWorkSession>,
    object: &XCAFDimTolObjectsDatumObject,
    unit: &StepBasicUnit,
) -> Result<Handle<HArray1<StepDimTolDatumReferenceModifier>>, Failure> {
    let (with_value, value): (XCAFDimTolObjectsDatumModifWithValue, f64) =
        object.get_modifier_with_value();
    let modifiers = STEPCAFControlGDTProperty::get_datum_ref_modifiers(
        &object.get_modifiers(),
        with_value,
        value,
        unit,
    )?;
    // Add Datum_Reference_Modifier_With_Value
    if modifiers.is_some() {
        let last = step_array1_item(&modifiers, step_array1_len(&modifiers), WHERE)?;
        let with = last.datum_reference_modifier_with_value();
        if with.is_some() {
            model_of(ws)?.add_with_refs(&with.cast(), 0, false)?;
        }
    }
    Ok(modifiers)
}

/// A compartment of the datums at one position; `first_datum` keeps the datum of the system.
fn write_compartment(
    ws: &Handle<XSControlWorkSession>,
    at_position: &[&Handle<XCAFDimTolObjectsDatumObject>],
    datum_map: &DatumMap,
    unit: &StepBasicUnit,
    first_datum: &mut Handle<StepDimTolDatum>,
) -> Result<Handle<StepDimTolDatumReferenceCompartment>, Failure> {
    let model = model_of(ws)?;
    let mut reference = StepDimTolDatumOrCommonDatum::default();
    let mut modifiers = Handle::default();
    if let [single] = at_position {
        // Datum entity
        let object = step_live(single, WHERE)?;
        if let Some(found) = datum_map
            .get(&super::writer_gdt_common::bytes_of(&object.get_name())?)
            .filter(|found| found.is_some())
        {
            *first_datum = found.cast();
        }
        reference.set_value(&first_datum.cast())?;
        // Modifiers
        modifiers = datum_ref_modifiers(ws, object, unit)?;
    } else {
        let common = step_new_array1::<Handle<StepDimTolDatumReferenceElement>>(
            1,
            at_position.len() as i32,
        )?;
        for (index, datum_object) in at_position.iter().enumerate() {
            // Datum entity
            let object = step_live(datum_object, WHERE)?;
            let datum: Handle<StepDimTolDatum> = datum_map
                .get(&super::writer_gdt_common::bytes_of(&object.get_name())?)
                .map_or_else(Handle::default, Handle::cast);
            let mut element_reference = StepDimTolDatumOrCommonDatum::default();
            element_reference.set_value(&datum.cast())?;
            if first_datum.is_null() {
                *first_datum = datum.clone();
            }
            // Modifiers
            let element_modifiers = datum_ref_modifiers(ws, object, unit)?;
            // Datum_Reference_Element
            let element = Handle::new(StepDimTolDatumReferenceElement::new());
            let live_datum = step_live(&datum, WHERE)?;
            element.init(
                &empty(),
                &empty(),
                &live_datum.of_shape(),
                live_datum.product_definitional(),
                &element_reference,
                element_modifiers.is_some(),
                &element_modifiers,
            );
            model.add_with_refs(&element.cast(), 0, false)?;
            step_array1_set(&common, index as i32 + 1, element, WHERE)?;
        }
        reference.set_value(&common.cast())?;
    }
    let compartment = Handle::new(StepDimTolDatumReferenceCompartment::new());
    let live_first = step_live(first_datum, WHERE)?;
    compartment.init(
        &empty(),
        &empty(),
        &live_first.of_shape(),
        live_first.product_definitional(),
        &reference,
        modifiers.is_some(),
        &modifiers,
    );
    model.add_with_refs(&compartment.cast(), 0, false)?;
    Ok(compartment)
}

/// The axis of a tolerance, as a usage of the datum system.
fn write_system_axis(
    ws: &Handle<XSControlWorkSession>,
    object: &XCAFDimTolObjectsGeomToleranceObject,
    system: &Handle<StepDimTolDatumSystem>,
    first_datum: &Handle<StepDimTolDatum>,
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    let model = model_of(ws)?;
    let graph = graph_of(ws)?;
    let axis = GeomToStepMakeAxis2Placement3d::from_ax2(&object.get_axis(), factors)?.value()?;
    step_live(&axis, WHERE)?.set_name(&text("orientation"));
    let usage = Handle::new(StepAP242GeometricItemSpecificUsage::new());
    let mut definition = StepAP242ItemIdentifiedRepresentationUsageDefinition::default();
    definition.set_value(&system.cast())?;
    let items = one_item(&axis.cast())?;
    let of_shape = step_live(first_datum, WHERE)?.of_shape();
    let sdr = sharings_of(&graph, &of_shape)?
        .iter()
        .map(|sharing| sharing.cast::<StepShapeShapeDefinitionRepresentation>())
        .find(Handle::is_some);
    let Some(sdr) = sdr else {
        return Ok(());
    };
    let used = step_live(&sdr, WHERE)?.used_representation();
    usage.init(&empty(), &empty(), &definition, &used, &items);
    model.add_with_refs(&axis.cast(), 0, false)?;
    model.add_with_refs(&usage.cast(), 0, false)
}

/// `write_datum_system`: the datum system of a tolerance; null without datums with a position.
pub(super) fn write_datum_system(
    ws: &Handle<XSControlWorkSession>,
    tolerance_label: &TdfLabel,
    datums: &[TdfLabel],
    datum_map: &DatumMap,
    context: &Handle<StepReprRepresentationContext>,
    factors: &StepDataFactors,
) -> Result<Handle<HArray1<StepDimTolDatumSystemOrReference>>, Failure> {
    let model = model_of(ws)?;
    if graph_of(ws)?.is_null() {
        return Ok(Handle::default());
    }
    let mut attribute: Handle<XCAFDocGeomTolerance> = Handle::default();
    if !tolerance_label.find_attribute_as(&XCAFDocGeomTolerance::get_id(), &mut attribute)? {
        return Ok(Handle::default());
    }
    let object = step_live(&attribute, WHERE)?.get_object()?;
    let Some(live) = object.get() else {
        return Ok(Handle::default());
    };
    // Unit
    let unit = get_unit(context, false)?;
    let (objects, max_position) = datum_objects(datums)?;
    if max_position == 0 {
        return Ok(Handle::default());
    }
    // Auxiliary datum to initialize attributes in Datum_System
    let mut first_datum = Handle::default();
    let mut compartments = Vec::new();
    for position in 1..=max_position {
        // Collect datums with i-th position
        let mut at_position = Vec::new();
        for datum_object in &objects {
            if step_live(datum_object, WHERE)?.get_position() == position {
                at_position.push(datum_object);
            }
        }
        if !at_position.is_empty() {
            compartments.push(write_compartment(
                ws,
                &at_position,
                datum_map,
                &unit,
                &mut first_datum,
            )?);
        }
    }
    // Remove null elements from aConstituents: only those at a position were collected
    let constituents = step_new_array1::<Handle<StepDimTolDatumReferenceCompartment>>(
        1,
        compartments.len() as i32,
    )?;
    for (index, compartment) in compartments.into_iter().enumerate() {
        step_array1_set(&constituents, index as i32 + 1, compartment, WHERE)?;
    }
    let system = Handle::new(StepDimTolDatumSystem::new());
    let live_first = step_live(&first_datum, WHERE)?;
    system.init(
        &empty(),
        &empty(),
        &live_first.of_shape(),
        live_first.product_definitional(),
        &constituents,
    );
    model.add_with_refs(&system.cast(), 0, false)?;
    let mut value = StepDimTolDatumSystemOrReference::default();
    value.set_value(&system.cast())?;
    let result = step_new_array1::<StepDimTolDatumSystemOrReference>(1, 1)?;
    step_array1_set(&result, 1, value, WHERE)?;
    // Axis
    if live.has_axis() {
        write_system_axis(ws, live, &system, &first_datum, factors)?;
    }
    Ok(result)
}
