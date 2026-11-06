//! `StepControlReader::file_units`: the unit names of the file, by the roots' representations.
use super::StepControlReader;
use crate::step_basic::{
    StepBasicConversionBasedUnit, StepBasicLengthMeasureWithUnit, StepBasicMeasureWithUnit,
    StepBasicPlaneAngleMeasureWithUnit, StepBasicProductDefinition, StepBasicSiPrefix,
    StepBasicSiUnit, StepBasicSiUnitAndLengthUnit, StepBasicSiUnitAndPlaneAngleUnit,
    StepBasicSiUnitAndSolidAngleUnit, StepBasicSiUnitName, StepBasicSolidAngleMeasureWithUnit,
};
use crate::step_entity::step_live;
use crate::step_geom::{
    StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx,
    StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext,
};
use crate::step_repr::{
    StepReprGlobalUnitAssignedContext, StepReprProductDefinitionShape,
    StepReprReprItemAndMeasureWithUnit, StepReprRepresentationContext,
};
use crate::step_selections::entities_of;
use crate::step_shape::{StepShapeShapeDefinitionRepresentation, StepShapeShapeRepresentation};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkxsbase::interface::{InterfaceGraph, InterfaceModelEntities};
use martensite_tkxsbase::xs_control::XSControlReaderOps;

const WHERE: &str = "StepControlReader::FileUnits";

/// `getSiName`: the prefix and the name of an SI unit.
fn si_name(unit: &StepBasicSiUnit) -> String {
    let mut name = String::new();
    if unit.has_prefix() {
        name.push_str(match unit.prefix() {
            StepBasicSiPrefix::SpExa => "exa",
            StepBasicSiPrefix::SpPeta => "peta",
            StepBasicSiPrefix::SpTera => "tera",
            StepBasicSiPrefix::SpGiga => "giga",
            StepBasicSiPrefix::SpMega => "mega",
            StepBasicSiPrefix::SpHecto => "hecto",
            StepBasicSiPrefix::SpDeca => "deca",
            StepBasicSiPrefix::SpDeci => "deci",
            StepBasicSiPrefix::SpPico => "pico",
            StepBasicSiPrefix::SpFemto => "femto",
            StepBasicSiPrefix::SpAtto => "atto",
            StepBasicSiPrefix::SpKilo => "kilo",
            StepBasicSiPrefix::SpCenti => "centi",
            StepBasicSiPrefix::SpMilli => "milli",
            StepBasicSiPrefix::SpMicro => "micro",
            StepBasicSiPrefix::SpNano => "nano",
        });
    }
    name.push_str(match unit.name() {
        StepBasicSiUnitName::SunMetre => "metre",
        StepBasicSiUnitName::SunRadian => "radian",
        StepBasicSiUnitName::SunSteradian => "steradian",
        _ => "",
    });
    name
}

/// The measure of a conversion-based unit, from its factor.
fn measure_of(conversion: &StepBasicConversionBasedUnit) -> Handle<StepBasicMeasureWithUnit> {
    let factor: Handle<StandardTransient> = conversion.conversion_factor();
    let direct: Handle<StepBasicMeasureWithUnit> = factor.cast();
    if direct.is_some() {
        return direct;
    }
    let item: Handle<StepReprReprItemAndMeasureWithUnit> = factor.cast();
    item.get()
        .map(|live| live.get_measure_with_unit())
        .unwrap_or_default()
}

/// The slot (1 length, 2 angle, 3 solid angle) and name of a unit; `None` for any other.
fn unit_slot(
    unit: &Handle<crate::step_basic::StepBasicNamedUnit>,
) -> Result<Option<(usize, String)>, Failure> {
    let conversion: Handle<StepBasicConversionBasedUnit> = unit.cast();
    if let Some(live) = conversion.get() {
        let measure = measure_of(live);
        let slot = if measure.cast::<StepBasicLengthMeasureWithUnit>().is_some() {
            1
        } else if measure
            .cast::<StepBasicPlaneAngleMeasureWithUnit>()
            .is_some()
        {
            2
        } else if measure
            .cast::<StepBasicSolidAngleMeasureWithUnit>()
            .is_some()
        {
            3
        } else {
            return Ok(None);
        };
        let name = live.name();
        let text = step_live(&name, WHERE)?.string().clone();
        return Ok(Some((
            slot,
            String::from_utf8_lossy(text.as_bytes()).into_owned(),
        )));
    }
    let si: Handle<StepBasicSiUnit> = unit.cast();
    let Some(live) = si.get() else {
        return Ok(None);
    };
    let slot = if si.cast::<StepBasicSiUnitAndLengthUnit>().is_some() {
        1
    } else if si.cast::<StepBasicSiUnitAndPlaneAngleUnit>().is_some() {
        2
    } else if si.cast::<StepBasicSiUnitAndSolidAngleUnit>().is_some() {
        3
    } else {
        return Ok(None);
    };
    Ok(Some((slot, si_name(live))))
}

