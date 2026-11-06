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
        let mut a_tx: StepKinematicsActuatedDirection =
            StepKinematicsActuatedDirection::AdNotActuated;
        let mut has_tx: bool = true;
        if the_data.is_param_defined(the_num, 7)? {
            if the_data.param_type(the_num, 7)? == InterfaceParamType::ParamEnum {
                let text: Vec<u8> = the_data.param_c_value(the_num, 7)?;
                if text == b".BIDIRECTIONAL." {
                    a_tx = StepKinematicsActuatedDirection::AdBidirectional;
                } else {
                    if text == b".POSITIVE_ONLY." {
                        a_tx = StepKinematicsActuatedDirection::AdPositiveOnly;
                    } else {
                        if text == b".NEGATIVE_ONLY." {
                            a_tx = StepKinematicsActuatedDirection::AdNegativeOnly;
                        } else {
                            if text == b".NOT_ACTUATED." {
                                a_tx = StepKinematicsActuatedDirection::AdNotActuated;
                            } else {
                                the_arch
                                    .add_fail_text("Parameter #7 (t_x) has not allowed value", "");
                            }
                        }
                    }
                }
            } else {
                the_arch.add_fail_text("Parameter #7 (t_x) is not enumeration", "");
            }
        } else {
            has_tx = false;
        }
        let mut a_ty: StepKinematicsActuatedDirection =
            StepKinematicsActuatedDirection::AdNotActuated;
        let mut has_ty: bool = true;
        if the_data.is_param_defined(the_num, 8)? {
            if the_data.param_type(the_num, 8)? == InterfaceParamType::ParamEnum {
                let text: Vec<u8> = the_data.param_c_value(the_num, 8)?;
                if text == b".BIDIRECTIONAL." {
                    a_ty = StepKinematicsActuatedDirection::AdBidirectional;
                } else {
                    if text == b".POSITIVE_ONLY." {
                        a_ty = StepKinematicsActuatedDirection::AdPositiveOnly;
                    } else {
                        if text == b".NEGATIVE_ONLY." {
                            a_ty = StepKinematicsActuatedDirection::AdNegativeOnly;
                        } else {
                            if text == b".NOT_ACTUATED." {
                                a_ty = StepKinematicsActuatedDirection::AdNotActuated;
                            } else {
                                the_arch
                                    .add_fail_text("Parameter #8 (t_y) has not allowed value", "");
                            }
                        }
                    }
                }
            } else {
                the_arch.add_fail_text("Parameter #8 (t_y) is not enumeration", "");
            }
        } else {
            has_ty = false;
        }
        let mut a_tz: StepKinematicsActuatedDirection =
            StepKinematicsActuatedDirection::AdNotActuated;
        let mut has_tz: bool = true;
        if the_data.is_param_defined(the_num, 9)? {
            if the_data.param_type(the_num, 9)? == InterfaceParamType::ParamEnum {
                let text: Vec<u8> = the_data.param_c_value(the_num, 9)?;
                if text == b".BIDIRECTIONAL." {
                    a_tz = StepKinematicsActuatedDirection::AdBidirectional;
                } else {
                    if text == b".POSITIVE_ONLY." {
                        a_tz = StepKinematicsActuatedDirection::AdPositiveOnly;
                    } else {
                        if text == b".NEGATIVE_ONLY." {
                            a_tz = StepKinematicsActuatedDirection::AdNegativeOnly;
                        } else {
                            if text == b".NOT_ACTUATED." {
                                a_tz = StepKinematicsActuatedDirection::AdNotActuated;
                            } else {
                                the_arch
                                    .add_fail_text("Parameter #9 (t_z) has not allowed value", "");
                            }
                        }
                    }
                }
            } else {
                the_arch.add_fail_text("Parameter #9 (t_z) is not enumeration", "");
            }
        } else {
            has_tz = false;
        }
        let mut a_rx: StepKinematicsActuatedDirection =
            StepKinematicsActuatedDirection::AdNotActuated;
        let mut has_rx: bool = true;
        if the_data.is_param_defined(the_num, 10)? {
            if the_data.param_type(the_num, 10)? == InterfaceParamType::ParamEnum {
                let text: Vec<u8> = the_data.param_c_value(the_num, 10)?;
                if text == b".BIDIRECTIONAL." {
                    a_rx = StepKinematicsActuatedDirection::AdBidirectional;
                } else {
                    if text == b".POSITIVE_ONLY." {
                        a_rx = StepKinematicsActuatedDirection::AdPositiveOnly;
                    } else {
                        if text == b".NEGATIVE_ONLY." {
                            a_rx = StepKinematicsActuatedDirection::AdNegativeOnly;
                        } else {
                            if text == b".NOT_ACTUATED." {
                                a_rx = StepKinematicsActuatedDirection::AdNotActuated;
                            } else {
                                the_arch
                                    .add_fail_text("Parameter #10 (r_x) has not allowed value", "");
                            }
                        }
                    }
                }
            } else {
                the_arch.add_fail_text("Parameter #10 (r_x) is not enumeration", "");
            }
        } else {
            has_rx = false;
        }
        let mut a_ry: StepKinematicsActuatedDirection =
            StepKinematicsActuatedDirection::AdNotActuated;
        let mut has_ry: bool = true;
        if the_data.is_param_defined(the_num, 11)? {
            if the_data.param_type(the_num, 11)? == InterfaceParamType::ParamEnum {
                let text: Vec<u8> = the_data.param_c_value(the_num, 11)?;
                if text == b".BIDIRECTIONAL." {
                    a_ry = StepKinematicsActuatedDirection::AdBidirectional;
                } else {
                    if text == b".POSITIVE_ONLY." {
                        a_ry = StepKinematicsActuatedDirection::AdPositiveOnly;
                    } else {
                        if text == b".NEGATIVE_ONLY." {
                            a_ry = StepKinematicsActuatedDirection::AdNegativeOnly;
                        } else {
                            if text == b".NOT_ACTUATED." {
                                a_ry = StepKinematicsActuatedDirection::AdNotActuated;
                            } else {
                                the_arch
                                    .add_fail_text("Parameter #11 (r_y) has not allowed value", "");
                            }
                        }
                    }
                }
            } else {
                the_arch.add_fail_text("Parameter #11 (r_y) is not enumeration", "");
            }
        } else {
            has_ry = false;
        }
        let mut a_rz: StepKinematicsActuatedDirection =
            StepKinematicsActuatedDirection::AdNotActuated;
        let mut has_rz: bool = true;
        if the_data.is_param_defined(the_num, 12)? {
            if the_data.param_type(the_num, 12)? == InterfaceParamType::ParamEnum {
                let text: Vec<u8> = the_data.param_c_value(the_num, 12)?;
                if text == b".BIDIRECTIONAL." {
                    a_rz = StepKinematicsActuatedDirection::AdBidirectional;
                } else {
                    if text == b".POSITIVE_ONLY." {
                        a_rz = StepKinematicsActuatedDirection::AdPositiveOnly;
                    } else {
                        if text == b".NEGATIVE_ONLY." {
                            a_rz = StepKinematicsActuatedDirection::AdNegativeOnly;
                        } else {
                            if text == b".NOT_ACTUATED." {
                                a_rz = StepKinematicsActuatedDirection::AdNotActuated;
                            } else {
                                the_arch
                                    .add_fail_text("Parameter #12 (r_z) has not allowed value", "");
                            }
                        }
                    }
                }
            } else {
                the_arch.add_fail_text("Parameter #12 (r_z) is not enumeration", "");
            }
        } else {
            has_rz = false;
        }
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
