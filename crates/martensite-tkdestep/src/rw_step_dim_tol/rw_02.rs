#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(RwStepDimTolRwDatum, "RwStepDimTolRwDatum");
impl RwStepDimTolRwDatum {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolDatum>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 5, ach, "datum")? {
            return Ok(());
        }
        rw_block_b_0058!(
            a_shape_aspect_description,
            a_shape_aspect_name,
            a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
            ach,
            data,
            num
        );
        let mut a_identification: Handle<HAsciiString> = Default::default();
        data.read_string(num, 5, "identification", ach, &mut a_identification)?;
        ent.init(
            &a_shape_aspect_name,
            &a_shape_aspect_description,
            &a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
            &a_identification,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolDatum>,
    ) -> Result<(), Failure> {
        rw_block_b_0158!(ent, sw);
        sw.send_entity(&ent.identification())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolDatum>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(RwStepDimTolRwDatumFeature, "RwStepDimTolRwDatumFeature");
impl RwStepDimTolRwDatumFeature {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolDatumFeature>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "datum_feature")? {
            return Ok(());
        }
        rw_block_b_0058!(
            a_shape_aspect_description,
            a_shape_aspect_name,
            a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
            ach,
            data,
            num
        );
        ent.init(
            &a_shape_aspect_name,
            &a_shape_aspect_description,
            &a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolDatumFeature>,
    ) -> Result<(), Failure> {
        rw_block_b_0158!(ent, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolDatumFeature>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.parent.of_shape().cast());
        Ok(())
    }
}

step_rw_class!(RwStepDimTolRwDatumReference, "RwStepDimTolRwDatumReference");
impl RwStepDimTolRwDatumReference {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolDatumReference>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "datum_reference")? {
            return Ok(());
        }
        let mut a_precedence: i32 = Default::default();
        data.read_integer(num, 1, "precedence", ach, &mut a_precedence)?;
        let mut a_referenced_datum: Handle<StepDimTolDatum> = Default::default();
        data.read_entity(
            num,
            2,
            "referenced_datum",
            ach,
            StepDimTolDatum::static_type(),
            &mut a_referenced_datum,
        )?;
        ent.init(a_precedence, &a_referenced_datum);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepDimTolDatumReference>,
    ) -> Result<(), Failure> {
        sw.send_integer(ent.precedence())?;
        sw.send_entity(&ent.referenced_datum())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolDatumReference>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.add_item(&ent.referenced_datum().cast());
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwDatumReferenceCompartment,
    "RwStepDimTolRwDatumReferenceCompartment"
);
impl RwStepDimTolRwDatumReferenceCompartment {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolDatumReferenceCompartment>,
    ) -> Result<(), Failure> {
        rw_block_b_0010!(
            a_base,
            a_datum,
            a_drmwv,
            a_member,
            a_modifiers,
            a_new_nb_sub,
            a_sdrm,
            a_shape_aspect_description,
            a_shape_aspect_name,
            a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
            a_type,
            ach,
            an_ent,
            an_enum_text,
            an_items,
            data,
            ent,
            has_modifiers,
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
        ent: &Handle<StepDimTolDatumReferenceCompartment>,
    ) -> Result<(), Failure> {
        rw_block_b_0080!(a_base_type, an_array, ent, i, nb, sw);
        if ent.has_modifiers()? {
            let i: i32 = Default::default();
            let nb: i32 = ent.nb_modifiers();
            sw.open_sub()?;
            for i in 1..=nb {
                let a_modifier: StepDimTolDatumReferenceModifier = ent.modifiers_value(i)?;
                let a_type: i32 = a_modifier.case_num(&a_modifier.value());
                if a_type == 1 {
                    sw.send_entity(&a_modifier.datum_reference_modifier_with_value())?;
                } else {
                    if a_type == 2 {
                        sw.send_entity(&a_modifier.simple_datum_reference_modifier_member())?;
                    }
                }
            }
            sw.close_sub()?;
        } else {
            sw.send_undef()?;
        }
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolDatumReferenceCompartment>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0167!(a_base_type, an_array, ent, i, iter, nb);
        Ok(())
    }
}

step_rw_class!(
    RwStepDimTolRwDatumReferenceElement,
    "RwStepDimTolRwDatumReferenceElement"
);
impl RwStepDimTolRwDatumReferenceElement {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepDimTolDatumReferenceElement>,
    ) -> Result<(), Failure> {
        rw_block_b_0010!(
            a_base,
            a_datum,
            a_drmwv,
            a_member,
            a_modifiers,
            a_new_nb_sub,
            a_sdrm,
            a_shape_aspect_description,
            a_shape_aspect_name,
            a_shape_aspect_of_shape,
            a_shape_aspect_product_definitional,
            a_type,
            ach,
            an_ent,
            an_enum_text,
            an_items,
            data,
            ent,
            has_modifiers,
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
        ent: &Handle<StepDimTolDatumReferenceElement>,
    ) -> Result<(), Failure> {
        rw_block_b_0063!(a_base_type, a_modifier, a_type, an_array, ent, i, nb, sw);
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepDimTolDatumReferenceElement>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        rw_block_b_0167!(a_base_type, an_array, ent, i, iter, nb);
        Ok(())
    }
}
