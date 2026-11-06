//! IgesDrawToolRectArraySubfigure.
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
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesEntity, IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader,
    iges_array1_set, iges_dump_vals, iges_dump_xyz_l, iges_new_array1, iges_real_text,
};
use crate::iges_draw::IgesDrawRectArraySubfigure;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{GeneralTransform3, Xyz};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDrawToolRectArraySubfigure,
    "The own-parameter work on `IgesDrawRectArraySubfigure`."
);

impl IgesDrawToolRectArraySubfigure {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDrawRectArraySubfigure>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_lower_left_corner: Xyz = Default::default();
        let mut temp_scale_factor: f64 = Default::default();
        let mut temp_base_entity: Handle<IgesDataIgesEntity> = Default::default();
        let mut temp_positions: Handle<HArray1<i32>> = Default::default();
        let mut temp_column_separation: f64 = Default::default();
        let mut temp_row_separation: f64 = Default::default();
        let mut temp_rotation_angle: f64 = Default::default();
        let mut temp_nb_columns: i32 = Default::default();
        let mut temp_nb_rows: i32 = Default::default();
        let mut temp_do_dont_flag: i32 = Default::default();
        let mut temp_list_count: i32 = Default::default();
        pr.read_entity(
            ir,
            &pr.current(),
            "Base Entity",
            &mut temp_base_entity,
            false,
        )?;
        if pr.defined_else_skip()? {
            pr.read_real_text(&pr.current(), "Scale Factor", &mut temp_scale_factor)?;
        } else {
            temp_scale_factor = 1.0;
        }
        pr.read_xyz(
            &pr.current_list(1, 3),
            "Lower Left Coordinate Of Array",
            &mut temp_lower_left_corner,
        )?;
        pr.read_integer_text(&pr.current(), "Number Of Columns", &mut temp_nb_columns)?;
        pr.read_integer_text(&pr.current(), "Number Of Rows", &mut temp_nb_rows)?;
        pr.read_real_text(
            &pr.current(),
            "Horizontal Distance Between Columns",
            &mut temp_column_separation,
        )?;
        pr.read_real_text(
            &pr.current(),
            "Vertical Distance Between Rows",
            &mut temp_row_separation,
        )?;
        pr.read_real_text(&pr.current(), "Rotation Angle", &mut temp_rotation_angle)?;
        if pr.read_integer_text(&pr.current(), "DO-DONT List Count", &mut temp_list_count)? {
            if temp_list_count > 0 {
                temp_positions = iges_new_array1::<i32>(1, temp_list_count)?;
            } else {
                if temp_list_count < 0 {
                    pr.add_fail("DO-DONT List Count : Less than Zero", "");
                }
            }
        }
        pr.read_integer_text(&pr.current(), "DO-DONT Flag", &mut temp_do_dont_flag)?;
        if !temp_positions.is_null() {
            let mut i: i32 = Default::default();
            for i in 1..=temp_list_count {
                iges_read_integer_item(
                    pr,
                    "Number Of Position To Process",
                    &temp_positions,
                    i,
                    "IgesDrawToolRectArraySubfigure::read_own_params",
                )?;
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            &temp_base_entity,
            temp_scale_factor,
            temp_lower_left_corner,
            temp_nb_columns,
            temp_nb_rows,
            temp_column_separation,
            temp_row_separation,
            temp_rotation_angle,
            temp_do_dont_flag,
            &temp_positions,
        )?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDrawRectArraySubfigure>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send_entity(&ent.base_entity(), false)?;
        iw.send(ent.scale_factor())?;
        crate::iges_shared_blocks::iges_block_0062!(ent, iw);
        iw.send(ent.nb_columns())?;
        iw.send(ent.nb_rows())?;
        iw.send(ent.column_separation())?;
        iw.send(ent.row_separation())?;
        iw.send(ent.rotation_angle())?;
        crate::iges_shared_blocks::iges_block_0034!(ent, i, iw, up);
        Ok(())
    }
    /// Lists the Entities shared by a RectArraySubfigure `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDrawRectArraySubfigure>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.base_entity().cast());
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesDrawRectArraySubfigure>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(412, 0);
        crate::iges_shared_blocks::iges_block_0038!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesDrawRectArraySubfigure>,
        _unused_1: &InterfaceShareTool,
        _unused_2: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        Ok(())
    }
    /// Copies Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDrawRectArraySubfigure>,
        ent: &Handle<IgesDrawRectArraySubfigure>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut temp_base_entity: Handle<IgesDataIgesEntity> =
            tc.transferred(&another.base_entity().cast())?.cast();
        let mut temp_scale_factor: f64 = another.scale_factor();
        let mut temp_lower_left_corner: Xyz = another.lower_left_corner().xyz();
        let mut temp_nb_columns: i32 = another.nb_columns();
        let mut temp_nb_rows: i32 = another.nb_rows();
        let mut temp_column_separation: f64 = another.column_separation();
        let mut temp_row_separation: f64 = another.row_separation();
        let mut temp_rotation_angle: f64 = another.rotation_angle();
        let mut temp_list_count: i32 = another.list_count()?;
        let mut temp_do_dont_flag: i32 = if another.do_dont_flag() { 1 } else { 0 };
        let mut temp_positions: Handle<HArray1<i32>> = Default::default();
        if temp_list_count != 0 {
            temp_positions = iges_new_array1::<i32>(1, temp_list_count)?;
            let mut i: i32 = Default::default();
            for i in 1..=temp_list_count {
                iges_array1_set(
                    &temp_positions,
                    i,
                    another.list_position(i)?,
                    "IgesDrawToolRectArraySubfigure::own_copy",
                )?;
            }
        }
        ent.init(
            &temp_base_entity,
            temp_scale_factor,
            temp_lower_left_corner,
            temp_nb_columns,
            temp_nb_rows,
            temp_column_separation,
            temp_row_separation,
            temp_rotation_angle,
            temp_do_dont_flag,
            &temp_positions,
        )?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDrawRectArraySubfigure>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut temp_sub_level: i32 = if level <= 4 { 0 } else { 1 };
        write!(s, "IgesDrawRectArraySubfigure\nBase Entity : ")?;
        dumper.dump(&ent.base_entity(), s, temp_sub_level, -1)?;
        write!(
            s,
            "\nScale Factor : {}  Lower Left Corner Of Array : ",
            iges_real_text(ent.scale_factor())
        )?;
        iges_dump_xyz_l(
            s,
            level,
            &ent.lower_left_corner().xyz(),
            &GeneralTransform3::default(),
        )?;
        write!(
            s,
            "Number Of Columns : {}  Number Of Rows    : {}\nHorizontal Distance Between Columns : {}\nVertical Distance Between Rows      : {}\nRotation Angle (in radians)         : {}\nDo-Dont Flag : ",
            ent.nb_columns(),
            ent.nb_rows(),
            iges_real_text(ent.column_separation()),
            iges_real_text(ent.row_separation()),
            iges_real_text(ent.rotation_angle())
        )?;
        if ent.do_dont_flag() {
            write!(s, "(1)Dont  ")?;
        } else {
            write!(s, "(0)Do  ")?;
        }
        write!(s, "Do-Dont List : ")?;
        iges_dump_vals(s, level, 1, ent.list_count()?, |first| {
            ent.list_position(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
