#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepKinematicsRwActuatedKinPairAndOrderKinPair,
    "RwStepKinematicsRwActuatedKinPairAndOrderKinPair"
);
impl RwStepKinematicsRwActuatedKinPairAndOrderKinPair {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        num0: i32,
        the_arch: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepKinematicsActuatedKinPairAndOrderKinPair>,
    ) -> Result<(), Failure> {
        let mut the_num: i32 = 0;
        the_data.named_for_complex_short(
            "ACTUATED_KINEMATIC_PAIR",
            "ACKNPR",
            num0,
            &mut the_num,
            the_arch,
        );
        if !the_data.check_nb_params(the_num, 6, the_arch, "kinematic actuated_kinematic_pair")? {
            return Ok(());
        }
        let an_actuated_kinematic_pair: Handle<StepKinematicsActuatedKinematicPair> =
            Handle::new(StepKinematicsActuatedKinematicPair::new());
        let mut a_tx: StepKinematicsActuatedDirection =
            StepKinematicsActuatedDirection::AdNotActuated;
        let mut has_tx: bool = true;
        if the_data.is_param_defined(the_num, 1)? {
            if the_data.param_type(the_num, 1)? == InterfaceParamType::ParamEnum {
                let text: Vec<u8> = the_data.param_c_value(the_num, 1)?;
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
                                    .add_fail_text("Parameter #1 (t_x) has not allowed value", "");
                            }
                        }
                    }
                }
            } else {
                the_arch.add_fail_text("Parameter #1 (t_x) is not enumeration", "");
            }
        } else {
            has_tx = false;
        }
        let mut a_ty: StepKinematicsActuatedDirection =
            StepKinematicsActuatedDirection::AdNotActuated;
        let mut has_ty: bool = true;
        if the_data.is_param_defined(the_num, 2)? {
            if the_data.param_type(the_num, 2)? == InterfaceParamType::ParamEnum {
                let text: Vec<u8> = the_data.param_c_value(the_num, 2)?;
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
                                    .add_fail_text("Parameter #2 (t_y) has not allowed value", "");
                            }
                        }
                    }
                }
            } else {
                the_arch.add_fail_text("Parameter #2 (t_y) is not enumeration", "");
            }
        } else {
            has_ty = false;
        }
        let mut a_tz: StepKinematicsActuatedDirection =
            StepKinematicsActuatedDirection::AdNotActuated;
        let mut has_tz: bool = true;
        if the_data.is_param_defined(the_num, 3)? {
            if the_data.param_type(the_num, 3)? == InterfaceParamType::ParamEnum {
                let text: Vec<u8> = the_data.param_c_value(the_num, 3)?;
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
                                    .add_fail_text("Parameter #3 (t_z) has not allowed value", "");
                            }
                        }
                    }
                }
            } else {
                the_arch.add_fail_text("Parameter #3 (t_z) is not enumeration", "");
            }
        } else {
            has_tz = false;
        }
        let mut a_rx: StepKinematicsActuatedDirection =
            StepKinematicsActuatedDirection::AdNotActuated;
        let mut has_rx: bool = true;
        if the_data.is_param_defined(the_num, 4)? {
            if the_data.param_type(the_num, 4)? == InterfaceParamType::ParamEnum {
                let text: Vec<u8> = the_data.param_c_value(the_num, 4)?;
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
                                    .add_fail_text("Parameter #14 (r_x) has not allowed value", "");
                            }
                        }
                    }
                }
            } else {
                the_arch.add_fail_text("Parameter #14 (r_x) is not enumeration", "");
            }
        } else {
            has_rx = false;
        }
        let mut a_ry: StepKinematicsActuatedDirection =
            StepKinematicsActuatedDirection::AdNotActuated;
        let mut has_ry: bool = true;
        if the_data.is_param_defined(the_num, 5)? {
            if the_data.param_type(the_num, 5)? == InterfaceParamType::ParamEnum {
                let text: Vec<u8> = the_data.param_c_value(the_num, 5)?;
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
                                    .add_fail_text("Parameter #5 (r_y) has not allowed value", "");
                            }
                        }
                    }
                }
            } else {
                the_arch.add_fail_text("Parameter #5 (r_y) is not enumeration", "");
            }
        } else {
            has_ry = false;
        }
        let mut a_rz: StepKinematicsActuatedDirection =
            StepKinematicsActuatedDirection::AdNotActuated;
        let mut has_rz: bool = true;
        if the_data.is_param_defined(the_num, 6)? {
            if the_data.param_type(the_num, 6)? == InterfaceParamType::ParamEnum {
                let text: Vec<u8> = the_data.param_c_value(the_num, 6)?;
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
                                    .add_fail_text("Parameter #6 (r_z) has not allowed value", "");
                            }
                        }
                    }
                }
            } else {
                the_arch.add_fail_text("Parameter #6 (r_z) is not enumeration", "");
            }
        } else {
            has_rz = false;
        }
        let mut a_representation_item_name: Handle<HAsciiString> = Default::default();
        the_data.named_for_complex_short(
            "GEOMETRIC_REPRESENTATION_ITEM",
            "GMRPIT",
            num0,
            &mut the_num,
            the_arch,
        );
        the_data.named_for_complex_short(
            "ITEM_DEFINED_TRANSFORMATION",
            "ITDFTR",
            num0,
            &mut the_num,
            the_arch,
        );
        let mut a_item_defined_transformation_name: Handle<HAsciiString> = Default::default();
        if the_data.param_c_value(the_num, 1)? != b"*" {
            the_data.read_string(
                the_num,
                1,
                "item_defined_transformation.name",
                the_arch,
                &mut a_item_defined_transformation_name,
            )?;
        }
        let mut a_item_defined_transformation_description: Handle<HAsciiString> =
            Default::default();
        let mut has_item_defined_transformation_description: bool = true;
        if the_data.is_param_defined(the_num, 2)? {
            the_data.read_string(
                the_num,
                2,
                "item_defined_transformation.description",
                the_arch,
                &mut a_item_defined_transformation_description,
            )?;
        } else {
            has_item_defined_transformation_description = false;
            a_item_defined_transformation_description = Default::default();
        }
        let mut a_item_defined_transformation_transform_item1: Handle<StepReprRepresentationItem> =
            Default::default();
        the_data.read_entity(
            the_num,
            3,
            "ITEM_DEFINED_TRANSFORMATION",
            the_arch,
            StepReprRepresentationItem::static_type(),
            &mut a_item_defined_transformation_transform_item1,
        )?;
        let mut a_item_defined_transformation_transform_item2: Handle<StepReprRepresentationItem> =
            Default::default();
        the_data.read_entity(
            the_num,
            4,
            "ITEM_DEFINED_TRANSFORMATION",
            the_arch,
            StepReprRepresentationItem::static_type(),
            &mut a_item_defined_transformation_transform_item2,
        )?;
        the_data.named_for_complex_short("KINEMATIC_PAIR", "KNMPR", num0, &mut the_num, the_arch);
        let mut a_kinematic_pair_joint: Handle<StepKinematicsKinematicJoint> = Default::default();
        the_data.read_entity(
            the_num,
            1,
            "kinematic_pair.joint",
            the_arch,
            StepKinematicsKinematicJoint::static_type(),
            &mut a_kinematic_pair_joint,
        )?;
        the_num = the_data.next_for_complex(the_num);
        let a_pair_super_type: Vec<u8> = the_data.c_type(the_num).into_bytes();
        let mut a_kinematic_pair: Handle<StepKinematicsKinematicPair> = Default::default();
        if a_pair_super_type == b"LOW_ORDER_KINEMATIC_PAIR" {
            let mut a_low_order_kinematic_pair_tx: bool = true;
            let mut a_low_order_kinematic_pair_ty: bool = true;
            let mut a_low_order_kinematic_pair_tz: bool = true;
            let mut a_low_order_kinematic_pair_rx: bool = true;
            let mut a_low_order_kinematic_pair_ry: bool = true;
            let mut a_low_order_kinematic_pair_rz: bool = true;
            if the_data.check_nb_params(the_num, 6, the_arch, "LOW_ORDER_KINEMATIC_PAIR")? {
                if the_data.param_c_value(the_num, 1)? != b"*" {
                    the_data.read_boolean(
                        the_num,
                        1,
                        "low_order_kinematic_pair.t_x",
                        the_arch,
                        &mut a_low_order_kinematic_pair_tx,
                    )?;
                }
                if the_data.param_c_value(the_num, 2)? != b"*" {
                    the_data.read_boolean(
                        the_num,
                        2,
                        "low_order_kinematic_pair.t_y",
                        the_arch,
                        &mut a_low_order_kinematic_pair_ty,
                    )?;
                }
                if the_data.param_c_value(the_num, 3)? != b"*" {
                    the_data.read_boolean(
                        the_num,
                        3,
                        "low_order_kinematic_pair.t_z",
                        the_arch,
                        &mut a_low_order_kinematic_pair_tz,
                    )?;
                }
                if the_data.param_c_value(the_num, 4)? != b"*" {
                    the_data.read_boolean(
                        the_num,
                        4,
                        "low_order_kinematic_pair.r_x",
                        the_arch,
                        &mut a_low_order_kinematic_pair_rx,
                    )?;
                }
                if the_data.param_c_value(the_num, 5)? != b"*" {
                    the_data.read_boolean(
                        the_num,
                        5,
                        "low_order_kinematic_pair.r_y",
                        the_arch,
                        &mut a_low_order_kinematic_pair_ry,
                    )?;
                }
                if the_data.param_c_value(the_num, 6)? != b"*" {
                    the_data.read_boolean(
                        the_num,
                        6,
                        "low_order_kinematic_pair.r_z",
                        the_arch,
                        &mut a_low_order_kinematic_pair_rz,
                    )?;
                }
            } else {
                the_arch.add_fail_text("The supertype of kinematic pair is not supported", "");
            }
            a_kinematic_pair = Handle::new(StepKinematicsKinematicPair::new());
            a_kinematic_pair.init(
                &a_representation_item_name,
                &a_item_defined_transformation_name,
                has_item_defined_transformation_description,
                &a_item_defined_transformation_description,
                &a_item_defined_transformation_transform_item1,
                &a_item_defined_transformation_transform_item2,
                &a_kinematic_pair_joint,
            );
            the_num = the_data.next_for_complex(the_num);
            let a_pairtype: Vec<u8> = the_data.c_type(the_num).into_bytes();
            if a_pairtype == b"REVOLUTE_PAIR" {
                if the_data.named_for_complex_short(
                    "REVOLUTE_PAIR_WITH_RANGE",
                    "RPWR",
                    num0,
                    &mut the_num,
                    the_arch,
                ) {
                    if the_data.check_nb_params(the_num, 2, the_arch, "REVOLUTE_PAIR_WITH_RANGE")? {
                        a_kinematic_pair =
                            Handle::new(StepKinematicsRevolutePairWithRange::new()).cast();
                        let mut a_lower_limit_actual_rotation: f64 = Default::default();
                        let mut has_lower_limit_actual_rotation: bool = true;
                        if the_data.is_param_defined(the_num, 1)? {
                            the_data.read_real(
                                the_num,
                                1,
                                "lower_limit_actual_rotation",
                                the_arch,
                                &mut a_lower_limit_actual_rotation,
                            )?;
                        } else {
                            has_lower_limit_actual_rotation = false;
                            a_lower_limit_actual_rotation = 0.0;
                        }
                        let mut a_upper_limit_actual_rotation: f64 = Default::default();
                        let mut has_upper_limit_actual_rotation: bool = true;
                        if the_data.is_param_defined(the_num, 2)? {
                            the_data.read_real(
                                the_num,
                                2,
                                "upper_limit_actual_rotation",
                                the_arch,
                                &mut a_upper_limit_actual_rotation,
                            )?;
                        } else {
                            has_upper_limit_actual_rotation = false;
                            a_upper_limit_actual_rotation = 0.0;
                        }
                        let a_local_pair: Handle<StepKinematicsRevolutePairWithRange> =
                            a_kinematic_pair.cast::<StepKinematicsRevolutePairWithRange>();
                        step_live(&a_local_pair, "ReadStep")?.init(
                            &a_representation_item_name,
                            &a_item_defined_transformation_name,
                            has_item_defined_transformation_description,
                            &a_item_defined_transformation_description,
                            &a_item_defined_transformation_transform_item1,
                            &a_item_defined_transformation_transform_item2,
                            &a_kinematic_pair_joint,
                            a_low_order_kinematic_pair_tx,
                            a_low_order_kinematic_pair_ty,
                            a_low_order_kinematic_pair_tz,
                            a_low_order_kinematic_pair_rx,
                            a_low_order_kinematic_pair_ry,
                            a_low_order_kinematic_pair_rz,
                            has_lower_limit_actual_rotation,
                            a_lower_limit_actual_rotation,
                            has_upper_limit_actual_rotation,
                            a_upper_limit_actual_rotation,
                        );
                    } else {
                        a_kinematic_pair = Handle::new(StepKinematicsRevolutePair::new()).cast();
                        let a_local_pair: Handle<StepKinematicsRevolutePair> =
                            a_kinematic_pair.cast::<StepKinematicsRevolutePair>();
                        step_live(&a_local_pair, "ReadStep")?.init(
                            &a_representation_item_name,
                            &a_item_defined_transformation_name,
                            has_item_defined_transformation_description,
                            &a_item_defined_transformation_description,
                            &a_item_defined_transformation_transform_item1,
                            &a_item_defined_transformation_transform_item2,
                            &a_kinematic_pair_joint,
                            a_low_order_kinematic_pair_tx,
                            a_low_order_kinematic_pair_ty,
                            a_low_order_kinematic_pair_tz,
                            a_low_order_kinematic_pair_rx,
                            a_low_order_kinematic_pair_ry,
                            a_low_order_kinematic_pair_rz,
                        );
                    }
                }
            } else {
                if a_pairtype == b"CYLINDRICAL_PAIR" {
                    if the_data.named_for_complex_short(
                        "CYLINDRICAL_PAIR_WITH_RANGE",
                        "CPW0",
                        num0,
                        &mut the_num,
                        the_arch,
                    ) {
                        if the_data.check_nb_params(
                            the_num,
                            4,
                            the_arch,
                            "CYLINDRICAL_PAIR_WITH_RANGE",
                        )? {
                            a_kinematic_pair =
                                Handle::new(StepKinematicsCylindricalPairWithRange::new()).cast();
                            let mut a_lower_limit_actual_translation: f64 = Default::default();
                            let mut has_lower_limit_actual_translation: bool = true;
                            if the_data.is_param_defined(the_num, 1)? {
                                the_data.read_real(
                                    the_num,
                                    1,
                                    "lower_limit_actual_translation",
                                    the_arch,
                                    &mut a_lower_limit_actual_translation,
                                )?;
                            } else {
                                has_lower_limit_actual_translation = false;
                                a_lower_limit_actual_translation = 0.0;
                            }
                            let mut a_upper_limit_actual_translation: f64 = Default::default();
                            let mut has_upper_limit_actual_translation: bool = true;
                            if the_data.is_param_defined(the_num, 2)? {
                                the_data.read_real(
                                    the_num,
                                    2,
                                    "upper_limit_actual_translation",
                                    the_arch,
                                    &mut a_upper_limit_actual_translation,
                                )?;
                            } else {
                                has_upper_limit_actual_translation = false;
                                a_upper_limit_actual_translation = 0.0;
                            }
                            let mut a_lower_limit_actual_rotation: f64 = Default::default();
                            let mut has_lower_limit_actual_rotation: bool = true;
                            if the_data.is_param_defined(the_num, 3)? {
                                the_data.read_real(
                                    the_num,
                                    3,
                                    "lower_limit_actual_rotation",
                                    the_arch,
                                    &mut a_lower_limit_actual_rotation,
                                )?;
                            } else {
                                has_lower_limit_actual_rotation = false;
                                a_lower_limit_actual_rotation = 0.0;
                            }
                            let mut a_upper_limit_actual_rotation: f64 = Default::default();
                            let mut has_upper_limit_actual_rotation: bool = true;
                            if the_data.is_param_defined(the_num, 4)? {
                                the_data.read_real(
                                    the_num,
                                    4,
                                    "upper_limit_actual_rotation",
                                    the_arch,
                                    &mut a_upper_limit_actual_rotation,
                                )?;
                            } else {
                                has_upper_limit_actual_rotation = false;
                                a_upper_limit_actual_rotation = 0.0;
                            }
                            let a_local_pair: Handle<StepKinematicsCylindricalPairWithRange> =
                                a_kinematic_pair.cast::<StepKinematicsCylindricalPairWithRange>();
                            step_live(&a_local_pair, "ReadStep")?.init(
                                &a_representation_item_name,
                                &a_item_defined_transformation_name,
                                has_item_defined_transformation_description,
                                &a_item_defined_transformation_description,
                                &a_item_defined_transformation_transform_item1,
                                &a_item_defined_transformation_transform_item2,
                                &a_kinematic_pair_joint,
                                a_low_order_kinematic_pair_tx,
                                a_low_order_kinematic_pair_ty,
                                a_low_order_kinematic_pair_tz,
                                a_low_order_kinematic_pair_rx,
                                a_low_order_kinematic_pair_ry,
                                a_low_order_kinematic_pair_rz,
                                has_lower_limit_actual_translation,
                                a_lower_limit_actual_translation,
                                has_upper_limit_actual_translation,
                                a_upper_limit_actual_translation,
                                has_lower_limit_actual_rotation,
                                a_lower_limit_actual_rotation,
                                has_upper_limit_actual_rotation,
                                a_upper_limit_actual_rotation,
                            );
                        } else {
                            a_kinematic_pair =
                                Handle::new(StepKinematicsCylindricalPair::new()).cast();
                            let a_local_pair: Handle<StepKinematicsCylindricalPair> =
                                a_kinematic_pair.cast::<StepKinematicsCylindricalPair>();
                            step_live(&a_local_pair, "ReadStep")?.init(
                                &a_representation_item_name,
                                &a_item_defined_transformation_name,
                                has_item_defined_transformation_description,
                                &a_item_defined_transformation_description,
                                &a_item_defined_transformation_transform_item1,
                                &a_item_defined_transformation_transform_item2,
                                &a_kinematic_pair_joint,
                                a_low_order_kinematic_pair_tx,
                                a_low_order_kinematic_pair_ty,
                                a_low_order_kinematic_pair_tz,
                                a_low_order_kinematic_pair_rx,
                                a_low_order_kinematic_pair_ry,
                                a_low_order_kinematic_pair_rz,
                            );
                        }
                    }
                } else {
                    if a_pairtype == b"SPHERICAL_PAIR" {
                        if the_data.named_for_complex_short(
                            "SPHERICAL_PAIR_WITH_RANGE",
                            "SPW0",
                            num0,
                            &mut the_num,
                            the_arch,
                        ) {
                            if the_data.check_nb_params(
                                the_num,
                                6,
                                the_arch,
                                "SPHERICAL_PAIR_WITH_RANGE",
                            )? {
                                a_kinematic_pair =
                                    Handle::new(StepKinematicsSphericalPairWithRange::new()).cast();
                                let mut a_lower_limit_yaw: f64 = Default::default();
                                let mut has_lower_limit_yaw: bool = true;
                                if the_data.is_param_defined(the_num, 1)? {
                                    the_data.read_real(
                                        the_num,
                                        1,
                                        "lower_limit_yaw",
                                        the_arch,
                                        &mut a_lower_limit_yaw,
                                    )?;
                                } else {
                                    has_lower_limit_yaw = false;
                                    a_lower_limit_yaw = 0.0;
                                }
                                let mut a_upper_limit_yaw: f64 = Default::default();
                                let mut has_upper_limit_yaw: bool = true;
                                if the_data.is_param_defined(the_num, 2)? {
                                    the_data.read_real(
                                        the_num,
                                        2,
                                        "upper_limit_yaw",
                                        the_arch,
                                        &mut a_upper_limit_yaw,
                                    )?;
                                } else {
                                    has_upper_limit_yaw = false;
                                    a_upper_limit_yaw = 0.0;
                                }
                                let mut a_lower_limit_pitch: f64 = Default::default();
                                let mut has_lower_limit_pitch: bool = true;
                                if the_data.is_param_defined(the_num, 3)? {
                                    the_data.read_real(
                                        the_num,
                                        3,
                                        "lower_limit_pitch",
                                        the_arch,
                                        &mut a_lower_limit_pitch,
                                    )?;
                                } else {
                                    has_lower_limit_pitch = false;
                                    a_lower_limit_pitch = 0.0;
                                }
                                let mut a_upper_limit_pitch: f64 = Default::default();
                                let mut has_upper_limit_pitch: bool = true;
                                if the_data.is_param_defined(the_num, 4)? {
                                    the_data.read_real(
                                        the_num,
                                        4,
                                        "upper_limit_pitch",
                                        the_arch,
                                        &mut a_upper_limit_pitch,
                                    )?;
                                } else {
                                    has_upper_limit_pitch = false;
                                    a_upper_limit_pitch = 0.0;
                                }
                                let mut a_lower_limit_roll: f64 = Default::default();
                                let mut has_lower_limit_roll: bool = true;
                                if the_data.is_param_defined(the_num, 5)? {
                                    the_data.read_real(
                                        the_num,
                                        5,
                                        "lower_limit_roll",
                                        the_arch,
                                        &mut a_lower_limit_roll,
                                    )?;
                                } else {
                                    has_lower_limit_roll = false;
                                    a_lower_limit_roll = 0.0;
                                }
                                let mut a_upper_limit_roll: f64 = Default::default();
                                let mut has_upper_limit_roll: bool = true;
                                if the_data.is_param_defined(the_num, 6)? {
                                    the_data.read_real(
                                        the_num,
                                        6,
                                        "upper_limit_roll",
                                        the_arch,
                                        &mut a_upper_limit_roll,
                                    )?;
                                } else {
                                    has_upper_limit_roll = false;
                                    a_upper_limit_roll = 0.0;
                                }
                                let a_local_pair: Handle<StepKinematicsSphericalPairWithRange> =
                                    a_kinematic_pair.cast::<StepKinematicsSphericalPairWithRange>();
                                step_live(&a_local_pair, "ReadStep")?.init(
                                    &a_representation_item_name,
                                    &a_item_defined_transformation_name,
                                    has_item_defined_transformation_description,
                                    &a_item_defined_transformation_description,
                                    &a_item_defined_transformation_transform_item1,
                                    &a_item_defined_transformation_transform_item2,
                                    &a_kinematic_pair_joint,
                                    a_low_order_kinematic_pair_tx,
                                    a_low_order_kinematic_pair_ty,
                                    a_low_order_kinematic_pair_tz,
                                    a_low_order_kinematic_pair_rx,
                                    a_low_order_kinematic_pair_ry,
                                    a_low_order_kinematic_pair_rz,
                                    has_lower_limit_yaw,
                                    a_lower_limit_yaw,
                                    has_upper_limit_yaw,
                                    a_upper_limit_yaw,
                                    has_lower_limit_pitch,
                                    a_lower_limit_pitch,
                                    has_upper_limit_pitch,
                                    a_upper_limit_pitch,
                                    has_lower_limit_roll,
                                    a_lower_limit_roll,
                                    has_upper_limit_roll,
                                    a_upper_limit_roll,
                                );
                            } else {
                                a_kinematic_pair =
                                    Handle::new(StepKinematicsSphericalPair::new()).cast();
                                let a_local_pair: Handle<StepKinematicsSphericalPair> =
                                    a_kinematic_pair.cast::<StepKinematicsSphericalPair>();
                                step_live(&a_local_pair, "ReadStep")?.init(
                                    &a_representation_item_name,
                                    &a_item_defined_transformation_name,
                                    has_item_defined_transformation_description,
                                    &a_item_defined_transformation_description,
                                    &a_item_defined_transformation_transform_item1,
                                    &a_item_defined_transformation_transform_item2,
                                    &a_kinematic_pair_joint,
                                    a_low_order_kinematic_pair_tx,
                                    a_low_order_kinematic_pair_ty,
                                    a_low_order_kinematic_pair_tz,
                                    a_low_order_kinematic_pair_rx,
                                    a_low_order_kinematic_pair_ry,
                                    a_low_order_kinematic_pair_rz,
                                );
                            }
                        }
                    } else {
                        if a_pairtype == b"SPHERICAL_PAIR_WITH_PIN" {
                            if the_data.named_for_complex_short(
                                "SPHERICAL_PAIR_WITH_PIN_AND_RANGE",
                                "SPWPAR",
                                num0,
                                &mut the_num,
                                the_arch,
                            ) {
                                if the_data.check_nb_params(
                                    the_num,
                                    4,
                                    the_arch,
                                    "SPHERICAL_PAIR_WITH_PIN_AND_RANGE",
                                )? {
                                    a_kinematic_pair = Handle::new(
                                        StepKinematicsSphericalPairWithPinAndRange::new(),
                                    )
                                    .cast();
                                    let mut a_lower_limit_yaw: f64 = Default::default();
                                    let mut has_lower_limit_yaw: bool = true;
                                    if the_data.is_param_defined(the_num, 1)? {
                                        the_data.read_real(
                                            the_num,
                                            1,
                                            "lower_limit_yaw",
                                            the_arch,
                                            &mut a_lower_limit_yaw,
                                        )?;
                                    } else {
                                        has_lower_limit_yaw = false;
                                        a_lower_limit_yaw = 0.0;
                                    }
                                    let mut a_upper_limit_yaw: f64 = Default::default();
                                    let mut has_upper_limit_yaw: bool = true;
                                    if the_data.is_param_defined(the_num, 2)? {
                                        the_data.read_real(
                                            the_num,
                                            2,
                                            "upper_limit_yaw",
                                            the_arch,
                                            &mut a_upper_limit_yaw,
                                        )?;
                                    } else {
                                        has_upper_limit_yaw = false;
                                        a_upper_limit_yaw = 0.0;
                                    }
                                    let mut a_lower_limit_roll: f64 = Default::default();
                                    let mut has_lower_limit_roll: bool = true;
                                    if the_data.is_param_defined(the_num, 3)? {
                                        the_data.read_real(
                                            the_num,
                                            3,
                                            "lower_limit_roll",
                                            the_arch,
                                            &mut a_lower_limit_roll,
                                        )?;
                                    } else {
                                        has_lower_limit_roll = false;
                                        a_lower_limit_roll = 0.0;
                                    }
                                    let mut a_upper_limit_roll: f64 = Default::default();
                                    let mut has_upper_limit_roll: bool = true;
                                    if the_data.is_param_defined(the_num, 4)? {
                                        the_data.read_real(
                                            the_num,
                                            4,
                                            "upper_limit_roll",
                                            the_arch,
                                            &mut a_upper_limit_roll,
                                        )?;
                                    } else {
                                        has_upper_limit_roll = false;
                                        a_upper_limit_roll = 0.0;
                                    }
                                    let a_local_pair: Handle<
                                        StepKinematicsSphericalPairWithPinAndRange,
                                    > = a_kinematic_pair
                                        .cast::<StepKinematicsSphericalPairWithPinAndRange>();
                                    step_live(&a_local_pair, "ReadStep")?.init(
                                        &a_representation_item_name,
                                        &a_item_defined_transformation_name,
                                        has_item_defined_transformation_description,
                                        &a_item_defined_transformation_description,
                                        &a_item_defined_transformation_transform_item1,
                                        &a_item_defined_transformation_transform_item2,
                                        &a_kinematic_pair_joint,
                                        a_low_order_kinematic_pair_tx,
                                        a_low_order_kinematic_pair_ty,
                                        a_low_order_kinematic_pair_tz,
                                        a_low_order_kinematic_pair_rx,
                                        a_low_order_kinematic_pair_ry,
                                        a_low_order_kinematic_pair_rz,
                                        has_lower_limit_yaw,
                                        a_lower_limit_yaw,
                                        has_upper_limit_yaw,
                                        a_upper_limit_yaw,
                                        has_lower_limit_roll,
                                        a_lower_limit_roll,
                                        has_upper_limit_roll,
                                        a_upper_limit_roll,
                                    );
                                } else {
                                    a_kinematic_pair =
                                        Handle::new(StepKinematicsSphericalPairWithPin::new())
                                            .cast();
                                    let a_local_pair: Handle<StepKinematicsSphericalPairWithPin> =
                                        a_kinematic_pair
                                            .cast::<StepKinematicsSphericalPairWithPin>();
                                    step_live(&a_local_pair, "ReadStep")?.init(
                                        &a_representation_item_name,
                                        &a_item_defined_transformation_name,
                                        has_item_defined_transformation_description,
                                        &a_item_defined_transformation_description,
                                        &a_item_defined_transformation_transform_item1,
                                        &a_item_defined_transformation_transform_item2,
                                        &a_kinematic_pair_joint,
                                        a_low_order_kinematic_pair_tx,
                                        a_low_order_kinematic_pair_ty,
                                        a_low_order_kinematic_pair_tz,
                                        a_low_order_kinematic_pair_rx,
                                        a_low_order_kinematic_pair_ry,
                                        a_low_order_kinematic_pair_rz,
                                    );
                                }
                            } else {
                                the_arch.add_fail_text(
                                    "The type of kinematic pair with range is not supported",
                                    "",
                                );
                            }
                        } else {
                            if a_pairtype == b"PRISMATIC_PAIR" {
                                if the_data.named_for_complex_short(
                                    "PRISMATIC_PAIR_WITH_RANGE",
                                    "PPW0",
                                    num0,
                                    &mut the_num,
                                    the_arch,
                                ) {
                                    if the_data.check_nb_params(
                                        the_num,
                                        2,
                                        the_arch,
                                        "PRISMATIC_PAIR_WITH_RANGE",
                                    )? {
                                        a_kinematic_pair = Handle::new(
                                            StepKinematicsPrismaticPairWithRange::new(),
                                        )
                                        .cast();
                                        let mut a_lower_limit_actual_translation: f64 =
                                            Default::default();
                                        let mut has_lower_limit_actual_translation: bool = true;
                                        if the_data.is_param_defined(the_num, 1)? {
                                            the_data.read_real(
                                                the_num,
                                                1,
                                                "lower_limit_actual_translation",
                                                the_arch,
                                                &mut a_lower_limit_actual_translation,
                                            )?;
                                        } else {
                                            has_lower_limit_actual_translation = false;
                                            a_lower_limit_actual_translation = 0.0;
                                        }
                                        let mut a_upper_limit_actual_translation: f64 =
                                            Default::default();
                                        let mut has_upper_limit_actual_translation: bool = true;
                                        if the_data.is_param_defined(the_num, 2)? {
                                            the_data.read_real(
                                                the_num,
                                                2,
                                                "upper_limit_actual_translation",
                                                the_arch,
                                                &mut a_upper_limit_actual_translation,
                                            )?;
                                        } else {
                                            has_upper_limit_actual_translation = false;
                                            a_upper_limit_actual_translation = 0.0;
                                        }
                                        let a_local_pair: Handle<
                                            StepKinematicsPrismaticPairWithRange,
                                        > = a_kinematic_pair
                                            .cast::<StepKinematicsPrismaticPairWithRange>();
                                        step_live(&a_local_pair, "ReadStep")?.init(
                                            &a_representation_item_name,
                                            &a_item_defined_transformation_name,
                                            has_item_defined_transformation_description,
                                            &a_item_defined_transformation_description,
                                            &a_item_defined_transformation_transform_item1,
                                            &a_item_defined_transformation_transform_item2,
                                            &a_kinematic_pair_joint,
                                            a_low_order_kinematic_pair_tx,
                                            a_low_order_kinematic_pair_ty,
                                            a_low_order_kinematic_pair_tz,
                                            a_low_order_kinematic_pair_rx,
                                            a_low_order_kinematic_pair_ry,
                                            a_low_order_kinematic_pair_rz,
                                            has_lower_limit_actual_translation,
                                            a_lower_limit_actual_translation,
                                            has_upper_limit_actual_translation,
                                            a_upper_limit_actual_translation,
                                        );
                                    } else {
                                        a_kinematic_pair =
                                            Handle::new(StepKinematicsPrismaticPair::new()).cast();
                                        let a_local_pair: Handle<StepKinematicsPrismaticPair> =
                                            a_kinematic_pair.cast::<StepKinematicsPrismaticPair>();
                                        step_live(&a_local_pair, "ReadStep")?.init(
                                            &a_representation_item_name,
                                            &a_item_defined_transformation_name,
                                            has_item_defined_transformation_description,
                                            &a_item_defined_transformation_description,
                                            &a_item_defined_transformation_transform_item1,
                                            &a_item_defined_transformation_transform_item2,
                                            &a_kinematic_pair_joint,
                                            a_low_order_kinematic_pair_tx,
                                            a_low_order_kinematic_pair_ty,
                                            a_low_order_kinematic_pair_tz,
                                            a_low_order_kinematic_pair_rx,
                                            a_low_order_kinematic_pair_ry,
                                            a_low_order_kinematic_pair_rz,
                                        );
                                    }
                                }
                            } else {
                                if a_pairtype == b"UNIVERSAL_PAIR" {
                                    a_kinematic_pair =
                                        Handle::new(StepKinematicsUniversalPair::new()).cast();
                                    let mut a_input_skew_angle: f64 = Default::default();
                                    let mut has_input_skew_angle: bool = true;
                                    if the_data.is_param_defined(the_num, 1)? {
                                        the_data.read_real(
                                            the_num,
                                            1,
                                            "input_skew_angle",
                                            the_arch,
                                            &mut a_input_skew_angle,
                                        )?;
                                    } else {
                                        has_input_skew_angle = false;
                                        a_input_skew_angle = 0.0;
                                    }
                                    if the_data.named_for_complex_short(
                                        "UNIVERSAL_PAIR_WITH_RANGE",
                                        "UPWR",
                                        num0,
                                        &mut the_num,
                                        the_arch,
                                    ) {
                                        if the_data.check_nb_params(
                                            the_num,
                                            4,
                                            the_arch,
                                            "UNIVERSAL_PAIR_WITH_RANGE",
                                        )? {
                                            let mut a_lower_limit_first_rotation: f64 =
                                                Default::default();
                                            let mut has_lower_limit_first_rotation: bool = true;
                                            if the_data.is_param_defined(the_num, 1)? {
                                                the_data.read_real(
                                                    the_num,
                                                    1,
                                                    "lower_limit_first_rotation",
                                                    the_arch,
                                                    &mut a_lower_limit_first_rotation,
                                                )?;
                                            } else {
                                                has_lower_limit_first_rotation = false;
                                                a_lower_limit_first_rotation = 0.0;
                                            }
                                            let mut a_upper_limit_first_rotation: f64 =
                                                Default::default();
                                            let mut has_upper_limit_first_rotation: bool = true;
                                            if the_data.is_param_defined(the_num, 2)? {
                                                the_data.read_real(
                                                    the_num,
                                                    2,
                                                    "upper_limit_first_rotation",
                                                    the_arch,
                                                    &mut a_upper_limit_first_rotation,
                                                )?;
                                            } else {
                                                has_upper_limit_first_rotation = false;
                                                a_upper_limit_first_rotation = 0.0;
                                            }
                                            let mut a_lower_limit_second_rotation: f64 =
                                                Default::default();
                                            let mut has_lower_limit_second_rotation: bool = true;
                                            if the_data.is_param_defined(the_num, 3)? {
                                                the_data.read_real(
                                                    the_num,
                                                    3,
                                                    "lower_limit_second_rotation",
                                                    the_arch,
                                                    &mut a_lower_limit_second_rotation,
                                                )?;
                                            } else {
                                                has_lower_limit_second_rotation = false;
                                                a_lower_limit_second_rotation = 0.0;
                                            }
                                            let mut a_upper_limit_second_rotation: f64 =
                                                Default::default();
                                            let mut has_upper_limit_second_rotation: bool = true;
                                            if the_data.is_param_defined(the_num, 4)? {
                                                the_data.read_real(
                                                    the_num,
                                                    4,
                                                    "upper_limit_second_rotation",
                                                    the_arch,
                                                    &mut a_upper_limit_second_rotation,
                                                )?;
                                            } else {
                                                has_upper_limit_second_rotation = false;
                                                a_upper_limit_second_rotation = 0.0;
                                            }
                                            a_kinematic_pair = Handle::new(
                                                StepKinematicsUniversalPairWithRange::new(),
                                            )
                                            .cast();
                                            let a_local_pair: Handle<
                                                StepKinematicsUniversalPairWithRange,
                                            > = a_kinematic_pair
                                                .cast::<StepKinematicsUniversalPairWithRange>();
                                            step_live(&a_local_pair, "ReadStep")?.init(
                                                &a_representation_item_name,
                                                &a_item_defined_transformation_name,
                                                has_item_defined_transformation_description,
                                                &a_item_defined_transformation_description,
                                                &a_item_defined_transformation_transform_item1,
                                                &a_item_defined_transformation_transform_item2,
                                                &a_kinematic_pair_joint,
                                                a_low_order_kinematic_pair_tx,
                                                a_low_order_kinematic_pair_ty,
                                                a_low_order_kinematic_pair_tz,
                                                a_low_order_kinematic_pair_rx,
                                                a_low_order_kinematic_pair_ry,
                                                a_low_order_kinematic_pair_rz,
                                                has_input_skew_angle,
                                                a_input_skew_angle,
                                                has_lower_limit_first_rotation,
                                                a_lower_limit_first_rotation,
                                                has_upper_limit_first_rotation,
                                                a_upper_limit_first_rotation,
                                                has_lower_limit_second_rotation,
                                                a_lower_limit_second_rotation,
                                                has_upper_limit_second_rotation,
                                                a_upper_limit_second_rotation,
                                            );
                                        } else {
                                            a_kinematic_pair =
                                                Handle::new(StepKinematicsUniversalPair::new())
                                                    .cast();
                                            let a_local_pair: Handle<StepKinematicsUniversalPair> =
                                                a_kinematic_pair
                                                    .cast::<StepKinematicsUniversalPair>();
                                            step_live(&a_local_pair, "ReadStep")?.init(
                                                &a_representation_item_name,
                                                &a_item_defined_transformation_name,
                                                has_item_defined_transformation_description,
                                                &a_item_defined_transformation_description,
                                                &a_item_defined_transformation_transform_item1,
                                                &a_item_defined_transformation_transform_item2,
                                                &a_kinematic_pair_joint,
                                                a_low_order_kinematic_pair_tx,
                                                a_low_order_kinematic_pair_ty,
                                                a_low_order_kinematic_pair_tz,
                                                a_low_order_kinematic_pair_rx,
                                                a_low_order_kinematic_pair_ry,
                                                a_low_order_kinematic_pair_rz,
                                                has_input_skew_angle,
                                                a_input_skew_angle,
                                            );
                                        }
                                    }
                                } else {
                                    if a_pairtype == b"PLANAR_PAIR"
                                        && the_data.named_for_complex_short(
                                            "PLANAR_PAIR_WITH_RANGE",
                                            "PPWR",
                                            num0,
                                            &mut the_num,
                                            the_arch,
                                        )
                                    {
                                        if the_data.check_nb_params(
                                            the_num,
                                            6,
                                            the_arch,
                                            "UNIVERSAL_PAIR_WITH_RANGE",
                                        )? {
                                            a_kinematic_pair = Handle::new(
                                                StepKinematicsPlanarPairWithRange::new(),
                                            )
                                            .cast();
                                            let mut a_lower_limit_actual_rotation: f64 =
                                                Default::default();
                                            let mut has_lower_limit_actual_rotation: bool = true;
                                            if the_data.is_param_defined(the_num, 1)? {
                                                the_data.read_real(
                                                    the_num,
                                                    1,
                                                    "lower_limit_actual_rotation",
                                                    the_arch,
                                                    &mut a_lower_limit_actual_rotation,
                                                )?;
                                            } else {
                                                has_lower_limit_actual_rotation = false;
                                                a_lower_limit_actual_rotation = 0.0;
                                            }
                                            let mut a_upper_limit_actual_rotation: f64 =
                                                Default::default();
                                            let mut has_upper_limit_actual_rotation: bool = true;
                                            if the_data.is_param_defined(the_num, 2)? {
                                                the_data.read_real(
                                                    the_num,
                                                    2,
                                                    "upper_limit_actual_rotation",
                                                    the_arch,
                                                    &mut a_upper_limit_actual_rotation,
                                                )?;
                                            } else {
                                                has_upper_limit_actual_rotation = false;
                                                a_upper_limit_actual_rotation = 0.0;
                                            }
                                            let mut a_lower_limit_actual_translation_x: f64 =
                                                Default::default();
                                            let mut has_lower_limit_actual_translation_x: bool =
                                                true;
                                            if the_data.is_param_defined(the_num, 3)? {
                                                the_data.read_real(
                                                    the_num,
                                                    3,
                                                    "lower_limit_actual_translation_x",
                                                    the_arch,
                                                    &mut a_lower_limit_actual_translation_x,
                                                )?;
                                            } else {
                                                has_lower_limit_actual_translation_x = false;
                                                a_lower_limit_actual_translation_x = 0.0;
                                            }
                                            let mut a_upper_limit_actual_translation_x: f64 =
                                                Default::default();
                                            let mut has_upper_limit_actual_translation_x: bool =
                                                true;
                                            if the_data.is_param_defined(the_num, 4)? {
                                                the_data.read_real(
                                                    the_num,
                                                    4,
                                                    "upper_limit_actual_translation_x",
                                                    the_arch,
                                                    &mut a_upper_limit_actual_translation_x,
                                                )?;
                                            } else {
                                                has_upper_limit_actual_translation_x = false;
                                                a_upper_limit_actual_translation_x = 0.0;
                                            }
                                            let mut a_lower_limit_actual_translation_y: f64 =
                                                Default::default();
                                            let mut has_lower_limit_actual_translation_y: bool =
                                                true;
                                            if the_data.is_param_defined(the_num, 5)? {
                                                the_data.read_real(
                                                    the_num,
                                                    5,
                                                    "lower_limit_actual_translation_y",
                                                    the_arch,
                                                    &mut a_lower_limit_actual_translation_y,
                                                )?;
                                            } else {
                                                has_lower_limit_actual_translation_y = false;
                                                a_lower_limit_actual_translation_y = 0.0;
                                            }
                                            let mut a_upper_limit_actual_translation_y: f64 =
                                                Default::default();
                                            let mut has_upper_limit_actual_translation_y: bool =
                                                true;
                                            if the_data.is_param_defined(the_num, 6)? {
                                                the_data.read_real(
                                                    the_num,
                                                    6,
                                                    "upper_limit_actual_translation_y",
                                                    the_arch,
                                                    &mut a_upper_limit_actual_translation_y,
                                                )?;
                                            } else {
                                                has_upper_limit_actual_translation_y = false;
                                                a_upper_limit_actual_translation_y = 0.0;
                                            }
                                            let a_local_pair: Handle<
                                                StepKinematicsPlanarPairWithRange,
                                            > = a_kinematic_pair
                                                .cast::<StepKinematicsPlanarPairWithRange>();
                                            step_live(&a_local_pair, "ReadStep")?.init(
                                                &a_representation_item_name,
                                                &a_item_defined_transformation_name,
                                                has_item_defined_transformation_description,
                                                &a_item_defined_transformation_description,
                                                &a_item_defined_transformation_transform_item1,
                                                &a_item_defined_transformation_transform_item2,
                                                &a_kinematic_pair_joint,
                                                a_low_order_kinematic_pair_tx,
                                                a_low_order_kinematic_pair_ty,
                                                a_low_order_kinematic_pair_tz,
                                                a_low_order_kinematic_pair_rx,
                                                a_low_order_kinematic_pair_ry,
                                                a_low_order_kinematic_pair_rz,
                                                has_lower_limit_actual_rotation,
                                                a_lower_limit_actual_rotation,
                                                has_upper_limit_actual_rotation,
                                                a_upper_limit_actual_rotation,
                                                has_lower_limit_actual_translation_x,
                                                a_lower_limit_actual_translation_x,
                                                has_upper_limit_actual_translation_x,
                                                a_upper_limit_actual_translation_x,
                                                has_lower_limit_actual_translation_y,
                                                a_lower_limit_actual_translation_y,
                                                has_upper_limit_actual_translation_y,
                                                a_upper_limit_actual_translation_y,
                                            );
                                        } else {
                                            a_kinematic_pair =
                                                Handle::new(StepKinematicsPlanarPair::new()).cast();
                                            let a_local_pair: Handle<StepKinematicsPlanarPair> =
                                                a_kinematic_pair.cast::<StepKinematicsPlanarPair>();
                                            step_live(&a_local_pair, "ReadStep")?.init(
                                                &a_representation_item_name,
                                                &a_item_defined_transformation_name,
                                                has_item_defined_transformation_description,
                                                &a_item_defined_transformation_description,
                                                &a_item_defined_transformation_transform_item1,
                                                &a_item_defined_transformation_transform_item2,
                                                &a_kinematic_pair_joint,
                                                a_low_order_kinematic_pair_tx,
                                                a_low_order_kinematic_pair_ty,
                                                a_low_order_kinematic_pair_tz,
                                                a_low_order_kinematic_pair_rx,
                                                a_low_order_kinematic_pair_ry,
                                                a_low_order_kinematic_pair_rz,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if a_pair_super_type == b"LOW_ORDER_KINEMATIC_PAIR_WITH_MOTION_COUPLING" {
                the_num = the_data.next_for_complex(the_num);
                let a_pairtype: Vec<u8> = the_data.c_type(the_num).into_bytes();
                if a_pairtype == b"GEAR_PAIR" {
                    let mut a_gear_pair_radius_first_link: f64 = Default::default();
                    the_data.read_real(
                        the_num,
                        1,
                        "gear_pair.radius_first_link",
                        the_arch,
                        &mut a_gear_pair_radius_first_link,
                    )?;
                    let mut a_gear_pair_radius_second_link: f64 = Default::default();
                    the_data.read_real(
                        the_num,
                        2,
                        "gear_pair.radius_second_link",
                        the_arch,
                        &mut a_gear_pair_radius_second_link,
                    )?;
                    let mut a_gear_pair_bevel: f64 = Default::default();
                    the_data.read_real(
                        the_num,
                        3,
                        "gear_pair.bevel",
                        the_arch,
                        &mut a_gear_pair_bevel,
                    )?;
                    let mut a_gear_pair_helical_angle: f64 = Default::default();
                    the_data.read_real(
                        the_num,
                        4,
                        "gear_pair.helical_angle",
                        the_arch,
                        &mut a_gear_pair_helical_angle,
                    )?;
                    let mut a_gear_pair_gear_ratio: f64 = Default::default();
                    the_data.read_real(
                        the_num,
                        5,
                        "gear_pair.gear_ratio",
                        the_arch,
                        &mut a_gear_pair_gear_ratio,
                    )?;
                    if the_data.named_for_complex_short(
                        "GEAR_PAIR_WITH_RANGE",
                        "GPWR",
                        num0,
                        &mut the_num,
                        the_arch,
                    ) {
                        if the_data.check_nb_params(the_num, 2, the_arch, "GEAR_PAIR_WITH_RANGE")? {
                            a_kinematic_pair =
                                Handle::new(StepKinematicsGearPairWithRange::new()).cast();
                            let mut a_lower_limit_actual_rotation1: f64 = Default::default();
                            let mut has_lower_limit_actual_rotation1: bool = true;
                            if the_data.is_param_defined(the_num, 1)? {
                                the_data.read_real(
                                    the_num,
                                    1,
                                    "lower_limit_actual_rotation1",
                                    the_arch,
                                    &mut a_lower_limit_actual_rotation1,
                                )?;
                            } else {
                                has_lower_limit_actual_rotation1 = false;
                                a_lower_limit_actual_rotation1 = 0.0;
                            }
                            let mut a_upper_limit_actual_rotation1: f64 = Default::default();
                            let mut has_upper_limit_actual_rotation1: bool = true;
                            if the_data.is_param_defined(the_num, 2)? {
                                the_data.read_real(
                                    the_num,
                                    2,
                                    "upper_limit_actual_rotation1",
                                    the_arch,
                                    &mut a_upper_limit_actual_rotation1,
                                )?;
                            } else {
                                has_upper_limit_actual_rotation1 = false;
                                a_upper_limit_actual_rotation1 = 0.0;
                            }
                            let a_local_pair: Handle<StepKinematicsGearPairWithRange> =
                                a_kinematic_pair.cast::<StepKinematicsGearPairWithRange>();
                            step_live(&a_local_pair, "ReadStep")?.init(
                                &a_representation_item_name,
                                &a_item_defined_transformation_name,
                                has_item_defined_transformation_description,
                                &a_item_defined_transformation_description,
                                &a_item_defined_transformation_transform_item1,
                                &a_item_defined_transformation_transform_item2,
                                &a_kinematic_pair_joint,
                                a_gear_pair_radius_first_link,
                                a_gear_pair_radius_second_link,
                                a_gear_pair_bevel,
                                a_gear_pair_helical_angle,
                                a_gear_pair_gear_ratio,
                                has_lower_limit_actual_rotation1,
                                a_lower_limit_actual_rotation1,
                                has_upper_limit_actual_rotation1,
                                a_upper_limit_actual_rotation1,
                            );
                        } else {
                            a_kinematic_pair = Handle::new(StepKinematicsGearPair::new()).cast();
                            let a_local_pair: Handle<StepKinematicsGearPair> =
                                a_kinematic_pair.cast::<StepKinematicsGearPair>();
                            step_live(&a_local_pair, "ReadStep")?.init(
                                &a_representation_item_name,
                                &a_item_defined_transformation_name,
                                has_item_defined_transformation_description,
                                &a_item_defined_transformation_description,
                                &a_item_defined_transformation_transform_item1,
                                &a_item_defined_transformation_transform_item2,
                                &a_kinematic_pair_joint,
                                a_gear_pair_radius_first_link,
                                a_gear_pair_radius_second_link,
                                a_gear_pair_bevel,
                                a_gear_pair_helical_angle,
                                a_gear_pair_gear_ratio,
                            );
                        }
                    }
                } else {
                    if a_pairtype == b"RACK_AND_PINION_PAIR" {
                        let mut a_rack_and_pinion_pair_pinion_radius: f64 = Default::default();
                        the_data.read_real(
                            the_num,
                            1,
                            "rack_and_pinion_pair.pinion_radius",
                            the_arch,
                            &mut a_rack_and_pinion_pair_pinion_radius,
                        )?;
                        if the_data.named_for_complex_short(
                            "RACK_AND_PINION_PAIR_WITH_RANGE",
                            "RAPPWR",
                            num0,
                            &mut the_num,
                            the_arch,
                        ) {
                            if the_data.check_nb_params(
                                the_num,
                                2,
                                the_arch,
                                "RACK_AND_PINION_PAIR_WITH_RANGE",
                            )? {
                                a_kinematic_pair =
                                    Handle::new(StepKinematicsRackAndPinionPairWithRange::new())
                                        .cast();
                                let mut a_lower_limit_rack_displacement: f64 = Default::default();
                                let mut has_lower_limit_rack_displacement: bool = true;
                                if the_data.is_param_defined(the_num, 1)? {
                                    the_data.read_real(
                                        the_num,
                                        1,
                                        "lower_limit_rack_displacement",
                                        the_arch,
                                        &mut a_lower_limit_rack_displacement,
                                    )?;
                                } else {
                                    has_lower_limit_rack_displacement = false;
                                    a_lower_limit_rack_displacement = 0.0;
                                }
                                let mut a_upper_limit_rack_displacement: f64 = Default::default();
                                let mut has_upper_limit_rack_displacement: bool = true;
                                if the_data.is_param_defined(the_num, 2)? {
                                    the_data.read_real(
                                        the_num,
                                        2,
                                        "upper_limit_rack_displacement",
                                        the_arch,
                                        &mut a_upper_limit_rack_displacement,
                                    )?;
                                } else {
                                    has_upper_limit_rack_displacement = false;
                                    a_upper_limit_rack_displacement = 0.0;
                                }
                                let a_local_pair: Handle<StepKinematicsRackAndPinionPairWithRange> =
                                    a_kinematic_pair
                                        .cast::<StepKinematicsRackAndPinionPairWithRange>();
                                step_live(&a_local_pair, "ReadStep")?.init(
                                    &a_representation_item_name,
                                    &a_item_defined_transformation_name,
                                    has_item_defined_transformation_description,
                                    &a_item_defined_transformation_description,
                                    &a_item_defined_transformation_transform_item1,
                                    &a_item_defined_transformation_transform_item2,
                                    &a_kinematic_pair_joint,
                                    a_rack_and_pinion_pair_pinion_radius,
                                    has_lower_limit_rack_displacement,
                                    a_lower_limit_rack_displacement,
                                    has_upper_limit_rack_displacement,
                                    a_upper_limit_rack_displacement,
                                );
                            } else {
                                a_kinematic_pair =
                                    Handle::new(StepKinematicsRackAndPinionPair::new()).cast();
                                let a_local_pair: Handle<StepKinematicsRackAndPinionPair> =
                                    a_kinematic_pair.cast::<StepKinematicsRackAndPinionPair>();
                                step_live(&a_local_pair, "ReadStep")?.init(
                                    &a_representation_item_name,
                                    &a_item_defined_transformation_name,
                                    has_item_defined_transformation_description,
                                    &a_item_defined_transformation_description,
                                    &a_item_defined_transformation_transform_item1,
                                    &a_item_defined_transformation_transform_item2,
                                    &a_kinematic_pair_joint,
                                    a_rack_and_pinion_pair_pinion_radius,
                                );
                            }
                        }
                    } else {
                        if a_pairtype == b"SCREW_PAIR" {
                            let mut a_screw_pair_pitch: f64 = Default::default();
                            the_data.read_real(
                                the_num,
                                1,
                                "screw_pair.pitch",
                                the_arch,
                                &mut a_screw_pair_pitch,
                            )?;
                            if the_data.named_for_complex_short(
                                "SCREW_PAIR_WITH_RANGE",
                                "SPWR",
                                num0,
                                &mut the_num,
                                the_arch,
                            ) {
                                if the_data.check_nb_params(
                                    the_num,
                                    2,
                                    the_arch,
                                    "SCREW_PAIR_WITH_RANGE",
                                )? {
                                    a_kinematic_pair =
                                        Handle::new(StepKinematicsScrewPairWithRange::new()).cast();
                                    let mut a_lower_limit_actual_rotation: f64 = Default::default();
                                    let mut has_lower_limit_actual_rotation: bool = true;
                                    if the_data.is_param_defined(the_num, 1)? {
                                        the_data.read_real(
                                            the_num,
                                            1,
                                            "lower_limit_actual_rotation",
                                            the_arch,
                                            &mut a_lower_limit_actual_rotation,
                                        )?;
                                    } else {
                                        has_lower_limit_actual_rotation = false;
                                        a_lower_limit_actual_rotation = 0.0;
                                    }
                                    let mut a_upper_limit_actual_rotation: f64 = Default::default();
                                    let mut has_upper_limit_actual_rotation: bool = true;
                                    if the_data.is_param_defined(the_num, 2)? {
                                        the_data.read_real(
                                            the_num,
                                            2,
                                            "upper_limit_actual_rotation",
                                            the_arch,
                                            &mut a_upper_limit_actual_rotation,
                                        )?;
                                    } else {
                                        has_upper_limit_actual_rotation = false;
                                        a_upper_limit_actual_rotation = 0.0;
                                    }
                                    let a_local_pair: Handle<StepKinematicsScrewPairWithRange> =
                                        a_kinematic_pair.cast::<StepKinematicsScrewPairWithRange>();
                                    step_live(&a_local_pair, "ReadStep")?.init(
                                        &a_representation_item_name,
                                        &a_item_defined_transformation_name,
                                        has_item_defined_transformation_description,
                                        &a_item_defined_transformation_description,
                                        &a_item_defined_transformation_transform_item1,
                                        &a_item_defined_transformation_transform_item2,
                                        &a_kinematic_pair_joint,
                                        a_screw_pair_pitch,
                                        has_lower_limit_actual_rotation,
                                        a_lower_limit_actual_rotation,
                                        has_upper_limit_actual_rotation,
                                        a_upper_limit_actual_rotation,
                                    );
                                } else {
                                    a_kinematic_pair =
                                        Handle::new(StepKinematicsScrewPair::new()).cast();
                                    let a_local_pair: Handle<StepKinematicsScrewPair> =
                                        a_kinematic_pair.cast::<StepKinematicsScrewPair>();
                                    step_live(&a_local_pair, "ReadStep")?.init(
                                        &a_representation_item_name,
                                        &a_item_defined_transformation_name,
                                        has_item_defined_transformation_description,
                                        &a_item_defined_transformation_description,
                                        &a_item_defined_transformation_transform_item1,
                                        &a_item_defined_transformation_transform_item2,
                                        &a_kinematic_pair_joint,
                                        a_screw_pair_pitch,
                                    );
                                }
                            }
                        }
                    }
                }
            } else {
                the_num = the_data.next_for_complex(the_num);
                let a_pairtype: Vec<u8> = the_data.c_type(the_num).into_bytes();
                if a_pairtype == b"POINT_ON_PLANAR_CURVE_PAIR" {
                    let mut a_point_on_planar_curve_pair_pair_curve: Handle<StepGeomCurve> =
                        Default::default();
                    the_data.read_entity(
                        the_num,
                        1,
                        "point_on_planar_curve_pair.pair_curve",
                        the_arch,
                        StepGeomCurve::static_type(),
                        &mut a_point_on_planar_curve_pair_pair_curve,
                    )?;
                    let mut a_point_on_planar_curve_pair_orientation: bool = Default::default();
                    the_data.read_boolean(
                        the_num,
                        2,
                        "point_on_planar_curve_pair.orientation",
                        the_arch,
                        &mut a_point_on_planar_curve_pair_orientation,
                    )?;
                    if the_data.named_for_complex_short(
                        "POINT_ON_PLANAR_CURVE_PAIR_WITH_RANGE",
                        "POPCPW",
                        num0,
                        &mut the_num,
                        the_arch,
                    ) {
                        if the_data.check_nb_params(
                            the_num,
                            7,
                            the_arch,
                            "POINT_ON_PLANAR_CURVE_PAIR_WITH_RANGE",
                        )? {
                            a_kinematic_pair =
                                Handle::new(StepKinematicsPointOnPlanarCurvePairWithRange::new())
                                    .cast();
                            let mut a_range_on_pair_curve: Handle<StepGeomTrimmedCurve> =
                                Default::default();
                            the_data.read_entity(
                                the_num,
                                1,
                                "range_on_pair_curve",
                                the_arch,
                                StepGeomTrimmedCurve::static_type(),
                                &mut a_range_on_pair_curve,
                            )?;
                            let mut a_lower_limit_yaw: f64 = Default::default();
                            let mut has_lower_limit_yaw: bool = true;
                            if the_data.is_param_defined(the_num, 2)? {
                                the_data.read_real(
                                    the_num,
                                    2,
                                    "lower_limit_yaw",
                                    the_arch,
                                    &mut a_lower_limit_yaw,
                                )?;
                            } else {
                                has_lower_limit_yaw = false;
                                a_lower_limit_yaw = 0.0;
                            }
                            let mut a_upper_limit_yaw: f64 = Default::default();
                            let mut has_upper_limit_yaw: bool = true;
                            if the_data.is_param_defined(the_num, 3)? {
                                the_data.read_real(
                                    the_num,
                                    3,
                                    "upper_limit_yaw",
                                    the_arch,
                                    &mut a_upper_limit_yaw,
                                )?;
                            } else {
                                has_upper_limit_yaw = false;
                                a_upper_limit_yaw = 0.0;
                            }
                            let mut a_lower_limit_pitch: f64 = Default::default();
                            let mut has_lower_limit_pitch: bool = true;
                            if the_data.is_param_defined(the_num, 4)? {
                                the_data.read_real(
                                    the_num,
                                    4,
                                    "lower_limit_pitch",
                                    the_arch,
                                    &mut a_lower_limit_pitch,
                                )?;
                            } else {
                                has_lower_limit_pitch = false;
                                a_lower_limit_pitch = 0.0;
                            }
                            let mut a_upper_limit_pitch: f64 = Default::default();
                            let mut has_upper_limit_pitch: bool = true;
                            if the_data.is_param_defined(the_num, 5)? {
                                the_data.read_real(
                                    the_num,
                                    5,
                                    "upper_limit_pitch",
                                    the_arch,
                                    &mut a_upper_limit_pitch,
                                )?;
                            } else {
                                has_upper_limit_pitch = false;
                                a_upper_limit_pitch = 0.0;
                            }
                            let mut a_lower_limit_roll: f64 = Default::default();
                            let mut has_lower_limit_roll: bool = true;
                            if the_data.is_param_defined(the_num, 6)? {
                                the_data.read_real(
                                    the_num,
                                    6,
                                    "lower_limit_roll",
                                    the_arch,
                                    &mut a_lower_limit_roll,
                                )?;
                            } else {
                                has_lower_limit_roll = false;
                                a_lower_limit_roll = 0.0;
                            }
                            let mut a_upper_limit_roll: f64 = Default::default();
                            let mut has_upper_limit_roll: bool = true;
                            if the_data.is_param_defined(the_num, 7)? {
                                the_data.read_real(
                                    the_num,
                                    7,
                                    "upper_limit_roll",
                                    the_arch,
                                    &mut a_upper_limit_roll,
                                )?;
                            } else {
                                has_upper_limit_roll = false;
                                a_upper_limit_roll = 0.0;
                            }
                            let a_local_pair: Handle<
                                StepKinematicsPointOnPlanarCurvePairWithRange,
                            > = a_kinematic_pair
                                .cast::<StepKinematicsPointOnPlanarCurvePairWithRange>();
                            step_live(&a_local_pair, "ReadStep")?.init(
                                &a_representation_item_name,
                                &a_item_defined_transformation_name,
                                has_item_defined_transformation_description,
                                &a_item_defined_transformation_description,
                                &a_item_defined_transformation_transform_item1,
                                &a_item_defined_transformation_transform_item2,
                                &a_kinematic_pair_joint,
                                &a_point_on_planar_curve_pair_pair_curve,
                                a_point_on_planar_curve_pair_orientation,
                                &a_range_on_pair_curve,
                                has_lower_limit_yaw,
                                a_lower_limit_yaw,
                                has_upper_limit_yaw,
                                a_upper_limit_yaw,
                                has_lower_limit_pitch,
                                a_lower_limit_pitch,
                                has_upper_limit_pitch,
                                a_upper_limit_pitch,
                                has_lower_limit_roll,
                                a_lower_limit_roll,
                                has_upper_limit_roll,
                                a_upper_limit_roll,
                            );
                        } else {
                            a_kinematic_pair =
                                Handle::new(StepKinematicsPointOnPlanarCurvePair::new()).cast();
                            let a_local_pair: Handle<StepKinematicsPointOnPlanarCurvePair> =
                                a_kinematic_pair.cast::<StepKinematicsPointOnPlanarCurvePair>();
                            step_live(&a_local_pair, "ReadStep")?.init(
                                &a_representation_item_name,
                                &a_item_defined_transformation_name,
                                has_item_defined_transformation_description,
                                &a_item_defined_transformation_description,
                                &a_item_defined_transformation_transform_item1,
                                &a_item_defined_transformation_transform_item2,
                                &a_kinematic_pair_joint,
                                &a_point_on_planar_curve_pair_pair_curve,
                                a_point_on_planar_curve_pair_orientation,
                            );
                        }
                    }
                } else {
                    if a_pairtype == b"POINT_ON_SURFACE_PAIR" {
                        let mut a_point_on_surface_pair_pair_surface: Handle<StepGeomSurface> =
                            Default::default();
                        the_data.read_entity(
                            the_num,
                            1,
                            "point_on_surface_pair.pair_surface",
                            the_arch,
                            StepGeomSurface::static_type(),
                            &mut a_point_on_surface_pair_pair_surface,
                        )?;
                        if the_data.named_for_complex_short(
                            "POINT_ON_SURFACE_PAIR_WITH_RANGE ",
                            "RAPPWR",
                            num0,
                            &mut the_num,
                            the_arch,
                        ) {
                            if the_data.check_nb_params(
                                the_num,
                                2,
                                the_arch,
                                "POINT_ON_SURFACE_PAIR_WITH_RANGE ",
                            )? {
                                a_kinematic_pair =
                                    Handle::new(StepKinematicsPointOnSurfacePairWithRange::new())
                                        .cast();
                                let mut a_range_on_pair_surface: Handle<
                                    StepGeomRectangularTrimmedSurface,
                                > = Default::default();
                                the_data.read_entity(
                                    the_num,
                                    1,
                                    "range_on_pair_surface",
                                    the_arch,
                                    StepGeomRectangularTrimmedSurface::static_type(),
                                    &mut a_range_on_pair_surface,
                                )?;
                                let mut a_lower_limit_yaw: f64 = Default::default();
                                let mut has_lower_limit_yaw: bool = true;
                                if the_data.is_param_defined(the_num, 2)? {
                                    the_data.read_real(
                                        the_num,
                                        2,
                                        "lower_limit_yaw",
                                        the_arch,
                                        &mut a_lower_limit_yaw,
                                    )?;
                                } else {
                                    has_lower_limit_yaw = false;
                                    a_lower_limit_yaw = 0.0;
                                }
                                let mut a_upper_limit_yaw: f64 = Default::default();
                                let mut has_upper_limit_yaw: bool = true;
                                if the_data.is_param_defined(the_num, 3)? {
                                    the_data.read_real(
                                        the_num,
                                        3,
                                        "upper_limit_yaw",
                                        the_arch,
                                        &mut a_upper_limit_yaw,
                                    )?;
                                } else {
                                    has_upper_limit_yaw = false;
                                    a_upper_limit_yaw = 0.0;
                                }
                                let mut a_lower_limit_pitch: f64 = Default::default();
                                let mut has_lower_limit_pitch: bool = true;
                                if the_data.is_param_defined(the_num, 4)? {
                                    the_data.read_real(
                                        the_num,
                                        4,
                                        "lower_limit_pitch",
                                        the_arch,
                                        &mut a_lower_limit_pitch,
                                    )?;
                                } else {
                                    has_lower_limit_pitch = false;
                                    a_lower_limit_pitch = 0.0;
                                }
                                let mut a_upper_limit_pitch: f64 = Default::default();
                                let mut has_upper_limit_pitch: bool = true;
                                if the_data.is_param_defined(the_num, 5)? {
                                    the_data.read_real(
                                        the_num,
                                        5,
                                        "upper_limit_pitch",
                                        the_arch,
                                        &mut a_upper_limit_pitch,
                                    )?;
                                } else {
                                    has_upper_limit_pitch = false;
                                    a_upper_limit_pitch = 0.0;
                                }
                                let mut a_lower_limit_roll: f64 = Default::default();
                                let mut has_lower_limit_roll: bool = true;
                                if the_data.is_param_defined(the_num, 6)? {
                                    the_data.read_real(
                                        the_num,
                                        6,
                                        "lower_limit_roll",
                                        the_arch,
                                        &mut a_lower_limit_roll,
                                    )?;
                                } else {
                                    has_lower_limit_roll = false;
                                    a_lower_limit_roll = 0.0;
                                }
                                let mut a_upper_limit_roll: f64 = Default::default();
                                let mut has_upper_limit_roll: bool = true;
                                if the_data.is_param_defined(the_num, 7)? {
                                    the_data.read_real(
                                        the_num,
                                        7,
                                        "upper_limit_roll",
                                        the_arch,
                                        &mut a_upper_limit_roll,
                                    )?;
                                } else {
                                    has_upper_limit_roll = false;
                                    a_upper_limit_roll = 0.0;
                                }
                                let a_local_pair: Handle<
                                    StepKinematicsPointOnSurfacePairWithRange,
                                > = a_kinematic_pair
                                    .cast::<StepKinematicsPointOnSurfacePairWithRange>();
                                step_live(&a_local_pair, "ReadStep")?.init(
                                    &a_representation_item_name,
                                    &a_item_defined_transformation_name,
                                    has_item_defined_transformation_description,
                                    &a_item_defined_transformation_description,
                                    &a_item_defined_transformation_transform_item1,
                                    &a_item_defined_transformation_transform_item2,
                                    &a_kinematic_pair_joint,
                                    &a_point_on_surface_pair_pair_surface,
                                    &a_range_on_pair_surface,
                                    has_lower_limit_yaw,
                                    a_lower_limit_yaw,
                                    has_upper_limit_yaw,
                                    a_upper_limit_yaw,
                                    has_lower_limit_pitch,
                                    a_lower_limit_pitch,
                                    has_upper_limit_pitch,
                                    a_upper_limit_pitch,
                                    has_lower_limit_roll,
                                    a_lower_limit_roll,
                                    has_upper_limit_roll,
                                    a_upper_limit_roll,
                                );
                            } else {
                                a_kinematic_pair =
                                    Handle::new(StepKinematicsPointOnSurfacePair::new()).cast();
                                let a_local_pair: Handle<StepKinematicsPointOnSurfacePair> =
                                    a_kinematic_pair.cast::<StepKinematicsPointOnSurfacePair>();
                                step_live(&a_local_pair, "ReadStep")?.init(
                                    &a_representation_item_name,
                                    &a_item_defined_transformation_name,
                                    has_item_defined_transformation_description,
                                    &a_item_defined_transformation_description,
                                    &a_item_defined_transformation_transform_item1,
                                    &a_item_defined_transformation_transform_item2,
                                    &a_kinematic_pair_joint,
                                    &a_point_on_surface_pair_pair_surface,
                                );
                            }
                        }
                    } else {
                        if a_pairtype == b"PLANAR_CURVE_PAIR" {
                            let mut a_planar_curve_pair_curve1: Handle<StepGeomCurve> =
                                Default::default();
                            the_data.read_entity(
                                the_num,
                                1,
                                "planar_curve_pair.curve1",
                                the_arch,
                                StepGeomCurve::static_type(),
                                &mut a_planar_curve_pair_curve1,
                            )?;
                            let mut a_planar_curve_pair_curve2: Handle<StepGeomCurve> =
                                Default::default();
                            the_data.read_entity(
                                the_num,
                                2,
                                "planar_curve_pair.curve2",
                                the_arch,
                                StepGeomCurve::static_type(),
                                &mut a_planar_curve_pair_curve2,
                            )?;
                            let mut a_planar_curve_pair_orientation: bool = Default::default();
                            the_data.read_boolean(
                                the_num,
                                3,
                                "planar_curve_pair.orientation",
                                the_arch,
                                &mut a_planar_curve_pair_orientation,
                            )?;
                            if the_data.named_for_complex_short(
                                "PLANAR_CURVE_PAIR_RANGE",
                                "PCPR",
                                num0,
                                &mut the_num,
                                the_arch,
                            ) {
                                if the_data.check_nb_params(
                                    the_num,
                                    2,
                                    the_arch,
                                    "PLANAR_CURVE_PAIR_RANGE",
                                )? {
                                    a_kinematic_pair =
                                        Handle::new(StepKinematicsPlanarCurvePairRange::new())
                                            .cast();
                                    let mut a_range_on_curve1: Handle<StepGeomTrimmedCurve> =
                                        Default::default();
                                    the_data.read_entity(
                                        the_num,
                                        1,
                                        "range_on_curve1",
                                        the_arch,
                                        StepGeomTrimmedCurve::static_type(),
                                        &mut a_range_on_curve1,
                                    )?;
                                    let mut a_range_on_curve2: Handle<StepGeomTrimmedCurve> =
                                        Default::default();
                                    the_data.read_entity(
                                        the_num,
                                        2,
                                        "range_on_curve2",
                                        the_arch,
                                        StepGeomTrimmedCurve::static_type(),
                                        &mut a_range_on_curve2,
                                    )?;
                                    let a_local_pair: Handle<StepKinematicsPlanarCurvePairRange> =
                                        a_kinematic_pair
                                            .cast::<StepKinematicsPlanarCurvePairRange>();
                                    step_live(&a_local_pair, "ReadStep")?.init(
                                        &a_representation_item_name,
                                        &a_item_defined_transformation_name,
                                        has_item_defined_transformation_description,
                                        &a_item_defined_transformation_description,
                                        &a_item_defined_transformation_transform_item1,
                                        &a_item_defined_transformation_transform_item2,
                                        &a_kinematic_pair_joint,
                                        &a_planar_curve_pair_curve1,
                                        &a_planar_curve_pair_curve2,
                                        a_planar_curve_pair_orientation,
                                        &a_range_on_curve1,
                                        &a_range_on_curve2,
                                    );
                                } else {
                                    a_kinematic_pair =
                                        Handle::new(StepKinematicsPlanarCurvePair::new()).cast();
                                    let a_local_pair: Handle<StepKinematicsPlanarCurvePair> =
                                        a_kinematic_pair.cast::<StepKinematicsPlanarCurvePair>();
                                    step_live(&a_local_pair, "ReadStep")?.init(
                                        &a_representation_item_name,
                                        &a_item_defined_transformation_name,
                                        has_item_defined_transformation_description,
                                        &a_item_defined_transformation_description,
                                        &a_item_defined_transformation_transform_item1,
                                        &a_item_defined_transformation_transform_item2,
                                        &a_kinematic_pair_joint,
                                        &a_planar_curve_pair_curve1,
                                        &a_planar_curve_pair_curve2,
                                        a_planar_curve_pair_orientation,
                                    );
                                }
                            }
                        } else {
                            if a_pairtype == b"SURFACE_PAIR" {
                                let mut a_surface_pair_surface1: Handle<StepGeomSurface> =
                                    Default::default();
                                the_data.read_entity(
                                    the_num,
                                    1,
                                    "surface_pair.surface1",
                                    the_arch,
                                    StepGeomSurface::static_type(),
                                    &mut a_surface_pair_surface1,
                                )?;
                                let mut a_surface_pair_surface2: Handle<StepGeomSurface> =
                                    Default::default();
                                the_data.read_entity(
                                    the_num,
                                    2,
                                    "surface_pair.surface2",
                                    the_arch,
                                    StepGeomSurface::static_type(),
                                    &mut a_surface_pair_surface2,
                                )?;
                                let mut a_surface_pair_orientation: bool = Default::default();
                                the_data.read_boolean(
                                    the_num,
                                    3,
                                    "surface_pair.orientation",
                                    the_arch,
                                    &mut a_surface_pair_orientation,
                                )?;
                                if the_data.named_for_complex_short(
                                    "SURFACE_PAIR_WITH_RANGE",
                                    "SPW1",
                                    num0,
                                    &mut the_num,
                                    the_arch,
                                ) {
                                    if the_data.check_nb_params(
                                        the_num,
                                        4,
                                        the_arch,
                                        "SURFACE_PAIR_WITH_RANGE",
                                    )? {
                                        a_kinematic_pair =
                                            Handle::new(StepKinematicsSurfacePairWithRange::new())
                                                .cast();
                                        let mut a_range_on_surface1: Handle<
                                            StepGeomRectangularTrimmedSurface,
                                        > = Default::default();
                                        the_data.read_entity(
                                            the_num,
                                            1,
                                            "range_on_surface1",
                                            the_arch,
                                            StepGeomRectangularTrimmedSurface::static_type(),
                                            &mut a_range_on_surface1,
                                        )?;
                                        let mut a_range_on_surface2: Handle<
                                            StepGeomRectangularTrimmedSurface,
                                        > = Default::default();
                                        the_data.read_entity(
                                            the_num,
                                            2,
                                            "range_on_surface2",
                                            the_arch,
                                            StepGeomRectangularTrimmedSurface::static_type(),
                                            &mut a_range_on_surface2,
                                        )?;
                                        let mut a_lower_limit_actual_rotation: f64 =
                                            Default::default();
                                        let mut has_lower_limit_actual_rotation: bool = true;
                                        if the_data.is_param_defined(the_num, 3)? {
                                            the_data.read_real(
                                                the_num,
                                                3,
                                                "lower_limit_actual_rotation",
                                                the_arch,
                                                &mut a_lower_limit_actual_rotation,
                                            )?;
                                        } else {
                                            has_lower_limit_actual_rotation = false;
                                            a_lower_limit_actual_rotation = 0.0;
                                        }
                                        let mut a_upper_limit_actual_rotation: f64 =
                                            Default::default();
                                        let mut has_upper_limit_actual_rotation: bool = true;
                                        if the_data.is_param_defined(the_num, 4)? {
                                            the_data.read_real(
                                                the_num,
                                                4,
                                                "upper_limit_actual_rotation",
                                                the_arch,
                                                &mut a_upper_limit_actual_rotation,
                                            )?;
                                        } else {
                                            has_upper_limit_actual_rotation = false;
                                            a_upper_limit_actual_rotation = 0.0;
                                        }
                                        let a_local_pair: Handle<
                                            StepKinematicsSurfacePairWithRange,
                                        > = a_kinematic_pair
                                            .cast::<StepKinematicsSurfacePairWithRange>();
                                        step_live(&a_local_pair, "ReadStep")?.init(
                                            &a_representation_item_name,
                                            &a_item_defined_transformation_name,
                                            has_item_defined_transformation_description,
                                            &a_item_defined_transformation_description,
                                            &a_item_defined_transformation_transform_item1,
                                            &a_item_defined_transformation_transform_item2,
                                            &a_kinematic_pair_joint,
                                            &a_surface_pair_surface1,
                                            &a_surface_pair_surface2,
                                            a_surface_pair_orientation,
                                            &a_range_on_surface1,
                                            &a_range_on_surface2,
                                            has_lower_limit_actual_rotation,
                                            a_lower_limit_actual_rotation,
                                            has_upper_limit_actual_rotation,
                                            a_upper_limit_actual_rotation,
                                        );
                                    } else {
                                        a_kinematic_pair =
                                            Handle::new(StepKinematicsSurfacePair::new()).cast();
                                        let a_local_pair: Handle<StepKinematicsSurfacePair> =
                                            a_kinematic_pair.cast::<StepKinematicsSurfacePair>();
                                        step_live(&a_local_pair, "ReadStep")?.init(
                                            &a_representation_item_name,
                                            &a_item_defined_transformation_name,
                                            has_item_defined_transformation_description,
                                            &a_item_defined_transformation_description,
                                            &a_item_defined_transformation_transform_item1,
                                            &a_item_defined_transformation_transform_item2,
                                            &a_kinematic_pair_joint,
                                            &a_surface_pair_surface1,
                                            &a_surface_pair_surface2,
                                            a_surface_pair_orientation,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        the_data.named_for_complex_short(
            "REPRESENTATION_ITEM",
            "RPRITM",
            num0,
            &mut the_num,
            the_arch,
        );
        the_data.read_string(
            the_num,
            1,
            "representation_item.name",
            the_arch,
            &mut a_representation_item_name,
        )?;
        an_actuated_kinematic_pair.init(
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
        a_kinematic_pair.set_name(&a_representation_item_name);
        the_ent.init(
            &a_representation_item_name,
            &a_item_defined_transformation_name,
            has_item_defined_transformation_description,
            &a_item_defined_transformation_description,
            &a_item_defined_transformation_transform_item1,
            &a_item_defined_transformation_transform_item2,
            &a_kinematic_pair_joint,
            &an_actuated_kinematic_pair,
            &a_kinematic_pair,
        );
        Ok(())
    }
}
