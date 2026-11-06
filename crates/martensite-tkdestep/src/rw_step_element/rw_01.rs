#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepElementRwAnalysisItemWithinRepresentation,
    "RwStepElementRwAnalysisItemWithinRepresentation"
);
impl RwStepElementRwAnalysisItemWithinRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepElementAnalysisItemWithinRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "analysis_item_within_representation")? {
            return Ok(());
        }
        rw_block_b_0147!(a_description, a_name, ach, data, num);
        let mut a_item: Handle<StepReprRepresentationItem> = Default::default();
        data.read_entity(
            num,
            3,
            "item",
            ach,
            StepReprRepresentationItem::static_type(),
            &mut a_item,
        )?;
        let mut a_rep: Handle<StepReprRepresentation> = Default::default();
        data.read_entity(
            num,
            4,
            "rep",
            ach,
            StepReprRepresentation::static_type(),
            &mut a_rep,
        )?;
        ent.init(&a_name, &a_description, &a_item, &a_rep);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepElementAnalysisItemWithinRepresentation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.description())?;
        sw.send_entity(&ent.item())?;
        sw.send_entity(&ent.rep())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepElementAnalysisItemWithinRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.item().cast());
        iter.add_item(&ent.rep().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepElementRwCurve3dElementDescriptor,
    "RwStepElementRwCurve3dElementDescriptor"
);
impl RwStepElementRwCurve3dElementDescriptor {
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepElementCurve3dElementDescriptor>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepElementRwCurveElementEndReleasePacket,
    "RwStepElementRwCurveElementEndReleasePacket"
);
impl RwStepElementRwCurveElementEndReleasePacket {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepElementCurveElementEndReleasePacket>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "curve_element_end_release_packet")? {
            return Ok(());
        }
        let mut a_release_freedom: StepElementCurveElementFreedom = Default::default();
        data.read_entity_select(num, 1, "release_freedom", ach, &mut a_release_freedom)?;
        let mut a_release_stiffness: f64 = Default::default();
        data.read_real(num, 2, "release_stiffness", ach, &mut a_release_stiffness)?;
        ent.init(&a_release_freedom, a_release_stiffness);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepElementCurveElementEndReleasePacket>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.release_freedom().value())?;
        sw.send_real(ent.release_stiffness())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepElementCurveElementEndReleasePacket>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.release_freedom().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepElementRwCurveElementSectionDefinition,
    "RwStepElementRwCurveElementSectionDefinition"
);
impl RwStepElementRwCurveElementSectionDefinition {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepElementCurveElementSectionDefinition>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "curve_element_section_definition")? {
            return Ok(());
        }
        let mut a_description: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "description", ach, &mut a_description)?;
        let mut a_section_angle: f64 = Default::default();
        data.read_real(num, 2, "section_angle", ach, &mut a_section_angle)?;
        ent.init(&a_description, a_section_angle);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepElementCurveElementSectionDefinition>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.description())?;
        sw.send_real(ent.section_angle())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepElementCurveElementSectionDefinition>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}
