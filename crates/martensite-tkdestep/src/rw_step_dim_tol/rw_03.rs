#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepDimTolRwDatumReferenceModifierWithValue,
    "RwStepDimTolRwDatumReferenceModifierWithValue"
);
impl RwStepDimTolRwDatumReferenceModifierWithValue {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolDatumReferenceModifierWithValue>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "datum_reference_modifier_with_value")? {
            return Ok(());
        }
        let mut a_modifier_type: StepDimTolDatumReferenceModifierType =
            StepDimTolDatumReferenceModifierType::CircularOrCylindrical;
        if data.param_type(num, 1)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 1)?;
            if text == b".CIRCULAR_OR_CYLINDRICAL." {
                a_modifier_type = StepDimTolDatumReferenceModifierType::CircularOrCylindrical;
            } else {
                if text == b".DISTANCE." {
                    a_modifier_type = StepDimTolDatumReferenceModifierType::Distance;
                } else {
                    if text == b".PROJECTED." {
                        a_modifier_type = StepDimTolDatumReferenceModifierType::Projected;
                    } else {
                        if text == b".SPHERICAL." {
                            a_modifier_type = StepDimTolDatumReferenceModifierType::Spherical;
                        } else {
                            ach.add_fail_text(
                                "Parameter #1 (modifier_type) has not allowed value",
                                "",
                            );
                        }
                    }
                }
            }
        } else {
            ach.add_fail_text("Parameter #1 (modifier_type) is not an enumeration", "");
        }
        let mut a_modifier_value: Handle<StepBasicLengthMeasureWithUnit> = Default::default();
        data.read_entity(
            num,
            2,
            "modifier_value",
            ach,
            StepBasicLengthMeasureWithUnit::static_type(),
            &mut a_modifier_value,
        )?;
        ent.init(a_modifier_type, &a_modifier_value);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolDatumReferenceModifierWithValue>,
    ) -> Result<(), Failure> {
        if ent.modifier_type() == StepDimTolDatumReferenceModifierType::CircularOrCylindrical {
            sw.send_enum(b".CIRCULAR_OR_CYLINDRICAL.")?;
        } else {
            if ent.modifier_type() == StepDimTolDatumReferenceModifierType::Distance {
                sw.send_enum(b".DISTANCE.")?;
            } else {
                if ent.modifier_type() == StepDimTolDatumReferenceModifierType::Projected {
                    sw.send_enum(b".PROJECTED.")?;
                } else {
                    if ent.modifier_type() == StepDimTolDatumReferenceModifierType::Spherical {
                        sw.send_enum(b".SPHERICAL.")?;
                    }
                }
            }
        }
        sw.send_entity(&ent.modifier_value())?;
        Ok(())
    }
}

step_rw_class!(RwStepDimTolRwDatumSystem, "RwStepDimTolRwDatumSystem");
impl RwStepDimTolRwDatumSystem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolDatumSystem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "datum_system")? {
            return Ok(());
        }
        rw_block_b_0058!(
            a_shape_aspect_description,
            a_shape_aspect_name,
            a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
            ach,
            data,
            num
        );
        let mut a_constituents: Handle<HArray1<Handle<StepDimTolDatumReferenceCompartment>>> =
            Default::default();
        let mut an_ent: Handle<StepDimTolDatumReferenceCompartment> = Default::default();
        let mut nb_sub: i32 = Default::default();
        if data.read_sub_list(num, 5, "base", ach, &mut nb_sub, false, 0, 0)? {
            let nb_elements: i32 = data.nb_params(nb_sub)?;
            a_constituents =
                step_new_array1::<Handle<StepDimTolDatumReferenceCompartment>>(1, nb_elements)?;
            for i in 1..=nb_elements {
                if data.read_entity(
                    nb_sub,
                    i,
                    "datum_reference_compartment",
                    ach,
                    StepDimTolDatumReferenceCompartment::static_type(),
                    &mut an_ent,
                )? {
                    step_array1_set(&a_constituents, i, an_ent.clone(), "ReadStep")?;
                }
            }
        }
        ent.init(
            &a_shape_aspect_name,
            &a_shape_aspect_description,
            &a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
            &a_constituents,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolDatumSystem>,
    ) -> Result<(), Failure> {
        rw_block_b_0165!(ent, sw);
        let i: i32 = Default::default();
        let nb: i32 = ent.nb_constituents();
        sw.open_sub()?;
        for i in 1..=nb {
            sw.send_entity(&ent.constituents_value(i)?)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolDatumSystem>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.of_shape().cast());
        let i: i32 = Default::default();
        let nb: i32 = ent.nb_constituents();
        for i in 1..=nb {
            iter.add_item(&ent.constituents_value(i)?.cast());
        }
        Ok(())
    }
}

step_rw_class!(RwStepDimTolRwDatumTarget, "RwStepDimTolRwDatumTarget");
impl RwStepDimTolRwDatumTarget {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolDatumTarget>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "datum_target")? {
            return Ok(());
        }
        rw_block_b_0058!(
            a_shape_aspect_description,
            a_shape_aspect_name,
            a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
            ach,
            data,
            num
        );
        let mut a_target_id: Handle<HAsciiString> = Default::default();
        data.read_string(num, 5, "target_id", ach, &mut a_target_id)?;
        ent.init(
            &a_shape_aspect_name,
            &a_shape_aspect_description,
            &a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
            &a_target_id,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolDatumTarget>,
    ) -> Result<(), Failure> {
        rw_block_b_0158!(ent, sw);
        sw.send_entity(&ent.target_id())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolDatumTarget>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwFlatnessTolerance,
    "RwStepDimTolRwFlatnessTolerance"
);
impl RwStepDimTolRwFlatnessTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolFlatnessTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "flatness_tolerance")? {
            return Ok(());
        }
        rw_block_b_0052!(
            a_geometric_tolerance_description,
            a_geometric_tolerance_magnitude,
            a_geometric_tolerance_name,
            a_geometric_tolerance_toleranced_shape_aspect,
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
        ent: &Handle<StepDimTolFlatnessTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0155!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolFlatnessTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.magnitude());
        iter.add_item(&ent.parent.toleranced_shape_aspect().value());
        Ok(())
    }
}
