#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepReprRwDataEnvironment, "RwStepReprRwDataEnvironment");
impl RwStepReprRwDataEnvironment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprDataEnvironment>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "data_environment")? {
            return Ok(());
        }
        rw_block_0149!(a_description, a_name, ach, data, num);
        let mut a_elements: Handle<HArray1<Handle<StepReprPropertyDefinitionRepresentation>>> =
            Default::default();
        let mut sub3: i32 = 0;
        if data.read_sub_list(num, 3, "elements", ach, &mut sub3, false, 0, 0)? {
            let nb0: i32 = data.nb_params(sub3)?;
            a_elements =
                step_new_array1::<Handle<StepReprPropertyDefinitionRepresentation>>(1, nb0)?;
            let num2: i32 = sub3;
            for i0 in 1..=nb0 {
                let mut an_it0: Handle<StepReprPropertyDefinitionRepresentation> =
                    Default::default();
                data.read_entity(
                    num2,
                    i0,
                    "property_definition_representation",
                    ach,
                    StepReprPropertyDefinitionRepresentation::static_type(),
                    &mut an_it0,
                )?;
                step_array1_set(&a_elements, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        ent.init(&a_name, &a_description, &a_elements);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprDataEnvironment>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.description())?;
        sw.open_sub()?;
        for i2 in 1..=step_live(&ent.elements(), "WriteStep")?.array1().length() {
            let var0: Handle<StepReprPropertyDefinitionRepresentation> =
                step_array1_item(&ent.elements(), i2, "WriteStep")?;
            sw.send_entity(&var0)?;
        }
        sw.close_sub()?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprDataEnvironment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        for i1 in 1..=step_live(&ent.elements(), "Share")?.array1().length() {
            let var0: Handle<StepReprPropertyDefinitionRepresentation> =
                step_array1_item(&ent.elements(), i1, "Share")?;
            iter.add_item(&var0.cast());
        }
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwDefinitionalRepresentation,
    "RwStepReprRwDefinitionalRepresentation"
);
impl RwStepReprRwDefinitionalRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprDefinitionalRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "definitional_representation")? {
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
        ent: &Handle<StepReprDefinitionalRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_0116!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprDefinitionalRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0190!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwDerivedShapeAspect,
    "RwStepReprRwDerivedShapeAspect"
);
impl RwStepReprRwDerivedShapeAspect {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprDerivedShapeAspect>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "derived_shape_aspect")? {
            return Ok(());
        }
        rw_block_0045!(
            a_shape_aspect_description,
            a_shape_aspect_name,
            a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
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
        ent: &Handle<StepReprDerivedShapeAspect>,
    ) -> Result<(), Failure> {
        rw_block_0158!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprDerivedShapeAspect>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwDescriptiveRepresentationItem,
    "RwStepReprRwDescriptiveRepresentationItem"
);
impl RwStepReprRwDescriptiveRepresentationItem {
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprDescriptiveRepresentationItem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.description())?;
        Ok(())
    }
}

step_rw_class!(RwStepReprRwExtension, "RwStepReprRwExtension");
impl RwStepReprRwExtension {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprExtension>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "extension")? {
            return Ok(());
        }
        rw_block_0045!(
            a_shape_aspect_description,
            a_shape_aspect_name,
            a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
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
        ent: &Handle<StepReprExtension>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.parent.name())?;
        sw.send_entity(&ent.parent.parent.description())?;
        sw.send_entity(&ent.parent.parent.of_shape())?;
        sw.send_logical(ent.parent.parent.product_definitional())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprExtension>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.parent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwFeatureForDatumTargetRelationship,
    "RwStepReprRwFeatureForDatumTargetRelationship"
);
impl RwStepReprRwFeatureForDatumTargetRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprFeatureForDatumTargetRelationship>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "feature_for_datum_target-relationship")? {
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
        ent: &Handle<StepReprFeatureForDatumTargetRelationship>,
    ) -> Result<(), Failure> {
        rw_block_0144!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprFeatureForDatumTargetRelationship>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.relating_shape_aspect().cast());
        iter.add_item(&ent.related_shape_aspect().cast());
        Ok(())
    }
}
