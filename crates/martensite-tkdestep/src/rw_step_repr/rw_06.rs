#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepReprRwFunctionallyDefinedTransformation,
    "RwStepReprRwFunctionallyDefinedTransformation"
);
impl RwStepReprRwFunctionallyDefinedTransformation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprFunctionallyDefinedTransformation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "functionally_defined_transformation")? {
            return Ok(());
        }
        rw_block_0126!(a_description, a_name, ach, data, num);
        ent.init(&a_name, &a_description);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprFunctionallyDefinedTransformation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.description())?;
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwGeometricAlignment,
    "RwStepReprRwGeometricAlignment"
);
impl RwStepReprRwGeometricAlignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprGeometricAlignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "geometric_alignment")? {
            return Ok(());
        }
        rw_block_0045!(
            a_shape_aspect_description,
            a_shape_aspect_name,
            a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
            ach,
            data,
            ent,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprGeometricAlignment>,
    ) -> Result<(), Failure> {
        rw_block_0166!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprGeometricAlignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwGlobalUncertaintyAssignedContext,
    "RwStepReprRwGlobalUncertaintyAssignedContext"
);
impl RwStepReprRwGlobalUncertaintyAssignedContext {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprGlobalUncertaintyAssignedContext>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "global_uncertainty_assigned_context")? {
            return Ok(());
        }
        rw_block_0150!(a_context_identifier, a_context_type, ach, data, num);
        rw_block_0192!(a_uncertainty, anent3, nsub3);
        if data.read_sub_list(num, 3, "uncertainty", ach, &mut nsub3, false, 0, 0)? {
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
        ent.init(&a_context_identifier, &a_context_type, &a_uncertainty);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprGlobalUncertaintyAssignedContext>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.context_identifier())?;
        sw.send_entity(&ent.context_type())?;
        sw.open_sub()?;
        for i3 in 1..=ent.nb_uncertainty() {
            sw.send_entity(&ent.uncertainty_value(i3)?)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprGlobalUncertaintyAssignedContext>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_uncertainty();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.uncertainty_value(is1)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwGlobalUnitAssignedContext,
    "RwStepReprRwGlobalUnitAssignedContext"
);
impl RwStepReprRwGlobalUnitAssignedContext {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprGlobalUnitAssignedContext>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "global_unit_assigned_context")? {
            return Ok(());
        }
        rw_block_0150!(a_context_identifier, a_context_type, ach, data, num);
        let mut a_units: Handle<HArray1<Handle<StepBasicNamedUnit>>> = Default::default();
        let mut anent3: Handle<StepBasicNamedUnit> = Default::default();
        let mut nsub3: i32 = Default::default();
        if data.read_sub_list(num, 3, "units", ach, &mut nsub3, false, 0, 0)? {
            let nb3: i32 = data.nb_params(nsub3)?;
            a_units = step_new_array1::<Handle<StepBasicNamedUnit>>(1, nb3)?;
            for i3 in 1..=nb3 {
                if data.read_entity(
                    nsub3,
                    i3,
                    "unit",
                    ach,
                    StepBasicNamedUnit::static_type(),
                    &mut anent3,
                )? {
                    step_array1_set(&a_units, i3, anent3.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_context_identifier, &a_context_type, &a_units);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprGlobalUnitAssignedContext>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.context_identifier())?;
        sw.send_entity(&ent.context_type())?;
        sw.open_sub()?;
        for i3 in 1..=ent.nb_units() {
            sw.send_entity(&ent.units_value(i3)?)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprGlobalUnitAssignedContext>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_units();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.units_value(is1)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwIntegerRepresentationItem,
    "RwStepReprRwIntegerRepresentationItem"
);
impl RwStepReprRwIntegerRepresentationItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprIntegerRepresentationItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "integer_representation_item")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_value: i32 = Default::default();
        data.read_integer(num, 2, "value", ach, &mut a_value)?;
        ent.init(&a_name, a_value);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprIntegerRepresentationItem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_integer(ent.value())?;
        Ok(())
    }
}
