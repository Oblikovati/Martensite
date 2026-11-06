#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepFEARwCurveElementIntervalLinearlyVarying,
    "RwStepFEARwCurveElementIntervalLinearlyVarying"
);
impl RwStepFEARwCurveElementIntervalLinearlyVarying {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEACurveElementIntervalLinearlyVarying>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "curve_element_interval_linearly_varying")? {
            return Ok(());
        }
        rw_block_b_0132!(
            a_curve_element_interval_eu_angles,
            a_curve_element_interval_finish_position,
            ach,
            data,
            num
        );
        let mut a_sections: Handle<HArray1<Handle<StepElementCurveElementSectionDefinition>>> =
            Default::default();
        let mut sub3: i32 = 0;
        if data.read_sub_list(num, 3, "sections", ach, &mut sub3, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub3)?;
            a_sections =
                step_new_array1::<Handle<StepElementCurveElementSectionDefinition>>(1, nb0)?;
            let num2: i32 = sub3;
            for i0 in 1..=nb0 {
                let mut an_it0: Handle<StepElementCurveElementSectionDefinition> =
                    Default::default();
                data.read_entity(
                    num2,
                    i0,
                    "curve_element_section_definition",
                    ach,
                    StepElementCurveElementSectionDefinition::static_type(),
                    &mut an_it0,
                )?;
                step_array1_set(&a_sections, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(
            &a_curve_element_interval_finish_position,
            &a_curve_element_interval_eu_angles,
            &a_sections,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEACurveElementIntervalLinearlyVarying>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.finish_position())?;
        sw.send_entity(&ent.parent.eu_angles())?;
        sw.open_sub()?;
        for i2 in 1..=step_live(&ent.sections(), "WriteStep")?.array1().length() {
            let var0: Handle<StepElementCurveElementSectionDefinition> =
                step_array1_item(&ent.sections(), i2, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEACurveElementIntervalLinearlyVarying>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.finish_position().cast());
        iter.add_item(&ent.parent.eu_angles().cast());
        for i3 in 1..=step_live(&ent.sections(), "Share")?.array1().length() {
            let var0: Handle<StepElementCurveElementSectionDefinition> =
                step_array1_item(&ent.sections(), i3, "Share")?;
            iter.add_item(&var0.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwCurveElementLocation,
    "RwStepFEARwCurveElementLocation"
);
impl RwStepFEARwCurveElementLocation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEACurveElementLocation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "curve_element_location")? {
            return Ok(());
        }
        let mut a_coordinate: Handle<StepFEAFeaParametricPoint> = Default::default();
        data.read_entity(
            num,
            1,
            "coordinate",
            ach,
            StepFEAFeaParametricPoint::static_type(),
            &mut a_coordinate,
        )?;
        ent.init(&a_coordinate);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEACurveElementLocation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.coordinate())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEACurveElementLocation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.coordinate().cast());
        Ok(())
    }
}

step_rw_class!(RwStepFEARwDummyNode, "RwStepFEARwDummyNode");
impl RwStepFEARwDummyNode {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEADummyNode>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "dummy_node")? {
            return Ok(());
        }
        rw_block_b_0037!(
            a_node_representation_model_ref,
            a_representation_context_of_items,
            a_representation_items,
            a_representation_name,
            ach,
            an_it0,
            data,
            ent,
            i0,
            nb0,
            num,
            num2,
            sub2
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEADummyNode>,
    ) -> Result<(), Failure> {
        rw_block_b_0091!(ent, i1, sw, var0);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEADummyNode>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0174!(ent, i1, iter, var0);
        Ok(())
    }
}

step_rw_class!(
    RwStepFEARwElementGeometricRelationship,
    "RwStepFEARwElementGeometricRelationship"
);
impl RwStepFEARwElementGeometricRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAElementGeometricRelationship>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "element_geometric_relationship")? {
            return Ok(());
        }
        let mut a_element_ref: StepFEAElementOrElementGroup = Default::default();
        data.read_entity_select(num, 1, "element_ref", ach, &mut a_element_ref)?;
        let mut a_item: Handle<StepElementAnalysisItemWithinRepresentation> = Default::default();
        data.read_entity(
            num,
            2,
            "item",
            ach,
            StepElementAnalysisItemWithinRepresentation::static_type(),
            &mut a_item,
        )?;
        let mut a_aspect: StepElementElementAspect = Default::default();
        data.read_entity_select(num, 3, "aspect", ach, &mut a_aspect)?;
        ent.init(&a_element_ref, &a_item, &a_aspect);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAElementGeometricRelationship>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.element_ref().value())?;
        sw.send_entity(&ent.item())?;
        sw.send_entity(&ent.aspect().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepFEAElementGeometricRelationship>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.element_ref().value());
        iter.add_item(&ent.item().cast());
        iter.add_item(&ent.aspect().value());
        Ok(())
    }
}
