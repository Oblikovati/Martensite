//! IgesDimenToolDimensionDisplayData.
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
use crate::iges_basic::entity_shape::iges_tool;
use crate::iges_data::iges_read_integer_item;
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesReaderData, IgesDataIgesWriter,
    IgesDataParamReader, iges_array1_set, iges_new_array1,
};
use crate::iges_dimen::IgesDimenDimensionDisplayData;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::InterfaceEntityIterator;

iges_tool!(
    IgesDimenToolDimensionDisplayData,
    "The own-parameter work on `IgesDimenDimensionDisplayData`."
);

impl IgesDimenToolDimensionDisplayData {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenDimensionDisplayData>,
        _unused_1: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_dim_type: i32 = Default::default();
        let mut temp_label_pos: i32 = Default::default();
        let mut temp_char_set: i32 = Default::default();
        let mut temp_l_string: Handle<HAsciiString> = Default::default();
        let mut temp_wit_line_ang: f64 = Default::default();
        let mut temp_deci_symb: i32 = Default::default();
        let mut temp_text_align: i32 = Default::default();
        let mut temp_text_level: i32 = Default::default();
        let mut temp_text_place: i32 = Default::default();
        let mut temp_arr_head_orient: i32 = Default::default();
        let mut temp_init_val: f64 = Default::default();
        let mut temp_nb_props: i32 = Default::default();
        let mut temp_supple_notes: Handle<HArray1<i32>> = Default::default();
        let mut temp_start_ind: Handle<HArray1<i32>> = Default::default();
        let mut temp_end_ind: Handle<HArray1<i32>> = Default::default();
        pr.read_integer_text(&pr.current(), "Number of Properties", &mut temp_nb_props)?;
        pr.read_integer_text(&pr.current(), "Dimension Type", &mut temp_dim_type)?;
        pr.read_integer_text(&pr.current(), "Label Position", &mut temp_label_pos)?;
        if pr.defined_else_skip()? {
            pr.read_integer_text(&pr.current(), "Character Set", &mut temp_char_set)?;
        } else {
            temp_char_set = 1;
        }
        pr.read_text(&pr.current(), "L String", &mut temp_l_string)?;
        pr.read_integer_text(&pr.current(), "Decimal Symbol", &mut temp_deci_symb)?;
        if pr.defined_else_skip()? {
            pr.read_real_text(&pr.current(), "Witness Line Angle", &mut temp_wit_line_ang)?;
        } else {
            temp_wit_line_ang = (std::f64::consts::PI) / 2.0;
        }
        pr.read_integer_text(&pr.current(), "Text Alignment", &mut temp_text_align)?;
        if pr.defined_else_skip()? {
            pr.read_integer_text(&pr.current(), "Text Level", &mut temp_text_level)?;
        } else {
            temp_text_level = 0;
        }
        if pr.defined_else_skip()? {
            pr.read_integer_text(&pr.current(), "Text Place", &mut temp_text_place)?;
        } else {
            temp_text_place = 0;
        }
        pr.read_integer_text(
            &pr.current(),
            "ArrowHeadOrientation",
            &mut temp_arr_head_orient,
        )?;
        pr.read_real_text(&pr.current(), "Initial Value", &mut temp_init_val)?;
        let mut tempnbval: i32 = Default::default();
        let mut st: bool =
            pr.read_integer_text(&pr.current(), "No. of supplementary notes", &mut tempnbval)?;
        if st && (tempnbval > 0) {
            temp_supple_notes = iges_new_array1::<i32>(1, tempnbval)?;
            temp_start_ind = iges_new_array1::<i32>(1, tempnbval)?;
            temp_end_ind = iges_new_array1::<i32>(1, tempnbval)?;
            for i in 1..=tempnbval {
                iges_read_integer_item(
                    pr,
                    "Supplementary Notes",
                    &temp_supple_notes,
                    i,
                    "IgesDimenToolDimensionDisplayData::read_own_params",
                )?;
                iges_read_integer_item(
                    pr,
                    "Start Index",
                    &temp_start_ind,
                    i,
                    "IgesDimenToolDimensionDisplayData::read_own_params",
                )?;
                iges_read_integer_item(
                    pr,
                    "End Index",
                    &temp_end_ind,
                    i,
                    "IgesDimenToolDimensionDisplayData::read_own_params",
                )?;
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            temp_nb_props,
            temp_dim_type,
            temp_label_pos,
            temp_char_set,
            &temp_l_string,
            temp_deci_symb,
            temp_wit_line_ang,
            temp_text_align,
            temp_text_level,
            temp_text_place,
            temp_arr_head_orient,
            temp_init_val,
            &temp_supple_notes,
            &temp_start_ind,
            &temp_end_ind,
        )?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenDimensionDisplayData>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_property_values())?;
        iw.send(ent.dimension_type())?;
        iw.send(ent.label_position())?;
        iw.send(ent.character_set())?;
        iw.send(ent.l_string())?;
        iw.send(ent.decimal_symbol())?;
        iw.send(ent.witness_line_angle())?;
        iw.send(ent.text_alignment())?;
        iw.send(ent.text_level())?;
        iw.send(ent.text_placement())?;
        iw.send(ent.arrow_head_orientation())?;
        iw.send(ent.initial_value())?;
        let mut nbval: i32 = ent.nb_supplementary_notes()?;
        iw.send(nbval)?;
        for i in 1..=nbval {
            iw.send(ent.supplementary_note(i)?)?;
            iw.send(ent.start_index(i)?)?;
            iw.send(ent.end_index(i)?)?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a DimensionDisplayData `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        _unused_0: &Handle<IgesDimenDimensionDisplayData>,
        _unused_1: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a DimensionDisplayData
    /// (NbPropertyValues forced to 14)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(
        &self,
        ent: &Handle<IgesDimenDimensionDisplayData>,
    ) -> Result<bool, Failure> {
        let mut res: bool = ent.nb_property_values() != 14;
        if !res {
            return Ok(res);
        }
        crate::iges_shared_blocks::iges_block_0056!(end_list, notes_list, start_list);
        let mut upper: i32 = ent.nb_supplementary_notes()?;
        if upper > 0 {
            end_list = iges_new_array1::<i32>(1, upper)?;
            start_list = iges_new_array1::<i32>(1, upper)?;
            notes_list = iges_new_array1::<i32>(1, upper)?;
            for i in 1..=upper {
                iges_array1_set(
                    &end_list,
                    i,
                    ent.end_index(i)?,
                    "IgesDimenToolDimensionDisplayData::own_correct",
                )?;
                iges_array1_set(
                    &start_list,
                    i,
                    ent.start_index(i)?,
                    "IgesDimenToolDimensionDisplayData::own_correct",
                )?;
                iges_array1_set(
                    &notes_list,
                    i,
                    ent.supplementary_note(i)?,
                    "IgesDimenToolDimensionDisplayData::own_correct",
                )?;
            }
        }
        ent.init(
            14,
            ent.dimension_type(),
            ent.label_position(),
            ent.character_set(),
            &ent.l_string(),
            ent.decimal_symbol(),
            ent.witness_line_angle(),
            ent.text_alignment(),
            ent.text_level(),
            ent.text_placement(),
            ent.arrow_head_orientation(),
            ent.initial_value(),
            &notes_list,
            &start_list,
            &end_list,
        )?;
        return Ok(res);
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesDimenDimensionDisplayData>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(406, 30);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.blank_status_ignored();
        dc.subordinate_status_required(2);
        dc.use_flag_required(2);
        dc.hierarchy_status_ignored();
        dc
    }
}
