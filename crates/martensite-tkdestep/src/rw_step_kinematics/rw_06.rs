#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepKinematicsRwKinematicTopologyNetworkStructure,
    "RwStepKinematicsRwKinematicTopologyNetworkStructure"
);
impl RwStepKinematicsRwKinematicTopologyNetworkStructure {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsKinematicTopologyNetworkStructure>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(
            the_num,
            4,
            the_arch,
            "kinematic_topology_network_structure",
        )? {
            return Ok(());
        }
        rw_block_b_0035!(
            a_parent,
            a_representation_context_of_items,
            a_representation_items,
            a_representation_name,
            an_it0,
            i0,
            nb0,
            num2,
            sub2,
            the_arch,
            the_data,
            the_ent,
            the_num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsKinematicTopologyNetworkStructure>,
    ) -> Result<(), Failure> {
        rw_block_b_0092!(i1, the_ent, the_sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsKinematicTopologyNetworkStructure>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0171!(i1, iter, the_ent, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwKinematicTopologyStructure,
    "RwStepKinematicsRwKinematicTopologyStructure"
);
impl RwStepKinematicsRwKinematicTopologyStructure {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsKinematicTopologyStructure>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 3, the_arch, "kinematic_topology_structure")? {
            return Ok(());
        }
        rw_block_b_0065!(
            a_representation_context_of_items,
            a_representation_items,
            a_representation_name,
            an_it0,
            i0,
            nb0,
            num2,
            sub2,
            the_arch,
            the_data,
            the_num
        );
        the_ent.init(
            &a_representation_name,
            &a_representation_items,
            &a_representation_context_of_items,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsKinematicTopologyStructure>,
    ) -> Result<(), Failure> {
        rw_block_b_0105!(i1, the_ent, the_sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsKinematicTopologyStructure>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0204!(i1, iter, the_ent, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwLinearFlexibleAndPinionPair,
    "RwStepKinematicsRwLinearFlexibleAndPinionPair"
);
impl RwStepKinematicsRwLinearFlexibleAndPinionPair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsLinearFlexibleAndPinionPair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 7, the_arch, "linear_flexible_and_pinion_pair")? {
            return Ok(());
        }
        rw_block_b_0014!(
            a_item_defined_transformation_description,
            a_item_defined_transformation_name,
            a_item_defined_transformation_transform_item1,
            a_item_defined_transformation_transform_item2,
            a_kinematic_pair_joint,
            a_pinion_radius,
            a_representation_item_name,
            has_item_defined_transformation_description,
            the_arch,
            the_data,
            the_ent,
            the_num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsLinearFlexibleAndPinionPair>,
    ) -> Result<(), Failure> {
        rw_block_b_0071!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsLinearFlexibleAndPinionPair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0176!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwLinearFlexibleAndPlanarCurvePair,
    "RwStepKinematicsRwLinearFlexibleAndPlanarCurvePair"
);
impl RwStepKinematicsRwLinearFlexibleAndPlanarCurvePair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsLinearFlexibleAndPlanarCurvePair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(
            the_num,
            8,
            the_arch,
            "linear_flexible_and_planar_curve_pair",
        )? {
            return Ok(());
        }
        rw_block_b_0011!(
            a_item_defined_transformation_description,
            a_item_defined_transformation_name,
            a_item_defined_transformation_transform_item1,
            a_item_defined_transformation_transform_item2,
            a_kinematic_pair_joint,
            a_orientation,
            a_pair_curve,
            a_representation_item_name,
            has_item_defined_transformation_description,
            the_arch,
            the_data,
            the_ent,
            the_num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsLinearFlexibleAndPlanarCurvePair>,
    ) -> Result<(), Failure> {
        rw_block_b_0057!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsLinearFlexibleAndPlanarCurvePair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0129!(iter, the_ent);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwLinearFlexibleLinkRepresentation,
    "RwStepKinematicsRwLinearFlexibleLinkRepresentation"
);
impl RwStepKinematicsRwLinearFlexibleLinkRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsLinearFlexibleLinkRepresentation>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 4, the_arch, "linear_flexible_link_representation")? {
            return Ok(());
        }
        rw_block_b_0036!(
            a_kinematic_link_representation_represented_link,
            a_representation_context_of_items,
            a_representation_items,
            a_representation_name,
            an_it0,
            i0,
            nb0,
            num2,
            sub2,
            the_arch,
            the_data,
            the_ent,
            the_num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsLinearFlexibleLinkRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_b_0093!(i1, the_ent, the_sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsLinearFlexibleLinkRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0169!(i1, iter, the_ent, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepKinematicsRwLowOrderKinematicPair,
    "RwStepKinematicsRwLowOrderKinematicPair"
);
impl RwStepKinematicsRwLowOrderKinematicPair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsLowOrderKinematicPair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 12, the_arch, "low_order_kinematic_pair")? {
            return Ok(());
        }
        rw_block_b_0022!(
            a_item_defined_transformation_description,
            a_item_defined_transformation_name,
            a_item_defined_transformation_transform_item1,
            a_item_defined_transformation_transform_item2,
            a_kinematic_pair_joint,
            a_representation_item_name,
            has_item_defined_transformation_description,
            the_arch,
            the_data,
            the_num
        );
        let mut a_tx: bool = Default::default();
        the_data.read_boolean(the_num, 7, "t_x", the_arch, &mut a_tx)?;
        let mut a_ty: bool = Default::default();
        the_data.read_boolean(the_num, 8, "t_y", the_arch, &mut a_ty)?;
        let mut a_tz: bool = Default::default();
        the_data.read_boolean(the_num, 9, "t_z", the_arch, &mut a_tz)?;
        let mut a_rx: bool = Default::default();
        the_data.read_boolean(the_num, 10, "r_x", the_arch, &mut a_rx)?;
        let mut a_ry: bool = Default::default();
        the_data.read_boolean(the_num, 11, "r_y", the_arch, &mut a_ry)?;
        let mut a_rz: bool = Default::default();
        the_data.read_boolean(the_num, 12, "r_z", the_arch, &mut a_rz)?;
        the_ent.init(
            &a_representation_item_name,
            &a_item_defined_transformation_name,
            has_item_defined_transformation_description,
            &a_item_defined_transformation_description,
            &a_item_defined_transformation_transform_item1,
            &a_item_defined_transformation_transform_item2,
            &a_kinematic_pair_joint,
            a_tx,
            a_ty,
            a_tz,
            a_rx,
            a_ry,
            a_rz,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsLowOrderKinematicPair>,
    ) -> Result<(), Failure> {
        rw_block_b_0029!(the_ent, the_sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsLowOrderKinematicPair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0177!(iter, the_ent);
        Ok(())
    }
}
