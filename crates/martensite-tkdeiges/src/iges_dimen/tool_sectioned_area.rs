//! IgesDimenToolSectionedArea.
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
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataDumpResult, IgesDataIgesDumper,
    IgesDataIgesEntity, IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader,
    iges_array1_set, iges_dump_entities, iges_dump_xyz_l, iges_new_array1, iges_real_text,
};
use crate::iges_dimen::IgesDimenSectionedArea;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolSectionedArea,
    "The own-parameter work on `IgesDimenSectionedArea`."
);

impl IgesDimenToolSectionedArea {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenSectionedArea>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut ext_curve: Handle<IgesDataIgesEntity> = Default::default();
        let mut temp_pattern: i32 = Default::default();
        let mut nbislands: i32 = Default::default();
        let mut pass_pnt: Xyz = Default::default();
        let mut temp_distance: f64 = Default::default();
        let mut temp_angle: f64 = Default::default();
        let mut temp_islands: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Default::default();
        let mut anent: Handle<IgesDataIgesEntity> = Default::default();
        pr.read_entity(
            ir,
            &pr.current(),
            "Exterior curve",
            &mut ext_curve,
            ent.form_number() == 1,
        )?;
        pr.read_integer_text(&pr.current(), "Fill pattern", &mut temp_pattern)?;
        pr.read_xyz(&pr.current_list(1, 3), "Passing point", &mut pass_pnt)?;
        pr.read_real_text(&pr.current(), "Distance between lines", &mut temp_distance)?;
        if pr.defined_else_skip()? {
            pr.read_real_text(
                &pr.current(),
                "Angle between line and X axis",
                &mut temp_angle,
            )?;
        } else {
            temp_angle = (std::f64::consts::PI) / 4.0;
        }
        let mut st: bool =
            pr.read_integer_text(&pr.current(), "Number of island curves", &mut nbislands)?;
        if st && (nbislands > 0) {
            pr.read_ents(
                ir,
                &pr.current_list(nbislands, 1),
                "Island curves",
                &mut temp_islands,
                1,
            )?;
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            &ext_curve,
            temp_pattern,
            pass_pnt,
            temp_distance,
            temp_angle,
            &temp_islands,
        )?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenSectionedArea>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut length: i32 = ent.nb_islands()?;
        iw.send_entity(&ent.exterior_curve(), false)?;
        iw.send(ent.pattern())?;
        iw.send(ent.passing_point().x())?;
        iw.send(ent.passing_point().y())?;
        iw.send(ent.passing_point().z())?;
        iw.send(ent.distance())?;
        iw.send(ent.angle())?;
        iw.send(length)?;
        for i in 1..=length {
            iw.send_entity(&ent.island_curve(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a SectionedArea `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDimenSectionedArea>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut length: i32 = ent.nb_islands()?;
        iter.get_one_item(&ent.exterior_curve().cast());
        for i in 1..=length {
            iter.get_one_item(&ent.island_curve(i)?.cast());
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDimenSectionedArea>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_forms(230, 0, 1);
        crate::iges_shared_blocks::iges_block_0039!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesDimenSectionedArea>,
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
        another: &Handle<IgesDimenSectionedArea>,
        ent: &Handle<IgesDimenSectionedArea>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut ext_curve: Handle<IgesDataIgesEntity> =
            tc.transferred(&another.exterior_curve().cast())?.cast();
        let mut temp_pattern: i32 = another.pattern();
        let mut pass_pnt: Xyz = another.passing_point().xyz();
        let mut temp_distance: f64 = another.distance();
        let mut temp_angle: f64 = another.angle();
        let mut temp_islands: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Default::default();
        let mut nbislands: i32 = another.nb_islands()?;
        if nbislands > 0 {
            temp_islands = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nbislands)?;
            for i in 1..=nbislands {
                let mut anent: Handle<IgesDataIgesEntity> =
                    tc.transferred(&another.island_curve(i)?.cast())?.cast();
                iges_array1_set(
                    &temp_islands,
                    i,
                    anent.clone(),
                    "IgesDimenToolSectionedArea::own_copy",
                )?;
            }
        }
        ent.init(
            &ext_curve,
            temp_pattern,
            pass_pnt,
            temp_distance,
            temp_angle,
            &temp_islands,
        )?;
        ent.set_inverted(another.is_inverted());
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenSectionedArea>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level <= 4 { 0 } else { 1 };
        write!(
            s,
            "IgesDimenSectionedArea\n{} - Exterior curve : ",
            if ent.is_inverted() {
                "Inverted Cross Hatches"
            } else {
                "Standard Cross Hatches"
            }
        )?;
        dumper.dump(&ent.exterior_curve(), s, sublevel, -1)?;
        write!(
            s,
            "\nFill pattern code : {}\nPassing point : ",
            ent.pattern()
        )?;
        iges_dump_xyz_l(s, level, &ent.passing_point().xyz(), &ent.location()?)?;
        writeln!(s)?;
        write!(
            s,
            "Distance between lines : {}\nAngle between lines and X axis : {}\nIsland Curve : ",
            iges_real_text(ent.distance()),
            iges_real_text(ent.angle())
        )?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_islands()?, |first| {
            ent.island_curve(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
