#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepShapeRwAngularSize, "RwStepShapeRwAngularSize");
impl RwStepShapeRwAngularSize {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeAngularSize>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "angular_size")? {
            return Ok(());
        }
        rw_block_0135!(
            a_dimensional_size_applies_to,
            a_dimensional_size_name,
            ach,
            data,
            num
        );
        let mut a_angle_selection: StepShapeAngleRelator = StepShapeAngleRelator::Small;
        if data.param_type(num, 3)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 3)?;
            if text == b".EQUAL." {
                a_angle_selection = StepShapeAngleRelator::Equal;
            } else {
                if text == b".LARGE." {
                    a_angle_selection = StepShapeAngleRelator::Large;
                } else {
                    if text == b".SMALL." {
                        a_angle_selection = StepShapeAngleRelator::Small;
                    } else {
                        ach.add_fail_text(
                            "Parameter #3 (angle_selection) has not allowed value",
                            "",
                        );
                    }
                }
            }
        } else {
            ach.add_fail_text("Parameter #3 (angle_selection) is not enumeration", "");
        }
        ent.init(
            &a_dimensional_size_applies_to,
            &a_dimensional_size_name,
            a_angle_selection,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeAngularSize>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.applies_to())?;
        sw.send_entity(&ent.parent.name())?;
        rw_block_0215!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeAngularSize>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.applies_to().cast());
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwBlock, "RwStepShapeRwBlock");
impl RwStepShapeRwBlock {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeBlock>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "block")? {
            return Ok(());
        }
        rw_block_0036!(a_name, a_position, a_x, a_y, a_z, ach, data, num);
        ent.init(&a_name, &a_position, a_x, a_y, a_z);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeBlock>,
    ) -> Result<(), Failure> {
        rw_block_0119!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeBlock>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.position().cast());
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwBooleanResult, "RwStepShapeRwBooleanResult");
impl RwStepShapeRwBooleanResult {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeBooleanResult>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "boolean_result")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_operator: StepShapeBooleanOperator = StepShapeBooleanOperator::BoDifference;
        if data.param_type(num, 2)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 2)?;
            if !RwStepShapeRwBooleanOperator::convert_to_enum(&text, &mut a_operator) {
                ach.add_fail_text("Enumeration boolean_operator has not an allowed value", "");
            }
        } else {
            ach.add_fail_text("Parameter #2 (operator) is not an enumeration", "");
        }
        let mut a_solid_model1: Handle<StepShapeSolidModel> = Default::default();
        data.read_entity(
            num,
            3,
            "first_operand",
            ach,
            StepShapeSolidModel::static_type(),
            &mut a_solid_model1,
        )?;
        let mut a_first_operand: StepShapeBooleanOperand = Default::default();
        a_first_operand.set_solid_model(&a_solid_model1);
        let mut a_solid_model2: Handle<StepShapeSolidModel> = Default::default();
        data.read_entity(
            num,
            4,
            "second_operand",
            ach,
            StepShapeSolidModel::static_type(),
            &mut a_solid_model2,
        )?;
        let mut a_second_operand: StepShapeBooleanOperand = Default::default();
        a_second_operand.set_solid_model(&a_solid_model2);
        ent.init(&a_name, a_operator, &a_first_operand, &a_second_operand);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeBooleanResult>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_enum(RwStepShapeRwBooleanOperator::convert_to_string(
            ent.operator(),
        ))?;
        sw.send_entity(&ent.first_operand().solid_model())?;
        sw.send_entity(&ent.second_operand().solid_model())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeBooleanResult>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.first_operand().solid_model().cast());
        iter.get_one_item(&ent.second_operand().solid_model().cast());
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwBoxDomain, "RwStepShapeRwBoxDomain");
impl RwStepShapeRwBoxDomain {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeBoxDomain>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "box_domain")? {
            return Ok(());
        }
        let mut a_corner: Handle<StepGeomCartesianPoint> = Default::default();
        data.read_entity(
            num,
            1,
            "corner",
            ach,
            StepGeomCartesianPoint::static_type(),
            &mut a_corner,
        )?;
        let mut a_xlength: f64 = Default::default();
        data.read_real(num, 2, "xlength", ach, &mut a_xlength)?;
        let mut a_ylength: f64 = Default::default();
        data.read_real(num, 3, "ylength", ach, &mut a_ylength)?;
        let mut a_zlength: f64 = Default::default();
        data.read_real(num, 4, "zlength", ach, &mut a_zlength)?;
        ent.init(&a_corner, a_xlength, a_ylength, a_zlength);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeBoxDomain>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.corner())?;
        sw.send_real(ent.xlength())?;
        sw.send_real(ent.ylength())?;
        sw.send_real(ent.zlength())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeBoxDomain>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.corner().cast());
        Ok(())
    }
}
