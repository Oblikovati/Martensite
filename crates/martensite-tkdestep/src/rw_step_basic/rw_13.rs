#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepBasicRwGroupAssignment, "RwStepBasicRwGroupAssignment");
impl RwStepBasicRwGroupAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicGroupAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "group_assignment")? {
            return Ok(());
        }
        let mut a_assigned_group: Handle<StepBasicGroup> = Default::default();
        data.read_entity(
            num,
            1,
            "assigned_group",
            ach,
            StepBasicGroup::static_type(),
            &mut a_assigned_group,
        )?;
        ent.init(&a_assigned_group);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicGroupAssignment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.assigned_group())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicGroupAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.assigned_group().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwGroupRelationship,
    "RwStepBasicRwGroupRelationship"
);
impl RwStepBasicRwGroupRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicGroupRelationship>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "group_relationship")? {
            return Ok(());
        }
        rw_block_0102!(a_description, a_name, ach, data, has_description, num);
        let mut a_relating_group: Handle<StepBasicGroup> = Default::default();
        data.read_entity(
            num,
            3,
            "relating_group",
            ach,
            StepBasicGroup::static_type(),
            &mut a_relating_group,
        )?;
        let mut a_related_group: Handle<StepBasicGroup> = Default::default();
        data.read_entity(
            num,
            4,
            "related_group",
            ach,
            StepBasicGroup::static_type(),
            &mut a_related_group,
        )?;
        ent.init(
            &a_name,
            has_description,
            &a_description,
            &a_relating_group,
            &a_related_group,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicGroupRelationship>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        if ent.has_description() {
            sw.send_entity(&ent.description())?;
        } else {
            sw.send_undef()?;
        }
        sw.send_entity(&ent.relating_group())?;
        sw.send_entity(&ent.related_group())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicGroupRelationship>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.relating_group().cast());
        iter.add_item(&ent.related_group().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwIdentificationAssignment,
    "RwStepBasicRwIdentificationAssignment"
);
impl RwStepBasicRwIdentificationAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicIdentificationAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "identification_assignment")? {
            return Ok(());
        }
        let mut a_assigned_id: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "assigned_id", ach, &mut a_assigned_id)?;
        let mut a_role: Handle<StepBasicIdentificationRole> = Default::default();
        data.read_entity(
            num,
            2,
            "role",
            ach,
            StepBasicIdentificationRole::static_type(),
            &mut a_role,
        )?;
        ent.init(&a_assigned_id, &a_role);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicIdentificationAssignment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.assigned_id())?;
        sw.send_entity(&ent.role())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicIdentificationAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.role().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwIdentificationRole,
    "RwStepBasicRwIdentificationRole"
);
impl RwStepBasicRwIdentificationRole {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicIdentificationRole>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "identification_role")? {
            return Ok(());
        }
        rw_block_0082!(a_description, a_name, ach, data, ent, has_description, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicIdentificationRole>,
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
        _unused_0: &Handle<StepBasicIdentificationRole>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwLengthMeasureWithUnit,
    "RwStepBasicRwLengthMeasureWithUnit"
);
impl RwStepBasicRwLengthMeasureWithUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicLengthMeasureWithUnit>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "length_measure_with_unit")? {
            return Ok(());
        }
        rw_block_0111!(a_unit_component, ach, data, ent, mvc, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicLengthMeasureWithUnit>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.value_component_member())?;
        sw.send_entity(&ent.unit_component().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicLengthMeasureWithUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.unit_component().value());
        Ok(())
    }
}
