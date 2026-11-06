#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepFEARwFeaMoistureAbsorption,
    "RwStepFEARwFeaMoistureAbsorption"
);
impl RwStepFEARwFeaMoistureAbsorption {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaMoistureAbsorption>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "fea_moisture_absorption")? {
            return Ok(());
        }
        rw_block_b_0110!(
            a_fea_constants,
            a_representation_item_name,
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
        ent: &Handle<StepFEAFeaMoistureAbsorption>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_entity(&ent.fea_constants().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFeaMoistureAbsorption>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.fea_constants().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwFeaParametricPoint,
    "RwStepFEARwFeaParametricPoint"
);
impl RwStepFEARwFeaParametricPoint {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaParametricPoint>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "fea_parametric_point")? {
            return Ok(());
        }
        let mut a_representation_item_name: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            1,
            "representation_item.name",
            ach,
            &mut a_representation_item_name,
        )?;
        let mut a_coordinates: Handle<HArray1<f64>> = Default::default();
        let mut sub2: i32 = 0;
        if data.read_sub_list(num, 2, "coordinates", ach, &mut sub2, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub2)?;
            a_coordinates = step_new_array1::<f64>(1, nb0)?;
            let num2: i32 = sub2;
            for i0 in 1..=nb0 {
                let mut an_it0: f64 = Default::default();
                data.read_real(num2, i0, "real", ach, &mut an_it0)?;
                step_array1_set(&a_coordinates, i0, an_it0, "ReadStep")?;
            }
        }
        ent.init(&a_representation_item_name, &a_coordinates);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAFeaParametricPoint>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.parent.name())?;
        sw.open_sub()?;
        for i1 in 1..=step_live(&ent.coordinates(), "WriteStep")?
            .array1()
            .length()
        {
            let var0: f64 = step_array1_item(&ent.coordinates(), i1, "WriteStep")?;
            sw.send_real(var0)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepFEAFeaParametricPoint>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwFeaRepresentationItem,
    "RwStepFEARwFeaRepresentationItem"
);
impl RwStepFEARwFeaRepresentationItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaRepresentationItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "fea_representation_item")? {
            return Ok(());
        }
        let mut a_representation_item_name: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            1,
            "representation_item.name",
            ach,
            &mut a_representation_item_name,
        )?;
        ent.init(&a_representation_item_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAFeaRepresentationItem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.name())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepFEAFeaRepresentationItem>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwFeaSecantCoefficientOfLinearThermalExpansion,
    "RwStepFEARwFeaSecantCoefficientOfLinearThermalExpansion"
);
impl RwStepFEARwFeaSecantCoefficientOfLinearThermalExpansion {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaSecantCoefficientOfLinearThermalExpansion>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            3,
            ach,
            "fea_secant_coefficient_of_linear_thermal_expansion",
        )? {
            return Ok(());
        }
        rw_block_b_0152!(a_fea_constants, a_representation_item_name, ach, data, num);
        let mut a_reference_temperature: f64 = Default::default();
        data.read_real(
            num,
            3,
            "reference_temperature",
            ach,
            &mut a_reference_temperature,
        )?;
        ent.init(
            &a_representation_item_name,
            &a_fea_constants,
            a_reference_temperature,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAFeaSecantCoefficientOfLinearThermalExpansion>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_entity(&ent.fea_constants().value())?;
        sw.send_real(ent.reference_temperature())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFeaSecantCoefficientOfLinearThermalExpansion>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.fea_constants().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwFeaShellBendingStiffness,
    "RwStepFEARwFeaShellBendingStiffness"
);
impl RwStepFEARwFeaShellBendingStiffness {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaShellBendingStiffness>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "fea_shell_bending_stiffness")? {
            return Ok(());
        }
        rw_block_b_0111!(
            a_fea_constants,
            a_representation_item_name,
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
        ent: &Handle<StepFEAFeaShellBendingStiffness>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_entity(&ent.fea_constants().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFeaShellBendingStiffness>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.fea_constants().value());
        Ok(())
    }
}
