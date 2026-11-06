//! `STEPCAFControlWriter`: the values, qualifiers, modifiers, orientation and tolerance class of a
//! dimension, and its connection points.
use super::STEPCAFControlGDTProperty;
use super::writer::WHERE;
use super::writer_gdt_common::{
    create_dim_value, create_qualified_dim_value, empty, get_unit, measure_with_unit, one_item,
    text,
};
use super::writer_support::model_of;
use crate::geom_to_step::GeomToStepMakeCartesianPoint;
use crate::step_ap242::{
    StepAP242GeometricItemSpecificUsage, StepAP242ItemIdentifiedRepresentationUsageDefinition,
};
use crate::step_basic::StepBasicUnit;
use crate::step_data::{StepDataFactors, StepDataLogical, StepDataSelectType};
use crate::step_entity::{HArray1, step_array1_item, step_array1_set, step_live, step_new_array1};
use crate::step_geom::{StepGeomAxis2Placement3d, StepGeomCartesianPoint, StepGeomDirection};
use crate::step_repr::{
    StepReprCompoundRepresentationItem, StepReprConstructiveGeometryRepresentation,
    StepReprDerivedShapeAspect, StepReprDescriptiveRepresentationItem,
    StepReprRepresentationContext, StepReprRepresentationItem, StepReprShapeAspect,
    StepReprShapeAspectDerivingRelationship,
};
use crate::step_shape::{
    StepShapeAngularLocation, StepShapeAngularSize, StepShapeDimensionalCharacteristic,
    StepShapeDimensionalCharacteristicRepresentation, StepShapePlusMinusTolerance,
    StepShapeQualifiedRepresentationItem, StepShapeShapeDimensionRepresentation,
    StepShapeToleranceMethodDefinition, StepShapeToleranceValue, StepShapeTypeQualifier,
    StepShapeValueFormatTypeQualifier, StepShapeValueQualifier,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Direction3, Point3};
use martensite_tkxcaf::xcaf_dim_tol_objects::{
    XCAFDimTolObjectsDimensionFormVariance, XCAFDimTolObjectsDimensionGrade,
    XCAFDimTolObjectsDimensionObject, XCAFDimTolObjectsDimensionType,
};
use martensite_tkxsbase::interface::InterfaceModelEntities;
use martensite_tkxsbase::xs_control::XSControlWorkSession;

/// A representation item array under construction.
struct ValueItems {
    items: Handle<HArray1<Handle<StepReprRepresentationItem>>>,
    next: i32,
}

impl ValueItems {
    fn push(&mut self, item: Handle<StepReprRepresentationItem>) -> Result<(), Failure> {
        step_array1_set(&self.items, self.next, item, WHERE)?;
        self.next += 1;
        Ok(())
    }
}

/// The qualifiers of the nominal value: the type, then the number of decimal places; null without any.
fn nominal_qualifiers(
    ws: &Handle<XSControlWorkSession>,
    object: &XCAFDimTolObjectsDimensionObject,
    is_angle: bool,
) -> Result<Handle<StepShapeQualifiedRepresentationItem>, Failure> {
    let model = model_of(ws)?;
    let (left, right) = object.get_nb_of_decimal_places();
    let has_type = object.has_qualifier() && !is_angle;
    let has_places = left > 0 || right > 0;
    let count = i32::from(has_type) + i32::from(has_places);
    if count == 0 {
        return Ok(Handle::default());
    }
    let qualified = Handle::new(StepShapeQualifiedRepresentationItem::new());
    let qualifiers = step_new_array1::<StepShapeValueQualifier>(1, count)?;
    // Type qualifier
    if has_type {
        let kind = Handle::new(StepShapeTypeQualifier::new());
        kind.init(&STEPCAFControlGDTProperty::get_dim_qualifier_name(
            object.get_qualifier(),
        ));
        model.add_with_refs(&kind.cast(), 0, false)?;
        let mut item = StepShapeValueQualifier::default();
        item.set_value(&kind.cast())?;
        step_array1_set(&qualifiers, 1, item, WHERE)?;
    }
    // Number of decimal places
    if has_places {
        let format = Handle::new(StepShapeValueFormatTypeQualifier::new());
        format.init(&text(&format!("NR2 {left}.{right}")));
        model.add_with_refs(&format.cast(), 0, false)?;
        let mut item = StepShapeValueQualifier::default();
        item.set_value(&format.cast())?;
        step_array1_set(&qualifiers, count, item, WHERE)?;
    }
    // Set qualifiers
    step_live(&qualified, WHERE)?.set_qualifiers(&qualifiers);
    Ok(qualified)
}

