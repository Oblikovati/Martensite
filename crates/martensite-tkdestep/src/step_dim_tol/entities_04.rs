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
    /// Representation of STEP entity GeometricTolerance
    StepDimTolGeometricTolerance, StepDimTolGeometricToleranceState, "StepDimTolGeometricTolerance", StandardTransient, {
        my_name: Handle<HAsciiString>,
        my_description: Handle<HAsciiString>,
        my_magnitude: Handle<StandardTransient>,
        my_toleranced_shape_aspect: StepDimTolGeometricToleranceTarget,
    }
}
impl StepDimTolGeometricTolerance {
    step_methods! {
        get name = my_name: Handle<HAsciiString>;
        set set_name = my_name: Handle<HAsciiString>;
        get description = my_description: Handle<HAsciiString>;
        set set_description = my_description: Handle<HAsciiString>;
        get magnitude = my_magnitude: Handle<StandardTransient>;
        set set_magnitude = my_magnitude: Handle<StandardTransient>;
        get toleranced_shape_aspect = my_toleranced_shape_aspect: StepDimTolGeometricToleranceTarget;
    }
    /// `init`.
    /// Initialize all fields (own and inherited) AP214
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &Handle<StepReprShapeAspect>,
    ) -> Result<(), Failure> {
        let mut state = self.state.write();
        state.my_name = the_name.clone();
        state.my_description = the_description.clone();
        state.my_magnitude = the_magnitude.clone();
        state
            .my_toleranced_shape_aspect
            .set_value(&the_toleranced_shape_aspect.cast())?;
        Ok(())
    }
    /// `init`.
    /// Initialize all fields (own and inherited) AP242
    pub fn init_geometric_tolerance_target(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &StepDimTolGeometricToleranceTarget,
    ) {
        let mut state = self.state.write();
        state.my_name = the_name.clone();
        state.my_description = the_description.clone();
        state.my_magnitude = the_magnitude.clone();
        state.my_toleranced_shape_aspect = the_toleranced_shape_aspect.clone();
    }
    /// `set_toleranced_shape_aspect`.
    /// Set field TolerancedShapeAspect AP214
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_toleranced_shape_aspect(
        &self,
        the_toleranced_shape_aspect: &Handle<StepReprShapeAspect>,
    ) -> Result<(), Failure> {
        self.state
            .write()
            .my_toleranced_shape_aspect
            .set_value(&the_toleranced_shape_aspect.cast())?;
        Ok(())
    }
    /// `set_toleranced_shape_aspect`.
    /// Set field TolerancedShapeAspect AP242
    pub fn set_toleranced_shape_aspect_geometric_tolerance_target(
        &self,
        the_toleranced_shape_aspect: &StepDimTolGeometricToleranceTarget,
    ) {
        self.state.write().my_toleranced_shape_aspect = the_toleranced_shape_aspect.clone();
    }
}

step_entity! {
    /// Representation of STEP entity GeometricToleranceRelationship
    StepDimTolGeometricToleranceRelationship, StepDimTolGeometricToleranceRelationshipState, "StepDimTolGeometricToleranceRelationship", StandardTransient, {
        my_name: Handle<HAsciiString>,
        my_description: Handle<HAsciiString>,
        my_relating_geometric_tolerance: Handle<StepDimTolGeometricTolerance>,
        my_related_geometric_tolerance: Handle<StepDimTolGeometricTolerance>,
    }
}
impl StepDimTolGeometricToleranceRelationship {
    step_methods! {
        get name = my_name: Handle<HAsciiString>;
        set set_name = my_name: Handle<HAsciiString>;
        get description = my_description: Handle<HAsciiString>;
        set set_description = my_description: Handle<HAsciiString>;
        get relating_geometric_tolerance = my_relating_geometric_tolerance: Handle<StepDimTolGeometricTolerance>;
        set set_relating_geometric_tolerance = my_relating_geometric_tolerance: Handle<StepDimTolGeometricTolerance>;
        get related_geometric_tolerance = my_related_geometric_tolerance: Handle<StepDimTolGeometricTolerance>;
        set set_related_geometric_tolerance = my_related_geometric_tolerance: Handle<StepDimTolGeometricTolerance>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_relating_geometric_tolerance: &Handle<StepDimTolGeometricTolerance>,
        the_related_geometric_tolerance: &Handle<StepDimTolGeometricTolerance>,
    ) {
        let mut state = self.state.write();
        state.my_name = the_name.clone();
        state.my_description = the_description.clone();
        state.my_relating_geometric_tolerance = the_relating_geometric_tolerance.clone();
        state.my_related_geometric_tolerance = the_related_geometric_tolerance.clone();
    }
}

