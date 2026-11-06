//! IgesDrawToolCircArraySubfigure.
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
use crate::iges_draw::IgesDrawCircArraySubfigure;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDrawToolCircArraySubfigure,
    "The own-parameter work on `IgesDrawCircArraySubfigure`."
);

impl IgesDrawToolCircArraySubfigure {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDrawCircArraySubfigure>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_center: Xyz = Default::default();
        let mut temp_base: Handle<IgesDataIgesEntity> = Default::default();
        let mut temp_radius: f64 = Default::default();
        let mut temp_st_angle: f64 = Default::default();
        let mut temp_del_angle: f64 = Default::default();
        let mut temp_num_locs: i32 = Default::default();
        let mut temp_flag: i32 = Default::default();
        let mut temp_list_count: i32 = Default::default();
        let mut temp_num_pos: Handle<HArray1<i32>> = Default::default();
        pr.read_entity(ir, &pr.current(), "Base Entity", &mut temp_base, false)?;
        pr.read_integer_text(
            &pr.current(),
            "Number Of Instance Locations",
            &mut temp_num_locs,
        )?;
        pr.read_xyz(
            &pr.current_list(1, 3),
            "Imaginary Circle Center Coordinate",
            &mut temp_center,
        )?;
        pr.read_real_text(
            &pr.current(),
            "Radius Of Imaginary Circle",
            &mut temp_radius,
        )?;
        pr.read_real_text(&pr.current(), "Start Angle in Radians", &mut temp_st_angle)?;
        pr.read_real_text(&pr.current(), "Delta Angle in Radians", &mut temp_del_angle)?;
        if pr.read_integer_text(&pr.current(), "DO-DONT List Count", &mut temp_list_count)? {
            if temp_list_count > 0 {
                temp_num_pos = iges_new_array1::<i32>(1, temp_list_count)?;
            } else {
                if temp_list_count < 0 {
                    pr.add_fail("DO-DONT List Count : Less than Zero", "");
                }
            }
        }
        pr.read_integer_text(&pr.current(), "DO-DONT Flag", &mut temp_flag)?;
        if !temp_num_pos.is_null() {
            let mut i: i32 = Default::default();
            for i in 1..=temp_list_count {
                iges_read_integer_item(
                    pr,
                    "Number Of Position To Process",
                    &temp_num_pos,
                    i,
                    "IgesDrawToolCircArraySubfigure::read_own_params",
                )?;
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            &temp_base,
            temp_num_locs,
            temp_center,
            temp_radius,
            temp_st_angle,
            temp_del_angle,
            temp_flag,
            &temp_num_pos,
        )?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDrawCircArraySubfigure>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send_entity(&ent.base_entity(), false)?;
        iw.send(ent.nb_locations())?;
        iw.send(ent.center_point().x())?;
        iw.send(ent.center_point().y())?;
        iw.send(ent.center_point().z())?;
        iw.send(ent.circle_radius())?;
        iw.send(ent.start_angle())?;
        iw.send(ent.delta_angle())?;
        crate::iges_shared_blocks::iges_block_0034!(ent, i, iw, up);
        Ok(())
    }
    /// Lists the entities shared by the entity: its base entity.
    ///
    /// # Errors
    /// A failure of an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDrawCircArraySubfigure>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        iter.get_one_item(&ent.base_entity().cast());
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(
        &self,
        _unused_0: &Handle<IgesDrawCircArraySubfigure>,
    ) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(414, 0);
        crate::iges_shared_blocks::iges_block_0038!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesDrawCircArraySubfigure>,
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
        another: &Handle<IgesDrawCircArraySubfigure>,
        ent: &Handle<IgesDrawCircArraySubfigure>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut temp_base: Handle<IgesDataIgesEntity> =
            tc.transferred(&another.base_entity().cast())?.cast();
        let mut temp_num_locs: i32 = another.nb_locations();
        let mut temp_center: Xyz = another.center_point().xyz();
        let mut temp_radius: f64 = another.circle_radius();
        let mut temp_st_angle: f64 = another.start_angle();
        let mut temp_del_angle: f64 = another.delta_angle();
        let mut temp_list_count: i32 = another.list_count()?;
        let mut temp_flag: i32 = i32::from(another.do_dont_flag());
        let mut temp_num_pos: Handle<HArray1<i32>> = Default::default();
        if !another.display_flag() {
            temp_num_pos = iges_new_array1::<i32>(1, temp_list_count)?;
            let mut i: i32 = Default::default();
            for i in 1..=temp_list_count {
                let mut temp_position: i32 = another.list_position(i)?;
                iges_array1_set(
                    &temp_num_pos,
                    i,
                    temp_position,
                    "IgesDrawToolCircArraySubfigure::own_copy",
                )?;
            }
        }
        ent.init(
            &temp_base,
            temp_num_locs,
            temp_center,
            temp_radius,
            temp_st_angle,
            temp_del_angle,
            temp_flag,
            &temp_num_pos,
        )?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDrawCircArraySubfigure>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut temp_sub_level: i32 = if level <= 4 { 0 } else { 1 };
        write!(s, "IgesDrawCircArraySubfigure\nBase Entity : ")?;
        dumper.dump(&ent.base_entity(), s, temp_sub_level, -1)?;
        write!(
            s,
            "\nTotal Number Of Possible Instance Locations : {}\nImaginary Circle. Radius : {}  Center : ",
            ent.nb_locations(),
            iges_real_text(ent.circle_radius())
        )?;
        iges_dump_xyz_l(s, level, &ent.center_point().xyz(), &ent.location()?)?;
        writeln!(s)?;
        write!(
            s,
            "Start Angle (in radians) : {}  Delta Angle (in radians) : {}\nDo-Dont Flag : ",
            iges_real_text(ent.start_angle()),
            iges_real_text(ent.delta_angle())
        )?;
        if ent.do_dont_flag() {
            writeln!(s, "Dont")?;
        } else {
            writeln!(s, "Do")?;
        }
        write!(s, "The Do-Dont List : ")?;
        iges_dump_vals(s, level, 1, ent.list_count()?, |first| {
            ent.list_position(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
