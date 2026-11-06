#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepFEARwElementGroup, "RwStepFEARwElementGroup");
impl RwStepFEARwElementGroup {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAElementGroup>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "element_group")? {
            return Ok(());
        }
        rw_block_b_0094!(
            a_fea_group_model_ref,
            a_group_description,
            a_group_name,
            ach,
            data,
            num
        );
        let mut a_elements: Handle<HArray1<Handle<StepFEAElementRepresentation>>> =
            Default::default();
        let mut sub4: i32 = 0;
        if data.read_sub_list(num, 4, "elements", ach, &mut sub4, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub4)?;
            a_elements = step_new_array1::<Handle<StepFEAElementRepresentation>>(1, nb0)?;
            let num2: i32 = sub4;
            for i0 in 1..=nb0 {
                let mut an_it0: Handle<StepFEAElementRepresentation> = Default::default();
                data.read_entity(
                    num2,
                    i0,
                    "element_representation",
                    ach,
                    StepFEAElementRepresentation::static_type(),
                    &mut an_it0,
                )?;
                step_array1_set(&a_elements, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(
            &a_group_name,
            &a_group_description,
            &a_fea_group_model_ref,
            &a_elements,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAElementGroup>,
    ) -> Result<(), Failure> {
        rw_block_b_0162!(ent, sw);
        for i3 in 1..=step_live(&ent.elements(), "WriteStep")?.array1().length() {
            let var0: Handle<StepFEAElementRepresentation> =
                step_array1_item(&ent.elements(), i3, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAElementGroup>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.model_ref().cast());
        for i2 in 1..=step_live(&ent.elements(), "Share")?.array1().length() {
            let var0: Handle<StepFEAElementRepresentation> =
                step_array1_item(&ent.elements(), i2, "Share")?;
            iter.add_item(&var0.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwElementRepresentation,
    "RwStepFEARwElementRepresentation"
);
impl RwStepFEARwElementRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAElementRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "element_representation")? {
            return Ok(());
        }
        rw_block_b_0066!(
            a_representation_context_of_items,
            a_representation_items,
            a_representation_name,
            ach,
            an_it0,
            data,
            i0,
            nb0,
            num,
            num2,
            sub2
        );
        let mut a_node_list: Handle<HArray1<Handle<StepFEANodeRepresentation>>> =
            Default::default();
        let mut sub4: i32 = 0;
        if data.read_sub_list(num, 4, "node_list", ach, &mut sub4, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub4)?;
            a_node_list = step_new_array1::<Handle<StepFEANodeRepresentation>>(1, nb0)?;
            let num2: i32 = sub4;
            for i0 in 1..=nb0 {
                let mut an_it0: Handle<StepFEANodeRepresentation> = Default::default();
                data.read_entity(
                    num2,
                    i0,
                    "node_representation",
                    ach,
                    StepFEANodeRepresentation::static_type(),
                    &mut an_it0,
                )?;
                step_array1_set(&a_node_list, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(
            &a_representation_name,
            &a_representation_items,
            &a_representation_context_of_items,
            &a_node_list,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAElementRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_b_0107!(ent, i1, sw, var0);
        sw.open_sub()?;
        for i3 in 1..=step_live(&ent.node_list(), "WriteStep")?.array1().length() {
            let var0: Handle<StepFEANodeRepresentation> =
                step_array1_item(&ent.node_list(), i3, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAElementRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0207!(ent, i1, iter, var0);
        for i3 in 1..=step_live(&ent.node_list(), "Share")?.array1().length() {
            let var0: Handle<StepFEANodeRepresentation> =
                step_array1_item(&ent.node_list(), i3, "Share")?;
            iter.add_item(&var0.cast());
        }
        Ok(())
    }
}

step_rw_class!(RwStepFEARwFeaAreaDensity, "RwStepFEARwFeaAreaDensity");
impl RwStepFEARwFeaAreaDensity {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaAreaDensity>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "fea_area_density")? {
            return Ok(());
        }
        rw_block_b_0112!(
            a_fea_constant,
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
        ent: &Handle<StepFEAFeaAreaDensity>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_real(ent.fea_constant())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepFEAFeaAreaDensity>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
