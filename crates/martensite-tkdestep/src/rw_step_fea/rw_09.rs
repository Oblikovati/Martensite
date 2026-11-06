#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepFEARwFeaShellMembraneBendingCouplingStiffness,
    "RwStepFEARwFeaShellMembraneBendingCouplingStiffness"
);
impl RwStepFEARwFeaShellMembraneBendingCouplingStiffness {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaShellMembraneBendingCouplingStiffness>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "fea_shell_membrane_bending_coupling_stiffness")? {
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
        ent: &Handle<StepFEAFeaShellMembraneBendingCouplingStiffness>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_entity(&ent.fea_constants().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFeaShellMembraneBendingCouplingStiffness>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.fea_constants().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwFeaShellMembraneStiffness,
    "RwStepFEARwFeaShellMembraneStiffness"
);
impl RwStepFEARwFeaShellMembraneStiffness {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaShellMembraneStiffness>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "fea_shell_membrane_stiffness")? {
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
        ent: &Handle<StepFEAFeaShellMembraneStiffness>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_entity(&ent.fea_constants().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFeaShellMembraneStiffness>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.fea_constants().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwFeaShellShearStiffness,
    "RwStepFEARwFeaShellShearStiffness"
);
impl RwStepFEARwFeaShellShearStiffness {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaShellShearStiffness>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "fea_shell_shear_stiffness")? {
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
        let mut a_fea_constants: StepFEASymmetricTensor22d = Default::default();
        data.read_entity_select(num, 2, "fea_constants", ach, &mut a_fea_constants)?;
        ent.init(&a_representation_item_name, &a_fea_constants);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAFeaShellShearStiffness>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_entity(&ent.fea_constants().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFeaShellShearStiffness>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.fea_constants().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwFeaSurfaceSectionGeometricRelationship,
    "RwStepFEARwFeaSurfaceSectionGeometricRelationship"
);
impl RwStepFEARwFeaSurfaceSectionGeometricRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaSurfaceSectionGeometricRelationship>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "fea_surface_section_geometric_relationship")? {
            return Ok(());
        }
        let mut a_section_ref: Handle<StepElementSurfaceSection> = Default::default();
        data.read_entity(
            num,
            1,
            "section_ref",
            ach,
            StepElementSurfaceSection::static_type(),
            &mut a_section_ref,
        )?;
        rw_block_b_0189!(a_item, a_section_ref, ach, data, ent, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAFeaSurfaceSectionGeometricRelationship>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.section_ref())?;
        sw.send_entity(&ent.item())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFeaSurfaceSectionGeometricRelationship>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.section_ref().cast());
        iter.add_item(&ent.item().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwFeaTangentialCoefficientOfLinearThermalExpansion,
    "RwStepFEARwFeaTangentialCoefficientOfLinearThermalExpansion"
);
impl RwStepFEARwFeaTangentialCoefficientOfLinearThermalExpansion {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaTangentialCoefficientOfLinearThermalExpansion>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            2,
            ach,
            "fea_tangential_coefficient_of_linear_thermal_expansion",
        )? {
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
        ent: &Handle<StepFEAFeaTangentialCoefficientOfLinearThermalExpansion>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_entity(&ent.fea_constants().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFeaTangentialCoefficientOfLinearThermalExpansion>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.fea_constants().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwFreedomAndCoefficient,
    "RwStepFEARwFreedomAndCoefficient"
);
impl RwStepFEARwFreedomAndCoefficient {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFreedomAndCoefficient>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "freedom_and_coefficient")? {
            return Ok(());
        }
        let mut a_freedom: StepFEADegreeOfFreedom = Default::default();
        data.read_entity_select(num, 1, "freedom", ach, &mut a_freedom)?;
        let mut a_a: StepElementMeasureOrUnspecifiedValue = Default::default();
        data.read_entity_select(num, 2, "a", ach, &mut a_a)?;
        ent.init(&a_freedom, &a_a);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAFreedomAndCoefficient>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.freedom().value())?;
        sw.send_entity(&ent.a().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFreedomAndCoefficient>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.freedom().value());
        iter.add_item(&ent.a().value());
        Ok(())
    }
}
