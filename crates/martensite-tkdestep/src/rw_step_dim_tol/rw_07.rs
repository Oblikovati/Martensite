#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepDimTolRwGeometricToleranceWithMaximumTolerance,
    "RwStepDimTolRwGeometricToleranceWithMaximumTolerance"
);
impl RwStepDimTolRwGeometricToleranceWithMaximumTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolGeometricToleranceWithMaximumTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 6, ach, "geometric_tolerance_with_modifiers")? {
            return Ok(());
        }
        rw_block_b_0042!(
            a_description,
            a_magnitude,
            a_modifiers,
            a_name,
            a_toleranced_shape_aspect,
            ach,
            data,
            num,
            sub5
        );
        if data.read_sub_list(
            num,
            5,
            "geometric_tolerance_with_modifiers.modifiers",
            ach,
            &mut sub5,
            false,
            0,
            0,
        )? {
            let nb0: i32 = data.nb_params(sub5)?;
            a_modifiers = step_new_array1::<StepDimTolGeometricToleranceModifier>(1, nb0)?;
            let num2: i32 = sub5;
            for i0 in 1..=nb0 {
                let mut an_it0: StepDimTolGeometricToleranceModifier =
                    StepDimTolGeometricToleranceModifier::GTMMaximumMaterialRequirement;
                if data.param_type(num2, i0)? == InterfaceParamType::ParamEnum {
                    let text: Vec<u8> = data.param_c_value(num2, i0)?;
                    if text == b".ANY_CROSS_SECTION." {
                        an_it0 = StepDimTolGeometricToleranceModifier::GTMAnyCrossSection;
                    } else {
                        if text == b".COMMON_ZONE." {
                            an_it0 = StepDimTolGeometricToleranceModifier::GTMCommonZone;
                        } else {
                            if text == b".EACH_RADIAL_ELEMENT." {
                                an_it0 = StepDimTolGeometricToleranceModifier::GTMEachRadialElement;
                            } else {
                                if text == b".FREE_self.state.read()." {
                                    an_it0 = StepDimTolGeometricToleranceModifier::GTMFreeState;
                                } else {
                                    if text == b".LEAST_MATERIAL_REQUIREMENT." {
                                        an_it0 = StepDimTolGeometricToleranceModifier::GTMLeastMaterialRequirement;
                                    } else {
                                        if text == b".LINE_ELEMENT." {
                                            an_it0 = StepDimTolGeometricToleranceModifier::GTMLineElement;
                                        } else {
                                            if text == b".MAJOR_DIAMETER." {
                                                an_it0 = StepDimTolGeometricToleranceModifier::GTMMajorDiameter;
                                            } else {
                                                if text == b".MAXIMUM_MATERIAL_REQUIREMENT." {
                                                    an_it0 = StepDimTolGeometricToleranceModifier::GTMMaximumMaterialRequirement;
                                                } else {
                                                    if text == b".MINOR_DIAMETER." {
                                                        an_it0 = StepDimTolGeometricToleranceModifier::GTMMinorDiameter;
                                                    } else {
                                                        if text == b".NOT_CONVEX." {
                                                            an_it0 = StepDimTolGeometricToleranceModifier::GTMNotConvex;
                                                        } else {
                                                            if text == b".PITCH_DIAMETER." {
                                                                an_it0 = StepDimTolGeometricToleranceModifier::GTMPitchDiameter;
                                                            } else {
                                                                if text
                                                                    == b".RECIPROCITY_REQUIREMENT."
                                                                {
                                                                    an_it0 = StepDimTolGeometricToleranceModifier::GTMReciprocityRequirement;
                                                                } else {
                                                                    if text
                                                                        == b".SEPARATE_REQUIREMENT."
                                                                    {
                                                                        an_it0 = StepDimTolGeometricToleranceModifier::GTMSeparateRequirement;
                                                                    } else {
                                                                        if text == b".STATISTICAL_TOLERANCE." {
an_it0 = StepDimTolGeometricToleranceModifier::GTMStatisticalTolerance;
} else {
if text == b".TANGENT_PLANE." {
an_it0 = StepDimTolGeometricToleranceModifier::GTMTangentPlane;
} else {
ach.add_fail_text("Parameter #5 (modifiers) has not allowed value", "");
}
}
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    ach.add_fail_text("Parameter #5 (modifier) is not set of enumerations", "");
                }
                step_array1_set(&a_modifiers, i0, an_it0, "ReadStep")?;
            }
        }
        let mut a_maximum_upper_tolerance: Handle<StepBasicLengthMeasureWithUnit> =
            Default::default();
        data.read_entity(
            num,
            6,
            "maximum_upper_tolerance",
            ach,
            StepBasicLengthMeasureWithUnit::static_type(),
            &mut a_maximum_upper_tolerance,
        )?;
        ent.init(
            &a_name,
            &a_description,
            &a_magnitude,
            &a_toleranced_shape_aspect,
            &a_modifiers,
            &a_maximum_upper_tolerance,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolGeometricToleranceWithMaximumTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0062!(ent, i, sw);
        sw.send_entity(&ent.maximum_upper_tolerance())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolGeometricToleranceWithMaximumTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.magnitude());
        iter.add_item(&ent.toleranced_shape_aspect().value());
        Ok(())
    }
}
