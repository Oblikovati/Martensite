//! More of IgesDimenToolDimensionDisplayData.
#![allow(
    unused_mut,
    unused_variables,
    unused_assignments,
    clippy::needless_late_init,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::manual_range_contains,
    clippy::too_many_arguments,
    clippy::collapsible_else_if,
    clippy::needless_return,
    reason = "the members keep the order and the locals of the record they read"
)]
use crate::iges_data::{iges_array1_set, iges_live, iges_new_array1};
use crate::iges_dimen::{IgesDimenDimensionDisplayData, IgesDimenToolDimensionDisplayData};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::{InterfaceCheck, InterfaceCopyTool, InterfaceShareTool};

impl IgesDimenToolDimensionDisplayData {
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDimenDimensionDisplayData>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ent.nb_property_values() != 14 {
            ach.add_fail_text("The No. of property values != 14 ", "");
        }
        if (ent.dimension_type() < 0) || (ent.dimension_type() > 2) {
            ach.add_fail_text("Incorrect Dimension Type", "");
        }
        if (ent.label_position() < 0) || (ent.label_position() > 4) {
            ach.add_fail_text("Incorrect Preferred Label Position", "");
        }
        if (((ent.character_set() != 1) && (ent.character_set() != 1001))
            && (ent.character_set() != 1002))
            && (ent.character_set() != 1003)
        {
            ach.add_fail_text("Incorrect Character Set", "");
        }
        if (ent.decimal_symbol() != 0) && (ent.decimal_symbol() != 1) {
            ach.add_fail_text("Incorrect Decimal Symbol", "");
        }
        if (ent.text_alignment() != 0) && (ent.text_alignment() != 1) {
            ach.add_fail_text("Incorrect Text Alignment", "");
        }
        if (ent.text_level() < 0) || (ent.text_level() > 2) {
            ach.add_fail_text("Incorrect Text Level", "");
        }
        if (ent.text_placement() < 0) || (ent.text_placement() > 2) {
            ach.add_fail_text("Incorrect Text Placement", "");
        }
        if (ent.arrow_head_orientation() != 0) && (ent.arrow_head_orientation() != 1) {
            ach.add_fail_text("Incorrect ArrowHead Orientation", "");
        }
        {
            let mut upper: i32 = ent.nb_supplementary_notes()?;
            let mut i: i32 = 1;
            while i <= upper {
                if (ent.supplementary_note(i)? < 1) || (ent.supplementary_note(i)? > 4) {
                    ach.add_fail_text("Incorrect First supplement note", "");
                }
                i += 1;
            }
        }
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDimenDimensionDisplayData>,
        ent: &Handle<IgesDimenDimensionDisplayData>,
        _unused_2: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0056!(end_list, notes_list, start_list);
        let mut upper: i32 = another.nb_supplementary_notes()?;
        if upper > 0 {
            end_list = iges_new_array1::<i32>(1, upper)?;
            start_list = iges_new_array1::<i32>(1, upper)?;
            notes_list = iges_new_array1::<i32>(1, upper)?;
            for i in 1..=upper {
                iges_array1_set(
                    &end_list,
                    i,
                    another.end_index(i)?,
                    "IgesDimenToolDimensionDisplayData::own_copy",
                )?;
                iges_array1_set(
                    &start_list,
                    i,
                    another.start_index(i)?,
                    "IgesDimenToolDimensionDisplayData::own_copy",
                )?;
                iges_array1_set(
                    &notes_list,
                    i,
                    another.supplementary_note(i)?,
                    "IgesDimenToolDimensionDisplayData::own_copy",
                )?;
            }
        }
        let mut temp_nb_property_values: i32 = another.nb_property_values();
        let mut temp_dimension_type: i32 = another.dimension_type();
        let mut temp_label_pos: i32 = another.label_position();
        let mut temp_char_set: i32 = another.character_set();
        let mut temp_ls: Handle<HAsciiString> = Handle::new(HAsciiString::copied(iges_live(
            &another.l_string(),
            "IgesDimenToolDimensionDisplayData::own_copy",
        )?));
        let mut temp_symbol: i32 = another.decimal_symbol();
        let mut temp_angle: f64 = another.witness_line_angle();
        let mut temp_align: i32 = another.text_alignment();
        let mut temp_level: i32 = another.text_level();
        let mut temp_placement: i32 = another.text_placement();
        let mut temp_arrow_head: i32 = another.arrow_head_orientation();
        let mut temp_initial: f64 = another.initial_value();
        ent.init(
            temp_nb_property_values,
            temp_dimension_type,
            temp_label_pos,
            temp_char_set,
            &temp_ls,
            temp_symbol,
            temp_angle,
            temp_align,
            temp_level,
            temp_placement,
            temp_arrow_head,
            temp_initial,
            &notes_list,
            &start_list,
            &end_list,
        )?;
        Ok(())
    }
}
