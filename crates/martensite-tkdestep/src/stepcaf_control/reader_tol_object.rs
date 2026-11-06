//! `setGeomTolObjectToXCAF` of `STEPCAFControlReader`: the object of a geometric tolerance.
use super::STEPCAFControlGDTProperty;
use super::reader::WHERE;
use super::reader_pmi::read_annotation;
use super::reader_support::{
    SessionGraph, context_of_unit, convert_angle_value, measure_with_unit, named_unit_of,
};
use crate::step_basic::{StepBasicMeasureWithUnit, StepBasicUnit};
use crate::step_data::StepDataFactors;
use crate::step_dim_tol::*;
use crate::step_entity::{step_array1_item, step_live};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkxcaf::NativeEnum;
use martensite_tkxcaf::xcaf_dim_tol_objects::{
    XCAFDimTolObjectsGeomToleranceMatReqModif, XCAFDimTolObjectsGeomToleranceModif,
    XCAFDimTolObjectsGeomToleranceObject, XCAFDimTolObjectsGeomToleranceType,
    XCAFDimTolObjectsGeomToleranceTypeValue, XCAFDimTolObjectsGeomToleranceZoneModif,
};
use martensite_tkxcaf::xcaf_doc::XCAFDocGeomTolerance;

/// The type a combined tolerance records, `None` for another entity.
fn recorded_type(entity: &Handle<StandardTransient>) -> Option<StepDimTolGeometricToleranceType> {
    macro_rules! recorded {
        ($($class:ty),+) => {
            $(if let Some(live) = entity.cast::<$class>().get() {
                return Some(live.get_tolerance_type());
            })+
        };
    }
    recorded!(
        StepDimTolGeoTolAndGeoTolWthDatRef,
        StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol,
        StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMod,
        StepDimTolGeoTolAndGeoTolWthMaxTol,
        StepDimTolGeoTolAndGeoTolWthMod,
        StepDimTolGeoTolAndGeoTolWthDatRefAndUneqDisGeoTol
    );
    None
}

/// `getTolType`: the type of a tolerance; `None` for another entity.
fn tolerance_type_of(entity: &Handle<StandardTransient>) -> XCAFDimTolObjectsGeomToleranceType {
    use XCAFDimTolObjectsGeomToleranceType as Kind;
    if entity.cast::<StepDimTolGeometricTolerance>().is_null() {
        return Kind::None;
    }
    if let Some(recorded) = recorded_type(entity) {
        return STEPCAFControlGDTProperty::get_xcaf_tolerance_type(recorded);
    }
    macro_rules! by_class {
        ($($class:ty => $kind:ident),+) => {
            $(if entity.cast::<$class>().is_some() {
                return Kind::$kind;
            })+
        };
    }
    by_class!(
        StepDimTolAngularityTolerance => Angularity,
        StepDimTolCircularRunoutTolerance => CircularRunout,
        StepDimTolRoundnessTolerance => CircularityOrRoundness,
        StepDimTolCoaxialityTolerance => Coaxiality,
        StepDimTolConcentricityTolerance => Concentricity,
        StepDimTolCylindricityTolerance => Cylindricity,
        StepDimTolFlatnessTolerance => Flatness,
        StepDimTolParallelismTolerance => Parallelism,
        StepDimTolPerpendicularityTolerance => Perpendicularity,
        StepDimTolPositionTolerance => Position,
        StepDimTolGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol => Position,
        StepDimTolLineProfileTolerance => ProfileOfLine,
        StepDimTolSurfaceProfileTolerance => ProfileOfSurface,
        StepDimTolStraightnessTolerance => Straightness,
        StepDimTolSymmetryTolerance => Symmetry,
        StepDimTolTotalRunoutTolerance => TotalRunout
    );
    Kind::None
}

/// A length of a measure with a named unit, `None` otherwise.
fn scaled_length(
    measure: &StepBasicMeasureWithUnit,
    factors: &StepDataFactors,
) -> Result<Option<f64>, Failure> {
    let Some(named) = named_unit_of(&measure.unit_component()) else {
        return Ok(None);
    };
    Ok(Some(
        measure.value_component() * context_of_unit(&named, factors)?.length_factor(),
    ))
}

/// The zone of a tolerance: projected or runout, and its form. False when the returns from the
/// whole member, on a projection without a named unit.
fn read_zone(
    session: &SessionGraph,
    zone_entity: &Handle<StandardTransient>,
    zone: &StepDimTolToleranceZone,
    object: &XCAFDimTolObjectsGeomToleranceObject,
    factors: &StepDataFactors,
) -> Result<bool, Failure> {
    let mut type_value = XCAFDimTolObjectsGeomToleranceTypeValue::None;
    let form = zone.form();
    STEPCAFControlGDTProperty::get_tol_value_type(
        &step_live(&form, WHERE)?.name(),
        &mut type_value,
    )?;
    for sharing in session.sharings(zone_entity)? {
        if let Some(projected) = sharing.cast::<StepDimTolProjectedZoneDefinition>().get() {
            let length = projected.projection_length();
            let Some(length) = length.get() else {
                continue;
            };
            let Some(value) = scaled_length(length, factors)? else {
                return Ok(false);
            };
            object.set_value_of_zone_modifier(value);
            object.set_zone_modifier(XCAFDimTolObjectsGeomToleranceZoneModif::Projected);
        } else if let Some(runout) = sharing.cast::<StepDimTolRunoutZoneDefinition>().get() {
            let orientation = runout.orientation();
            let Some(orientation) = orientation.get() else {
                continue;
            };
            let angle = orientation.angle();
            let angle = step_live(&angle, WHERE)?;
            let mut value = angle.value_component();
            let Some(named) = named_unit_of(&angle.unit_component()) else {
                continue;
            };
            convert_angle_value(&context_of_unit(&named, factors)?, &mut value);
            object.set_value_of_zone_modifier(value);
            object.set_zone_modifier(XCAFDimTolObjectsGeomToleranceZoneModif::Runout);
        }
    }
    object.set_type_of_value(type_value);
    Ok(true)
}

