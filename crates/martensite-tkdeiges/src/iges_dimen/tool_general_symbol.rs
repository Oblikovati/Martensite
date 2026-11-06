//! IgesDimenToolGeneralSymbol.
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
    iges_array1_set, iges_dump_entities, iges_new_array1,
};
use crate::iges_dimen::{IgesDimenGeneralNote, IgesDimenGeneralSymbol, IgesDimenLeaderArrow};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDimenToolGeneralSymbol,
    "The own-parameter work on `IgesDimenGeneralSymbol`."
);

impl IgesDimenToolGeneralSymbol {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDimenGeneralSymbol>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
        let mut temp_geoms: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Default::default();
        let mut temp_leaders: Handle<HArray1<Handle<IgesDimenLeaderArrow>>> = Default::default();
        let mut temp_note: Handle<IgesDimenGeneralNote> = Default::default();
        if (ent.form_number() == 0) && (!(pr.is_param_entity(pr.current_number())?)) {
            pr.set_current_number(pr.current_number() + 1);
        } else {
            pr.read_entity_typed(
                ir,
                &pr.current(),
                "General Note Entity",
                <IgesDimenGeneralNote>::static_type(),
                &mut temp_note,
                false,
            )?;
        }
        let mut st: bool = pr.read_integer_text(&pr.current(), "Number of Geometries", &mut num)?;
        if (!st) || (num <= 0) {
            pr.add_fail("Number of Geometries: Not Positive", "");
        }
        if num > 0 {
            pr.read_ents(
                ir,
                &pr.current_list(num, 1),
                "Geometry Entities",
                &mut temp_geoms,
                1,
            )?;
        } else {
            return Ok(());
        }
        st = pr.read_integer_text(&pr.current(), "Number of Leaders", &mut num)?;
        if st && (num > 0) {
            temp_leaders = iges_new_array1::<Handle<IgesDimenLeaderArrow>>(1, num)?;
        } else {
            if num < 0 {
                pr.add_fail("Number of Leaders: Less than zero", "");
            }
        }
        if !temp_leaders.is_null() {
            for i in 1..=num {
                let mut temp_ent: Handle<IgesDimenLeaderArrow> = Default::default();
                if pr.read_entity_typed(
                    ir,
                    &pr.current(),
                    "Leader Entity",
                    <IgesDimenLeaderArrow>::static_type(),
                    &mut temp_ent,
                    false,
                )? {
                    iges_array1_set(
                        &temp_leaders,
                        i,
                        temp_ent.clone(),
                        "IgesDimenToolGeneralSymbol::read_own_params",
                    )?;
                }
            }
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(&temp_note, &temp_geoms, &temp_leaders)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDimenGeneralSymbol>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
        iw.send_entity(&ent.note(), false)?;
        iw.send(ent.nb_geom_entities()?)?;
        {
            num = ent.nb_geom_entities()?;
            i = 1;
            while i <= num {
                iw.send_entity(&ent.geom_entity(i)?, false)?;
                i += 1;
            }
        }
        iw.send(ent.nb_leaders()?)?;
        {
            num = ent.nb_leaders()?;
            i = 1;
            while i <= num {
                iw.send_entity(&ent.leader_arrow(i)?, false)?;
                i += 1;
            }
        }
        Ok(())
    }
    /// Lists the Entities shared by a GeneralSymbol `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDimenGeneralSymbol>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut num: i32 = Default::default();
        iter.get_one_item(&ent.note().cast());
        {
            num = ent.nb_geom_entities()?;
            i = 1;
            while i <= num {
                iter.get_one_item(&ent.geom_entity(i)?.cast());
                i += 1;
            }
        }
        {
            num = ent.nb_leaders()?;
            i = 1;
            while i <= num {
                iter.get_one_item(&ent.leader_arrow(i)?.cast());
                i += 1;
            }
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDimenGeneralSymbol>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_forms(228, 0, 9999);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.line_font(IgesDataDefType::DefAny);
        dc.line_weight(IgesDataDefType::DefValue);
        dc.color(IgesDataDefType::DefAny);
        dc.use_flag_required(1);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDimenGeneralSymbol>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        if ((ent.form_number() < 0) || (ent.form_number() > 3))
            && ((ent.form_number() < 5001) || (ent.form_number() > 9999))
        {
            ach.add_fail_text("Invalid Form Number", "");
        }
        if ent.form_number() != 0 {
            if ent.note().is_null() {
                ach.add_fail_text("No General Note defined for form number non 0", "");
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
        another: &Handle<IgesDimenGeneralSymbol>,
        ent: &Handle<IgesDimenGeneralSymbol>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut temp_note: Handle<IgesDimenGeneralNote> =
            tc.transferred(&another.note().cast())?.cast();
        let mut num: i32 = another.nb_geom_entities()?;
        let mut temp_geoms: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            iges_new_array1::<Handle<IgesDataIgesEntity>>(1, num)?;
        let mut i: i32 = Default::default();
        {
            i = 1;
            while i <= num {
                let mut new_item: Handle<IgesDataIgesEntity> =
                    tc.transferred(&another.geom_entity(i)?.cast())?.cast();
                iges_array1_set(
                    &temp_geoms,
                    i,
                    new_item.clone(),
                    "IgesDimenToolGeneralSymbol::own_copy",
                )?;
                i += 1;
            }
        }
        let mut temp_leaders: Handle<HArray1<Handle<IgesDimenLeaderArrow>>> = Default::default();
        num = another.nb_leaders()?;
        if num > 0 {
            temp_leaders = iges_new_array1::<Handle<IgesDimenLeaderArrow>>(1, num)?;
            for i in 1..=num {
                let mut new_item: Handle<IgesDimenLeaderArrow> =
                    tc.transferred(&another.leader_arrow(i)?.cast())?.cast();
                iges_array1_set(
                    &temp_leaders,
                    i,
                    new_item.clone(),
                    "IgesDimenToolGeneralSymbol::own_copy",
                )?;
            }
        }
        ent.init(&temp_note, &temp_geoms, &temp_leaders)?;
        ent.set_form_number(another.form_number())?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDimenGeneralSymbol>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level > 4 { 1 } else { 0 };
        write!(s, "IgesDimenGeneralSymbol\nGeneral Note : ")?;
        dumper.dump(&ent.note(), s, sublevel, -1)?;
        write!(s, "\nGeometric Entities : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_geom_entities()?, |first| {
            ent.geom_entity(first)
        })?;
        write!(s, "\nLeader Arrows : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_leaders()?, |first| {
            ent.leader_arrow(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
