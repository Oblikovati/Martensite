#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepFEARwFreedomsList, "RwStepFEARwFreedomsList");
impl RwStepFEARwFreedomsList {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFreedomsList>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "freedoms_list")? {
            return Ok(());
        }
        let mut a_freedoms: Handle<HArray1<StepFEADegreeOfFreedom>> = Default::default();
        let mut sub1: i32 = 0;
        if data.read_sub_list(num, 1, "freedoms", ach, &mut sub1, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub1)?;
            a_freedoms = step_new_array1::<StepFEADegreeOfFreedom>(1, nb0)?;
            let num2: i32 = sub1;
            for i0 in 1..=nb0 {
                let mut an_it0: StepFEADegreeOfFreedom = Default::default();
                data.read_entity_select(num2, i0, "degree_of_freedom", ach, &mut an_it0)?;
                step_array1_set(&a_freedoms, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(&a_freedoms);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAFreedomsList>,
    ) -> Result<(), Failure> {
        sw.open_sub()?;
        for i0 in 1..=step_live(&ent.freedoms(), "WriteStep")?.array1().length() {
            let var0: StepFEADegreeOfFreedom = step_array1_item(&ent.freedoms(), i0, "WriteStep")?;
            sw.send_entity(&var0.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFreedomsList>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        for i1 in 1..=step_live(&ent.freedoms(), "Share")?.array1().length() {
            let var0: StepFEADegreeOfFreedom = step_array1_item(&ent.freedoms(), i1, "Share")?;
            iter.add_item(&var0.value());
        }
        Ok(())
    }
}

step_rw_class!(RwStepFEARwGeometricNode, "RwStepFEARwGeometricNode");
impl RwStepFEARwGeometricNode {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAGeometricNode>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "geometric_node")? {
            return Ok(());
        }
        rw_block_b_0037!(
            a_node_representation_model_ref,
            a_representation_context_of_items,
            a_representation_items,
            a_representation_name,
            ach,
            an_it0,
            data,
            ent,
            i0,
            nb0,
            num,
            num2,
            sub2
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAGeometricNode>,
    ) -> Result<(), Failure> {
        rw_block_b_0091!(ent, i1, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAGeometricNode>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0174!(ent, i1, iter, var0);
        Ok(())
    }
}

step_rw_class!(RwStepFEARwNode, "RwStepFEARwNode");
impl RwStepFEARwNode {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEANode>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "node")? {
            return Ok(());
        }
        rw_block_b_0037!(
            a_node_representation_model_ref,
            a_representation_context_of_items,
            a_representation_items,
            a_representation_name,
            ach,
            an_it0,
            data,
            ent,
            i0,
            nb0,
            num,
            num2,
            sub2
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEANode>,
    ) -> Result<(), Failure> {
        rw_block_b_0091!(ent, i1, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEANode>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0174!(ent, i1, iter, var0);
        Ok(())
    }
}

step_rw_class!(RwStepFEARwNodeDefinition, "RwStepFEARwNodeDefinition");
impl RwStepFEARwNodeDefinition {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEANodeDefinition>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "node_definition")? {
            return Ok(());
        }
        rw_block_b_0054!(
            a_shape_aspect_description,
            a_shape_aspect_name,
            a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
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
        ent: &Handle<StepFEANodeDefinition>,
    ) -> Result<(), Failure> {
        rw_block_b_0158!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEANodeDefinition>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(RwStepFEARwNodeGroup, "RwStepFEARwNodeGroup");
impl RwStepFEARwNodeGroup {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEANodeGroup>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "node_group")? {
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
        let mut a_nodes: Handle<HArray1<Handle<StepFEANodeRepresentation>>> = Default::default();
        let mut sub4: i32 = 0;
        if data.read_sub_list(num, 4, "nodes", ach, &mut sub4, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub4)?;
            a_nodes = step_new_array1::<Handle<StepFEANodeRepresentation>>(1, nb0)?;
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
                step_array1_set(&a_nodes, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(
            &a_group_name,
            &a_group_description,
            &a_fea_group_model_ref,
            &a_nodes,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEANodeGroup>,
    ) -> Result<(), Failure> {
        rw_block_b_0162!(ent, sw);
        for i3 in 1..=step_live(&ent.nodes(), "WriteStep")?.array1().length() {
            let var0: Handle<StepFEANodeRepresentation> =
                step_array1_item(&ent.nodes(), i3, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEANodeGroup>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.model_ref().cast());
        for i2 in 1..=step_live(&ent.nodes(), "Share")?.array1().length() {
            let var0: Handle<StepFEANodeRepresentation> =
                step_array1_item(&ent.nodes(), i2, "Share")?;
            iter.add_item(&var0.cast());
        }
        Ok(())
    }
}
