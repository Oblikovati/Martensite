#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepReprRwRepresentationRelationshipWithTransformation,
    "RwStepReprRwRepresentationRelationshipWithTransformation"
);
impl RwStepReprRwRepresentationRelationshipWithTransformation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprRepresentationRelationshipWithTransformation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            5,
            ach,
            "representation_relationship_with_transformation",
        )? {
            return Ok(());
        }
        rw_block_0149!(a_description, a_name, ach, data, num);
        rw_block_0125!(a_rep1, a_rep2, ach, data, num);
        let mut a_trans: StepReprTransformation = Default::default();
        data.read_entity_select(num, 5, "transformation_operator", ach, &mut a_trans)?;
        ent.init(&a_name, &a_description, &a_rep1, &a_rep2, &a_trans);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprRepresentationRelationshipWithTransformation>,
    ) -> Result<(), Failure> {
        rw_block_0165!(ent, sw);
        sw.send_entity(&ent.transformation_operator().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprRepresentationRelationshipWithTransformation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0205!(ent, iter);
        Ok(())
    }
}

step_rw_class!(RwStepReprRwShapeAspect, "RwStepReprRwShapeAspect");
impl RwStepReprRwShapeAspect {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprShapeAspect>,
    ) -> Result<(), Failure> {
        rw_block_0033!(
            a_description,
            a_name,
            a_of_shape,
            a_product_definitional,
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
        ent: &Handle<StepReprShapeAspect>,
    ) -> Result<(), Failure> {
        rw_block_0166!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprShapeAspect>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwShapeAspectDerivingRelationship,
    "RwStepReprRwShapeAspectDerivingRelationship"
);
impl RwStepReprRwShapeAspectDerivingRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprShapeAspectDerivingRelationship>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "shape_aspect_deriving_relationship")? {
            return Ok(());
        }
        rw_block_0032!(
            a_shape_aspect_relationship_description,
            a_shape_aspect_relationship_name,
            a_shape_aspect_relationship_related_shape_aspect,
            a_shape_aspect_relationship_relating_shape_aspect,
            ach,
            data,
            ent,
            has_shape_aspect_relationship_description,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprShapeAspectDerivingRelationship>,
    ) -> Result<(), Failure> {
        rw_block_0128!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprShapeAspectDerivingRelationship>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.relating_shape_aspect().cast());
        iter.add_item(&ent.parent.related_shape_aspect().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwShapeAspectRelationship,
    "RwStepReprRwShapeAspectRelationship"
);
impl RwStepReprRwShapeAspectRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprShapeAspectRelationship>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "shape_aspect_relationship")? {
            return Ok(());
        }
        rw_block_0031!(
            a_description,
            a_name,
            a_related_shape_aspect,
            a_relating_shape_aspect,
            ach,
            data,
            ent,
            has_description,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprShapeAspectRelationship>,
    ) -> Result<(), Failure> {
        rw_block_0144!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprShapeAspectRelationship>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.relating_shape_aspect().cast());
        iter.add_item(&ent.related_shape_aspect().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwShapeAspectTransition,
    "RwStepReprRwShapeAspectTransition"
);
impl RwStepReprRwShapeAspectTransition {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprShapeAspectTransition>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "shape_aspect_transition")? {
            return Ok(());
        }
        rw_block_0032!(
            a_shape_aspect_relationship_description,
            a_shape_aspect_relationship_name,
            a_shape_aspect_relationship_related_shape_aspect,
            a_shape_aspect_relationship_relating_shape_aspect,
            ach,
            data,
            ent,
            has_shape_aspect_relationship_description,
            num
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprShapeAspectTransition>,
    ) -> Result<(), Failure> {
        rw_block_0128!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprShapeAspectTransition>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.relating_shape_aspect().cast());
        iter.add_item(&ent.parent.related_shape_aspect().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwShapeRepresentationRelationshipWithTransformation,
    "RwStepReprRwShapeRepresentationRelationshipWithTransformation"
);
impl RwStepReprRwShapeRepresentationRelationshipWithTransformation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprShapeRepresentationRelationshipWithTransformation>,
    ) -> Result<(), Failure> {
        let mut num: i32 = 0;
        data.named_for_complex_short("REPRESENTATION_RELATIONSHIP", "RPRRLT", num0, &mut num, ach);
        rw_block_0042!(a_description, a_name, a_rep1, a_rep2, ach, data, num);
        data.named_for_complex_short(
            "REPRESENTATION_RELATIONSHIP_WITH_TRANSFORMATION",
            "RRWT",
            num0,
            &mut num,
            ach,
        );
        if !data.check_nb_params(
            num,
            1,
            ach,
            "representation_relationship_with_transformation",
        )? {
            return Ok(());
        }
        let mut a_trans: StepReprTransformation = Default::default();
        data.read_entity_select(num, 1, "transformation_operator", ach, &mut a_trans)?;
        data.named_for_complex_short(
            "SHAPE_REPRESENTATION_RELATIONSHIP",
            "SHRPRL",
            num0,
            &mut num,
            ach,
        );
        if !data.check_nb_params(num, 0, ach, "shape_representation_relationship")? {
            return Ok(());
        }
        ent.init(&a_name, &a_description, &a_rep1, &a_rep2, &a_trans);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprShapeRepresentationRelationshipWithTransformation>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "REPRESENTATION_RELATIONSHIP")?;
        rw_block_0165!(ent, sw);
        step_start_entity(sw, "REPRESENTATION_RELATIONSHIP_WITH_TRANSFORMATION")?;
        sw.send_entity(&ent.transformation_operator().value())?;
        step_start_entity(sw, "SHAPE_REPRESENTATION_RELATIONSHIP")?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprShapeRepresentationRelationshipWithTransformation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0205!(ent, iter);
        Ok(())
    }
}
