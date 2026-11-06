//! `STEPCAFControlWriter::writeDGTs`: the GD&T of the first version of the XDE attributes, for the
//! schemas before AP242.
use super::writer::{WHERE, WriterState};
use super::writer_gdt_common::{
    bytes_of, empty, find_pds_for_dgt, get_unit, graph_of, one_item, text,
};
use super::writer_support::{find_entities, finder_process_of, model_of};
use crate::step_basic::{
    StepBasicLengthMeasureWithUnit, StepBasicMeasureValueMember, StepBasicMeasureWithUnit,
    StepBasicUnit,
};
use crate::step_data::{StepDataLogical, StepDataSelectMember, StepDataSelectType};
use crate::step_dim_tol::{
    StepDimTolAngularityTolerance, StepDimTolCircularRunoutTolerance,
    StepDimTolCoaxialityTolerance, StepDimTolConcentricityTolerance, StepDimTolDatum,
    StepDimTolDatumFeature, StepDimTolDatumReference,
    StepDimTolGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol,
    StepDimTolGeometricToleranceWithDatumReference, StepDimTolLimitCondition,
    StepDimTolModifiedGeometricTolerance, StepDimTolParallelismTolerance,
    StepDimTolPerpendicularityTolerance, StepDimTolSymmetryTolerance,
    StepDimTolTotalRunoutTolerance,
};
use crate::step_entity::{
    HArray1, step_array1_item, step_array1_len, step_array1_set, step_live, step_new_array1,
};
use crate::step_repr::{
    StepReprCharacterizedDefinition, StepReprProductDefinitionShape, StepReprPropertyDefinition,
    StepReprReprItemAndLengthMeasureWithUnit, StepReprRepresentationContext,
    StepReprRepresentationItem, StepReprRepresentedDefinition, StepReprShapeAspect,
    StepReprShapeAspectRelationship, StepReprValueRange,
};
use crate::step_shape::{
    StepShapeAdvancedFace, StepShapeDimensionalCharacteristic,
    StepShapeDimensionalCharacteristicRepresentation, StepShapeDimensionalSize, StepShapeEdgeCurve,
    StepShapeShapeDefinitionRepresentation, StepShapeShapeDimensionRepresentation,
    StepShapeShapeRepresentation,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkmath::toploc::Location;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocDatum, XCAFDocDimTol, XCAFDocDimTolTool, XCAFDocDocumentTool, XCAFDocShapeTool,
};
use martensite_tkxsbase::interface::{InterfaceInterfaceModel, InterfaceModelEntities};
use martensite_tkxsbase::xs_control::XSControlWorkSession;
use std::collections::HashMap;

/// The written datums by name, description and identification.
type LegacyDatumMap = HashMap<Vec<u8>, Handle<StepDimTolDatum>>;

/// The product definition shape, context, face and edge of the first shape of a GD&T label.
#[derive(Default)]
struct DgtTarget {
    pds: Handle<StepReprProductDefinitionShape>,
    context: Handle<StepReprRepresentationContext>,
    face: Handle<StepShapeAdvancedFace>,
    edge: Handle<StepShapeEdgeCurve>,
}

/// The target of a GD&T label: `None` without a shape, an entity or a product definition shape.
fn target_of(
    ws: &Handle<XSControlWorkSession>,
    label: &TdfLabel,
) -> Result<Option<DgtTarget>, Failure> {
    let (mut shapes, mut null_sequence) = (Vec::new(), Vec::new());
    if !XCAFDocDimTolTool::get_ref_shape_label(label, &mut shapes, &mut null_sequence)? {
        return Ok(None);
    }
    // find target shape
    let mut shape = Shape::default();
    XCAFDocShapeTool::get_shape(&shapes[0], &mut shape)?;
    let mut location = Location::default();
    let mut items = Vec::new();
    find_entities(&finder_process_of(ws)?, &shape, &mut location, &mut items)?;
    let Some(entity) = items.first() else {
        return Ok(None);
    };
    let mut target = DgtTarget::default();
    find_pds_for_dgt(
        &graph_of(ws)?,
        entity,
        &mut target.pds,
        &mut target.context,
        &mut target.face,
        &mut target.edge,
    )?;
    Ok(target.pds.is_some().then_some(target))
}

