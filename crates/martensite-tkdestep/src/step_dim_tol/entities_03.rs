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
    /// Geo tol and geo tol wth dat ref and geo tol wth max tol.
    StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol, StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTolState, "StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol", StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMod, {
        my_max_tol: Handle<StepBasicLengthMeasureWithUnit>,
    }
}
impl StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol {
    step_methods! {
        set set_max_tolerance = my_max_tol: Handle<StepBasicLengthMeasureWithUnit>;
        get get_max_tolerance = my_max_tol: Handle<StepBasicLengthMeasureWithUnit>;
    }
    /// `init`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &Handle<StepReprShapeAspect>,
        the_gtwdr: &Handle<StepDimTolGeometricToleranceWithDatumReference>,
        the_gtwm: &Handle<StepDimTolGeometricToleranceWithModifiers>,
        the_max_tol: &Handle<StepBasicLengthMeasureWithUnit>,
        the_type: StepDimTolGeometricToleranceType,
    ) -> Result<(), Failure> {
        self.parent.init(
            the_name,
            the_description,
            the_magnitude,
            the_toleranced_shape_aspect,
            the_gtwdr,
            the_gtwm,
            the_type,
        )?;
        self.state.write().my_max_tol = the_max_tol.clone();
        Ok(())
    }
    /// `init`.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init_geometric_tolerance_target(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &StepDimTolGeometricToleranceTarget,
        the_gtwdr: &Handle<StepDimTolGeometricToleranceWithDatumReference>,
        the_gtwm: &Handle<StepDimTolGeometricToleranceWithModifiers>,
        the_max_tol: &Handle<StepBasicLengthMeasureWithUnit>,
        the_type: StepDimTolGeometricToleranceType,
    ) {
        self.parent.init_geometric_tolerance_target(
            the_name,
            the_description,
            the_magnitude,
            the_toleranced_shape_aspect,
            the_gtwdr,
            the_gtwm,
            the_type,
        );
        self.state.write().my_max_tol = the_max_tol.clone();
    }
}

step_entity! {
    /// Geo tol and geo tol wth dat ref and geo tol wth mod.
    StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMod, StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthModState, "StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMod", StepDimTolGeometricTolerance, {
        my_geometric_tolerance_with_datum_reference: Handle<StepDimTolGeometricToleranceWithDatumReference>,
        my_geometric_tolerance_with_modifiers: Handle<StepDimTolGeometricToleranceWithModifiers>,
        my_tolerance_type: StepDimTolGeometricToleranceType,
    }
}
impl StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMod {
    step_methods! {
        set set_geometric_tolerance_with_datum_reference = my_geometric_tolerance_with_datum_reference: Handle<StepDimTolGeometricToleranceWithDatumReference>;
        get get_geometric_tolerance_with_datum_reference = my_geometric_tolerance_with_datum_reference: Handle<StepDimTolGeometricToleranceWithDatumReference>;
        set set_geometric_tolerance_with_modifiers = my_geometric_tolerance_with_modifiers: Handle<StepDimTolGeometricToleranceWithModifiers>;
        get get_geometric_tolerance_with_modifiers = my_geometric_tolerance_with_modifiers: Handle<StepDimTolGeometricToleranceWithModifiers>;
        set_copy set_geometric_tolerance_type = my_tolerance_type: StepDimTolGeometricToleranceType;
        get get_tolerance_type = my_tolerance_type: StepDimTolGeometricToleranceType;
    }
    /// `init`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &Handle<StepReprShapeAspect>,
        the_gtwdr: &Handle<StepDimTolGeometricToleranceWithDatumReference>,
        the_gtwm: &Handle<StepDimTolGeometricToleranceWithModifiers>,
        the_type: StepDimTolGeometricToleranceType,
    ) -> Result<(), Failure> {
        self.set_name(the_name);
        self.set_description(the_description);
        self.set_magnitude(the_magnitude);
        self.set_toleranced_shape_aspect(the_toleranced_shape_aspect)?;
        let mut state = self.state.write();
        state.my_geometric_tolerance_with_datum_reference = the_gtwdr.clone();
        state.my_geometric_tolerance_with_modifiers = the_gtwm.clone();
        state.my_tolerance_type = the_type;
        Ok(())
    }
    /// `init`.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init_geometric_tolerance_target(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &StepDimTolGeometricToleranceTarget,
        the_gtwdr: &Handle<StepDimTolGeometricToleranceWithDatumReference>,
        the_gtwm: &Handle<StepDimTolGeometricToleranceWithModifiers>,
        the_type: StepDimTolGeometricToleranceType,
    ) {
        self.set_name(the_name);
        self.set_description(the_description);
        self.set_magnitude(the_magnitude);
        self.set_toleranced_shape_aspect_geometric_tolerance_target(the_toleranced_shape_aspect);
        let mut state = self.state.write();
        state.my_geometric_tolerance_with_datum_reference = the_gtwdr.clone();
        state.my_geometric_tolerance_with_modifiers = the_gtwm.clone();
        state.my_tolerance_type = the_type;
    }
}

