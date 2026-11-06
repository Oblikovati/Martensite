//! `setDimObjectToXCAF` of `STEPCAFControlReader`: the object of a dimension.
use super::STEPCAFControlGDTProperty;
use super::reader::WHERE;
use super::reader_gdt_common::find_shape_index_for_dgt;
use super::reader_pmi::{read_annotation, read_connection_points};
use super::reader_support::{
    SessionGraph, context_of_unit, convert_angle_value, first_of, measure_with_unit, named_unit_of,
};
use crate::geom_to_step::shared_array1;
use crate::step_ap242::StepAP242GeometricItemSpecificUsage;
use crate::step_basic::{StepBasicMeasureWithUnit, StepBasicPlaneAngleMeasureWithUnit};
use crate::step_data::{StepDataFactors, StepDataSelectType};
use crate::step_entity::{step_array1_item, step_live};
use crate::step_geom::{StepGeomAxis2Placement3d, StepGeomGeometricRepresentationItem};
use crate::step_repr::{
    StepReprCompoundRepresentationItem, StepReprDescriptiveRepresentationItem,
    StepReprReprItemAndLengthMeasureWithUnit, StepReprReprItemAndLengthMeasureWithUnitAndQRI,
    StepReprReprItemAndMeasureWithUnit, StepReprReprItemAndMeasureWithUnitAndQRI,
    StepReprReprItemAndPlaneAngleMeasureWithUnit,
    StepReprReprItemAndPlaneAngleMeasureWithUnitAndQRI, StepReprRepresentationItem,
    StepReprShapeAspect,
};
use crate::step_shape::{
    StepShapeAngleRelator, StepShapeAngularLocation, StepShapeAngularSize,
    StepShapeDimensionalCharacteristicRepresentation, StepShapeDimensionalLocationWithPath,
    StepShapeDimensionalSizeWithPath, StepShapeLimitsAndFits, StepShapePlusMinusTolerance,
    StepShapeQualifiedRepresentationItem, StepShapeToleranceValue, StepShapeTypeQualifier,
    StepShapeValueFormatTypeQualifier,
};
use martensite_tkbrep::topods;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkmath::gp::Direction3;
use martensite_tkxcaf::xcaf_dim_tol_objects::{
    XCAFDimTolObjectsAngularQualifier, XCAFDimTolObjectsDimensionFormVariance,
    XCAFDimTolObjectsDimensionGrade, XCAFDimTolObjectsDimensionObject,
    XCAFDimTolObjectsDimensionQualifier, XCAFDimTolObjectsDimensionType,
};
use martensite_tkxcaf::xcaf_doc::XCAFDocDimension;

/// What the sharings of a dimension give.
struct DimensionValues {
    first: f64,
    second: f64,
    third: f64,
    is_plus_minus: bool,
    type_qualifier: Handle<StepShapeTypeQualifier>,
    format_qualifier: Handle<StepShapeValueFormatTypeQualifier>,
    limits_and_fits: Handle<StepShapeLimitsAndFits>,
    compound: Handle<StepReprCompoundRepresentationItem>,
    placement: Handle<StepGeomAxis2Placement3d>,
}

impl Default for DimensionValues {
    fn default() -> Self {
        Self {
            first: -1.,
            second: -1.,
            third: -1.,
            is_plus_minus: false,
            type_qualifier: Handle::default(),
            format_qualifier: Handle::default(),
            limits_and_fits: Handle::default(),
            compound: Handle::default(),
            placement: Handle::default(),
        }
    }
}

/// The value of a measure of a dimension item: a length is scaled and an angle converted. `None` is
/// the `continue` for a unit that is not named.
fn item_value(
    measure: &StepBasicMeasureWithUnit,
    is_length: bool,
    is_angle: bool,
    factors: &StepDataFactors,
) -> Result<Option<f64>, Failure> {
    let mut value = measure.value_component();
    let Some(named) = named_unit_of(&measure.unit_component()) else {
        return Ok(None);
    };
    let context = context_of_unit(&named, factors)?;
    if is_length {
        value *= context.length_factor();
    } else if is_angle {
        convert_angle_value(&context, &mut value);
    }
    Ok(Some(value))
}