/// The modifiers of a tolerance of any kind, null without.
fn modifiers_of(
    entity: &Handle<StandardTransient>,
) -> Result<Handle<HArray1<StepDimTolGeometricToleranceModifier>>, Failure> {
    if let Some(live) = entity
        .cast::<StepDimTolGeometricToleranceWithModifiers>()
        .get()
    {
        return Ok(live.modifiers());
    }
    macro_rules! through_part {
        ($($class:ty),+) => {
            $(if let Some(live) = entity.cast::<$class>().get() {
                let part = live.get_geometric_tolerance_with_modifiers();
                return Ok(step_live(&part, WHERE)?.modifiers());
            })+
        };
    }
    through_part!(
        StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMod,
        StepDimTolGeoTolAndGeoTolWthMod,
        StepDimTolGeoTolAndGeoTolWthMaxTol,
        StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol
    );
    Ok(Handle::default())
}

/// The value and the unit of the maximal tolerance, when there is one.
fn maximum_of(entity: &Handle<StandardTransient>) -> Result<(f64, StepBasicUnit), Failure> {
    if let Some(live) = entity
        .cast::<StepDimTolGeometricToleranceWithMaximumTolerance>()
        .get()
    {
        let measure = live.maximum_upper_tolerance();
        let measure = step_live(&measure, WHERE)?;
        return Ok((measure.value_component(), measure.unit_component()));
    }
    macro_rules! through_part {
        ($($class:ty),+) => {
            $(if let Some(live) = entity.cast::<$class>().get() {
                let measure = live.get_max_tolerance();
                let measure = step_live(&measure, WHERE)?;
                return Ok((measure.value_component(), measure.unit_component()));
            })+
        };
    }
    through_part!(
        StepDimTolGeoTolAndGeoTolWthMaxTol,
        StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol
    );
    Ok((0., StepBasicUnit::default()))
}

/// `setGeomTolObjectToXCAF`: the object of a tolerance on its label.
pub(super) fn set_geom_tol_object_to_xcaf(
    entity: &Handle<StandardTransient>,
    label: &TdfLabel,
    session: &SessionGraph,
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    let mut attribute = Handle::<XCAFDocGeomTolerance>::default();
    if !label.find_attribute_as(&XCAFDocGeomTolerance::get_id(), &mut attribute)? {
        return Ok(());
    }
    let live_attribute = step_live(&attribute, WHERE)?;
    let object = live_attribute.get_object()?;
    let live_object = step_live(&object, WHERE)?;
    let tolerance = entity.cast::<StepDimTolGeometricTolerance>();
    let live_tolerance = step_live(&tolerance, WHERE)?;
    live_object.set_type(tolerance_type_of(entity));
    // get value
    let measure = measure_with_unit(&live_tolerance.magnitude());
    if let Some(measure) = measure.get() {
        let Some(value) = scaled_length(measure, factors)? else {
            return Ok(());
        };
        live_object.set_value(value);
    }
    // get modifiers
    for sharing in session.sharings(entity)? {
        if let Some(zone) = sharing.cast::<StepDimTolToleranceZone>().get()
            && !read_zone(session, &sharing, zone, live_object, factors)?
        {
            return Ok(());
        }
    }
    let modifiers = modifiers_of(entity)?;
    if let Some(live) = modifiers.get() {
        let (lower, upper) = {
            let items = live.array1();
            (items.lower(), items.upper())
        };
        for rank in lower..=upper {
            match step_array1_item(&modifiers, rank, WHERE)? {
                StepDimTolGeometricToleranceModifier::GTMLeastMaterialRequirement => live_object
                    .set_material_requirement_modifier(
                        XCAFDimTolObjectsGeomToleranceMatReqModif::L,
                    ),
                StepDimTolGeometricToleranceModifier::GTMMaximumMaterialRequirement => live_object
                    .set_material_requirement_modifier(
                        XCAFDimTolObjectsGeomToleranceMatReqModif::M,
                    ),
                other => {
                    if let Some(modifier) =
                        XCAFDimTolObjectsGeomToleranceModif::from_i32(other as i32)
                    {
                        live_object.add_modifier(modifier);
                    }
                }
            }
        }
    }
    let (mut value, unit) = maximum_of(entity)?;
    if let Some(named) = named_unit_of(&unit) {
        convert_angle_value(&context_of_unit(&named, factors)?, &mut value);
        live_object.set_max_value_modifier(value);
    }
    read_annotation(session, entity, &object.cast(), factors)?;
    live_attribute.set_object(&object)?;
    Ok(())
}
