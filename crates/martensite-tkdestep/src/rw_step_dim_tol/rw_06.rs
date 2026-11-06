#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepDimTolRwGeometricToleranceWithDatumReference,
    "RwStepDimTolRwGeometricToleranceWithDatumReference"
);
impl RwStepDimTolRwGeometricToleranceWithDatumReference {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolGeometricToleranceWithDatumReference>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "geometric_tolerance_with_datum_reference")? {
            return Ok(());
        }
        rw_block_b_0060!(
            a_geometric_tolerance_description,
            a_geometric_tolerance_magnitude,
            a_geometric_tolerance_name,
            a_geometric_tolerance_toleranced_shape_aspect,
            ach,
            data,
            num
        );
        let mut a_datum_system: Handle<HArray1<StepDimTolDatumSystemOrReference>> =
            Default::default();
        let mut sub5: i32 = 0;
        if data.read_sub_list(num, 5, "datum_system", ach, &mut sub5, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub5)?;
            a_datum_system = step_new_array1::<StepDimTolDatumSystemOrReference>(1, nb0)?;
            let num2: i32 = sub5;
            for i0 in 1..=nb0 {
                let mut an_it0: StepDimTolDatumSystemOrReference = Default::default();
                data.read_entity_select(num2, i0, "datum_system_or_reference", ach, &mut an_it0)?;
                step_array1_set(&a_datum_system, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init_geometric_tolerance_target(
            &a_geometric_tolerance_name,
            &a_geometric_tolerance_description,
            &a_geometric_tolerance_magnitude,
            &a_geometric_tolerance_toleranced_shape_aspect,
            &a_datum_system,
        )?;
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolGeometricToleranceWithDatumReference>,
    ) -> Result<(), Failure> {
        rw_block_b_0155!(ent, sw);
        sw.open_sub()?;
        for i4 in 1..=step_live(&ent.datum_system_ap242(), "WriteStep")?
            .array1()
            .length()
        {
            let var0: StepDimTolDatumSystemOrReference =
                step_array1_item(&ent.datum_system_ap242(), i4, "WriteStep")?;
            sw.send_entity(&var0.value())?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolGeometricToleranceWithDatumReference>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.magnitude());
        iter.add_item(&ent.parent.toleranced_shape_aspect().value());
        for i3 in 1..=step_live(&ent.datum_system_ap242(), "Share")?
            .array1()
            .length()
        {
            let var0: StepDimTolDatumSystemOrReference =
                step_array1_item(&ent.datum_system_ap242(), i3, "Share")?;
            iter.add_item(&var0.value());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwGeometricToleranceWithDefinedAreaUnit,
    "RwStepDimTolRwGeometricToleranceWithDefinedAreaUnit"
);
impl RwStepDimTolRwGeometricToleranceWithDefinedAreaUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolGeometricToleranceWithDefinedAreaUnit>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 7, ach, "geometric_tolerance_with_defined_area_unit")? {
            return Ok(());
        }
        rw_block_b_0053!(
            a_description,
            a_magnitude,
            a_name,
            a_toleranced_shape_aspect,
            ach,
            an_unit_size,
            data,
            num
        );
        data.read_entity(
            num,
            5,
            "geometric_tolerance_with_defined_unit.unit_size",
            ach,
            StepBasicLengthMeasureWithUnit::static_type(),
            &mut an_unit_size,
        )?;
        let mut a_type: StepDimTolAreaUnitType = StepDimTolAreaUnitType::Circular;
        if data.param_type(num, 6)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 6)?;
            if text == b".CIRCULAR." {
                a_type = StepDimTolAreaUnitType::Circular;
            } else {
                if text == b".RECTANGULAR." {
                    a_type = StepDimTolAreaUnitType::Rectangular;
                } else {
                    if text == b".SQUARE." {
                        a_type = StepDimTolAreaUnitType::Square;
                    } else {
                        ach.add_fail_text("Parameter #6 (area_type) has not allowed value", "");
                    }
                }
            }
        } else {
            ach.add_fail_text("Parameter #6 (area_type) is not enumerations", "");
        }
        let mut a_second_unit_size: Handle<StepBasicLengthMeasureWithUnit> = Default::default();
        let has_second_unit_size: bool = data.is_param_defined(num, 7)?;
        if has_second_unit_size {
            data.read_entity(
                num,
                7,
                "second_unit_size",
                ach,
                StepBasicLengthMeasureWithUnit::static_type(),
                &mut a_second_unit_size,
            )?;
        }
        ent.init(
            &a_name,
            &a_description,
            &a_magnitude,
            &a_toleranced_shape_aspect,
            &an_unit_size,
            a_type,
            has_second_unit_size,
            &a_second_unit_size,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolGeometricToleranceWithDefinedAreaUnit>,
    ) -> Result<(), Failure> {
        rw_block_b_0118!(ent, sw);
        if ent.area_type() == StepDimTolAreaUnitType::Circular {
            sw.send_enum(b".CIRCULAR.")?;
        } else {
            if ent.area_type() == StepDimTolAreaUnitType::Rectangular {
                sw.send_enum(b".RECTANGULAR.")?;
            } else {
                if ent.area_type() == StepDimTolAreaUnitType::Square {
                    sw.send_enum(b".SQUARE.")?;
                }
            }
        }
        if ent.has_second_unit_size() {
            sw.send_entity(&ent.second_unit_size())?;
        } else {
            sw.send_undef()?;
        }
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolGeometricToleranceWithDefinedAreaUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.magnitude());
        iter.add_item(&ent.toleranced_shape_aspect().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwGeometricToleranceWithDefinedUnit,
    "RwStepDimTolRwGeometricToleranceWithDefinedUnit"
);
impl RwStepDimTolRwGeometricToleranceWithDefinedUnit {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolGeometricToleranceWithDefinedUnit>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "geometric_tolerance_with_defined_unit")? {
            return Ok(());
        }
        rw_block_b_0053!(
            a_description,
            a_magnitude,
            a_name,
            a_toleranced_shape_aspect,
            ach,
            an_unit_size,
            data,
            num
        );
        data.read_entity(
            num,
            5,
            "unit_size",
            ach,
            StepBasicLengthMeasureWithUnit::static_type(),
            &mut an_unit_size,
        )?;
        ent.init_geometric_tolerance_target(
            &a_name,
            &a_description,
            &a_magnitude,
            &a_toleranced_shape_aspect,
            &an_unit_size,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolGeometricToleranceWithDefinedUnit>,
    ) -> Result<(), Failure> {
        rw_block_b_0118!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolGeometricToleranceWithDefinedUnit>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.magnitude());
        iter.add_item(&ent.toleranced_shape_aspect().value());
        Ok(())
    }
}
