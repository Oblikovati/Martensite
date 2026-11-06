#[allow(
    unused_imports,
    reason = "the classes of the package reference each other across its files"
)]
use super::*;
#[allow(unused_imports)]
use crate::step_basic::*;
#[allow(unused_imports)]
use crate::step_data::*;
use crate::step_entity::*;
#[allow(unused_imports)]
use crate::step_repr::*;

step_entity! {
    /// Representation of STEP entity NonUniformZoneDefinition
    StepDimTolNonUniformZoneDefinition, StepDimTolNonUniformZoneDefinitionState, "StepDimTolNonUniformZoneDefinition", StepDimTolToleranceZoneDefinition, {
    }
}

step_entity! {
    /// Representation of STEP entity ParallelismTolerance
    StepDimTolParallelismTolerance, StepDimTolParallelismToleranceState, "StepDimTolParallelismTolerance", StepDimTolGeometricToleranceWithDatumReference, {
    }
}

step_entity! {
    /// Representation of STEP entity PerpendicularityTolerance
    StepDimTolPerpendicularityTolerance, StepDimTolPerpendicularityToleranceState, "StepDimTolPerpendicularityTolerance", StepDimTolGeometricToleranceWithDatumReference, {
    }
}

step_entity! {
    /// Representation of STEP entity PlacedDatumTargetFeature
    StepDimTolPlacedDatumTargetFeature, StepDimTolPlacedDatumTargetFeatureState, "StepDimTolPlacedDatumTargetFeature", StepDimTolDatumTarget, {
    }
}

step_entity! {
    /// Representation of STEP entity PositionTolerance
    StepDimTolPositionTolerance, StepDimTolPositionToleranceState, "StepDimTolPositionTolerance", StepDimTolGeometricTolerance, {
    }
}

step_entity! {
    /// Representation of STEP entity ProjectedZoneDefinition
    StepDimTolProjectedZoneDefinition, StepDimTolProjectedZoneDefinitionState, "StepDimTolProjectedZoneDefinition", StepDimTolToleranceZoneDefinition, {
        my_projection_end: Handle<StepReprShapeAspect>,
        my_projection_length: Handle<StepBasicLengthMeasureWithUnit>,
    }
}
impl StepDimTolProjectedZoneDefinition {
    step_methods! {
        get projection_end = my_projection_end: Handle<StepReprShapeAspect>;
        set set_projection_end = my_projection_end: Handle<StepReprShapeAspect>;
        get projection_length = my_projection_length: Handle<StepBasicLengthMeasureWithUnit>;
        set set_projection_length = my_projection_length: Handle<StepBasicLengthMeasureWithUnit>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_zone: &Handle<StepDimTolToleranceZone>,
        the_boundaries: &Handle<HArray1<Handle<StepReprShapeAspect>>>,
        the_projection_end: &Handle<StepReprShapeAspect>,
        the_projection_length: &Handle<StepBasicLengthMeasureWithUnit>,
    ) {
        self.parent.init(the_zone, the_boundaries);
        let mut state = self.state.write();
        state.my_projection_end = the_projection_end.clone();
        state.my_projection_length = the_projection_length.clone();
    }
}

step_entity! {
    /// Representation of STEP entity RoundnessTolerance
    StepDimTolRoundnessTolerance, StepDimTolRoundnessToleranceState, "StepDimTolRoundnessTolerance", StepDimTolGeometricTolerance, {
    }
}

