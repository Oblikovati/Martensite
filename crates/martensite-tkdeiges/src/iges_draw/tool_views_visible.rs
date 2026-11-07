//! IgesDrawToolViewsVisible.
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
    IgesDataViewKindEntity, iges_array1_set, iges_dump_entities, iges_live, iges_new_array1,
};
use crate::iges_draw::IgesDrawViewsVisible;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

iges_tool!(
    IgesDrawToolViewsVisible,
    "The own-parameter work on `IgesDrawViewsVisible`."
);

impl IgesDrawToolViewsVisible {
    /// Reads own parameters from file. `PR` gives access to them,
    /// `IR` detains parameter types and values
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesDrawViewsVisible>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let mut temp_nb_views_visible: i32 = Default::default();
        let mut temp_nb_displayed_entities: i32 = Default::default();
        let mut temp_view_entities: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>> =
            Default::default();
        let mut temp_display_entity: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            Default::default();
        if pr.read_integer_text(
            &pr.current(),
            "Number Of Views Visible",
            &mut temp_nb_views_visible,
        )? {
            if temp_nb_views_visible <= 0 {
                pr.add_fail("Number Of Views Visible : Not Positive", "");
            } else {
                temp_view_entities = iges_new_array1::<Handle<dyn IgesDataViewKindEntity>>(
                    1,
                    temp_nb_views_visible,
                )?;
            }
        }
        if pr.defined_else_skip()? {
            pr.read_integer_text(
                &pr.current(),
                "Number of Entities Displayed",
                &mut temp_nb_displayed_entities,
            )?;
        } else {
            temp_nb_displayed_entities = 0;
            pr.add_warning("Number of Entities Displayed : undefined, set to Zero", "");
        }
        if temp_nb_displayed_entities < 0 {
            pr.add_fail("Number Of Entities Displayed : Less than Zero", "");
        }
        if !temp_view_entities.is_null() {
            let mut i: i32 = Default::default();
            for i in 1..=temp_nb_views_visible {
                let mut temp_view_entity1: Handle<dyn IgesDataViewKindEntity> = Default::default();
                if pr.read_entity_typed(
                    ir,
                    &pr.current(),
                    "View Entity",
                    <dyn IgesDataViewKindEntity>::static_type(),
                    &mut temp_view_entity1,
                    false,
                )? {
                    iges_array1_set(
                        &temp_view_entities,
                        i,
                        temp_view_entity1.clone(),
                        "IgesDrawToolViewsVisible::read_own_params",
                    )?;
                }
            }
        }
        if temp_nb_displayed_entities > 0 {
            pr.read_ents(
                ir,
                &pr.current_list(temp_nb_displayed_entities, 1),
                "Displayed Entities",
                &mut temp_display_entity,
                1,
            )?;
        }
        self.dir_checker(ent).check_type_and_form(pr.c_check(), ent);
        ent.init(&temp_view_entities, &temp_display_entity)?;
        Ok(())
    }
    /// Writes own parameters to the writer
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesDrawViewsVisible>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        iw.send(ent.nb_views()?)?;
        iw.send(ent.nb_displayed_entities()?)?;
        let mut i: i32 = Default::default();
        let mut up: i32 = ent.nb_views()?;
        for i in 1..=up {
            iw.send_entity(&ent.view_item(i)?, false)?;
        }
        up = ent.nb_displayed_entities()?;
        for i in 1..=up {
            iw.send_entity(&ent.displayed_entity(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the Entities shared by a ViewsVisible `ent`, from
    /// its specific (own) parameters shared not implied (the Views)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesDrawViewsVisible>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let mut i: i32 = Default::default();
        let mut up: i32 = Default::default();
        up = ent.nb_views()?;
        for i in 1..=up {
            iter.get_one_item(&ent.view_item(i)?.cast());
        }
        Ok(())
    }
    /// Lists the Entities shared by a ViewsVisible `ent`, from
    /// its specific (own) implied parameters : the Displayed Entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_implied(
        &self,
        ent: &Handle<IgesDrawViewsVisible>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0045!(ent, i, iter, up);
        Ok(())
    }
    /// Returns specific DirChecker
    pub fn dir_checker(&self, _unused_0: &Handle<IgesDrawViewsVisible>) -> IgesDataDirChecker {
        let mut dc: IgesDataDirChecker = IgesDataDirChecker::with_form(402, 3);
        crate::iges_shared_blocks::iges_block_0013!(dc);
        dc
    }
    /// Performs Specific Semantic Check
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_check(
        &self,
        ent: &Handle<IgesDrawViewsVisible>,
        _unused_1: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let mut res: i32 = 0;
        let mut nb: i32 = ent.nb_displayed_entities()?;
        let mut entcomp: Handle<dyn IgesDataViewKindEntity> = ent.cast();
        for i in 1..=nb {
            let mut displayed: Handle<IgesDataIgesEntity> = ent.displayed_entity(i)?;
            if entcomp.object_address()
                != iges_live(&displayed, "IgesDrawToolViewsVisible::own_check")?
                    .view()
                    .object_address()
            {
                res += 1;
            }
        }
        crate::iges_shared_blocks::iges_block_0050!(ach, mess, res);
        Ok(())
    }
    /// Copies Specific Parameters shared not implied, i.e. all but
    /// the Displayed Entities
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_copy(
        &self,
        another: &Handle<IgesDrawViewsVisible>,
        ent: &Handle<IgesDrawViewsVisible>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let mut temp_view_entities: Handle<HArray1<Handle<dyn IgesDataViewKindEntity>>> =
            iges_new_array1::<Handle<dyn IgesDataViewKindEntity>>(1, another.nb_views()?)?;
        let mut i: i32 = Default::default();
        let mut up: i32 = Default::default();
        up = another.nb_views()?;
        for i in 1..=up {
            let mut temp_view: Handle<dyn IgesDataViewKindEntity> =
                tc.transferred(&another.view_item(i)?.cast())?.cast();
            iges_array1_set(
                &temp_view_entities,
                i,
                temp_view.clone(),
                "IgesDrawToolViewsVisible::own_copy",
            )?;
        }
        let mut temp_display_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            Default::default();
        ent.init(&temp_view_entities, &temp_display_entities)?;
        Ok(())
    }
    /// Copies Specific implied Parameters : the Displayed Entities
    /// which have already been copied
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_renew(
        &self,
        another: &Handle<IgesDrawViewsVisible>,
        ent: &Handle<IgesDrawViewsVisible>,
        tc: &InterfaceCopyTool,
    ) -> Result<(), Failure> {
        crate::iges_shared_blocks::iges_block_0002!(
            anew,
            another,
            i,
            newdisp,
            tc,
            temp_display_entities,
            up
        );
        {
            newdisp.start();
            while newdisp.more() {
                i += 1;
                let mut temp_entity: Handle<IgesDataIgesEntity> = newdisp.value()?.cast();
                iges_array1_set(
                    &temp_display_entities,
                    i,
                    temp_entity.clone(),
                    "IgesDrawToolViewsVisible::own_renew",
                )?;
                newdisp.next();
            }
        }
        ent.init_implied(&temp_display_entities)?;
        Ok(())
    }
    /// Clears specific implied parameters, which cause looping
    /// structures; required for deletion
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_when_delete(&self, ent: &Handle<IgesDrawViewsVisible>) -> Result<(), Failure> {
        let mut temp_display_entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> =
            Default::default();
        ent.init_implied(&temp_display_entities)?;
        Ok(())
    }
    /// Dump of Specific Parameters
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesDrawViewsVisible>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        write!(s, "IgesDrawViewsVisible\nViews Visible : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_views()?, |first| {
            ent.view_item(first)
        })?;
        write!(s, "\nEntities Displayed : ")?;
        iges_dump_entities(s, dumper, level, 1, ent.nb_displayed_entities()?, |first| {
            ent.displayed_entity(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
    /// Sets automatic unambiguous Correction on a ViewsVisible
    /// (all displayed entities must refer to `ent` in directory part,
    /// else the list is cleared)
    ///
    /// # Errors
    /// A failure when a list or a referred entity is not set up, or a rank is out of range.
    pub fn own_correct(&self, ent: &Handle<IgesDrawViewsVisible>) -> Result<bool, Failure> {
        let mut res: bool = false;
        let mut nb: i32 = ent.nb_displayed_entities()?;
        let mut entcomp: Handle<dyn IgesDataViewKindEntity> = ent.cast();
        for i in 1..=nb {
            let mut displayed: Handle<IgesDataIgesEntity> = ent.displayed_entity(i)?;
            if entcomp.object_address()
                != iges_live(&displayed, "IgesDrawToolViewsVisible::own_correct")?
                    .view()
                    .object_address()
            {
                res = true;
            }
        }
        crate::iges_shared_blocks::iges_block_0051!(ent, nul_display_entities, res);
    }
}
