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
    /// Evaluated degenerate pcurve.
    StepGeomEvaluatedDegeneratePcurve, StepGeomEvaluatedDegeneratePcurveState, "StepGeomEvaluatedDegeneratePcurve", StepGeomDegeneratePcurve, {
        equivalent_point: Handle<StepGeomCartesianPoint>,
    }
}
impl StepGeomEvaluatedDegeneratePcurve {
    step_methods! {
        set set_equivalent_point = equivalent_point: Handle<StepGeomCartesianPoint>;
        get equivalent_point = equivalent_point: Handle<StepGeomCartesianPoint>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_basis_surface: &Handle<StepGeomSurface>,
        a_reference_to_curve: &Handle<StepReprDefinitionalRepresentation>,
        a_equivalent_point: &Handle<StepGeomCartesianPoint>,
    ) {
        self.state.write().equivalent_point = a_equivalent_point.clone();
        self.parent
            .init(a_name, a_basis_surface, a_reference_to_curve);
    }
}

step_entity! {
    /// Geom rep context and glob unit ass ctx and glob uncertainty ass ctx.
    StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx, StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtxState, "StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx", StepReprRepresentationContext, {
        geometric_representation_context: Handle<StepGeomGeometricRepresentationContext>,
        global_unit_assigned_context: Handle<StepReprGlobalUnitAssignedContext>,
        global_uncertainty_assigned_context: Handle<StepReprGlobalUncertaintyAssignedContext>,
    }
}
impl StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx {
    step_methods! {
        set set_geometric_representation_context = geometric_representation_context: Handle<StepGeomGeometricRepresentationContext>;
        get geometric_representation_context = geometric_representation_context: Handle<StepGeomGeometricRepresentationContext>;
        set set_global_unit_assigned_context = global_unit_assigned_context: Handle<StepReprGlobalUnitAssignedContext>;
        get global_unit_assigned_context = global_unit_assigned_context: Handle<StepReprGlobalUnitAssignedContext>;
        set set_global_uncertainty_assigned_context = global_uncertainty_assigned_context: Handle<StepReprGlobalUncertaintyAssignedContext>;
        get global_uncertainty_assigned_context = global_uncertainty_assigned_context: Handle<StepReprGlobalUncertaintyAssignedContext>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_context_identifier: &Handle<HAsciiString>,
        a_context_type: &Handle<HAsciiString>,
        a_geometric_representation_context: &Handle<StepGeomGeometricRepresentationContext>,
        a_global_unit_assigned_context: &Handle<StepReprGlobalUnitAssignedContext>,
        a_global_uncertainty_assigned_ctx: &Handle<StepReprGlobalUncertaintyAssignedContext>,
    ) {
        let mut state = self.state.write();
        state.geometric_representation_context = a_geometric_representation_context.clone();
        state.global_unit_assigned_context = a_global_unit_assigned_context.clone();
        state.global_uncertainty_assigned_context = a_global_uncertainty_assigned_ctx.clone();
        drop(state);
        self.parent.init(a_context_identifier, a_context_type);
    }
    /// `init`.
    pub fn init_int(
        &self,
        a_context_identifier: &Handle<HAsciiString>,
        a_context_type: &Handle<HAsciiString>,
        a_coordinate_space_dimension: i32,
        a_units: &Handle<HArray1<Handle<StepBasicNamedUnit>>>,
        an_uncertainty: &Handle<HArray1<Handle<StepBasicUncertaintyMeasureWithUnit>>>,
    ) {
        self.parent.init(a_context_identifier, a_context_type);
        self.state.write().geometric_representation_context =
            Handle::new(StepGeomGeometricRepresentationContext::new());
        let field_geometric_representation_context =
            self.state.read().geometric_representation_context.clone();
        field_geometric_representation_context.init(
            a_context_identifier,
            a_context_type,
            a_coordinate_space_dimension,
        );
        self.state.write().global_unit_assigned_context =
            Handle::new(StepReprGlobalUnitAssignedContext::new());
        let field_global_unit_assigned_context =
            self.state.read().global_unit_assigned_context.clone();
        field_global_unit_assigned_context.init(a_context_identifier, a_context_type, a_units);
        self.state.write().global_uncertainty_assigned_context =
            Handle::new(StepReprGlobalUncertaintyAssignedContext::new());
        let field_global_uncertainty_assigned_context = self
            .state
            .read()
            .global_uncertainty_assigned_context
            .clone();
        field_global_uncertainty_assigned_context.init(
            a_context_identifier,
            a_context_type,
            an_uncertainty,
        );
    }
    /// `set_coordinate_space_dimension`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_coordinate_space_dimension(
        &self,
        a_coordinate_space_dimension: i32,
    ) -> Result<(), Failure> {
        let field_geometric_representation_context =
            self.state.read().geometric_representation_context.clone();
        step_live(&field_geometric_representation_context, "StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx::SetCoordinateSpaceDimension")?.set_coordinate_space_dimension(a_coordinate_space_dimension);
        Ok(())
    }
    /// `coordinate_space_dimension`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn coordinate_space_dimension(&self) -> Result<i32, Failure> {
        let field_geometric_representation_context =
            self.state.read().geometric_representation_context.clone();
        Ok(step_live(&field_geometric_representation_context, "StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx::CoordinateSpaceDimension")?.coordinate_space_dimension())
    }
    /// `set_units`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_units(
        &self,
        a_units: &Handle<HArray1<Handle<StepBasicNamedUnit>>>,
    ) -> Result<(), Failure> {
        let field_global_unit_assigned_context =
            self.state.read().global_unit_assigned_context.clone();
        step_live(
            &field_global_unit_assigned_context,
            "StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx::SetUnits",
        )?
        .set_units(a_units);
        Ok(())
    }
    /// `Units`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn units(&self) -> Result<Handle<HArray1<Handle<StepBasicNamedUnit>>>, Failure> {
        let field_global_unit_assigned_context =
            self.state.read().global_unit_assigned_context.clone();
        Ok(step_live(
            &field_global_unit_assigned_context,
            "StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx::Units",
        )?
        .units())
    }
    /// `units_value`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn units_value(&self, num: i32) -> Result<Handle<StepBasicNamedUnit>, Failure> {
        let field_global_unit_assigned_context =
            self.state.read().global_unit_assigned_context.clone();
        step_live(
            &field_global_unit_assigned_context,
            "StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx::UnitsValue",
        )?
        .units_value(num)
    }
    /// `nb_units`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn nb_units(&self) -> Result<i32, Failure> {
        let field_global_unit_assigned_context =
            self.state.read().global_unit_assigned_context.clone();
        Ok(step_live(
            &field_global_unit_assigned_context,
            "StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx::NbUnits",
        )?
        .nb_units())
    }
    /// `set_uncertainty`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_uncertainty(
        &self,
        a_uncertainty: &Handle<HArray1<Handle<StepBasicUncertaintyMeasureWithUnit>>>,
    ) -> Result<(), Failure> {
        let field_global_uncertainty_assigned_context = self
            .state
            .read()
            .global_uncertainty_assigned_context
            .clone();
        step_live(
            &field_global_uncertainty_assigned_context,
            "StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx::SetUncertainty",
        )?
        .set_uncertainty(a_uncertainty);
        Ok(())
    }
    /// `uncertainty`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn uncertainty(
        &self,
    ) -> Result<Handle<HArray1<Handle<StepBasicUncertaintyMeasureWithUnit>>>, Failure> {
        let field_global_uncertainty_assigned_context = self
            .state
            .read()
            .global_uncertainty_assigned_context
            .clone();
        Ok(step_live(
            &field_global_uncertainty_assigned_context,
            "StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx::Uncertainty",
        )?
        .uncertainty())
    }
    /// `uncertainty_value`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn uncertainty_value(
        &self,
        num: i32,
    ) -> Result<Handle<StepBasicUncertaintyMeasureWithUnit>, Failure> {
        let field_global_uncertainty_assigned_context = self
            .state
            .read()
            .global_uncertainty_assigned_context
            .clone();
        step_live(
            &field_global_uncertainty_assigned_context,
            "StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx::UncertaintyValue",
        )?
        .uncertainty_value(num)
    }
    /// `nb_uncertainty`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn nb_uncertainty(&self) -> Result<i32, Failure> {
        let field_global_uncertainty_assigned_context = self
            .state
            .read()
            .global_uncertainty_assigned_context
            .clone();
        Ok(step_live(
            &field_global_uncertainty_assigned_context,
            "StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx::NbUncertainty",
        )?
        .nb_uncertainty())
    }
}

