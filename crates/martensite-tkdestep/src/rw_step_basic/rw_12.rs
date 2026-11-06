#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepBasicRwExternallyDefinedItem,
    "RwStepBasicRwExternallyDefinedItem"
);
impl RwStepBasicRwExternallyDefinedItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicExternallyDefinedItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "externally_defined_item")? {
            return Ok(());
        }
        let mut a_item_id: StepBasicSourceItem = Default::default();
        data.read_entity_select(num, 1, "item_id", ach, &mut a_item_id)?;
        let mut a_source: Handle<StepBasicExternalSource> = Default::default();
        data.read_entity(
            num,
            2,
            "source",
            ach,
            StepBasicExternalSource::static_type(),
            &mut a_source,
        )?;
        ent.init(&a_item_id, &a_source);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicExternallyDefinedItem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.item_id().value())?;
        sw.send_entity(&ent.source())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicExternallyDefinedItem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.item_id().value());
        iter.add_item(&ent.source().cast());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwGeneralProperty, "RwStepBasicRwGeneralProperty");
impl RwStepBasicRwGeneralProperty {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicGeneralProperty>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "general_property")? {
            return Ok(());
        }
        rw_block_0068!(a_description, a_id, a_name, ach, data, has_description, num);
        ent.init(&a_id, &a_name, has_description, &a_description);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicGeneralProperty>,
    ) -> Result<(), Failure> {
        rw_block_0178!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepBasicGeneralProperty>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwGeneralPropertyAssociation,
    "RwStepBasicRwGeneralPropertyAssociation"
);
impl RwStepBasicRwGeneralPropertyAssociation {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_ach: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepBasicGeneralPropertyAssociation>,
    ) -> Result<(), Failure> {
        rw_block_0141!(a_description, a_name, the_ach, the_data, the_num);
        the_data.read_string(the_num, 2, "description", the_ach, &mut a_description)?;
        let mut a_general_property: Handle<StepBasicGeneralProperty> = Default::default();
        the_data.read_entity(
            the_num,
            3,
            "base_definition",
            the_ach,
            StepBasicGeneralProperty::static_type(),
            &mut a_general_property,
        )?;
        let mut a_property_definition: Handle<StepReprPropertyDefinition> = Default::default();
        the_data.read_entity(
            the_num,
            4,
            "derived_definition",
            the_ach,
            StepReprPropertyDefinition::static_type(),
            &mut a_property_definition,
        )?;
        the_ent.init(
            &a_name,
            &a_description,
            &a_general_property,
            &a_property_definition,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepBasicGeneralPropertyAssociation>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.send_entity(&the_ent.description())?;
        the_sw.send_entity(&the_ent.general_property())?;
        the_sw.send_entity(&the_ent.property_definition())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepBasicGeneralPropertyAssociation>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        the_iter.add_item(&the_ent.general_property().cast());
        the_iter.add_item(&the_ent.property_definition().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwGeneralPropertyRelationship,
    "RwStepBasicRwGeneralPropertyRelationship"
);
impl RwStepBasicRwGeneralPropertyRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_ach: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepBasicGeneralPropertyRelationship>,
    ) -> Result<(), Failure> {
        rw_block_0141!(a_description, a_name, the_ach, the_data, the_num);
        let mut has_description: bool = true;
        if the_data.is_param_defined(the_num, 2)? {
            the_data.read_string(the_num, 2, "description", the_ach, &mut a_description)?;
        } else {
            has_description = false;
        }
        let mut a_relating_general_property: Handle<StepBasicGeneralProperty> = Default::default();
        the_data.read_entity(
            the_num,
            3,
            "relating_property",
            the_ach,
            StepBasicGeneralProperty::static_type(),
            &mut a_relating_general_property,
        )?;
        let mut a_related_general_property: Handle<StepBasicGeneralProperty> = Default::default();
        the_data.read_entity(
            the_num,
            4,
            "related_property",
            the_ach,
            StepBasicGeneralProperty::static_type(),
            &mut a_related_general_property,
        )?;
        the_ent.init(
            &a_name,
            has_description,
            &a_description,
            &a_relating_general_property,
            &a_related_general_property,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepBasicGeneralPropertyRelationship>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        if the_ent.has_description() {
            the_sw.send_entity(&the_ent.description())?;
        } else {
            the_sw.send_undef()?;
        }
        the_sw.send_entity(&the_ent.relating_general_property())?;
        the_sw.send_entity(&the_ent.related_general_property())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepBasicGeneralPropertyRelationship>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        the_iter.add_item(&the_ent.relating_general_property().cast());
        the_iter.add_item(&the_ent.related_general_property().cast());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwGroup, "RwStepBasicRwGroup");
impl RwStepBasicRwGroup {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicGroup>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "group")? {
            return Ok(());
        }
        rw_block_0082!(a_description, a_name, ach, data, ent, has_description, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicGroup>,
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
        _unused_0: &Handle<StepBasicGroup>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
