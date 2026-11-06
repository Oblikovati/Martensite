#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepAP214RwExternallyDefinedGeneralProperty,
    "RwStepAP214RwExternallyDefinedGeneralProperty"
);
impl RwStepAP214RwExternallyDefinedGeneralProperty {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214ExternallyDefinedGeneralProperty>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "externally_defined_general_property")? {
            return Ok(());
        }
        let mut a_general_property_id: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            1,
            "general_property.id",
            ach,
            &mut a_general_property_id,
        )?;
        let mut a_general_property_name: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            2,
            "general_property.name",
            ach,
            &mut a_general_property_name,
        )?;
        let mut a_general_property_description: Handle<HAsciiString> = Default::default();
        let mut has_general_property_description: bool = true;
        if data.is_param_defined(num, 3)? {
            data.read_string(
                num,
                3,
                "general_property.description",
                ach,
                &mut a_general_property_description,
            )?;
        } else {
            has_general_property_description = false;
        }
        let mut a_externally_defined_item_item_id: StepBasicSourceItem = Default::default();
        data.read_entity_select(
            num,
            4,
            "externally_defined_item.item_id",
            ach,
            &mut a_externally_defined_item_item_id,
        )?;
        let mut a_externally_defined_item_source: Handle<StepBasicExternalSource> =
            Default::default();
        data.read_entity(
            num,
            5,
            "externally_defined_item.source",
            ach,
            StepBasicExternalSource::static_type(),
            &mut a_externally_defined_item_source,
        )?;
        ent.init(
            &a_general_property_id,
            &a_general_property_name,
            has_general_property_description,
            &a_general_property_description,
            &a_externally_defined_item_item_id,
            &a_externally_defined_item_source,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214ExternallyDefinedGeneralProperty>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.id())?;
        rw_block_b_0214!(ent, sw);
        rw_block_b_0213!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214ExternallyDefinedGeneralProperty>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0211!(ent, iter);
        Ok(())
    }
}

step_rw_class!(RwStepAP214RwRepItemGroup, "RwStepAP214RwRepItemGroup");
impl RwStepAP214RwRepItemGroup {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214RepItemGroup>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "rep_item_group")? {
            return Ok(());
        }
        rw_block_b_0106!(
            a_group_description,
            a_group_name,
            ach,
            data,
            has_group_description,
            num
        );
        let mut a_representation_item_name: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            3,
            "representation_item.name",
            ach,
            &mut a_representation_item_name,
        )?;
        ent.init(
            &a_group_name,
            has_group_description,
            &a_group_description,
            &a_representation_item_name,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214RepItemGroup>,
    ) -> Result<(), Failure> {
        rw_block_b_0214!(ent, sw);
        sw.send_entity(&step_live(&ent.representation_item(), "WriteStep")?.name())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepAP214RepItemGroup>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
