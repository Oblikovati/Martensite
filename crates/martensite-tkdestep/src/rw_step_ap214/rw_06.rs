#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepAP214RwAutoDesignPresentedItem,
    "RwStepAP214RwAutoDesignPresentedItem"
);
impl RwStepAP214RwAutoDesignPresentedItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AutoDesignPresentedItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "auto_design_presented_item")? {
            return Ok(());
        }
        let mut a_items: Handle<HArray1<StepAP214AutoDesignPresentedItemSelect>> =
            Default::default();
        let mut anent1: StepAP214AutoDesignPresentedItemSelect = Default::default();
        let mut nsub1: i32 = Default::default();
        if data.read_sub_list(num, 1, "items", ach, &mut nsub1, false, 0, 0)? {
            let nb1: i32 = data.nb_params(nsub1)?;
            a_items = step_new_array1::<StepAP214AutoDesignPresentedItemSelect>(1, nb1)?;
            for i1 in 1..=nb1 {
                let stat1: bool = data.read_entity_select(
                    nsub1,
                    i1,
                    "auto_design_displayed_item",
                    ach,
                    &mut anent1,
                )?;
                if stat1 {
                    step_array1_set(&a_items, i1, anent1.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214AutoDesignPresentedItem>,
    ) -> Result<(), Failure> {
        rw_block_b_0199!(ent, i1, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AutoDesignPresentedItem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_items();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.items_value(is1)?.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepAP214RwAutoDesignSecurityClassificationAssignment,
    "RwStepAP214RwAutoDesignSecurityClassificationAssignment"
);
impl RwStepAP214RwAutoDesignSecurityClassificationAssignment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214AutoDesignSecurityClassificationAssignment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            2,
            ach,
            "auto_design_security_classification_assignment",
        )? {
            return Ok(());
        }
        let mut a_assigned_security_classification: Handle<StepBasicSecurityClassification> =
            Default::default();
        data.read_entity(
            num,
            1,
            "assigned_security_classification",
            ach,
            StepBasicSecurityClassification::static_type(),
            &mut a_assigned_security_classification,
        )?;
        let mut a_items: Handle<HArray1<Handle<StepBasicApproval>>> = Default::default();
        let mut anent2: Handle<StepBasicApproval> = Default::default();
        let mut nsub2: i32 = Default::default();
        if data.read_sub_list(num, 2, "items", ach, &mut nsub2, false, 0, 0)? {
            let nb2: i32 = data.nb_params(nsub2)?;
            a_items = step_new_array1::<Handle<StepBasicApproval>>(1, nb2)?;
            for i2 in 1..=nb2 {
                let stat2: bool = data.read_entity(
                    nsub2,
                    i2,
                    "auto_design_security_classified_item",
                    ach,
                    StepBasicApproval::static_type(),
                    &mut anent2,
                )?;
                if stat2 {
                    step_array1_set(&a_items, i2, anent2.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_assigned_security_classification, &a_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214AutoDesignSecurityClassificationAssignment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.assigned_security_classification())?;
        sw.open_sub()?;
        for i2 in 1..=ent.nb_items() {
            sw.send_entity(&ent.items_value(i2)?)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214AutoDesignSecurityClassificationAssignment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.assigned_security_classification().cast());
        let nb_elem2: i32 = ent.nb_items();
        for is2 in 1..=nb_elem2 {
            iter.get_one_item(&ent.items_value(is2)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(RwStepAP214RwClass, "RwStepAP214RwClass");
impl RwStepAP214RwClass {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214Class>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "class")? {
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
        ent.init(&a_group_name, has_group_description, &a_group_description);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214Class>,
    ) -> Result<(), Failure> {
        rw_block_b_0214!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepAP214Class>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepAP214RwExternallyDefinedClass,
    "RwStepAP214RwExternallyDefinedClass"
);
impl RwStepAP214RwExternallyDefinedClass {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP214ExternallyDefinedClass>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "externally_defined_class")? {
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
        let mut a_externally_defined_item_item_id: StepBasicSourceItem = Default::default();
        data.read_entity_select(
            num,
            3,
            "externally_defined_item.item_id",
            ach,
            &mut a_externally_defined_item_item_id,
        )?;
        let mut a_externally_defined_item_source: Handle<StepBasicExternalSource> =
            Default::default();
        data.read_entity(
            num,
            4,
            "externally_defined_item.source",
            ach,
            StepBasicExternalSource::static_type(),
            &mut a_externally_defined_item_source,
        )?;
        ent.init(
            &a_group_name,
            has_group_description,
            &a_group_description,
            &a_externally_defined_item_item_id,
            &a_externally_defined_item_source,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP214ExternallyDefinedClass>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        if ent.parent.parent.has_description() {
            sw.send_entity(&ent.parent.parent.description())?;
        } else {
            sw.send_undef()?;
        }
        rw_block_b_0213!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP214ExternallyDefinedClass>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0211!(ent, iter);
        Ok(())
    }
}
