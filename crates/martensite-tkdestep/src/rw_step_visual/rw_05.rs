#![allow(
    unused_variables,
    unused_assignments,
    clippy::assign_op_pattern,
    reason = "the readers keep values they then ignore"
)]
use crate::step_rw::*;

step_rw_class!(
    RwStepVisualRwCharacterizedObjAndRepresentationAndDraughtingModel,
    "RwStepVisualRwCharacterizedObjAndRepresentationAndDraughtingModel"
);
impl RwStepVisualRwCharacterizedObjAndRepresentationAndDraughtingModel {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num0: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualCharacterizedObjAndRepresentationAndDraughtingModel>,
    ) -> Result<(), Failure> {
        let mut num: i32 = 0;
        data.named_for_complex_short("REPRESENTATION", "RPRSNT", num0, &mut num, ach);
        if !data.check_nb_params(num, 3, ach, "shape_aspect")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut an_items: Handle<HArray1<Handle<StepReprRepresentationItem>>> = Default::default();
        let mut an_item: Handle<StepReprRepresentationItem> = Default::default();
        let mut nsub: i32 = Default::default();
        if data.read_sub_list(num, 2, "items", ach, &mut nsub, false, 0, 0)? {
            let nb: i32 = data.nb_params(nsub)?;
            an_items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, nb)?;
            for i in 1..=nb {
                if data.read_entity(
                    nsub,
                    i,
                    "representation_item",
                    ach,
                    StepReprRepresentationItem::static_type(),
                    &mut an_item,
                )? {
                    step_array1_set(&an_items, i, an_item.clone(), "ReadStep")?;
                }
            }
        }
        let mut a_context_of_items: Handle<StepReprRepresentationContext> = Default::default();
        data.read_entity(
            num,
            3,
            "context_of_items",
            ach,
            StepReprRepresentationContext::static_type(),
            &mut a_context_of_items,
        )?;
        ent.init(&a_name, &an_items, &a_context_of_items);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualCharacterizedObjAndRepresentationAndDraughtingModel>,
    ) -> Result<(), Failure> {
        step_start_entity(sw, "CHARACTERIZED_OBJECT")?;
        sw.send_derived()?;
        sw.send_derived()?;
        step_start_entity(sw, "CHARACTERIZED_REPRESENTATION")?;
        step_start_entity(sw, "DRAUGHTING_MODEL")?;
        step_start_entity(sw, "REPRESENTATION")?;
        sw.send_entity(&ent.name())?;
        sw.open_sub()?;
        for i in 1..=ent.nb_items() {
            sw.send_entity(&ent.items_value(i)?)?;
        }
        sw.close_sub()?;
        sw.send_entity(&ent.context_of_items())?;
        Ok(())
    }
    /// `share`.
    pub fn share(
        &self,
        ent: &Handle<StepVisualCharacterizedObjAndRepresentationAndDraughtingModel>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let nb_elem: i32 = ent.nb_items();
        for i in 1..=nb_elem {
            iter.get_one_item(&ent.items_value(i)?.cast());
        }
        iter.get_one_item(&ent.context_of_items().cast());
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwColour, "RwStepVisualRwColour");
impl RwStepVisualRwColour {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        _unused_3: &Handle<StepVisualColour>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 0, ach, "colour")? {
            return Ok(());
        }
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        _unused_0: &mut StepDataStepWriter,
        _unused_1: &Handle<StepVisualColour>,
    ) -> Result<(), Failure> {
        Ok(())
    }
}

step_rw_class!(RwStepVisualRwColourRgb, "RwStepVisualRwColourRgb");
impl RwStepVisualRwColourRgb {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualColourRgb>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 4, ach, "colour_rgb")? {
            return Ok(());
        }
        let mut a_name: Handle<HAsciiString> = Default::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_red: f64 = Default::default();
        data.read_real(num, 2, "red", ach, &mut a_red)?;
        let mut a_green: f64 = Default::default();
        data.read_real(num, 3, "green", ach, &mut a_green)?;
        let mut a_blue: f64 = Default::default();
        data.read_real(num, 4, "blue", ach, &mut a_blue)?;
        ent.init(&a_name, a_red, a_green, a_blue);
        Ok(())
    }
    /// `WriteStep`.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepVisualColourRgb>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        sw.send_real(ent.red())?;
        sw.send_real(ent.green())?;
        sw.send_real(ent.blue())?;
        Ok(())
    }
}

step_rw_class!(
    RwStepVisualRwColourSpecification,
    "RwStepVisualRwColourSpecification"
);
impl RwStepVisualRwColourSpecification {
    /// `read_step`.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepVisualColourSpecification>,
    ) -> Result<(), Failure> {
        if !data.check_nb_params(num, 1, ach, "colour_specification")? {
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
        ent: &Handle<StepVisualColourSpecification>,
    ) -> Result<(), Failure> {
        sw.send_entity(&ent.name())?;
        Ok(())
    }
}
