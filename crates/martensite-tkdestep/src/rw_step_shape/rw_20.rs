#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepShapeRwSweptFaceSolid, "RwStepShapeRwSweptFaceSolid");
impl RwStepShapeRwSweptFaceSolid {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeSweptFaceSolid>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "swept_face_solid")? {
            return Ok(());
        }
        rw_block_0138!(a_name, a_swept_face, ach, data, num);
        ent.init(&a_name, &a_swept_face);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeSweptFaceSolid>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.swept_face())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeSweptFaceSolid>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.swept_face().cast());
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwToleranceValue, "RwStepShapeRwToleranceValue");
impl RwStepShapeRwToleranceValue {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeToleranceValue>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "tolerance_value")? {
            return Ok(());
        }
        let mut a_lower_bound: Handle<StandardTransient> = Default::default();
        data.read_entity(
            num,
            1,
            "lower_bound",
            ach,
            StandardTransient::static_type(),
            &mut a_lower_bound,
        )?;
        let mut an_upper_bound: Handle<StandardTransient> = Default::default();
        data.read_entity(
            num,
            2,
            "upper_bound",
            ach,
            StandardTransient::static_type(),
            &mut an_upper_bound,
        )?;
        if !a_lower_bound.is_null() && !an_upper_bound.is_null() {
            ach.clear_fails();
        }
        ent.init(&a_lower_bound, &an_upper_bound);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeToleranceValue>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.lower_bound())?;
        sw.send_entity(&ent.upper_bound())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeToleranceValue>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.lower_bound());
        iter.add_item(&ent.upper_bound());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwTopologicalRepresentationItem,
    "RwStepShapeRwTopologicalRepresentationItem"
);
impl RwStepShapeRwTopologicalRepresentationItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeTopologicalRepresentationItem>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "topological_representation_item")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        ent.init(&a_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeTopologicalRepresentationItem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwTorus, "RwStepShapeRwTorus");
impl RwStepShapeRwTorus {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeTorus>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "torus")? {
            return Ok(());
        }
        rw_block_0139!(a_name, a_position, ach, data, num);
        let mut a_major_radius: f64 = Default::default();
        rw_block_0204!(a_major_radius, a_minor_radius, ach, data, num);
        ent.init(&a_name, &a_position, a_major_radius, a_minor_radius);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeTorus>,
    ) -> Result<(), Failure> {
        rw_block_0167!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeTorus>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.position().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepShapeRwTransitionalShapeRepresentation,
    "RwStepShapeRwTransitionalShapeRepresentation"
);
impl RwStepShapeRwTransitionalShapeRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeTransitionalShapeRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "transitional_shape_representation")? {
            return Ok(());
        }
        rw_block_0039!(
            a_context_of_items,
            a_items,
            a_name,
            ach,
            anent2,
            data,
            ent,
            i2,
            nb2,
            nsub2,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeTransitionalShapeRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_0116!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepShapeTransitionalShapeRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0190!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(RwStepShapeRwTypeQualifier, "RwStepShapeRwTypeQualifier");
impl RwStepShapeRwTypeQualifier {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepShapeTypeQualifier>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "type_qualifier")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        ent.init(&a_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepShapeTypeQualifier>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}