/// A property of a shape aspect, represented with the shape representation of an item. The recorded
/// chain of a datum adds the representation before its SDR, which orders the entities of the file.
fn write_characterized_chain(
    model: &Handle<dyn InterfaceInterfaceModel>,
    name: &Handle<HAsciiString>,
    description: &Handle<HAsciiString>,
    aspect: &Handle<StepReprShapeAspect>,
    item: &Handle<StepReprRepresentationItem>,
    context: &Handle<StepReprRepresentationContext>,
    add_representation_first: bool,
) -> Result<(), Failure> {
    let mut characterized = StepReprCharacterizedDefinition::default();
    characterized.set_value(&aspect.cast())?;
    let property = Handle::new(StepReprPropertyDefinition::new());
    property.init(name, true, description, &characterized);
    model.add_with_refs(&property.cast(), 0, false)?;
    let mut represented = StepReprRepresentedDefinition::default();
    represented.set_value(&property.cast())?;
    let representation = Handle::new(StepShapeShapeRepresentation::new());
    representation.init(name, &one_item(item)?, context);
    if add_representation_first {
        model.add_with_refs(&representation.cast(), 0, false)?;
    }
    let sdr = Handle::new(StepShapeShapeDefinitionRepresentation::new());
    sdr.init(&represented, &representation.cast());
    model.add_with_refs(&sdr.cast(), 0, false)
}

/// The key of a datum in the map: its name, description and identification.
fn datum_key(
    name: &Handle<HAsciiString>,
    description: &Handle<HAsciiString>,
    identification: &Handle<HAsciiString>,
) -> Result<Vec<u8>, Failure> {
    let mut key = bytes_of(name)?;
    key.extend(bytes_of(description)?);
    key.extend(bytes_of(identification)?);
    Ok(key)
}

/// A datum: its feature, relationship and two property chains.
fn write_legacy_datum(
    ws: &Handle<XSControlWorkSession>,
    datum_label: &TdfLabel,
    datum_map: &mut LegacyDatumMap,
) -> Result<(), Failure> {
    let model = model_of(ws)?;
    let Some(target) = target_of(ws, datum_label)? else {
        return Ok(());
    };
    let mut attribute: Handle<XCAFDocDatum> = Handle::default();
    if !datum_label.find_attribute_as(&XCAFDocDatum::get_id(), &mut attribute)? {
        return Ok(());
    }
    let live = step_live(&attribute, WHERE)?;
    let name = live.get_name();
    let identification = live.get_identification();
    let description = if live.get_description().is_null() {
        empty()
    } else {
        live.get_description()
    };
    let feature = Handle::new(StepDimTolDatumFeature::new());
    let datum = Handle::new(StepDimTolDatum::new());
    feature.init(&name, &empty(), &target.pds, StepDataLogical::LTrue);
    model.add_with_refs(&feature.cast(), 0, false)?;
    datum.init(
        &name,
        &empty(),
        &target.pds,
        StepDataLogical::LFalse,
        &identification,
    );
    model.add_with_refs(&datum.cast(), 0, false)?;
    let relationship = Handle::new(StepReprShapeAspectRelationship::new());
    relationship.set_name(&name);
    relationship.set_relating_shape_aspect(&feature.cast());
    relationship.set_related_shape_aspect(&datum.cast());
    model.add_with_refs(&relationship.cast(), 0, false)?;
    // write chain for DatumFeature
    write_characterized_chain(
        &model,
        &name,
        &description,
        &feature.cast(),
        &target.face.cast(),
        &target.context,
        false,
    )?;
    // write chain for Datum
    let geometry = step_live(&target.face, WHERE)?.face_geometry();
    write_characterized_chain(
        &model,
        &name,
        &description,
        &datum.cast(),
        &geometry.cast(),
        &target.context,
        true,
    )?;
    // add created Datum into Map
    datum_map.insert(datum_key(&name, &description, &identification)?, datum);
    Ok(())
}

/// A limit of a range: the item of a positive length.
fn limit_item(
    value: f64,
    unit: &StepBasicUnit,
    name: &str,
) -> Result<Handle<StepReprReprItemAndLengthMeasureWithUnit>, Failure> {
    let member = Handle::new(StepBasicMeasureValueMember::new());
    member.set_name("POSITIVE_LENGTH_MEASURE");
    member.set_real(value);
    let measure = Handle::new(StepBasicMeasureWithUnit::new());
    measure.init(&member, unit);
    let item = Handle::new(StepReprRepresentationItem::new());
    item.init(&text(name));
    let result = Handle::new(StepReprReprItemAndLengthMeasureWithUnit::new());
    result.init(&measure, &item)?;
    Ok(result)
}

