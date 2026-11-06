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
    /// Representation of STEP entity Datum
    StepDimTolDatum, StepDimTolDatumState, "StepDimTolDatum", StepReprShapeAspect, {
        my_identification: Handle<HAsciiString>,
    }
}
impl StepDimTolDatum {
    step_methods! {
        get identification = my_identification: Handle<HAsciiString>;
        set set_identification = my_identification: Handle<HAsciiString>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_shape_aspect_name: &Handle<HAsciiString>,
        the_shape_aspect_description: &Handle<HAsciiString>,
        the_shape_aspect_of_shape: &Handle<StepReprProductDefinitionShape>,
        the_shape_aspect_product_definitional: StepDataLogical,
        the_identification: &Handle<HAsciiString>,
    ) {
        self.parent.init(
            the_shape_aspect_name,
            the_shape_aspect_description,
            the_shape_aspect_of_shape,
            the_shape_aspect_product_definitional,
        );
        self.state.write().my_identification = the_identification.clone();
    }
}

step_entity! {
    /// Representation of STEP entity DatumFeature
    StepDimTolDatumFeature, StepDimTolDatumFeatureState, "StepDimTolDatumFeature", StepReprShapeAspect, {
    }
}

step_entity! {
    /// Representation of STEP entity DatumReference
    StepDimTolDatumReference, StepDimTolDatumReferenceState, "StepDimTolDatumReference", StandardTransient, {
        my_precedence: i32,
        my_referenced_datum: Handle<StepDimTolDatum>,
    }
}
impl StepDimTolDatumReference {
    step_methods! {
        get precedence = my_precedence: i32;
        set_copy set_precedence = my_precedence: i32;
        get referenced_datum = my_referenced_datum: Handle<StepDimTolDatum>;
        set set_referenced_datum = my_referenced_datum: Handle<StepDimTolDatum>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(&self, the_precedence: i32, the_referenced_datum: &Handle<StepDimTolDatum>) {
        let mut state = self.state.write();
        state.my_precedence = the_precedence;
        state.my_referenced_datum = the_referenced_datum.clone();
    }
}

step_entity! {
    /// Representation of STEP entity DatumReferenceCompartment
    StepDimTolDatumReferenceCompartment, StepDimTolDatumReferenceCompartmentState, "StepDimTolDatumReferenceCompartment", StepDimTolGeneralDatumReference, {
    }
}

step_entity! {
    /// Representation of STEP entity DatumReferenceElement
    StepDimTolDatumReferenceElement, StepDimTolDatumReferenceElementState, "StepDimTolDatumReferenceElement", StepDimTolGeneralDatumReference, {
    }
}

step_entity! {
    /// Representation of STEP entity DatumReferenceModifierWithValue
    StepDimTolDatumReferenceModifierWithValue, StepDimTolDatumReferenceModifierWithValueState, "StepDimTolDatumReferenceModifierWithValue", StandardTransient, {
        my_modifier_type: StepDimTolDatumReferenceModifierType,
        my_modifier_value: Handle<StepBasicLengthMeasureWithUnit>,
    }
}
impl StepDimTolDatumReferenceModifierWithValue {
    step_methods! {
        get modifier_type = my_modifier_type: StepDimTolDatumReferenceModifierType;
        set_copy set_modifier_type = my_modifier_type: StepDimTolDatumReferenceModifierType;
        get modifier_value = my_modifier_value: Handle<StepBasicLengthMeasureWithUnit>;
        set set_modifier_value = my_modifier_value: Handle<StepBasicLengthMeasureWithUnit>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_modifier_type: StepDimTolDatumReferenceModifierType,
        the_modifier_value: &Handle<StepBasicLengthMeasureWithUnit>,
    ) {
        let mut state = self.state.write();
        state.my_modifier_type = the_modifier_type;
        state.my_modifier_value = the_modifier_value.clone();
    }
}

step_entity! {
    /// Representation of STEP entity DatumSystem
    StepDimTolDatumSystem, StepDimTolDatumSystemState, "StepDimTolDatumSystem", StepReprShapeAspect, {
        my_constituents: Handle<HArray1<Handle<StepDimTolDatumReferenceCompartment>>>,
    }
}
impl StepDimTolDatumSystem {
    step_methods! {
        get constituents = my_constituents: Handle<HArray1<Handle<StepDimTolDatumReferenceCompartment>>>;
        set set_constituents = my_constituents: Handle<HArray1<Handle<StepDimTolDatumReferenceCompartment>>>;
        len nb_constituents = my_constituents;
        item constituents_value = my_constituents: Handle<StepDimTolDatumReferenceCompartment>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_of_shape: &Handle<StepReprProductDefinitionShape>,
        the_product_definitional: StepDataLogical,
        the_constituents: &Handle<HArray1<Handle<StepDimTolDatumReferenceCompartment>>>,
    ) {
        self.parent.init(
            the_name,
            the_description,
            the_of_shape,
            the_product_definitional,
        );
        self.state.write().my_constituents = the_constituents.clone();
    }
    /// `ConstituentsValue`.
    /// Sets Constituents with given number
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn constituents_value_datum_reference_compartment(
        &self,
        num: i32,
        the_item: &Handle<StepDimTolDatumReferenceCompartment>,
    ) -> Result<(), Failure> {
        let field_my_constituents = self.state.read().my_constituents.clone();
        step_array1_set(
            &field_my_constituents,
            num,
            the_item.clone(),
            "StepDimTolDatumSystem::ConstituentsValue",
        )?;
        Ok(())
    }
}

step_entity! {
    /// Representation of STEP entity DatumTarget
    StepDimTolDatumTarget, StepDimTolDatumTargetState, "StepDimTolDatumTarget", StepReprShapeAspect, {
        my_target_id: Handle<HAsciiString>,
    }
}
impl StepDimTolDatumTarget {
    step_methods! {
        get target_id = my_target_id: Handle<HAsciiString>;
        set set_target_id = my_target_id: Handle<HAsciiString>;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    pub fn init(
        &self,
        the_shape_aspect_name: &Handle<HAsciiString>,
        the_shape_aspect_description: &Handle<HAsciiString>,
        the_shape_aspect_of_shape: &Handle<StepReprProductDefinitionShape>,
        the_shape_aspect_product_definitional: StepDataLogical,
        the_target_id: &Handle<HAsciiString>,
    ) {
        self.parent.init(
            the_shape_aspect_name,
            the_shape_aspect_description,
            the_shape_aspect_of_shape,
            the_shape_aspect_product_definitional,
        );
        self.state.write().my_target_id = the_target_id.clone();
    }
}

step_entity! {
    /// Representation of STEP entity FlatnessTolerance
    StepDimTolFlatnessTolerance, StepDimTolFlatnessToleranceState, "StepDimTolFlatnessTolerance", StepDimTolGeometricTolerance, {
    }
}

step_entity! {
    /// Representation of STEP entity GeneralDatumReference
    StepDimTolGeneralDatumReference, StepDimTolGeneralDatumReferenceState, "StepDimTolGeneralDatumReference", StepReprShapeAspect, {
        my_base: StepDimTolDatumOrCommonDatum,
        my_modifiers: Handle<HArray1<StepDimTolDatumReferenceModifier>>,
    }
}
impl StepDimTolGeneralDatumReference {
    step_methods! {
        get base = my_base: StepDimTolDatumOrCommonDatum;
        set set_base = my_base: StepDimTolDatumOrCommonDatum;
        get modifiers = my_modifiers: Handle<HArray1<StepDimTolDatumReferenceModifier>>;
        set set_modifiers = my_modifiers: Handle<HArray1<StepDimTolDatumReferenceModifier>>;
        len nb_modifiers = my_modifiers;
        item modifiers_value = my_modifiers: StepDimTolDatumReferenceModifier;
    }
    /// `init`.
    /// Initialize all fields (own and inherited)
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_of_shape: &Handle<StepReprProductDefinitionShape>,
        the_product_definitional: StepDataLogical,
        the_base: &StepDimTolDatumOrCommonDatum,
        the_has_modifiers: bool,
        the_modifiers: &Handle<HArray1<StepDimTolDatumReferenceModifier>>,
    ) {
        self.parent.init(
            the_name,
            the_description,
            the_of_shape,
            the_product_definitional,
        );
        let mut state = self.state.write();
        state.my_base = the_base.clone();
        if the_has_modifiers {
            state.my_modifiers = the_modifiers.clone();
        } else {
            state.my_modifiers = Default::default();
        }
    }
    /// `has_modifiers`.
    /// Indicates is field Modifiers exist
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn has_modifiers(&self) -> Result<bool, Failure> {
        let field_my_modifiers = self.state.read().my_modifiers.clone();
        Ok(!self.state.read().my_modifiers.is_null()
            && step_live(
                &field_my_modifiers,
                "StepDimTolGeneralDatumReference::HasModifiers",
            )?
            .array1()
            .length()
                != 0)
    }
    /// `ModifiersValue`.
    /// Sets Modifiers with given number
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn modifiers_value_datum_reference_modifier(
        &self,
        the_num: i32,
        the_item: &StepDimTolDatumReferenceModifier,
    ) -> Result<(), Failure> {
        let field_my_modifiers = self.state.read().my_modifiers.clone();
        step_array1_set(
            &field_my_modifiers,
            the_num,
            the_item.clone(),
            "StepDimTolGeneralDatumReference::ModifiersValue",
        )?;
        Ok(())
    }
}

step_entity! {
    /// Geo tol and geo tol wth dat ref.
    StepDimTolGeoTolAndGeoTolWthDatRef, StepDimTolGeoTolAndGeoTolWthDatRefState, "StepDimTolGeoTolAndGeoTolWthDatRef", StepDimTolGeometricTolerance, {
        my_geometric_tolerance_with_datum_reference: Handle<StepDimTolGeometricToleranceWithDatumReference>,
        my_tolerance_type: StepDimTolGeometricToleranceType,
    }
}
impl StepDimTolGeoTolAndGeoTolWthDatRef {
    step_methods! {
        set set_geometric_tolerance_with_datum_reference = my_geometric_tolerance_with_datum_reference: Handle<StepDimTolGeometricToleranceWithDatumReference>;
        get get_geometric_tolerance_with_datum_reference = my_geometric_tolerance_with_datum_reference: Handle<StepDimTolGeometricToleranceWithDatumReference>;
        set_copy set_geometric_tolerance_type = my_tolerance_type: StepDimTolGeometricToleranceType;
        get get_tolerance_type = my_tolerance_type: StepDimTolGeometricToleranceType;
    }
    /// `init`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &Handle<StepReprShapeAspect>,
        the_gtwdr: &Handle<StepDimTolGeometricToleranceWithDatumReference>,
        the_type: StepDimTolGeometricToleranceType,
    ) -> Result<(), Failure> {
        self.set_name(the_name);
        self.set_description(the_description);
        self.set_magnitude(the_magnitude);
        self.set_toleranced_shape_aspect(the_toleranced_shape_aspect)?;
        let mut state = self.state.write();
        state.my_geometric_tolerance_with_datum_reference = the_gtwdr.clone();
        state.my_tolerance_type = the_type;
        Ok(())
    }
    /// `init`.
    pub fn init_geometric_tolerance_target(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &StepDimTolGeometricToleranceTarget,
        the_gtwdr: &Handle<StepDimTolGeometricToleranceWithDatumReference>,
        the_type: StepDimTolGeometricToleranceType,
    ) {
        self.set_name(the_name);
        self.set_description(the_description);
        self.set_magnitude(the_magnitude);
        self.set_toleranced_shape_aspect_geometric_tolerance_target(the_toleranced_shape_aspect);
        let mut state = self.state.write();
        state.my_geometric_tolerance_with_datum_reference = the_gtwdr.clone();
        state.my_tolerance_type = the_type;
    }
}