impl DimensionValues {
    /// A simple value or range: upper limit, or lower limit or simple nominal value.
    fn read_measure_item(
        &mut self,
        item: &Handle<StepReprRepresentationItem>,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let with_unit = item.cast::<StepReprReprItemAndMeasureWithUnit>();
        let live = step_live(&with_unit, WHERE)?;
        let measure = live.get_measure_with_unit();
        let is_length = item
            .cast::<StepReprReprItemAndLengthMeasureWithUnit>()
            .is_some()
            || item
                .cast::<StepReprReprItemAndLengthMeasureWithUnitAndQRI>()
                .is_some();
        let is_angle = item
            .cast::<StepReprReprItemAndPlaneAngleMeasureWithUnitAndQRI>()
            .is_some()
            || item
                .cast::<StepReprReprItemAndPlaneAngleMeasureWithUnit>()
                .is_some();
        let Some(value) = item_value(step_live(&measure, WHERE)?, is_length, is_angle, factors)?
        else {
            return Ok(());
        };
        let name = live.name();
        if step_live(&name, WHERE)?.search("upper") > 0 {
            self.second = value;
        } else {
            self.first = value;
        }
        Ok(())
    }
    /// A value with a qualifier: minimum, maximum or average.
    fn read_qualified_measure(
        &mut self,
        item: &Handle<StepReprRepresentationItem>,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let with_qualifier = item.cast::<StepReprReprItemAndMeasureWithUnitAndQRI>();
        let live = step_live(&with_qualifier, WHERE)?;
        let measure = live.get_measure_with_unit();
        let is_length = item
            .cast::<StepReprReprItemAndLengthMeasureWithUnitAndQRI>()
            .is_some();
        let is_angle = item
            .cast::<StepReprReprItemAndPlaneAngleMeasureWithUnitAndQRI>()
            .is_some();
        let Some(value) = item_value(step_live(&measure, WHERE)?, is_length, is_angle, factors)?
        else {
            return Ok(());
        };
        let qualified = live.get_qualified_representation_item();
        let qualifiers = step_live(&qualified, WHERE)?.qualifiers();
        if step_live(&qualifiers, WHERE)?.array1().length() == 0 {
            self.first = value;
            return Ok(());
        }
        let kind = step_array1_item(&qualifiers, 1, WHERE)?.type_qualifier();
        let name = step_live(&kind, WHERE)?.name();
        let name = step_live(&name, WHERE)?.string();
        match name.as_bytes() {
            b"minimum" => self.second = value,
            b"maximum" => self.third = value,
            _ => self.first = value,
        }
        Ok(())
    }
    /// The items of the representation of a dimension.
    fn read_representation(
        &mut self,
        characteristic: &StepShapeDimensionalCharacteristicRepresentation,
        object: &XCAFDimTolObjectsDimensionObject,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let representation = characteristic.representation();
        let Some(representation) = representation.get() else {
            return Ok(());
        };
        for rank in 1..=representation.nb_items() {
            let item = representation.items_value(rank)?;
            if item.is_null() {
                continue;
            }
            if item.cast::<StepReprReprItemAndMeasureWithUnit>().is_some() {
                self.read_measure_item(&item, factors)?;
            } else if item
                .cast::<StepReprReprItemAndMeasureWithUnitAndQRI>()
                .is_some()
            {
                self.read_qualified_measure(&item, factors)?;
            } else if let Some(qualified) =
                item.cast::<StepShapeQualifiedRepresentationItem>().get()
            {
                // get qualifier
                let qualifiers = qualified.qualifiers();
                for index in 1..=qualified.nb_qualifiers()? {
                    let select = step_array1_item(&qualifiers, index, WHERE)?;
                    self.type_qualifier = select.type_qualifier();
                    self.format_qualifier = select.value_format_type_qualifier();
                }
            } else if let Some(description) =
                item.cast::<StepReprDescriptiveRepresentationItem>().get()
            {
                object.add_description(&description.description(), &description.name());
            } else if item.cast::<StepReprCompoundRepresentationItem>().is_some() {
                self.compound = item.cast();
            } else if item.cast::<StepGeomAxis2Placement3d>().is_some() {
                self.placement = item.cast();
            }
        }
        Ok(())
    }
    /// One bound of a plus minus tolerance; `None` is the `continue`.
    fn bound_value(
        bound: &Handle<StandardTransient>,
        factors: &StepDataFactors,
    ) -> Result<Option<f64>, Failure> {
        if bound.is_null() {
            return Ok(None);
        }
        let measure = measure_with_unit(bound);
        let Some(live) = measure.get() else {
            return Ok(None);
        };
        let mut value = live.value_component();
        let Some(named) = named_unit_of(&live.unit_component()) else {
            return Ok(None);
        };
        let context = context_of_unit(&named, factors)?;
        if measure
            .cast::<StepBasicPlaneAngleMeasureWithUnit>()
            .is_some()
        {
            convert_angle_value(&context, &mut value);
        } else if context.length_factor() > 0. {
            value *= context.length_factor();
        }
        Ok(Some(value))
    }
    /// A tolerance of the dimension: plus minus values, or a class of tolerance.
    fn read_tolerance(
        &mut self,
        tolerance: &StepShapePlusMinusTolerance,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        self.is_plus_minus = true;
        let range = tolerance.range();
        if range.case_number() != 1 {
            // class of tolerance
            self.limits_and_fits = range.limits_and_fits();
            return Ok(());
        }
        // plus minus tolerance
        let value: Handle<StepShapeToleranceValue> = range.tolerance_value();
        let Some(value) = value.get() else {
            return Ok(());
        };
        let Some(upper) = Self::bound_value(&value.upper_bound(), factors)? else {
            return Ok(());
        };
        self.third = upper;
        if let Some(lower) = Self::bound_value(&value.lower_bound(), factors)? {
            self.second = lower.abs();
        }
        Ok(())
    }
    /// The values on the object; false without a nominal value.
    fn set_values(&mut self, object: &XCAFDimTolObjectsDimensionObject) -> bool {
        if self.first < 0. {
            return false;
        }
        if self.second < 0. {
            object.set_value(self.first);
        } else if self.third < 0. {
            object.set_values(&shared_array1(vec![self.first, self.second]));
        } else {
            if !self.is_plus_minus {
                self.second = self.first - self.second;
                self.third -= self.first;
            }
            object.set_values(&shared_array1(vec![self.first, self.second, self.third]));
        }
        true
    }
    /// Qualifier, format, class of tolerance and modifiers on the object.
    fn set_qualifiers(&self, object: &XCAFDimTolObjectsDimensionObject) -> Result<(), Failure> {
        if let Some(qualifier) = self.type_qualifier.get() {
            let mut kind = XCAFDimTolObjectsDimensionQualifier::None;
            if STEPCAFControlGDTProperty::get_dim_qualifier_type(&qualifier.name(), &mut kind)? {
                object.set_qualifier(kind);
            }
        }
        if let Some(format) = self.format_qualifier.get() {
            // A typical value would be 'NR2 2.2'
            let text = format.format_type();
            let text = step_live(&text, WHERE)?;
            let blank = text.location_char(1, b' ', 1, text.length())?;
            let left = text.sub_string(blank + 1, blank + 1)?.integer_value()?;
            let right = text.sub_string(blank + 3, blank + 3)?.integer_value()?;
            object.set_nb_of_decimal_places(left, right);
        }
        if self.limits_and_fits.is_some() {
            // get class of tolerance
            let mut hole = false;
            let mut form_variance = XCAFDimTolObjectsDimensionFormVariance::None;
            let mut grade = XCAFDimTolObjectsDimensionGrade::IT01;
            STEPCAFControlGDTProperty::get_dim_class_of_tolerance(
                &self.limits_and_fits,
                &mut hole,
                &mut form_variance,
                &mut grade,
            )?;
            object.set_class_of_tolerance(hole, form_variance, grade);
        }
        if let Some(compound) = self.compound.get()
            && compound
                .item_element()
                .get()
                .is_some_and(|items| items.array1().length() > 0)
        {
            // get modifiers
            let mut modifiers = Vec::new();
            STEPCAFControlGDTProperty::get_dim_modifiers(&self.compound, &mut modifiers)?;
            if !modifiers.is_empty() {
                object.set_modifiers(&modifiers);
            }
        }
        Ok(())
    }
}