step_entity! {
    /// Representation of STEP entity GeometricToleranceWithDatumReference
    StepDimTolGeometricToleranceWithDatumReference, StepDimTolGeometricToleranceWithDatumReferenceState, "StepDimTolGeometricToleranceWithDatumReference", StepDimTolGeometricTolerance, {
        my_datum_system: Handle<HArray1<StepDimTolDatumSystemOrReference>>,
    }
}
impl StepDimTolGeometricToleranceWithDatumReference {
    step_methods! {
        get datum_system_ap242 = my_datum_system: Handle<HArray1<StepDimTolDatumSystemOrReference>>;
    }
}

step_entity! {
    /// Representation of STEP entity GeometricToleranceWithDefinedAreaUnit
    StepDimTolGeometricToleranceWithDefinedAreaUnit, StepDimTolGeometricToleranceWithDefinedAreaUnitState, "StepDimTolGeometricToleranceWithDefinedAreaUnit", StepDimTolGeometricToleranceWithDefinedUnit, {
        my_area_type: StepDimTolAreaUnitType,
        my_second_unit_size: Handle<StepBasicLengthMeasureWithUnit>,
    }
}
impl StepDimTolGeometricToleranceWithDefinedAreaUnit {
    step_methods! {
        get area_type = my_area_type: StepDimTolAreaUnitType;
        set_copy set_area_type = my_area_type: StepDimTolAreaUnitType;
        get second_unit_size = my_second_unit_size: Handle<StepBasicLengthMeasureWithUnit>;
        set set_second_unit_size = my_second_unit_size: Handle<StepBasicLengthMeasureWithUnit>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &StepDimTolGeometricToleranceTarget,
        the_unit_size: &Handle<StepBasicLengthMeasureWithUnit>,
        the_unit_type: StepDimTolAreaUnitType,
        the_has_second_unit_size: bool,
        the_second_unit_size: &Handle<StepBasicLengthMeasureWithUnit>,
    ) {
        self.parent.init_geometric_tolerance_target(
            the_name,
            the_description,
            the_magnitude,
            the_toleranced_shape_aspect,
            the_unit_size,
        );
        let mut state = self.state.write();
        state.my_area_type = the_unit_type;
        if the_has_second_unit_size {
            state.my_second_unit_size = the_second_unit_size.clone();
        } else {
            state.my_second_unit_size = Default::default();
        }
    }
    /// `has_second_unit_size`.
    /// Indicates if SecondUnitSize field exist
    pub fn has_second_unit_size(&self) -> bool {
        self.state.read().my_second_unit_size.is_null()
    }
}

step_entity! {
    /// Representation of STEP entity GeometricToleranceWithDefinedUnit
    StepDimTolGeometricToleranceWithDefinedUnit, StepDimTolGeometricToleranceWithDefinedUnitState, "StepDimTolGeometricToleranceWithDefinedUnit", StepDimTolGeometricTolerance, {
        my_unit_size: Handle<StepBasicLengthMeasureWithUnit>,
    }
}
impl StepDimTolGeometricToleranceWithDefinedUnit {
    step_methods! {
        get unit_size = my_unit_size: Handle<StepBasicLengthMeasureWithUnit>;
        set set_unit_size = my_unit_size: Handle<StepBasicLengthMeasureWithUnit>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited) AP214
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &Handle<StepReprShapeAspect>,
        the_unit_size: &Handle<StepBasicLengthMeasureWithUnit>,
    ) -> Result<(), Failure> {
        self.parent.init(
            the_name,
            the_description,
            the_magnitude,
            the_toleranced_shape_aspect,
        )?;
        self.state.write().my_unit_size = the_unit_size.clone();
        Ok(())
    }
    /// `init`.
    /// Initialize all fields (own and inherited) AP242
    pub fn init_geometric_tolerance_target(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &StepDimTolGeometricToleranceTarget,
        the_unit_size: &Handle<StepBasicLengthMeasureWithUnit>,
    ) {
        self.parent.init_geometric_tolerance_target(
            the_name,
            the_description,
            the_magnitude,
            the_toleranced_shape_aspect,
        );
        self.state.write().my_unit_size = the_unit_size.clone();
    }
}

step_entity! {
    /// Representation of STEP entity GeometricToleranceWithMaximumTolerance
    StepDimTolGeometricToleranceWithMaximumTolerance, StepDimTolGeometricToleranceWithMaximumToleranceState, "StepDimTolGeometricToleranceWithMaximumTolerance", StepDimTolGeometricToleranceWithModifiers, {
        my_maximum_upper_tolerance: Handle<StepBasicLengthMeasureWithUnit>,
    }
}
impl StepDimTolGeometricToleranceWithMaximumTolerance {
    step_methods! {
        get maximum_upper_tolerance = my_maximum_upper_tolerance: Handle<StepBasicLengthMeasureWithUnit>;
        set set_maximum_upper_tolerance = my_maximum_upper_tolerance: Handle<StepBasicLengthMeasureWithUnit>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &StepDimTolGeometricToleranceTarget,
        the_modifiers: &Handle<HArray1<StepDimTolGeometricToleranceModifier>>,
        the_maximum_upper_tolerance: &Handle<StepBasicLengthMeasureWithUnit>,
    ) {
        self.parent.init(
            the_name,
            the_description,
            the_magnitude,
            the_toleranced_shape_aspect,
            the_modifiers,
        );
        self.state.write().my_maximum_upper_tolerance = the_maximum_upper_tolerance.clone();
    }
}

