#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::{
    IgesBasicOrderedGroup, IgesBasicOrderedGroupWithoutBackP, IgesBasicSingleParent,
    IgesBasicToolOrderedGroup,
};
use crate::iges_basic::entity_shape::{iges_send_read_status, iges_tool};
use crate::iges_basic::shared_runs::{iges_shared_005, iges_shared_030, iges_shared_048};
use crate::iges_data::{
    IgesDataDefType, IgesDataDirChecker, IgesDataIgesDumper, IgesDataIgesEntity,
    IgesDataIgesReaderData, IgesDataIgesWriter, IgesDataParamReader, IgesDataStatus,
};
use crate::iges_data::{
    IgesDataDumpResult, iges_array1_set, iges_dump_entities, iges_live, iges_msg, iges_new_array1,
};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceCopyTool, InterfaceEntityIterator, InterfaceShareTool,
};
use std::io::Write;

impl IgesBasicToolOrderedGroup {
    /// Corrects what can be derived from the other fields; true when something changed.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_correct(&self, ent: &Handle<IgesBasicOrderedGroup>) -> Result<bool, Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolOrderedGroup::own_correct")?;
        let mut ianul: i32 = 0;
        let mut nbtrue: i32 = 0;
        let nb: i32 = ent_live.nb_entities();
        for i in 1..=nb {
            let val: Handle<IgesDataIgesEntity> = ent_live.entity(i)?;
            if val.is_null() {
                ianul += 1;
            } else if iges_live(&val, "IgesBasicToolOrderedGroup::own_correct")?.type_number() == 0
            {
                ianul += 1;
            }
        }
        iges_shared_030!(ent_array, ianul, nb);
        for i in 1..=nb {
            let val: Handle<IgesDataIgesEntity> = ent_live.entity(i)?;
            if val.is_null() {
                continue;
            } else if iges_live(&val, "IgesBasicToolOrderedGroup::own_correct")?.type_number() == 0
            {
                continue;
            }
            nbtrue += 1;
            iges_array1_set(
                &ent_array,
                nbtrue,
                ent_live.entity(i)?,
                "IgesBasicToolOrderedGroup::own_correct",
            )?;
        }
        ent_live.init(&ent_array)?;
        Ok(true)
    }
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesBasicOrderedGroup>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolOrderedGroup::own_check")?;
        let mut ianul: bool = false;
        let nb: i32 = ent_live.nb_entities();
        for i in 1..=nb {
            let val: Handle<IgesDataIgesEntity> = ent_live.entity(i)?;
            if val.is_null() {
                ianul = true;
            } else if iges_live(&val, "IgesBasicToolOrderedGroup::own_check")?.type_number() == 0 {
                ianul = true;
            }
            if ianul {
                ach.add_warning_text("At least one element is Null", "");
                break;
            }
        }
        Ok(())
    }
}
impl IgesBasicToolOrderedGroup {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesBasicOrderedGroup>,
        ent: &Handle<IgesBasicOrderedGroup>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesBasicToolOrderedGroup::own_copy")?;
        let ent_live = iges_live(ent, "IgesBasicToolOrderedGroup::own_copy")?;
        iges_shared_048!(another_live, ent_array, lower, upper);
        for i in lower..=upper {
            let myentity = tc
                .transferred(&another_live.entity(i)?.cast::<StandardTransient>())?
                .cast::<IgesDataIgesEntity>();
            iges_array1_set(
                &ent_array,
                i,
                myentity.clone(),
                "IgesBasicToolOrderedGroup::own_copy",
            )?;
        }
        ent_live.init(&ent_array)?;
        Ok(())
    }
}
impl IgesBasicToolOrderedGroup {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesBasicOrderedGroup>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesBasicToolOrderedGroup::own_dump")?;
        write!(s, "IgesBasicOrderedGroup\nEntries in the Group : ")?;
        iges_dump_entities(s, dumper, level, 1, ent_live.nb_entities(), |first| {
            ent_live.entity(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesBasicToolOrderedGroupWithoutBackP,
    "The own-parameter work on `IgesBasicOrderedGroupWithoutBackP`."
);
impl IgesBasicToolOrderedGroupWithoutBackP {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesBasicOrderedGroupWithoutBackP>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(
            ent,
            "IgesBasicToolOrderedGroupWithoutBackP::read_own_params",
        )?;
        iges_shared_005!(ent, ent_array, ent_live, ir, nbval, pr, self);
        Ok(())
    }
}
impl IgesBasicToolOrderedGroupWithoutBackP {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesBasicOrderedGroupWithoutBackP>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(
            ent,
            "IgesBasicToolOrderedGroupWithoutBackP::write_own_params",
        )?;
        let upper: i32 = ent_live.nb_entities();
        iw.send(ent_live.nb_entities())?;
        for i in 1..=upper {
            iw.send_entity(&ent_live.entity(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesBasicOrderedGroupWithoutBackP>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolOrderedGroupWithoutBackP::own_shared")?;
        let upper: i32 = ent_live.nb_entities();
        for i in 1..=upper {
            iter.get_one_item(&ent_live.entity(i)?.cast::<StandardTransient>());
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(
        &self,
        _ent: &Handle<IgesBasicOrderedGroupWithoutBackP>,
    ) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(402, 15);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.blank_status_ignored();
        dc.hierarchy_status_ignored();
        dc
    }
}
impl IgesBasicToolOrderedGroupWithoutBackP {
    /// Corrects what can be derived from the other fields; true when something changed.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_correct(
        &self,
        ent: &Handle<IgesBasicOrderedGroupWithoutBackP>,
    ) -> Result<bool, Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolOrderedGroupWithoutBackP::own_correct")?;
        let mut ianul: i32 = 0;
        let mut nbtrue: i32 = 0;
        let nb: i32 = ent_live.nb_entities();
        for i in 1..=nb {
            let val: Handle<IgesDataIgesEntity> = ent_live.entity(i)?;
            if val.is_null() {
                ianul += 1;
            } else if iges_live(&val, "IgesBasicToolOrderedGroupWithoutBackP::own_correct")?
                .type_number()
                == 0
            {
                ianul += 1;
            }
        }
        iges_shared_030!(ent_array, ianul, nb);
        for i in 1..=nb {
            let val: Handle<IgesDataIgesEntity> = ent_live.entity(i)?;
            if val.is_null() {
                continue;
            } else if iges_live(&val, "IgesBasicToolOrderedGroupWithoutBackP::own_correct")?
                .type_number()
                == 0
            {
                continue;
            }
            nbtrue += 1;
            iges_array1_set(
                &ent_array,
                nbtrue,
                ent_live.entity(i)?,
                "IgesBasicToolOrderedGroupWithoutBackP::own_correct",
            )?;
        }
        ent_live.init(&ent_array)?;
        Ok(true)
    }
    /// Performs the semantic check of the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_check(
        &self,
        ent: &Handle<IgesBasicOrderedGroupWithoutBackP>,
        _shares: &InterfaceShareTool,
        ach: &Handle<InterfaceCheck>,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolOrderedGroupWithoutBackP::own_check")?;
        let mut ianul: bool = false;
        let nb: i32 = ent_live.nb_entities();
        for i in 1..=nb {
            let val: Handle<IgesDataIgesEntity> = ent_live.entity(i)?;
            if val.is_null() {
                ianul = true;
            } else if iges_live(&val, "IgesBasicToolOrderedGroupWithoutBackP::own_check")?
                .type_number()
                == 0
            {
                ianul = true;
            }
            if ianul {
                ach.add_warning_text("At least one element is Null", "");
                break;
            }
        }
        Ok(())
    }
}
impl IgesBasicToolOrderedGroupWithoutBackP {
    /// Copies the own parameters, through the copy tool for the entities they refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_copy(
        &self,
        another: &Handle<IgesBasicOrderedGroupWithoutBackP>,
        ent: &Handle<IgesBasicOrderedGroupWithoutBackP>,
        tc: &mut InterfaceCopyTool,
    ) -> Result<(), Failure> {
        let another_live = iges_live(another, "IgesBasicToolOrderedGroupWithoutBackP::own_copy")?;
        let ent_live = iges_live(ent, "IgesBasicToolOrderedGroupWithoutBackP::own_copy")?;
        iges_shared_048!(another_live, ent_array, lower, upper);
        for i in lower..=upper {
            let myentity = tc
                .transferred(&another_live.entity(i)?.cast::<StandardTransient>())?
                .cast::<IgesDataIgesEntity>();
            iges_array1_set(
                &ent_array,
                i,
                myentity.clone(),
                "IgesBasicToolOrderedGroupWithoutBackP::own_copy",
            )?;
        }
        ent_live.init(&ent_array)?;
        Ok(())
    }
}
impl IgesBasicToolOrderedGroupWithoutBackP {
    /// Dumps the own parameters.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_dump(
        &self,
        ent: &Handle<IgesBasicOrderedGroupWithoutBackP>,
        dumper: &IgesDataIgesDumper,
        s: &mut dyn Write,
        level: i32,
    ) -> IgesDataDumpResult {
        let ent_live = iges_live(ent, "IgesBasicToolOrderedGroupWithoutBackP::own_dump")?;
        write!(
            s,
            "IgesBasicOrderedGroupWithoutBackP\nEntries in the Group : "
        )?;
        iges_dump_entities(s, dumper, level, 1, ent_live.nb_entities(), |first| {
            ent_live.entity(first)
        })?;
        writeln!(s)?;
        Ok(())
    }
}
iges_tool!(
    IgesBasicToolSingleParent,
    "The own-parameter work on `IgesBasicSingleParent`."
);
impl IgesBasicToolSingleParent {
    /// Reads the own parameters of the entity, then sets it up.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn read_own_params(
        &self,
        ent: &Handle<IgesBasicSingleParent>,
        ir: &Handle<IgesDataIgesReaderData>,
        pr: &mut IgesDataParamReader,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolSingleParent::read_own_params")?;
        let msg207 = iges_msg("XSTEP_207");
        let mut temp_nb_parent_entities: i32 = 0;
        let mut temp_parent: Handle<IgesDataIgesEntity> = Handle::default();
        let mut nbval: i32 = 0;
        let mut temp_children: Handle<HArray1<Handle<IgesDataIgesEntity>>> = Handle::default();
        let mut a_status: IgesDataStatus = IgesDataStatus::default();
        if !pr.read_integer(&pr.current(), &mut temp_nb_parent_entities)? {
            let msg204 = iges_msg("XSTEP_204");
            pr.send_fail(&msg204);
        }
        if !pr.read_integer(&pr.current(), &mut nbval)? {
            let msg205 = iges_msg("XSTEP_205");
            pr.send_fail(&msg205);
            nbval = -1;
        }
        if !pr.read_entity_status(ir, &pr.current(), &mut a_status, &mut temp_parent, false)? {
            let mut msg206 = iges_msg("XSTEP_206");
            iges_send_read_status(pr, &mut msg206, a_status, false)?;
        }
        if nbval > 0 {
            pr.read_ents(
                ir,
                &pr.current_list(nbval, 1),
                &msg207,
                &mut temp_children,
                1,
            )?;
        }
        self.dir_checker(ent).check_type_and_form(pr.check(), ent);
        ent_live.init(temp_nb_parent_entities, &temp_parent, &temp_children)?;
        Ok(())
    }
}
impl IgesBasicToolSingleParent {
    /// Writes the own parameters of the entity.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn write_own_params(
        &self,
        ent: &Handle<IgesBasicSingleParent>,
        iw: &mut IgesDataIgesWriter,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolSingleParent::write_own_params")?;
        let upper: i32 = ent_live.nb_children();
        iw.send(ent_live.nb_parent_entities())?;
        iw.send(upper)?;
        iw.send_entity(&ent_live.single_parent(), false)?;
        for i in 1..=upper {
            iw.send_entity(&ent_live.child(i)?, false)?;
        }
        Ok(())
    }
    /// Lists the entities the own parameters refer to.
    ///
    /// # Errors
    /// A failure of a parameter list or an entity member.
    pub fn own_shared(
        &self,
        ent: &Handle<IgesBasicSingleParent>,
        iter: &mut InterfaceEntityIterator,
    ) -> Result<(), Failure> {
        let ent_live = iges_live(ent, "IgesBasicToolSingleParent::own_shared")?;
        iter.get_one_item(&ent_live.single_parent().cast::<StandardTransient>());
        let upper: i32 = ent_live.nb_children();
        for i in 1..=upper {
            iter.get_one_item(&ent_live.child(i)?.cast::<StandardTransient>());
        }
        Ok(())
    }
    /// The checker of the directory part of this kind of entity.
    pub fn dir_checker(&self, _ent: &Handle<IgesBasicSingleParent>) -> IgesDataDirChecker {
        let mut dc = IgesDataDirChecker::with_form(402, 9);
        dc.structure(IgesDataDefType::DefVoid);
        dc.graphics_ignored(-1);
        dc.blank_status_ignored();
        dc.hierarchy_status_ignored();
        dc
    }
}