step_entity! {
    /// Geometric representation context.
    StepGeomGeometricRepresentationContext, StepGeomGeometricRepresentationContextState, "StepGeomGeometricRepresentationContext", StepReprRepresentationContext, {
        coordinate_space_dimension: i32,
    }
}
impl StepGeomGeometricRepresentationContext {
    step_methods! {
        set_copy set_coordinate_space_dimension = coordinate_space_dimension: i32;
        get coordinate_space_dimension = coordinate_space_dimension: i32;
    }
    /// `init`.
    pub fn init(
        &self,
        a_context_identifier: &Handle<HAsciiString>,
        a_context_type: &Handle<HAsciiString>,
        a_coordinate_space_dimension: i32,
    ) {
        self.state.write().coordinate_space_dimension = a_coordinate_space_dimension;
        self.parent.init(a_context_identifier, a_context_type);
    }
}

step_entity! {
    /// Geometric representation context and global unit assigned context.
    StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext, StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContextState, "StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext", StepReprRepresentationContext, {
        geometric_representation_context: Handle<StepGeomGeometricRepresentationContext>,
        global_unit_assigned_context: Handle<StepReprGlobalUnitAssignedContext>,
    }
}
impl StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext {
    step_methods! {
        set set_geometric_representation_context = geometric_representation_context: Handle<StepGeomGeometricRepresentationContext>;
        get geometric_representation_context = geometric_representation_context: Handle<StepGeomGeometricRepresentationContext>;
        set set_global_unit_assigned_context = global_unit_assigned_context: Handle<StepReprGlobalUnitAssignedContext>;
        get global_unit_assigned_context = global_unit_assigned_context: Handle<StepReprGlobalUnitAssignedContext>;
    }
    /// `init`.
    pub fn init(
        &self,
        a_context_identifier: &Handle<HAsciiString>,
        a_context_type: &Handle<HAsciiString>,
        a_geometric_representation_context: &Handle<StepGeomGeometricRepresentationContext>,
        a_global_unit_assigned_context: &Handle<StepReprGlobalUnitAssignedContext>,
    ) {
        let mut state = self.state.write();
        state.geometric_representation_context = a_geometric_representation_context.clone();
        state.global_unit_assigned_context = a_global_unit_assigned_context.clone();
        drop(state);
        self.parent.init(a_context_identifier, a_context_type);
    }
    /// `init`.
    pub fn init_int(
        &self,
        a_context_identifier: &Handle<HAsciiString>,
        a_context_type: &Handle<HAsciiString>,
        a_coordinate_space_dimension: i32,
        a_units: &Handle<HArray1<Handle<StepBasicNamedUnit>>>,
    ) {
        self.parent.init(a_context_identifier, a_context_type);
        self.state.write().geometric_representation_context =
            Handle::new(StepGeomGeometricRepresentationContext::new());
        let field_geometric_representation_context =
            self.state.read().geometric_representation_context.clone();
        field_geometric_representation_context.init(
            a_context_identifier,
            a_context_type,
            a_coordinate_space_dimension,
        );
        self.state.write().global_unit_assigned_context =
            Handle::new(StepReprGlobalUnitAssignedContext::new());
        let field_global_unit_assigned_context =
            self.state.read().global_unit_assigned_context.clone();
        field_global_unit_assigned_context.init(a_context_identifier, a_context_type, a_units);
    }
    /// `set_coordinate_space_dimension`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_coordinate_space_dimension(
        &self,
        a_coordinate_space_dimension: i32,
    ) -> Result<(), Failure> {
        let field_geometric_representation_context =
            self.state.read().geometric_representation_context.clone();
        step_live(&field_geometric_representation_context, "StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext::SetCoordinateSpaceDimension")?.set_coordinate_space_dimension(a_coordinate_space_dimension);
        Ok(())
    }
    /// `coordinate_space_dimension`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn coordinate_space_dimension(&self) -> Result<i32, Failure> {
        let field_geometric_representation_context =
            self.state.read().geometric_representation_context.clone();
        Ok(step_live(&field_geometric_representation_context, "StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext::CoordinateSpaceDimension")?.coordinate_space_dimension())
    }
    /// `set_units`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn set_units(
        &self,
        a_units: &Handle<HArray1<Handle<StepBasicNamedUnit>>>,
    ) -> Result<(), Failure> {
        let field_global_unit_assigned_context =
            self.state.read().global_unit_assigned_context.clone();
        step_live(
            &field_global_unit_assigned_context,
            "StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext::SetUnits",
        )?
        .set_units(a_units);
        Ok(())
    }
    /// `Units`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn units(&self) -> Result<Handle<HArray1<Handle<StepBasicNamedUnit>>>, Failure> {
        let field_global_unit_assigned_context =
            self.state.read().global_unit_assigned_context.clone();
        Ok(step_live(
            &field_global_unit_assigned_context,
            "StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext::Units",
        )?
        .units())
    }
    /// `units_value`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn units_value(&self, num: i32) -> Result<Handle<StepBasicNamedUnit>, Failure> {
        let field_global_unit_assigned_context =
            self.state.read().global_unit_assigned_context.clone();
        step_live(
            &field_global_unit_assigned_context,
            "StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext::UnitsValue",
        )?
        .units_value(num)
    }
    /// `nb_units`.
    ///
    /// # Errors
    /// The null dereference or array index failure.
    pub fn nb_units(&self) -> Result<i32, Failure> {
        let field_global_unit_assigned_context =
            self.state.read().global_unit_assigned_context.clone();
        Ok(step_live(
            &field_global_unit_assigned_context,
            "StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext::NbUnits",
        )?
        .nb_units())
    }
}