/// Kind under 20: a dimension.
fn write_legacy_dimension(
    model: &Handle<dyn InterfaceInterfaceModel>,
    aspect: &Handle<StepReprShapeAspect>,
    name: &Handle<HAsciiString>,
    description: &Handle<HAsciiString>,
    values: &Handle<HArray1<f64>>,
    unit: &StepBasicUnit,
    context: &Handle<StepReprRepresentationContext>,
) -> Result<(), Failure> {
    let size = Handle::new(StepShapeDimensionalSize::new());
    size.init(aspect, description);
    model.add_with_refs(&size.cast(), 0, false)?;
    if step_array1_len(values) <= 1 {
        return Ok(());
    }
    // create MeasureWithUnits
    let lower = limit_item(step_array1_item(values, 1, WHERE)?, unit, "lower limit")?;
    let upper = limit_item(step_array1_item(values, 2, WHERE)?, unit, "upper limit")?;
    model.add_with_refs(&lower.cast(), 0, false)?;
    model.add_with_refs(&upper.cast(), 0, false)?;
    let limits = step_new_array1::<Handle<StepReprRepresentationItem>>(1, 2)?;
    step_array1_set(&limits, 1, lower.cast(), WHERE)?;
    step_array1_set(&limits, 2, upper.cast(), WHERE)?;
    let range = Handle::new(StepReprValueRange::new());
    range.init(name, &limits);
    model.add_entity(&range.cast())?;
    let representation = Handle::new(StepShapeShapeDimensionRepresentation::new());
    representation.init(name, &one_item(&range.cast())?, context);
    model.add_with_refs(&representation.cast(), 0, false)?;
    let characteristic = Handle::new(StepShapeDimensionalCharacteristicRepresentation::new());
    let mut dimension = StepShapeDimensionalCharacteristic::default();
    dimension.set_value(&size.cast())?;
    characteristic.init(&dimension, &representation);
    model.add_entity(&characteristic.cast())
}

/// The datum references of a tolerance, of the datums written with the same key.
fn datum_references_of(
    model: &Handle<dyn InterfaceInterfaceModel>,
    tolerance_label: &TdfLabel,
    datum_map: &LegacyDatumMap,
) -> Result<Handle<HArray1<Handle<StepDimTolDatumReference>>>, Failure> {
    let mut labels = Vec::new();
    XCAFDocDimTolTool::get_datum_of_toler_labels(tolerance_label, &mut labels)?;
    let references = step_new_array1::<Handle<StepDimTolDatumReference>>(1, labels.len() as i32)?;
    for (index, label) in labels.iter().enumerate() {
        let position = index as i32 + 1;
        let mut attribute: Handle<XCAFDocDatum> = Handle::default();
        if !label.find_attribute_as(&XCAFDocDatum::get_id(), &mut attribute)? {
            continue;
        }
        let live = step_live(&attribute, WHERE)?;
        let key = datum_key(
            &live.get_name(),
            &live.get_description(),
            &live.get_identification(),
        )?;
        if let Some(datum) = datum_map.get(&key) {
            let reference = Handle::new(StepDimTolDatumReference::new());
            reference.init(position, datum);
            model.add_with_refs(&reference.cast(), 0, false)?;
            step_array1_set(&references, position, reference, WHERE)?;
        }
    }
    Ok(references)
}

/// Declares the writer of the tolerances that only differ by their class.
macro_rules! write_tolerance_of_kind {
    ($kind:expr, $model:expr, $name:expr, $description:expr, $measure:expr, $aspect:expr, $references:expr, $($value:literal => $class:ident),+) => {
        match $kind {
            $($value => {
                let tolerance = Handle::new($class::new());
                tolerance.init($name, $description, &$measure.cast(), $aspect, $references)?;
                $model.add_with_refs(&tolerance.cast(), 0, false)?;
            })+
            _ => {}
        }
    };
}

