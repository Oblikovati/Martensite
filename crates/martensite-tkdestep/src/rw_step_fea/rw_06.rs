#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepFEARwFeaAxis2Placement3d,
    "RwStepFEARwFeaAxis2Placement3d"
);
impl RwStepFEARwFeaAxis2Placement3d {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaAxis2Placement3d>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 6, ach, "fea_axis2_placement3d")? {
            return Ok(());
        }
        let mut a_representation_item_name: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            1,
            "representation_item.name",
            ach,
            &mut a_representation_item_name,
        )?;
        let mut a_placement_location: Handle<StepGeomCartesianPoint> = Default::default();
        data.read_entity(
            num,
            2,
            "placement.location",
            ach,
            StepGeomCartesianPoint::static_type(),
            &mut a_placement_location,
        )?;
        let mut a_axis2_placement3d_axis: Handle<StepGeomDirection> = Default::default();
        let mut has_axis2_placement3d_axis: bool = true;
        if data.is_param_defined(num, 3)? {
            data.read_entity(
                num,
                3,
                "axis2_placement3d.axis",
                ach,
                StepGeomDirection::static_type(),
                &mut a_axis2_placement3d_axis,
            )?;
        } else {
            has_axis2_placement3d_axis = false;
        }
        let mut a_axis2_placement3d_ref_direction: Handle<StepGeomDirection> = Default::default();
        let mut has_axis2_placement3d_ref_direction: bool = true;
        if data.is_param_defined(num, 4)? {
            data.read_entity(
                num,
                4,
                "axis2_placement3d.ref_direction",
                ach,
                StepGeomDirection::static_type(),
                &mut a_axis2_placement3d_ref_direction,
            )?;
        } else {
            has_axis2_placement3d_ref_direction = false;
        }
        let mut a_system_type: StepFEACoordinateSystemType = StepFEACoordinateSystemType::Cartesian;
        if data.param_type(num, 5)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 5)?;
            if text == b".CARTESIAN." {
                a_system_type = StepFEACoordinateSystemType::Cartesian;
            } else {
                if text == b".CYLINDRICAL." {
                    a_system_type = StepFEACoordinateSystemType::Cylindrical;
                } else {
                    if text == b".SPHERICAL." {
                        a_system_type = StepFEACoordinateSystemType::Spherical;
                    } else {
                        ach.add_fail_text("Parameter #5 (system_type) has not allowed value", "");
                    }
                }
            }
        } else {
            ach.add_fail_text("Parameter #5 (system_type) is not enumeration", "");
        }
        let mut a_description: Handle<HAsciiString> = Default::default();
        data.read_string(num, 6, "description", ach, &mut a_description)?;
        ent.init(
            &a_representation_item_name,
            &a_placement_location,
            has_axis2_placement3d_axis,
            &a_axis2_placement3d_axis,
            has_axis2_placement3d_ref_direction,
            &a_axis2_placement3d_ref_direction,
            a_system_type,
            &a_description,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAFeaAxis2Placement3d>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.parent.parent.name())?;
        sw.send_entity(&ent.parent.parent.location())?;
        if ent.parent.has_axis() {
            sw.send_entity(&ent.parent.axis())?;
        } else {
            sw.send_undef()?;
        }
        if ent.parent.has_ref_direction() {
            sw.send_entity(&ent.parent.ref_direction())?;
        } else {
            sw.send_undef()?;
        }
        if ent.system_type() == StepFEACoordinateSystemType::Cartesian {
            sw.send_enum(b".CARTESIAN.")?;
        } else {
            if ent.system_type() == StepFEACoordinateSystemType::Cylindrical {
                sw.send_enum(b".CYLINDRICAL.")?;
            } else {
                if ent.system_type() == StepFEACoordinateSystemType::Spherical {
                    sw.send_enum(b".SPHERICAL.")?;
                }
            }
        }
        sw.send_entity(&ent.description())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFeaAxis2Placement3d>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.parent.location().cast());
        if ent.parent.has_axis() {
            iter.add_item(&ent.parent.axis().cast());
        }
        if ent.parent.has_ref_direction() {
            iter.add_item(&ent.parent.ref_direction().cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwFeaCurveSectionGeometricRelationship,
    "RwStepFEARwFeaCurveSectionGeometricRelationship"
);
impl RwStepFEARwFeaCurveSectionGeometricRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaCurveSectionGeometricRelationship>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "fea_curve_section_geometric_relationship")? {
            return Ok(());
        }
        let mut a_section_ref: Handle<StepElementCurveElementSectionDefinition> =
            Default::default();
        data.read_entity(
            num,
            1,
            "section_ref",
            ach,
            StepElementCurveElementSectionDefinition::static_type(),
            &mut a_section_ref,
        )?;
        rw_block_b_0189!(a_item, a_section_ref, ach, data, ent, num);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAFeaCurveSectionGeometricRelationship>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.section_ref())?;
        sw.send_entity(&ent.item())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFeaCurveSectionGeometricRelationship>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.section_ref().cast());
        iter.add_item(&ent.item().cast());
        Ok(())
    }
}

step_rw_class!(RwStepFEARwFeaGroup, "RwStepFEARwFeaGroup");
impl RwStepFEARwFeaGroup {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaGroup>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "fea_group")? {
            return Ok(());
        }
        rw_block_b_0149!(a_group_description, a_group_name, ach, data, num);
        let mut a_model_ref: Handle<StepFEAFeaModel> = Default::default();
        data.read_entity(
            num,
            3,
            "model_ref",
            ach,
            StepFEAFeaModel::static_type(),
            &mut a_model_ref,
        )?;
        ent.init(&a_group_name, &a_group_description, &a_model_ref);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAFeaGroup>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.name())?;
        sw.send_entity(&ent.parent.description())?;
        sw.send_entity(&ent.model_ref())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAFeaGroup>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.model_ref().cast());
        Ok(())
    }
}
