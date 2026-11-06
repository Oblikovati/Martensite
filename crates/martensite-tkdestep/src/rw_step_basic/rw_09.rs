#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepBasicRwDimensionalExponents,
    "RwStepBasicRwDimensionalExponents"
);
impl RwStepBasicRwDimensionalExponents {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicDimensionalExponents>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 7, ach, "dimensional_exponents")? {
            return Ok(());
        }
        let mut a_length_exponent: f64 = Default::default();
        data.read_real(num, 1, "length_exponent", ach, &mut a_length_exponent)?;
        let mut a_mass_exponent: f64 = Default::default();
        data.read_real(num, 2, "mass_exponent", ach, &mut a_mass_exponent)?;
        let mut a_time_exponent: f64 = Default::default();
        data.read_real(num, 3, "time_exponent", ach, &mut a_time_exponent)?;
        let mut a_electric_current_exponent: f64 = Default::default();
        data.read_real(
            num,
            4,
            "electric_current_exponent",
            ach,
            &mut a_electric_current_exponent,
        )?;
        let mut a_thermodynamic_temperature_exponent: f64 = Default::default();
        data.read_real(
            num,
            5,
            "thermodynamic_temperature_exponent",
            ach,
            &mut a_thermodynamic_temperature_exponent,
        )?;
        let mut a_amount_of_substance_exponent: f64 = Default::default();
        data.read_real(
            num,
            6,
            "amount_of_substance_exponent",
            ach,
            &mut a_amount_of_substance_exponent,
        )?;
        let mut a_luminous_intensity_exponent: f64 = Default::default();
        data.read_real(
            num,
            7,
            "luminous_intensity_exponent",
            ach,
            &mut a_luminous_intensity_exponent,
        )?;
        ent.init(
            a_length_exponent,
            a_mass_exponent,
            a_time_exponent,
            a_electric_current_exponent,
            a_thermodynamic_temperature_exponent,
            a_amount_of_substance_exponent,
            a_luminous_intensity_exponent,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicDimensionalExponents>,
    ) -> Result<(), Failure> {
        sw.send_real(ent.length_exponent())?;
        sw.send_real(ent.mass_exponent())?;
        sw.send_real(ent.time_exponent())?;
        sw.send_real(ent.electric_current_exponent())?;
        sw.send_real(ent.thermodynamic_temperature_exponent())?;
        sw.send_real(ent.amount_of_substance_exponent())?;
        sw.send_real(ent.luminous_intensity_exponent())?;
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwDocument, "RwStepBasicRwDocument");
impl RwStepBasicRwDocument {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicDocument>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "document")? {
            return Ok(());
        }
        rw_block_0068!(a_description, a_id, a_name, ach, data, has_description, num);
        let mut a_kind: Handle<StepBasicDocumentType> = Default::default();
        data.read_entity(
            num,
            4,
            "kind",
            ach,
            StepBasicDocumentType::static_type(),
            &mut a_kind,
        )?;
        ent.init(&a_id, &a_name, has_description, &a_description, &a_kind);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicDocument>,
    ) -> Result<(), Failure> {
        rw_block_0178!(ent, sw);
        sw.send_entity(&ent.kind())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicDocument>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.kind().cast());
        Ok(())
    }
}

step_rw_class!(RwStepBasicRwDocumentFile, "RwStepBasicRwDocumentFile");
impl RwStepBasicRwDocumentFile {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicDocumentFile>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 6, ach, "document_file")? {
            return Ok(());
        }
        let mut a_document_id: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "document.id", ach, &mut a_document_id)?;
        let mut a_document_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 2, "document.name", ach, &mut a_document_name)?;
        let mut a_document_description: Handle<HAsciiString> = Default::default();
        let mut has_document_description: bool = true;
        if data.is_param_defined(num, 3)? {
            data.read_string(
                num,
                3,
                "document.description",
                ach,
                &mut a_document_description,
            )?;
        } else {
            has_document_description = false;
        }
        let mut a_document_kind: Handle<StepBasicDocumentType> = Default::default();
        data.read_entity(
            num,
            4,
            "document.kind",
            ach,
            StepBasicDocumentType::static_type(),
            &mut a_document_kind,
        )?;
        let mut a_characterized_object_name: Handle<HAsciiString> = Default::default();
        data.read_string(
            num,
            5,
            "characterized_object.name",
            ach,
            &mut a_characterized_object_name,
        )?;
        let mut a_characterized_object_description: Handle<HAsciiString> = Default::default();
        let mut has_characterized_object_description: bool = true;
        if data.is_param_defined(num, 6)? {
            data.read_string(
                num,
                6,
                "characterized_object.description",
                ach,
                &mut a_characterized_object_description,
            )?;
        } else {
            has_characterized_object_description = false;
        }
        ent.init(
            &a_document_id,
            &a_document_name,
            has_document_description,
            &a_document_description,
            &a_document_kind,
            &a_characterized_object_name,
            has_characterized_object_description,
            &a_characterized_object_description,
        )?;
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicDocumentFile>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.parent.id())?;
        rw_block_0210!(ent, sw);
        sw.send_entity(&ent.parent.kind())?;
        sw.send_entity(&step_live(&ent.characterized_object(), "WriteStep")?.name())?;
        if step_live(&ent.characterized_object(), "WriteStep")?.has_description() {
            sw.send_entity(&step_live(&ent.characterized_object(), "WriteStep")?.description())?;
        } else {
            sw.send_undef()?;
        }
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicDocumentFile>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.kind().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepBasicRwDocumentProductAssociation,
    "RwStepBasicRwDocumentProductAssociation"
);
impl RwStepBasicRwDocumentProductAssociation {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepBasicDocumentProductAssociation>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "document_product_association")? {
            return Ok(());
        }
        rw_block_0102!(a_description, a_name, ach, data, has_description, num);
        let mut a_relating_document: Handle<StepBasicDocument> = Default::default();
        data.read_entity(
            num,
            3,
            "relating_document",
            ach,
            StepBasicDocument::static_type(),
            &mut a_relating_document,
        )?;
        let mut a_related_product: StepBasicProductOrFormationOrDefinition = Default::default();
        data.read_entity_select(num, 4, "related_product", ach, &mut a_related_product)?;
        ent.init(
            &a_name,
            has_description,
            &a_description,
            &a_relating_document,
            &a_related_product,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepBasicDocumentProductAssociation>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        if ent.has_description() {
            sw.send_entity(&ent.description())?;
        } else {
            sw.send_undef()?;
        }
        sw.send_entity(&ent.relating_document())?;
        sw.send_entity(&ent.related_product().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepBasicDocumentProductAssociation>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.relating_document().cast());
        iter.add_item(&ent.related_product().value());
        Ok(())
    }
}