/// The qualifier of an angular dimension.
fn angular_qualifier_of(relator: StepShapeAngleRelator, object: &XCAFDimTolObjectsDimensionObject) {
    object.set_angular_qualifier(match relator {
        StepShapeAngleRelator::Equal => XCAFDimTolObjectsAngularQualifier::Equal,
        StepShapeAngleRelator::Large => XCAFDimTolObjectsAngularQualifier::Large,
        StepShapeAngleRelator::Small => XCAFDimTolObjectsAngularQualifier::Small,
    });
}

/// The direction of an oriented dimensional location, from a placement named "orientation".
fn set_orientation(
    placement: &Handle<StepGeomAxis2Placement3d>,
    object: &XCAFDimTolObjectsDimensionObject,
) -> Result<(), Failure> {
    let Some(live) = placement.get() else {
        return Ok(());
    };
    let (reference, name) = (live.ref_direction(), live.name());
    let (Some(reference), Some(name)) = (reference.get(), name.get()) else {
        return Ok(());
    };
    if live.axis().is_null() || name.string().as_bytes() != b"orientation" {
        return Ok(());
    }
    let count = reference.nb_direction_ratios();
    if count >= 3 {
        object.set_direction(&Direction3::new(
            reference.direction_ratios_value(1)?,
            reference.direction_ratios_value(2)?,
            reference.direction_ratios_value(3)?,
        )?);
    } else if count == 2 {
        object.set_direction(&Direction3::new(
            reference.direction_ratios_value(1)?,
            reference.direction_ratios_value(2)?,
            0.,
        )?);
    }
    Ok(())
}