/// The compound item of the modifiers of a dimension.
fn modifiers_item(
    ws: &Handle<XSControlWorkSession>,
    object: &XCAFDimTolObjectsDimensionObject,
) -> Result<Handle<StepReprRepresentationItem>, Failure> {
    let modifiers = object.get_modifiers();
    let compound = Handle::new(StepReprCompoundRepresentationItem::new());
    let items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, modifiers.len() as i32)?;
    for (index, modifier) in modifiers.iter().enumerate() {
        let item = Handle::new(StepReprDescriptiveRepresentationItem::new());
        item.init(
            &empty(),
            &STEPCAFControlGDTProperty::get_dim_modifier_name(*modifier),
        );
        model_of(ws)?.add_with_refs(&item.cast(), 0, false)?;
        step_array1_set(&items, index as i32 + 1, item.cast(), WHERE)?;
    }
    compound.init(&empty(), &items);
    Ok(compound.cast())
}

/// The orientation of an oriented location.
fn orientation_item(
    object: &XCAFDimTolObjectsDimensionObject,
    factors: &StepDataFactors,
) -> Result<Handle<StepReprRepresentationItem>, Failure> {
    let orientation = Handle::new(StepGeomAxis2Placement3d::new());
    let mut direction = Direction3::default();
    object.get_direction(&mut direction);
    let origin =
        GeomToStepMakeCartesianPoint::from_pnt(&Point3::new(0., 0., 0.), factors.length_factor())
            .value()?;
    let axis = Handle::new(StepGeomDirection::new());
    let coordinates = step_new_array1::<f64>(1, 3)?;
    step_array1_set(&coordinates, 1, direction.x(), WHERE)?;
    step_array1_set(&coordinates, 2, direction.y(), WHERE)?;
    step_array1_set(&coordinates, 3, direction.z(), WHERE)?;
    axis.init(&empty(), &coordinates)?;
    orientation.init(
        &text("orientation"),
        &origin,
        true,
        &axis,
        false,
        &Handle::default(),
    );
    Ok(orientation.cast())
}

/// The plus-minus tolerance, the limits and fits of a dimension.
fn write_tolerances(
    ws: &Handle<XSControlWorkSession>,
    object: &XCAFDimTolObjectsDimensionObject,
    unit: &StepBasicUnit,
    measure_name: &str,
    dimension: &StepShapeDimensionalCharacteristic,
) -> Result<(), Failure> {
    let model = model_of(ws)?;
    if object.is_dim_with_plus_minus_tolerance() {
        // delete "POSITIVE_": the sub-string starts at its last character, so the name is
        // not one of a measure and the value is written without a type. Kept, the files are the same.
        let name = &measure_name["POSITIVE".len()..];
        let upper = measure_with_unit(object.get_upper_tol_value(), unit, name)?;
        model.add_with_refs(&upper.cast(), 0, false)?;
        let lower = measure_with_unit(-object.get_lower_tol_value(), unit, name)?;
        model.add_with_refs(&lower.cast(), 0, false)?;
        // Tolerance
        let value = Handle::new(StepShapeToleranceValue::new());
        value.init(&lower.cast(), &upper.cast());
        model.add_with_refs(&value.cast(), 0, false)?;
        let mut method = StepShapeToleranceMethodDefinition::default();
        method.set_value(&value.cast())?;
        let tolerance = Handle::new(StepShapePlusMinusTolerance::new());
        tolerance.init(&method, dimension);
        model.add_with_refs(&tolerance.cast(), 0, false)?;
    }
    // Tolerance class
    if object.is_dim_with_class_of_tolerance() {
        let (mut is_hole, mut form, mut grade) = (
            false,
            XCAFDimTolObjectsDimensionFormVariance::None,
            XCAFDimTolObjectsDimensionGrade::IT01,
        );
        if !object.get_class_of_tolerance(&mut is_hole, &mut form, &mut grade) {
            return Ok(());
        }
        let limits = STEPCAFControlGDTProperty::get_limits_and_fits(is_hole, form, grade);
        model.add_with_refs(&limits.cast(), 0, false)?;
        let mut method = StepShapeToleranceMethodDefinition::default();
        method.set_value(&limits.cast())?;
        let tolerance = Handle::new(StepShapePlusMinusTolerance::new());
        tolerance.init(&method, dimension);
        model.add_with_refs(&tolerance.cast(), 0, false)?;
    }
    Ok(())
}

