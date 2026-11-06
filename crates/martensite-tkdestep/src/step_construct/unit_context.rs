//! `StepConstructUnitContext`: the context defining units and tolerances, created for writing and
//! decoded for reading.
//!
//! abv 17.11.99: renamed from the unit and tolerance context and merged with the unit
//! abv 30.02.00: ability to write file in units other than MM
use crate::step_basic::{
    StepBasicConversionBasedUnit, StepBasicConversionBasedUnitAndAreaUnit,
    StepBasicConversionBasedUnitAndLengthUnit, StepBasicConversionBasedUnitAndPlaneAngleUnit,
    StepBasicConversionBasedUnitAndSolidAngleUnit, StepBasicConversionBasedUnitAndVolumeUnit,
    StepBasicDimensionalExponents, StepBasicLengthMeasureWithUnit, StepBasicMeasureValueMember,
    StepBasicMeasureWithUnit, StepBasicNamedUnit, StepBasicSiPrefix, StepBasicSiUnit,
    StepBasicSiUnitAndAreaUnit, StepBasicSiUnitAndLengthUnit, StepBasicSiUnitAndPlaneAngleUnit,
    StepBasicSiUnitAndSolidAngleUnit, StepBasicSiUnitAndVolumeUnit, StepBasicSiUnitName,
    StepBasicUncertaintyMeasureWithUnit, StepBasicUnit,
};
use crate::step_data::{
    StepDataFactors, StepDataSelectMember, StepDataSelectType, StepDataStepModel,
};
use crate::step_entity::{step_live, step_new_array1};
use crate::step_geom::StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx;
use crate::step_repr::{
    StepReprGlobalUncertaintyAssignedContext, StepReprGlobalUnitAssignedContext,
    StepReprReprItemAndMeasureWithUnit,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;

/// ```
/// use martensite_tkdestep::step_basic::StepBasicSiPrefix;
/// use martensite_tkdestep::step_construct::StepConstructUnitContext;
/// let tool = StepConstructUnitContext::new();
/// assert!(!tool.is_done() && !tool.has_uncertainty());
/// assert_eq!(StepConstructUnitContext::convert_si_prefix(StepBasicSiPrefix::SpMilli), 1e-3);
/// ```
#[derive(Clone)]
pub struct StepConstructUnitContext {
    done: bool,
    grc: Handle<StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx>,
    length_factor: f64,
    plane_angle_factor: f64,
    solid_angle_factor: f64,
    the_uncertainty: f64,
    area_factor: f64,
    volume_factor: f64,
    area_done: bool,
    volume_done: bool,
    length_done: bool,
    plane_angle_done: bool,
    solid_angle_done: bool,
    has_uncertainty: bool,
}

impl Default for StepConstructUnitContext {
    fn default() -> Self {
        Self {
            done: false,
            grc: Handle::default(),
            length_factor: 0.0,
            plane_angle_factor: 0.0,
            solid_angle_factor: 0.0,
            // pdn file r_47-sd.stp initialize field.
            the_uncertainty: f64::MAX,
            area_factor: 0.0,
            volume_factor: 0.0,
            area_done: false,
            volume_done: false,
            length_done: false,
            plane_angle_done: false,
            solid_angle_done: false,
            has_uncertainty: false,
        }
    }
}

/// The parts of a length unit `init` writes for a `WriteUnit` value: conversion name, prefix, whether
/// the prefix is written, and the scale to millimetres.
fn write_unit_parts(write_unit: i32) -> (Option<&'static str>, StepBasicSiPrefix, bool, f64) {
    let milli = StepBasicSiPrefix::SpMilli;
    match write_unit {
        1 => (Some("INCH"), milli, true, 25.4),
        4 => (Some("FOOT"), milli, true, 304.8),
        5 => (Some("MILE"), milli, true, 1609344.0),
        6 => (None, milli, false, 1000.0),
        7 => (None, StepBasicSiPrefix::SpKilo, true, 1000000.0),
        8 => (Some("MIL"), milli, true, 0.0254),
        9 => (None, StepBasicSiPrefix::SpMicro, true, 0.001),
        10 => (None, StepBasicSiPrefix::SpCenti, true, 10.0),
        11 => (Some("MICROINCH"), milli, true, 0.0000254),
        _ => (None, milli, true, 1.0),
    }
}

/// The length unit `init` writes: an SI unit, under a conversion based unit for a non-metric one.
fn write_length_unit(write_unit: i32) -> Result<Handle<StepBasicNamedUnit>, Failure> {
    let (name, prefix, has_prefix, scale) = write_unit_parts(write_unit);
    let si_unit = Handle::new(StepBasicSiUnitAndLengthUnit::new());
    si_unit.init(has_prefix, prefix, StepBasicSiUnitName::SunMetre);
    let Some(name) = name else {
        return Ok(si_unit.cast());
    };
    let val = Handle::new(StepBasicMeasureValueMember::new());
    val.set_name("LENGTH_UNIT");
    val.set_real(scale);
    let measure = Handle::new(StepBasicLengthMeasureWithUnit::new());
    let mut unit = StepBasicUnit::default();
    unit.set_value(&si_unit.cast())?;
    measure.init(&val, &unit);
    let dim_exp = Handle::new(StepBasicDimensionalExponents::new());
    dim_exp.init(1., 0., 0., 0., 0., 0., 0.);
    let conv_unit = Handle::new(StepBasicConversionBasedUnitAndLengthUnit::new());
    conv_unit.init(
        &dim_exp,
        &Handle::new(HAsciiString::new(name)),
        &measure.cast(),
    );
    Ok(conv_unit.cast())
}

/// An SI unit without prefix, as `init` writes radians and steradians (the prefix is not written).
fn plane_and_solid_units() -> (Handle<StepBasicNamedUnit>, Handle<StepBasicNamedUnit>) {
    let radian = Handle::new(StepBasicSiUnitAndPlaneAngleUnit::new());
    radian.init(
        false,
        StepBasicSiPrefix::SpMilli,
        StepBasicSiUnitName::SunRadian,
    );
    let steradian = Handle::new(StepBasicSiUnitAndSolidAngleUnit::new());
    steradian.init(
        false,
        StepBasicSiPrefix::SpMilli,
        StepBasicSiUnitName::SunSteradian,
    );
    (radian.cast(), steradian.cast())
}

/// The uncertainty `init` writes: the 3D confusion tolerance, in the written length unit.
fn write_uncertainty(
    tol3d: f64,
    length_unit: &Handle<StepBasicNamedUnit>,
    factors: &StepDataFactors,
) -> Result<Handle<StepBasicUncertaintyMeasureWithUnit>, Failure> {
    let the_tol3d = Handle::new(StepBasicUncertaintyMeasureWithUnit::new());
    let mvs = Handle::new(StepBasicMeasureValueMember::new());
    mvs.set_name("LENGTH_MEASURE");
    mvs.set_real(tol3d / factors.length_factor());
    let mut unit = StepBasicUnit::default();
    unit.set_value(&length_unit.cast())?;
    the_tol3d.init(
        &mvs,
        &unit,
        &Handle::new(HAsciiString::new("distance_accuracy_value")),
        &Handle::new(HAsciiString::new("confusion accuracy")),
    );
    Ok(the_tol3d)
}

impl StepConstructUnitContext {
    /// `StepConstructUnitContext()`: an empty tool.
    pub fn new() -> Self {
        Self::default()
    }
    /// `init`: a new context; units are those of `WriteUnit` and radians, uncertainty is `tol3d`.
    pub fn init(
        &mut self,
        tol3d: f64,
        model: &Handle<StepDataStepModel>,
        local_factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        self.done = true;
        let write_unit = step_live(model, "StepConstructUnitContext::Init")?
            .internal_parameters()
            .write_unit;
        let length_unit = write_length_unit(write_unit)?;
        let (radian, steradian) = plane_and_solid_units();
        let units = step_new_array1::<Handle<StepBasicNamedUnit>>(1, 3)?;
        let tols = step_new_array1::<Handle<StepBasicUncertaintyMeasureWithUnit>>(1, 1)?;
        for (index, unit) in [length_unit.clone(), radian, steradian]
            .into_iter()
            .enumerate()
        {
            crate::step_entity::step_array1_set(
                &units,
                index as i32 + 1,
                unit,
                "StepConstructUnitContext::Init",
            )?;
        }
        let tol = write_uncertainty(tol3d, &length_unit, local_factors)?;
        crate::step_entity::step_array1_set(&tols, 1, tol, "StepConstructUnitContext::Init")?;
        let grc =
            Handle::new(StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx::new());
        grc.init_int(
            &Handle::new(HAsciiString::new("Context #1")),
            &Handle::new(HAsciiString::new("3D Context with UNIT and UNCERTAINTY")),
            3,
            &units,
            &tols,
        );
        self.grc = grc;
        Ok(())
    }
    /// `is_done`: `init` was called successfully.
    pub fn is_done(&self) -> bool {
        self.done
    }
    /// `Value`: the context, null when not done.
    pub fn value(&self) -> Handle<StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx> {
        self.grc.clone()
    }
    /// `convert_si_prefix`: the factor of an SI prefix (1e6 for mega).
    pub fn convert_si_prefix(prefix: StepBasicSiPrefix) -> f64 {
        match prefix {
            StepBasicSiPrefix::SpExa => 1.0E+18,
            StepBasicSiPrefix::SpPeta => 1.0E+15,
            StepBasicSiPrefix::SpTera => 1.0E+12,
            StepBasicSiPrefix::SpGiga => 1.0E+9,
            StepBasicSiPrefix::SpMega => 1.0E+6,
            StepBasicSiPrefix::SpKilo => 1.0E+3,
            StepBasicSiPrefix::SpHecto => 1.0E+2,
            StepBasicSiPrefix::SpDeca => 1.0E+1,
            StepBasicSiPrefix::SpDeci => 1.0E-1,
            StepBasicSiPrefix::SpCenti => 1.0E-2,
            StepBasicSiPrefix::SpMilli => 1.0E-3,
            StepBasicSiPrefix::SpMicro => 1.0E-6,
            StepBasicSiPrefix::SpNano => 1.0E-9,
            StepBasicSiPrefix::SpPico => 1.0E-12,
            StepBasicSiPrefix::SpFemto => 1.0E-15,
            StepBasicSiPrefix::SpAtto => 1.0E-18,
        }
    }
    /// `si_unit_name_factor`: the numerical equivalent of an SI unit name; false when not recognized.
    fn si_unit_name_factor(si_unit: &StepBasicSiUnit, factor: &mut f64) -> bool {
        *factor = 1.;
        matches!(
            si_unit.name(),
            StepBasicSiUnitName::SunMetre
                | StepBasicSiUnitName::SunRadian
                | StepBasicSiUnitName::SunSteradian
        )
    }
    /// `compute_factors(context)`: the conversion factors of all units; a status, 0 if OK.
    pub fn compute_factors(
        &mut self,
        context: &Handle<StepReprGlobalUnitAssignedContext>,
        local_factors: &StepDataFactors,
    ) -> Result<i32, Failure> {
        // Initialise the default value. status: 0 if OK, else 1 2 3
        self.length_factor = 1.;
        self.solid_angle_factor = 1.;
        self.plane_angle_factor = std::f64::consts::PI / 180.;
        self.length_done = false;
        self.plane_angle_done = false;
        self.solid_angle_done = false;
        let Some(context) = context.get() else {
            return Ok(1);
        };
        let mut status = 0;
        for i in 1..=context.nb_units() {
            status = self.compute_unit_factors(&context.units_value(i)?, local_factors)?;
        }
        Ok(status)
    }
    /// The measure of a conversion based unit; null for an unrecognized entity (sln 8.10.2001).
    fn conversion_measure(
        unit: &StepBasicConversionBasedUnit,
        where_called: &str,
    ) -> Result<Handle<StepBasicMeasureWithUnit>, Failure> {
        let factor = unit.conversion_factor();
        step_live(&factor, where_called)?;
        if factor.cast::<StepBasicMeasureWithUnit>().is_some() {
            return Ok(factor.cast());
        }
        let repr = factor.cast::<StepReprReprItemAndMeasureWithUnit>();
        Ok(repr
            .get()
            .map(StepReprReprItemAndMeasureWithUnit::get_measure_with_unit)
            .unwrap_or_default())
    }
    /// The value of a conversion based unit, or the status that ends `compute_factors`.
    fn conversion_parameter(
        unit: &Handle<StepBasicConversionBasedUnit>,
        status: &mut i32,
    ) -> Result<Result<f64, i32>, Failure> {
        const WHERE: &str = "StepConstructUnitContext::ComputeFactors";
        let cbu = step_live(unit, WHERE)?;
        let measure = Self::conversion_measure(cbu, WHERE)?;
        let Some(measure) = measure.get() else {
            return Ok(Err(-1));
        };
        // :f5 abv 24 Apr 98: ProSTEP TR8 tr8_bv1_tc: INCHES
        let target = measure
            .unit_component()
            .named_unit()
            .cast::<StepBasicSiUnit>();
        let Some(si_unit) = target.get() else {
            return Ok(Err(3));
        };
        let prefix_factor = Self::prefix_factor(si_unit);
        let mut name_factor = 0.;
        if !Self::si_unit_name_factor(si_unit, &mut name_factor) {
            *status = 11; // et continue
        }
        Ok(Ok(prefix_factor * measure.value_component()))
    }
    /// The factor of the prefix of an SI unit, 1 without prefix.
    fn prefix_factor(si_unit: &StepBasicSiUnit) -> f64 {
        if si_unit.has_prefix() {
            return Self::convert_si_prefix(si_unit.prefix());
        }
        1.
    }
    /// `compute_factors(unit)`: the conversion factor of one unit; a status, 0 if OK.
    pub fn compute_unit_factors(
        &mut self,
        unit: &Handle<StepBasicNamedUnit>,
        local_factors: &StepDataFactors,
    ) -> Result<i32, Failure> {
        // :f3 abv 8 Apr 98: ProSTEP TR8 tr8_as_sd_sw: the case of unrecognized entity
        if unit.is_null() {
            return Ok(-1);
        }
        let mut status = 0;
        let parameter = if unit.cast::<StepBasicConversionBasedUnit>().is_some() {
            match Self::conversion_parameter(&unit.cast(), &mut status)? {
                Ok(value) => value,
                Err(early) => return Ok(early),
            }
        } else if let Some(si_unit) = unit.cast::<StepBasicSiUnit>().get() {
            let mut name_factor = 0.;
            if !Self::si_unit_name_factor(si_unit, &mut name_factor) {
                status = 11;
            }
            Self::prefix_factor(si_unit) * name_factor
        } else {
            return Ok(0);
        };
        self.record_parameter(unit, parameter, local_factors.cascade_unit(), &mut status);
        Ok(status)
    }
    /// The last part of `compute_factors`: stores the value by the kind of the unit.
    fn record_parameter(
        &mut self,
        unit: &Handle<StepBasicNamedUnit>,
        parameter: f64,
        cascade_unit: f64,
        status: &mut i32,
    ) {
        let scaled = parameter * 1000. / cascade_unit;
        if unit
            .cast::<StepBasicConversionBasedUnitAndLengthUnit>()
            .is_some()
            || unit.cast::<StepBasicSiUnitAndLengthUnit>().is_some()
        {
            self.length_factor = scaled;
            if self.length_done {
                *status = 14;
            }
            self.length_done = true;
        } else if unit
            .cast::<StepBasicConversionBasedUnitAndPlaneAngleUnit>()
            .is_some()
            || unit.cast::<StepBasicSiUnitAndPlaneAngleUnit>().is_some()
        {
            self.plane_angle_factor = parameter;
            self.plane_angle_done = true;
        } else if unit
            .cast::<StepBasicConversionBasedUnitAndSolidAngleUnit>()
            .is_some()
            || unit.cast::<StepBasicSiUnitAndSolidAngleUnit>().is_some()
        {
            self.solid_angle_factor = parameter;
            self.solid_angle_done = true;
        } else if unit
            .cast::<StepBasicConversionBasedUnitAndAreaUnit>()
            .is_some()
            || unit.cast::<StepBasicSiUnitAndAreaUnit>().is_some()
        {
            self.area_done = true;
            self.area_factor = scaled.powi(2);
        } else if unit
            .cast::<StepBasicConversionBasedUnitAndVolumeUnit>()
            .is_some()
            || unit.cast::<StepBasicSiUnitAndVolumeUnit>().is_some()
        {
            self.volume_done = true;
            self.volume_factor = scaled.powi(3);
        }
    }
    /// `compute_tolerance`: the uncertainty for length; a status, 0 if OK.
    pub fn compute_tolerance(
        &mut self,
        context: &Handle<StepReprGlobalUncertaintyAssignedContext>,
    ) -> Result<i32, Failure> {
        // Decode the Uncertainty information (geometric accuracy)
        self.has_uncertainty = false;
        let Some(context) = context.get() else {
            return Ok(40);
        };
        for un in 1..=context.nb_uncertainty() {
            let measure = context.uncertainty_value(un)?;
            let Some(measure) = measure.get() else {
                continue;
            };
            let unit = measure.unit_component().named_unit();
            if unit.cast::<StepBasicSiUnitAndLengthUnit>().is_some()
                || unit
                    .cast::<StepBasicConversionBasedUnitAndLengthUnit>()
                    .is_some()
            {
                // pdn r_47-sd.stp to choose minimal uncertainty
                self.the_uncertainty = self.the_uncertainty.min(measure.value_component());
                self.has_uncertainty = true;
            }
        }
        Ok(0)
    }
    /// `length_factor`.
    pub fn length_factor(&self) -> f64 {
        self.length_factor
    }
    /// `plane_angle_factor`.
    pub fn plane_angle_factor(&self) -> f64 {
        self.plane_angle_factor
    }
    /// `solid_angle_factor`.
    pub fn solid_angle_factor(&self) -> f64 {
        self.solid_angle_factor
    }
    /// `uncertainty`: for length; the caller converts it with the length factor.
    pub fn uncertainty(&self) -> f64 {
        self.the_uncertainty
    }
    /// `area_factor`.
    pub fn area_factor(&self) -> f64 {
        self.area_factor
    }
    /// `volume_factor`.
    pub fn volume_factor(&self) -> f64 {
        self.volume_factor
    }
    /// `has_uncertainty`.
    pub fn has_uncertainty(&self) -> bool {
        self.has_uncertainty
    }
    /// `length_done`.
    pub fn length_done(&self) -> bool {
        self.length_done
    }
    /// `plane_angle_done`.
    pub fn plane_angle_done(&self) -> bool {
        self.plane_angle_done
    }
    /// `solid_angle_done`.
    pub fn solid_angle_done(&self) -> bool {
        self.solid_angle_done
    }
    /// `area_done`.
    pub fn area_done(&self) -> bool {
        self.area_done
    }
    /// `volume_done`.
    pub fn volume_done(&self) -> bool {
        self.volume_done
    }
    /// `status_message`: the message of a status, to add as a transfer warning (0: empty).
    pub fn status_message(&self, status: i32) -> &'static str {
        match status {
            0 => "",
            1 => "No GlobalUnitAssignedContext, default taken",
            2 => "No LengthMeasureWithUnit, default taken",
            3 => "No SiUnit for LengthMeasure undefined, default taken",
            4 => "No PlaneAngleMeasureWithUnit, default taken",
            5 => "No SiUnit for PlaneAngleMeasure undefined, default taken",
            6 => "No SolidAngleMeasureWithUnit, default taken",
            7 => "No SiUnit for SolidAngleMeasure undefined, default taken",
            11 => "Length Unit not recognized, default taken",
            12 => "Plane Angle Unit not recognized, default taken",
            13 => "Solid Angle Unit not recognized, default taken",
            14 => "At least one unit is twice defined",
            40 => "Bad GlobalUncertaintyAssignedContext, default unit taken",
            _ => "Badly defined units, default taken",
        }
    }
}