/// The path of a dimension from its aspect; false when the returns before setting the object.
fn set_path(
    session: &SessionGraph,
    aspect: &Handle<StepReprShapeAspect>,
    object: &XCAFDimTolObjectsDimensionObject,
) -> Result<bool, Failure> {
    // for DimensionalLocationWithPath
    let usage: Handle<StepAP242GeometricItemSpecificUsage> =
        first_of(&session.sharings(&aspect.cast())?);
    let Some(usage) = usage.get() else {
        return Ok(false);
    };
    let item: Handle<StepGeomGeometricRepresentationItem> = if usage.nb_identified_item() > 0 {
        usage.identified_item_value(1)?.cast()
    } else {
        Handle::default()
    };
    if item.is_null() {
        return Ok(false);
    }
    let holder: Handle<StepReprRepresentationItem> = first_of(&session.sharings(&item.cast())?);
    let shape = session.shape_at(find_shape_index_for_dgt(&holder.cast(), session)?)?;
    let edge = if shape.is_null() {
        Default::default()
    } else {
        topods::edge(&shape)?
    };
    if edge.is_null() {
        return Ok(false);
    }
    object.set_path(&edge);
    Ok(true)
}

/// The type of a dimension whose name says nothing: angular, with a path or oriented. False when
/// the returns before setting the object.
fn type_from_entity(
    session: &SessionGraph,
    entity: &Handle<StandardTransient>,
    placement: &Handle<StepGeomAxis2Placement3d>,
    kind: &mut XCAFDimTolObjectsDimensionType,
    object: &XCAFDimTolObjectsDimensionObject,
) -> Result<bool, Failure> {
    let is_size = entity
        .cast::<crate::step_shape::StepShapeDimensionalSize>()
        .is_some();
    if let Some(angular) = entity.cast::<StepShapeAngularSize>().get() {
        // get qualifier for angular value
        *kind = XCAFDimTolObjectsDimensionType::SizeAngular;
        angular_qualifier_of(angular.angle_selection(), object);
    } else if let Some(angular) = entity.cast::<StepShapeAngularLocation>().get() {
        *kind = XCAFDimTolObjectsDimensionType::LocationAngular;
        angular_qualifier_of(angular.angle_selection(), object);
    }
    if *kind != XCAFDimTolObjectsDimensionType::LocationNone {
        return Ok(true);
    }
    let mut path = Handle::<StepReprShapeAspect>::default();
    if is_size {
        if let Some(with_path) = entity.cast::<StepShapeDimensionalSizeWithPath>().get() {
            *kind = XCAFDimTolObjectsDimensionType::SizeWithPath;
            path = with_path.path();
        }
    } else if let Some(with_path) = entity.cast::<StepShapeDimensionalLocationWithPath>().get() {
        *kind = XCAFDimTolObjectsDimensionType::LocationWithPath;
        path = with_path.path();
    }
    if path.is_some() {
        return set_path(session, &path, object);
    }
    set_orientation(placement, object)?;
    Ok(true)
}

/// `setDimObjectToXCAF`: the object of a dimension on its label; none without a nominal value.
pub(super) fn set_dim_object_to_xcaf(
    entity: &Handle<StandardTransient>,
    label: &TdfLabel,
    session: &SessionGraph,
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    let size = entity.cast::<crate::step_shape::StepShapeDimensionalSize>();
    let location = entity.cast::<crate::step_shape::StepShapeDimensionalLocation>();
    if size.is_null() && location.is_null() {
        return Ok(());
    }
    let object = Handle::new(XCAFDimTolObjectsDimensionObject::new());
    let live_object = step_live(&object, WHERE)?;
    let mut values = DimensionValues::default();
    for sharing in session.sharings(entity)? {
        if let Some(characteristic) = sharing
            .cast::<StepShapeDimensionalCharacteristicRepresentation>()
            .get()
        {
            values.read_representation(characteristic, live_object, factors)?;
        } else if let Some(tolerance) = sharing.cast::<StepShapePlusMinusTolerance>().get() {
            values.read_tolerance(tolerance, factors)?;
        }
    }
    if !values.set_values(live_object) {
        return Ok(());
    }
    values.set_qualifiers(live_object)?;
    let name: Handle<HAsciiString> = size.get().map_or_else(
        || location.get().map(|live| live.name()).unwrap_or_default(),
        |live| live.name(),
    );
    let mut kind = XCAFDimTolObjectsDimensionType::LocationNone;
    if !STEPCAFControlGDTProperty::get_dim_type(&name, &mut kind)?
        && !type_from_entity(session, entity, &values.placement, &mut kind, live_object)?
    {
        return Ok(());
    }
    live_object.set_type(kind);
    let mut attribute = Handle::<XCAFDocDimension>::default();
    if label.find_attribute_as(&XCAFDocDimension::get_id(), &mut attribute)? {
        read_annotation(session, entity, &object.cast(), factors)?;
        read_connection_points(session, entity, &object, factors)?;
        step_live(&attribute, WHERE)?.set_object(&object)?;
    }
    Ok(())
}