/// Kind under 35: a tolerance with a datum system.
#[allow(clippy::too_many_arguments)]
fn write_legacy_tolerance(
    model: &Handle<dyn InterfaceInterfaceModel>,
    kind: i32,
    tolerance_label: &TdfLabel,
    name: &Handle<HAsciiString>,
    description: &Handle<HAsciiString>,
    aspect: &Handle<StepReprShapeAspect>,
    values: &Handle<HArray1<f64>>,
    unit: &StepBasicUnit,
    datum_map: &LegacyDatumMap,
) -> Result<(), Failure> {
    let references = datum_references_of(model, tolerance_label, datum_map)?;
    // create LengthMeasureWithUnit
    let member = Handle::new(StepBasicMeasureValueMember::new());
    member.set_name("LENGTH_MEASURE");
    member.set_real(step_array1_item(values, 1, WHERE)?);
    let measure = Handle::new(StepBasicLengthMeasureWithUnit::new());
    measure.init(&member, unit);
    // create tolerance by it's type
    if kind < 24 {
        let with_datums = Handle::new(StepDimTolGeometricToleranceWithDatumReference::new());
        with_datums.set_datum_system(&references)?;
        let modified = Handle::new(StepDimTolModifiedGeometricTolerance::new());
        match kind {
            21 => modified.set_modifier(StepDimTolLimitCondition::MaximumMaterialCondition),
            22 => modified.set_modifier(StepDimTolLimitCondition::LeastMaterialCondition),
            23 => modified.set_modifier(StepDimTolLimitCondition::RegardlessOfFeatureSize),
            _ => {}
        }
        let complex = Handle::new(StepDimTolGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol::new());
        complex.init(
            name,
            description,
            &measure.cast(),
            aspect,
            &with_datums,
            &modified,
        )?;
        model.add_with_refs(&complex.cast(), 0, false)?;
        return Ok(());
    }
    write_tolerance_of_kind!(kind, model, name, description, measure, aspect, &references,
        24 => StepDimTolAngularityTolerance,
        25 => StepDimTolCircularRunoutTolerance,
        26 => StepDimTolCoaxialityTolerance,
        27 => StepDimTolConcentricityTolerance,
        28 => StepDimTolParallelismTolerance,
        29 => StepDimTolPerpendicularityTolerance,
        30 => StepDimTolSymmetryTolerance,
        31 => StepDimTolTotalRunoutTolerance
    );
    Ok(())
}

/// A dimension or tolerance of the first version.
fn write_legacy_dim_tol(
    ws: &Handle<XSControlWorkSession>,
    label: &TdfLabel,
    datum_map: &LegacyDatumMap,
) -> Result<(), Failure> {
    let model = model_of(ws)?;
    let Some(target) = target_of(ws, label)? else {
        return Ok(());
    };
    let mut attribute: Handle<XCAFDocDimTol> = Handle::default();
    if !label.find_attribute_as(&XCAFDocDimTol::get_id(), &mut attribute)? {
        return Ok(());
    }
    let live = step_live(&attribute, WHERE)?;
    let kind = live.get_kind();
    let values = live.get_val();
    let name = live.get_name();
    let description = live.get_description();
    // common part of writing D&GT entities
    let aspect = Handle::new(StepReprShapeAspect::new());
    aspect.init(&name, &empty(), &target.pds, StepDataLogical::LTrue);
    model.add_with_refs(&aspect.cast(), 0, false)?;
    let item: Handle<StepReprRepresentationItem> = if kind < 20 {
        target.edge.cast()
    } else {
        target.face.cast()
    };
    write_characterized_chain(
        &model,
        &name,
        &description,
        &aspect,
        &item,
        &target.context,
        false,
    )?;
    // define aUnit for creation LengthMeasureWithUnit (common for all)
    let unit = get_unit(&target.context, false)?;
    // specific part of writing D&GT entities
    if kind < 20 {
        write_legacy_dimension(
            &model,
            &aspect,
            &name,
            &description,
            &values,
            &unit,
            &target.context,
        )
    } else if kind < 35 {
        write_legacy_tolerance(
            &model,
            kind,
            label,
            &name,
            &description,
            &aspect,
            &values,
            &unit,
            datum_map,
        )
    } else {
        Ok(())
    }
}

impl WriterState {
    /// `writeDGTs`.
    pub(super) fn write_dgts(
        &self,
        ws: &Handle<XSControlWorkSession>,
        labels: &[TdfLabel],
    ) -> Result<bool, Failure> {
        if labels.is_empty() || graph_of(ws)?.is_null() {
            return Ok(false);
        }
        let mut datum_map = LegacyDatumMap::new();
        // Iterate on requested shapes collect Tools
        let mut datums = Vec::new();
        for root in &self.root_labels {
            let tool = XCAFDocDocumentTool::dim_tol_tool(root)?;
            datums.extend(step_live(&tool, WHERE)?.get_datum_labels()?);
        }
        if datums.is_empty() {
            return Ok(false);
        }
        for datum in &datums {
            write_legacy_datum(ws, datum, &mut datum_map)?;
        }
        // write Tolerances and Dimensions
        let mut dim_tols = Vec::new();
        for root in &self.root_labels {
            let tool = XCAFDocDocumentTool::dim_tol_tool(root)?;
            dim_tols.extend(step_live(&tool, WHERE)?.get_dim_tol_labels()?);
        }
        if dim_tols.is_empty() {
            return Ok(false);
        }
        for dim_tol in &dim_tols {
            write_legacy_dim_tol(ws, dim_tol, &datum_map)?;
        }
        Ok(true)
    }
}
