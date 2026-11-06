#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepDimTolRwGeneralDatumReference,
    "RwStepDimTolRwGeneralDatumReference"
);
impl RwStepDimTolRwGeneralDatumReference {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolGeneralDatumReference>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 6, ach, "general_datum_reference")? {
            return Ok(());
        }
        rw_block_b_0012!(
            a_base,
            a_datum,
            a_drmwv,
            a_member,
            a_modifiers,
            a_new_nb_sub,
            a_sdrm,
            a_shape_aspect_description,
            a_shape_aspect_name,
            a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
            a_type,
            ach,
            an_ent,
            an_enum_text,
            an_items,
            data,
            ent,
            has_modifiers,
            i,
            nb_elements,
            nb_sub,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolGeneralDatumReference>,
    ) -> Result<(), Failure> {
        rw_block_b_0063!(a_base_type, a_modifier, a_type, an_array, ent, i, nb, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolGeneralDatumReference>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0167!(a_base_type, an_array, ent, i, iter, nb);
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwGeoTolAndGeoTolWthDatRef,
    "RwStepDimTolRwGeoTolAndGeoTolWthDatRef"
);
impl RwStepDimTolRwGeoTolAndGeoTolWthDatRef {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthDatRef>,
    ) -> Result<(), Failure> {
        rw_block_b_0013!(
            a_datum_system,
            a_description,
            a_gtwdr,
            a_magnitude,
            a_name,
            a_toleranced_shape_aspect,
            ach,
            an_it0,
            data,
            i0,
            nb0,
            num,
            num0,
            num2,
            sub5
        );
        rw_block_b_0122!(a_first, a_last, a_type, ach, data, num0);
        ent.init_geometric_tolerance_target(
            &a_name,
            &a_description,
            &a_magnitude,
            &a_toleranced_shape_aspect,
            &a_gtwdr,
            a_type,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthDatRef>,
    ) -> Result<(), Failure> {
        rw_block_b_0024!(a_type, ent, i4, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthDatRef>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0168!(ent, i3, iter, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol,
    "RwStepDimTolRwGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol"
);
impl RwStepDimTolRwGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol>,
    ) -> Result<(), Failure> {
        rw_block_b_0013!(
            a_datum_system,
            a_description,
            a_gtwdr,
            a_magnitude,
            a_name,
            a_toleranced_shape_aspect,
            ach,
            an_it0,
            data,
            i0,
            nb0,
            num,
            num0,
            num2,
            sub5
        );
        rw_block_b_0021!(
            a_first,
            a_gtwm,
            a_last,
            a_max_tol,
            a_modifiers,
            a_type,
            ach,
            an_it0,
            data,
            i0,
            nb0,
            num,
            num0,
            num2,
            sub,
            text
        );
        ent.init_geometric_tolerance_target(
            &a_name,
            &a_description,
            &a_magnitude,
            &a_toleranced_shape_aspect,
            &a_gtwdr,
            &a_gtwm,
            &a_max_tol,
            a_type,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol>,
    ) -> Result<(), Failure> {
        rw_block_b_0047!(a_type, ent, sw);
        rw_block_b_0128!(ent, i4, sw, var0);
        rw_block_b_0078!(a_gtwm, a_type, ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMaxTol>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0168!(ent, i3, iter, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwGeoTolAndGeoTolWthDatRefAndGeoTolWthMod,
    "RwStepDimTolRwGeoTolAndGeoTolWthDatRefAndGeoTolWthMod"
);
impl RwStepDimTolRwGeoTolAndGeoTolWthDatRefAndGeoTolWthMod {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMod>,
    ) -> Result<(), Failure> {
        rw_block_b_0013!(
            a_datum_system,
            a_description,
            a_gtwdr,
            a_magnitude,
            a_name,
            a_toleranced_shape_aspect,
            ach,
            an_it0,
            data,
            i0,
            nb0,
            num,
            num0,
            num2,
            sub5
        );
        rw_block_b_0034!(
            a_first,
            a_gtwm,
            a_last,
            a_modifiers,
            a_type,
            ach,
            an_it0,
            data,
            i0,
            nb0,
            num,
            num0,
            num2,
            sub,
            text
        );
        ent.init_geometric_tolerance_target(
            &a_name,
            &a_description,
            &a_magnitude,
            &a_toleranced_shape_aspect,
            &a_gtwdr,
            &a_gtwm,
            a_type,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMod>,
    ) -> Result<(), Failure> {
        rw_block_b_0030!(a_type, ent, i4, sw, var0);
        rw_block_b_0078!(a_gtwm, a_type, ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthDatRefAndGeoTolWthMod>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0168!(ent, i3, iter, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol,
    "RwStepDimTolRwGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol"
);
impl RwStepDimTolRwGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol>,
    ) -> Result<(), Failure> {
        rw_block_b_0017!(
            a_datum_system,
            a_description,
            a_magnitude,
            a_name,
            a_toleranced_shape_aspect,
            ach,
            an_it0,
            data,
            i0,
            nb0,
            num,
            num0,
            num2,
            sub5
        );
        let gtwdr: Handle<StepDimTolGeometricToleranceWithDatumReference> =
            Handle::new(StepDimTolGeometricToleranceWithDatumReference::new());
        gtwdr.set_datum_system_fields(&a_datum_system)?;
        data.named_for_complex_short(
            "MODIFIED_GEOMETRIC_TOLERANCE",
            "MDGMTL",
            num0,
            &mut num,
            ach,
        );
        let mut a_modifier: StepDimTolLimitCondition =
            StepDimTolLimitCondition::MaximumMaterialCondition;
        if data.param_type(num, 1)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 1)?;
            if text == b".MAXIMUM_MATERIAL_CONDITION." {
                a_modifier = StepDimTolLimitCondition::MaximumMaterialCondition;
            } else {
                if text == b".LEAST_MATERIAL_CONDITION." {
                    a_modifier = StepDimTolLimitCondition::LeastMaterialCondition;
                } else {
                    if text == b".REGARDLESS_OF_FEATURE_SIZE." {
                        a_modifier = StepDimTolLimitCondition::RegardlessOfFeatureSize;
                    } else {
                        ach.add_fail_text(
                            "Parameter in MODIFIED_GEOMETRIC_TOLERANCE has not allowed value",
                            "",
                        );
                    }
                }
            }
        } else {
            ach.add_fail_text(
                "Parameter in MODIFIED_GEOMETRIC_TOLERANCE is not enumeration",
                "",
            );
        }
        let mgt: Handle<StepDimTolModifiedGeometricTolerance> =
            Handle::new(StepDimTolModifiedGeometricTolerance::new());
        mgt.set_modifier(a_modifier);
        ent.init_geometric_tolerance_target(
            &a_name,
            &a_description,
            &a_magnitude,
            &a_toleranced_shape_aspect,
            &gtwdr,
            &mgt,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol>,
    ) -> Result<(), Failure> {
        rw_block_b_0050!(ent, i4, sw, var0);
        step_start_entity(sw, "MODIFIED_GEOMETRIC_TOLERANCE")?;
        if step_live(&ent.get_modified_geometric_tolerance(), "WriteStep")?.modifier()
            == StepDimTolLimitCondition::MaximumMaterialCondition
        {
            sw.send_enum(b".MAXIMUM_MATERIAL_CONDITION.")?;
        } else {
            if step_live(&ent.get_modified_geometric_tolerance(), "WriteStep")?.modifier()
                == StepDimTolLimitCondition::LeastMaterialCondition
            {
                sw.send_enum(b".LEAST_MATERIAL_CONDITION.")?;
            } else {
                if step_live(&ent.get_modified_geometric_tolerance(), "WriteStep")?.modifier()
                    == StepDimTolLimitCondition::RegardlessOfFeatureSize
                {
                    sw.send_enum(b".REGARDLESS_OF_FEATURE_SIZE.")?;
                }
            }
        }
        step_start_entity(sw, "POSITION_TOLERANCE")?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0168!(ent, i3, iter, var0);
        Ok(())
    }
}