step_entity! {
    /// Geo tol and geo tol wth dat ref and mod geo tol and pos tol.
    StepDimTolGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol, StepDimTolGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTolState, "StepDimTolGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol", StepDimTolGeometricTolerance, {
        my_geometric_tolerance_with_datum_reference: Handle<StepDimTolGeometricToleranceWithDatumReference>,
        my_modified_geometric_tolerance: Handle<StepDimTolModifiedGeometricTolerance>,
        my_position_tolerance: Handle<StepDimTolPositionTolerance>,
    }
}
impl StepDimTolGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol {
    step_methods! {
        set set_geometric_tolerance_with_datum_reference = my_geometric_tolerance_with_datum_reference: Handle<StepDimTolGeometricToleranceWithDatumReference>;
        get get_geometric_tolerance_with_datum_reference = my_geometric_tolerance_with_datum_reference: Handle<StepDimTolGeometricToleranceWithDatumReference>;
        set set_modified_geometric_tolerance = my_modified_geometric_tolerance: Handle<StepDimTolModifiedGeometricTolerance>;
        get get_modified_geometric_tolerance = my_modified_geometric_tolerance: Handle<StepDimTolModifiedGeometricTolerance>;
        set set_position_tolerance = my_position_tolerance: Handle<StepDimTolPositionTolerance>;
        get get_position_tolerance = my_position_tolerance: Handle<StepDimTolPositionTolerance>;
    }
    /// `init`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_magnitude: &Handle<StandardTransient>,
        a_toleranced_shape_aspect: &Handle<StepReprShapeAspect>,
        a_gtwdr: &Handle<StepDimTolGeometricToleranceWithDatumReference>,
        a_mgt: &Handle<StepDimTolModifiedGeometricTolerance>,
    ) -> Result<(), Failure> {
        self.set_name(a_name);
        self.set_description(a_description);
        self.set_magnitude(a_magnitude);
        self.set_toleranced_shape_aspect(a_toleranced_shape_aspect)?;
        let mut state = self.state.write();
        state.my_geometric_tolerance_with_datum_reference = a_gtwdr.clone();
        state.my_modified_geometric_tolerance = a_mgt.clone();
        Ok(())
    }
    /// `init`.
    pub fn init_geometric_tolerance_target(
        &self,
        a_name: &Handle<HAsciiString>,
        a_description: &Handle<HAsciiString>,
        a_magnitude: &Handle<StandardTransient>,
        a_toleranced_shape_aspect: &StepDimTolGeometricToleranceTarget,
        a_gtwdr: &Handle<StepDimTolGeometricToleranceWithDatumReference>,
        a_mgt: &Handle<StepDimTolModifiedGeometricTolerance>,
    ) {
        self.set_name(a_name);
        self.set_description(a_description);
        self.set_magnitude(a_magnitude);
        self.set_toleranced_shape_aspect_geometric_tolerance_target(a_toleranced_shape_aspect);
        let mut state = self.state.write();
        state.my_geometric_tolerance_with_datum_reference = a_gtwdr.clone();
        state.my_modified_geometric_tolerance = a_mgt.clone();
    }
}

step_entity! {
    /// Geo tol and geo tol wth dat ref and uneq dis geo tol.
    StepDimTolGeoTolAndGeoTolWthDatRefAndUneqDisGeoTol, StepDimTolGeoTolAndGeoTolWthDatRefAndUneqDisGeoTolState, "StepDimTolGeoTolAndGeoTolWthDatRefAndUneqDisGeoTol", StepDimTolGeoTolAndGeoTolWthDatRef, {
        my_unequally_disposed_geometric_tolerance: Handle<StepDimTolUnequallyDisposedGeometricTolerance>,
    }
}
impl StepDimTolGeoTolAndGeoTolWthDatRefAndUneqDisGeoTol {
    step_methods! {
        set set_unequally_disposed_geometric_tolerance = my_unequally_disposed_geometric_tolerance: Handle<StepDimTolUnequallyDisposedGeometricTolerance>;
        get get_unequally_disposed_geometric_tolerance = my_unequally_disposed_geometric_tolerance: Handle<StepDimTolUnequallyDisposedGeometricTolerance>;
    }
    /// `init`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &Handle<StepReprShapeAspect>,
        the_gtwdr: &Handle<StepDimTolGeometricToleranceWithDatumReference>,
        the_type: StepDimTolGeometricToleranceType,
        the_udgt: &Handle<StepDimTolUnequallyDisposedGeometricTolerance>,
    ) -> Result<(), Failure> {
        self.parent.init(
            the_name,
            the_description,
            the_magnitude,
            the_toleranced_shape_aspect,
            the_gtwdr,
            the_type,
        )?;
        self.state.write().my_unequally_disposed_geometric_tolerance = the_udgt.clone();
        Ok(())
    }
    /// `init`.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init_geometric_tolerance_target(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &StepDimTolGeometricToleranceTarget,
        the_gtwdr: &Handle<StepDimTolGeometricToleranceWithDatumReference>,
        the_type: StepDimTolGeometricToleranceType,
        the_udgt: &Handle<StepDimTolUnequallyDisposedGeometricTolerance>,
    ) {
        self.parent.init_geometric_tolerance_target(
            the_name,
            the_description,
            the_magnitude,
            the_toleranced_shape_aspect,
            the_gtwdr,
            the_type,
        );
        self.state.write().my_unequally_disposed_geometric_tolerance = the_udgt.clone();
    }
}

