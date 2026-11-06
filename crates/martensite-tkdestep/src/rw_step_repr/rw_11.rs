#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepReprRwRepresentation, "RwStepReprRwRepresentation");
impl RwStepReprRwRepresentation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_0038!(
            a_context_of_items,
            a_items,
            a_name,
            ach,
            anent2,
            data,
            i2,
            nb2,
            nsub2,
            num
        );
        ent.init(&a_name, &a_items, &a_context_of_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprRepresentation>,
    ) -> Result<(), Failure> {
        rw_block_0116!(ent, i2, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprRepresentation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_0190!(ent, is1, iter, nb_elem1);
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwRepresentationContext,
    "RwStepReprRwRepresentationContext"
);
impl RwStepReprRwRepresentationContext {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprRepresentationContext>,
    ) -> Result<(), Failure> {
        rw_block_0104!(a_context_identifier, a_context_type, ach, data, num);
        ent.init(&a_context_identifier, &a_context_type);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprRepresentationContext>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.context_identifier())?;
        sw.send_entity(&ent.context_type())?;
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwRepresentationContextReference,
    "RwStepReprRwRepresentationContextReference"
);
impl RwStepReprRwRepresentationContextReference {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_ach: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepReprRepresentationContextReference>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 1, the_ach, "representation_context_reference")? {
            return Ok(());
        }
        let mut a_context_identifier: Handle<HAsciiString> = Default::default();
        the_data.read_string(
            the_num,
            1,
            "context_identifier",
            the_ach,
            &mut a_context_identifier,
        )?;
        the_ent.init(&a_context_identifier);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepReprRepresentationContextReference>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.context_identifier())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepReprRepresentationContextReference>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwRepresentationItem,
    "RwStepReprRwRepresentationItem"
);
impl RwStepReprRwRepresentationItem {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprRepresentationItem>,
    ) -> Result<(), Failure> {
        rw_block_0176!(a_name, ach, data, num);
        ent.init(&a_name);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprRepresentationItem>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwRepresentationMap,
    "RwStepReprRwRepresentationMap"
);
impl RwStepReprRwRepresentationMap {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprRepresentationMap>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "representation_map")? {
            return Ok(());
        }
        let mut a_mapping_origin: Handle<StepReprRepresentationItem> = Default::default();
        data.read_entity(
            num,
            1,
            "mapping_origin",
            ach,
            StepReprRepresentationItem::static_type(),
            &mut a_mapping_origin,
        )?;
        let mut a_mapped_representation: Handle<StepReprRepresentation> = Default::default();
        data.read_entity(
            num,
            2,
            "mapped_representation",
            ach,
            StepReprRepresentation::static_type(),
            &mut a_mapped_representation,
        )?;
        ent.init(&a_mapping_origin, &a_mapped_representation);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprRepresentationMap>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.mapping_origin())?;
        sw.send_entity(&ent.mapped_representation())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprRepresentationMap>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.mapping_origin().cast());
        iter.get_one_item(&ent.mapped_representation().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwRepresentationReference,
    "RwStepReprRwRepresentationReference"
);
impl RwStepReprRwRepresentationReference {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_ach: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepReprRepresentationReference>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 2, the_ach, "representation_reference")? {
            return Ok(());
        }
        let mut a_id: Handle<HAsciiString> = Default::default();
        the_data.read_string(the_num, 1, "id", the_ach, &mut a_id)?;
        let mut a_context_of_items: Handle<StepReprRepresentationContextReference> =
            Default::default();
        the_data.read_entity(
            the_num,
            2,
            "context_of_items",
            the_ach,
            StepReprRepresentationContextReference::static_type(),
            &mut a_context_of_items,
        )?;
        the_ent.init(&a_id, &a_context_of_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepReprRepresentationReference>,
    ) -> Result<(), Failure> {
        sw.send_entity(&the_ent.id())?;
        sw.send_entity(&the_ent.context_of_items())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepReprRepresentationReference>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&the_ent.context_of_items().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepReprRwRepresentationRelationship,
    "RwStepReprRwRepresentationRelationship"
);
impl RwStepReprRwRepresentationRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprRepresentationRelationship>,
    ) -> Result<(), Failure> {
        rw_block_0042!(a_description, a_name, a_rep1, a_rep2, ach, data, num);
        ent.init(&a_name, &a_description, &a_rep1, &a_rep2);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepReprRepresentationRelationship>,
    ) -> Result<(), Failure> {
        rw_block_0165!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepReprRepresentationRelationship>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.rep1().cast());
        iter.get_one_item(&ent.rep2().cast());
        Ok(())
    }
}
