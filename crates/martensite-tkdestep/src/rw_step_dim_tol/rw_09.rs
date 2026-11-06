#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepDimTolRwModifiedGeometricTolerance,
    "RwStepDimTolRwModifiedGeometricTolerance"
);
impl RwStepDimTolRwModifiedGeometricTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolModifiedGeometricTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "modified_geometric_tolerance")? {
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
        let mut a_modifier: StepDimTolLimitCondition =
            StepDimTolLimitCondition::MaximumMaterialCondition;
        if data.param_type(num, 5)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 5)?;
            if text == b".MAXIMUM_MATERIAL_CONDITION." {
                a_modifier = StepDimTolLimitCondition::MaximumMaterialCondition;
            } else {
                if text == b".LEAST_MATERIAL_CONDITION." {
                    a_modifier = StepDimTolLimitCondition::LeastMaterialCondition;
                } else {
                    if text == b".REGARDLESS_OF_FEATURE_SIZE." {
                        a_modifier = StepDimTolLimitCondition::RegardlessOfFeatureSize;
                    } else {
                        ach.add_fail_text("Parameter #5 (modifier) has not allowed value", "");
                    }
                }
            }
        } else {
            ach.add_fail_text("Parameter #5 (modifier) is not enumeration", "");
        }
        ent.init_geometric_tolerance_target(
            &a_geometric_tolerance_name,
            &a_geometric_tolerance_description,
            &a_geometric_tolerance_magnitude,
            &a_geometric_tolerance_toleranced_shape_aspect,
            a_modifier,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolModifiedGeometricTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0155!(ent, sw);
        if ent.modifier() == StepDimTolLimitCondition::MaximumMaterialCondition {
            sw.send_enum(b".MAXIMUM_MATERIAL_CONDITION.")?;
        } else {
            if ent.modifier() == StepDimTolLimitCondition::LeastMaterialCondition {
                sw.send_enum(b".LEAST_MATERIAL_CONDITION.")?;
            } else {
                if ent.modifier() == StepDimTolLimitCondition::RegardlessOfFeatureSize {
                    sw.send_enum(b".REGARDLESS_OF_FEATURE_SIZE.")?;
                }
            }
        }
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolModifiedGeometricTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.magnitude());
        iter.add_item(&ent.parent.toleranced_shape_aspect().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwNonUniformZoneDefinition,
    "RwStepDimTolRwNonUniformZoneDefinition"
);
impl RwStepDimTolRwNonUniformZoneDefinition {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolNonUniformZoneDefinition>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "non_uniform_zone_definition")? {
            return Ok(());
        }
        rw_block_b_0081!(
            a_tolerance_zone,
            ach,
            an_ent,
            an_items,
            data,
            i,
            nb_elements,
            nb_sub,
            num
        );
        ent.init(&a_tolerance_zone, &an_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolNonUniformZoneDefinition>,
    ) -> Result<(), Failure> {
        rw_block_b_0164!(ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolNonUniformZoneDefinition>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0151!(ent, i, iter, nb);
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwParallelismTolerance,
    "RwStepDimTolRwParallelismTolerance"
);
impl RwStepDimTolRwParallelismTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolParallelismTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "parallelism_tolerance")? {
            return Ok(());
        }
        rw_block_b_0027!(
            a_geometric_tolerance_description,
            a_geometric_tolerance_magnitude,
            a_geometric_tolerance_name,
            a_geometric_tolerance_toleranced_shape_aspect,
            a_geometric_tolerance_with_datum_reference_datum_system,
            ach,
            an_it0,
            data,
            ent,
            i0,
            nb0,
            num,
            num2,
            sub5
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolParallelismTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0073!(ent, i4, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolParallelismTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0173!(ent, i3, iter, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwPerpendicularityTolerance,
    "RwStepDimTolRwPerpendicularityTolerance"
);
impl RwStepDimTolRwPerpendicularityTolerance {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolPerpendicularityTolerance>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "perpendicularity_tolerance")? {
            return Ok(());
        }
        rw_block_b_0027!(
            a_geometric_tolerance_description,
            a_geometric_tolerance_magnitude,
            a_geometric_tolerance_name,
            a_geometric_tolerance_toleranced_shape_aspect,
            a_geometric_tolerance_with_datum_reference_datum_system,
            ach,
            an_it0,
            data,
            ent,
            i0,
            nb0,
            num,
            num2,
            sub5
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolPerpendicularityTolerance>,
    ) -> Result<(), Failure> {
        rw_block_b_0073!(ent, i4, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolPerpendicularityTolerance>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0173!(ent, i3, iter, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwPlacedDatumTargetFeature,
    "RwStepDimTolRwPlacedDatumTargetFeature"
);
impl RwStepDimTolRwPlacedDatumTargetFeature {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolPlacedDatumTargetFeature>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "placed_datum_target_feature")? {
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
        let mut a_datum_target_target_id: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            5,
            "datum_target.target_id",
            ach,
            &mut a_datum_target_target_id,
        )?;
        ent.init(
            &a_shape_aspect_name,
            &a_shape_aspect_description,
            &a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
            &a_datum_target_target_id,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolPlacedDatumTargetFeature>,
    ) -> Result<(), Failure> {
        rw_block_b_0150!(ent, sw);
        sw.send_entity(&ent.parent.target_id())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolPlacedDatumTargetFeature>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.parent.of_shape().cast());
        Ok(())
    }
}
