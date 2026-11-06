#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepFEARwNodeRepresentation,
    "RwStepFEARwNodeRepresentation"
);
impl RwStepFEARwNodeRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEANodeRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "node_representation")? {
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
        let mut a_model_ref: Handle<StepFEAFeaModel> = Default::default();
        data.read_entity(
            num,
            4,
            "model_ref",
            ach,
            StepFEAFeaModel::static_type(),
            &mut a_model_ref,
        )?;
        ent.init(
            &a_representation_name,
            &a_representation_items,
            &a_representation_context_of_items,
            &a_model_ref,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEANodeRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_b_0107!(ent, i1, sw, var0);
        sw.send_entity(&ent.model_ref())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEANodeRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0207!(ent, i1, iter, var0);
        iter.add_item(&ent.model_ref().cast());
        Ok(())
    }
}

step_rw_class!(RwStepFEARwNodeSet, "RwStepFEARwNodeSet");
impl RwStepFEARwNodeSet {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEANodeSet>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "node_set")? {
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
        let mut a_nodes: Handle<HArray1<Handle<StepFEANodeRepresentation>>> = Default::default();
        let mut sub2: i32 = 0;
        if data.read_sub_list(num, 2, "nodes", ach, &mut sub2, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub2)?;
            a_nodes = step_new_array1::<Handle<StepFEANodeRepresentation>>(1, nb0)?;
            let num2: i32 = sub2;
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
        ent.init(&a_representation_item_name, &a_nodes);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEANodeSet>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.open_sub()?;
        for i1 in 1..=step_live(&ent.nodes(), "WriteStep")?.array1().length() {
            let var0: Handle<StepFEANodeRepresentation> =
                step_array1_item(&ent.nodes(), i1, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEANodeSet>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        for i1 in 1..=step_live(&ent.nodes(), "Share")?.array1().length() {
            let var0: Handle<StepFEANodeRepresentation> =
                step_array1_item(&ent.nodes(), i1, "Share")?;
            iter.add_item(&var0.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwNodeWithSolutionCoordinateSystem,
    "RwStepFEARwNodeWithSolutionCoordinateSystem"
);
impl RwStepFEARwNodeWithSolutionCoordinateSystem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEANodeWithSolutionCoordinateSystem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "node_with_solution_coordinate_system")? {
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
        ent: &Handle<StepFEANodeWithSolutionCoordinateSystem>,
    ) -> Result<(), Failure> {
        rw_block_b_0086!(ent, i1, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEANodeWithSolutionCoordinateSystem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0170!(ent, i1, iter, var0);
        Ok(())
    }
}

step_rw_class!(RwStepFEARwNodeWithVector, "RwStepFEARwNodeWithVector");
impl RwStepFEARwNodeWithVector {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEANodeWithVector>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "node_with_vector")? {
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
        ent: &Handle<StepFEANodeWithVector>,
    ) -> Result<(), Failure> {
        rw_block_b_0086!(ent, i1, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEANodeWithVector>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0170!(ent, i1, iter, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwParametricCurve3dElementCoordinateDirection,
    "RwStepFEARwParametricCurve3dElementCoordinateDirection"
);
impl RwStepFEARwParametricCurve3dElementCoordinateDirection {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAParametricCurve3dElementCoordinateDirection>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            2,
            ach,
            "parametric_curve3d_element_coordinate_direction",
        )? {
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
        let mut a_orientation: Handle<StepGeomDirection> = Default::default();
        data.read_entity(
            num,
            2,
            "orientation",
            ach,
            StepGeomDirection::static_type(),
            &mut a_orientation,
        )?;
        ent.init(&a_representation_item_name, &a_orientation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAParametricCurve3dElementCoordinateDirection>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_entity(&ent.orientation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAParametricCurve3dElementCoordinateDirection>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.orientation().cast());
        Ok(())
    }
}
