#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepBasicRwProductDefinitionFormationWithSpecifiedSource,
    "RwStepBasicRwProductDefinitionFormationWithSpecifiedSource"
);
impl RwStepBasicRwProductDefinitionFormationWithSpecifiedSource {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicProductDefinitionFormationWithSpecifiedSource>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            4,
            ach,
            "product_definition_formation_with_specified_source",
        )? {
            return Ok(());
        }
        rw_block_0148!(a_description, a_id, ach, data, num);
        let mut a_of_product: Handle<StepBasicProduct> = Default::default();
        data.read_entity(
            num,
            3,
            "of_product",
            ach,
            StepBasicProduct::static_type(),
            &mut a_of_product,
        )?;
        let mut a_make_or_buy: StepBasicSource = StepBasicSource::SNotKnown;
        if data.param_type(num, 4)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 4)?;
            if !RwStepBasicRwSource::convert_to_enum(&text, &mut a_make_or_buy) {
                ach.add_fail_text("Enumeration source has not an allowed value", "");
            }
        } else {
            ach.add_fail_text("Parameter #4 (make_or_buy) is not an enumeration", "");
        }
        ent.init(&a_id, &a_description, &a_of_product, a_make_or_buy);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicProductDefinitionFormationWithSpecifiedSource>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.id())?;
        sw.send_entity(&ent.description())?;
        sw.send_entity(&ent.of_product())?;
        sw.send_enum(RwStepBasicRwSource::convert_to_string(ent.make_or_buy()))?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicProductDefinitionFormationWithSpecifiedSource>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.of_product().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwProductDefinitionReference,
    "RwStepBasicRwProductDefinitionReference"
);
impl RwStepBasicRwProductDefinitionReference {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicProductDefinitionReference>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "product_definition_reference")? {
            return Ok(());
        }
        let mut a_source: Handle<StepBasicExternalSource> = Default::default();
        data.read_entity(
            num,
            1,
            "source",
            ach,
            StepBasicExternalSource::static_type(),
            &mut a_source,
        )?;
        let mut a_product_id: Handle<HAsciiString> = Default::default();
        data.read_string(num, 2, "product_id", ach, &mut a_product_id)?;
        let mut a_product_definition_formation_id: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            3,
            "product_definition_formation_id",
            ach,
            &mut a_product_definition_formation_id,
        )?;
        let mut a_product_definition_id: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            4,
            "product_definition_id",
            ach,
            &mut a_product_definition_id,
        )?;
        let mut a_id_owning_organization_name: Handle<HAsciiString> = Default::default();
        if data.is_param_defined(num, 5)? {
            data.read_string(
                num,
                5,
                "id_owning_organization_name",
                ach,
                &mut a_id_owning_organization_name,
            )?;
        }
        ent.init(
            &a_source,
            &a_product_id,
            &a_product_definition_formation_id,
            &a_product_definition_id,
            &a_id_owning_organization_name,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicProductDefinitionReference>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.source())?;
        sw.send_entity(&ent.product_id())?;
        sw.send_entity(&ent.product_definition_formation_id())?;
        sw.send_entity(&ent.product_definition_id())?;
        if ent.has_id_owning_organization_name() {
            sw.send_entity(&ent.id_owning_organization_name())?;
        } else {
            sw.send_undef()?;
        }
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicProductDefinitionReference>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.source().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwProductDefinitionReferenceWithLocalRepresentation,
    "RwStepBasicRwProductDefinitionReferenceWithLocalRepresentation"
);
impl RwStepBasicRwProductDefinitionReferenceWithLocalRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicProductDefinitionReferenceWithLocalRepresentation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(
            num,
            5,
            ach,
            "product_definition_reference_with_local_representation",
        )? {
            return Ok(());
        }
        let mut a_source: Handle<StepBasicExternalSource> = Default::default();
        data.read_entity(
            num,
            1,
            "source",
            ach,
            StepBasicExternalSource::static_type(),
            &mut a_source,
        )?;
        let mut a_id: Handle<HAsciiString> = Default::default();
        data.read_string(num, 2, "id", ach, &mut a_id)?;
        let mut a_description: Handle<HAsciiString> = Default::default();
        if data.is_param_defined(num, 3)? {
            data.read_string(num, 3, "description", ach, &mut a_description)?;
        }
        let mut a_formation: Handle<StepBasicProductDefinitionFormation> = Default::default();
        data.read_entity(
            num,
            4,
            "formation",
            ach,
            StepBasicProductDefinitionFormation::static_type(),
            &mut a_formation,
        )?;
        let mut a_frame_of_reference: Handle<StepBasicProductDefinitionContext> =
            Default::default();
        data.read_entity(
            num,
            5,
            "frame_of_reference",
            ach,
            StepBasicProductDefinitionContext::static_type(),
            &mut a_frame_of_reference,
        )?;
        ent.init(
            &a_source,
            &a_id,
            &a_description,
            &a_formation,
            &a_frame_of_reference,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicProductDefinitionReferenceWithLocalRepresentation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.source())?;
        rw_block_0164!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicProductDefinitionReferenceWithLocalRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.source().cast());
        iter.get_one_item(&ent.formation().cast());
        iter.get_one_item(&ent.frame_of_reference().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwProductDefinitionRelationship,
    "RwStepBasicRwProductDefinitionRelationship"
);
impl RwStepBasicRwProductDefinitionRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicProductDefinitionRelationship>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "product_definition_relationship")? {
            return Ok(());
        }
        rw_block_0068!(a_description, a_id, a_name, ach, data, has_description, num);
        let mut a_relating_product_definition: StepBasicProductDefinitionOrReference =
            Default::default();
        data.read_entity_select(
            num,
            4,
            "relating_product_definition",
            ach,
            &mut a_relating_product_definition,
        )?;
        let mut a_related_product_definition: StepBasicProductDefinitionOrReference =
            Default::default();
        data.read_entity_select(
            num,
            5,
            "related_product_definition",
            ach,
            &mut a_related_product_definition,
        )?;
        ent.init_product_definition_or_reference(
            &a_id,
            &a_name,
            has_description,
            &a_description,
            &a_relating_product_definition,
            &a_related_product_definition,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicProductDefinitionRelationship>,
    ) -> Result<(), Failure> {
        rw_block_0178!(ent, sw);
        sw.send_entity(&ent.relating_product_definition_ap242().value())?;
        sw.send_entity(&ent.related_product_definition_ap242().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicProductDefinitionRelationship>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.relating_product_definition_ap242().value());
        iter.add_item(&ent.related_product_definition_ap242().value());
        Ok(())
    }
}