step_entity! {
    /// Geo tol and geo tol wth max tol.
    StepDimTolGeoTolAndGeoTolWthMaxTol, StepDimTolGeoTolAndGeoTolWthMaxTolState, "StepDimTolGeoTolAndGeoTolWthMaxTol", StepDimTolGeoTolAndGeoTolWthMod, {
        my_max_tol: Handle<StepBasicLengthMeasureWithUnit>,
    }
}
impl StepDimTolGeoTolAndGeoTolWthMaxTol {
    step_methods! {
        set set_max_tolerance = my_max_tol: Handle<StepBasicLengthMeasureWithUnit>;
        get get_max_tolerance = my_max_tol: Handle<StepBasicLengthMeasureWithUnit>;
    }
    /// `init`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &Handle<StepReprShapeAspect>,
        the_gtwm: &Handle<StepDimTolGeometricToleranceWithModifiers>,
        the_max_tol: &Handle<StepBasicLengthMeasureWithUnit>,
        the_type: StepDimTolGeometricToleranceType,
    ) -> Result<(), Failure> {
        self.parent.init(
            the_name,
            the_description,
            the_magnitude,
            the_toleranced_shape_aspect,
            the_gtwm,
            the_type,
        )?;
        self.state.write().my_max_tol = the_max_tol.clone();
        Ok(())
    }
    /// `init`.
    #[allow(clippy::too_many_arguments, reason = "the member takes every field")]
    pub fn init_geometric_tolerance_target(
        &self,
        the_name: &Handle<HAsciiString>,
        the_description: &Handle<HAsciiString>,
        the_magnitude: &Handle<StandardTransient>,
        the_toleranced_shape_aspect: &StepDimTolGeometricToleranceTarget,
        the_gtwm: &Handle<StepDimTolGeometricToleranceWithModifiers>,
        the_max_tol: &Handle<StepBasicLengthMeasureWithUnit>,
        the_type: StepDimTolGeometricToleranceType,
    ) {
        self.parent.init_geometric_tolerance_target(
            the_name,
            the_description,
            the_magnitude,
            the_toleranced_shape_aspect,
            the_gtwm,
            the_type,
        );
        self.state.write().my_max_tol = the_max_tol.clone();
    }
}

step_entity! {
    /// Geo tol and geo tol wth mod.
    StepDimTolGeoTolAndGeoTolWthMod, StepDimTolGeoTolAndGeoTolWthModState, "StepDimTolGeoTolAndGeoTolWthMod", StepDimTolGeometricTolerance, {
        my_geometric_tolerance_with_modifiers: Handle<StepDimTolGeometricToleranceWithModifiers>,
        my_tolerance_type: StepDimTolGeometricToleranceType,
    }
}
impl StepDimTolGeoTolAndGeoTolWthMod {
    step_methods! {
        set set_geometric_tolerance_with_modifiers = my_geometric_tolerance_with_modifiers: Handle<StepDimTolGeometricToleranceWithModifiers>;
        get get_geometric_tolerance_with_modifiers = my_geometric_tolerance_with_modifiers: Handle<StepDimTolGeometricToleranceWithModifiers>;
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
        the_gtwm: &Handle<StepDimTolGeometricToleranceWithModifiers>,
        the_type: StepDimTolGeometricToleranceType,
    ) -> Result<(), Failure> {
        self.set_name(the_name);
        self.set_description(the_description);
        self.set_magnitude(the_magnitude);
        self.set_toleranced_shape_aspect(the_toleranced_shape_aspect)?;
        let mut state = self.state.write();
        state.my_geometric_tolerance_with_modifiers = the_gtwm.clone();
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
        the_gtwm: &Handle<StepDimTolGeometricToleranceWithModifiers>,
        the_type: StepDimTolGeometricToleranceType,
    ) {
        self.set_name(the_name);
        self.set_description(the_description);
        self.set_magnitude(the_magnitude);
        self.set_toleranced_shape_aspect_geometric_tolerance_target(the_toleranced_shape_aspect);
        let mut state = self.state.write();
        state.my_geometric_tolerance_with_modifiers = the_gtwm.clone();
        state.my_tolerance_type = the_type;
    }
}
