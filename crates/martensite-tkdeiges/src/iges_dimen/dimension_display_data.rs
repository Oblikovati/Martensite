//! IgesDimenDimensionDisplayData.
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
use crate::iges_basic::entity_shape::{iges_array1_length, iges_array1_lower};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item};
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkernel::tcollection::HAsciiString;

iges_entity! {
/// Defines IGES Dimension Display Data, Type `406` Form `30`,
/// The Dimensional Display Data Property is optional but when
/// present must be referenced by a dimension entity.
/// The information it contains could be extracted from the text,
/// leader and witness line data with difficulty.
IgesDimenDimensionDisplayData, IgesDimenDimensionDisplayDataState, "IgesDimenDimensionDisplayData", parent IgesDataIgesEntity, {
the_nb_property_values: i32,
the_dimension_type: i32,
the_label_position: i32,
the_character_set: i32,
the_l_string: Handle<HAsciiString>,
the_decimal_symbol: i32,
the_witness_line_angle: f64,
the_text_alignment: i32,
the_text_level: i32,
the_text_placement: i32,
the_arrow_head_orientation: i32,
the_initial_value: f64,
the_supplementary_notes: Handle<HArray1<i32>>,
the_start_index: Handle<HArray1<i32>>,
the_end_index: Handle<HArray1<i32>>,
}
}

impl IgesDimenDimensionDisplayData {
    iges_methods! {
    get nb_property_values = the_nb_property_values: i32;
    get dimension_type = the_dimension_type: i32;
    get label_position = the_label_position: i32;
    get character_set = the_character_set: i32;
    get l_string = the_l_string: Handle<HAsciiString>;
    get decimal_symbol = the_decimal_symbol: i32;
    get witness_line_angle = the_witness_line_angle: f64;
    get text_alignment = the_text_alignment: i32;
    get text_level = the_text_level: i32;
    get text_placement = the_text_placement: i32;
    get arrow_head_orientation = the_arrow_head_orientation: i32;
    get initial_value = the_initial_value: f64;
    }
    /// `init`.
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn init(
        &self,
        num_props: i32,
        a_dim_type: i32,
        a_label_pos: i32,
        a_char_set: i32,
        a_string: &Handle<HAsciiString>,
        a_symbol: i32,
        an_ang: f64,
        an_align: i32,
        a_level: i32,
        a_place: i32,
        an_orient: i32,
        init_val: f64,
        notes: &Handle<HArray1<i32>>,
        start_ind: &Handle<HArray1<i32>>,
        end_ind: &Handle<HArray1<i32>>,
    ) -> Result<(), Failure> {
        if !notes.is_null() {
            if ((iges_array1_lower(notes, "IgesDimenDimensionDisplayData::init")? != 1)
                || ((iges_array1_lower(start_ind, "IgesDimenDimensionDisplayData::init")? != 1)
                    || (iges_array1_length(notes, "IgesDimenDimensionDisplayData::init")?
                        != iges_array1_length(start_ind, "IgesDimenDimensionDisplayData::init")?)))
                || ((iges_array1_lower(end_ind, "IgesDimenDimensionDisplayData::init")? != 1)
                    || (iges_array1_length(notes, "IgesDimenDimensionDisplayData::init")?
                        != iges_array1_length(end_ind, "IgesDimenDimensionDisplayData::init")?))
            {
                return Err(exceptions::dimension_mismatch(
                    "IgesDimenDimensionDisplayData : Init",
                ));
            }
        }
        {
            let mut state = self.state.write();
            state.the_nb_property_values = num_props;
            state.the_dimension_type = a_dim_type;
            state.the_label_position = a_label_pos;
            state.the_character_set = a_char_set;
            state.the_l_string = a_string.clone();
            state.the_decimal_symbol = a_symbol;
            state.the_witness_line_angle = an_ang;
            state.the_text_alignment = an_align;
            state.the_text_level = a_level;
            state.the_text_placement = a_place;
            state.the_arrow_head_orientation = an_orient;
            state.the_initial_value = init_val;
            state.the_supplementary_notes = notes.clone();
            state.the_start_index = start_ind.clone();
            state.the_end_index = end_ind.clone();
        }
        self.init_type_and_form(406, 30);
        Ok(())
    }
    /// returns the number of supplementary notes or zero
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn nb_supplementary_notes(&self) -> Result<i32, Failure> {
        Ok(if self.state.read().the_supplementary_notes.is_null() {
            0
        } else {
            iges_array1_length(
                &self.state.read().the_supplementary_notes,
                "IgesDimenDimensionDisplayData::nb_supplementary_notes",
            )?
        })
    }
    /// returns the Index'th supplementary note
    /// raises exception if Index at most 0 or Index &gt; NbSupplementaryNotes()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn supplementary_note(&self, num: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_supplementary_notes,
            num,
            "IgesDimenDimensionDisplayData::supplementary_note",
        )
    }
    /// returns the Index'th note start index
    /// raises exception if Index at most 0 or Index &gt; NbSupplementaryNotes()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn start_index(&self, num: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_start_index,
            num,
            "IgesDimenDimensionDisplayData::start_index",
        )
    }
    /// returns the Index'th note end index
    /// raises exception if Index at most 0 or Index &gt; NbSupplemetaryNotes()
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn end_index(&self, num: i32) -> Result<i32, Failure> {
        iges_array1_item(
            &self.state.read().the_end_index,
            num,
            "IgesDimenDimensionDisplayData::end_index",
        )
    }
}
