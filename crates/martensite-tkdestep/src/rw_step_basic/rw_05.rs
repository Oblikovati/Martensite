#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepBasicRwCharacterizedObject,
    "RwStepBasicRwCharacterizedObject"
);
impl RwStepBasicRwCharacterizedObject {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicCharacterizedObject>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "characterized_object")? {
            return Ok(());
        }
        rw_block_0082!(a_description, a_name, ach, data, ent, has_description, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicCharacterizedObject>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        if ent.has_description() {
            sw.send_entity(&ent.description())?;
        } else {
            sw.send_undef()?;
        }
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepBasicCharacterizedObject>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwContract, "RwStepBasicRwContract");
impl RwStepBasicRwContract {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicContract>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "contract")? {
            return Ok(());
        }
        rw_block_0147!(a_name, a_purpose, ach, data, num);
        let mut a_kind: Handle<StepBasicContractType> = Default::default();
        data.read_entity(
            num,
            3,
            "kind",
            ach,
            StepBasicContractType::static_type(),
            &mut a_kind,
        )?;
        ent.init(&a_name, &a_purpose, &a_kind);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicContract>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.purpose())?;
        sw.send_entity(&ent.kind())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicContract>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.kind().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwContractAssignment,
    "RwStepBasicRwContractAssignment"
);
impl RwStepBasicRwContractAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicContractAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "contract_assignment")? {
            return Ok(());
        }
        let mut a_assigned_contract: Handle<StepBasicContract> = Default::default();
        data.read_entity(
            num,
            1,
            "assigned_contract",
            ach,
            StepBasicContract::static_type(),
            &mut a_assigned_contract,
        )?;
        ent.init(&a_assigned_contract);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicContractAssignment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.assigned_contract())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicContractAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.assigned_contract().cast());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwContractType, "RwStepBasicRwContractType");
impl RwStepBasicRwContractType {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicContractType>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "contract_type")? {
            return Ok(());
        }
        let mut a_description: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "description", ach, &mut a_description)?;
        ent.init(&a_description);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicContractType>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.description())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepBasicContractType>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwConversionBasedUnit,
    "RwStepBasicRwConversionBasedUnit"
);
impl RwStepBasicRwConversionBasedUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicConversionBasedUnit>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "conversion_based_unit")? {
            return Ok(());
        }
        let mut a_dimensions: Handle<StepBasicDimensionalExponents> = Default::default();
        data.read_entity(
            num,
            1,
            "dimensions",
            ach,
            StepBasicDimensionalExponents::static_type(),
            &mut a_dimensions,
        )?;
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 2, "name", ach, &mut a_name)?;
        let mut a_conversion_factor: Handle<StandardTransient> = Default::default();
        data.read_entity(
            num,
            3,
            "conversion_factor",
            ach,
            StandardTransient::static_type(),
            &mut a_conversion_factor,
        )?;
        ent.init(&a_dimensions, &a_name, &a_conversion_factor);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicConversionBasedUnit>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.dimensions())?;
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.conversion_factor())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicConversionBasedUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.dimensions().cast());
        iter.get_one_item(&ent.conversion_factor());
        Ok(())
    }
}
