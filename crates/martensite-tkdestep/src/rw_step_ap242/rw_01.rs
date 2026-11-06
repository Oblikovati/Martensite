#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepAP242RwDraughtingModelItemAssociation,
    "RwStepAP242RwDraughtingModelItemAssociation"
);
impl RwStepAP242RwDraughtingModelItemAssociation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP242DraughtingModelItemAssociation>,
    ) -> Result<(), Failure> {
        rw_block_b_0016!(
            a_definition,
            a_description,
            a_name,
            a_representation,
            a_type,
            ach,
            an_ent,
            an_items,
            data,
            ent,
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
        ent: &Handle<StepAP242DraughtingModelItemAssociation>,
    ) -> Result<(), Failure> {
        rw_block_b_0101!(ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP242DraughtingModelItemAssociation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0153!(ent, i, iter, nb);
        Ok(())
    }
}

step_rw_class!(
    RwStepAP242RwGeometricItemSpecificUsage,
    "RwStepAP242RwGeometricItemSpecificUsage"
);
impl RwStepAP242RwGeometricItemSpecificUsage {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP242GeometricItemSpecificUsage>,
    ) -> Result<(), Failure> {
        rw_block_b_0016!(
            a_definition,
            a_description,
            a_name,
            a_representation,
            a_type,
            ach,
            an_ent,
            an_items,
            data,
            ent,
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
        ent: &Handle<StepAP242GeometricItemSpecificUsage>,
    ) -> Result<(), Failure> {
        rw_block_b_0101!(ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP242GeometricItemSpecificUsage>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0153!(ent, i, iter, nb);
        Ok(())
    }
}

step_rw_class!(RwStepAP242RwIdAttribute, "RwStepAP242RwIdAttribute");
impl RwStepAP242RwIdAttribute {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP242IdAttribute>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "id_attribute")? {
            return Ok(());
        }
        let mut an_attribute_value: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "attribute_value", ach, &mut an_attribute_value)?;
        let mut an_identified_item: StepAP242IdAttributeSelect = Default::default();
        data.read_entity_select(num, 2, "identified_item", ach, &mut an_identified_item)?;
        ent.init(&an_attribute_value, &an_identified_item);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepAP242IdAttribute>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.attribute_value())?;
        sw.send_entity(&ent.identified_item().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP242IdAttribute>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.identified_item().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepAP242RwItemIdentifiedRepresentationUsage,
    "RwStepAP242RwItemIdentifiedRepresentationUsage"
);
impl RwStepAP242RwItemIdentifiedRepresentationUsage {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepAP242ItemIdentifiedRepresentationUsage>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "item_identified_representation_usage")? {
            return Ok(());
        }
        rw_block_b_0191!(a_description, a_name, ach, data, num);
        if data.is_param_defined(num, 2)? {
            data.read_string(num, 2, "description", ach, &mut a_description)?;
        }
        let mut a_definition: StepAP242ItemIdentifiedRepresentationUsageDefinition =
            Default::default();
        data.read_entity_select(num, 3, "definition", ach, &mut a_definition)?;
        let mut a_representation: Handle<StepReprRepresentation> = Default::default();
        data.read_entity(
            num,
            4,
            "used_representation",
            ach,
            StepReprRepresentation::static_type(),
            &mut a_representation,
        )?;
        rw_block_b_0079!(
            a_definition,
            a_description,
            a_name,
            a_representation,
            a_type,
            ach,
            an_ent,
            an_items,
            data,
            ent,
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
        ent: &Handle<StepAP242ItemIdentifiedRepresentationUsage>,
    ) -> Result<(), Failure> {
        rw_block_b_0101!(ent, i, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepAP242ItemIdentifiedRepresentationUsage>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0153!(ent, i, iter, nb);
        Ok(())
    }
}
