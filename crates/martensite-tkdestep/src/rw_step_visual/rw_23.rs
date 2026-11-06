#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwTessellatedWire,
    "RwStepVisualRwTessellatedWire"
);
impl RwStepVisualRwTessellatedWire {
    /// `read_step`.
    pub fn read_step(
        &self,
        the_data: &Handle<StepDataStepReaderData>,
        the_num: i32,
        the_check: &Handle<InterfaceCheck>,
        the_ent: &Handle<StepVisualTessellatedWire>,
    ) -> Result<(), Failure> {
        if !the_data.check_nb_params(the_num, 3, the_check, "tessellated_wire")? {
            return Ok(());
        }
        let mut a_representation_item_name: Handle<HAsciiString> = Default::default();
        the_data.read_string(
            the_num,
            1,
            "representation_item.name",
            the_check,
            &mut a_representation_item_name,
        )?;
        let mut a_items: Handle<HArray1<StepVisualTessellatedEdgeOrVertex>> = Default::default();
        let mut sub2: i32 = 0;
        if the_data.read_sub_list(the_num, 2, "items", the_check, &mut sub2, false, 0, 0)? {
            let nb0: i32 = the_data.nb_params(sub2)?;
            a_items = step_new_array1::<StepVisualTessellatedEdgeOrVertex>(1, nb0)?;
            let num2: i32 = sub2;
            for i0 in 1..=nb0 {
                let mut an_it0: StepVisualTessellatedEdgeOrVertex = Default::default();
                the_data.read_entity_select(
                    num2,
                    i0,
                    "tessellated_edge_or_vertex",
                    the_check,
                    &mut an_it0,
                )?;
                step_array1_set(&a_items, i0, an_it0.clone(), "ReadStep")?;
            }
        }
        let mut a_geometric_model_link: StepVisualPathOrCompositeCurve = Default::default();
        let mut has_geometric_model_link: bool = true;
        if the_data.is_param_defined(the_num, 3)? {
            the_data.read_entity_select(
                the_num,
                3,
                "geometric_model_link",
                the_check,
                &mut a_geometric_model_link,
            )?;
        } else {
            has_geometric_model_link = false;
            a_geometric_model_link = StepVisualPathOrCompositeCurve::default();
        }
        the_ent.init(
            &a_representation_item_name,
            &a_items,
            has_geometric_model_link,
            &a_geometric_model_link,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        the_sw: &mut StepDataStepWriter,
        the_ent: &Handle<StepVisualTessellatedWire>,
    ) -> Result<(), Failure> {
        the_sw.send_entity(&the_ent.name())?;
        the_sw.open_sub()?;
        for i1 in 1..=step_live(&the_ent.items(), "WriteStep")?.array1().length() {
            let var0: StepVisualTessellatedEdgeOrVertex =
                step_array1_item(&the_ent.items(), i1, "WriteStep")?;
            the_sw.send_entity(&var0.value())?;
        }
        the_sw.close_sub()?;
        if the_ent.has_geometric_model_link() {
            the_sw.send_entity(&the_ent.geometric_model_link().value())?;
        } else {
            the_sw.send_undef()?;
        }
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        the_ent: &Handle<StepVisualTessellatedWire>,
        the_iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        for i1 in 1..=step_live(&the_ent.items(), "Share")?.array1().length() {
            let var0: StepVisualTessellatedEdgeOrVertex =
                step_array1_item(&the_ent.items(), i1, "Share")?;
            the_iter.add_item(&var0.value());
        }
        if the_ent.has_geometric_model_link() {
            the_iter.add_item(&the_ent.geometric_model_link().value());
        }
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwTextLiteral, "RwStepVisualRwTextLiteral");
impl RwStepVisualRwTextLiteral {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualTextLiteral>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 6, ach, "text_literal has not 6 parameter(s)")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_literal: Handle<HAsciiString> = Default::default();
        data.read_string(num, 2, "literal", ach, &mut a_literal)?;
        let mut a_placement: StepGeomAxis2Placement = Default::default();
        data.read_entity_select(num, 3, "placement", ach, &mut a_placement)?;
        let mut a_alignment: Handle<HAsciiString> = Default::default();
        data.read_string(num, 4, "alignment", ach, &mut a_alignment)?;
        let mut a_path: StepVisualTextPath = StepVisualTextPath::TpUp;
        if data.param_type(num, 5)? == InterfaceParamType::ParamEnum {
            let text: Vec<u8> = data.param_c_value(num, 5)?;
            if !RwStepVisualRwTextPath::convert_to_enum(&text, &mut a_path) {
                ach.add_fail_text("Enumeration text_path has not an allowed value", "");
            }
        } else {
            ach.add_fail_text("Parameter #5 (path) is not an enumeration", "");
        }
        let mut a_font: StepVisualFontSelect = Default::default();
        data.read_entity_select(num, 6, "font", ach, &mut a_font)?;
        ent.init(
            &a_name,
            &a_literal,
            &a_placement,
            &a_alignment,
            a_path,
            &a_font,
        );
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualTextLiteral>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.literal())?;
        sw.send_entity(&ent.placement().value())?;
        sw.send_entity(&ent.alignment())?;
        sw.send_enum(RwStepVisualRwTextPath::convert_to_string(ent.path()))?;
        sw.send_entity(&ent.font().value())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualTextLiteral>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.placement().value());
        iter.get_one_item(&ent.font().value());
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwTextStyle, "RwStepVisualRwTextStyle");
impl RwStepVisualRwTextStyle {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualTextStyle>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 2, ach, "text_style has not 2 parameter(s)")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_character_appearance: Handle<StepVisualTextStyleForDefinedFont> =
            Default::default();
        data.read_entity(
            num,
            2,
            "character_appearance",
            ach,
            StepVisualTextStyleForDefinedFont::static_type(),
            &mut a_character_appearance,
        )?;
        ent.init(&a_name, &a_character_appearance);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualTextStyle>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_entity(&ent.character_appearance())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualTextStyle>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.character_appearance().cast());
        Ok(())
    }
}