step_entity! {
    /// Representation of STEP entity GeometricToleranceWithModifiers
    StepDimTolGeometricToleranceWithModifiers, StepDimTolGeometricToleranceWithModifiersState, "StepDimTolGeometricToleranceWithModifiers", StepDimTolGeometricTolerance, {
        my_modifiers: Handle<HArray1<StepDimTolGeometricToleranceModifier>>,
    }
}
impl StepDimTolGeometricToleranceWithModifiers {
    step_methods! {
        get modifiers = my_modifiers: Handle<HArray1<StepDimTolGeometricToleranceModifier>>;
        set set_modifiers = my_modifiers: Handle<HArray1<StepDimTolGeometricToleranceModifier>>;
        len nb_modifiers = my_modifiers;
        item modifier_value = my_modifiers: StepDimTolGeometricToleranceModifier;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &StepDimTolGeometricToleranceTarget,
        the_modifiers: &Handle<HArray1<StepDimTolGeometricToleranceModifier>>,
    ) {
        self.parent.init_geometric_tolerance_target(
            the_name,
            the_description,
            the_magnitude,
            the_toleranced_shape_aspect,
        );
        self.state.write().my_modifiers = the_modifiers.clone();
    }
    /// `set_modifier_value`.
    /// Sets modifier with given number
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_modifier_value(
        &self,
        the_num: i32,
        the_item: StepDimTolGeometricToleranceModifier,
    ) -> Result<(), Failure> {
        let field_my_modifiers = self.state.read().my_modifiers.clone();
        step_array1_set(
            &field_my_modifiers,
            the_num,
            the_item,
            "StepDimTolGeometricToleranceWithModifiers::SetModifierValue",
        )?;
        Ok(())
    }
}

step_entity! {
    /// Representation of STEP entity LineProfileTolerance
    StepDimTolLineProfileTolerance, StepDimTolLineProfileToleranceState, "StepDimTolLineProfileTolerance", StepDimTolGeometricTolerance, {
    }
}

step_entity! {
    /// Representation of STEP entity ModifiedGeometricTolerance
    StepDimTolModifiedGeometricTolerance, StepDimTolModifiedGeometricToleranceState, "StepDimTolModifiedGeometricTolerance", StepDimTolGeometricTolerance, {
        my_modifier: StepDimTolLimitCondition,
    }
}
impl StepDimTolModifiedGeometricTolerance {
    step_methods! {
        get modifier = my_modifier: StepDimTolLimitCondition;
        set_copy set_modifier = my_modifier: StepDimTolLimitCondition;
    }
    /// `init`.
    /// Initialize all fields (own and inherited) AP214
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn init(
        &self,
        the_geometric_tolerance_name: &Handle<HAsciiString>,
        the_geometric_tolerance_description: &Handle<HAsciiString>,
        the_geometric_tolerance_magnitude: &Handle<StandardTransient>,
        the_geometric_tolerance_toleranced_shape_aspect: &Handle<StepReprShapeAspect>,
        the_modifier: StepDimTolLimitCondition,
    ) -> Result<(), Failure> {
        self.parent.init(
            the_geometric_tolerance_name,
            the_geometric_tolerance_description,
            the_geometric_tolerance_magnitude,
            the_geometric_tolerance_toleranced_shape_aspect,
        )?;
        self.state.write().my_modifier = the_modifier;
        Ok(())
    }
    /// `init`.
    /// Initialize all fields (own and inherited) AP242
    pub fn init_geometric_tolerance_target(
        &self,
        the_geometric_tolerance_name: &Handle<HAsciiString>,
        the_geometric_tolerance_description: &Handle<HAsciiString>,
        the_geometric_tolerance_magnitude: &Handle<StandardTransient>,
        the_geometric_tolerance_toleranced_shape_aspect: &StepDimTolGeometricToleranceTarget,
        the_modifier: StepDimTolLimitCondition,
    ) {
        self.parent.init_geometric_tolerance_target(
            the_geometric_tolerance_name,
            the_geometric_tolerance_description,
            the_geometric_tolerance_magnitude,
            the_geometric_tolerance_toleranced_shape_aspect,
        );
        self.state.write().my_modifier = the_modifier;
    }
}