step_entity! {
    /// Representation of STEP entity ToleranceZoneDefinition
    StepDimTolRunoutZoneDefinition, StepDimTolRunoutZoneDefinitionState, "StepDimTolRunoutZoneDefinition", StepDimTolToleranceZoneDefinition, {
        my_orientation: Handle<StepDimTolRunoutZoneOrientation>,
    }
}
impl StepDimTolRunoutZoneDefinition {
    step_methods! {
        get orientation = my_orientation: Handle<StepDimTolRunoutZoneOrientation>;
        set set_orientation = my_orientation: Handle<StepDimTolRunoutZoneOrientation>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_zone: &Handle<StepDimTolToleranceZone>,
        the_boundaries: &Handle<HArray1<Handle<StepReprShapeAspect>>>,
        the_orientation: &Handle<StepDimTolRunoutZoneOrientation>,
    ) {
        self.parent.init(the_zone, the_boundaries);
        self.state.write().my_orientation = the_orientation.clone();
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepDimTolRunoutZoneOrientation, StepDimTolRunoutZoneOrientationState, "StepDimTolRunoutZoneOrientation", StandardTransient, {
        my_angle: Handle<StepBasicPlaneAngleMeasureWithUnit>,
    }
}
impl StepDimTolRunoutZoneOrientation {
    step_methods! {
        set init = my_angle: Handle<StepBasicPlaneAngleMeasureWithUnit>;
        get angle = my_angle: Handle<StepBasicPlaneAngleMeasureWithUnit>;
        set set_angle = my_angle: Handle<StepBasicPlaneAngleMeasureWithUnit>;
    }
}

step_entity! {
    /// Representation of STEP entity StraightnessTolerance
    StepDimTolStraightnessTolerance, StepDimTolStraightnessToleranceState, "StepDimTolStraightnessTolerance", StepDimTolGeometricTolerance, {
    }
}

step_entity! {
    /// Representation of STEP entity SurfaceProfileTolerance
    StepDimTolSurfaceProfileTolerance, StepDimTolSurfaceProfileToleranceState, "StepDimTolSurfaceProfileTolerance", StepDimTolGeometricTolerance, {
    }
}

step_entity! {
    /// Representation of STEP entity SymmetryTolerance
    StepDimTolSymmetryTolerance, StepDimTolSymmetryToleranceState, "StepDimTolSymmetryTolerance", StepDimTolGeometricToleranceWithDatumReference, {
    }
}

step_entity! {
    /// Representation of STEP entity ToleranceZone
    StepDimTolToleranceZone, StepDimTolToleranceZoneState, "StepDimTolToleranceZone", StepReprShapeAspect, {
        my_defining_tolerance: Handle<HArray1<StepDimTolToleranceZoneTarget>>,
        my_form: Handle<StepDimTolToleranceZoneForm>,
    }
}
impl StepDimTolToleranceZone {
    step_methods! {
        get defining_tolerance = my_defining_tolerance: Handle<HArray1<StepDimTolToleranceZoneTarget>>;
        set set_defining_tolerance = my_defining_tolerance: Handle<HArray1<StepDimTolToleranceZoneTarget>>;
        len nb_defining_tolerances = my_defining_tolerance;
        item defining_tolerance_value = my_defining_tolerance: StepDimTolToleranceZoneTarget;
        get form = my_form: Handle<StepDimTolToleranceZoneForm>;
        set set_form = my_form: Handle<StepDimTolToleranceZoneForm>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_of_shape: &Handle<StepReprProductDefinitionShape>,
        the_product_definitional: StepDataLogical,
        the_defining_tolerance: &Handle<HArray1<StepDimTolToleranceZoneTarget>>,
        the_form: &Handle<StepDimTolToleranceZoneForm>,
    ) {
        self.parent.init(
            the_name,
            the_description,
            the_of_shape,
            the_product_definitional,
        );
        let mut state = self.state.write();
        state.my_defining_tolerance = the_defining_tolerance.clone();
        state.my_form = the_form.clone();
    }
    /// `set_defining_tolerance_value`.
    /// Sets Defining Tolerance with given number
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_defining_tolerance_value(
        &self,
        the_num: i32,
        the_item: &StepDimTolToleranceZoneTarget,
    ) -> Result<(), Failure> {
        let field_my_defining_tolerance = self.state.read().my_defining_tolerance.clone();
        step_array1_set(
            &field_my_defining_tolerance,
            the_num,
            the_item.clone(),
            "StepDimTolToleranceZone::SetDefiningToleranceValue",
        )?;
        Ok(())
    }
}

step_entity! {
    /// Representation of STEP entity ToleranceZoneDefinition
    StepDimTolToleranceZoneDefinition, StepDimTolToleranceZoneDefinitionState, "StepDimTolToleranceZoneDefinition", StandardTransient, {
        my_boundaries: Handle<HArray1<Handle<StepReprShapeAspect>>>,
        my_zone: Handle<StepDimTolToleranceZone>,
    }
}
impl StepDimTolToleranceZoneDefinition {
    step_methods! {
        get boundaries = my_boundaries: Handle<HArray1<Handle<StepReprShapeAspect>>>;
        set set_boundaries = my_boundaries: Handle<HArray1<Handle<StepReprShapeAspect>>>;
        len nb_boundaries = my_boundaries;
        item boundaries_value = my_boundaries: Handle<StepReprShapeAspect>;
        get zone = my_zone: Handle<StepDimTolToleranceZone>;
        set set_zone = my_zone: Handle<StepDimTolToleranceZone>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_zone: &Handle<StepDimTolToleranceZone>,
        the_boundaries: &Handle<HArray1<Handle<StepReprShapeAspect>>>,
    ) {
        let mut state = self.state.write();
        state.my_boundaries = the_boundaries.clone();
        state.my_zone = the_zone.clone();
    }
    /// `set_boundaries_value`.
    /// Sets Boundaries with given number
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_boundaries_value(
        &self,
        the_num: i32,
        the_item: &Handle<StepReprShapeAspect>,
    ) -> Result<(), Failure> {
        let field_my_boundaries = self.state.read().my_boundaries.clone();
        step_array1_set(
            &field_my_boundaries,
            the_num,
            the_item.clone(),
            "StepDimTolToleranceZoneDefinition::SetBoundariesValue",
        )?;
        Ok(())
    }
}

step_entity! {
    /// Added for Dimensional Tolerances
    StepDimTolToleranceZoneForm, StepDimTolToleranceZoneFormState, "StepDimTolToleranceZoneForm", StandardTransient, {
        my_name: Handle<HAsciiString>,
    }
}
impl StepDimTolToleranceZoneForm {
    step_methods! {
        set init = my_name: Handle<HAsciiString>;
        get name = my_name: Handle<HAsciiString>;
        set set_name = my_name: Handle<HAsciiString>;
    }
}

step_entity! {
    /// Representation of STEP entity TotalRunoutTolerance
    StepDimTolTotalRunoutTolerance, StepDimTolTotalRunoutToleranceState, "StepDimTolTotalRunoutTolerance", StepDimTolGeometricToleranceWithDatumReference, {
    }
}

step_entity! {
    /// Representation of STEP entity UnequallyDisposedGeometricTolerance
    StepDimTolUnequallyDisposedGeometricTolerance, StepDimTolUnequallyDisposedGeometricToleranceState, "StepDimTolUnequallyDisposedGeometricTolerance", StepDimTolGeometricTolerance, {
        my_displacement: Handle<StepBasicLengthMeasureWithUnit>,
    }
}
impl StepDimTolUnequallyDisposedGeometricTolerance {
    step_methods! {
        get displacement = my_displacement: Handle<StepBasicLengthMeasureWithUnit>;
        set set_displacement = my_displacement: Handle<StepBasicLengthMeasureWithUnit>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &StepDimTolGeometricToleranceTarget,
        the_displacement: &Handle<StepBasicLengthMeasureWithUnit>,
    ) {
        self.parent.init_geometric_tolerance_target(
            the_name,
            the_description,
            the_magnitude,
            the_toleranced_shape_aspect,
        );
        self.state.write().my_displacement = the_displacement.clone();
    }
}
