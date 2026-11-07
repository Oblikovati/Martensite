//! IgesDrawToolLabelDisplay.
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
    IgesDataViewKindEntity, iges_array1_set, iges_dump_xyz_l, iges_new_array1,
};
use crate::iges_dimen::IgesDimenLeaderArrow;
use crate::iges_draw::IgesDrawLabelDisplay;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkmath::gp::Xyz;
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDrawToolLabelDisplay,
    "The own-parameter work on `IgesDrawLabelDisplay`."
);

impl IgesDrawToolLabelDisplay {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDrawLabelDisplay>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0020!(
            displayed_entities,
            label_levels,
            leader_entities,
            nbval,
            text_locations,
            views
        );
        let mut st: bool =
            pr.read_integer_text(&pr.current(), "No. of Label placements", &mut nbval)?;
        if st && (nbval > 0) {
            views = iges_new_array1::<Handle<dyn IgesDataViewKindEntity>>(1, nbval)?;
            text_locations = iges_new_array1::<Xyz>(1, nbval)?;
            leader_entities = iges_new_array1::<Handle<IgesDimenLeaderArrow>>(1, nbval)?;
            label_levels = iges_new_array1::<i32>(1, nbval)?;
            displayed_entities = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nbval)?;
            let mut temp_view: Handle<dyn IgesDataViewKindEntity> = Default::default();
            let mut temp_xyz: Xyz = Default::default();
            let mut temp_leader_arrow: Handle<IgesDimenLeaderArrow> = Default::default();
            let mut temp_displayed_entity: Handle<IgesDataIgesEntity> = Default::default();
            for i in 1..=nbval {
                if pr.read_entity_typed(
                    ir,
                    &pr.current(),
                    "Instance of views",
                    <dyn IgesDataViewKindEntity>::static_type(),
                    &mut temp_view,
                    false,
                )? {
                    iges_array1_set(
                        &views,
                        i,
                        temp_view.clone(),
                        "IgesDrawToolLabelDisplay::read_own_params",
                    )?;
                }
                if pr.read_xyz(&pr.current_list(1, 3), "array textLocations", &mut temp_xyz)? {
                    iges_array1_set(
                        &text_locations,
                        i,
                        temp_xyz,
                        "IgesDrawToolLabelDisplay::read_own_params",
                    )?;
                }
                if pr.read_entity_typed(
                    ir,
                    &pr.current(),
                    "Instance of LeaderArrow",
                    <IgesDimenLeaderArrow>::static_type(),
                    &mut temp_leader_arrow,
                    false,
                )? {
                    iges_array1_set(
                        &leader_entities,
                        i,
                        temp_leader_arrow.clone(),
                        "IgesDrawToolLabelDisplay::read_own_params",
                    )?;
                }
                iges_read_integer_item(
                    pr,
                    "array labelLevels",
                    &label_levels,
                    i,
                    "IgesDrawToolLabelDisplay::read_own_params",
                )?;
                if pr.read_entity(
                    ir,
                    &pr.current(),
                    "displayedEntities entity",
                    &mut temp_displayed_entity,
                    false,
                )? {
                    iges_array1_set(
                        &displayed_entities,
                        i,
                        temp_displayed_entity.clone(),
                        "IgesDrawToolLabelDisplay::read_own_params",
                    )?;
                }
            }
        } else {
            pr.add_fail("No. of Label placements : Not Positive", "");
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(
            &views,
            &text_locations,
            &leader_entities,
            &label_levels,
            &displayed_entities,
        )?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDrawLabelDisplay>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let mut up: i32 = ent.nb_labels()?;
        iw.send(up)?;
        for i in 1..=up {
            iw.send_entity(&ent.view_item(i)?, false)?;
            iw.send(ent.text_location(i)?.x())?;
            iw.send(ent.text_location(i)?.y())?;
            iw.send(ent.text_location(i)?.z())?;
            iw.send_entity(&ent.leader_entity(i)?, false)?;
            iw.send(ent.label_level(i)?)?;
            iw.send_entity(&ent.displayed_entity(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a LabelDisplay `ent`, from
    /// its specific (own) parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDrawLabelDisplay>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut up: i32 = ent.nb_labels()?;
        for i in 1..=up {
            iter.get_one_item(&ent.view_item(i)?.cast());
            iter.get_one_item(&ent.leader_entity(i)?.cast());
            iter.get_one_item(&ent.displayed_entity(i)?.cast());
        }
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDrawLabelDisplay>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(402, 5);
        dc.structure(IgesDataDefType::DefVoid);
        dc.hierarchy_status_ignored();
        dc.blank_status_ignored();
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        _unused_0: &Handle<IgesDrawLabelDisplay>,
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
        another: &Handle<IgesDrawLabelDisplay>,
        ent: &Handle<IgesDrawLabelDisplay>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0020!(
            displayed_entities,
            label_levels,
            leader_entities,
            nbval,
            text_locations,
            views
        );
        nbval = another.nb_labels()?;
        views = iges_new_array1::<Handle<dyn IgesDataViewKindEntity>>(1, nbval)?;
        text_locations = iges_new_array1::<Xyz>(1, nbval)?;
        leader_entities = iges_new_array1::<Handle<IgesDimenLeaderArrow>>(1, nbval)?;
        label_levels = iges_new_array1::<i32>(1, nbval)?;
        displayed_entities = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, nbval)?;
        for i in 1..=nbval {
            let mut temp_view: Handle<dyn IgesDataViewKindEntity> =
                tc.transferred(&another.view_item(i)?.cast())?.cast();
            iges_array1_set(
                &views,
                i,
                temp_view.clone(),
                "IgesDrawToolLabelDisplay::own_copy",
            )?;
            iges_array1_set(
                &text_locations,
                i,
                another.text_location(i)?.xyz(),
                "IgesDrawToolLabelDisplay::own_copy",
            )?;
            let mut temp_arrow: Handle<IgesDimenLeaderArrow> =
                tc.transferred(&another.leader_entity(i)?.cast())?.cast();
            iges_array1_set(
                &leader_entities,
                i,
                temp_arrow.clone(),
                "IgesDrawToolLabelDisplay::own_copy",
            )?;
            iges_array1_set(
                &label_levels,
                i,
                another.label_level(i)?,
                "IgesDrawToolLabelDisplay::own_copy",
            )?;
            let mut temp_entity: Handle<IgesDataIgesEntity> =
                tc.transferred(&another.displayed_entity(i)?.cast())?.cast();
            iges_array1_set(
                &displayed_entities,
                i,
                temp_entity.clone(),
                "IgesDrawToolLabelDisplay::own_copy",
            )?;
        }
        ent.init(
            &views,
            &text_locations,
            &leader_entities,
            &label_levels,
            &displayed_entities,
        )?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDrawLabelDisplay>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let mut sublevel: i32 = if level <= 4 { 0 } else { 1 };
        writeln!(
            s,
            "IgesDrawLabelDisplay\nView Entities       :\nText Locations      :\nLeader Entities     :\nLabel Level Numbers :\nDisplayed Entities  : Count = {}",
            ent.nb_labels()?
        )?;
        if level > 4 {
            let mut i: i32 = Default::default();
            let mut up: i32 = ent.nb_labels()?;
            for i in 1..=up {
                write!(s, "[{}]:\nView Entity : ", i)?;
                dumper.dump(&ent.view_item(i)?, s, sublevel, -1)?;
                write!(s, "\nText Location in View : ")?;
                iges_dump_xyz_l(s, level, &ent.text_location(i)?.xyz(), &ent.location()?)?;
                write!(s, "  Leader Entity in View : ")?;
                dumper.dump(&ent.leader_entity(i)?, s, sublevel, -1)?;
                write!(
                    s,
                    "\nEntity Label Level Number : {}  Displayed Entity : ",
                    ent.label_level(i)?
                )?;
                dumper.dump(&ent.displayed_entity(i)?, s, sublevel, -1)?;
                writeln!(s)?;
            }
        }
        writeln!(s)?;
        Ok(())
    }
}