/// `findUnits`: the three names, empty where the context has none; `None` when it has no unit.
fn find_units(
    context: &Handle<StepReprRepresentationContext>,
) -> Result<Option<[String; 3]>, Failure> {
    let mut assigned = Handle::<StepReprGlobalUnitAssignedContext>::default();
    let simple: Handle<StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext> =
        context.cast();
    if let Some(live) = simple.get() {
        assigned = live.global_unit_assigned_context();
    }
    let full: Handle<StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx> =
        context.cast();
    if let Some(live) = full.get() {
        assigned = live.global_unit_assigned_context();
    }
    let Some(live) = assigned.get() else {
        return Ok(None);
    };
    let mut names: [String; 3] = Default::default();
    let mut found = 0;
    for rank in 1..=live.nb_units() {
        if let Some((slot, name)) = unit_slot(&live.units_value(rank)?)? {
            names[slot - 1] = name;
            found += 1;
        }
    }
    Ok((found != 0).then_some(names))
}

/// The shape_definition_representation of a root.
fn sdr_of_root(
    root: &Handle<StandardTransient>,
    graph: &InterfaceGraph,
) -> Result<Handle<StepShapeShapeDefinitionRepresentation>, Failure> {
    if root.cast::<StepBasicProductDefinition>().is_null() {
        return Ok(root.cast());
    }
    for shape in entities_of(graph.sharings(root)?)? {
        if shape.cast::<StepReprProductDefinitionShape>().is_null() {
            continue;
        }
        for sharing in entities_of(graph.sharings(&shape)?)? {
            let sdr: Handle<StepShapeShapeDefinitionRepresentation> = sharing.cast();
            let shaped = sdr.get().is_some_and(|live| {
                live.used_representation()
                    .cast::<StepShapeShapeRepresentation>()
                    .is_some()
            });
            if shaped {
                return Ok(sdr);
            }
        }
    }
    Ok(Handle::default())
}

/// Adds the names not met yet, per the slot.
fn record_units(names: [String; 3], units: &mut [Vec<AsciiString>; 3]) {
    for (slot, name) in names.iter().enumerate() {
        let text = AsciiString::new(name);
        if !units[slot].contains(&text) {
            units[slot].push(text);
        }
    }
}

impl StepControlReader {
    /// `file_units(unit_length_names, unit_angle_names, unit_solid_angle_names)`: the names per
    /// the roots, or from every representation context when the roots give no length unit.
    pub fn file_units(&mut self) -> Result<[Vec<AsciiString>; 3], Failure> {
        let mut units: [Vec<AsciiString>; 3] = Default::default();
        if self.nb_roots_for_transfer()? == 0 {
            return Ok(units);
        }
        let ws = self.xs_reader().ws();
        let session = step_live(&ws, WHERE)?;
        let hgraph = session.graph()?;
        let graph = step_live(&hgraph, WHERE)?.graph();
        for root in self.xs_reader().theroots.clone() {
            if graph.entity_number(&root) == 0 {
                continue;
            }
            let sdr = sdr_of_root(&root, &graph)?;
            let Some(live) = sdr.get() else {
                continue;
            };
            let rep: Handle<StepShapeShapeRepresentation> = live.used_representation().cast();
            let context = step_live(&rep, WHERE)?.context_of_items();
            if context.is_null() {
                continue;
            }
            if let Some(names) = find_units(&context)? {
                record_units(names, &mut units);
            }
        }
        // for case when units was not found through PDF or SDR
        if units[0].is_empty() {
            let model = self.step_model()?;
            if let Some(live) = model.get() {
                for rank in 1..=live.nb_entities() {
                    let context: Handle<StepReprRepresentationContext> = live.value(rank)?.cast();
                    if context.is_some()
                        && let Some(names) = find_units(&context)?
                    {
                        record_units(names, &mut units);
                    }
                }
            }
        }
        Ok(units)
    }
}
