#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

impl RwStepKinematicsRwActuatedKinPairAndOrderKinPair {
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepKinematicsActuatedKinPairAndOrderKinPair>,
    ) -> Result<(), Failure> {
        step_start_entity(the_sw, "ACTUATED_KINEMATIC_PAIR")?;
        if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.has_tx() {
            if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.tx()
                == StepKinematicsActuatedDirection::AdBidirectional
            {
                the_sw.send_enum(b".BIDIRECTIONAL.")?;
            } else {
                if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.tx()
                    == StepKinematicsActuatedDirection::AdPositiveOnly
                {
                    the_sw.send_enum(b".POSITIVE_ONLY.")?;
                } else {
                    if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.tx()
                        == StepKinematicsActuatedDirection::AdNegativeOnly
                    {
                        the_sw.send_enum(b".NEGATIVE_ONLY.")?;
                    } else {
                        if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.tx()
                            == StepKinematicsActuatedDirection::AdNotActuated
                        {
                            the_sw.send_enum(b".NOT_ACTUATED.")?;
                        }
                    }
                }
            }
        } else {
            the_sw.send_undef()?;
        }
        if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.has_ty() {
            if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.ty()
                == StepKinematicsActuatedDirection::AdBidirectional
            {
                the_sw.send_enum(b".BIDIRECTIONAL.")?;
            } else {
                if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.ty()
                    == StepKinematicsActuatedDirection::AdPositiveOnly
                {
                    the_sw.send_enum(b".POSITIVE_ONLY.")?;
                } else {
                    if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.ty()
                        == StepKinematicsActuatedDirection::AdNegativeOnly
                    {
                        the_sw.send_enum(b".NEGATIVE_ONLY.")?;
                    } else {
                        if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.ty()
                            == StepKinematicsActuatedDirection::AdNotActuated
                        {
                            the_sw.send_enum(b".NOT_ACTUATED.")?;
                        }
                    }
                }
            }
        } else {
            the_sw.send_undef()?;
        }
        if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.has_tz() {
            if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.tz()
                == StepKinematicsActuatedDirection::AdBidirectional
            {
                the_sw.send_enum(b".BIDIRECTIONAL.")?;
            } else {
                if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.tz()
                    == StepKinematicsActuatedDirection::AdPositiveOnly
                {
                    the_sw.send_enum(b".POSITIVE_ONLY.")?;
                } else {
                    if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.tz()
                        == StepKinematicsActuatedDirection::AdNegativeOnly
                    {
                        the_sw.send_enum(b".NEGATIVE_ONLY.")?;
                    } else {
                        if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.tz()
                            == StepKinematicsActuatedDirection::AdNotActuated
                        {
                            the_sw.send_enum(b".NOT_ACTUATED.")?;
                        }
                    }
                }
            }
        } else {
            the_sw.send_undef()?;
        }
        if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.has_rx() {
            if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.rx()
                == StepKinematicsActuatedDirection::AdBidirectional
            {
                the_sw.send_enum(b".BIDIRECTIONAL.")?;
            } else {
                if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.rx()
                    == StepKinematicsActuatedDirection::AdPositiveOnly
                {
                    the_sw.send_enum(b".POSITIVE_ONLY.")?;
                } else {
                    if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.rx()
                        == StepKinematicsActuatedDirection::AdNegativeOnly
                    {
                        the_sw.send_enum(b".NEGATIVE_ONLY.")?;
                    } else {
                        if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.rx()
                            == StepKinematicsActuatedDirection::AdNotActuated
                        {
                            the_sw.send_enum(b".NOT_ACTUATED.")?;
                        }
                    }
                }
            }
        } else {
            the_sw.send_undef()?;
        }
        if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.has_ry() {
            if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.ry()
                == StepKinematicsActuatedDirection::AdBidirectional
            {
                the_sw.send_enum(b".BIDIRECTIONAL.")?;
            } else {
                if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.ry()
                    == StepKinematicsActuatedDirection::AdPositiveOnly
                {
                    the_sw.send_enum(b".POSITIVE_ONLY.")?;
                } else {
                    if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.ry()
                        == StepKinematicsActuatedDirection::AdNegativeOnly
                    {
                        the_sw.send_enum(b".NEGATIVE_ONLY.")?;
                    } else {
                        if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.ry()
                            == StepKinematicsActuatedDirection::AdNotActuated
                        {
                            the_sw.send_enum(b".NOT_ACTUATED.")?;
                        }
                    }
                }
            }
        } else {
            the_sw.send_undef()?;
        }
        if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.has_rz() {
            if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.rz()
                == StepKinematicsActuatedDirection::AdBidirectional
            {
                the_sw.send_enum(b".BIDIRECTIONAL.")?;
            } else {
                if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.rz()
                    == StepKinematicsActuatedDirection::AdPositiveOnly
                {
                    the_sw.send_enum(b".POSITIVE_ONLY.")?;
                } else {
                    if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.rz()
                        == StepKinematicsActuatedDirection::AdNegativeOnly
                    {
                        the_sw.send_enum(b".NEGATIVE_ONLY.")?;
                    } else {
                        if step_live(&the_ent.get_actuated_kinematic_pair(), "WriteStep")?.rz()
                            == StepKinematicsActuatedDirection::AdNotActuated
                        {
                            the_sw.send_enum(b".NOT_ACTUATED.")?;
                        }
                    }
                }
            }
        } else {
            the_sw.send_undef()?;
        }
        step_start_entity(the_sw, "GEOMETRIC_REPRESENTATION_ITEM")?;
        step_start_entity(the_sw, "ITEM_DEFINED_TRANSFORMATION")?;
        rw_block_b_0125!(the_ent, the_sw);
        step_start_entity(the_sw, "KINEMATIC_PAIR")?;
        the_sw.send_entity(&the_ent.joint())?;
        if the_ent
            .get_order_kinematic_pair()
            .is_kind(StepKinematicsLowOrderKinematicPair::static_type())
        {
            let a_low_order_kinematic_pair: Handle<StepKinematicsLowOrderKinematicPair> = the_ent
                .get_order_kinematic_pair()
                .cast::<StepKinematicsLowOrderKinematicPair>();
            step_start_entity(the_sw, "LOW_ORDER_KINEMATIC_PAIR")?;
            the_sw.send_boolean(step_live(&a_low_order_kinematic_pair, "WriteStep")?.tx())?;
            the_sw.send_boolean(step_live(&a_low_order_kinematic_pair, "WriteStep")?.ty())?;
            the_sw.send_boolean(step_live(&a_low_order_kinematic_pair, "WriteStep")?.tz())?;
            the_sw.send_boolean(step_live(&a_low_order_kinematic_pair, "WriteStep")?.rx())?;
            the_sw.send_boolean(step_live(&a_low_order_kinematic_pair, "WriteStep")?.ry())?;
            the_sw.send_boolean(step_live(&a_low_order_kinematic_pair, "WriteStep")?.rz())?;
            if the_ent
                .get_order_kinematic_pair()
                .is_kind(StepKinematicsRevolutePair::static_type())
            {
                step_start_entity(the_sw, "REVOLUTE_PAIR")?;
                step_start_entity(the_sw, "REVOLUTE_PAIR_WITH_RANGE")?;
                if the_ent
                    .get_order_kinematic_pair()
                    .is_kind(StepKinematicsRevolutePairWithRange::static_type())
                {
                    let a_revolute_pair_with_range: Handle<StepKinematicsRevolutePairWithRange> =
                        the_ent
                            .get_order_kinematic_pair()
                            .cast::<StepKinematicsRevolutePairWithRange>();
                    if step_live(&a_revolute_pair_with_range, "WriteStep")?
                        .has_lower_limit_actual_rotation()
                    {
                        the_sw.send_real(
                            step_live(&a_revolute_pair_with_range, "WriteStep")?
                                .lower_limit_actual_rotation(),
                        )?;
                    } else {
                        the_sw.send_undef()?;
                    }
                    if step_live(&a_revolute_pair_with_range, "WriteStep")?
                        .has_upper_limit_actual_rotation()
                    {
                        the_sw.send_real(
                            step_live(&a_revolute_pair_with_range, "WriteStep")?
                                .upper_limit_actual_rotation(),
                        )?;
                    } else {
                        the_sw.send_undef()?;
                    }
                }
            } else {
                if the_ent
                    .get_order_kinematic_pair()
                    .is_kind(StepKinematicsCylindricalPair::static_type())
                {
                    step_start_entity(the_sw, "CYLINDRICAL_PAIR")?;
                    step_start_entity(the_sw, "CYLINDRICAL_PAIR_WITH_RANGE")?;
                    if the_ent
                        .get_order_kinematic_pair()
                        .is_kind(StepKinematicsCylindricalPairWithRange::static_type())
                    {
                        let a_cylindrical_pair_with_range: Handle<
                            StepKinematicsCylindricalPairWithRange,
                        > = the_ent
                            .get_order_kinematic_pair()
                            .cast::<StepKinematicsCylindricalPairWithRange>();
                        if step_live(&a_cylindrical_pair_with_range, "WriteStep")?
                            .has_lower_limit_actual_translation()
                        {
                            the_sw.send_real(
                                step_live(&a_cylindrical_pair_with_range, "WriteStep")?
                                    .lower_limit_actual_translation(),
                            )?;
                        } else {
                            the_sw.send_undef()?;
                        }
                        if step_live(&a_cylindrical_pair_with_range, "WriteStep")?
                            .has_upper_limit_actual_translation()
                        {
                            the_sw.send_real(
                                step_live(&a_cylindrical_pair_with_range, "WriteStep")?
                                    .upper_limit_actual_translation(),
                            )?;
                        } else {
                            the_sw.send_undef()?;
                        }
                        if step_live(&a_cylindrical_pair_with_range, "WriteStep")?
                            .has_lower_limit_actual_rotation()
                        {
                            the_sw.send_real(
                                step_live(&a_cylindrical_pair_with_range, "WriteStep")?
                                    .lower_limit_actual_rotation(),
                            )?;
                        } else {
                            the_sw.send_undef()?;
                        }
                        if step_live(&a_cylindrical_pair_with_range, "WriteStep")?
                            .has_upper_limit_actual_rotation()
                        {
                            the_sw.send_real(
                                step_live(&a_cylindrical_pair_with_range, "WriteStep")?
                                    .upper_limit_actual_rotation(),
                            )?;
                        } else {
                            the_sw.send_undef()?;
                        }
                    }
                } else {
                    if the_ent
                        .get_order_kinematic_pair()
                        .is_kind(StepKinematicsSphericalPair::static_type())
                    {
                        step_start_entity(the_sw, "SPHERICAL_PAIR")?;
                        step_start_entity(the_sw, "SPHERICAL_PAIR_WITH_RANGE")?;
                        if the_ent
                            .get_order_kinematic_pair()
                            .is_kind(StepKinematicsSphericalPairWithRange::static_type())
                        {
                            let a_spherical_pair_with_range: Handle<
                                StepKinematicsSphericalPairWithRange,
                            > = the_ent
                                .get_order_kinematic_pair()
                                .cast::<StepKinematicsSphericalPairWithRange>();
                            if step_live(&a_spherical_pair_with_range, "WriteStep")?
                                .has_lower_limit_yaw()
                            {
                                the_sw.send_real(
                                    step_live(&a_spherical_pair_with_range, "WriteStep")?
                                        .lower_limit_yaw(),
                                )?;
                            } else {
                                the_sw.send_undef()?;
                            }
                            if step_live(&a_spherical_pair_with_range, "WriteStep")?
                                .has_upper_limit_yaw()
                            {
                                the_sw.send_real(
                                    step_live(&a_spherical_pair_with_range, "WriteStep")?
                                        .upper_limit_yaw(),
                                )?;
                            } else {
                                the_sw.send_undef()?;
                            }
                            if step_live(&a_spherical_pair_with_range, "WriteStep")?
                                .has_lower_limit_pitch()
                            {
                                the_sw.send_real(
                                    step_live(&a_spherical_pair_with_range, "WriteStep")?
                                        .lower_limit_pitch(),
                                )?;
                            } else {
                                the_sw.send_undef()?;
                            }
                            if step_live(&a_spherical_pair_with_range, "WriteStep")?
                                .has_upper_limit_pitch()
                            {
                                the_sw.send_real(
                                    step_live(&a_spherical_pair_with_range, "WriteStep")?
                                        .upper_limit_pitch(),
                                )?;
                            } else {
                                the_sw.send_undef()?;
                            }
                            if step_live(&a_spherical_pair_with_range, "WriteStep")?
                                .has_lower_limit_roll()
                            {
                                the_sw.send_real(
                                    step_live(&a_spherical_pair_with_range, "WriteStep")?
                                        .lower_limit_roll(),
                                )?;
                            } else {
                                the_sw.send_undef()?;
                            }
                            if step_live(&a_spherical_pair_with_range, "WriteStep")?
                                .has_upper_limit_roll()
                            {
                                the_sw.send_real(
                                    step_live(&a_spherical_pair_with_range, "WriteStep")?
                                        .upper_limit_roll(),
                                )?;
                            } else {
                                the_sw.send_undef()?;
                            }
                        }
                    } else {
                        if the_ent
                            .get_order_kinematic_pair()
                            .is_kind(StepKinematicsSphericalPairWithPin::static_type())
                        {
                            step_start_entity(the_sw, "SPHERICAL_PAIR_WITH_PIN")?;
                            step_start_entity(the_sw, "SPHERICAL_PAIR_WITH_PIN_AND_RANGE")?;
                            if the_ent
                                .get_order_kinematic_pair()
                                .is_kind(StepKinematicsSphericalPairWithPinAndRange::static_type())
                            {
                                let a_spherical_pair_with_pin_and_range: Handle<
                                    StepKinematicsSphericalPairWithPinAndRange,
                                > = the_ent
                                    .get_order_kinematic_pair()
                                    .cast::<StepKinematicsSphericalPairWithPinAndRange>();
                                if step_live(&a_spherical_pair_with_pin_and_range, "WriteStep")?
                                    .has_lower_limit_yaw()
                                {
                                    the_sw.send_real(
                                        step_live(
                                            &a_spherical_pair_with_pin_and_range,
                                            "WriteStep",
                                        )?
                                        .lower_limit_yaw(),
                                    )?;
                                } else {
                                    the_sw.send_undef()?;
                                }
                                if step_live(&a_spherical_pair_with_pin_and_range, "WriteStep")?
                                    .has_upper_limit_yaw()
                                {
                                    the_sw.send_real(
                                        step_live(
                                            &a_spherical_pair_with_pin_and_range,
                                            "WriteStep",
                                        )?
                                        .upper_limit_yaw(),
                                    )?;
                                } else {
                                    the_sw.send_undef()?;
                                }
                                if step_live(&a_spherical_pair_with_pin_and_range, "WriteStep")?
                                    .has_lower_limit_roll()
                                {
                                    the_sw.send_real(
                                        step_live(
                                            &a_spherical_pair_with_pin_and_range,
                                            "WriteStep",
                                        )?
                                        .lower_limit_roll(),
                                    )?;
                                } else {
                                    the_sw.send_undef()?;
                                }
                                if step_live(&a_spherical_pair_with_pin_and_range, "WriteStep")?
                                    .has_upper_limit_roll()
                                {
                                    the_sw.send_real(
                                        step_live(
                                            &a_spherical_pair_with_pin_and_range,
                                            "WriteStep",
                                        )?
                                        .upper_limit_roll(),
                                    )?;
                                } else {
                                    the_sw.send_undef()?;
                                }
                            }
                        } else {
                            if the_ent
                                .get_order_kinematic_pair()
                                .is_kind(StepKinematicsPrismaticPair::static_type())
                            {
                                step_start_entity(the_sw, "PRISMATIC_PAIR")?;
                                step_start_entity(the_sw, "PRISMATIC_PAIR_WITH_RANGE")?;
                                if the_ent
                                    .get_order_kinematic_pair()
                                    .is_kind(StepKinematicsPrismaticPairWithRange::static_type())
                                {
                                    let a_prismatic_pair_with_range: Handle<
                                        StepKinematicsPrismaticPairWithRange,
                                    > = the_ent
                                        .get_order_kinematic_pair()
                                        .cast::<StepKinematicsPrismaticPairWithRange>();
                                    if step_live(&a_prismatic_pair_with_range, "WriteStep")?
                                        .has_lower_limit_actual_translation()
                                    {
                                        the_sw.send_real(
                                            step_live(&a_prismatic_pair_with_range, "WriteStep")?
                                                .lower_limit_actual_translation(),
                                        )?;
                                    } else {
                                        the_sw.send_undef()?;
                                    }
                                    if step_live(&a_prismatic_pair_with_range, "WriteStep")?
                                        .has_upper_limit_actual_translation()
                                    {
                                        the_sw.send_real(
                                            step_live(&a_prismatic_pair_with_range, "WriteStep")?
                                                .upper_limit_actual_translation(),
                                        )?;
                                    } else {
                                        the_sw.send_undef()?;
                                    }
                                }
                            } else {
                                if the_ent
                                    .get_order_kinematic_pair()
                                    .is_kind(StepKinematicsUniversalPair::static_type())
                                {
                                    step_start_entity(the_sw, "UNIVERSAL_PAIR")?;
                                    let an_universal_pair: Handle<StepKinematicsUniversalPair> =
                                        the_ent
                                            .get_order_kinematic_pair()
                                            .cast::<StepKinematicsUniversalPair>();
                                    if step_live(&an_universal_pair, "WriteStep")?
                                        .has_input_skew_angle()
                                    {
                                        the_sw.send_real(
                                            step_live(&an_universal_pair, "WriteStep")?
                                                .input_skew_angle(),
                                        )?;
                                    } else {
                                        the_sw.send_undef()?;
                                    }
                                    step_start_entity(the_sw, "PRISMATIC_PAIR_WITH_RANGE")?;
                                    if the_ent.get_order_kinematic_pair().is_kind(StepKinematicsPrismaticPairWithRange::static_type()) {
let an_universal_pair_with_range: Handle<StepKinematicsUniversalPairWithRange> = the_ent.get_order_kinematic_pair().cast::<StepKinematicsUniversalPairWithRange>();
if step_live(&an_universal_pair_with_range, "WriteStep")?.has_lower_limit_first_rotation() {
the_sw.send_real(step_live(&an_universal_pair_with_range, "WriteStep")?.lower_limit_first_rotation())?;
} else {
the_sw.send_undef()?;
}
if step_live(&an_universal_pair_with_range, "WriteStep")?.has_upper_limit_first_rotation() {
the_sw.send_real(step_live(&an_universal_pair_with_range, "WriteStep")?.upper_limit_first_rotation())?;
} else {
the_sw.send_undef()?;
}
if step_live(&an_universal_pair_with_range, "WriteStep")?.has_lower_limit_second_rotation() {
the_sw.send_real(step_live(&an_universal_pair_with_range, "WriteStep")?.lower_limit_second_rotation())?;
} else {
the_sw.send_undef()?;
}
if step_live(&an_universal_pair_with_range, "WriteStep")?.has_upper_limit_second_rotation() {
the_sw.send_real(step_live(&an_universal_pair_with_range, "WriteStep")?.upper_limit_second_rotation())?;
} else {
the_sw.send_undef()?;
}
}
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if the_ent
                .get_order_kinematic_pair()
                .is_kind(StepKinematicsLowOrderKinematicPairWithMotionCoupling::static_type())
            {
                if the_ent
                    .get_order_kinematic_pair()
                    .is_kind(StepKinematicsGearPair::static_type())
                {
                    step_start_entity(the_sw, "GEAR_PAIR")?;
                    let a_gear_pair: Handle<StepKinematicsGearPair> = the_ent
                        .get_order_kinematic_pair()
                        .cast::<StepKinematicsGearPair>();
                    the_sw.send_real(step_live(&a_gear_pair, "WriteStep")?.radius_first_link())?;
                    the_sw.send_real(step_live(&a_gear_pair, "WriteStep")?.radius_second_link())?;
                    the_sw.send_real(step_live(&a_gear_pair, "WriteStep")?.bevel())?;
                    the_sw.send_real(step_live(&a_gear_pair, "WriteStep")?.helical_angle())?;
                    the_sw.send_real(step_live(&a_gear_pair, "WriteStep")?.gear_ratio())?;
                    step_start_entity(the_sw, "PRISMATIC_PAIR_WITH_RANGE")?;
                    if the_ent
                        .get_order_kinematic_pair()
                        .is_kind(StepKinematicsGearPairWithRange::static_type())
                    {
                        let a_gear_pair_with_range: Handle<StepKinematicsGearPairWithRange> =
                            the_ent
                                .get_order_kinematic_pair()
                                .cast::<StepKinematicsGearPairWithRange>();
                        if step_live(&a_gear_pair_with_range, "WriteStep")?
                            .has_lower_limit_actual_rotation1()
                        {
                            the_sw.send_real(
                                step_live(&a_gear_pair_with_range, "WriteStep")?
                                    .lower_limit_actual_rotation1(),
                            )?;
                        } else {
                            the_sw.send_undef()?;
                        }
                        if step_live(&a_gear_pair_with_range, "WriteStep")?
                            .has_upper_limit_actual_rotation1()
                        {
                            the_sw.send_real(
                                step_live(&a_gear_pair_with_range, "WriteStep")?
                                    .upper_limit_actual_rotation1(),
                            )?;
                        } else {
                            the_sw.send_undef()?;
                        }
                    }
                } else {
                    if the_ent
                        .get_order_kinematic_pair()
                        .is_kind(StepKinematicsRackAndPinionPair::static_type())
                    {
                        step_start_entity(the_sw, "RACK_AND_PINION_PAIR")?;
                        let a_rack_and_pinion_pair: Handle<StepKinematicsRackAndPinionPair> =
                            the_ent
                                .get_order_kinematic_pair()
                                .cast::<StepKinematicsRackAndPinionPair>();
                        the_sw.send_real(
                            step_live(&a_rack_and_pinion_pair, "WriteStep")?.pinion_radius(),
                        )?;
                        step_start_entity(the_sw, "RACK_AND_PINION_PAIR_WITH_RANGE")?;
                        if the_ent
                            .get_order_kinematic_pair()
                            .is_kind(StepKinematicsRackAndPinionPairWithRange::static_type())
                        {
                            let a_rack_and_pinion_pair_with_range: Handle<
                                StepKinematicsRackAndPinionPairWithRange,
                            > = the_ent
                                .get_order_kinematic_pair()
                                .cast::<StepKinematicsRackAndPinionPairWithRange>();
                            if step_live(&a_rack_and_pinion_pair_with_range, "WriteStep")?
                                .has_lower_limit_rack_displacement()
                            {
                                the_sw.send_real(
                                    step_live(&a_rack_and_pinion_pair_with_range, "WriteStep")?
                                        .lower_limit_rack_displacement(),
                                )?;
                            } else {
                                the_sw.send_undef()?;
                            }
                            if step_live(&a_rack_and_pinion_pair_with_range, "WriteStep")?
                                .has_upper_limit_rack_displacement()
                            {
                                the_sw.send_real(
                                    step_live(&a_rack_and_pinion_pair_with_range, "WriteStep")?
                                        .upper_limit_rack_displacement(),
                                )?;
                            } else {
                                the_sw.send_undef()?;
                            }
                        }
                    } else {
                        if the_ent
                            .get_order_kinematic_pair()
                            .is_kind(StepKinematicsScrewPair::static_type())
                        {
                            step_start_entity(the_sw, "SCREW_PAIR")?;
                            let a_screw_pair: Handle<StepKinematicsScrewPair> = the_ent
                                .get_order_kinematic_pair()
                                .cast::<StepKinematicsScrewPair>();
                            the_sw.send_real(step_live(&a_screw_pair, "WriteStep")?.pitch())?;
                            step_start_entity(the_sw, "SCREW_PAIR_WITH_RANGE")?;
                            if the_ent
                                .get_order_kinematic_pair()
                                .is_kind(StepKinematicsScrewPairWithRange::static_type())
                            {
                                let a_screw_pair_with_range: Handle<
                                    StepKinematicsScrewPairWithRange,
                                > = the_ent
                                    .get_order_kinematic_pair()
                                    .cast::<StepKinematicsScrewPairWithRange>();
                                if step_live(&a_screw_pair_with_range, "WriteStep")?
                                    .has_lower_limit_actual_rotation()
                                {
                                    the_sw.send_real(
                                        step_live(&a_screw_pair_with_range, "WriteStep")?
                                            .lower_limit_actual_rotation(),
                                    )?;
                                } else {
                                    the_sw.send_undef()?;
                                }
                                if step_live(&a_screw_pair_with_range, "WriteStep")?
                                    .has_upper_limit_actual_rotation()
                                {
                                    the_sw.send_real(
                                        step_live(&a_screw_pair_with_range, "WriteStep")?
                                            .upper_limit_actual_rotation(),
                                    )?;
                                } else {
                                    the_sw.send_undef()?;
                                }
                            }
                        }
                    }
                }
            } else {
                if the_ent
                    .get_order_kinematic_pair()
                    .is_kind(StepKinematicsHighOrderKinematicPair::static_type())
                {
                    if the_ent
                        .get_order_kinematic_pair()
                        .is_kind(StepKinematicsScrewPair::static_type())
                    {
                        step_start_entity(the_sw, "POINT_ON_PLANAR_CURVE_PAIR")?;
                        let a_point_on_planar_curve_pair: Handle<
                            StepKinematicsPointOnPlanarCurvePair,
                        > = the_ent
                            .get_order_kinematic_pair()
                            .cast::<StepKinematicsPointOnPlanarCurvePair>();
                        the_sw.send_entity(
                            &step_live(&a_point_on_planar_curve_pair, "WriteStep")?.pair_curve(),
                        )?;
                        the_sw.send_boolean(
                            step_live(&a_point_on_planar_curve_pair, "WriteStep")?.orientation(),
                        )?;
                        step_start_entity(the_sw, "POINT_ON_PLANAR_CURVE_PAIR_WITH_RANGE")?;
                        if the_ent
                            .get_order_kinematic_pair()
                            .is_kind(StepKinematicsPointOnPlanarCurvePairWithRange::static_type())
                        {
                            let a_point_on_planar_curve_pair_with_range: Handle<
                                StepKinematicsPointOnPlanarCurvePairWithRange,
                            > = the_ent
                                .get_order_kinematic_pair()
                                .cast::<StepKinematicsPointOnPlanarCurvePairWithRange>();
                            the_sw.send_entity(
                                &step_live(&a_point_on_planar_curve_pair_with_range, "WriteStep")?
                                    .range_on_pair_curve(),
                            )?;
                            if step_live(&a_point_on_planar_curve_pair_with_range, "WriteStep")?
                                .has_lower_limit_yaw()
                            {
                                the_sw.send_real(
                                    step_live(
                                        &a_point_on_planar_curve_pair_with_range,
                                        "WriteStep",
                                    )?
                                    .lower_limit_yaw(),
                                )?;
                            } else {
                                the_sw.send_undef()?;
                            }
                            if step_live(&a_point_on_planar_curve_pair_with_range, "WriteStep")?
                                .has_upper_limit_yaw()
                            {
                                the_sw.send_real(
                                    step_live(
                                        &a_point_on_planar_curve_pair_with_range,
                                        "WriteStep",
                                    )?
                                    .upper_limit_yaw(),
                                )?;
                            } else {
                                the_sw.send_undef()?;
                            }
                            if step_live(&a_point_on_planar_curve_pair_with_range, "WriteStep")?
                                .has_lower_limit_pitch()
                            {
                                the_sw.send_real(
                                    step_live(
                                        &a_point_on_planar_curve_pair_with_range,
                                        "WriteStep",
                                    )?
                                    .lower_limit_pitch(),
                                )?;
                            } else {
                                the_sw.send_undef()?;
                            }
                            if step_live(&a_point_on_planar_curve_pair_with_range, "WriteStep")?
                                .has_upper_limit_pitch()
                            {
                                the_sw.send_real(
                                    step_live(
                                        &a_point_on_planar_curve_pair_with_range,
                                        "WriteStep",
                                    )?
                                    .upper_limit_pitch(),
                                )?;
                            } else {
                                the_sw.send_undef()?;
                            }
                            if step_live(&a_point_on_planar_curve_pair_with_range, "WriteStep")?
                                .has_lower_limit_roll()
                            {
                                the_sw.send_real(
                                    step_live(
                                        &a_point_on_planar_curve_pair_with_range,
                                        "WriteStep",
                                    )?
                                    .lower_limit_roll(),
                                )?;
                            } else {
                                the_sw.send_undef()?;
                            }
                            if step_live(&a_point_on_planar_curve_pair_with_range, "WriteStep")?
                                .has_upper_limit_roll()
                            {
                                the_sw.send_real(
                                    step_live(
                                        &a_point_on_planar_curve_pair_with_range,
                                        "WriteStep",
                                    )?
                                    .upper_limit_roll(),
                                )?;
                            } else {
                                the_sw.send_undef()?;
                            }
                        }
                    } else {
                        if the_ent
                            .get_order_kinematic_pair()
                            .is_kind(StepKinematicsPointOnSurfacePair::static_type())
                        {
                            step_start_entity(the_sw, "POINT_ON_SURFACE_PAIR")?;
                            let a_point_on_surface_pair: Handle<StepKinematicsPointOnSurfacePair> =
                                the_ent
                                    .get_order_kinematic_pair()
                                    .cast::<StepKinematicsPointOnSurfacePair>();
                            the_sw.send_entity(
                                &step_live(&a_point_on_surface_pair, "WriteStep")?.pair_surface(),
                            )?;
                            step_start_entity(the_sw, "SCREW_PAIR_WITH_RANGE")?;
                            if the_ent
                                .get_order_kinematic_pair()
                                .is_kind(StepKinematicsPointOnSurfacePairWithRange::static_type())
                            {
                                let a_point_on_surface_pair_with_range: Handle<
                                    StepKinematicsPointOnSurfacePairWithRange,
                                > = the_ent
                                    .get_order_kinematic_pair()
                                    .cast::<StepKinematicsPointOnSurfacePairWithRange>();
                                the_sw.send_entity(
                                    &step_live(&a_point_on_surface_pair_with_range, "WriteStep")?
                                        .range_on_pair_surface(),
                                )?;
                                if step_live(&a_point_on_surface_pair_with_range, "WriteStep")?
                                    .has_lower_limit_yaw()
                                {
                                    the_sw.send_real(
                                        step_live(
                                            &a_point_on_surface_pair_with_range,
                                            "WriteStep",
                                        )?
                                        .lower_limit_yaw(),
                                    )?;
                                } else {
                                    the_sw.send_undef()?;
                                }
                                if step_live(&a_point_on_surface_pair_with_range, "WriteStep")?
                                    .has_upper_limit_yaw()
                                {
                                    the_sw.send_real(
                                        step_live(
                                            &a_point_on_surface_pair_with_range,
                                            "WriteStep",
                                        )?
                                        .upper_limit_yaw(),
                                    )?;
                                } else {
                                    the_sw.send_undef()?;
                                }
                                if step_live(&a_point_on_surface_pair_with_range, "WriteStep")?
                                    .has_lower_limit_pitch()
                                {
                                    the_sw.send_real(
                                        step_live(
                                            &a_point_on_surface_pair_with_range,
                                            "WriteStep",
                                        )?
                                        .lower_limit_pitch(),
                                    )?;
                                } else {
                                    the_sw.send_undef()?;
                                }
                                if step_live(&a_point_on_surface_pair_with_range, "WriteStep")?
                                    .has_upper_limit_pitch()
                                {
                                    the_sw.send_real(
                                        step_live(
                                            &a_point_on_surface_pair_with_range,
                                            "WriteStep",
                                        )?
                                        .upper_limit_pitch(),
                                    )?;
                                } else {
                                    the_sw.send_undef()?;
                                }
                                if step_live(&a_point_on_surface_pair_with_range, "WriteStep")?
                                    .has_lower_limit_roll()
                                {
                                    the_sw.send_real(
                                        step_live(
                                            &a_point_on_surface_pair_with_range,
                                            "WriteStep",
                                        )?
                                        .lower_limit_roll(),
                                    )?;
                                } else {
                                    the_sw.send_undef()?;
                                }
                                if step_live(&a_point_on_surface_pair_with_range, "WriteStep")?
                                    .has_upper_limit_roll()
                                {
                                    the_sw.send_real(
                                        step_live(
                                            &a_point_on_surface_pair_with_range,
                                            "WriteStep",
                                        )?
                                        .upper_limit_roll(),
                                    )?;
                                } else {
                                    the_sw.send_undef()?;
                                }
                            }
                        } else {
                            if the_ent
                                .get_order_kinematic_pair()
                                .is_kind(StepKinematicsPlanarCurvePair::static_type())
                            {
                                step_start_entity(the_sw, "PLANAR_CURVE_PAIR")?;
                                let a_planar_curve_pair: Handle<StepKinematicsPlanarCurvePair> =
                                    the_ent
                                        .get_order_kinematic_pair()
                                        .cast::<StepKinematicsPlanarCurvePair>();
                                the_sw.send_entity(
                                    &step_live(&a_planar_curve_pair, "WriteStep")?.curve1(),
                                )?;
                                the_sw.send_entity(
                                    &step_live(&a_planar_curve_pair, "WriteStep")?.curve2(),
                                )?;
                                step_start_entity(the_sw, "PLANAR_CURVE_PAIR_RANGE")?;
                                if the_ent
                                    .get_order_kinematic_pair()
                                    .is_kind(StepKinematicsPlanarCurvePairRange::static_type())
                                {
                                    let a_planar_curve_pair_range: Handle<
                                        StepKinematicsPlanarCurvePairRange,
                                    > = the_ent
                                        .get_order_kinematic_pair()
                                        .cast::<StepKinematicsPlanarCurvePairRange>();
                                    the_sw.send_entity(
                                        &step_live(&a_planar_curve_pair_range, "WriteStep")?
                                            .range_on_curve1(),
                                    )?;
                                    the_sw.send_entity(
                                        &step_live(&a_planar_curve_pair_range, "WriteStep")?
                                            .range_on_curve2(),
                                    )?;
                                }
                            } else {
                                if the_ent
                                    .get_order_kinematic_pair()
                                    .is_kind(StepKinematicsSurfacePair::static_type())
                                {
                                    step_start_entity(the_sw, "SURFACE_PAIR")?;
                                    let a_surface_pair: Handle<StepKinematicsSurfacePair> = the_ent
                                        .get_order_kinematic_pair()
                                        .cast::<StepKinematicsSurfacePair>();
                                    the_sw.send_entity(
                                        &step_live(&a_surface_pair, "WriteStep")?.surface1(),
                                    )?;
                                    the_sw.send_entity(
                                        &step_live(&a_surface_pair, "WriteStep")?.surface2(),
                                    )?;
                                    the_sw.send_boolean(
                                        step_live(&a_surface_pair, "WriteStep")?.orientation(),
                                    )?;
                                    step_start_entity(the_sw, "SURFACE_PAIR_WITH_RANGE")?;
                                    if the_ent
                                        .get_order_kinematic_pair()
                                        .is_kind(StepKinematicsSurfacePairWithRange::static_type())
                                    {
                                        let a_surface_pair_with_range: Handle<
                                            StepKinematicsSurfacePairWithRange,
                                        > = the_ent
                                            .get_order_kinematic_pair()
                                            .cast::<StepKinematicsSurfacePairWithRange>();
                                        the_sw.send_entity(
                                            &step_live(&a_surface_pair_with_range, "WriteStep")?
                                                .range_on_surface1(),
                                        )?;
                                        the_sw.send_entity(
                                            &step_live(&a_surface_pair_with_range, "WriteStep")?
                                                .range_on_surface2(),
                                        )?;
                                        if step_live(&a_surface_pair_with_range, "WriteStep")?
                                            .has_lower_limit_actual_rotation()
                                        {
                                            the_sw.send_real(
                                                step_live(&a_surface_pair_with_range, "WriteStep")?
                                                    .lower_limit_actual_rotation(),
                                            )?;
                                        } else {
                                            the_sw.send_undef()?;
                                        }
                                        if step_live(&a_surface_pair_with_range, "WriteStep")?
                                            .has_upper_limit_actual_rotation()
                                        {
                                            the_sw.send_real(
                                                step_live(&a_surface_pair_with_range, "WriteStep")?
                                                    .upper_limit_actual_rotation(),
                                            )?;
                                        } else {
                                            the_sw.send_undef()?;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        step_start_entity(the_sw, "REPRESENTATION_ITEM")?;
        the_sw.send_entity(&the_ent.name())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepKinematicsActuatedKinPairAndOrderKinPair>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.get_order_kinematic_pair().cast());
        iter.add_item(
            &step_live(&the_ent.get_order_kinematic_pair(), "Share")?
                .joint()
                .cast(),
        );
        iter.add_item(&the_ent.get_actuated_kinematic_pair().cast());
        iter.add_item(
            &step_live(&the_ent.get_order_kinematic_pair(), "Share")?
                .item_defined_transformation()
                .cast(),
        );
        Ok(())
    }
}
