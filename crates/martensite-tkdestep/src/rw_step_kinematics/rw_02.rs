#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepKinematicsRwActuatedKinematicPair,
    "RwStepKinematicsRwActuatedKinematicPair"
);
impl RwStepKinematicsRwActuatedKinematicPair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsActuatedKinematicPair>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 12, the_arch, "actuated_kinematic_pair")? {
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
        let (a_tx, has_tx) =
            step_read_actuated_direction(the_data, the_num, 7, 7, "t_x", the_arch)?;
        let (a_ty, has_ty) =
            step_read_actuated_direction(the_data, the_num, 8, 8, "t_y", the_arch)?;
        let (a_tz, has_tz) =
            step_read_actuated_direction(the_data, the_num, 9, 9, "t_z", the_arch)?;
        let (a_rx, has_rx) =
            step_read_actuated_direction(the_data, the_num, 10, 10, "r_x", the_arch)?;
        let (a_ry, has_ry) =
            step_read_actuated_direction(the_data, the_num, 11, 11, "r_y", the_arch)?;
        let (a_rz, has_rz) =
            step_read_actuated_direction(the_data, the_num, 12, 12, "r_z", the_arch)?;
        the_ent.init(
            &a_representation_item_name,
            &a_item_defined_transformation_name,
            has_item_defined_transformation_description,
            &a_item_defined_transformation_description,
            &a_item_defined_transformation_transform_item1,
            &a_item_defined_transformation_transform_item2,
            &a_kinematic_pair_joint,
            has_tx,
            a_tx,
            has_ty,
            a_ty,
            has_tz,
            a_tz,
            has_rx,
            a_rx,
            has_ry,
            a_ry,
            has_rz,
            a_rz,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsActuatedKinematicPair>,
    ) -> Result<(), Failure> {
        rw_block_b_0084!(the_ent, the_sw);
        if the_ent.has_tx() {
            if the_ent.tx() == StepKinematicsActuatedDirection::AdBidirectional {
                the_sw.send_enum(b".BIDIRECTIONAL.")?;
            } else {
                if the_ent.tx() == StepKinematicsActuatedDirection::AdPositiveOnly {
                    the_sw.send_enum(b".POSITIVE_ONLY.")?;
                } else {
                    if the_ent.tx() == StepKinematicsActuatedDirection::AdNegativeOnly {
                        the_sw.send_enum(b".NEGATIVE_ONLY.")?;
                    } else {
                        if the_ent.tx() == StepKinematicsActuatedDirection::AdNotActuated {
                            the_sw.send_enum(b".NOT_ACTUATED.")?;
                        }
                    }
                }
            }
        } else {
            the_sw.send_undef()?;
        }
        if the_ent.has_ty() {
            if the_ent.ty() == StepKinematicsActuatedDirection::AdBidirectional {
                the_sw.send_enum(b".BIDIRECTIONAL.")?;
            } else {
                if the_ent.ty() == StepKinematicsActuatedDirection::AdPositiveOnly {
                    the_sw.send_enum(b".POSITIVE_ONLY.")?;
                } else {
                    if the_ent.ty() == StepKinematicsActuatedDirection::AdNegativeOnly {
                        the_sw.send_enum(b".NEGATIVE_ONLY.")?;
                    } else {
                        if the_ent.ty() == StepKinematicsActuatedDirection::AdNotActuated {
                            the_sw.send_enum(b".NOT_ACTUATED.")?;
                        }
                    }
                }
            }
        } else {
            the_sw.send_undef()?;
        }
        if the_ent.has_tz() {
            if the_ent.tz() == StepKinematicsActuatedDirection::AdBidirectional {
                the_sw.send_enum(b".BIDIRECTIONAL.")?;
            } else {
                if the_ent.tz() == StepKinematicsActuatedDirection::AdPositiveOnly {
                    the_sw.send_enum(b".POSITIVE_ONLY.")?;
                } else {
                    if the_ent.tz() == StepKinematicsActuatedDirection::AdNegativeOnly {
                        the_sw.send_enum(b".NEGATIVE_ONLY.")?;
                    } else {
                        if the_ent.tz() == StepKinematicsActuatedDirection::AdNotActuated {
                            the_sw.send_enum(b".NOT_ACTUATED.")?;
                        }
                    }
                }
            }
        } else {
            the_sw.send_undef()?;
        }
        if the_ent.has_rx() {
            if the_ent.rx() == StepKinematicsActuatedDirection::AdBidirectional {
                the_sw.send_enum(b".BIDIRECTIONAL.")?;
            } else {
                if the_ent.rx() == StepKinematicsActuatedDirection::AdPositiveOnly {
                    the_sw.send_enum(b".POSITIVE_ONLY.")?;
                } else {
                    if the_ent.rx() == StepKinematicsActuatedDirection::AdNegativeOnly {
                        the_sw.send_enum(b".NEGATIVE_ONLY.")?;
                    } else {
                        if the_ent.rx() == StepKinematicsActuatedDirection::AdNotActuated {
                            the_sw.send_enum(b".NOT_ACTUATED.")?;
                        }
                    }
                }
            }
        } else {
            the_sw.send_undef()?;
        }
        if the_ent.has_ry() {
            if the_ent.ry() == StepKinematicsActuatedDirection::AdBidirectional {
                the_sw.send_enum(b".BIDIRECTIONAL.")?;
            } else {
                if the_ent.ry() == StepKinematicsActuatedDirection::AdPositiveOnly {
                    the_sw.send_enum(b".POSITIVE_ONLY.")?;
                } else {
                    if the_ent.ry() == StepKinematicsActuatedDirection::AdNegativeOnly {
                        the_sw.send_enum(b".NEGATIVE_ONLY.")?;
                    } else {
                        if the_ent.ry() == StepKinematicsActuatedDirection::AdNotActuated {
                            the_sw.send_enum(b".NOT_ACTUATED.")?;
                        }
                    }
                }
            }
        } else {
            the_sw.send_undef()?;
        }
        if the_ent.has_rz() {
            if the_ent.rz() == StepKinematicsActuatedDirection::AdBidirectional {
                the_sw.send_enum(b".BIDIRECTIONAL.")?;
            } else {
                if the_ent.rz() == StepKinematicsActuatedDirection::AdPositiveOnly {
                    the_sw.send_enum(b".POSITIVE_ONLY.")?;
                } else {
                    if the_ent.rz() == StepKinematicsActuatedDirection::AdNegativeOnly {
                        the_sw.send_enum(b".NEGATIVE_ONLY.")?;
                    } else {
                        if the_ent.rz() == StepKinematicsActuatedDirection::AdNotActuated {
                            the_sw.send_enum(b".NOT_ACTUATED.")?;
                        }
                    }
                }
            }
        } else {
            the_sw.send_undef()?;
        }
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsActuatedKinematicPair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0177!(iter, the_ent);
        Ok(())
    }
}
