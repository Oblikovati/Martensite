#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepBasicRwProduct, "RwStepBasicRwProduct");
impl RwStepBasicRwProduct {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicProduct>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "product")? {
            return Ok(());
        }
        rw_block_0110!(a_description, a_id, a_name, ach, data, num);
        if data.is_param_defined(num, 3)? {
            data.read_string(num, 3, "description", ach, &mut a_description)?;
        }
        let mut a_frame_of_reference: Handle<HArray1<Handle<StepBasicProductContext>>> =
            Default::default();
        let mut anent4: Handle<StepBasicProductContext> = Default::default();
        let mut nsub4: i32 = Default::default();
        if data.read_sub_list(num, 4, "frame_of_reference", ach, &mut nsub4, false, 0, 0)? {
            let nb4: i32 = data.nb_params(nsub4)?;
            a_frame_of_reference = step_new_array1::<Handle<StepBasicProductContext>>(1, nb4)?;
            for i4 in 1..=nb4 {
                if data.read_entity(
                    nsub4,
                    i4,
                    "product_context",
                    ach,
                    StepBasicProductContext::static_type(),
                    &mut anent4,
                )? {
                    step_array1_set(&a_frame_of_reference, i4, anent4.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(&a_id, &a_name, &a_description, &a_frame_of_reference);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicProduct>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.id())?;
        rw_block_0211!(ent, sw);
        sw.open_sub()?;
        for i4 in 1..=ent.nb_frame_of_reference() {
            sw.send_entity(&ent.frame_of_reference_value(i4)?)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicProduct>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem1: i32 = ent.nb_frame_of_reference();
        for is1 in 1..=nb_elem1 {
            iter.get_one_item(&ent.frame_of_reference_value(is1)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwProductCategory, "RwStepBasicRwProductCategory");
impl RwStepBasicRwProductCategory {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicProductCategory>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "product_category")? {
            return Ok(());
        }
        rw_block_0098!(a_description, a_name, ach, data, has_adescription, num);
        ent.init(&a_name, has_adescription, &a_description);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicProductCategory>,
    ) -> Result<(), Failure> {
        rw_block_0191!(ent, has_adescription, sw);
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwProductCategoryRelationship,
    "RwStepBasicRwProductCategoryRelationship"
);
impl RwStepBasicRwProductCategoryRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicProductCategoryRelationship>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "product_category_relationship")? {
            return Ok(());
        }
        rw_block_0102!(a_description, a_name, ach, data, has_description, num);
        let mut a_category: Handle<StepBasicProductCategory> = Default::default();
        data.read_entity(
            num,
            3,
            "category",
            ach,
            StepBasicProductCategory::static_type(),
            &mut a_category,
        )?;
        let mut a_sub_category: Handle<StepBasicProductCategory> = Default::default();
        data.read_entity(
            num,
            4,
            "sub_category",
            ach,
            StepBasicProductCategory::static_type(),
            &mut a_sub_category,
        )?;
        ent.init(
            &a_name,
            has_description,
            &a_description,
            &a_category,
            &a_sub_category,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicProductCategoryRelationship>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        if ent.has_description() {
            sw.send_entity(&ent.description())?;
        } else {
            sw.send_undef()?;
        }
        sw.send_entity(&ent.category())?;
        sw.send_entity(&ent.sub_category())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicProductCategoryRelationship>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.category().cast());
        iter.add_item(&ent.sub_category().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwProductConceptContext,
    "RwStepBasicRwProductConceptContext"
);
impl RwStepBasicRwProductConceptContext {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicProductConceptContext>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "product_concept_context")? {
            return Ok(());
        }
        let mut a_application_context_element_name: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            1,
            "application_context_element.name",
            ach,
            &mut a_application_context_element_name,
        )?;
        let mut a_application_context_element_frame_of_reference: Handle<
            StepBasicApplicationContext,
        > = Default::default();
        data.read_entity(
            num,
            2,
            "application_context_element.frame_of_reference",
            ach,
            StepBasicApplicationContext::static_type(),
            &mut a_application_context_element_frame_of_reference,
        )?;
        let mut a_market_segment_type: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            3,
            "market_segment_type",
            ach,
            &mut a_market_segment_type,
        )?;
        ent.init(
            &a_application_context_element_name,
            &a_application_context_element_frame_of_reference,
            &a_market_segment_type,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicProductConceptContext>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.name())?;
        sw.send_entity(&ent.parent.frame_of_reference())?;
        sw.send_entity(&ent.market_segment_type())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicProductConceptContext>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.frame_of_reference().cast());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwProductContext, "RwStepBasicRwProductContext");
impl RwStepBasicRwProductContext {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicProductContext>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "product_context")? {
            return Ok(());
        }
        rw_block_0069!(
            a_discipline_type,
            a_frame_of_reference,
            a_name,
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
        ent: &Handle<StepBasicProductContext>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.frame_of_reference())?;
        sw.send_entity(&ent.discipline_type())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicProductContext>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.frame_of_reference().cast());
        Ok(())
    }
}
