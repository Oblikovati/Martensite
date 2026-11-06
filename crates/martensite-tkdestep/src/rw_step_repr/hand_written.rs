//! A reader that accepts a missing description, so it counts its parameters itself.
use super::*;
use crate::step_data::StepDataStepReaderData;
use crate::step_entity::*;
use crate::step_repr::StepReprDescriptiveRepresentationItem;
use crate::step_rw::*;

impl RwStepReprRwDescriptiveRepresentationItem {
    /// `read_step`: 1 or 2 parameters; a missing description goes through the reader's own message.
    ///
    /// # Errors
    /// an out of range failure for a record outside the file.
    pub fn read_step(
        &self,
        data: &Handle<StepDataStepReaderData>,
        num: i32,
        ach: &Handle<InterfaceCheck>,
        ent: &Handle<StepReprDescriptiveRepresentationItem>,
    ) -> Result<(), Failure> {
        let count = data.nb_params(num)?;
        if count != 2 && count != 1 {
            ach.add_fail_text(
                "Count of Parameters is not 1 or 2 for descriptive_representation_item",
                "Count of Parameters is not %d or %d for %s",
            );
            return Ok(());
        }
        let mut a_name = Handle::<HAsciiString>::default();
        data.read_string(num, 1, "name", ach, &mut a_name)?;
        let mut a_description = Handle::<HAsciiString>::default();
        data.read_string(num, 2, "description", ach, &mut a_description)?;
        ent.init(&a_name, &a_description);
        Ok(())
    }
}
