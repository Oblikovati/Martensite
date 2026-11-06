#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepGeomRwGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx,
    "RwStepGeomRwGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx"
);
impl RwStepGeomRwGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx>,
    ) -> Result<(), Failure> {
        rw_block_0115!(a_coordinate_space_dimension, ach, data, num, num0);
        if !data.check_nb_params(num, 1, ach, "global_uncertainty_assigned_context")? {
            return Ok(());
        }
        rw_block_0192!(a_uncertainty, anent3, nsub3);
        if data.read_sub_list(num, 1, "uncertainty", ach, &mut nsub3, false, 0, 0)? {
            let nb3: i32 = data.nb_params(nsub3)?;
            a_uncertainty = step_new_array1::<Handle<StepBasicUncertaintyMeasureWithUnit>>(1, nb3)?;
            for i3 in 1..=nb3 {
                if data.read_entity(
                    nsub3,
                    i3,
                    "uncertainty_measure_with_unit",
                    ach,
                    StepBasicUncertaintyMeasureWithUnit::static_type(),
                    &mut anent3,
                )? {
                    step_array1_set(&a_uncertainty, i3, anent3.clone(), "ReadStep")?;
                }
            }
        }
        rw_block_0019!(
            a_context_identifier,
            a_context_type,
            a_units,
            ach,
            anent2,
            data,
            i2,
            nb2,
            nsub2,
            num
        );
        ent.init_int(
            &a_context_identifier,
            &a_context_type,
            a_coordinate_space_dimension,
            &a_units,
            &a_uncertainty,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "GEOMETRIC_REPRESENTATION_CONTEXT")?;
        sw.send_integer(ent.coordinate_space_dimension()?)?;
        step_start_entity(sw, "GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT")?;
        sw.open_sub()?;
        for i3 in 1..=ent.nb_uncertainty()? {
            sw.send_entity(&ent.uncertainty_value(i3)?)?;
        }
        sw.close_sub()?;
        rw_block_0075!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomGeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_units()?;
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.units_value(is1)?.cast());
        }
        let nb_elem2: i32 = ent.nb_uncertainty()?;
        for is2 in 1..=nb_elem2 {
            iter.get_one_item(&ent.uncertainty_value(is2)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwGeometricRepresentationContext,
    "RwStepGeomRwGeometricRepresentationContext"
);
impl RwStepGeomRwGeometricRepresentationContext {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomGeometricRepresentationContext>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "geometric_representation_context")? {
            return Ok(());
        }
        rw_block_0150!(a_context_identifier, a_context_type, ach, data, num);
        let mut a_coordinate_space_dimension: i32 = Default::default();
        data.read_integer(
            num,
            3,
            "coordinate_space_dimension",
            ach,
            &mut a_coordinate_space_dimension,
        )?;
        ent.init(
            &a_context_identifier,
            &a_context_type,
            a_coordinate_space_dimension,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomGeometricRepresentationContext>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.context_identifier())?;
        sw.send_entity(&ent.context_type())?;
        sw.send_integer(ent.coordinate_space_dimension())?;
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwGeometricRepresentationContextAndGlobalUnitAssignedContext,
    "RwStepGeomRwGeometricRepresentationContextAndGlobalUnitAssignedContext"
);
impl RwStepGeomRwGeometricRepresentationContextAndGlobalUnitAssignedContext {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext>,
    ) -> Result<(), Failure> {
        rw_block_0156!(a_coordinate_space_dimension, ach, data, num, num0);
        rw_block_0019!(
            a_context_identifier,
            a_context_type,
            a_units,
            ach,
            anent2,
            data,
            i2,
            nb2,
            nsub2,
            num
        );
        ent.init_int(
            &a_context_identifier,
            &a_context_type,
            a_coordinate_space_dimension,
            &a_units,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "GEOMETRIC_REPRESENTATION_CONTEXT")?;
        sw.send_integer(ent.coordinate_space_dimension()?)?;
        rw_block_0075!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepGeomGeometricRepresentationContextAndGlobalUnitAssignedContext>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_units()?;
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.units_value(is1)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwGeometricRepresentationContextAndParametricRepresentationContext,
    "RwStepGeomRwGeometricRepresentationContextAndParametricRepresentationContext"
);
impl RwStepGeomRwGeometricRepresentationContextAndParametricRepresentationContext {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomGeometricRepresentationContextAndParametricRepresentationContext>,
    ) -> Result<(), Failure> {
        rw_block_0115!(a_coordinate_space_dimension, ach, data, num, num0);
        if !data.check_nb_params(num, 0, ach, "parametric_representation_context")? {
            return Ok(());
        }
        rw_block_0088!(a_context_identifier, a_context_type, ach, data, num);
        ent.init_int(
            &a_context_identifier,
            &a_context_type,
            a_coordinate_space_dimension,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomGeometricRepresentationContextAndParametricRepresentationContext>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "GEOMETRIC_REPRESENTATION_CONTEXT")?;
        sw.send_integer(ent.coordinate_space_dimension()?)?;
        step_start_entity(sw, "PARAMETRIC_REPRESENTATION_CONTEXT")?;
        step_start_entity(sw, "REPRESENTATION_CONTEXT")?;
        sw.send_entity(&ent.context_identifier())?;
        sw.send_entity(&ent.context_type())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<
            StepGeomGeometricRepresentationContextAndParametricRepresentationContext,
        >,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepGeomRwGeometricRepresentationItem,
    "RwStepGeomRwGeometricRepresentationItem"
);
impl RwStepGeomRwGeometricRepresentationItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepGeomGeometricRepresentationItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "geometric_representation_item")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        ent.init(&a_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepGeomGeometricRepresentationItem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}
