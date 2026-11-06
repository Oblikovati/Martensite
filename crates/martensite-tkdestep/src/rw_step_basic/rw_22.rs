#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepBasicRwProductDefinitionWithAssociatedDocuments,
    "RwStepBasicRwProductDefinitionWithAssociatedDocuments"
);
impl RwStepBasicRwProductDefinitionWithAssociatedDocuments {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicProductDefinitionWithAssociatedDocuments>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "product_definition")? {
            return Ok(());
        }
        rw_block_0148!(a_description, a_id, ach, data, num);
        rw_block_0124!(a_formation, a_frame_of_reference, ach, data, num);
        let mut a_doc_ids: Handle<HArray1<Handle<StepBasicDocument>>> = Default::default();
        let mut anent5: Handle<StepBasicDocument> = Default::default();
        let mut nsub5: i32 = Default::default();
        if data.read_sub_list(num, 5, "frame_of_reference", ach, &mut nsub5, false, 0, 0)? {
            let nb5: i32 = data.nb_params(nsub5)?;
            if nb5 > 0 {
                a_doc_ids = step_new_array1::<Handle<StepBasicDocument>>(1, nb5)?;
            }
            for i5 in 1..=nb5 {
                if data.read_entity(
                    nsub5,
                    i5,
                    "product_context",
                    ach,
                    StepBasicDocument::static_type(),
                    &mut anent5,
                )? {
                    step_array1_set(&a_doc_ids, i5, anent5.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(
            &a_id,
            &a_description,
            &a_formation,
            &a_frame_of_reference,
            &a_doc_ids,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicProductDefinitionWithAssociatedDocuments>,
    ) -> Result<(), Failure> {
        rw_block_0164!(ent, sw);
        sw.open_sub()?;
        let i: i32 = Default::default();
        let nb: i32 = ent.nb_doc_ids();
        for i in 1..=nb {
            sw.send_entity(&ent.doc_ids_value(i)?)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicProductDefinitionWithAssociatedDocuments>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.formation().cast());
        iter.get_one_item(&ent.frame_of_reference().cast());
        let i: i32 = Default::default();
        let nb: i32 = ent.nb_doc_ids();
        for i in 1..=nb {
            iter.add_item(&ent.doc_ids_value(i)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwProductRelatedProductCategory,
    "RwStepBasicRwProductRelatedProductCategory"
);
impl RwStepBasicRwProductRelatedProductCategory {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicProductRelatedProductCategory>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "product_related_product_category")? {
            return Ok(());
        }
        rw_block_0030!(
            a_description,
            a_name,
            a_products,
            ach,
            anent3,
            data,
            ent,
            has_adescription,
            i3,
            nb3,
            nsub3,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicProductRelatedProductCategory>,
    ) -> Result<(), Failure> {
        rw_block_0090!(ent, has_adescription, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicProductRelatedProductCategory>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_products();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.products_value(is1)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwProductType, "RwStepBasicRwProductType");
impl RwStepBasicRwProductType {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicProductType>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "product_type")? {
            return Ok(());
        }
        rw_block_0030!(
            a_description,
            a_name,
            a_products,
            ach,
            anent3,
            data,
            ent,
            has_adescription,
            i3,
            nb3,
            nsub3,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicProductType>,
    ) -> Result<(), Failure> {
        rw_block_0090!(ent, has_adescription, i3, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicProductType>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_products();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.products_value(is1)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwRatioMeasureWithUnit,
    "RwStepBasicRwRatioMeasureWithUnit"
);
impl RwStepBasicRwRatioMeasureWithUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicRatioMeasureWithUnit>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "ratio_measure_with_unit")? {
            return Ok(());
        }
        rw_block_0111!(a_unit_component, ach, data, ent, mvc, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicRatioMeasureWithUnit>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.value_component_member())?;
        sw.send_entity(&ent.unit_component().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicRatioMeasureWithUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.unit_component().value());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwRoleAssociation, "RwStepBasicRwRoleAssociation");
impl RwStepBasicRwRoleAssociation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicRoleAssociation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "role_association")? {
            return Ok(());
        }
        let mut a_role: Handle<StepBasicObjectRole> = Default::default();
        data.read_entity(
            num,
            1,
            "role",
            ach,
            StepBasicObjectRole::static_type(),
            &mut a_role,
        )?;
        let mut a_item_with_role: StepBasicRoleSelect = Default::default();
        data.read_entity_select(num, 2, "item_with_role", ach, &mut a_item_with_role)?;
        ent.init(&a_role, &a_item_with_role);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicRoleAssociation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.role())?;
        sw.send_entity(&ent.item_with_role().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicRoleAssociation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.role().cast());
        iter.add_item(&ent.item_with_role().value());
        Ok(())
    }
}