/// `write_dim_values`: all data of a dimension.
pub(super) fn write_dim_values(
    ws: &Handle<XSControlWorkSession>,
    object: &XCAFDimTolObjectsDimensionObject,
    context: &Handle<StepReprRepresentationContext>,
    dimension: &StepShapeDimensionalCharacteristic,
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    // Get working data
    let model = model_of(ws)?;
    let modifiers = object.get_modifiers();
    let value = dimension.value();
    let is_angle = value.cast::<StepShapeAngularLocation>().is_some()
        || value.cast::<StepShapeAngularSize>().is_some();
    // Unit
    let unit = get_unit(context, is_angle)?;
    let measure_name = if is_angle {
        "POSITIVE_PLANE_ANGLE_MEASURE"
    } else {
        "POSITIVE_LENGTH_MEASURE"
    };
    // Values
    let is_oriented = object.get_type() == XCAFDimTolObjectsDimensionType::LocationOriented;
    let count = 1
        + if object.is_dim_with_range() { 2 } else { 0 }
        + i32::from(!modifiers.is_empty())
        + i32::from(is_oriented)
        + object.nb_descriptions();
    let mut values = ValueItems {
        items: step_new_array1::<Handle<StepReprRepresentationItem>>(1, count)?,
        next: 1,
    };
    // Nominal value
    let qualified = nominal_qualifiers(ws, object, is_angle)?;
    let nominal = text("nominal value");
    let item = if qualified.is_some() {
        create_qualified_dim_value(
            object.get_value(),
            &unit,
            &nominal,
            measure_name,
            is_angle,
            &qualified,
        )?
    } else {
        create_dim_value(object.get_value(), &unit, &nominal, measure_name, is_angle)?
    };
    values.push(item.cast())?;
    // Ranges
    if object.is_dim_with_range() {
        let lower = create_dim_value(
            object.get_lower_bound(),
            &unit,
            &text("lower limit"),
            measure_name,
            is_angle,
        )?;
        let upper = create_dim_value(
            object.get_upper_bound(),
            &unit,
            &text("upper limit"),
            measure_name,
            is_angle,
        )?;
        values.push(lower.cast())?;
        values.push(upper.cast())?;
    }
    // Modifiers
    if !modifiers.is_empty() {
        values.push(modifiers_item(ws, object)?)?;
    }
    // Orientation
    if is_oriented {
        values.push(orientation_item(object, factors)?)?;
    }
    // Descriptions
    for number in 0..object.nb_descriptions() {
        let description = Handle::new(StepReprDescriptiveRepresentationItem::new());
        description.init(
            &object.get_description_name(number),
            &object.get_description(number),
        );
        values.push(description.cast())?;
    }
    for index in 1..=count {
        let item = step_array1_item(&values.items, index, WHERE)?;
        model.add_with_refs(&item.cast(), 0, false)?;
    }
    // Create resulting Shape_Dimension_Representation
    let representation = Handle::new(StepShapeShapeDimensionRepresentation::new());
    representation.init(&empty(), &values.items, context);
    model.add_with_refs(&representation.cast(), 0, false)?;
    let characteristic = Handle::new(StepShapeDimensionalCharacteristicRepresentation::new());
    characteristic.init(dimension, &representation);
    model.add_with_refs(&characteristic.cast(), 0, false)?;
    write_tolerances(ws, object, &unit, measure_name, dimension)
}

/// A point as a derived shape aspect of `aspect` (always of the shape of `first`).
fn derived_point(
    ws: &Handle<XSControlWorkSession>,
    point: &Point3,
    first: &Handle<StepReprShapeAspect>,
    related: &Handle<StepReprShapeAspect>,
    representation: &Handle<StepReprConstructiveGeometryRepresentation>,
    points: &mut Vec<Handle<StepGeomCartesianPoint>>,
    factors: &StepDataFactors,
) -> Result<Handle<StepReprShapeAspect>, Failure> {
    let model = model_of(ws)?;
    let created = GeomToStepMakeCartesianPoint::from_pnt(point, factors.length_factor()).value()?;
    points.push(created.clone());
    let derived = Handle::new(StepReprDerivedShapeAspect::new());
    derived.init(
        &empty(),
        &empty(),
        &step_live(first, WHERE)?.of_shape(),
        StepDataLogical::LFalse,
    );
    let usage = Handle::new(StepAP242GeometricItemSpecificUsage::new());
    let mut definition = StepAP242ItemIdentifiedRepresentationUsageDefinition::default();
    definition.set_value(&derived.cast())?;
    let item = one_item(&created.cast())?;
    usage.init(
        &empty(),
        &empty(),
        &definition,
        &representation.cast(),
        &item,
    );
    let relationship = Handle::new(StepReprShapeAspectDerivingRelationship::new());
    relationship.init(&empty(), false, &empty(), &derived.cast(), related);
    model.add_with_refs(&usage.cast(), 0, false)?;
    model.add_with_refs(&relationship.cast(), 0, false)?;
    Ok(derived.cast())
}

/// `write_derived_geometry`: the connection points of a dimension.
pub(super) fn write_derived_geometry(
    ws: &Handle<XSControlWorkSession>,
    object: &XCAFDimTolObjectsDimensionObject,
    representation: &Handle<StepReprConstructiveGeometryRepresentation>,
    first: &mut Handle<StepReprShapeAspect>,
    second: &mut Handle<StepReprShapeAspect>,
    points: &mut Vec<Handle<StepGeomCartesianPoint>>,
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    // First point
    if object.has_point() {
        *first = derived_point(
            ws,
            &object.get_point(),
            first,
            first,
            representation,
            points,
            factors,
        )?;
    }
    // Second point (for locations)
    if object.has_point2() {
        *second = derived_point(
            ws,
            &object.get_point2(),
            first,
            second,
            representation,
            points,
            factors,
        )?;
    }
    Ok(())
}
