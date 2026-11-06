#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepBasicRwDocumentProductEquivalence,
    "RwStepBasicRwDocumentProductEquivalence"
);
impl RwStepBasicRwDocumentProductEquivalence {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicDocumentProductEquivalence>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "document_product_equivalence")? {
            return Ok(());
        }
        let mut a_document_product_association_name: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            1,
            "document_product_association.name",
            ach,
            &mut a_document_product_association_name,
        )?;
        let mut a_document_product_association_description: Handle<HAsciiString> =
            Default::default();
        let mut has_document_product_association_description: bool = true;
        if data.is_param_defined(num, 2)? {
            data.read_string(
                num,
                2,
                "document_product_association.description",
                ach,
                &mut a_document_product_association_description,
            )?;
        } else {
            has_document_product_association_description = false;
        }
        let mut a_document_product_association_relating_document: Handle<StepBasicDocument> =
            Default::default();
        data.read_entity(
            num,
            3,
            "document_product_association.relating_document",
            ach,
            StepBasicDocument::static_type(),
            &mut a_document_product_association_relating_document,
        )?;
        let mut a_document_product_association_related_product: StepBasicProductOrFormationOrDefinition = Default::default();
        data.read_entity_select(
            num,
            4,
            "document_product_association.related_product",
            ach,
            &mut a_document_product_association_related_product,
        )?;
        ent.init(
            &a_document_product_association_name,
            has_document_product_association_description,
            &a_document_product_association_description,
            &a_document_product_association_relating_document,
            &a_document_product_association_related_product,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicDocumentProductEquivalence>,
    ) -> Result<(), Failure> {
        rw_block_0210!(ent, sw);
        sw.send_entity(&ent.parent.relating_document())?;
        sw.send_entity(&ent.parent.related_product().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicDocumentProductEquivalence>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.relating_document().cast());
        iter.add_item(&ent.parent.related_product().value());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwDocumentRelationship,
    "RwStepBasicRwDocumentRelationship"
);
impl RwStepBasicRwDocumentRelationship {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicDocumentRelationship>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "document_relationship")? {
            return Ok(());
        }
        rw_block_0193!(a_descr, a_name, ach, data, num);
        if data.is_param_defined(num, 2)? {
            data.read_string(num, 2, "description", ach, &mut a_descr)?;
        }
        let mut a_relating: Handle<StepBasicDocument> = Default::default();
        data.read_entity(
            num,
            3,
            "relating_document",
            ach,
            StepBasicDocument::static_type(),
            &mut a_relating,
        )?;
        let mut a_related: Handle<StepBasicDocument> = Default::default();
        data.read_entity(
            num,
            4,
            "related_document",
            ach,
            StepBasicDocument::static_type(),
            &mut a_related,
        )?;
        ent.init(&a_name, &a_descr, &a_relating, &a_related);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicDocumentRelationship>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.description())?;
        sw.send_entity(&ent.relating_document())?;
        sw.send_entity(&ent.related_document())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicDocumentRelationship>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.relating_document().cast());
        iter.add_item(&ent.related_document().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwDocumentRepresentationType,
    "RwStepBasicRwDocumentRepresentationType"
);
impl RwStepBasicRwDocumentRepresentationType {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicDocumentRepresentationType>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "document_representation_type")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_represented_document: Handle<StepBasicDocument> = Default::default();
        data.read_entity(
            num,
            2,
            "represented_document",
            ach,
            StepBasicDocument::static_type(),
            &mut a_represented_document,
        )?;
        ent.init(&a_name, &a_represented_document);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicDocumentRepresentationType>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.represented_document())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicDocumentRepresentationType>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.represented_document().cast());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwDocumentType, "RwStepBasicRwDocumentType");
impl RwStepBasicRwDocumentType {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicDocumentType>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "document_type")? {
            return Ok(());
        }
        let mut a_id: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "product_data_type", ach, &mut a_id)?;
        ent.init(&a_id);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicDocumentType>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.product_data_type())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        _unused_0: &Handle<StepBasicDocumentType>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwDocumentUsageConstraint,
    "RwStepBasicRwDocumentUsageConstraint"
);
impl RwStepBasicRwDocumentUsageConstraint {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicDocumentUsageConstraint>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 3, ach, "document_usage_constraint")? {
            return Ok(());
        }
        let mut a_source: Handle<StepBasicDocument> = Default::default();
        data.read_entity(
            num,
            1,
            "source",
            ach,
            StepBasicDocument::static_type(),
            &mut a_source,
        )?;
        let mut a_label: Handle<HAsciiString> = Default::default();
        data.read_string(num, 2, "subject_element", ach, &mut a_label)?;
        let mut a_description: Handle<HAsciiString> = Default::default();
        data.read_string(num, 3, "subject_element_value", ach, &mut a_description)?;
        ent.init(&a_source, &a_label, &a_description);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicDocumentUsageConstraint>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.source())?;
        sw.send_entity(&ent.subject_element())?;
        sw.send_entity(&ent.subject_element_value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicDocumentUsageConstraint>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.source().cast());
        Ok(())
    }
}
