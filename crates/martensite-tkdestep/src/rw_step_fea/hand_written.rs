//! The FEA model readers and writers: their intended analysis codes are plain strings in an array.
use super::*;
use crate::step_data::{StepDataStepReaderData, StepDataStepWriter};
use crate::step_entity::*;
use crate::step_fea::*;
use crate::step_repr::{StepReprRepresentationContext, StepReprRepresentationItem};
use crate::step_rw::*;

/// The fields a model record holds, in file order.
#[derive(Default)]
struct FeaModelFields {
    name: Handle<HAsciiString>,
    items: Handle<HArray1<Handle<StepReprRepresentationItem>>>,
    context: Handle<StepReprRepresentationContext>,
    creating_software: Handle<HAsciiString>,
    intended_analysis_code: Handle<HArray1<AsciiString>>,
    description: Handle<HAsciiString>,
    analysis_type: Handle<HAsciiString>,
}

/// The parameter messages of a model record: the two classes differ only in these.
struct FeaModelMessages {
    record: &'static str,
    name: &'static str,
    items: &'static str,
    context: &'static str,
    creating_software: &'static str,
    intended_analysis_code: &'static str,
    description: &'static str,
    analysis_type: &'static str,
}

/// Reads a model record; `None` when its parameter count is wrong.
fn read_fea_model(
    data: &StepDataStepReaderData,
    num: i32,
    ach: &Handle<InterfaceCheck>,
    messages: &FeaModelMessages,
    where_called: &str,
) -> Result<Option<FeaModelFields>, Failure> {
    if !data.check_nb_params(num, 7, ach, messages.record)? {
        return Ok(None);
    }
    let mut fields = FeaModelFields::default();
    data.read_string(num, 1, messages.name, ach, &mut fields.name)?;
    let mut sub2 = 0;
    if data.read_sub_list(num, 2, messages.items, ach, &mut sub2, false, 0, 0)? {
        let count = data.nb_params(sub2)?;
        fields.items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, count)?;
        for rank in 1..=count {
            let mut item = Handle::<StepReprRepresentationItem>::default();
            data.read_entity(
                sub2,
                rank,
                "representation_item",
                ach,
                StepReprRepresentationItem::static_type(),
                &mut item,
            )?;
            step_array1_set(&fields.items, rank, item, where_called)?;
        }
    }
    data.read_entity(
        num,
        3,
        messages.context,
        ach,
        StepReprRepresentationContext::static_type(),
        &mut fields.context,
    )?;
    data.read_string(
        num,
        4,
        messages.creating_software,
        ach,
        &mut fields.creating_software,
    )?;
    let mut sub5 = 0;
    if data.read_sub_list(
        num,
        5,
        messages.intended_analysis_code,
        ach,
        &mut sub5,
        false,
        0,
        0,
    )? {
        let count = data.nb_params(sub5)?;
        fields.intended_analysis_code = step_new_array1::<AsciiString>(1, count)?;
        for rank in 1..=count {
            let mut code = Handle::<HAsciiString>::default();
            data.read_string(sub5, rank, "h_ascii_string", ach, &mut code)?;
            let text = step_live(&code, where_called)?.string().clone();
            step_array1_set(&fields.intended_analysis_code, rank, text, where_called)?;
        }
    }
    data.read_string(num, 6, messages.description, ach, &mut fields.description)?;
    data.read_string(
        num,
        7,
        messages.analysis_type,
        ach,
        &mut fields.analysis_type,
    )?;
    Ok(Some(fields))
}

/// Writes a model record through the embedded `StepFEAFeaModel` of either class.
fn write_fea_model(
    sw: &mut StepDataStepWriter,
    model: &StepFEAFeaModel,
    where_called: &str,
) -> Result<(), Failure> {
    sw.send_entity(&model.name())?;
    sw.open_sub()?;
    let items = model.items();
    let live = step_live(&items, where_called)?;
    for rank in 1..=live.array1().length() {
        sw.send_entity(&step_array1_item(&items, rank, where_called)?)?;
    }
    sw.close_sub()?;
    sw.send_entity(&model.context_of_items())?;
    sw.send_entity(&model.creating_software())?;
    sw.open_sub()?;
    let codes = model.intended_analysis_code();
    let live = step_live(&codes, where_called)?;
    for rank in 1..=live.array1().length() {
        let code = step_array1_item(&codes, rank, where_called)?;
        sw.send_entity(&Handle::new(HAsciiString::from_string(&code)))?;
    }
    sw.close_sub()?;
    sw.send_entity(&model.description())?;
    sw.send_entity(&model.analysis_type())
}

impl RwStepFEARwFeaModel {
    /// `read_step`. A code that cannot be read is the null dereference.
    ///
    /// # Errors
    /// a null object failure, or an out of range failure for a record outside the file.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaModel>,
    ) -> Result<(), Failure> {
        const WHERE: &str = "RwStepFEARwFeaModel::ReadStep";
        let messages = FeaModelMessages {
            record: "fea_model",
            name: "representation.name",
            items: "representation.items",
            context: "representation.context_of_items",
            creating_software: "creating_software",
            intended_analysis_code: "intended_analysis_code",
            description: "description",
            analysis_type: "analysis_type",
        };
        let Some(f) = read_fea_model(data, num, ach, &messages, WHERE)? else {
            return Ok(());
        };
        step_live(ent, WHERE)?.init(
            &f.name,
            &f.items,
            &f.context,
            &f.creating_software,
            &f.intended_analysis_code,
            &f.description,
            &f.analysis_type,
        );
        Ok(())
    }
    /// `WriteStep`.
    ///
    /// # Errors
    /// a null object failure for a null entity or array.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAFeaModel>,
    ) -> Result<(), Failure> {
        const WHERE: &str = "RwStepFEARwFeaModel::WriteStep";
        write_fea_model(sw, step_live(ent, WHERE)?, WHERE)
    }
}

impl RwStepFEARwFeaModel3d {
    /// `read_step`. A code that cannot be read is the null dereference.
    ///
    /// # Errors
    /// a null object failure, or an out of range failure for a record outside the file.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepFEAFeaModel3d>,
    ) -> Result<(), Failure> {
        const WHERE: &str = "RwStepFEARwFeaModel3d::ReadStep";
        let messages = FeaModelMessages {
            record: "fea_model3d",
            name: "representation.name",
            items: "representation.items",
            context: "representation.context_of_items",
            creating_software: "fea_model.creating_software",
            intended_analysis_code: "fea_model.intended_analysis_code",
            description: "fea_model.description",
            analysis_type: "fea_model.analysis_type",
        };
        let Some(f) = read_fea_model(data, num, ach, &messages, WHERE)? else {
            return Ok(());
        };
        step_live(ent, WHERE)?.init(
            &f.name,
            &f.items,
            &f.context,
            &f.creating_software,
            &f.intended_analysis_code,
            &f.description,
            &f.analysis_type,
        );
        Ok(())
    }
    /// `WriteStep`.
    ///
    /// # Errors
    /// a null object failure for a null entity or array.
    pub fn write_step(
        &self,
        sw: &mut StepDataStepWriter,
        ent: &Handle<StepFEAFeaModel3d>,
    ) -> Result<(), Failure> {
        const WHERE: &str = "RwStepFEARwFeaModel3d::WriteStep";
        write_fea_model(sw, step_live(ent, WHERE)?, WHERE)
    }
}
